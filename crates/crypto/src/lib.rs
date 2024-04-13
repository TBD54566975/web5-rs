pub mod ed25519;
pub mod secp256k1;

use base64::DecodeError;
use ed25519::Ed25199;
use jwk::Jwk;
use secp256k1::Secp256k1;

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
    #[error("verification failure {0}")]
    VerificationFailure(String)
}

pub enum Curve {
    Secp256k1,
    Ed25519,
}

pub struct JwkGenerator;

impl JwkGenerator {
    pub fn generate_jwk(curve: Curve) -> Result<Jwk, CryptoError> {
        match curve {
            Curve::Ed25519 => Ed25199::generate(),
            Curve::Secp256k1 => Secp256k1::generate(),
        }
    }
}

pub trait CurveOperations {
    fn sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify(public_key: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError>;
}
