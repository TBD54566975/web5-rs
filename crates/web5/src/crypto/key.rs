mod private;
mod public;

pub use private::*;
pub use public::*;

use ssi_jwk::{Error as JwkError, JWK as Jwk};
use ssi_jws::Error as JwsError;

pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error(transparent)]
    JwsError(#[from] JwsError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
}

pub trait Key {
    fn jwk(&self) -> &Jwk;
    fn alias(&self) -> Result<String, KeyError> {
        Ok(self.jwk().thumbprint()?)
    }
}
