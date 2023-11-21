use crate::error::Result;
use crate::key_store::{KeyStore, KeyStoreWrapper};
use crypto::key::{KeyAlgorithm, PublicKey};
use crypto::key_manager::LocalKeyManager as RustLocalKeyManager;
use crypto::key_manager::{KeyManager as RustKeyManager, KeyManagerError};
use std::sync::Arc;

pub struct KeyManager {
    inner: Arc<dyn RustKeyManager>,
}

impl KeyManager {
    pub fn new(key_store: Arc<dyn KeyStore>) -> Self {
        let wrapper = Arc::new(KeyStoreWrapper(key_store));
        Self {
            inner: Arc::new(RustLocalKeyManager::new(wrapper)),
        }
    }

    pub fn in_memory() -> Self {
        Self {
            inner: Arc::new(RustLocalKeyManager::new_in_memory()),
        }
    }
}

impl RustKeyManager for KeyManager {
    fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, KeyManagerError> {
        self.inner.generate_private_key(key_algorithm)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError> {
        self.inner.get_public_key(key_alias)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        self.inner.sign(key_alias, payload)
    }

    fn alias(&self, public_key: &PublicKey) -> std::result::Result<String, KeyManagerError> {
        self.inner.alias(public_key)
    }
}
