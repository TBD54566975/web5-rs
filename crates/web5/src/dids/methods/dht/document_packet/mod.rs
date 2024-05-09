use crate::crypto::CryptoError;
use crate::jwk::JwkError;
use simple_dns::SimpleDnsError;
use std::collections::HashMap;

use simple_dns::{Packet, ResourceRecord};

use crate::dids::document::{Document, Service, VerificationMethod};

use self::{also_known_as::AlsoKnownAs, controller::Controller, root_record::RootRecord};

mod also_known_as;
mod controller;
mod rdata_encoder;
mod root_record;
mod service;
mod verification_method;

const DEFAULT_TTL: u32 = 7200; // seconds

/// Convert indices from RootRecord and idx_to_vm_id map
/// to get verification method ids for each verification relationship
fn reconstitute_verification_relationship(
    indices: &[u32],
    idx_to_vm_id: &HashMap<u32, String>,
    relationship_name: &str,
) -> Result<Option<Vec<String>>, DocumentPacketError> {
    let methods: Vec<String> = indices.iter()
        .map(|idx| {
            idx_to_vm_id.get(idx)
                .cloned()
                .ok_or_else(|| DocumentPacketError::RootRecord(
                    format!("Root record contains reference to {} that is not found in verification methods", relationship_name)
                ))
        })
        .collect::<Result<Vec<String>, DocumentPacketError>>()?;

    let opt_methods = if methods.is_empty() {
        None
    } else {
        Some(methods)
    };
    Ok(opt_methods)
}

/// Errors that can occur when converting between did:dht documents and DNS packets.
#[derive(thiserror::Error, Debug)]
pub enum DocumentPacketError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error("DID Document is malformed for did:dht: {0}")]
    DocumentError(String),
    #[error(transparent)]
    Dns(#[from] SimpleDnsError),
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("DNS packet was malformed: {0}")]
    RootRecord(String),
    #[error("Could not convert between publicKeyJwk and resource record: {0}")]
    PublicKeyJwk(String),
    #[error("Could not extract fragment from DID url {0}")]
    MissingFragment(String),
    #[error("RData was invalid: {0}")]
    RDataError(String),
}

impl Document {
    pub fn to_dns_packet(&self) -> Result<Packet, DocumentPacketError> {
        // 0. Init root_record and empty answers array
        let did_uri = &self.id;
        let did_id = did_uri
            .split(':')
            .last()
            .ok_or(DocumentPacketError::DocumentError(
                "Malformed id".to_string(),
            ))?;
        let mut root_record = RootRecord::new(did_id);
        let mut answers: Vec<ResourceRecord> = vec![];

        // 1. Add verification methods and verification relationships to root_record and answers
        let mut vm_id_to_idx: HashMap<String, u32> = HashMap::new();

        // 1.1 Add identity key verification method
        let identity_key_id = format!("{}#0", did_uri);
        let id_key_vm = match self.verification_method {
            None => None,
            Some(ref vms) => vms.iter().find(|vm| vm.id == identity_key_id),
        };
        match id_key_vm {
            None => {
                return Err(DocumentPacketError::DocumentError(
                    "Missing identity key verification method with id #0".to_string(),
                ))
            }
            Some(vm) => {
                let idx = 0;

                let vm_record = vm.to_resource_record(did_uri, idx)?.to_owned();
                answers.push(vm_record);
                root_record.vm.push(idx);
                vm_id_to_idx.insert(vm.id.clone(), idx);
            }
        }

        // 1.2 Add all other verification methods
        let mut idx = 1;
        match self.verification_method {
            None => {}
            Some(ref vms) => {
                vms.iter()
                    .try_for_each(|vm| -> Result<(), DocumentPacketError> {
                        // skip identity key because we already added it
                        if vm.id == identity_key_id {
                            return Ok(());
                        }

                        let vm_record = vm.to_resource_record(did_uri, idx)?.to_owned();
                        answers.push(vm_record);
                        root_record.vm.push(idx);
                        vm_id_to_idx.insert(vm.id.clone(), idx);

                        idx += 1;
                        Ok(())
                    })?;
            }
        }

        // 2. Add verification relationships to root_record
        // 2.1 Add assertion methods to root_record
        if let Some(assertion_method) = &self.assertion_method {
            assertion_method.iter().for_each(|am| {
                if let Some(idx) = vm_id_to_idx.get(am) {
                    root_record.asm.push(*idx)
                }
            });
        }

        // 2.2 Add authentication methods to root_record
        if let Some(authentication) = &self.authentication {
            authentication.iter().for_each(|auth| {
                if let Some(idx) = vm_id_to_idx.get(auth) {
                    root_record.auth.push(*idx)
                }
            });
        }

        // 2.3 Add capability delegations to root_record
        if let Some(capability_delegation) = &self.capability_delegation {
            capability_delegation.iter().for_each(|auth| {
                if let Some(idx) = vm_id_to_idx.get(auth) {
                    root_record.del.push(*idx)
                }
            });
        }

        // 2.4 Add capability invocations to root_record
        if let Some(capability_invocation) = &self.capability_invocation {
            capability_invocation.iter().for_each(|inv| {
                if let Some(idx) = vm_id_to_idx.get(inv) {
                    root_record.inv.push(*idx)
                }
            });
        }

        // 2.5 Add key agreements methods to root_record
        if let Some(key_agreement) = &self.key_agreement {
            key_agreement.iter().for_each(|agm| {
                if let Some(idx) = vm_id_to_idx.get(agm) {
                    root_record.agm.push(*idx)
                }
            });
        }

        // 3. Add service records to root_record and answers
        if let Some(service) = &self.service {
            service.iter().enumerate().try_for_each(
                |(idx, src)| -> Result<(), DocumentPacketError> {
                    let idx = idx as u32;
                    let service_record = src.to_resource_record(idx)?;
                    answers.push(service_record);
                    root_record.srv.push(idx);

                    Ok(())
                },
            )?;
        }

        // 4. Add controller to answers
        if let Some(controllers) = &self.controller {
            answers.push(Controller::to_resource_record(controllers)?);
        }

        // 5. Add alsoKnownAs to answers
        if let Some(also_known_as) = &self.also_known_as {
            answers.push(AlsoKnownAs::to_resource_record(also_known_as)?);
        }

        // 6. Create Packet from root_record and answers
        let mut packet = Packet::new_reply(0);
        packet
            .answers
            .push(root_record.to_resource_record()?.into_owned());
        packet.answers.append(&mut answers);

        Ok(packet)
    }

