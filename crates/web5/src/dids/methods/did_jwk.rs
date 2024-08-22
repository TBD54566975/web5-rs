use crate::{
    crypto::{
        dsa::{ed25519::Ed25519Generator, secp256k1::Secp256k1Generator, Dsa},
        jwk::Jwk,
        key_managers::{in_memory_key_manager::InMemoryKeyManager, key_manager::KeyManager},
    },
    dids::{
        bearer_did::BearerDid,
        data_model::{document::Document, verification_method::VerificationMethod},
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    errors::Result,
};
use base64::{engine::general_purpose, Engine as _};
use std::sync::Arc;

#[derive(Default)]
pub struct CreateOptions {
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub dsa: Option<Dsa>,
}

pub struct DidJwk;

impl DidJwk {
    pub fn create(options: Option<CreateOptions>) -> Result<BearerDid> {
        let options = options.unwrap_or_default();

        let key_manager = match options.key_manager {
            Some(km) => km,
            None => Arc::new(InMemoryKeyManager::new()),
        };

        let private_jwk = match options.dsa {
            None => Ed25519Generator::generate(),
            Some(dsa) => match dsa {
                Dsa::Ed25519 => Ed25519Generator::generate(),
                Dsa::Secp256k1 => Secp256k1Generator::generate(),
            },
        };
        let mut public_jwk = private_jwk.clone();
        public_jwk.d = None;

        let jwk_string = serde_json::to_string(&public_jwk)?;
        let method_specific_id = general_purpose::URL_SAFE_NO_PAD.encode(jwk_string);

        let did_uri = format!("did:jwk:{}", method_specific_id);

        let did = Did::parse(&did_uri)?;

        let verification_method_id = format!("{}#0", did_uri);

        let document = Document {
            id: did_uri.clone(),
            verification_method: vec![VerificationMethod {
                id: verification_method_id.clone(),
                r#type: "JsonWebKey".to_string(),
                controller: did_uri.clone(),
                public_key_jwk: public_jwk.clone(),
            }],
            authentication: Some(vec![verification_method_id.clone()]),
            assertion_method: Some(vec![verification_method_id.clone()]),
            capability_invocation: Some(vec![verification_method_id.clone()]),
            capability_delegation: Some(vec![verification_method_id.clone()]),
            ..Default::default()
        };

        Ok(BearerDid {
            did,
            document,
            key_manager,
        })
    }

    pub fn resolve(uri: &str) -> ResolutionResult {
        let did = match Did::parse(uri) {
            Ok(d) => d,
            Err(_) => return ResolutionResult::from_error(ResolutionMetadataError::InvalidDid),
        };

        let decoded_jwk = match general_purpose::URL_SAFE_NO_PAD.decode(did.id) {
            Ok(dj) => dj,
            Err(_) => return ResolutionResult::from_error(ResolutionMetadataError::InvalidDid),
        };

        let public_jwk = match serde_json::from_slice::<Jwk>(&decoded_jwk) {
            Ok(pj) => pj,
            Err(_) => return ResolutionResult::from_error(ResolutionMetadataError::InvalidDid),
        };

        let kid = format!("{}#0", did.uri);
        let document = Document {
            context: Some(vec!["https://www.w3.org/ns/did/v1".to_string()]),
            id: did.uri.clone(),
            verification_method: vec![VerificationMethod {
                id: kid.clone(),
                r#type: "JsonWebKey".to_string(),
                controller: did.uri.clone(),
                public_key_jwk: public_jwk,
            }],
            assertion_method: Some(vec![kid.clone()]),
            authentication: Some(vec![kid.clone()]),
            capability_invocation: Some(vec![kid.clone()]),
            capability_delegation: Some(vec![kid.clone()]),

            // TODO: https://github.com/TBD54566975/web5-rs/issues/257 - If the JWK contains a `use` property with the value "sig" then the `keyAgreement` property
            // is not included in the DID Document. If the `use` value is "enc" then only the `keyAgreement`
            // property is included in the DID Document.
            // key_agreement: if public_jwk.use_.as_deref() != Some("sig") { Some(vec![kid.clone()]) } else { None },
            ..Default::default()
        };

        ResolutionResult {
            document: Some(document),
            ..Default::default()
        }
    }
}
