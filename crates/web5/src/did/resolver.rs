use crate::did::method::{did_jwk::DidJwk, did_key::DidKey, DidMethod};
use crate::did::parsed_did::{ParsedDid, ParsedDidError};
use async_trait::async_trait;
use ssi_dids::{
    did_resolve::{DocumentMetadata as DidDocumentMetadata, ResolutionMetadata},
    Document as DidDocument,
};
use std::str::FromStr;

#[async_trait]
pub trait DidResolver {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResult, DidResolutionError>;
}

pub struct DidResolutionResult {
    pub resolution_metadata: ResolutionMetadata,
    pub did_document: DidDocument,
    pub did_document_metadata: Option<DidDocumentMetadata>,
}

#[derive(thiserror::Error, Debug)]
pub enum DidResolutionError {
    #[error("Provided Did URI is invalid")]
    InvalidDidUri,
    #[error("Unsupported DID method")]
    UnsupportedDidMethod,
    #[error("DID document not found")]
    DidDocumentNotFound,
    #[error(transparent)]
    ParsedDidError(#[from] ParsedDidError),
}

pub async fn resolve(did_uri: &str) -> Result<DidResolutionResult, DidResolutionError> {
    let parsed_did = ParsedDid::from_str(did_uri)?;

    match parsed_did.method {
        DidMethod::Jwk => DidJwk::resolve(did_uri).await,
        DidMethod::Key => DidKey::resolve(did_uri).await,
    }
}
