pub mod in_memory_key_manager;
pub mod key_manager;

use crate::crypto::jwk::JwkError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyManagerError {
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("{0}")]
    InternalKeyStoreError(String),
    #[error("key not found {0}")]
    KeyNotFound(String),
    #[error("unknown error")]
    Unknown,
}

impl Default for KeyManagerError {
    fn default() -> Self {
        Self::Unknown
    }
}

pub type Result<T> = std::result::Result<T, KeyManagerError>;
