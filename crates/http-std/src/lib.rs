mod client;
#[cfg(not(target_arch = "wasm32"))]
mod default_client;
mod error;
#[cfg(not(target_arch = "wasm32"))]
mod reqwest_client;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub use client::{Client, FetchOptions, Method, Response};
pub use error::{Error, Result};

#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    static ref CLIENT: Mutex<Arc<dyn Client>> =
        Mutex::new(Arc::new(reqwest_client::ReqwestClient::new()));
}

#[cfg(target_arch = "wasm32")]
lazy_static! {
    static ref CLIENT: Mutex<Arc<dyn Client>> = Mutex::new(Arc::new(ForeignEmptyClient));
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

#[cfg(target_arch = "wasm32")]
pub struct ForeignEmptyClient;

#[cfg(target_arch = "wasm32")]
impl Client for ForeignEmptyClient {
    fn fetch(&self, _url: &str, _options: Option<FetchOptions>) -> Result<Response> {
        return Err(Error::Unknown("global client not set".to_string()));
    }
}
