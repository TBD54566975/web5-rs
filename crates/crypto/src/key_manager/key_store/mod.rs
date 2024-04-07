pub mod in_memory_key_store;

use crate::key::PrivateKey;

#[derive(thiserror::Error, Debug)]
pub enum KeyStoreError {
    #[error("{0}")]
    InternalKeyStoreError(String),
}

// Trait for storing and retrieving private keys.
//
// Implementations of this trait should be thread-safe and allow for concurrent access.
pub trait KeyStore<T: PrivateKey>: Send + Sync {
    fn get(&self, key_alias: &str) -> Result<Option<T>, KeyStoreError>;
    fn insert(&self, key_alias: &str, private_key: T) -> Result<(), KeyStoreError>;
}
