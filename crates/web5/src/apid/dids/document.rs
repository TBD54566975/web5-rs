use crate::apid::jwk::Jwk;
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
    pub service_endpoint: Vec<String>,
}
