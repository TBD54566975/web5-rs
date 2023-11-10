use crate::crypto::key::{KeyAlgorithm, PrivateKey};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum KeyManagerError {
    #[error("An unknown error occurred")]
    Unknown,
}

#[uniffi::export]
pub trait KeyManager: Send + Sync {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyManagerError>;
}
