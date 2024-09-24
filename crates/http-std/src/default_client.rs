use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use rustls_native_certs::load_native_certs;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use url::Url;

use crate::{Client, Error, FetchOptions, Method, Response, Result};

struct Destination {
    pub host: String,
    pub path: String,
    pub port: u16,
    pub schema: String,
}

fn parse_destination(url: &str) -> Result<Destination> {
    let parsed_url =
        Url::parse(url).map_err(|err| Error::Parameter(format!("failed to parse url {}", err)))?;

    let host = parsed_url
        .host_str()
        .ok_or_else(|| Error::Parameter(format!("url must have a host: {}", url)))?;

    let path = if parsed_url.path().is_empty() {
        "/".to_string()
    } else {
        parsed_url.path().to_string()
    };

    let port = parsed_url
        .port_or_known_default()
        .ok_or_else(|| Error::Parameter("unable to determine port".to_string()))?;

    let schema = parsed_url.scheme().to_string();

    Ok(Destination {
        host: host.to_string(),
        path,
        port,
        schema,
    })
}

fn transmit(destination: &Destination, request: &[u8]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    if destination.schema == "https" {
        let mut root_store = RootCertStore::empty();
        for cert in load_native_certs().unwrap() {
            root_store.add(cert).unwrap();
        }

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let rc_config = Arc::new(config); // Arc allows sharing the config

        let stream = TcpStream::connect((&destination.host[..], destination.port))
            .map_err(|err| Error::Network(format!("failed to connect to host: {}", err)))?;

        let server_name = ServerName::try_from(destination.host.clone())
            .map_err(|_| Error::Network("invalid DNS name".to_string()))?;

        let client = ClientConnection::new(rc_config, server_name)
            .map_err(|err| Error::Network(err.to_string()))?;
        let mut tls_stream = StreamOwned::new(client, stream);

        tls_stream
            .write_all(request)
            .map_err(|err| Error::Network(err.to_string()))?;

        tls_stream
            .read_to_end(&mut buffer)
            .map_err(|err| Error::Network(err.to_string()))?;
    } else {
        let mut stream = TcpStream::connect((&destination.host[..], destination.port))
            .map_err(|err| Error::Network(format!("failed to connect to host: {}", err)))?;

        stream
            .write_all(request)
            .map_err(|err| Error::Network(err.to_string()))?;

        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Error::Network(err.to_string()))?;
    }

    Ok(buffer)
}

fn parse_response(response_bytes: &[u8]) -> Result<Response> {
    let header_end = response_bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| Error::Response("invalid HTTP response format".to_string()))?;

    let header_part = &response_bytes[..header_end];

    let header_str = String::from_utf8_lossy(header_part);

    let mut header_lines = header_str.lines();
    let status_line = header_lines
        .next()
        .ok_or_else(|| Error::Response("missing status line".to_string()))?;

    let status_parts: Vec<&str> = status_line.split_whitespace().collect();
    if status_parts.len() < 3 {
        return Err(Error::Response("invalid status line format".to_string()));
    }

    let status_code = status_parts[1]
        .parse::<u16>()
        .map_err(|_| Error::Response("invalid status code".to_string()))?;

    let mut headers = HashMap::new();
    for line in header_lines {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    let body = response_bytes[header_end + 4..].to_vec();

    Ok(Response {
        status_code,
        headers,
        body,
    })
}

pub struct DefaultClient;

impl Client for DefaultClient {
    fn fetch(&self, url: &str, options: Option<FetchOptions>) -> Result<Response> {
        let options = options.unwrap_or_default();
        let destination = parse_destination(url)?;
        let method = options.method.unwrap_or(Method::Get);

        let mut request = format!(
            "{} {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Connection: close\r\n",
            method.to_string(),
            destination.path,
            destination.host,
        );
        if let Some(headers) = &options.headers {
            if !headers.is_empty() {
                for (key, value) in headers {
                    request.push_str(&format!("{}: {}\r\n", key, value));
                }
            }
        }
        request.push_str("\r\n");

        let mut request_bytes = request.into_bytes();
        if let Some(body) = &options.body {
            request_bytes.extend_from_slice(body);
        }

        let response_bytes = transmit(&destination, &request_bytes)?;

        parse_response(&response_bytes)
    }
}
