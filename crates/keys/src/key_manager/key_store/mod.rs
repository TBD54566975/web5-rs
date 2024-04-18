pub mod in_memory_key_store;

use crate::key::{KeyError, PrivateKey, PublicKey};
use crypto::{CryptoError, Curve};
use jwk::JwkError;
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyStoreError {
    #[error("{0}")]
    InternalKeyStoreError(String),
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error("key not found {0}")]
    KeyNotFound(String),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("{0}")]
    UnsupportedOperation(String),
}

// Trait for storing and retrieving private keys.
//
// Implementations of this trait should be thread-safe and allow for concurrent access.
pub trait KeyStore: Send + Sync {
    fn generate_new(&self, curve: Curve, key_alias: Option<String>) -> Result<String, KeyStoreError>;
    fn get_all_aliases(&self) -> Result<Vec<String>, KeyStoreError>;
    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyStoreError>;
    fn get_public_key(&self, key_alias: &str) -> Result<Arc<dyn PublicKey>, KeyStoreError>;

    fn export_private_keys(&self) -> Result<Vec<Arc<dyn PrivateKey>>, KeyStoreError> {
        Err(KeyStoreError::UnsupportedOperation(
            "exporting private keys is not supported".to_string(),
        ))
    }
    fn import_private_keys(
        &self,
        _private_keys: Vec<Arc<dyn PrivateKey>>,
    ) -> Result<(), KeyStoreError> {
        Err(KeyStoreError::UnsupportedOperation(
            "importing private keys is not supported".to_string(),
        ))
    }
}
