use crate::method::DidCreationError;
use crate::resolver::{DidResolutionError, DidResolutionResponse, DidResolver};
use crate::Did;
use async_trait::async_trait;
use crypto::key::{Key, KeyAlgorithm};
use crypto::key_manager::KeyManager;
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
    pub fn new(
        key_manager: Arc<dyn KeyManager>,
        options: DidKeyCreateOptions,
    ) -> Result<Self, DidCreationError> {
        let (_, public_key) = key_manager.generate_private_key(options.key_algorithm)?;

        let uri = DIDKey
            .generate(&Source::Key(&public_key.jwk()))
            .ok_or(DidCreationError::DidGenerationFailed)?;

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
