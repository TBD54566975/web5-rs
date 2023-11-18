pub mod in_memory;

use crate::crypto::key::PrivateKey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyStoreError {
    #[error("{message}")]
    InternalKeyStoreError { message: String },
}

pub trait KeyStore: Send + Sync {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError>;
    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError>;
}
