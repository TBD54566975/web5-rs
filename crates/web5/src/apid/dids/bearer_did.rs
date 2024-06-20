use super::{
    data_model::{document::Document, DataModelError as DidDataModelError},
    did::{Did, DidError},
    resolution::{
        resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
    },
};
use crate::apid::{
    crypto::key_managers::{key_manager::KeyManager, KeyManagerError},
    dsa::Signer,
};
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BearerDidError {
    #[error(transparent)]
    DidError(#[from] DidError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionMetadataError),
    #[error(transparent)]
    DidDataModelError(#[from] DidDataModelError),
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
}

type Result<T> = std::result::Result<T, BearerDidError>;

#[derive(Clone)]
pub struct BearerDid {
    pub did: Did,
    pub document: Document,
    pub key_manager: Arc<dyn KeyManager>,
}

impl BearerDid {
    pub fn new(uri: &str, key_manager: Arc<dyn KeyManager>) -> Result<Self> {
        let resolution_result = ResolutionResult::new(uri);

        match resolution_result.document {
            None => Err(match resolution_result.resolution_metadata.error {
                None => BearerDidError::ResolutionError(ResolutionMetadataError::InternalError),
                Some(e) => BearerDidError::ResolutionError(e),
            }),
            Some(document) => {
                let did = Did::new(uri)?;
                Ok(Self {
                    did,
                    document,
                    key_manager,
                })
            }
        }
    }

    pub fn get_signer(&self, key_id: String) -> Result<Arc<dyn Signer>> {
        let public_jwk = self.document.find_public_key_jwk(key_id)?;
        Ok(self.key_manager.get_signer(public_jwk)?)
    }
}
