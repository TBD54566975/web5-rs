use crypto::key::KeyError;
use crypto::key_manager::key_store::KeyStoreError;
use crypto::key_manager::KeyManagerError;
use std::fmt::Display;
use uniffi::UnexpectedUniFFICallbackError;

pub type Result<T, E = CryptoError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("{msg}")]
    InternalError { msg: String },
}

impl CryptoError {
    pub fn new<E: Display>(error: E) -> Self {
        Self::InternalError {
            msg: error.to_string(),
        }
    }
}

impl From<KeyStoreError> for CryptoError {
    fn from(e: KeyStoreError) -> Self {
        Self::new(e)
    }
}

impl From<CryptoError> for KeyStoreError {
    fn from(e: CryptoError) -> Self {
        Self::InternalKeyStoreError(e.to_string())
    }
}

impl From<KeyManagerError> for CryptoError {
    fn from(e: KeyManagerError) -> Self {
        Self::new(e)
    }
}

impl From<CryptoError> for KeyManagerError {
    fn from(e: CryptoError) -> Self {
        Self::InternalKeyManagerError(e.to_string())
    }
}

impl From<KeyError> for CryptoError {
    fn from(e: KeyError) -> Self {
        Self::new(e.to_string())
    }
}

impl From<bincode::Error> for CryptoError {
    fn from(e: bincode::Error) -> Self {
        Self::new(e)
    }
}

impl From<UnexpectedUniFFICallbackError> for CryptoError {
    fn from(e: UnexpectedUniFFICallbackError) -> Self {
        Self::new(e)
    }
}
