use crate::{dids::resolution::resolution_result::ResolutionResult, errors::Result};
use std::sync::Arc;
use web5::{crypto::jwk::Jwk, dids::methods::did_jwk::DidJwk as InnerDidJwk};

pub struct DidJwk(pub InnerDidJwk);

pub fn did_jwk_resolve(uri: &str) -> Arc<ResolutionResult> {
    let resolution_result = InnerDidJwk::resolve(uri);
    Arc::new(ResolutionResult(resolution_result))
}

impl DidJwk {
    pub fn from_public_jwk(public_key: Jwk) -> Result<Self> {
        let did_jwk = InnerDidJwk::from_public_jwk(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_jwk))
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let did_jwk = InnerDidJwk::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_jwk))
    }

    pub fn get_data(&self) -> InnerDidJwk {
        self.0.clone()
    }
}
