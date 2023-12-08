use crate::error::Result;
use crypto_ffi::KeyManager;
use dids::did::Did;
use dids::method::key::{DidKey as DidsDidKey, DidKeyCreateOptions};
use dids::method::DidMethod;
use std::sync::Arc;

pub struct DidKey(DidsDidKey);

impl DidKey {
    pub fn new(key_manager: Arc<KeyManager>, options: DidKeyCreateOptions) -> Result<Self> {
        let inner = DidsDidKey::create(key_manager.clone(), options)?;
        Ok(Self(inner))
    }

    pub fn uri(&self) -> String {
        self.0.uri().to_string()
    }
}
