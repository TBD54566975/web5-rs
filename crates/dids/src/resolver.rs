use lazy_static::lazy_static;
use ssi_dids::did_resolve::{
    DIDResolver, DocumentMetadata as DidDocumentMetadata, ResolutionInputMetadata,
    ResolutionMetadata,
};
use ssi_dids::{DIDMethods, Document as DidDocument};

pub struct DidResolver;

pub struct DidResolutionResponse {
    pub resolution_metadata: ResolutionMetadata,
    pub did_document: DidDocument,
    pub did_document_metadata: Option<DidDocumentMetadata>,
}

#[derive(thiserror::Error, Debug)]
pub enum DidResolutionError {
    #[error("DID Document not found for DID {did_uri}")]
    DidDocumentNotFound { did_uri: String },
    #[error("Error resolving DID {did_uri}: {msg}")]
    ResolverError { did_uri: String, msg: String },
}

pub type DidResolutionResult = Result<DidResolutionResponse, DidResolutionError>;

impl DidResolver {
    pub async fn resolve(&self, did_uri: &str) -> DidResolutionResult {
        let input_metadata = &ResolutionInputMetadata::default();

        let (resolution_metadata, did_document, did_document_metadata) =
            DID_RESOLVER.resolve(did_uri, input_metadata).await;

        if let Some(error_message) = resolution_metadata.error {
            return Err(DidResolutionError::ResolverError {
                did_uri: did_uri.to_string(),
                msg: error_message,
            });
        }

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound {
            did_uri: did_uri.to_string(),
        })?;

        Ok(DidResolutionResponse {
            resolution_metadata,
            did_document,
            did_document_metadata,
        })
    }
}

lazy_static! {
    static ref DID_RESOLVER: DIDMethods<'static> = {
        let mut methods = DIDMethods::default();
        methods.insert(Box::new(did_method_key::DIDKey));
        methods.insert(Box::new(did_web::DIDWeb));
        methods.insert(Box::new(did_jwk::DIDJWK));
        methods.insert(Box::new(did_ion::DIDION::new(Some(
            "https://ion.tbddev.org".to_string(),
        ))));
        methods
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_did_web() {
        let did_uri = "did:web:tbd.website";
        let response = DidResolver
            .resolve(did_uri)
            .await
            .expect("Failed to resolve DID");
        assert_eq!(response.resolution_metadata.error, None);
        assert_eq!(response.did_document.id, did_uri)
    }

    #[tokio::test]
    async fn test_resolve_unknown_method() {
        let did_uri = "did:unknown:123";
        let response = DidResolver.resolve(did_uri).await;
        assert!(response.is_err());
    }
}
