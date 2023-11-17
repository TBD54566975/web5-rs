use crate::crypto::key::{Key, KeyAlgorithm};
use crate::crypto::key_manager::{KeyManager, KeyManagerError};
use crate::did::did::Did;
use crate::did::did_resolver::{DidResolutionError, DidResolutionResult, DidResolver};
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
        let key_alias = key_manager.generate_private_key(options.key_algorithm)?;
        let public_key =
            key_manager
                .get_public_key(&key_alias)?
                .ok_or(KeyManagerError::Generic {
                    message: "Public key not found immediately after creating private key"
                        .to_string(),
                })?;

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
    async fn resolve(did_uri: &str) -> Result<DidResolutionResult, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = DIDKey
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok(DidResolutionResult {
            resolution_metadata,
            did_document,
            did_document_metadata,
        })
    }
}
