use crate::errors::{Result, Web5Error};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::blocking::get;

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub trait HttpClient {
    fn get_json<T: DeserializeOwned>(url: &str) -> Result<T>;
    fn get(url: &str) -> Result<HttpResponse>;

    // TODO should this be named `put()` instead?
    fn put_bytes(url: &str, body: &[u8]) -> Result<HttpResponse>;
}

pub(crate) struct RustHttpClient;

#[cfg(not(target_arch = "wasm32"))]
impl HttpClient for RustHttpClient {
    fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
        let response = get(url)
            .map_err(|err| Web5Error::Http(format!("get request failed {} {}", url, err)))?;

        if !response.status().is_success() {
            return Err(Web5Error::Http(format!(
                "http error status code {} for url {}",
                response.status().as_u16(),
                url
            )));
        }

        let json = response
            .json::<T>()
            .map_err(|err| Web5Error::Http(format!("failed to parse json {}", err)))?;

        Ok(json)
    }

    fn get(url: &str) -> Result<HttpResponse> {
        let response = get(url)
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

    fn put_bytes(url: &str, body: &[u8]) -> Result<HttpResponse> {
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
impl HttpClient for RustHttpClient {
    fn get_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T> {
        // Implement the WASM-specific HTTP client using `wasm-bindgen` or `web-sys` fetch API
        unimplemented!("WASM HTTP client not implemented");
    }

    fn get(url: &str) -> Result<HttpResponse> {
        // Implement the WASM-specific HTTP client using `wasm-bindgen` or `web-sys` fetch API
        unimplemented!("WASM HTTP client not implemented");
    }

    fn put_bytes(url: &str, body: &[u8]) -> Result<HttpResponse> {
        // Implement the WASM-specific HTTP client using `wasm-bindgen` or `web-sys` fetch API
        unimplemented!("WASM HTTP client not implemented");
    }
}
