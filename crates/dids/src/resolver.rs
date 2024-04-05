use crate::document::DidDocument;
use crate::identifier::DidIdentifier;
use crate::method::jwk::DidJwk;
use crate::method::web::DidWeb;
use crate::method::DidMethod;
use serde::{Deserialize, Serialize};

pub struct DidResolver;

impl DidResolver {
    /// Resolves a DID URI, using the appropriate DID method, to a DID Document.
    pub async fn resolve_uri(did_uri: &str) -> DidResolutionResult {
        let identifier = match DidIdentifier::parse(did_uri) {
            Ok(identifier) => identifier,
            Err(_) => return DidResolutionResult::from_error(DidResolutionError::InvalidDid),
        };

        match identifier.method.as_str() {
            DidJwk::NAME => DidJwk::resolve_uri(did_uri).await,
            DidWeb::NAME => DidWeb::resolve_uri(did_uri).await,
            _ => DidResolutionResult::from_error(DidResolutionError::MethodNotSupported),
        }
    }
}

/// Result metadata of a DID resolution.
///
/// See [DID Resolution Metadata](https://www.w3.org/TR/did-core/#did-resolution-metadata) for more information
/// See [web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DidResolutionMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DidResolutionError>,
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

// todo remove?
const DID_RESOLUTION_V1_CONTEXT: &str = "https://w3id.org/did-resolution/v1";

/// Errors that can occur during DID resolution
/// https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-error-types
#[derive(thiserror::Error, Debug, PartialEq, Serialize, Deserialize)]
pub enum DidResolutionError {
    #[error("The requested DID was not valid and resolution could not proceed.")]
    #[serde(rename = "invalidDid")]
    InvalidDid,
    #[error("The requested DID was not found.")]
    #[serde(rename = "notFound")]
    NotFound,
    #[error("The requested representation of the DID payload is not supported by the resolver.")]
    #[serde(rename = "representationNotSupported")]
    RepresentationNotSupported,
    #[error("The requested DID method is not supported by the resolver.")]
    #[serde(rename = "methodNotSupported")]
    MethodNotSupported,
    #[error("The DID Document was found but did not represent a conformant document.")]
    #[serde(rename = "invalidDidDocument")]
    InvalidDidDocument,
    #[error("The size of the DID Document was not within the method's acceptable limit.")]
    #[serde(rename = "invalidDidDocumentLength")]
    InvalidDidDocumentLength,
    #[error("Something went wrong during DID resolution.")]
    #[serde(rename = "internalError")]
    InternalError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DidDocumentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,
    #[serde(rename = "nextUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "nextVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version_id: Option<String>,
    #[serde(rename = "equivalentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equivalent_id: Option<Vec<String>>,
    #[serde(rename = "canonicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_id: Option<String>,
}

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
    pub fn from_error(err: DidResolutionError) -> Self {
        Self {
            did_resolution_metadata: DidResolutionMetadata::from_error(err),
            ..Default::default()
        }
    }
}

impl DidResolutionMetadata {
    /// Convenience method for creating a DidResolutionResult with an error.
    pub fn from_error(err: DidResolutionError) -> Self {
        Self { error: Some(err) }
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
            Some(DidResolutionError::InvalidDid)
        );
    }

    #[tokio::test]
    async fn resolve_unsupported_method() {
        let did_uri = "did:unsupported:1234";
        let result = DidResolver::resolve_uri(did_uri).await;
        assert_eq!(
            result.did_resolution_metadata.error,
            Some(DidResolutionError::MethodNotSupported)
        );
    }
}
