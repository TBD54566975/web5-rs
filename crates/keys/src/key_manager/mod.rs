pub mod key_store;
pub mod local_key_manager;

use crate::key::{Curve, KeyError, PublicKey};
use crate::key_manager::key_store::KeyStoreError;
// use jose::jws_signer::JwsSignerError;
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

// impl From<KeyManagerError> for JwsSignerError {
//     fn from(value: KeyManagerError) -> Self {
//         Self::UnknownError(value.to_string())
//     }
// }

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
    fn get_public_key(
        &self,
        key_alias: &str,
    ) -> Result<Option<Arc<dyn PublicKey>>, KeyManagerError>;

    /// Signs the provided payload using the private key identified by the provided `key_alias`.
    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError>;
}
