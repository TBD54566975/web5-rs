use crate::{Client, FetchOptions, Method, Response, Result};
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        ReqwestClient {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Client for ReqwestClient {
    async fn fetch(&self, url: &str, options: Option<FetchOptions>) -> Result<Response> {
        let options = options.unwrap_or_default();
        let method = options.method.unwrap_or(Method::Get).to_string();

        let mut req = match method.as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            _ => unreachable!(),
        };

        if let Some(headers) = options.headers {
            let mut req_headers = HeaderMap::new();
            for (key, value) in headers {
                req_headers.insert(
                    reqwest::header::HeaderName::try_from(key.as_str()).unwrap(),
                    value.parse().unwrap(),
                );
            }
            req = req.headers(req_headers);
        }

        if let Some(body) = options.body {
            req = req.body(body);
        }

        let res = req.send().await.map_err(crate::Error::from)?;

        let status_code = res.status().as_u16();
        let mut headers = HashMap::new();
        for (key, value) in res.headers().iter() {
            headers.insert(key.to_string(), value.to_str().unwrap().to_string());
        }

        let body = res.bytes().await.map_err(crate::Error::from)?.to_vec();

        Ok(Response {
            status_code,
            headers,
            body,
        })
    }
}
