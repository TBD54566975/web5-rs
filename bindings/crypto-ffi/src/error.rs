use crypto::key_manager::key_store::KeyStoreError;
use std::fmt::Display;

pub type Result<T, E = CryptoError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    Generic { msg: String },
}

impl Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::Generic { msg } => write!(f, "{}", msg),
        }
    }
}

impl From<KeyStoreError> for CryptoError {
    fn from(e: KeyStoreError) -> Self {
        Self::Generic { msg: e.to_string() }
    }
}

impl From<CryptoError> for KeyStoreError {
    fn from(e: CryptoError) -> Self {
        Self::InternalKeyStoreError {
            message: e.to_string(),
        }
    }
}
