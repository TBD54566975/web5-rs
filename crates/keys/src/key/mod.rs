pub mod private_key;
pub mod public_key;

use jose::jwk::{Jwk, JwkError};
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyError {
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Failed to compute key thumbprint")]
    ThumprintFailed,
    #[error("failed to serialize")]
    SerializationFailed,
}

/// Trait defining all common behavior for cryptographic keys.
pub trait Key {
    fn jwk(&self) -> &Jwk;
}

pub trait PublicKey: Key + Send + Sync {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError>;

    fn alias(&self) -> Result<String, KeyError>;

    fn algorithm(&self) -> Result<String, KeyError>;

    fn to_json(&self) -> Result<String, KeyError>;
}

pub trait PrivateKey: Key + Send + Sync {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError>;

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError>;
}
