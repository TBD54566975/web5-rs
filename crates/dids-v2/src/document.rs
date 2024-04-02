use crypto::key::public_key::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub controller: String,
    #[serde(rename = "publicKeyJwk")]
    pub public_key_jwk: PublicKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationRelationship {
    AssertionMethod,
    Authentication,
    CapabilityDelegation,
    CapabilityInvocation,
    KeyAgreement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationMethodSelector {
    ID(String),
    Relationship(VerificationRelationship),
}

impl Document {
    pub fn add_verification_method(
        &mut self,
        method: VerificationMethod,
        relationships: &[VerificationRelationship],
    ) {
        self.verification_method.push(method.clone());

        for relationship in relationships {
            match relationship {
                VerificationRelationship::AssertionMethod => {
                    self.assertion_method
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                VerificationRelationship::Authentication => {
                    self.authentication
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                VerificationRelationship::KeyAgreement => {
                    self.key_agreement
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                VerificationRelationship::CapabilityDelegation => {
                    self.capability_delegation
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
                VerificationRelationship::CapabilityInvocation => {
                    self.capability_invocation
                        .get_or_insert_with(Vec::new)
                        .push(method.id.clone());
                }
            }
        }
    }

    pub fn select_verification_method(
        &self,
        selector: Option<VerificationMethodSelector>,
    ) -> Result<VerificationMethod, String> {
        if self.verification_method.is_empty() {
            return Err("no verification methods found".to_string());
        }

        match selector {
            Some(VerificationMethodSelector::ID(id)) => self
                .verification_method
                .iter()
                .find(|vm| vm.id == id)
                .cloned()
                .ok_or_else(|| format!("no verification method found for id: {}", id)),
            Some(VerificationMethodSelector::Relationship(relationship)) => {
                let vm_id = match relationship {
                    VerificationRelationship::AssertionMethod => self
                        .assertion_method
                        .as_ref()
                        .and_then(|v| v.first())
                        .map(|x| x.to_string()),
                    VerificationRelationship::Authentication => self
                        .authentication
                        .as_ref()
                        .and_then(|v| v.first())
                        .map(|x| x.to_string()),
                    VerificationRelationship::CapabilityDelegation => self
                        .capability_delegation
                        .as_ref()
                        .and_then(|v| v.first())
                        .map(|x| x.to_string()),
                    VerificationRelationship::CapabilityInvocation => self
                        .capability_invocation
                        .as_ref()
                        .and_then(|v| v.first())
                        .map(|x| x.to_string()),
                    VerificationRelationship::KeyAgreement => self
                        .key_agreement
                        .as_ref()
                        .and_then(|v| v.first())
                        .map(|x| x.to_string()),
                };

                match vm_id {
                    Some(id) => self
                        .verification_method
                        .iter()
                        .find(|vm| vm.id == id)
                        .cloned()
                        .ok_or_else(|| {
                            format!(
                                "no verification method found for verification relationship: {:?}",
                                relationship
                            )
                        }),
                    None => Err(format!(
                        "no verification method found for verification relationship: {:?}",
                        relationship
                    )),
                }
            }
            None => Ok(self.verification_method[0].clone()),
        }
    }

    pub fn add_service(&mut self, service: Service) {
        if let Some(services) = &mut self.service {
            services.push(service);
        } else {
            self.service = Some(vec![service]);
        }
    }

    pub fn get_absolute_resource_id(&self, id: &str) -> String {
        if id.starts_with('#') {
            format!("{}{}", self.id, id)
        } else {
            id.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::{key::KeyType, key_manager::local_key_manager::LocalKeyManager};
    use crypto::key_manager::KeyManager;

    #[test]
    fn test_add_verification_method() {
        let mut doc = Document {
            id: "did:example:123".to_string(),
            verification_method: vec![],
            ..Default::default()
        };

        let key_manager = LocalKeyManager::new_in_memory();
        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();
        let public_key = key_manager
            .get_public_key(&key_alias)
            .expect("KeyManagerError occurred")
            .expect("PublicKey not found");

        let method = VerificationMethod {
            id: "did:example:123#key1".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: public_key,
        };

        doc.add_verification_method(
            method.clone(),
            &[
                VerificationRelationship::AssertionMethod,
                VerificationRelationship::Authentication,
            ],
        );

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
        let key_manager = LocalKeyManager::new_in_memory();
        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();
        let public_key = key_manager
            .get_public_key(&key_alias)
            .expect("KeyManagerError occurred")
            .expect("PublicKey not found");
        let method1 = VerificationMethod {
            id: "did:example:123#key1".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: public_key,
        };

        let key_manager = LocalKeyManager::new_in_memory();
        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();
        let public_key = key_manager
            .get_public_key(&key_alias)
            .expect("KeyManagerError occurred")
            .expect("PublicKey not found");
        let method2 = VerificationMethod {
            id: "did:example:123#key2".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: public_key,
        };

        let doc = Document {
            id: "did:example:123".to_string(),
            verification_method: vec![method1.clone(), method2.clone()],
            assertion_method: Some(vec![method1.id.clone()]),
            authentication: Some(vec![method2.id.clone()]),
            ..Default::default()
        };

        let selector = VerificationMethodSelector::ID("did:example:123#key1".to_string());
        let selected_method = doc.select_verification_method(Some(selector)).unwrap();
        assert_eq!(selected_method, method1);

        let selector =
            VerificationMethodSelector::Relationship(VerificationRelationship::AssertionMethod);
        let selected_method = doc.select_verification_method(Some(selector)).unwrap();
        assert_eq!(selected_method, method1);

        let selector =
            VerificationMethodSelector::Relationship(VerificationRelationship::Authentication);
        let selected_method = doc.select_verification_method(Some(selector)).unwrap();
        assert_eq!(selected_method, method2);

        let selected_method = doc.select_verification_method(None).unwrap();
        assert_eq!(selected_method, method1);

        let selector = VerificationMethodSelector::ID("did:example:123#key3".to_string());
        let result = doc.select_verification_method(Some(selector));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "no verification method found for id: did:example:123#key3"
        );

        let selector =
            VerificationMethodSelector::Relationship(VerificationRelationship::KeyAgreement);
        let result = doc.select_verification_method(Some(selector));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "no verification method found for verification relationship: KeyAgreement"
        );
    }

    #[test]
    fn test_add_service() {
        let mut doc = Document {
            id: "did:example:123".to_string(),
            service: None,
            ..Default::default()
        };

        let service1 = Service {
            id: "did:example:123#service1".to_string(),
            r#type: "ExampleService".to_string(),
            service_endpoint: "https://example.com/service1".to_string(),
        };

        let service2 = Service {
            id: "did:example:123#service2".to_string(),
            r#type: "AnotherService".to_string(),
            service_endpoint: "https://example.com/service2".to_string(),
        };

        doc.add_service(service1.clone());
        assert!(doc.service.is_some());
        assert_eq!(doc.service.as_ref().unwrap().len(), 1);
        assert_eq!(doc.service.as_ref().unwrap()[0], service1);

        doc.add_service(service2.clone());
        assert!(doc.service.is_some());
        assert_eq!(doc.service.as_ref().unwrap().len(), 2);
        assert_eq!(doc.service.as_ref().unwrap()[0], service1);
        assert_eq!(doc.service.as_ref().unwrap()[1], service2);
    }

    #[test]
    fn test_get_absolute_resource_id() {
        let doc = Document {
            id: "did:example:123".to_string(),
            ..Default::default()
        };

        let relative_id = "#key1";
        let absolute_id = doc.get_absolute_resource_id(relative_id);
        assert_eq!(absolute_id, "did:example:123#key1");

        let absolute_id = "did:example:456#key2";
        let result = doc.get_absolute_resource_id(absolute_id);
        assert_eq!(result, "did:example:456#key2");
    }
}
