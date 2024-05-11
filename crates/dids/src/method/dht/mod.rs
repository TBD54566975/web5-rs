mod bep44;
mod convert;
pub mod registered_types;

use std::sync::Arc;

use crypto::Curve;
use keys::key_manager::KeyManager;
use reqwest::blocking::Client;

use crate::{
    bearer::BearerDid,
    document::{Document, Service, VerificationMethod},
    identifier::Identifier,
};

use self::{
    bep44::encode_bep44_message, convert::document_packet::document_to_packet,
    registered_types::RegisteredDidType,
};

use super::{Create, MethodError};

const DEFAULT_GATEWAY_URL: &str = "https://diddht.tbddev.org";

/// Concrete implementation for a did:dht DID
pub struct DidDht;

/// Options that can be used to create a did:dht DID
pub struct DidDhtCreateOptions {
    pub publish: bool,
    // TODO: gatewayUri - default 'https://diddht.tbddev.org'
    pub also_known_as: Option<Vec<String>>,
    pub controller: Option<Vec<String>>,
    pub service: Option<Vec<Service>>,
    pub registered_type: Option<Vec<RegisteredDidType>>,
    pub verification_methods: Option<Vec<VerificationMethod>>,
}

impl Create<DidDhtCreateOptions> for DidDht {
    fn create(
        key_manager: Arc<dyn KeyManager>,
        options: DidDhtCreateOptions,
    ) -> Result<BearerDid, MethodError> {
        // Generate private and public keypair
        let key_alias = key_manager.generate_private_key(
            Curve::Ed25519,
            Some("0".to_string()),
        )?;
        let public_key = key_manager.get_public_key(&key_alias)?;

        // Create did uri
        let public_jwk = public_key.jwk()?;
        let jwk_string = serde_json::to_string(public_jwk.as_ref()).map_err(|_| {
            MethodError::DidCreationFailure("failed to serialize public jwk".to_string())
        })?;
        let identifier = zbase32::encode_full_bytes(jwk_string.as_bytes());
        let uri = format!("did:dht:{}", identifier);
        let identifier = Identifier::parse(&uri).map_err(|e| {
            MethodError::DidCreationFailure(format!("Failed to parse did:jwk uri {} {}", &uri, e))
        })?;

        // Create did document
        let mut verification_methods = vec![VerificationMethod {
            id: format!("{}#0", uri),
            r#type: "JsonWebKey".to_string(),
            controller: uri.clone(),
            public_key_jwk: public_jwk.as_ref().clone(),
        }];

        if let Some(vms) = options.verification_methods {
            verification_methods.extend(vms);
        }

        let document = Document {
            id: uri.clone(),
            verification_method: verification_methods,
            service: options.service,
            ..Default::default()
        };

        // Publish to gateway
        if options.publish {
            let packet = document_to_packet(&document)?;

            let packet_bytes = packet
                .build_bytes_vec_compressed()
                .map_err(|_| -> MethodError {
                    MethodError::DidPublishingFailure(
                        "Failed to serialize DNS packet to bytes".to_string(),
                    )
                })?;

            let bep44_message = encode_bep44_message(&packet_bytes, |payload| {
                key_manager.sign(&key_alias, &payload)
            })?;

            // HTTP PUT
            let client = Client::new();
            let response = client
                .put(DEFAULT_GATEWAY_URL)
                .header("Content-Type", "application/octet-stream")
                .body(bep44_message)
                .send()
                .map_err(|e| {
                    MethodError::DidPublishingFailure(format!(
                        "Failed to publish did {} to gateway {}: {}",
                        uri, DEFAULT_GATEWAY_URL, e
                    ))
                })?;

            response.error_for_status().map_err(|e| {
                MethodError::DidPublishingFailure(format!(
                    "Failed to publish did {} to gateway {}: {}",
                    uri, DEFAULT_GATEWAY_URL, e
                ))
            })?;
        }

        Ok(BearerDid {
            identifier,
            key_manager,
            document,
        })
    }
}
