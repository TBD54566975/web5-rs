use jwk::jwk::JWK;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Purpose {
    AssertionMethod,
    Authentication,
    CapabilityDelegation,
    CapabilityInvocation,
    KeyAgreement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_jwk: JWK,
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

pub trait VMSelector {
    fn select(&self, doc: &Document) -> Result<String, String>;
}

impl VMSelector for Purpose {
    fn select(&self, doc: &Document) -> Result<String, String> {
        match self {
            Purpose::AssertionMethod => doc
                .assertion_method
                .as_ref()
                .and_then(|v| v.first())
                .map(|x| x.to_string())
                .ok_or_else(|| format!("no verification method found for purpose: {:?}", self)),
            Purpose::Authentication => doc
                .authentication
                .as_ref()
                .and_then(|v| v.first())
                .map(|x| x.to_string())
                .ok_or_else(|| format!("no {:?} verification method found", self)),
            Purpose::CapabilityDelegation => doc
                .capability_delegation
                .as_ref()
                .and_then(|v| v.first())
                .map(|x| x.to_string())
                .ok_or_else(|| format!("no {:?} verification method found", self)),
            Purpose::CapabilityInvocation => doc
                .capability_invocation
                .as_ref()
                .and_then(|v| v.first())
                .map(|x| x.to_string())
                .ok_or_else(|| format!("no {:?} verification method found", self)),
            Purpose::KeyAgreement => doc
                .key_agreement
                .as_ref()
                .and_then(|v| v.first())
                .map(|x| x.to_string())
                .ok_or_else(|| format!("no {:?} verification method found", self)),
        }
    }
}

impl VMSelector for &str {
    fn select(&self, _doc: &Document) -> Result<String, String> {
        Ok(self.to_string())
    }
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

    pub fn select_verification_method(
        &self,
        selector: Option<&dyn VMSelector>,
    ) -> Result<VerificationMethod, String> {
        if self.verification_method.is_empty() {
            return Err("no verification methods found".to_string());
        }

        if let Some(selector) = selector {
            let vm_id = selector.select(self)?;

            self.verification_method
                .iter()
                .find(|vm| vm.id == vm_id)
                .cloned()
                .ok_or_else(|| format!("no verification method found for id: {}", vm_id))
        } else {
            Ok(self.verification_method[0].clone())
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
            public_key_jwk: JWK {
                alg: Some("".to_string()),
                kty: Some("EC".to_string()),
                crv: Some("secp256k1".to_string()),
                d: Some("".to_string()),
                x: Some("IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY".to_string()),
                y: Some("UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA".to_string()),
            },
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

    #[test]
    fn test_select_verification_method() {
        let method1 = VerificationMethod {
            id: "did:example:123#key1".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: JWK {
                alg: Some("".to_string()),
                kty: Some("EC".to_string()),
                crv: Some("secp256k1".to_string()),
                d: Some("".to_string()),
                x: Some("IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY".to_string()),
                y: Some("UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA".to_string()),
            },
        };

        let method2 = VerificationMethod {
            id: "did:example:123#key2".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: JWK {
                alg: Some("".to_string()),
                kty: Some("EC".to_string()),
                crv: Some("secp256k1".to_string()),
                d: Some("".to_string()),
                x: Some("IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY".to_string()),
                y: Some("UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA".to_string()),
            },
        };

        let doc = Document {
            id: "did:example:123".to_string(),
            verification_method: vec![method1.clone(), method2.clone()],
            assertion_method: Some(vec![method1.id.clone()]),
            authentication: Some(vec![method2.id.clone()]),
            ..Default::default()
        };

        // Test selecting a verification method by ID
        let selected_method = doc
            .select_verification_method(Some(&"did:example:123#key1"))
            .unwrap();
        assert_eq!(selected_method, method1);

        // Test selecting a verification method by Purpose
        let selected_method = doc
            .select_verification_method(Some(&Purpose::AssertionMethod))
            .unwrap();
        assert_eq!(selected_method, method1);

        let selected_method = doc
            .select_verification_method(Some(&Purpose::Authentication))
            .unwrap();
        assert_eq!(selected_method, method2);

        // Test selecting the first verification method when no selector is provided
        let selected_method = doc.select_verification_method(None).unwrap();
        assert_eq!(selected_method, method1);

        // Test selecting a non-existent verification method
        let result = doc.select_verification_method(Some(&"did:example:123#key3"));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "no verification method found for id: did:example:123#key3"
        );

        // Test selecting a verification method for a non-existent purpose
        let result = doc.select_verification_method(Some(&Purpose::KeyAgreement));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "no KeyAgreement verification method found"
        );
    }
}
