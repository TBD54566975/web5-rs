use async_trait::async_trait;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, str::FromStr, sync::Arc};

use crate::errors::Web5Error;

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn fetch(
        &self,
        url: &str,
        options: Option<FetchOptions>,
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>>;

    async fn get(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
        self.fetch(
            url,
            Some(FetchOptions {
                method: Some(Method::Get),
                headers,
                body: None,
            }),
        )
        .await
    }

    async fn post(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: &[u8],
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
        self.fetch(
            url,
            Some(FetchOptions {
                method: Some(Method::Post),
                headers,
                body: Some(body.to_vec()),
            }),
        )
        .await
    }

    async fn put(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: &[u8],
    ) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
        self.fetch(
            url,
            Some(FetchOptions {
                method: Some(Method::Put),
                headers,
                body: Some(body.to_vec()),
            }),
        )
        .await
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct FetchOptions {
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
}

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
        };
        write!(f, "{}", method_str)
    }
}

impl FromStr for Method {
    type Err = Web5Error;

    fn from_str(s: &str) -> crate::errors::Result<Self> {
        match s.to_ascii_uppercase().as_ref() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            _ => Err(Web5Error::Parameter(format!("unknown method {}", s))),
        }
    }
}

static HTTP_CLIENT: OnceCell<Arc<dyn HttpClient>> = OnceCell::new();

#[cfg(feature = "http_reqwest")]
pub fn get_http_client() -> &'static dyn HttpClient {
    HTTP_CLIENT
        .get_or_init(|| Arc::new(reqwest_http_client::ReqwestHttpClient::new()))
        .as_ref()
}

#[cfg(not(feature = "http_reqwest"))]
pub fn get_http_client() -> &'static dyn HttpClient {
    HTTP_CLIENT
        .get()
        .expect("HttpClient has not been set. Please call set_http_client().")
        .as_ref()
}

#[cfg(feature = "http_reqwest")]
pub fn set_http_client(_: Arc<dyn HttpClient>) {
    panic!("Cannot set a custom HttpClient when the reqwest feature is enabled.");
}

#[cfg(not(feature = "http_reqwest"))]
pub fn set_http_client(client: Arc<dyn HttpClient>) {
    HTTP_CLIENT
        .set(client)
        .unwrap_or_else(|_| panic!("HttpClient has already been set."));
}

#[cfg(feature = "http_reqwest")]
mod reqwest_http_client {
    use super::*;
    use reqwest::{Client as ReqwestClient, Method as ReqwestMethod, Response as ReqwestResponse};
    use std::error::Error;

    pub struct ReqwestHttpClient {
        client: ReqwestClient,
    }

    impl ReqwestHttpClient {
        pub fn new() -> Self {
            ReqwestHttpClient {
                client: ReqwestClient::new(),
            }
        }

        fn map_method(method: Option<Method>) -> ReqwestMethod {
            match method {
                Some(Method::Post) => ReqwestMethod::POST,
                Some(Method::Put) => ReqwestMethod::PUT,
                _ => ReqwestMethod::GET,
            }
        }

        async fn build_request(
            &self,
            url: &str,
            options: Option<FetchOptions>,
        ) -> Result<reqwest::RequestBuilder, Box<dyn Error>> {
            let FetchOptions {
                method,
                headers,
                body,
            } = options.unwrap_or_default();

            let req_method = Self::map_method(method);
            let mut req = self.client.request(req_method, url);

            if let Some(headers) = headers {
                for (key, value) in headers {
                    req = req.header(&key, &value);
                }
            }

            if let Some(body) = body {
                req = req.body(body);
            }

            Ok(req)
        }

        async fn parse_response(response: ReqwestResponse) -> Result<HttpResponse, Box<dyn Error>> {
            let status_code = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            let body = response.bytes().await?.to_vec();

            Ok(HttpResponse {
                status_code,
                headers,
                body,
            })
        }
    }

    #[async_trait]
    impl HttpClient for ReqwestHttpClient {
        async fn fetch(
            &self,
            url: &str,
            options: Option<FetchOptions>,
        ) -> Result<HttpResponse, Box<dyn Error>> {
            let req = self.build_request(url, options).await?;
            let res = req.send().await?;
            Self::parse_response(res).await
        }
    }
}
