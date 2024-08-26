use super::{service::Service, verification_method::VerificationMethod};
use crate::errors::{Result, Web5Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
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

pub(crate) struct FindVerificationMethodOptions {
    pub verification_method_id: Option<String>,
}

impl Document {
    pub(crate) fn find_verification_method(
        &self,
        options: FindVerificationMethodOptions,
    ) -> Result<VerificationMethod> {
        let verification_method_id = options.verification_method_id.unwrap_or_default();
        if verification_method_id.is_empty() {
            return Err(Web5Error::Parameter(
                "verification method id cannot be empty".to_string(),
            ));
        }

        for vm in &self.verification_method {
            if vm.id == verification_method_id {
                return Ok(vm.clone());
            }
        }
        Err(Web5Error::NotFound(
            "verification method not found".to_string(),
        ))
    }
}
