use crate::{
    document::{Document, VerificationMethod},
    identifier::Identifier,
};
use crypto::key_manager::KeyManager;

pub struct BearerDid {
    pub identifier: Identifier,
    pub key_manager: Box<dyn KeyManager>,
    pub document: Document,
}

// todo is this necessary?
// impl std::fmt::Debug for BearerDid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("BearerDid").finish()
//     }
// }

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
pub enum VerificationMethodSelector {
    KeyId(String),
    MethodType(VerificationMethodType),
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BearerDidError {
    #[error("verfication method not found")]
    VerificationMethodNotFound,
}

impl BearerDid {
    pub fn select_verification_method(
        &self,
        selector: VerificationMethodSelector,
    ) -> Result<VerificationMethod, String> {
        let key_id = match selector {
            VerificationMethodSelector::KeyId(key_id) => key_id,
            VerificationMethodSelector::MethodType(method_type) => match method_type {
                VerificationMethodType::AssertionMethod => self
                    .document
                    .assertion_method
                    .as_ref()
                    .ok_or("error")?
                    .first()
                    .ok_or("error")?
                    .to_string(),
                VerificationMethodType::Authentication => self
                    .document
                    .authentication
                    .as_ref()
                    .ok_or("error")?
                    .first()
                    .ok_or("error")?
                    .to_string(),
                VerificationMethodType::CapabilityDelegation => self
                    .document
                    .capability_delegation
                    .as_ref()
                    .ok_or("error")?
                    .first()
                    .ok_or("error")?
                    .to_string(),
                VerificationMethodType::CapabilityInvocation => self
                    .document
                    .capability_invocation
                    .as_ref()
                    .ok_or("error")?
                    .first()
                    .ok_or("error")?
                    .to_string(),
                VerificationMethodType::VerificationMethod => self
                    .document
                    .verification_method
                    .first()
                    .ok_or("err")?
                    .id
                    .clone(),
            },
        };

        let verification_method = self
            .document
            .verification_method
            .iter()
            .find(|vm| vm.id == key_id)
            .cloned()
            .ok_or_else(|| "not found".to_string())?;

        Ok(verification_method)
    }
}
