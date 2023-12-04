use crypto::key_manager::key_store::KeyStoreError;
use crypto::key_manager::KeyManagerError;

pub type Result<T, E = CryptoError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("{msg}")]
    Generic { msg: String },
}

impl From<KeyStoreError> for CryptoError {
    fn from(e: KeyStoreError) -> Self {
        Self::Generic { msg: e.to_string() }
    }
}

impl From<CryptoError> for KeyStoreError {
    fn from(e: CryptoError) -> Self {
        Self::InternalKeyStoreError(e.to_string())
    }
}

impl From<KeyManagerError> for CryptoError {
    fn from(e: KeyManagerError) -> Self {
        Self::Generic { msg: e.to_string() }
    }
}

impl From<CryptoError> for KeyManagerError {
    fn from(e: CryptoError) -> Self {
        Self::InternalKeyManagerError(e.to_string())
    }
}

impl From<bincode::Error> for CryptoError {
    fn from(e: bincode::Error) -> Self {
        Self::Generic { msg: e.to_string() }
    }
}
