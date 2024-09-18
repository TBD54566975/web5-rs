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

#[derive(Clone)]
pub struct DidWeb;

#[derive(Default)]
pub struct DidWebCreateOptions {
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub dsa: Option<Dsa>,
    pub service: Option<Vec<Service>>,
    pub controller: Option<Vec<String>>,
    pub also_known_as: Option<Vec<String>>,
    pub verification_method: Option<Vec<VerificationMethod>>,
}

impl DidWeb {
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
                let mut methods = vec![verification_method];
                if let Some(mut additional_methods) = options.verification_method {
                    methods.append(&mut additional_methods);
                }
                methods
            },
            service: options.service,
            also_known_as: options.also_known_as,
            controller: options.controller,
            ..Default::default()
        };

        Ok(BearerDid {
            did: Did::parse(&did_uri)?,
            document,
            key_manager,
        })
    }

    pub fn resolve(uri: &str) -> ResolutionResult {
        let did = match Did::parse(uri) {
            Ok(did) => did,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        let resolution_result = match Resolver::new(did) {
            Ok(resolver) => resolver.into_future(),
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
    use crate::{test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;

    mod create {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("did_web_create");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_can_specify_key_manager() {
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

            let result = DidWeb::create("https://example.com/did.json", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.did.to_string(), "did:web:example.com");
        }

        #[test]
        fn test_should_trim_well_known() {
            TEST_SUITE.include(test_name!());

            let result = DidWeb::create("https://example.com/.well-known/did.json", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.did.to_string(), "did:web:example.com");
        }

        #[test]
        fn test_should_percent_encode_colons() {
            TEST_SUITE.include(test_name!());

            let result = DidWeb::create("https://example.com:8080", None);
            assert!(result.is_ok());

            let did_web = result.unwrap();
            assert_eq!(did_web.did.to_string(), "did:web:example.com%3A8080");
        }

        #[test]
        fn test_should_replace_path_with_colons() {
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("did_web_resolve");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_invalid_did() {
            TEST_SUITE.include(test_name!());

            let resolution_result = DidWeb::resolve("something invalid");
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InvalidDid)
            )
        }

        #[test]
        fn test_create_then_resolve() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new();
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

            let resolution_result = DidWeb::resolve(&bearer_did.did.uri);

            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert!(resolution_result.document.is_some());
            let resolved_document = resolution_result.document.unwrap();
            assert_eq!(resolved_document, bearer_did.document);
        }
    }
}
