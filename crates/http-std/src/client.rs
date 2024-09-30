use crate::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, str::FromStr};

#[async_trait]
pub trait Client: Send + Sync {
    async fn fetch(&self, url: &str, options: Option<FetchOptions>) -> Result<Response>;
}

#[derive(Default, Serialize, Deserialize)]
pub struct FetchOptions {
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_uppercase().as_ref() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            _ => Err(Error::Parameter(format!("unknown method {}", s))),
        }
    }
}
