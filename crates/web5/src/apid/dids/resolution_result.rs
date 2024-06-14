use super::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResolutionResult {
    pub resolution_metadata: ResolutionMetadata,
    pub document: Option<Document>,
    pub document_metadata: Option<DocumentMetadata>,
}

impl ResolutionResult {
    // 🚧 change in APID from STATIC METHOD to CONSTRUCTOR
    pub fn new(uri: &str) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResolutionMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResolutionError>,
}

#[derive(thiserror::Error, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum ResolutionError {
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
pub struct DocumentMetadata {
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
