pub mod key_store;
pub mod local_key_manager;

use crate::key::{KeyError, PrivateKey, PublicKey};
use crate::key_manager::key_store::KeyStoreError;
use crypto::{Curve, Signer};
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyManagerError {
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Signing key not found in KeyManager")]
    SigningKeyNotFound,
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error(transparent)]
    KeyStoreError(#[from] KeyStoreError),
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
    fn generate_private_key(&self, curve: Curve) -> Result<String, KeyManagerError>;

    /// Returns the public key associated with the provided `key_alias`, if one exists.
    fn get_public_key(&self, key_alias: &str) -> Result<Arc<dyn PublicKey>, KeyManagerError>;

    /// Signs the provided payload using the private key identified by the provided `key_alias`.
    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError>;

    /// Retrieves a signer for the given `key_alias`.
    fn get_signer(&self, key_alias: &str) -> Result<Signer, KeyManagerError>;

    /// Exports all private keys managed by this key manager.
    /// Default implementation returns an error indicating the feature is not supported.
    fn export_private_keys(&self) -> Result<Vec<Arc<dyn PrivateKey>>, KeyManagerError> {
        Err(KeyStoreError::UnsupportedOperation(
            "exporting private keys is not supported".to_string(),
        ))?
    }

    /// Imports a list of private keys into the key manager.
    /// Default implementation returns an error indicating the feature is not supported.
    fn import_private_keys(
        &self,
        _private_keys: Vec<Arc<dyn PrivateKey>>,
    ) -> Result<(), KeyManagerError> {
        Err(KeyStoreError::UnsupportedOperation(
            "importing private keys is not supported".to_string(),
        ))?
    }
}
