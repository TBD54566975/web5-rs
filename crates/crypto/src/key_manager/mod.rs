pub mod key_store;
pub mod local_key_manager;

use crate::key::{KeyError, KeyType, PublicKey};
use crate::key_manager::key_store::KeyStoreError;

#[derive(thiserror::Error, Debug)]
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
    /// Generates and securely stores a private key based on the provided `key_type`,
    /// returning a unique alias that can be utilized to reference the generated key for future
    /// operations.
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, KeyManagerError>;

    /// Returns the public key associated with the provided `key_alias`, if one exists.
    fn get_public_key(
        &self,
        key_alias: &str,
    ) -> Result<Option<Box<dyn PublicKey>>, KeyManagerError>;

    /// Signs the provided payload using the private key identified by the provided `key_alias`.
    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError>;

    /// Returns the key alias of a public key, as was originally returned by `generate_private_key`.
    fn alias(&self, public_key: Box<dyn PublicKey>) -> Result<String, KeyManagerError>;
}
