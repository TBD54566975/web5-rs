use crate::error::Result;
use crypto_ffi::KeyManager;
use dids::did::Did;
use dids::method::jwk::{DidJwk, DidJwkCreateOptions};
use dids::method::DidMethod;
use std::sync::Arc;

pub struct DidJwkFfi(DidJwk);

impl DidJwkFfi {
    pub fn new(key_manager: Arc<KeyManager>, options: DidJwkCreateOptions) -> Result<Self> {
        let inner = DidJwk::create(key_manager.clone(), options)?;
        Ok(Self(inner))
    }

    pub fn uri(&self) -> String {
        self.0.uri().to_string()
    }
}
