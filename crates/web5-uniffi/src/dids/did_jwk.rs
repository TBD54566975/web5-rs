use crate::crypto::key::KeyAlgorithm;
use crate::crypto::key_manager::KeyManager;
use crate::dids::did::{DidResolutionResult, DidResolver};
use crate::error::Web5Error;
use async_trait::async_trait;
use did_jwk::DIDJWK;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct DidJwk {
    pub uri: String,
}

#[uniffi::export]
impl DidJwk {
    #[uniffi::constructor]
    pub fn new(key_algorithm: KeyAlgorithm, key_manager: Arc<KeyManager>) -> Arc<Self> {
        // TODO: handle the error properly
        let key_alias = key_manager.generate_private_key(key_algorithm).unwrap();
        let private_key = key_manager
            .get_public_key(key_alias)
            .unwrap()
            .expect("public key not found immediately after creating the private key");
        let uri = DIDJWK.generate(&Source::Key(&private_key.0)).unwrap();

        Self { uri }.into()
    }

    pub fn get_uri(&self) -> String {
        self.uri.clone()
    }
}

pub struct DidJwkResolver;

#[async_trait]
impl DidResolver for DidJwkResolver {
    fn method_name() -> &'static str {
        "jwk"
    }

    async fn resolve(&self, did_uri: &str) -> Result<DidResolutionResult, Web5Error> {
        let (resolution_metadata, did_document, did_document_metadata) = DIDJWK
            .resolve(did_uri, &ResolutionInputMetadata::default())
            .await;

        if let Some(error_message) = resolution_metadata.error {
            // TODO: forward this error message into an error type
            println!("Error resolving DIDDocument: {}", error_message);
            return Err(Web5Error::Unknown);
        }

        // TODO: Proper error here
        let did_document = did_document.ok_or(Web5Error::Unknown)?;

        // TODO: Handle errors here
        Ok(DidResolutionResult {
            did_document: serde_json::to_string(&did_document).unwrap(),
            did_document_metadata: did_document_metadata
                .map(|md| serde_json::to_string(&md).unwrap()),
        })
    }
}
