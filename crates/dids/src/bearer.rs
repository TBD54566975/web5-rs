use crate::{
    document::{Document, DocumentError, KeyIdFragment, KeySelector},
    identifier::{Identifier, IdentifierError},
    resolver::{ResolutionError, Resolver},
};
use keys::{
    key::KeyError,
    key_manager::{KeyManager, KeyManagerError},
};
use std::sync::Arc;

pub struct BearerDid {
    pub identifier: Identifier,
    pub key_manager: Arc<dyn KeyManager>,
    pub document: Document,
}

// todo more precise errors
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BearerDidError {
    #[error("verfication method not found")]
    VerificationMethodNotFound,
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),
    #[error(transparent)]
    IdentifierError(#[from] IdentifierError),
}

impl BearerDid {
    pub async fn from_key_manager(
        did_uri: &str,
        key_manager: Arc<dyn KeyManager>,
    ) -> Result<Self, BearerDidError> {
        let resolution_result = Resolver::resolve_uri(did_uri).await;
        if let Some(err) = resolution_result.did_resolution_metadata.error {
            return Err(err)?;
        }

        Ok(BearerDid {
            identifier: Identifier::parse(did_uri)?,
            key_manager,
            document: resolution_result
                .did_document
                .ok_or(ResolutionError::NotFound)?,
        })
    }

    pub fn sign(&self, key_selector: &KeySelector, payload: &[u8]) -> Result<Vec<u8>, BearerDidError> {
        let verification_method = self.document.get_verification_method(key_selector)?;
        let key_alias = KeyIdFragment(verification_method.id.clone()).splice_key_alias();
        let signature = self.key_manager.sign(&key_alias, payload)?;
        Ok(signature)
    }
}
