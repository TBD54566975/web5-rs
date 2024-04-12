use crate::{document::Document, identifier::Identifier};
use josekit::jws::JwsSigner;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationMethodType {
    VerificationMethod,
    AssertionMethod,
    Authentication,
    CapabilityDelegation,
    CapabilityInvocation,
}

// Define an enum to encapsulate the selection criteria
#[derive(Debug, Clone, PartialEq)]
pub enum KeySelector {
    KeyId(String),
    MethodType(VerificationMethodType),
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
}

impl BearerDid {
    // todo 
    // pub fn get_verification_method(&self, key_selector: KeySelector) -> 

    pub fn get_jws_signer(
        &self,
        key_selector: KeySelector,
    ) -> Result<Arc<dyn JwsSigner>, BearerDidError> {
        let key_id = match key_selector {
            KeySelector::KeyId(key_id) => key_id,
            KeySelector::MethodType(method_type) => {
                let get_first_method =
                    |methods: &Option<Vec<String>>| -> Result<String, BearerDidError> {
                        methods
                            .as_ref()
                            .ok_or(BearerDidError::VerificationMethodNotFound)?
                            .first()
                            .ok_or(BearerDidError::VerificationMethodNotFound)
                            .map(|s| s.to_string())
                    };

                match method_type {
                    VerificationMethodType::AssertionMethod => {
                        get_first_method(&self.document.assertion_method)?
                    }
                    VerificationMethodType::Authentication => {
                        get_first_method(&self.document.authentication)?
                    }
                    VerificationMethodType::CapabilityDelegation => {
                        get_first_method(&self.document.capability_delegation)?
                    }
                    VerificationMethodType::CapabilityInvocation => {
                        get_first_method(&self.document.capability_invocation)?
                    }
                    VerificationMethodType::VerificationMethod => self
                        .document
                        .verification_method
                        .first()
                        .ok_or(BearerDidError::VerificationMethodNotFound)?
                        .id
                        .clone(),
                }
            }
        };

        let signer = self.key_manager.get_jws_signer(&key_id)?;

        Ok(signer)
    }
}
