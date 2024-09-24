use crate::errors::{Result, Web5Error};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// todo use generalized feature flag, not target_arch, b/c we'll do injection for all foreign bindings
#[cfg(not(target_arch = "wasm32"))]
use reqwest::blocking::get as reqwest_get;

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub trait HttpClient: Sync + Send {
    fn get(&self, url: &str) -> Result<HttpResponse>;
    fn put_bytes(&self, url: &str, body: &[u8]) -> Result<HttpResponse>;
}

#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    pub static ref HTTP_CLIENT: Mutex<Arc<dyn HttpClient>> = Mutex::new(Arc::new(RustHttpClient));
}

#[cfg(target_arch = "wasm32")]
lazy_static! {
    pub static ref HTTP_CLIENT: Mutex<Arc<dyn HttpClient>> =
        Mutex::new(Arc::new(ForeignEmptyHttpClient));
}

pub fn set_http_client(client: Arc<dyn HttpClient>) {
    let mut global_client = HTTP_CLIENT.lock().unwrap();
    *global_client = client;
}

pub fn get_http_client() -> Arc<dyn HttpClient> {
    let client = HTTP_CLIENT.lock().unwrap();
    client.clone()
}

pub(crate) fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
    let http_response = get(url)?;

    if !(200..300).contains(&http_response.status_code) {
        return Err(Web5Error::Http(format!(
            "http error status code {} for url {}",
            http_response.status_code, url
        )));
    }

    let json = serde_json::from_slice::<T>(&http_response.body)
        .map_err(|e| Web5Error::Http(format!("failed to parse json {}", e)))?;

    Ok(json)
}

pub(crate) fn get(url: &str) -> Result<HttpResponse> {
    let http_client = get_http_client();
    http_client.get(url)
}

