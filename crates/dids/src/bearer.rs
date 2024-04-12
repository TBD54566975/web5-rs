use crate::{
    document::{Document, VerificationMethod},
    identifier::Identifier,
};
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
    pub fn get_verification_method(
        &self,
        key_selector: &KeySelector,
    ) -> Result<VerificationMethod, BearerDidError> {
        let key_id = match key_selector {
            KeySelector::KeyId(key_id) => key_id.clone(),
            KeySelector::MethodType(method_type) => {
                let get_first_method =
                    |methods: &Option<Vec<String>>| -> Result<String, BearerDidError> {
                        methods
                            .as_ref()
                            .ok_or(BearerDidError::VerificationMethodNotFound)?
                            .first()
                            .cloned()
                            .ok_or(BearerDidError::VerificationMethodNotFound)
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
                    VerificationMethodType::VerificationMethod => {
                        self.document
                            .verification_method
                            .first()
                            .cloned()
                            .ok_or(BearerDidError::VerificationMethodNotFound)?
                            .id
                    }
                }
            }
        };

        let verification_method = self
            .document
            .verification_method
            .iter()
            .find(|method| method.id == *key_id)
            .cloned()
            .ok_or(BearerDidError::VerificationMethodNotFound)?;

        Ok(verification_method)
    }

    pub fn get_jws_signer(
        &self,
        key_selector: &KeySelector,
    ) -> Result<Arc<dyn JwsSigner>, BearerDidError> {
        let verification_method = self.get_verification_method(key_selector)?;
        let key_id = &verification_method.id;

        let key_alias = key_id
            .split_once('#')
            .map_or(&key_id[..], |(_, after)| after);
        let signer = self.key_manager.get_jws_signer(key_alias)?;

        Ok(signer)
    }
}
