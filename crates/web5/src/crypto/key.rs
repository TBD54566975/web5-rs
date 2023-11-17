pub mod private_key;
pub mod public_key;

use ssi_jwk::Error as SpruceJwkError;
use ssi_jws::Error as SpruceJwsError;

pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JwkError(#[from] SpruceJwkError),
    #[error(transparent)]
    JwsError(#[from] SpruceJwsError),
    #[error("Algorithm not found")]
    AlgorithmNotFound,
}

pub trait Key {
    fn alias(&self) -> Result<String, KeyError>;
}
