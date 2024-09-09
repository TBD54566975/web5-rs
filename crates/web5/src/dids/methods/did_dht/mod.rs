use bep44::Bep44Message;
use reqwest::blocking::Client;
use simple_dns::Packet;

use crate::{
    crypto::{
        dsa::ed25519::{self, Ed25519Generator, Ed25519Verifier},
        jwk::Jwk,
        key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
    },
    dids::{
        bearer_did::BearerDid,
        data_model::{
            document::Document, service::Service, verification_method::VerificationMethod,
        },
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    errors::{Result, Web5Error},
};
use std::sync::Arc;

mod bep44;
mod document_packet;

const JSON_WEB_KEY: &str = "JsonWebKey";
const DEFAULT_RELAY: &str = "https://diddht.tbddev.org";

fn create_identifier(identity_key_jwk: &Jwk) -> Result<String> {
    let pubkey_bytes = ed25519::public_jwk_extract_bytes(identity_key_jwk)?;
    let suffix = zbase32::encode_full_bytes(&pubkey_bytes);
    Ok(format!("did:dht:{}", suffix))
}

#[derive(Clone, Default)]
pub struct DidDht;

#[derive(Default)]
pub struct DidDhtCreateOptions {
    pub publish: Option<bool>,
    pub gateway_url: Option<String>,
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub service: Option<Vec<Service>>,
    pub controller: Option<Vec<String>>,
    pub also_known_as: Option<Vec<String>>,
    pub verification_method: Option<Vec<VerificationMethod>>,
}

impl DidDht {
    pub fn create(options: Option<DidDhtCreateOptions>) -> Result<BearerDid> {
        let options = options.unwrap_or_default();

        let key_manager = options
            .key_manager
            .unwrap_or_else(|| Arc::new(InMemoryKeyManager::new()));

        let private_jwk = Ed25519Generator::generate();
        let identity_jwk = key_manager.import_private_jwk(private_jwk)?;

        let did_uri = create_identifier(&identity_jwk)?;
        let identity_key_verification_method = VerificationMethod {
            id: format!("{}#0", &did_uri),
            r#type: JSON_WEB_KEY.to_string(),
            controller: did_uri.clone(),
            public_key_jwk: identity_jwk,
        };

        let did = Did::parse(&did_uri)?;
        let document = Document {
            id: did_uri.clone(),
            service: options.service,
            also_known_as: options.also_known_as,
            controller: options.controller,
            verification_method: {
                let mut methods = vec![identity_key_verification_method.clone()];
                if let Some(mut additional_methods) = options.verification_method {
                    methods.append(&mut additional_methods);
                }
                methods
            },
            capability_delegation: Some(vec![identity_key_verification_method.id.clone()]),
            capability_invocation: Some(vec![identity_key_verification_method.id.clone()]),
            authentication: Some(vec![identity_key_verification_method.id.clone()]),
            assertion_method: Some(vec![identity_key_verification_method.id.clone()]),
            ..Default::default()
        };

        let bearer_did = BearerDid {
            did,
            document,
            key_manager,
        };

        if options.publish.unwrap_or(true) {
            DidDht::publish(bearer_did.clone(), options.gateway_url)?;
        }

        Ok(bearer_did)
    }

    pub fn publish(bearer_did: BearerDid, gateway_url: Option<String>) -> Result<()> {
        let packet = bearer_did.document.to_packet().map_err(|e| {
            Web5Error::Encoding(format!("failed to convert document to packet {}", e))
        })?;

        let packet_bytes = packet
            .build_bytes_vec()
            .map_err(|_| Web5Error::Encoding("failed to serialize packet as bytes".to_string()))?;

        let public_jwk = bearer_did.document.verification_method[0]
            .public_key_jwk
            .clone();
        let signer = bearer_did.key_manager.get_signer(public_jwk)?;
        let bep44_message = Bep44Message::new(&packet_bytes, |payload| signer.sign(&payload))
            .map_err(|_| {
                Web5Error::Encoding("failed to convert packet bytes to bep44 message".to_string())
            })?;

        let body = bep44_message.encode().map_err(|_| {
            Web5Error::Encoding("failed to serialize bep44 message as bytes".to_string())
        })?;

        let url = format!(
            "{}/{}",
            gateway_url
                .unwrap_or_else(|| DEFAULT_RELAY.to_string())
                .trim_end_matches('/'),
            bearer_did.did.id.trim_start_matches('/')
        );

        let client = Client::new();
        let response = client
            .put(url)
            .header("Content-Type", "application/octet-stream")
            .body(body)
            .send()
            .map_err(|_| Web5Error::Network("failed to publish DID to mainline".to_string()))?;

        if response.status() != 200 {
            return Err(Web5Error::Network(
                "failed to PUT DID to mainline".to_string(),
            ));
        }

        Ok(())
    }

    pub fn resolve(uri: &str, gateway_url: Option<String>) -> ResolutionResult {
        let result: std::result::Result<ResolutionResult, ResolutionMetadataError> = (|| {
            // check did method and decode id
            let did = Did::parse(uri).map_err(|_| ResolutionMetadataError::InvalidDid)?;
            if did.method != "dht" {
                return Ok(ResolutionResult::from(
                    ResolutionMetadataError::MethodNotSupported,
                ));
            }
            let identity_key = zbase32::decode_full_bytes_str(&did.id)
                .map_err(|_| ResolutionMetadataError::InvalidPublicKey)?;
            let identity_key = ed25519::public_jwk_from_bytes(&identity_key)
                .map_err(|_| ResolutionMetadataError::InvalidPublicKey)?;

            // construct http endpoint from gateway url and last part of did_uri
            let url = format!(
                "{}/{}",
                gateway_url
                    .unwrap_or_else(|| DEFAULT_RELAY.to_string())
                    .trim_end_matches('/'),
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
            let bep44_message = Bep44Message::decode(&body)
                .map_err(|_| ResolutionMetadataError::InvalidDidDocument)?;
            bep44_message
                .verify(&Ed25519Verifier::new(identity_key))
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
        })();

        match result {
            Ok(resolution_result) => resolution_result,
            Err(e) => ResolutionResult::from(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    mod create {
        use super::*;

        #[test]
        fn test_can_specify_key_manager() {

            let key_manager = Arc::new(InMemoryKeyManager::new());
            let result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                key_manager: Some(key_manager.clone()),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            let result = key_manager.get_signer(public_jwk);
            assert!(result.is_ok())
        }

        #[test]
        fn test_can_specify_publish_and_gateway_url() {
            
            let mut mock_server = mockito::Server::new();
            let gateway_url = mock_server.url();

            let mock = mock_server
                .mock("PUT", mockito::Matcher::Any)
                .expect(1)
                .with_status(200)
                .with_header("content-type", "application/octet-stream")
                .create();

            let result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(true),
                gateway_url: Some(gateway_url.clone()), // Use the mock server's URL
                ..Default::default()
            }));

            assert!(result.is_ok());

            mock.assert();
        }

        #[test]
        fn test_should_add_optional_verification_methods() {
            
            let additional_verification_method = VerificationMethod {
                id: "did:web:example.com#key-1".to_string(),
                r#type: "JsonWebKey".to_string(),
                controller: "did:web:example.com".to_string(),
                public_key_jwk: Ed25519Generator::generate(),
            };

            let result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                verification_method: Some(vec![additional_verification_method.clone()]),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.verification_method.len(), 2);
            assert_eq!(
                did_web.document.verification_method[1],
                additional_verification_method
            );
        }

        #[test]
        fn test_should_add_optional_services() {
            
            let service = Service {
                id: "did:web:example.com#service-0".to_string(),
                r#type: "SomeService".to_string(),
                service_endpoint: vec!["https://example.com/service".to_string()],
            };

            let result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                service: Some(vec![service.clone()]),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.service.unwrap()[0], service);
        }

        #[test]
        fn test_should_add_optional_also_known_as() {
            
            let also_known_as = vec!["https://alias.example.com".to_string()];

            let result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                also_known_as: Some(also_known_as.clone()),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.also_known_as.unwrap(), also_known_as);
        }

        #[test]
        fn test_should_add_optional_controllers() {
            
            let controllers = vec!["did:web:controller.example.com".to_string()];

            let result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                controller: Some(controllers.clone()),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.controller.unwrap(), controllers);
        }
    }

    mod publish {
        use super::*;

        #[test]
        fn test_can_specify_gateway_url() {

            let mut mock_server = mockito::Server::new();
            let gateway_url = mock_server.url();

            let mock = mock_server
                .mock("PUT", mockito::Matcher::Any)
                .expect(1)
                .with_status(200)
                .with_header("content-type", "application/octet-stream")
                .create();

            let bearer_did = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                ..Default::default()
            }))
            .unwrap();

            let result = DidDht::publish(
                bearer_did,
                Some(gateway_url.clone()), // Use the mock server's URL
            );

            assert!(result.is_ok());

            mock.assert();
        }

        #[test]
        fn test_can_handle_network_error() {
            
            let mut mock_server = mockito::Server::new();
            let gateway_url = mock_server.url();

            let mock = mock_server
                .mock("PUT", mockito::Matcher::Any)
                .expect(1)
                .with_status(500)
                .with_header("content-type", "application/octet-stream")
                .create();

            let bearer_did = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                ..Default::default()
            }))
            .unwrap();

            let result = DidDht::publish(bearer_did, Some(gateway_url));

            assert!(result.is_err());
            if let Err(Web5Error::Network(msg)) = result {
                assert_eq!(msg, "failed to PUT DID to mainline");
            } else {
                panic!("expected Web5Error::Network error");
            }

            mock.assert();
        }
    }

    mod resolve {
        use std::sync::Mutex;

        use super::*;

        #[test]
        fn test_invalid_did() {

            let resolution_result = DidDht::resolve("something invalid", None);
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InvalidDid)
            )
        }

        #[test]
        fn test_method_not_supported() {
            
            let resolution_result = DidDht::resolve("did:web:example", None);
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::MethodNotSupported)
            )
        }

        #[test]
        fn test_not_found() {
            
            let bearer_did = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                ..Default::default()
            }))
            .unwrap();

            let mut mock_server = mockito::Server::new();
            let gateway_url = mock_server.url();

            let mock = mock_server
                .mock("GET", format!("/{}", bearer_did.did.id).as_str())
                .expect(1)
                .with_status(404)
                .with_header("content-type", "application/octet-stream")
                .create();

            let resolution_result = DidDht::resolve(&bearer_did.did.uri, Some(gateway_url));
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::NotFound)
            );

            mock.assert();
        }

        #[test]
        fn test_internal_error() {
            
            let bearer_did = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(false),
                ..Default::default()
            }))
            .unwrap();

            let mut mock_server = mockito::Server::new();
            let gateway_url = mock_server.url();

            let mock = mock_server
                .mock("GET", format!("/{}", bearer_did.did.id).as_str())
                .expect(1)
                .with_status(500)
                .with_header("content-type", "application/octet-stream")
                .create();

            let resolution_result = DidDht::resolve(&bearer_did.did.uri, Some(gateway_url));
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InternalError)
            );

            mock.assert();
        }

        #[test]
        fn test_can_create_then_resolve() {
            
            let mut mock_server = mockito::Server::new();
            let gateway_url = mock_server.url();

            let published_body = Arc::new(Mutex::new(Vec::new()));
            let published_body_clone = Arc::clone(&published_body);

            let mock_publish = mock_server
                .mock("PUT", mockito::Matcher::Any)
                .expect(1)
                .with_status(200)
                .with_header("content-type", "application/octet-stream")
                .with_body_from_request(move |request| {
                    let mut body = published_body_clone.lock().unwrap();
                    *body = request.body().unwrap().to_vec();
                    vec![] // Return an empty response body
                })
                .create();

            let create_result = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(true),
                gateway_url: Some(gateway_url),
                ..Default::default()
            }));
            assert!(create_result.is_ok());

            let bearer_did = create_result.unwrap();

            let stored_body = published_body.lock().unwrap();

            let mock_resolve = mock_server
                .mock("GET", format!("/{}", bearer_did.did.id).as_str())
                .expect(1)
                .with_status(200)
                .with_header("content-type", "application/octet-stream")
                .with_body(stored_body.clone()) // Use the captured body as the response
                .create();

            let resolution_result = DidDht::resolve(&bearer_did.did.uri, Some(mock_server.url()));

            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert!(resolution_result.document.is_some());
            let resolved_document = resolution_result.document.unwrap();
            assert_eq!(resolved_document, bearer_did.document);

            mock_publish.assert();
            mock_resolve.assert();
        }
    }
}
