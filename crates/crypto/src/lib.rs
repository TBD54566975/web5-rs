pub mod ed25519;
pub mod secp256k1;

use base64::DecodeError;
use jwk::Jwk;
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum CryptoError {
    #[error("missing required private key")]
    MissingPrivateKey,
    #[error(transparent)]
    DecodeError(#[from] DecodeError),
    #[error("invalid key length {0}")]
    InvalidKeyLength(String),
    #[error("invalid signature length {0}")]
    InvalidSignatureLength(String),
    #[error("public key failure {0}")]
    PublicKeyFailure(String),
    #[error("private key failure {0}")]
    PrivateKeyFailure(String),
    #[error("verification failure {0}")]
    VerificationFailure(String),
    #[error("sign failure {0}")]
    SignFailure(String),
}

pub enum Curve {
    Secp256k1,
    Ed25519,
}

pub type Signer = Arc<dyn Fn(&[u8]) -> Result<Vec<u8>, CryptoError> + Send + Sync>;

pub trait CurveOperations {
    fn generate() -> Result<Jwk, CryptoError>;
    fn sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify(public_key: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError>;
}
