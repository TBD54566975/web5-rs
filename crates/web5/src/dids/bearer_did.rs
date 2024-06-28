use super::{
    data_model::{document::Document, DataModelError as DidDataModelError},
    did::{Did, DidError},
    portable_did::PortableDid,
    resolution::{
        resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
    },
};
use crate::crypto::{
    dsa::Signer,
    key_managers::{
        in_memory_key_manager::InMemoryKeyManager, key_manager::KeyManager, KeyManagerError,
    },
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

    pub fn from_portable_did(portable_did: PortableDid) -> Result<Self> {
        let did = Did::new(&portable_did.did_uri)?;

        let key_manager = Arc::new(InMemoryKeyManager::new());
        for private_jwk in portable_did.private_jwks {
            key_manager.import_private_jwk(private_jwk)?;
        }

        Ok(Self {
            did,
            document: portable_did.document,
            key_manager,
        })
    }

    pub fn get_signer(&self, key_id: String) -> Result<Arc<dyn Signer>> {
        let public_jwk = self.document.find_public_key_jwk(key_id)?;
        Ok(self.key_manager.get_signer(public_jwk)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::key_managers::in_memory_key_manager::InMemoryKeyManager;

    #[test]
    fn can_instantiate_did_jwk() {
        let did_uri = "did:jwk:eyJrdHkiOiJPS1AiLCJ1c2UiOiJzaWciLCJjcnYiOiJFZDI1NTE5Iiwia2lkIjoiVnRTSFhQbEtEdzFFRW9PajVYTjNYV2hqU1BZVk52WC1lNHZqUk8weVlKQSIsIngiOiJpejcwc3ZTTHhOWmhzRHhlSlFfam5PVmJYM0tGTmtjQmNNaldqWm1YRXNBIiwiYWxnIjoiRWREU0EifQ";
        let key_manager = InMemoryKeyManager::new();

        let bearer_did = BearerDid::new(did_uri, Arc::new(key_manager)).unwrap();

        assert_eq!(did_uri, bearer_did.document.id);
    }
}
