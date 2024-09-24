
use std::{collections::HashMap, sync::Arc};
use once_cell::sync::OnceCell;

pub trait HttpClient: Send + Sync {
    fn get(
        &self, 
        url: &str, 
        headers: Option<HashMap<String, String>>
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>>;

    fn post(
        &self, 
        url: &str, 
        headers: Option<HashMap<String, String>>, 
        body: &[u8]
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>>;

    fn put(
        &self, 
        url: &str, 
        headers: Option<HashMap<String, String>>, 
        body: &[u8]
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>>;
}

pub struct HttpResponse {
    pub status_code: u16,
    #[allow(dead_code)]
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

static HTTP_CLIENT: OnceCell<Arc<dyn HttpClient>> = OnceCell::new();

#[cfg(feature = "default_http_client")]
pub fn get_http_client() -> &'static dyn HttpClient {
    HTTP_CLIENT.get_or_init(|| {
        Arc::new(reqwest_http_client::ReqwestHttpClient::new())
    }).as_ref()
}

#[cfg(not(feature = "default_http_client"))]
pub fn get_http_client() -> &'static dyn HttpClient {
    HTTP_CLIENT.get().expect("HttpClient has not been set. Please call set_http_client().").as_ref()
}

#[cfg(feature = "default_http_client")]
pub fn set_http_client() {
    panic!("Cannot set a custom HttpClient when the reqwest feature is enabled.");
}

#[cfg(not(feature = "default_http_client"))]
pub fn set_http_client() {
    HTTP_CLIENT.set(client).unwrap_or_else(|_| panic!("HttpClient has already been set."));
}

#[cfg(feature = "default_http_client")]
mod reqwest_http_client {
    use super::*;
    use reqwest::blocking::Client;
    use std::collections::HashMap;

    pub struct ReqwestHttpClient {
        client: Client,
    }

    impl ReqwestHttpClient {
        pub fn new() -> Self {
            ReqwestHttpClient {
                client: Client::new(),
            }
        }
    }

    impl HttpClient for ReqwestHttpClient {
        fn get(
            &self,
            url: &str,
            headers: Option<HashMap<String, String>>,
        ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let mut req = self.client.get(url);

            if let Some(headers) = headers {
                for (key, value) in headers {
                    req = req.header(&key, &value);
                }
            }

            let response = req.send()?.error_for_status()?;
            let status_code = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            let body = response.bytes()?.to_vec();

            Ok(HttpResponse {
                status_code,
                headers,
                body,
            })
        }

        fn post(
            &self,
            url: &str,
            headers: Option<HashMap<String, String>>,
            body: &[u8],
        ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let mut req = self.client.post(url).body(body.to_vec());

            if let Some(headers) = headers {
                for (key, value) in headers {
                    req = req.header(&key, &value);
                }
            }

            let response = req.send()?.error_for_status()?;
            let status_code = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            let body = response.bytes()?.to_vec();

            Ok(HttpResponse {
                status_code,
                headers,
                body,
            })
        }

        fn put(
            &self,
            url: &str,
            headers: Option<HashMap<String, String>>,
            body: &[u8],
        ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let mut req = self.client.put(url).body(body.to_vec());

            if let Some(headers) = headers {
                for (key, value) in headers {
                    req = req.header(&key, &value);
                }
            }

            let response = req.send()?.error_for_status()?;
            let status_code = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            let body = response.bytes()?.to_vec();

            Ok(HttpResponse {
                status_code,
                headers,
                body,
            })
        }
    }
}
