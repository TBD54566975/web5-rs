pub mod jwk;

use josekit::{jwk::Jwk, JoseError};
use std::sync::Arc;

/// Enum defining all supported cryptographic key types.
pub enum KeyType {
    Secp256k1,
    Ed25519,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum KeyError {
    // #[error(transparent)]
    // JoseError(#[from] JoseError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Failed to compute key thumbprint")]
    ThumprintFailed,
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
}

pub trait PrivateKey: Key + Send + Sync {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError>;

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError>;
}
