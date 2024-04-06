pub mod private_key;
pub mod public_key;

use josekit::{JoseError, jwk::Jwk};

/// Enum defining all supported cryptographic key types.
pub enum KeyType {
    Secp256k1,
    // Secp256r1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JoseError(#[from] JoseError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
}

/// Trait defining all common behavior for cryptographic keys.
pub trait Key {
    fn jwk(&self) -> &Jwk;
}
