mod resolver;

use std::sync::Arc;

use crate::{
    crypto::{
        dsa::{ed25519::Ed25519Generator, secp256k1::Secp256k1Generator, Dsa},
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
use resolver::Resolver;
use url::Url;

/// Provides functionality for creating and resolving "did:web" method Decentralized Identifiers (DIDs).
///
/// A "did:web" DID is based on web domain names, using the existing reputation system of the domain.
/// It allows for decentralized identifiers to be hosted on web servers without the need for a blockchain.
///
/// # See Also:
/// [DID Web Specification](https://w3c-ccg.github.io/did-method-web/)
#[derive(Clone)]
pub struct DidWeb;

#[derive(Default)]
pub struct DidWebCreateOptions {
    /// The key manager used for key storage and management. If not provided, an in-memory key manager will be used.
    pub key_manager: Option<Arc<dyn KeyManager>>,

    /// The digital signature algorithm (DSA) used to generate the key (e.g., Ed25519 or Secp256k1). Defaults to Ed25519.
    pub dsa: Option<Dsa>,

    /// Optional services to add to the DID document.
    pub service: Option<Vec<Service>>,

    /// Optional controllers for the DID document.
    pub controller: Option<Vec<String>>,

    /// Optional additional identifiers for the DID document (e.g., social accounts).
    pub also_known_as: Option<Vec<String>>,

    /// Optional additional verification methods for the DID document.
    pub verification_method: Option<Vec<VerificationMethod>>,
}

impl DidWeb {
    /// Creates a new "did:web" DID for a given domain.
    ///
    /// This method generates a "did:web" DID by creating a key pair, using the provided key manager,
    /// and constructing the DID document. The domain must be a valid URL, and additional options
    /// can be provided to customize the DID document.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain for which to create the "did:web" DID.
    /// * `options` - Optional parameters such as key manager, digital signature algorithm, and additional DID document properties.
    ///
    /// # Returns
    ///
    /// * `Result<BearerDid>` - The newly created "did:web" DID, encapsulated in a `BearerDid` object.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let did_web = DidWeb::create("example.com", None)?;
    /// println!("Created DID Web: {:?}", did_web);
    /// ```
    pub fn create(domain: &str, options: Option<DidWebCreateOptions>) -> Result<BearerDid> {
        let options = options.unwrap_or_default();

        let key_manager = options
            .key_manager
            .unwrap_or_else(|| Arc::new(InMemoryKeyManager::new()));

        let private_jwk = match options.dsa.unwrap_or(Dsa::Ed25519) {
            Dsa::Ed25519 => Ed25519Generator::generate(),
            Dsa::Secp256k1 => Secp256k1Generator::generate(),
        };
        let mut public_jwk = key_manager.import_private_jwk(private_jwk)?;
        public_jwk.d = None;

        let domain = &domain.to_string();
        let valid_url = if domain.starts_with("http://") || domain.starts_with("https://") {
            let url = Url::parse(domain)
                .map_err(|e| Web5Error::Parameter(format!("url parse failure {}", e)))?;

            // Ensure "http://" is only allowed for localhost or 127.0.0.1
            if url.scheme() == "http"
                && !(url.host_str() == Some("localhost") || url.host_str() == Some("127.0.0.1"))
            {
                return Err(Web5Error::Parameter(
                    "only https is allowed except for localhost or 127.0.0.1 with http".to_string(),
                ));
            }

            // Get the trimmed URL string without the scheme
            let trimmed_url = url[url::Position::BeforeHost..].to_string();

            // Remove the scheme
            let normalized = if let Some(trimmed) = trimmed_url.strip_prefix("//") {
                trimmed
            } else {
                &trimmed_url
            };

            normalized.to_string()
        } else {
            Url::parse(&format!("https://{}", domain))
                .map_err(|e| Web5Error::Parameter(format!("url parse failure {}", e)))?;
            domain.clone()
        };

        let mut normalized = valid_url.clone();
        if normalized.ends_with('/') {
            normalized = normalized.trim_end_matches('/').to_string()
        }
        if normalized.ends_with("/did.json") {
            normalized = normalized.trim_end_matches("/did.json").to_string()
        }
        if normalized.ends_with("/.well-known") {
            normalized = normalized.trim_end_matches("/.well-known").to_string()
        }

        let encoded_domain = normalized.replace(':', "%3A");
        let encoded_domain = encoded_domain.replace('/', ":");

        let did_uri = format!("did:web:{}", encoded_domain);

        let verification_method = VerificationMethod {
            id: format!("{}#key-0", did_uri),
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.clone(),
            public_key_jwk: public_jwk,
        };

        let document = Document {
            id: did_uri.clone(),
            context: Some(vec!["https://www.w3.org/ns/did/v1".to_string()]),
            verification_method: {
                let mut methods = vec![verification_method.clone()];
                if let Some(mut additional_methods) = options.verification_method {
                    methods.append(&mut additional_methods);
                }
                methods
            },
            service: options.service,
            also_known_as: options.also_known_as,
            controller: options.controller,
            capability_delegation: Some(vec![verification_method.id.clone()]),
            capability_invocation: Some(vec![verification_method.id.clone()]),
            authentication: Some(vec![verification_method.id.clone()]),
            assertion_method: Some(vec![verification_method.id.clone()]),
            ..Default::default()
        };

        Ok(BearerDid {
            did: Did::parse(&did_uri)?,
            document,
            key_manager,
        })
    }

    /// Resolves a "did:web" DID into a `ResolutionResult`.
    ///
    /// This method resolves a DID URI by fetching the DID document from the `.well-known` directory of
    /// the web domain. It parses the DID URI and retrieves the associated DID document from the web server.
    ///
    /// # Arguments
    ///
    /// * `uri` - The DID URI to resolve.
    ///
    /// # Returns
    ///
    /// * `ResolutionResult` - The result of the resolution, containing the DID document and related metadata.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = DidWeb::resolve("did:web:example.com");
    /// println!("Resolved DID Document: {:?}", result.document);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ResolutionMetadataError` if the DID is invalid or cannot be resolved.
    pub async fn resolve(uri: &str) -> ResolutionResult {
        let did = match Did::parse(uri) {
            Ok(did) => did,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        let resolution_result = match Resolver::new(did) {
            Ok(resolver) => resolver.resolve().await,
            Err(e) => return ResolutionResult::from(e),
        };

        match resolution_result {
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
            let result = DidWeb::create(
                "localhost",
                Some(DidWebCreateOptions {
                    key_manager: Some(key_manager.clone()),
                    ..Default::default()
                }),
            );

            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            let result = key_manager.get_signer(public_jwk);
            assert!(result.is_ok())
        }

        #[test]
        fn test_can_specify_secp256k1() {
            let result = DidWeb::create(
                "localhost",
                Some(DidWebCreateOptions {
                    dsa: Some(Dsa::Secp256k1),
                    ..Default::default()
                }),
            );

            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            assert_eq!(public_jwk.alg, Some("ES256K".to_string()));
            assert_eq!(public_jwk.kty, "EC".to_string());
            assert_eq!(public_jwk.crv, "secp256k1".to_string());
        }

        #[test]
        fn test_defaults_to_ed25519() {
            let result = DidWeb::create("localhost", None);
            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            assert_eq!(public_jwk.alg, Some("Ed25519".to_string()));
            assert_eq!(public_jwk.kty, "OKP".to_string());
            assert_eq!(public_jwk.crv, "Ed25519".to_string());
        }

        #[test]
        fn test_invalid_domain() {
            let result = DidWeb::create("invalid domain", None);
            assert!(result.is_err());

            if let Err(Web5Error::Parameter(msg)) = result {
                assert!(msg.contains("url parse failure"));
            } else {
                panic!("Expected Web5Error::Parameter error");
            }
        }

        #[test]
        fn test_should_allow_http_for_localhost() {
            let result = DidWeb::create("http://localhost", None);
            assert!(result.is_ok());

            let result = DidWeb::create("http://127.0.0.1", None);
            assert!(result.is_ok());

            let result = DidWeb::create("http://example.com", None);
            assert!(result.is_err());

            if let Err(Web5Error::Parameter(msg)) = result {
                assert_eq!(
                    msg,
                    "only https is allowed except for localhost or 127.0.0.1 with http"
                );
            } else {
                panic!("Expected Web5Error::Parameter error");
            }
        }

        #[test]
        fn test_must_be_https() {
            let result = DidWeb::create("http://example.com", None);
            assert!(result.is_err());

            if let Err(Web5Error::Parameter(msg)) = result {
                assert_eq!(
                    msg,
                    "only https is allowed except for localhost or 127.0.0.1 with http"
                );
            } else {
                panic!("Expected Web5Error::Parameter error");
            }

            let result = DidWeb::create("https://example.com", None);
            assert!(result.is_ok());
        }

        #[test]
        fn test_should_trim_did_json() {
            let result = DidWeb::create("https://example.com/did.json", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.did.to_string(), "did:web:example.com");
        }

        #[test]
        fn test_should_trim_well_known() {
            let result = DidWeb::create("https://example.com/.well-known/did.json", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.did.to_string(), "did:web:example.com");
        }

        #[test]
        fn test_should_percent_encode_colons() {
            let result = DidWeb::create("https://example.com:8080", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.did.to_string(), "did:web:example.com%3A8080");
        }

        #[test]
        fn test_should_replace_path_with_colons() {
            let result = DidWeb::create("https://example.com/path/to/resource", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(
                did_web.did.to_string(),
                "did:web:example.com:path:to:resource"
            );
        }

        #[test]
        fn test_should_add_optional_verification_methods() {
            let additional_verification_method = VerificationMethod {
                id: "did:web:example.com#key-1".to_string(),
                r#type: "JsonWebKey".to_string(),
                controller: "did:web:example.com".to_string(),
                public_key_jwk: Default::default(),
            };

            let result = DidWeb::create(
                "https://example.com",
                Some(DidWebCreateOptions {
                    verification_method: Some(vec![additional_verification_method.clone()]),
                    ..Default::default()
                }),
            );

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

            let result = DidWeb::create(
                "https://example.com",
                Some(DidWebCreateOptions {
                    service: Some(vec![service.clone()]),
                    ..Default::default()
                }),
            );

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.service.unwrap()[0], service);
        }

        #[test]
        fn test_should_add_optional_also_known_as() {
            let also_known_as = vec!["https://alias.example.com".to_string()];

            let result = DidWeb::create(
                "https://example.com",
                Some(DidWebCreateOptions {
                    also_known_as: Some(also_known_as.clone()),
                    ..Default::default()
                }),
            );

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.also_known_as.unwrap(), also_known_as);
        }

        #[test]
        fn test_should_add_optional_controllers() {
            let controllers = vec!["did:web:controller.example.com".to_string()];

            let result = DidWeb::create(
                "https://example.com",
                Some(DidWebCreateOptions {
                    controller: Some(controllers.clone()),
                    ..Default::default()
                }),
            );

            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.document.controller.unwrap(), controllers);
        }
    }

    mod resolve {
        use super::*;
        use mockito::Server;

        #[tokio::test]
        async fn test_invalid_did() {
            let resolution_result = DidWeb::resolve("something invalid").await;
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InvalidDid)
            )
        }

        #[tokio::test]
        async fn test_create_then_resolve() {
            let mut mock_server = Server::new_async().await;
            let url = mock_server.url();

            let result = DidWeb::create(&url, None);
            assert!(result.is_ok());
            let bearer_did = result.unwrap();

            let _ = mock_server
                .mock("GET", "/.well-known/did.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(&bearer_did.document).unwrap())
                .create();

            let resolution_result = DidWeb::resolve(&bearer_did.did.uri).await;

            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert!(resolution_result.document.is_some());
            let resolved_document = resolution_result.document.unwrap();
            assert_eq!(resolved_document, bearer_did.document);
        }
    }
}
