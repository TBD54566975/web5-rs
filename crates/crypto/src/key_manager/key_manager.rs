use crate::key::{KeyAlgorithm, KeyAlias, KeyError, PublicKey};
use crate::key_manager::key_store::KeyStoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyManagerError {
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error(transparent)]
    KeyStoreError(#[from] KeyStoreError),
    #[error("Signing key not found in KeyManager")]
    SigningKeyNotFound,
}

pub trait KeyManager: Send + Sync {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<GeneratePrivateKeyResponse, KeyManagerError>;

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError>;

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError>;
}

pub type GeneratePrivateKeyResponse = (KeyAlias, PublicKey);
