mod client;
mod default_client;
mod error;
mod reqwest_client;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub use client::{Client, FetchOptions, Method, Response};
pub use error::{Error, Result};

lazy_static! {
    static ref CLIENT: Mutex<Arc<dyn Client>> =
        Mutex::new(Arc::new(reqwest_client::ReqwestClient::new()));
}

pub fn set_client(client: Arc<dyn Client>) {
    let mut global_client = CLIENT.lock().unwrap();
    *global_client = client;
}

pub fn get_client() -> Arc<dyn Client> {
    let client = CLIENT.lock().unwrap();
    client.clone()
}

pub fn fetch(url: &str, options: Option<FetchOptions>) -> Result<Response> {
    let client = get_client();
    client.fetch(url, options)
}
