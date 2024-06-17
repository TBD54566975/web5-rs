use crate::{dids::resolution_result::RcbResolutionResult, errors::RcbResult};
use std::sync::Arc;
use web5::apid::{dids::methods::did_jwk::DidJwk, jwk::Jwk};

pub struct RcbDidJwk(pub DidJwk);

pub fn rcb_did_jwk_resolve(uri: &str) -> RcbResult<Arc<RcbResolutionResult>> {
    let resolution_result = DidJwk::resolve(uri).map_err(|e| Arc::new(e.into()))?;
    Ok(Arc::new(RcbResolutionResult(resolution_result)))
}

impl RcbDidJwk {
    pub fn from_public_jwk(public_key: Jwk) -> RcbResult<Self> {
        let did_jwk = DidJwk::from_public_jwk(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_jwk))
    }

    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let did_jwk = DidJwk::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_jwk))
    }

    pub fn get_data(&self) -> DidJwk {
        self.0.clone()
    }
}
