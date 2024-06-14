use crate::apid::{
    dids::{document::Document, identifier::Identifier, resolution_result::ResolutionResult},
    dsa::Signer,
    jwk::Jwk,
};
use std::sync::Arc;

pub struct DidDht {
    pub did: Identifier,
    pub document: Document,
}

impl DidDht {
    pub fn from_identity_key(identity_key: Jwk) -> Self {
        unimplemented!()
    }

    pub fn from_uri(uri: &str) -> Self {
        let resolution_result = DidDht::resolve(uri);
        match resolution_result.document {
            None => panic!(),
            Some(document) => {
                let identifer = Identifier::new(uri).unwrap();
                Self {
                    did: identifer,
                    document,
                }
            }
        }
    }

    pub fn resolve(uri: &str) -> ResolutionResult {
        unimplemented!()
    }

    pub fn publish(&self, signer: Arc<dyn Signer>) {
        unimplemented!()
    }

    pub fn deactivate(&self, signer: Arc<dyn Signer>) {
        unimplemented!()
    }
}
