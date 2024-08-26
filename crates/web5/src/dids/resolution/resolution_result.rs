use super::{document_metadata::DocumentMetadata, resolution_metadata::ResolutionMetadata};
use crate::dids::{
    data_model::document::Document,
    did::Did,
    methods::{
        did_dht::{DidDht, DidDhtResolveOptions},
        did_jwk::DidJwk,
        did_web::DidWeb,
    },
    resolution::resolution_metadata::ResolutionMetadataError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolutionResult {
    pub resolution_metadata: ResolutionMetadata,
    pub document: Option<Document>,
    pub document_metadata: Option<DocumentMetadata>,
}

#[derive(Default)]
pub struct ResolutionResultResolveOptions {
    pub did_dht_gateway_url: Option<String>,
}

impl ResolutionResult {
    pub fn resolve(uri: &str, options: Option<ResolutionResultResolveOptions>) -> Self {
        let options = options.unwrap_or_default();

        let did = match Did::parse(uri) {
            Ok(did) => did,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        match did.method.as_str() {
            "jwk" => DidJwk::resolve(uri),
            "dht" => DidDht::resolve(
                uri,
                Some(DidDhtResolveOptions {
                    gateway_url: options.did_dht_gateway_url,
                }),
            ),
            "web" => DidWeb::resolve(uri),
            _ => ResolutionResult::from(ResolutionMetadataError::MethodNotSupported),
        }
    }
}

impl From<ResolutionMetadataError> for ResolutionResult {
    fn from(error: ResolutionMetadataError) -> Self {
        Self {
            resolution_metadata: ResolutionMetadata { error: Some(error) },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dids::methods::did_dht::DidDhtCreateOptions;
    use crate::{test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;
    use mockito::Server;
    use std::sync::{Arc, Mutex};

    mod resolve {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("resolution_result_resolve");
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

            let resolution_result = ResolutionResult::resolve("something invalid", None);
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InvalidDid)
            )
        }

        #[test]
        fn test_did_jwk() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();

            let resolution_result = ResolutionResult::resolve(&bearer_did.did.uri, None);
            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert_eq!(resolution_result.document.unwrap(), bearer_did.document);
        }

        #[test]
        fn test_did_web() {
            TEST_SUITE.include(test_name!());

            let mut mock_server = Server::new();
            let url = mock_server.url();

            let bearer_did = DidWeb::create(&url, None).unwrap();

            let _ = mock_server
                .mock("GET", "/.well-known/did.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(&bearer_did.document).unwrap())
                .create();

            let resolution_result = ResolutionResult::resolve(&bearer_did.did.uri, None);

            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert!(resolution_result.document.is_some());
            assert_eq!(resolution_result.document.unwrap(), bearer_did.document);
        }

        #[test]
        fn test_did_dht_with_specified_gateway() {
            TEST_SUITE.include(test_name!());

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

            let bearer_did = DidDht::create(Some(DidDhtCreateOptions {
                publish: Some(true),
                gateway_url: Some(gateway_url),
                ..Default::default()
            }))
            .unwrap();

            let stored_body = published_body.lock().unwrap();

            let mock_resolve = mock_server
                .mock("GET", format!("/{}", bearer_did.did.id).as_str())
                .expect(1)
                .with_status(200)
                .with_header("content-type", "application/octet-stream")
                .with_body(stored_body.clone()) // Use the captured body as the response
                .create();

            let resolution_result = ResolutionResult::resolve(
                &bearer_did.did.uri,
                Some(ResolutionResultResolveOptions {
                    did_dht_gateway_url: Some(mock_server.url()),
                }),
            );

            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert!(resolution_result.document.is_some());
            assert_eq!(resolution_result.document.unwrap(), bearer_did.document);

            mock_publish.assert();
            mock_resolve.assert();
        }

        #[test]
        fn test_method_not_supported() {
            TEST_SUITE.include(test_name!());

            let resolution_result = ResolutionResult::resolve("did:example:123", None);
            assert!(resolution_result.resolution_metadata.error.is_some());
            assert_eq!(
                resolution_result.resolution_metadata.error.unwrap(),
                ResolutionMetadataError::MethodNotSupported
            );
        }
    }
}
