use crate::jwk::Jwk;
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
    #[serde(rename = "verificationMethod", skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<Vec<VerificationMethod>>,
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

pub struct KeyIdFragment(pub String);

impl KeyIdFragment {
    pub fn splice_key_alias(&self) -> String {
        self.0
            .split_once('#')
            .map_or(&self.0[..], |(_, after)| after)
            .to_string()
    }

    pub fn splice_uri(&self) -> String {
        self.0
            .split_once('#')
            .map_or(self.0.clone(), |(base, _)| base.to_string())
    }
}

/// DID Document Service data model as defined in the [web5 spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: Vec<String>,
}

/// DID Document Verification method data model as defined in the [web5 spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model)
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
    KeyId {
        key_id: String,
    },
    MethodType {
        verification_method_type: VerificationMethodType,
    },
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
            KeySelector::KeyId { key_id } => key_id.clone(),
            KeySelector::MethodType {
                verification_method_type,
            } => {
                let get_first_method =
                    |methods: &Option<Vec<String>>| -> Result<String, DocumentError> {
                        methods
                            .as_ref()
                            .ok_or(DocumentError::VerificationMethodNotFound)?
                            .first()
                            .cloned()
                            .ok_or(DocumentError::VerificationMethodNotFound)
                    };

                match verification_method_type {
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
                            .clone()
                            .ok_or(DocumentError::VerificationMethodNotFound)?
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
            .clone()
            .ok_or(DocumentError::VerificationMethodNotFound)?
            .iter()
            .find(|method| method.id == *key_id)
            .cloned()
            .ok_or(DocumentError::VerificationMethodNotFound)?;

        Ok(verification_method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_id_fragment() {
        let fragment = KeyIdFragment("did:example:123#key1".to_string());
        assert_eq!(fragment.splice_key_alias(), "key1".to_string());
        assert_eq!(fragment.splice_uri(), "did:example:123".to_string());

        let fragment = KeyIdFragment("did:example:123".to_string());
        assert_eq!(fragment.splice_key_alias(), "did:example:123".to_string());
        assert_eq!(fragment.splice_uri(), "did:example:123".to_string());
    }

    #[test]
    fn test_get_verification_method() {
        let document = Document {
            id: "did:example:123".to_string(),
            verification_method: Some(vec![
                VerificationMethod {
                    id: "did:example:123#key1".to_string(),
                    r#type: "JsonWebKey2020".to_string(),
                    controller: "did:example:123".to_string(),
                    public_key_jwk: Jwk::default(),
                },
                VerificationMethod {
                    id: "did:example:123#key2".to_string(),
                    r#type: "JsonWebKey2020".to_string(),
                    controller: "did:example:123".to_string(),
                    public_key_jwk: Jwk::default(),
                },
            ]),
            authentication: Some(vec!["did:example:123#key1".to_string()]),
            ..Default::default()
        };

        let key_selector = KeySelector::KeyId {
            key_id: "did:example:123#key1".to_string(),
        };
        let vm = document.get_verification_method(&key_selector).unwrap();
        assert_eq!(vm.id, "did:example:123#key1".to_string());

        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let vm = document.get_verification_method(&key_selector).unwrap();
        assert_eq!(vm.id, "did:example:123#key1".to_string());

        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::Authentication,
        };
        let vm = document.get_verification_method(&key_selector).unwrap();
        assert_eq!(vm.id, "did:example:123#key1".to_string());

        let key_selector = KeySelector::KeyId {
            key_id: "did:example:123#key3".to_string(),
        };
        let err = document.get_verification_method(&key_selector).unwrap_err();
        assert_eq!(err, DocumentError::VerificationMethodNotFound);

        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::AssertionMethod,
        };
        let err = document.get_verification_method(&key_selector).unwrap_err();
        assert_eq!(err, DocumentError::VerificationMethodNotFound);
    }
}
