use jwk::Jwk;

use crate::{CryptoError, CurveOperations};

pub struct Secp256k1;

impl Secp256k1 {
    pub fn generate() -> Result<Jwk, CryptoError> {
        unimplemented!()
    }
}

impl CurveOperations for Secp256k1 {
    fn sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        unimplemented!()
    }

    fn verify(public_jwk: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        unimplemented!()
    }
}
