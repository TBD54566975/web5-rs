use crate::error::Result;
use crypto_ffi::KeyManager;
pub use dids::method::{DidKey as RustDidKey, DidKeyCreateOptions};
use std::sync::Arc;

pub struct DidKey {
    inner: RustDidKey,
}

impl DidKey {
    pub fn new(key_manager: Arc<KeyManager>, options: DidKeyCreateOptions) -> Result<Self> {
        let inner = RustDidKey::new(key_manager.clone(), options)?;
        Ok(Self { inner })
    }

    pub fn uri(&self) -> String {
        self.inner.uri.to_string()
    }
}
