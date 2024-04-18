use crypto::{ed25519::Ed25199, CryptoError, CurveOperations};
use jwk::Jwk;

pub fn ed25519_generate() -> Result<Jwk, CryptoError> {
    Ed25199::generate()
}

pub fn ed25519_sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Ed25199::sign(private_jwk, payload)
}

pub fn ed25519_verify(public_jwk: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
    Ed25199::verify(public_jwk, payload, signature)
}
