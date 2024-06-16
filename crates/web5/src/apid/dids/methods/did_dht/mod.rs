use crate::apid::{
    dids::{did::Did, document::Document, resolution_result::ResolutionResult},
    dsa::Signer,
    jwk::Jwk,
};
use std::sync::Arc;

use super::Result;

#[derive(Clone)]
pub struct DidDht {
    pub did: Did,
    pub document: Document,
}

impl DidDht {
    pub fn from_identity_key(identity_key: Jwk) -> Result<Self> {
        unimplemented!()
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidDht::resolve(uri)?;
        match resolution_result.document {
            None => panic!(),
            Some(document) => {
                let identifer = Did::new(uri)?;
                Ok(Self {
                    did: identifer,
                    document,
                })
            }
        }
    }

    pub fn resolve(uri: &str) -> Result<ResolutionResult> {
        unimplemented!()
    }

    pub fn publish(&self, signer: Arc<dyn Signer>) -> Result<()> {
        unimplemented!()
    }

    pub fn deactivate(&self, signer: Arc<dyn Signer>) -> Result<()> {
        unimplemented!()
    }
}
