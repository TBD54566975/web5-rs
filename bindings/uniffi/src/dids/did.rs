use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::{
    dids::{
        did::Did as InnerDid,
        methods::{
            did_dht::DidDht as InnerDidDht, did_jwk::DidJwk as InnerDidJwk,
            did_web::DidWeb as InnerDidWeb,
        },
        resolution_result::ResolutionResult as InnerResolutionResult,
    },
    dsa::Signer,
    jwk::Jwk,
};

pub struct RcbDid(InnerDid);

impl RcbDid {
    pub fn new(uri: &str) -> RcbResult<Self> {
        let inner = InnerDid::new(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self { 0: inner })
    }

    pub fn get_data(&self) -> InnerDid {
        self.0.clone()
    }
}
