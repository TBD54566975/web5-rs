mod in_memory;

use crate::crypto::key::{Key, KeyAlgorithm, PrivateKey, PublicKey};
use crate::crypto::key_manager::{KeyManager, KeyManagerError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyStoreError {
    #[error("Key not found for alias: {key_alias}")]
    KeyNotFound { key_alias: String },
    #[error("UnexpectedReadError: {message}")]
    UnexpectedReadError { message: String },
    #[error("UnexpectedWriteError: {message}")]
    UnexpectedWriteError { message: String },
}

pub trait KeyStore {
    fn get(&self, key_alias: &str) -> Result<PrivateKey, KeyStoreError>;
    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError>;
}

impl KeyManager for dyn KeyStore {
    fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, KeyManagerError> {
        let private_key = PrivateKey::new(key_algorithm)?;
        let key_alias = private_key.alias()?;

        self.insert(&key_alias, private_key)?;
        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<PublicKey, KeyManagerError> {
        let private_key = self.get(key_alias)?;
        Ok(PublicKey::from(private_key))
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let private_key = self.get(key_alias)?;
        let signed_payload = private_key.sign(&payload.to_vec())?;

        Ok(signed_payload)
    }

    fn get_deterministic_alias(public_key: PublicKey) -> Result<String, KeyManagerError> {
        Ok(public_key.alias()?)
    }
}
