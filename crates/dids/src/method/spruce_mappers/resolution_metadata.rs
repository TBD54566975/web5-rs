use crate::resolver::{DidResolutionError, DidResolutionMetadata};
use ssi_dids::did_resolve::ResolutionMetadata as SpruceResolutionMetadata;

impl DidResolutionMetadata {
    pub fn from_spruce(
        spruce_resolution_metadata: SpruceResolutionMetadata,
    ) -> Result<Self, String> {
        let error = if let Some(err) = spruce_resolution_metadata.error {
            let error = match err.as_str() {
                ssi_dids::did_resolve::ERROR_INVALID_DID => Ok(DidResolutionError::InvalidDid),
                ssi_dids::did_resolve::ERROR_NOT_FOUND => Ok(DidResolutionError::NotFound),
                ssi_dids::did_resolve::ERROR_REPRESENTATION_NOT_SUPPORTED => {
                    Ok(DidResolutionError::RepresentationNotSupported)
                }
                ssi_dids::did_resolve::ERROR_METHOD_NOT_SUPPORTED => {
                    Ok(DidResolutionError::MethodNotSupported)
                }
                _ => Err(format!("Unknown error: {}", err)),
            }?;
            Some(error)
        } else {
            None
        };

        Ok(DidResolutionMetadata { error })
    }
}
