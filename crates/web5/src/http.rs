use native_tls::TlsConnector;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
use url::Url;

use crate::errors::Web5Error;

pub fn get_json_schema(url: &str) -> Result<Value, Web5Error> {
    // Parse the URL using the `url` crate
    let parsed_url =
        Url::parse(url).map_err(|err| Web5Error::Http(format!("failed to parse url {}", err)))?;

    // Extract the host and path
    let host = parsed_url
        .host_str()
        .ok_or_else(|| Web5Error::Http(format!("url must have a host {}", url)))?;
    let path = if parsed_url.path().is_empty() {
        "/"
    } else {
        parsed_url.path()
    };

    // Determine the port (80 for HTTP, 443 for HTTPS, or custom port)
    let port = parsed_url
        .port_or_known_default()
        .ok_or_else(|| Web5Error::Http("unable to determine port".to_string()))?;

    // Construct the HTTP request string in multi-line format
    let request = format!(
        "GET {path} HTTP/1.1\r\n\
        Host: {host}\r\n\
        Connection: close\r\n\
        Accept: application/json\r\n\r\n"
    );

    // Check if the URL is HTTPS or HTTP
    let response = if parsed_url.scheme() == "https" {
        // Handle HTTPS using native-tls
        let connector = TlsConnector::new().map_err(|err| Web5Error::Network(err.to_string()))?;
        let stream =
            TcpStream::connect((host, port)).map_err(|err| Web5Error::Network(err.to_string()))?;
        let mut stream = connector
            .connect(host, stream)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        // Send the HTTP request over the secure connection
        stream
            .write_all(request.as_bytes())
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        // Buffer to hold the response data
        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        // Convert the response to a String
        String::from_utf8(buffer)
            .map_err(|err| Web5Error::Http(format!("failed to stringify response body {}", err)))?
    } else {
        // Handle plain HTTP
        let mut stream =
            TcpStream::connect((host, port)).map_err(|err| Web5Error::Network(err.to_string()))?;

        // Send the HTTP request
        stream
            .write_all(request.as_bytes())
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        // Buffer to hold the response data
        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .map_err(|err| Web5Error::Network(err.to_string()))?;

        // Convert the response to a String
        String::from_utf8(buffer)
            .map_err(|err| Web5Error::Http(format!("failed to stringify response body {}", err)))?
    };

    // Split response into headers and body
    let parts: Vec<&str> = response.split("\r\n\r\n").collect();
    if parts.len() < 2 {
        return Err(Web5Error::Http("invalid http response".to_string()));
    }

    let header = parts[0];
    let body = parts[1];

    // Check the status code in the HTTP response header
    if let Some(status_line) = header.lines().next() {
        if !status_line.contains("200 OK") {
            // TODO cover all of 200..300
            return Err(Web5Error::Http(format!(
                "non-successful response code {}",
                status_line
            )));
        }
    }

    // Parse the response body as JSON
    let schema_json: Value = serde_json::from_str(body)
        .map_err(|err| Web5Error::Http(format!("unable to parse json response body {}", err)))?;

    Ok(schema_json)
}
