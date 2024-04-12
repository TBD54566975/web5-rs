use josekit::jwk::Jwk;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    #[serde(rename = "@context")]
    pub context: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<String>>,
    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Vec<String>>,
    #[serde(rename = "assertionMethod", skip_serializing_if = "Option::is_none")]
    pub assertion_method: Option<Vec<String>>,
    #[serde(rename = "keyAgreement", skip_serializing_if = "Option::is_none")]
    pub key_agreement: Option<Vec<String>>,
    #[serde(
        rename = "capabilityInvocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub capability_invocation: Option<Vec<String>>,
    #[serde(
        rename = "capabilityDelegation",
        skip_serializing_if = "Option::is_none"
    )]
    pub capability_delegation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Service>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub controller: String,
    #[serde(rename = "publicKeyJwk")]
    pub public_key_jwk: Jwk,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
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
pub enum DocumentError {
    #[error("verfication method not found")]
    VerificationMethodNotFound,
}

impl Document {
    pub fn get_verification_method(
        &self,
        key_selector: &KeySelector,
    ) -> Result<VerificationMethod, DocumentError> {
        let key_id = match key_selector {
            KeySelector::KeyId(key_id) => key_id.clone(),
            KeySelector::MethodType(method_type) => {
                let get_first_method =
                    |methods: &Option<Vec<String>>| -> Result<String, DocumentError> {
                        methods
                            .as_ref()
                            .ok_or(DocumentError::VerificationMethodNotFound)?
                            .first()
                            .cloned()
                            .ok_or(DocumentError::VerificationMethodNotFound)
                    };

                match method_type {
                    VerificationMethodType::AssertionMethod => {
                        get_first_method(&self.assertion_method)?
                    }
                    VerificationMethodType::Authentication => {
                        get_first_method(&self.authentication)?
                    }
                    VerificationMethodType::CapabilityDelegation => {
                        get_first_method(&self.capability_delegation)?
                    }
                    VerificationMethodType::CapabilityInvocation => {
                        get_first_method(&self.capability_invocation)?
                    }
                    VerificationMethodType::VerificationMethod => {
                        self.verification_method
                            .first()
                            .cloned()
                            .ok_or(DocumentError::VerificationMethodNotFound)?
                            .id
                    }
                }
            }
        };

        let verification_method = self
            .verification_method
            .iter()
            .find(|method| method.id == *key_id)
            .cloned()
            .ok_or(DocumentError::VerificationMethodNotFound)?;

        Ok(verification_method)
    }
}

// todo tests for serialization which enforce web5-spec data types
