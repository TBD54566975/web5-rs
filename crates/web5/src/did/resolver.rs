use crate::did::method::{DidJwk, DidKey, DidMethod, DidWeb};
use crate::did::parsed_did::{ParsedDid, ParsedDidError};
use async_trait::async_trait;
use ssi_dids::{
    did_resolve::{DocumentMetadata as DidDocumentMetadata, ResolutionMetadata},
    Document as DidDocument,
};
use std::str::FromStr;
use thiserror::Error;

#[async_trait]
pub trait DidResolver {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResponse, DidResolutionError>;
}

pub type DidResolutionResponse = (ResolutionMetadata, DidDocument, Option<DidDocumentMetadata>);

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DidResolutionError {
    #[error("Unsupported DID method")]
    UnsupportedDidMethod,
    #[error("DID document not found")]
    DidDocumentNotFound,
    #[error(transparent)]
    ParsedDidError(#[from] ParsedDidError),
}

pub async fn resolve(did_uri: &str) -> Result<DidResolutionResponse, DidResolutionError> {
    let parsed_did = ParsedDid::from_str(did_uri)?;

    match parsed_did.method {
        DidMethod::Jwk => DidJwk::resolve(did_uri).await,
        DidMethod::Key => DidKey::resolve(did_uri).await,
        DidMethod::Web => DidWeb::resolve(did_uri).await,
    }
}

#[derive(Error, Debug)]
pub enum Error1 {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::crypto::key::KeyAlgorithm;
    use crate::crypto::key_manager::local::key_store::in_memory::InMemoryKeyStore;
    use crate::crypto::key_manager::local::LocalKeyManager;
    use crate::did::method::{DidJwk, DidJwkCreateOptions, DidKey, DidKeyCreateOptions};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_resolve_did_jwk() {
        let in_memory_key_store = InMemoryKeyStore::new();
        let local_key_manager = LocalKeyManager::new(Arc::new(in_memory_key_store));

        let did_jwk = DidJwk::new(
            Arc::new(local_key_manager),
            DidJwkCreateOptions {
                key_algorithm: KeyAlgorithm::Ed25519,
            },
        )
        .expect("DidJwk initialization failed");

        let (_, did_document, _) = resolve(&did_jwk.uri).await.expect("Failed to resolve DID");
        assert_eq!(did_document.id, did_jwk.uri);
    }

    #[tokio::test]
    async fn test_resolve_did_key() {
        let in_memory_key_store = InMemoryKeyStore::new();
        let local_key_manager = LocalKeyManager::new(Arc::new(in_memory_key_store));

        let did_key = DidKey::new(
            Arc::new(local_key_manager),
            DidKeyCreateOptions {
                key_algorithm: KeyAlgorithm::Ed25519,
            },
        )
        .expect("DidKey initialization failed");

        let (_, did_document, _) = resolve(&did_key.uri).await.expect("Failed to resolve DID");
        assert_eq!(did_document.id, did_key.uri);
    }

    #[tokio::test]
    async fn test_resolve_did_web() {
        let did_web_uri = "did:web:tbd.website";
        let (_, did_document, _) = resolve(did_web_uri).await.expect("Failed to resolve DID");
        assert_eq!(did_document.id, did_web_uri);
    }

    #[tokio::test]
    async fn test_resolve_unsupported_did_method() {
        let did_uri = "did:unsupported:123123";
        let resolve_result = resolve(did_uri).await;
        assert!(resolve_result.is_err());
    }

    #[tokio::test]
    async fn test_resolve_invalid_did_uri() {
        let invalid_did_uri = "wrong:did:key:123123";
        let resolve_result = resolve(invalid_did_uri).await;
        assert!(resolve_result.is_err());
    }
}
