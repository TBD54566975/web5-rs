pub mod in_memory_key_store;

use crate::key::PrivateKey;
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone)]
pub enum KeyStoreError {
    #[error("{0}")]
    InternalKeyStoreError(String),
}

// Trait for storing and retrieving private keys.
//
// Implementations of this trait should be thread-safe and allow for concurrent access.
pub trait KeyStore: Send + Sync {
    fn get(&self, key_alias: &str) -> Result<Option<Arc<dyn PrivateKey>>, KeyStoreError>;
    fn insert(
        &self,
        key_alias: &str,
        private_key: Arc<dyn PrivateKey>,
    ) -> Result<(), KeyStoreError>;
}
