use crate::{dids::resolution::resolution_result::ResolutionResult, errors::ResultOld};
use std::sync::Arc;
use web5::{crypto::jwk::Jwk, dids::methods::did_jwk::DidJwk as InnerDidJwk};

pub struct DidJwk(pub InnerDidJwk);

pub fn did_jwk_resolve(uri: &str) -> Arc<ResolutionResult> {
    let resolution_result = InnerDidJwk::resolve(uri);
    Arc::new(ResolutionResult(resolution_result))
}

impl DidJwk {
    pub fn from_public_jwk(public_key: Jwk) -> ResultOld<Self> {
        let did_jwk = InnerDidJwk::from_public_jwk(public_key)?;
        Ok(Self(did_jwk))
    }

    pub fn from_uri(uri: &str) -> ResultOld<Self> {
        let did_jwk = InnerDidJwk::from_uri(uri)?;
        Ok(Self(did_jwk))
    }

    pub fn get_data(&self) -> InnerDidJwk {
        self.0.clone()
    }
}
