use crate::{document::Document, identifier::Identifier};
use jose::jws_signer::{JwsSigner, JwsSignerError};
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
pub enum SignerSelector {
    KeyId(String),
    MethodType(VerificationMethodType),
}

// todo more precise errors
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BearerDidError {
    #[error("verfication method not found")]
    VerificationMethodNotFound,
    #[error(transparent)]
    SignerError(#[from] JwsSignerError),
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error(transparent)]
    KeyError(#[from] KeyError),
}

impl BearerDid {
    // todo support optional selector
    pub fn get_jws_signer(&self, selector: SignerSelector) -> Result<JwsSigner, BearerDidError> {
        let key_id = match selector {
            SignerSelector::KeyId(key_id) => key_id,
            SignerSelector::MethodType(method_type) => {
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

        let identifier =
            Identifier::parse(&key_id).map_err(|_| BearerDidError::VerificationMethodNotFound)?;
        let key_alias = identifier
            .fragment
            .ok_or(BearerDidError::VerificationMethodNotFound)?;

        let public_key = self
            .key_manager
            .get_public_key(&key_alias)?
            .ok_or(KeyManagerError::SigningKeyNotFound)?;
        let algorithm = public_key.algorithm()?;

        let key_manager_clone = self.key_manager.clone();
        let signer_func = Arc::new(move |key_id: &str, message: &[u8]| {
            key_manager_clone
                .sign(key_id, message)
                .map_err(JwsSignerError::from)
        });

        Ok(JwsSigner::new(algorithm, key_alias, signer_func))
    }
}
