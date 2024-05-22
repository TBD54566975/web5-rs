pub mod ed25519;
pub mod secp256k1;

use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use base64::DecodeError;
use jwk::Jwk;

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
    #[error("Unsupported curve")]
    UnsupportedCurve,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Curve {
    Secp256k1,
    Ed25519,
}

impl FromStr for Curve {
    type Err = CryptoError;

    fn from_str(input: &str) -> Result<Self, CryptoError> {
        match input {
            "Ed25519" => Ok(Curve::Ed25519),
            "secp256k1" => Ok(Curve::Secp256k1),
            _ => Err(CryptoError::UnsupportedCurve),
        }
    }
}

impl Display for Curve {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Curve::Ed25519 => write!(f, "Ed25519"),
            Curve::Secp256k1 => write!(f, "secp256k1"),
        }
    }
}

pub trait CurveOperations {
    fn generate() -> Result<Jwk, CryptoError>;
    fn sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify(public_key: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_from_str() {
        let input = "Ed25519";
        let curve = Curve::from_str(input);
        assert_eq!(Ok(Curve::Ed25519), curve);
        let display = curve.unwrap().to_string();
        assert_eq!(display, input);

        let input = "secp256k1";
        let curve = Curve::from_str(input);
        assert_eq!(Ok(Curve::Secp256k1), curve);
        let display = curve.unwrap().to_string();
        assert_eq!(display, input);

        let input = "1234";
        let curve = Curve::from_str(input);
        assert_eq!(Err(CryptoError::UnsupportedCurve), curve);
    }
}
