use crate::Result;
use std::collections::HashMap;

pub trait Client: Send + Sync {
    fn fetch(&self, url: &str, options: Option<FetchOptions>) -> Result<Response>;
}

#[derive(Default)]
pub struct FetchOptions {
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub enum Method {
    Get,
    Post,
    Put,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => "GET".to_string(),
            Method::Post => "POST".to_string(),
            Method::Put => "PUT".to_string(),
        }
    }
}
