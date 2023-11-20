use crate::error::Result;
use crypto_ffi::KeyManager;
use dids::did::{Did, DidJwk as RustDidJwk, DidJwkCreateOptions};
use std::sync::Arc;

pub struct DidJwk {
    inner: RustDidJwk,
}

impl DidJwk {
    pub fn new(key_manager: Arc<KeyManager>, options: DidJwkCreateOptions) -> Result<Self> {
        let inner = RustDidJwk::new(key_manager.clone(), options)?;
        Ok(Self { inner })
    }

    pub fn uri(&self) -> String {
        self.inner.uri().to_string()
    }
}
