use crate::resolver::{DidResolutionError, DidResolutionMetadata};

impl From<ssi_dids::did_resolve::ResolutionMetadata> for DidResolutionMetadata {
    fn from(metadata: ssi_dids::did_resolve::ResolutionMetadata) -> Self {
        DidResolutionMetadata {
            error: metadata.error.map(|err| match err.as_str() {
                ssi_dids::did_resolve::ERROR_INVALID_DID => DidResolutionError::InvalidDid,
                ssi_dids::did_resolve::ERROR_NOT_FOUND => DidResolutionError::NotFound,
                ssi_dids::did_resolve::ERROR_REPRESENTATION_NOT_SUPPORTED => {
                    DidResolutionError::RepresentationNotSupported
                }
                ssi_dids::did_resolve::ERROR_METHOD_NOT_SUPPORTED => {
                    DidResolutionError::MethodNotSupported
                }
                _ => DidResolutionError::InternalError,
            }),
        }
    }
}
