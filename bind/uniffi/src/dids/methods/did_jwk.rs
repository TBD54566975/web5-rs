use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::{dids::methods::did_jwk::DidJwk, jwk::Jwk};

pub struct RcbDidJwk(DidJwk);

impl RcbDidJwk {
    pub fn from_public_jwk(public_key: Jwk) -> RcbResult<Self> {
        let did_jwk = DidJwk::from_public_jwk(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_jwk))
    }

    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let did_jwk = DidJwk::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_jwk))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn get_data(&self) -> DidJwk {
        self.0.clone()
    }
}
