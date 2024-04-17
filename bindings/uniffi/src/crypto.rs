use crate::Jwk;
use crypto::{ed25519::Ed25199 as InternalEd25199, CryptoError, CurveOperations};
use jwk::Jwk as InternalJwk;
use std::sync::Arc;

pub struct Ed25199 {}

impl Ed25199 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) -> Result<Arc<Jwk>, CryptoError> {
        let jwk = InternalEd25199::generate()?;
        Ok(Arc::new(Jwk(jwk)))
    }

    pub fn sign(&self, private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        InternalEd25199::sign(&InternalJwk::from(private_jwk), payload)
    }

    pub fn verify(&self, public_jwk: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        InternalEd25199::verify(&InternalJwk::from(public_jwk), payload, signature)
    }
}
