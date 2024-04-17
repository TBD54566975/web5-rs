use crate::Jwk;
use crypto::{ed25519::Ed25199 as InternalEd25199, CryptoError, CurveOperations};
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
}
