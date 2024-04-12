pub mod jwk;
// pub mod private_key;
// pub mod public_key;

use std::sync::Arc;

use josekit::jwk::Jwk;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyError {
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("failed to serialize")]
    SerializationFailed,
    #[error("josekit error {0}")]
    JoseError(String),
    #[error("curve not found")]
    CurveNotFound,
    #[error("algorithm not found")]
    AlgorithmNotFound,
    #[error("failed to compute key thumbprint {0}")]
    ThumprintFailed(String),
}

pub enum Curve {
    Secp256k1,
    Ed25519,
}

pub trait Key {
    fn alias(&self) -> Result<String, KeyError>;
    fn jwk(&self) -> Result<Jwk, KeyError>;
}

pub trait PublicKey: Key + Send + Sync {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError>;

    fn algorithm(&self) -> Result<String, KeyError>;

    // todo is this necessary?
    fn to_json(&self) -> Result<String, KeyError>;
}

pub trait PrivateKey: Key + Send + Sync {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError>;

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError>;
}
