pub mod local_key_manager;

use crate::key::{KeyError, PrivateKey, PublicKey};
use crypto::{CryptoError, Curve};
use jwk::JwkError;
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyManagerError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error("{0}")]
    InternalKeyStoreError(String),
    #[error("key not found {0}")]
    KeyNotFound(String),
}

/// A key management trait for generating, storing, and utilizing keys private keys and their
/// associated public keys.
///
/// Implementations of this trait might provide key management through various Key Management
/// Systems (KMS), such as AWS KMS, Google Cloud KMD, Hardware Security Modules (HSM), or simple
/// in-memory storage, each adhering to the same consistent API for usage within applications.
pub trait KeyManager: Send + Sync {
    /// Generates and securely stores a private key based on the provided `curve`,
    /// returning a unique alias that can be utilized to reference the generated key for future
    /// operations.
    fn generate_private_key(
        &self,
        curve: Curve,
        key_alias: Option<String>,
    ) -> Result<String, KeyManagerError>;

    /// Returns the public key associated with the provided `key_alias`, if one exists.
    fn get_public_key(&self, key_alias: &str) -> Result<Arc<dyn PublicKey>, KeyManagerError>;

    /// Signs the provided payload using the private key identified by the provided `key_alias`.
    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError>;
}

pub trait KeyImporter: KeyManager {
    /// Imports a private key with a custom key alias into the key manager.
    /// Returns the key alias
    fn import_with_alias(
        &self,
        private_key: Arc<dyn PrivateKey>,
        key_alias: &str,
    ) -> Result<(), KeyManagerError>;

    /// Imports a private key into the key manager using private_key.alias() as the alias
    /// Returns the key alias
    fn import(&self, private_key: Arc<dyn PrivateKey>) -> Result<String, KeyManagerError> {
        let key_alias = private_key.alias()?;
        self.import_with_alias(private_key, &key_alias)?;
        Ok(key_alias)
    }
}
