use crate::crypto::key::{Key, KeyAlgorithm};
use crate::crypto::key_manager::KeyManager;
use crate::did::resolver::{DidResolutionError, DidResolutionResponse, DidResolver};
use crate::did::Did;
use crate::result::Web5Result;
use async_trait::async_trait;
use did_method_key::DIDKey;
use ssi_dids::did_resolve::DIDResolver;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

pub struct DidKeyCreateOptions {
    pub key_algorithm: KeyAlgorithm,
}

pub struct DidKeyData {}

pub type DidKey = Did<DidKeyData>;

impl DidKey {
    pub fn new(key_manager: Arc<dyn KeyManager>, options: DidKeyCreateOptions) -> Web5Result<Self> {
        let (_, public_key) = key_manager.generate_private_key(options.key_algorithm)?;

        let uri = DIDKey
            .generate(&Source::Key(&public_key.jwk()))
            .expect("DidKey initialization failed");

        Ok(Self {
            uri,
            key_manager,
            method_data: DidKeyData {},
        })
    }
}

#[async_trait]
impl DidResolver for DidKey {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResponse, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = DIDKey
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok((resolution_metadata, did_document, did_document_metadata))
    }
}
