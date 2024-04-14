use crate::{
    document::{Document, DocumentError, KeySelector},
    identifier::Identifier,
};
use crypto::Signer;
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
}

impl BearerDid {
    pub fn get_signer(
        &self,
        key_selector: &KeySelector,
    ) -> Result<Signer, BearerDidError> {
        let verification_method = self.document.get_verification_method(key_selector)?;
        let key_id = &verification_method.id;

        let key_alias = key_id
            .split_once('#')
            .map_or(&key_id[..], |(_, after)| after);
        let signer = self.key_manager.get_signer(key_alias)?;

        Ok(signer)
    }
}
