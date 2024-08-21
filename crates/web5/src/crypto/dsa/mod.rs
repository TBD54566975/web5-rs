pub mod ed25519;
pub(crate) mod secp256k1;

use base64::DecodeError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DsaError {
    #[error("missing required private key")]
    MissingPrivateKey,
    #[error("base64 decode error {0}")]
    DecodeError(String),
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
    #[error("unsupported curve")]
    UnsupportedDsa,
    #[error("unknown error")]
    Unknown,
}

impl From<DecodeError> for DsaError {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, DsaError>;

pub enum Dsa {
    Ed25519,
    #[cfg(test)]
    Secp256k1,
}

impl std::str::FromStr for Dsa {
    type Err = DsaError;

    fn from_str(input: &str) -> std::result::Result<Self, DsaError> {
        match input.to_ascii_lowercase().as_str() {
            "ed25519" => Ok(Dsa::Ed25519),
            #[cfg(test)]
            "secp256k1" => Ok(Dsa::Secp256k1),
            _ => Err(DsaError::UnsupportedDsa),
        }
    }
}

pub trait Signer: Send + Sync {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;
}

pub trait Verifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool>;
}
