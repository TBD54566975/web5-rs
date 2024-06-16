use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::{dids::methods::did_jwk::DidJwk as InnerDidJwk, jwk::Jwk};

pub struct RcbDidJwk(InnerDidJwk);

impl RcbDidJwk {
    pub fn from_public_jwk(public_key: Jwk) -> RcbResult<Self> {
        let inner = InnerDidJwk::from_public_jwk(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(inner))
    }

    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let inner = InnerDidJwk::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(inner))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn get_data(&self) -> InnerDidJwk {
        self.0.clone()
    }
}
