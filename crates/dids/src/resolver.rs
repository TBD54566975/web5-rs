use crate::method::jwk::DidJwk;
use crate::method::web::DidWeb;
use crate::method::DidMethod;
use serde::{Deserialize, Serialize};
use ssi_dids::did_resolve::{
    DocumentMetadata as DidDocumentMetadata, ResolutionMetadata as DidResolutionMetadata,
};
use ssi_dids::Document as DidDocument;

pub struct DidResolver;

impl DidResolver {
    /// Resolves a DID URI, using the appropriate DID method, to a DID Document.
    pub async fn resolve_uri(did_uri: &str) -> DidResolutionResult {
        let method_name = match DidResolver::method_name(did_uri) {
            Some(method_name) => method_name,
            None => return DidResolutionResult::from_error(ERROR_INVALID_DID),
        };

        match method_name {
            DidJwk::NAME => DidJwk::resolve_uri(did_uri).await,
            DidWeb::NAME => DidWeb::resolve_uri(did_uri).await,
            _ => DidResolutionResult::from_error(ERROR_METHOD_NOT_SUPPORTED),
        }
    }

    /// Returns the method name of a DID URI, if the provided DID URI is valid, `None` otherwise.
    fn method_name(did_uri: &str) -> Option<&str> {
        let parts: Vec<&str> = did_uri.split(':').collect();
        if parts.len() < 3 || parts[0] != "did" {
            return None;
        };

        Some(parts[1])
    }
}

/// Result of a DID resolution.
///
/// See [Resolving a DID](https://w3c-ccg.github.io/did-resolution/#resolving) for more information
/// about the resolution process, and documentation around expected results formats in the case
/// there was an error resolving the DID.
#[derive(Debug, Deserialize, Serialize)]
pub struct DidResolutionResult {
    #[serde(rename = "@context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    pub did_resolution_metadata: DidResolutionMetadata,
    pub did_document: Option<DidDocument>,
    pub did_document_metadata: Option<DidDocumentMetadata>,
}

const DID_RESOLUTION_V1_CONTEXT: &str = "https://w3id.org/did-resolution/v1";
const ERROR_METHOD_NOT_SUPPORTED: &str = "methodNotSupported";
const ERROR_INVALID_DID: &str = "invalidDid";

impl Default for DidResolutionResult {
    fn default() -> Self {
        Self {
            context: Some(DID_RESOLUTION_V1_CONTEXT.to_string()),
            did_resolution_metadata: DidResolutionMetadata::default(),
            did_document: None,
            did_document_metadata: None,
        }
    }
}

impl DidResolutionResult {
    /// Convenience method for creating a DidResolutionResult with an error.
    pub fn from_error(err: &str) -> Self {
        Self {
            did_resolution_metadata: DidResolutionMetadata::from_error(err),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn resolve_did_jwk() {
        let did_uri = "did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9";
        let result = DidResolver::resolve_uri(did_uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.unwrap();
        assert_eq!(did_document.id, did_uri);
    }

    #[tokio::test]
    async fn resolve_did_web() {
        let did_uri = "did:web:tbd.website";
        let result = DidResolver::resolve_uri(did_uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.unwrap();
        assert_eq!(did_document.id, did_uri);
    }

    #[tokio::test]
    async fn resolve_invalid_did() {
        let did_uri = "did:jwk";
        let result = DidResolver::resolve_uri(did_uri).await;
        assert_eq!(
            result.did_resolution_metadata.error,
            Some(ERROR_INVALID_DID.to_string())
        );
    }

    #[tokio::test]
    async fn resolve_unsupported_method() {
        let did_uri = "did:unsupported:1234";
        let result = DidResolver::resolve_uri(did_uri).await;
        assert_eq!(
            result.did_resolution_metadata.error,
            Some(ERROR_METHOD_NOT_SUPPORTED.to_string())
        );
    }
}
