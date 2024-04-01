use jwk::jwk::JWK;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Purpose {
    AssertionMethod,
    Authentication,
    CapabilityDelegation,
    CapabilityInvocation,
    KeyAgreement,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_jwk: Option<JWK>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub r#type: String,
    pub service_endpoint: String,
}

// https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: String,

    #[serde(rename = "@context")]
    pub context: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Vec<String>>,

    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,

    #[serde(rename = "verificationMethod", skip_serializing_if = "Vec::is_empty")]
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

impl Document {
    pub fn add_verification_method(&mut self, method: VerificationMethod, purposes: &[Purpose]) {
        self.verification_method.push(method.clone());

        for purpose in purposes {
            match purpose {
                Purpose::AssertionMethod => {
                    self.assertion_method
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                Purpose::Authentication => {
                    self.authentication
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                Purpose::KeyAgreement => {
                    self.key_agreement
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                Purpose::CapabilityDelegation => {
                    self.capability_delegation
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                Purpose::CapabilityInvocation => {
                    self.capability_invocation
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_verification_method() {
        let mut doc = Document {
            id: "did:example:123".to_string(),
            verification_method: vec![],
            ..Default::default()
        };

        let method = VerificationMethod {
            id: "did:example:123#key1".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            ..Default::default()
        };

        let purposes = &[Purpose::AssertionMethod, Purpose::Authentication];

        doc.add_verification_method(method.clone(), purposes);

        assert_eq!(doc.verification_method.len(), 1);
        assert_eq!(doc.verification_method[0], method);

        assert!(doc.assertion_method.is_some());
        assert_eq!(doc.assertion_method.as_ref().unwrap().len(), 1);
        assert_eq!(doc.assertion_method.as_ref().unwrap()[0], method.id);

        assert!(doc.authentication.is_some());
        assert_eq!(doc.authentication.as_ref().unwrap().len(), 1);
        assert_eq!(doc.authentication.as_ref().unwrap()[0], method.id);

        assert!(doc.key_agreement.is_none());
        assert!(doc.capability_invocation.is_none());
        assert!(doc.capability_delegation.is_none());
    }
}
