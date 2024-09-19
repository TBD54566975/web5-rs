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
