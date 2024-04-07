pub mod in_memory_jwk_store;

use crate::key::{PrivateKey, PublicKey};

#[derive(thiserror::Error, Debug)]
pub enum KeyStoreError {
    #[error("{0}")]
    InternalKeyStoreError(String),
}

// Trait for storing and retrieving private keys.
//
// Implementations of this trait should be thread-safe and allow for concurrent access.
pub trait KeyStore<T, U>: Send + Sync
where
    T: PrivateKey<U>,
    U: PublicKey,
{
    fn get(&self, key_alias: &str) -> Result<Option<T>, KeyStoreError>;
    fn insert(&self, key_alias: &str, private_key: T) -> Result<(), KeyStoreError>;
}