pub(crate) fn put_bytes(url: &str, body: &[u8]) -> Result<HttpResponse> {
    let http_client = get_http_client();
    http_client.put_bytes(url, body)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct RustHttpClient;

#[cfg(not(target_arch = "wasm32"))]
impl HttpClient for RustHttpClient {
    fn get(&self, url: &str) -> Result<HttpResponse> {
        let response = reqwest_get(url)
            .map_err(|err| Web5Error::Http(format!("get request failed {} {}", url, err)))?;

        let status_code = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .map_err(|err| Web5Error::Http(format!("failed to read body {}", err)))?
            .to_vec();

        Ok(HttpResponse {
            status_code,
            headers,
            body,
        })
    }

    fn put_bytes(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .put(url)
            .header("Content-Type", "application/octet-stream")
            .body(body.to_vec())
            .send()
            .map_err(|err| Web5Error::Http(format!("put request failed {} {}", url, err)))?;

        let status_code = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .map_err(|err| Web5Error::Http(format!("failed to read body {}", err)))?
            .to_vec();

        Ok(HttpResponse {
            status_code,
            headers,
            body,
        })
    }
}

#[cfg(target_arch = "wasm32")]
pub struct ForeignEmptyHttpClient;

#[cfg(target_arch = "wasm32")]
impl HttpClient for ForeignEmptyHttpClient {
    fn get(&self, _url: &str) -> Result<HttpResponse> {
        Err(Web5Error::Http("http client not set".to_string()))
    }

    fn put_bytes(&self, _url: &str, _body: &[u8]) -> Result<HttpResponse> {
        Err(Web5Error::Http("http client not set".to_string()))
    }
}

// use crate::errors::{Result, Web5Error};
// use rustls::pki_types::ServerName;
// use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
// use rustls_native_certs::load_native_certs;
// use serde::de::DeserializeOwned;
// use std::collections::HashMap;
// use std::io::{Read, Write};
// use std::net::TcpStream;
// use std::sync::Arc;
// use url::Url;

// pub struct HttpResponse {
//     pub status_code: u16,
//     #[allow(dead_code)]
//     pub headers: HashMap<String, String>,
//     pub body: Vec<u8>,
// }

// struct Destination {
//     pub host: String,
//     pub path: String,
//     pub port: u16,
//     pub schema: String,
// }

// fn parse_destination(url: &str) -> Result<Destination> {
//     let parsed_url =
//         Url::parse(url).map_err(|err| Web5Error::Http(format!("failed to parse url {}", err)))?;

//     let host = parsed_url
//         .host_str()
//         .ok_or_else(|| Web5Error::Http(format!("url must have a host: {}", url)))?;

//     let path = if parsed_url.path().is_empty() {
//         "/".to_string()
//     } else {
//         parsed_url.path().to_string()
//     };

//     let port = parsed_url
//         .port_or_known_default()
//         .ok_or_else(|| Web5Error::Http("unable to determine port".to_string()))?;

//     let schema = parsed_url.scheme().to_string();

//     Ok(Destination {
//         host: host.to_string(),
//         path,
//         port,
//         schema,
//     })
// }

// fn transmit(destination: &Destination, request: &[u8]) -> Result<Vec<u8>> {
//     let mut buffer = Vec::new();

//     if destination.schema == "https" {
//         // HTTPS connection

//         // Create a RootCertStore and load the root certificates from rustls_native_certs
//         let mut root_store = RootCertStore::empty();
//         for cert in load_native_certs().unwrap() {
//             root_store.add(cert).unwrap();
//         }

//         // Build the ClientConfig using the root certificates and disabling client auth
//         let config = ClientConfig::builder()
//             .with_root_certificates(root_store)
//             .with_no_client_auth();

//         let rc_config = Arc::new(config); // Arc allows sharing the config

//         // Make the TCP connection to the server
//         let stream = TcpStream::connect((&destination.host[..], destination.port))
//             .map_err(|err| Web5Error::Network(format!("failed to connect to host: {}", err)))?;

//         // Convert the server name to the expected type for TLS validation
//         let server_name = ServerName::try_from(destination.host.clone())
//             .map_err(|_| Web5Error::Http("invalid DNS name".to_string()))?;

//         // Create the TLS connection
//         let client = ClientConnection::new(rc_config, server_name)
//             .map_err(|err| Web5Error::Network(err.to_string()))?;
//         let mut tls_stream = StreamOwned::new(client, stream);

//         // Write the request over the TLS stream
//         tls_stream
//             .write_all(request)
//             .map_err(|err| Web5Error::Network(err.to_string()))?;

//         // Read the response into the buffer
//         tls_stream
//             .read_to_end(&mut buffer)
//             .map_err(|err| Web5Error::Network(err.to_string()))?;
//     } else {
//         // HTTP connection
//         let mut stream = TcpStream::connect((&destination.host[..], destination.port))
//             .map_err(|err| Web5Error::Network(format!("failed to connect to host: {}", err)))?;

//         stream
//             .write_all(request)
//             .map_err(|err| Web5Error::Network(err.to_string()))?;

//         stream
//             .read_to_end(&mut buffer)
//             .map_err(|err| Web5Error::Network(err.to_string()))?;
//     }

//     Ok(buffer)
// }

// fn parse_response(response_bytes: &[u8]) -> Result<HttpResponse> {
//     // Find the position of the first \r\n\r\n, which separates headers and body
//     let header_end = response_bytes
//         .windows(4)
//         .position(|window| window == b"\r\n\r\n")
//         .ok_or_else(|| Web5Error::Http("invalid HTTP response format".to_string()))?;

//     // Extract the headers section (before the \r\n\r\n)
//     let header_part = &response_bytes[..header_end];

//     // Convert the header part to a string (since headers are ASCII/UTF-8 compliant)
//     let header_str = String::from_utf8_lossy(header_part);

//     // Parse the status line (first line in the headers)
//     let mut header_lines = header_str.lines();
//     let status_line = header_lines
//         .next()
//         .ok_or_else(|| Web5Error::Http("missing status line".to_string()))?;

//     let status_parts: Vec<&str> = status_line.split_whitespace().collect();
//     if status_parts.len() < 3 {
//         return Err(Web5Error::Http("invalid status line format".to_string()));
//     }

//     let status_code = status_parts[1]
//         .parse::<u16>()
//         .map_err(|_| Web5Error::Http("invalid status code".to_string()))?;

//     // Parse headers into a HashMap
//     let mut headers = HashMap::new();
//     for line in header_lines {
//         if let Some((key, value)) = line.split_once(": ") {
//             headers.insert(key.to_string(), value.to_string());
//         }
//     }

//     // The body is the part after the \r\n\r\n separator
//     let body = response_bytes[header_end + 4..].to_vec();

//     Ok(HttpResponse {
//         status_code,
//         headers,
//         body,
//     })
// }

// pub fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
//     let destination = parse_destination(url)?;
//     let request = format!(
//         "GET {} HTTP/1.1\r\n\
//         Host: {}\r\n\
//         Connection: close\r\n\
//         Accept: application/json\r\n\r\n",
//         destination.path, destination.host
//     );
//     let response_bytes = transmit(&destination, request.as_bytes())?;
//     let response = parse_response(&response_bytes)?;

//     if !(200..300).contains(&response.status_code) {
//         return Err(Web5Error::Http(format!(
//             "non-successful response code {}",
//             response.status_code
//         )));
//     }

//     let json_value = serde_json::from_slice::<T>(&response.body)
//         .map_err(|err| Web5Error::Http(format!("unable to parse json response body {}", err)))?;

//     Ok(json_value)
// }

// pub fn get(url: &str) -> Result<HttpResponse> {
//     let destination = parse_destination(url)?;

//     let request = format!(
//         "GET {} HTTP/1.1\r\n\
//         Host: {}\r\n\
//         Connection: close\r\n\
//         Accept: application/octet-stream\r\n\r\n",
//         destination.path, destination.host
//     );

//     let response_bytes = transmit(&destination, request.as_bytes())?;

//     parse_response(&response_bytes)
// }

// pub fn put(url: &str, body: &[u8]) -> Result<HttpResponse> {
//     let destination = parse_destination(url)?;

//     let request = format!(
//         "PUT {} HTTP/1.1\r\n\
//         Host: {}\r\n\
//         Connection: close\r\n\
//         Content-Length: {}\r\n\
//         Content-Type: application/octet-stream\r\n\r\n",
//         destination.path,
//         destination.host,
//         body.len()
//     );

//     // Concatenate the request headers and the body to form the full request
//     let mut request_with_body = request.into_bytes();
//     request_with_body.extend_from_slice(body);

//     let response_bytes = transmit(&destination, &request_with_body)?;

//     parse_response(&response_bytes)
// }
