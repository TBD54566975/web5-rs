use std::io::{self, Read, Write};
use std::net::TcpStream;
use url::Url;

pub struct HttpResponse {
    pub status_code: u16,
    pub body: String,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }
}

pub struct HttpClient;

impl HttpClient {
    fn send_request(host: &str, port: u16, request: &str) -> io::Result<HttpResponse> {
        let address = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(address)?;

        // Send the request
        stream.write_all(request.as_bytes())?;
        stream.flush()?;

        // Read the response
        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        // Split the response into the headers and body
        let (status_code, body) = Self::parse_response(&response)?;

        Ok(HttpResponse { status_code, body })
    }

    pub fn get(url: &str) -> io::Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid URL"))?;
        let host = parsed_url
            .host_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing host"))?
            .to_string();
        let port = parsed_url.port_or_known_default().unwrap_or(80);
        let path = parsed_url.path();

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path, host
        );

        Self::send_request(&host, port, &request)
    }

    pub fn put(url: &str, body: &[u8]) -> io::Result<HttpResponse> {
        let parsed_url = Url::parse(url)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid URL"))?;
        let host = parsed_url
            .host_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing host"))?
            .to_string();
        let port = parsed_url.port_or_known_default().unwrap_or(80);
        let path = parsed_url.path();

        let request = format!(
            "PUT {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            path,
            host,
            body.len()
        );

        Self::send_request(&host, port, &request)
    }

    fn parse_response(response: &str) -> io::Result<(u16, String)> {
        // Split headers and body
        let mut parts = response.split("\r\n\r\n");
        let header_part = parts.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid HTTP response format")
        })?;
        let body = parts.next().unwrap_or("").to_string();

        // Parse the status code from the first line
        let status_line = header_part.lines().next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing status line in HTTP response",
            )
        })?;

        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid status line"))?
            .parse::<u16>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid status code"))?;

        Ok((status_code, body))
    }
}
