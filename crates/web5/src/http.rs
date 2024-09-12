use crate::errors::{Result, Web5Error};
use native_tls::TlsConnector;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use url::Url;

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

struct Destination {
    pub host: String,
    pub path: String,
    pub port: u16,
    pub schema: String,
}

fn parse_destination(url: &str) -> Result<Destination> {
    let parsed_url =
        Url::parse(url).map_err(|err| Web5Error::Http(format!("failed to parse url {}", err)))?;

    let host = parsed_url
        .host_str()
        .ok_or_else(|| Web5Error::Http(format!("url must have a host: {}", url)))?;

    let path = if parsed_url.path().is_empty() {
        "/".to_string()
    } else {
        parsed_url.path().to_string()
    };

    let port = parsed_url
        .port_or_known_default()
        .ok_or_else(|| Web5Error::Http("unable to determine port".to_string()))?;

    let schema = parsed_url.scheme().to_string();

    Ok(Destination {
        host: host.to_string(),
        path,
        port,
        schema,
    })
}

fn transmit(destination: &Destination, request: &str) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    if destination.schema == "https" {
        // HTTPS connection
        let connector = TlsConnector::new().map_err(|err| Web5Error::Network(err.to_string()))?;
        let stream = TcpStream::connect((&destination.host[..], destination.port))
            .map_err(|err| Web5Error::Network(err.to_string()))?;
        let mut stream = connector
            .connect(&destination.host, stream)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        stream
            .write_all(request.as_bytes())
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Web5Error::Network(err.to_string()))?;
    } else {
        // HTTP connection
        let mut stream = TcpStream::connect((&destination.host[..], destination.port))
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        stream
            .write_all(request.as_bytes())
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Web5Error::Network(err.to_string()))?;
    }

    Ok(buffer)
}

fn parse_response(response_bytes: &[u8]) -> Result<HttpResponse> {
    let response = String::from_utf8_lossy(response_bytes);
    let (header_part, body_part) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| Web5Error::Http("invalid HTTP response format".to_string()))?;

    // Split header into lines
    let mut header_lines = header_part.lines();

    // Parse the status line (e.g., HTTP/1.1 200 OK)
    let status_line = header_lines
        .next()
        .ok_or_else(|| Web5Error::Http("missing status line".to_string()))?;
    let status_parts: Vec<&str> = status_line.split_whitespace().collect();
    if status_parts.len() < 3 {
        return Err(Web5Error::Http("invalid status line format".to_string()));
    }

    let status_code = status_parts[1]
        .parse::<u16>()
        .map_err(|_| Web5Error::Http("invalid status code".to_string()))?;

    // Parse headers
    let mut headers = HashMap::new();
    for line in header_lines {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // Extract body bytes
    let body_offset = response_bytes.len() - body_part.as_bytes().len();
    let body = response_bytes[body_offset..].to_vec();

    Ok(HttpResponse {
        status_code,
        headers,
        body,
    })
}

pub fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
    let destination = parse_destination(url)?;
    let request = format!(
        "GET {} HTTP/1.1\r\n\
        Host: {}\r\n\
        Connection: close\r\n\
        Accept: application/json\r\n\r\n",
        destination.path, destination.host
    );
    let response_bytes = transmit(&destination, &request)?;
    let response = parse_response(&response_bytes)?;

    if !(200..300).contains(&response.status_code) {
        return Err(Web5Error::Http(format!(
            "non-successful response code {}",
            response.status_code
        )));
    }

    let json_value = serde_json::from_slice::<T>(&response.body)
        .map_err(|err| Web5Error::Http(format!("unable to parse json response body {}", err)))?;

    Ok(json_value)
}
