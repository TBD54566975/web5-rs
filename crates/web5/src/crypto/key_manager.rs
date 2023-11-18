mod local_key_manager;

pub use local_key_manager::*;

use crate::crypto::key::{KeyAlgorithm, PublicKey};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyManagerError {
    #[error("{message}")]
    Generic { message: String }, // TODO: Fix this
                                 // #[error(transparent)]
                                 // KeyError(#[from] KeyError),
                                 // #[error(transparent)]
                                 // KeyStoreError(#[from] KeyStoreError),
}

pub trait KeyManager: Send + Sync {
    fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, KeyManagerError>;
    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError>;

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError>;

    // TODO: Do we REALLY need this?
    fn get_deterministic_alias(&self, public_key: PublicKey) -> Result<String, KeyManagerError>;
}
