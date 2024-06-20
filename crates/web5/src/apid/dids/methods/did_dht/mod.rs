use crate::apid::{
    crypto::jwk::Jwk,
    dids::{
        data_model::document::Document,
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    dsa::Signer,
};
use std::sync::Arc;

use super::{MethodError, Result};

#[derive(Clone, Default)]
pub struct DidDht {
    pub did: Did,
    pub document: Document,
}

impl DidDht {
    pub fn from_identity_key(_identity_key: Jwk) -> Result<Self> {
        println!("DidDht::from_identity_key() called");
        Ok(Self {
            ..Default::default()
        })
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidDht::resolve(uri)?;
        match resolution_result.document {
            None => Err(match resolution_result.resolution_metadata.error {
                None => MethodError::ResolutionError(ResolutionMetadataError::InternalError),
                Some(e) => MethodError::ResolutionError(e),
            }),
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
        println!("DidDht::resolve() called with {}", uri);
        Ok(ResolutionResult {
            ..Default::default()
        })
    }

    pub fn publish(&self, _signer: Arc<dyn Signer>) -> Result<()> {
        println!("DidDht.publish() called");
        Ok(())
    }

    pub fn deactivate(&self, _signer: Arc<dyn Signer>) -> Result<()> {
        println!("DidDht.deactivate() called");
        Ok(())
    }
}
