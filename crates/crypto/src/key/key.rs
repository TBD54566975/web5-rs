use ssi_jwk::JWK;
use ssi_jws::Error as JWSError;

/// Enum defining all supported cryptographic algorithms.
pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JWSError(#[from] JWSError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
}

/// Trait defining all common behavior for cryptographic keys.
pub trait Key {
    fn jwk(&self) -> &JWK;
}
