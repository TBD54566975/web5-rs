use serde::{Deserialize, Serialize};

/// Holds metadata about the results of the DID resolution process.
///
/// This struct provides information about the resolution process, such as any errors encountered.
/// It is part of the `ResolutionResult` and helps describe why a resolution might have failed.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ResolutionMetadata {
    /// An optional error that provides details on any issues encountered during the DID resolution process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResolutionMetadataError>,
}

#[derive(thiserror::Error, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum ResolutionMetadataError {
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
    #[error("The DID Document does not have a valid public key.")]
    #[serde(rename = "invalidPublicKey")]
    InvalidPublicKey,
    #[error("The size of the DID Document was not within the method's acceptable limit.")]
    #[serde(rename = "invalidDidDocumentLength")]
    InvalidDidDocumentLength,
    #[error("Something went wrong during DID resolution.")]
    #[serde(rename = "internalError")]
    InternalError,
}
