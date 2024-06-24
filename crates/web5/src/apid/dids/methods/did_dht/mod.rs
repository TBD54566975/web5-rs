use base64::{engine::general_purpose, Engine as _};
use bep44::Bep44Message;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use reqwest::blocking::Client;
use simple_dns::Packet;

use super::{MethodError, Result};
use crate::{
    apid::{
        crypto::jwk::Jwk,
        dids::{
            data_model::{document::Document, verification_method::VerificationMethod},
            did::Did,
            resolution::{
                resolution_metadata::{ResolutionMetadata, ResolutionMetadataError},
                resolution_result::ResolutionResult,
            },
        },
        dsa::Signer,
    },
    crypto::ed25519::Ed25519,
};
use std::sync::Arc;

pub mod bep44;
pub mod document_packet;

const JSON_WEB_KEY: &str = "JsonWebKey";
const DEFAULT_RELAY: &str = "https://diddht.tbddev.org";

fn extract_ed25519_public_key(jwk: &Jwk) -> Result<Vec<u8>> {
    let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;
    if decoded_x.len() != PUBLIC_KEY_LENGTH {
        return Err(MethodError::DidCreationFailure(format!(
            "Identity key JWK public key must have length {}",
            PUBLIC_KEY_LENGTH
        )));
    }
    Ok(decoded_x)
}

fn create_identifier(identity_key_jwk: &Jwk) -> Result<String> {
    let pubkey_bytes = extract_ed25519_public_key(identity_key_jwk)?;
    let suffix = zbase32::encode_full_bytes(&pubkey_bytes);
    Ok(format!("did:dht:{}", suffix))
}

#[derive(Clone, Default)]
pub struct DidDht {
    pub did: Did,
    pub document: Document,
}

impl DidDht {
    pub fn from_identity_key(identity_key: Jwk) -> Result<Self> {
        println!("DidDht::from_identity_key() called");
        let did_uri = create_identifier(&identity_key)?;
        let identity_key_verification_method = VerificationMethod {
            id: format!("{}#0", &did_uri),
            r#type: JSON_WEB_KEY.to_string(),
            controller: did_uri.clone(),
            public_key_jwk: identity_key,
        };

        let capability_delegation = vec![identity_key_verification_method.id.clone()];
        let capability_invocation = vec![identity_key_verification_method.id.clone()];
        let authentication = vec![identity_key_verification_method.id.clone()];
        let assertion_method = vec![identity_key_verification_method.id.clone()];
        let key_agreement = vec![];
        let verification_methods = vec![identity_key_verification_method];

        // TODO maybe add additional verification methods and verification purposes
        // if let Some(additional_verification_methods) = additional_verification_methods {
        //     for vm_opts in additional_verification_methods {
        //         let verification_method = VerificationMethod {
        //             id: format!("{}#{}", did_uri, &vm_opts.public_key.compute_thumbprint().unwrap()), // TODO: don't unwrap
        //             r#type: JSON_WEB_KEY.to_string(),
        //             controller: "foo".to_string(),
        //             public_key_jwk: vm_opts.public_key,
        //         };

        //         for purpose in vm_opts.purposes {
        //             match purpose {
        //                 VerificationPurposes::Authentication => authentication.push(verification_method.id.clone()),
        //                 VerificationPurposes::AssertionMethod => assertion_method.push(verification_method.id.clone()),
        //                 VerificationPurposes::CapabilityInvocation => capability_invocation.push(verification_method.id.clone()),
        //                 VerificationPurposes::CapabilityDelegation => capability_delegation.push(verification_method.id.clone()),
        //                 VerificationPurposes::KeyAgreement => key_agreement.push(verification_method.id.clone()),
        //             }
        //         }

        //         verification_methods.push(verification_method);
        //     }
        // }

        Ok(Self {
            did: Did::new(&did_uri)?,
            document: Document {
                id: did_uri.clone(),
                verification_method: verification_methods,
                capability_delegation: Some(capability_delegation),
                capability_invocation: Some(capability_invocation),
                authentication: Some(authentication),
                assertion_method: Some(assertion_method),
                key_agreement: Some(key_agreement),
                ..Default::default()
            },
        })
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidDht::resolve(uri)?;
        match resolution_result.document {
            None => Err(match resolution_result.resolution_metadata.error {
                None => MethodError::ResolutionError(ResolutionMetadataError::InternalError),
                Some(e) => MethodError::ResolutionError(e),
            }),
            Some(document) => {
                let identifer = Did::new(uri)?;
                Ok(Self {
                    did: identifer,
                    document,
                })
            }
        }
    }

    pub fn resolve(uri: &str) -> Result<ResolutionResult> {
        // check did method and decode id
        let did = Did::new(uri)?;
        if did.method != "dht" {
            return Ok(ResolutionResult {
                resolution_metadata: ResolutionMetadata {
                    error: Some(ResolutionMetadataError::MethodNotSupported),
                },
                ..Default::default()
            });
        }
        let identity_key = zbase32::decode_full_bytes_str(&did.id)
            .map_err(|_| ResolutionMetadataError::InvalidDid)?;
        let identity_key = Ed25519::from_public_key(&identity_key)
            .map_err(|_| ResolutionMetadataError::InvalidDid)?;

        // construct http endpoint from gateway url and last part of did_uri
        let url = format!(
            "{}/{}",
            DEFAULT_RELAY.trim_end_matches('/'),
            did.id.trim_start_matches('/')
        );

        let client = Client::new();

        // Make the GET request
        let response = client
            .get(url)
            .send()
            .map_err(|_| ResolutionMetadataError::InternalError)?;

        // Check if the status is not 200
        let status = response.status();
        if status == 404 {
            return Err(ResolutionMetadataError::NotFound)?;
        } else if status != 200 {
            return Err(ResolutionMetadataError::InternalError)?;
        }

        // check http response status is 200 and body is nonempty
        let body = response
            .bytes()
            .map_err(|_| ResolutionMetadataError::NotFound)?;

        // Check if the body is empty
        if body.is_empty() {
            return Err(ResolutionMetadataError::NotFound)?;
        }

        // bep44 decode and verify response body bytes
        let body: Vec<u8> = body.into();
        let bep44_message =
            Bep44Message::decode(&body).map_err(|_| ResolutionMetadataError::InvalidDidDocument)?;
        bep44_message
            .verify(&identity_key)
            .map_err(|_| ResolutionMetadataError::InvalidDidDocument)?;

        // convert bep44 decoded value from DNS packet to did doc
        let packet = Packet::parse(&bep44_message.v)
            .map_err(|_| ResolutionMetadataError::InvalidDidDocument)?;
        let document: Document = packet
            .try_into()
            .map_err(|_| ResolutionMetadataError::InvalidDidDocument)?;

        Ok(ResolutionResult {
            document: Some(document),
            ..Default::default()
        })
    }

    pub fn publish(&self, _signer: Arc<dyn Signer>) -> Result<()> {
        println!("DidDht.publish() called");
        Ok(())
    }

    pub fn deactivate(&self, _signer: Arc<dyn Signer>) -> Result<()> {
        println!("DidDht.deactivate() called");
        Ok(())
    }
}