    pub fn from_dns_packet(packet: Packet) -> Result<Document, DocumentPacketError> {
        let answers = &packet.answers;

        // 0. Get root record
        let root_record = answers
            .iter()
            .find(|record| RootRecord::is_root_record(record))
            .ok_or(DocumentPacketError::RootRecord(
                "Root record could not be found".to_string(),
            ))?;
        let root_record = RootRecord::from_resource_record(root_record)?;
        let did_uri = format!("did:dht:{}", root_record.did_id);

        // 1. Reconstitute verification methods
        let mut idx_to_vm_id: HashMap<u32, String> = HashMap::new();
        let mut verification_methods: Vec<VerificationMethod> = vec![];
        for idx in &root_record.vm {
            let record = answers
                .iter()
                .find(|record| VerificationMethod::is_vm_record_with_index(record, idx))
                .ok_or(DocumentPacketError::RootRecord(
                    "Root record contains reference to verification method record that is not found"
                        .to_string(),
                ))?;
            let vm = VerificationMethod::from_resource_record(&did_uri, record, *idx == 0)?;
            idx_to_vm_id.insert(*idx, vm.id.clone());
            verification_methods.push(vm);
        }

        // 2. Reconstitute verification relationships
        // 2.1 Reconstitute assertion methods
        let assertion_method = reconstitute_verification_relationship(
            &root_record.asm,
            &idx_to_vm_id,
            "assertion method",
        )?;

        // 2.2 Reconstitute authentication methods
        let authentication = reconstitute_verification_relationship(
            &root_record.auth,
            &idx_to_vm_id,
            "authentication method",
        )?;

        // 2.3 Reconstitute key agreement
        let key_agreement = reconstitute_verification_relationship(
            &root_record.agm,
            &idx_to_vm_id,
            "key agreement",
        )?;

        // 2.4 Reconstitute capability invocations
        let capability_invocation = reconstitute_verification_relationship(
            &root_record.inv,
            &idx_to_vm_id,
            "capability invocation",
        )?;

        // 2.5 Reconstitute capability delegations
        let capability_delegation = reconstitute_verification_relationship(
            &root_record.del,
            &idx_to_vm_id,
            "capability delegation",
        )?;
        // 3. Reconstitute services
        let mut services: Vec<Service> = vec![];
        for idx in root_record.srv {
            let record = answers
                .iter()
                .find(|record| Service::is_service_record_with_index(record, idx))
                .ok_or(DocumentPacketError::RootRecord(
                    "Root record contains reference to service record that is not found"
                        .to_string(),
                ))?;

            let service = Service::from_resource_record(&did_uri, record)?;
            services.push(service);
        }

        let service = if services.is_empty() {
            None
        } else {
            Some(services)
        };

        // 4. Reconstitute controllers
        let controller = answers
            .iter()
            .find(|record| Controller::is_cnt_record(record));
        let controller: Option<Vec<String>> = match controller {
            None => None,
            Some(cnt) => Some(Controller::from_resource_record(cnt)?),
        };

        // 5. Reconstitute alsoKnownAs
        let also_known_as = answers
            .iter()
            .find(|record| AlsoKnownAs::is_aka_record(record));
        let also_known_as = match also_known_as {
            None => None,
            Some(aka) => Some(AlsoKnownAs::from_resource_record(aka)?),
        };

        // 6. Create document
        Ok(Document {
            id: did_uri,
            context: None,
            controller,
            also_known_as,
            verification_method: Some(verification_methods),
            authentication,
            assertion_method,
            key_agreement,
            capability_invocation,
            capability_delegation,
            service,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::crypto::Curve;
    use crate::keys::key_manager::local_key_manager::LocalKeyManager;
    use crate::keys::key_manager::KeyManager;

    use super::*;

    fn generate_identity_key_vm(did_uri: &str) -> VerificationMethod {
        let key_manager = Arc::new(LocalKeyManager::new());
        let key_alias = key_manager
            .generate_private_key(Curve::Ed25519, Some("0".to_string()))
            .unwrap();
        let public_key = key_manager.get_public_key(&key_alias).unwrap();
        let public_jwk = public_key.jwk().unwrap();

        VerificationMethod {
            id: format!("{}#0", did_uri),
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk: public_jwk.clone(),
        }
    }

    fn generate_additional_vm(did_uri: &str) -> VerificationMethod {
        let key_manager = Arc::new(LocalKeyManager::new());
        let key_alias = key_manager
            .generate_private_key(Curve::Ed25519, Some("0".to_string()))
            .unwrap();
        let public_key = key_manager.get_public_key(&key_alias).unwrap();
        let public_jwk = public_key.jwk().unwrap();

        let thumbprint = public_jwk.compute_thumbprint().unwrap();

        VerificationMethod {
            id: format!("{}#{}", did_uri, thumbprint),
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk: public_jwk.clone(),
        }
    }

    #[test]
    fn test_to_and_from_packet_full_featured() {
        let did_uri = "did:dht:123";

        let verification_method1 = generate_identity_key_vm(did_uri);
        let verification_method2 = generate_additional_vm(did_uri);

        let service = Service {
            id: "did:dht:123#foo".to_string(),
            r#type: "bar".to_string(),
            service_endpoint: vec!["example.com".to_string()],
        };
        let document = Document {
            id: did_uri.to_string(),
            context: None,
            controller: Some(vec!["did:dht:345".to_string(), "did:dht:456".to_string()]),
            also_known_as: Some(vec!["did:dht:567".to_string(), "did:dht:789".to_string()]),
            authentication: Some(vec![
                verification_method2.id.clone(),
                verification_method2.id.clone(),
            ]),
            assertion_method: Some(vec![
                verification_method2.id.clone(),
                verification_method2.id.clone(),
            ]),
            key_agreement: Some(vec![
                verification_method2.id.clone(),
                verification_method2.id.clone(),
            ]),
            capability_invocation: Some(vec![
                verification_method1.id.clone(),
                verification_method2.id.clone(),
            ]),
            capability_delegation: Some(vec![
                verification_method1.id.clone(),
                verification_method2.id.clone(),
            ]),
            service: Some(vec![service]),
            verification_method: Some(vec![verification_method1, verification_method2]),
        };

        let packet = document
            .to_dns_packet()
            .expect("expected to convert document to packet");

        let document2 =
            Document::from_dns_packet(packet).expect("expected to convert back from packet");

        assert_eq!(document, document2);
    }

    #[test]
    fn test_to_and_from_packet() {
        let did_uri = "did:dht:123";

        let verification_method = generate_identity_key_vm(did_uri);
        let document = Document {
            id: did_uri.to_string(),
            verification_method: Some(vec![verification_method]),
            ..Default::default()
        };

        let packet = document
            .to_dns_packet()
            .expect("expected to convert document to packet");

        let document2 =
            Document::from_dns_packet(packet).expect("expected to convert back from packet");

        assert_eq!(document, document2);
    }
}
