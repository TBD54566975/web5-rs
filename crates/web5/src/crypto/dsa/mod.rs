pub mod ed25519;
pub(crate) mod secp256k1;

use base64::DecodeError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DsaError {
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
    #[error("Unsupported curve")]
    UnsupportedCurve,
}

type Result<T> = std::result::Result<T, DsaError>;

pub enum Dsa {
    Ed25519,
}

pub trait Signer: Send + Sync {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;
}

pub trait Verifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool>;
}
