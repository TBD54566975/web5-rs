use bep44::Bep44Message;
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
    http::get_http_client,
};
use std::{collections::HashMap, sync::Arc};

mod bep44;
mod document_packet;

const JSON_WEB_KEY: &str = "JsonWebKey";
const DEFAULT_RELAY: &str = "https://diddht.tbddev.org";

fn create_identifier(identity_key_jwk: &Jwk) -> Result<String> {
    let pubkey_bytes = ed25519::public_jwk_extract_bytes(identity_key_jwk)?;
    let suffix = zbase32::encode_full_bytes(&pubkey_bytes);
    Ok(format!("did:dht:{}", suffix))
}

/// Provides functionality for creating and resolving "did:dht" method Decentralized Identifiers (DIDs).
///
/// A "did:dht" DID is derived from an identity key and is stored on a Distributed Hash Table (DHT).
/// The method-specific identifier for "did:dht" is a z-base-32 encoded public key.
///
/// # See Also:
/// [DID DHT Specification](https://did-dht.com/)
#[derive(Clone, Default)]
pub struct DidDht;

#[derive(Default)]
pub struct DidDhtCreateOptions {
    /// Determines whether to publish the DID document to the DHT after creation. Defaults to true.
    pub publish: Option<bool>,

    /// The URL of the gateway to use for publishing or resolving the DID.
    pub gateway_url: Option<String>,

    /// The key manager used for key storage and management. If not provided, an in-memory key manager will be used.
    pub key_manager: Option<Arc<dyn KeyManager>>,

    /// Optional services to add to the DID document.
    pub service: Option<Vec<Service>>,

    /// Optional controllers for the DID document.
    pub controller: Option<Vec<String>>,

    /// Optional additional identifiers for the DID document (e.g., social accounts).
    pub also_known_as: Option<Vec<String>>,

    /// Optional additional verification methods for the DID document.
    pub verification_method: Option<Vec<VerificationMethod>>,
}

impl DidDht {
    /// Creates a new "did:dht" DID, derived from an identity key.
    ///
    /// This method generates a "did:dht" DID by creating a key pair using Ed25519, and constructs a DID document.
    /// The method-specific identifier is a z-base-32 encoded public key.
    ///
    /// By default, the DID document is published to the DHT, unless the `publish` option is set to false.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters such as key manager, services, controllers, and whether to publish the DID.
    ///
    /// # Returns
    ///
    /// * `Result<BearerDid>` - The newly created "did:dht" DID, encapsulated in a `BearerDid` object.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let did_dht = DidDht::create(None)?;
    /// println!("Created DID DHT: {:?}", did_dht);
    /// ```
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

    /// Publishes a "did:dht" DID document to the DHT.
    ///
    /// This method converts the DID document into a packet, signs it using the associated key, and
    /// publishes the signed packet to the DHT using the provided gateway URL or the default gateway.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The `BearerDid` object representing the DID to be published.
    /// * `gateway_url` - The URL of the gateway to use for publishing. If not provided, the default gateway is used.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns `Ok` if the DID is successfully published, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let bearer_did = DidDht::create(None)?;
    /// DidDht::publish(bearer_did, None)?;
    /// ```
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

        let headers: HashMap<String, String> = HashMap::from([
            ("Host".to_string(), "{}".to_string()),
            ("Connection".to_string(), "close".to_string()),
            ("Content-Length".to_string(), "{}".to_string()),
            ("Content-Type".to_string(), "application/octet-stream".to_string())
        ]);

        let response = get_http_client().put(&url, Some(headers), &body)
            .map_err(|e| Web5Error::Network(format!("Failed to PUT did:dht: {}", e)))?;

        if response.status_code != 200 {
            return Err(Web5Error::Network(
                "failed to PUT DID to mainline".to_string(),
            ));
        }

        Ok(())
    }

    /// Resolves a "did:dht" DID into a `ResolutionResult`.
    ///
    /// This method retrieves the DID document associated with the "did:dht" DID from the DHT.
    /// It sends a GET request to the DHT gateway, verifies the document, and constructs the result.
    ///
    /// # Arguments
    ///
    /// * `uri` - The DID URI to resolve.
    /// * `gateway_url` - The URL of the gateway to use for resolution. If not provided, the default gateway is used.
    ///
    /// # Returns
    ///
    /// * `ResolutionResult` - The result of the resolution, containing the DID document and related metadata.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = DidDht::resolve("did:dht:example", None);
    /// println!("Resolved DID Document: {:?}", result.document);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ResolutionMetadataError` if the DID cannot be resolved or verified.
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

            let headers: HashMap<String, String> = HashMap::from([
                ("Host".to_string(), "{}".to_string()),
                ("Connection".to_string(), "close".to_string()),
                ("Accept".to_string(), "application/octet-stream".to_string())
            ]);
            let response = get_http_client().get(&url, Some(headers)).map_err(|_| ResolutionMetadataError::InternalError)?;

            if response.status_code == 404 {
                return Err(ResolutionMetadataError::NotFound);
            } else if response.status_code != 200 {
                return Err(ResolutionMetadataError::InternalError);
            }

            // bep44 decode and verify response body bytes
            let bep44_message = Bep44Message::decode(&response.body)
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
