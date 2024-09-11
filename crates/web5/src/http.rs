use crate::errors::Web5Error;
use native_tls::TlsConnector;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
use url::Url;

pub fn get_json_schema(url: &str) -> Result<Value, Web5Error> {
    let parsed_url =
        Url::parse(url).map_err(|err| Web5Error::Http(format!("failed to parse url {}", err)))?;

    let host = parsed_url
        .host_str()
        .ok_or_else(|| Web5Error::Http(format!("url must have a host {}", url)))?;
    let path = if parsed_url.path().is_empty() {
        "/"
    } else {
        parsed_url.path()
    };

    let port = parsed_url
        .port_or_known_default()
        .ok_or_else(|| Web5Error::Http("unable to determine port".to_string()))?;

    let request = format!(
        "GET {path} HTTP/1.1\r\n\
        Host: {host}\r\n\
        Connection: close\r\n\
        Accept: application/json\r\n\r\n"
    );

    let response = if parsed_url.scheme() == "https" {
        let connector = TlsConnector::new().map_err(|err| Web5Error::Network(err.to_string()))?;
        let stream =
            TcpStream::connect((host, port)).map_err(|err| Web5Error::Network(err.to_string()))?;
        let mut stream = connector
            .connect(host, stream)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        stream
            .write_all(request.as_bytes())
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        String::from_utf8(buffer)
            .map_err(|err| Web5Error::Http(format!("failed to stringify response body {}", err)))?
    } else {
        let mut stream =
            TcpStream::connect((host, port)).map_err(|err| Web5Error::Network(err.to_string()))?;

        stream
            .write_all(request.as_bytes())
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        String::from_utf8(buffer)
            .map_err(|err| Web5Error::Http(format!("failed to stringify response body {}", err)))?
    };

    let parts: Vec<&str> = response.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err(Web5Error::Http("invalid http response".to_string()));
    }

    let header = parts[0];
    let body = parts[1];

    if let Some(status_line) = header.lines().next() {
        if let Some(code) = status_line.split_whitespace().nth(1) {
            if let Ok(code) = code.parse::<u16>() {
                if !(200..300).contains(&code) {
                    return Err(Web5Error::Http(format!(
                        "non-successful response code {}",
                        code
                    )));
                }
            } else {
                return Err(Web5Error::Http("Invalid HTTP status code".to_string()));
            }
        }
    }

    let schema_json: Value = serde_json::from_str(body)
        .map_err(|err| Web5Error::Http(format!("unable to parse json response body {}", err)))?;

    Ok(schema_json)
}
