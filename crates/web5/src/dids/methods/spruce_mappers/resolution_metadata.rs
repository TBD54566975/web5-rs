use crate::dids::resolver::{ResolutionError, ResolutionMetadata};
use ssi_dids::did_resolve::{
    ResolutionMetadata as SpruceResolutionMetadata, ERROR_INVALID_DID, ERROR_METHOD_NOT_SUPPORTED,
    ERROR_NOT_FOUND, ERROR_REPRESENTATION_NOT_SUPPORTED,
};

impl ResolutionMetadata {
    pub fn from_spruce(
        spruce_resolution_metadata: SpruceResolutionMetadata,
    ) -> Result<Self, String> {
        let error = if let Some(err) = spruce_resolution_metadata.error {
            let error = match err.as_str() {
                ERROR_INVALID_DID => Ok(ResolutionError::InvalidDid),
                ERROR_NOT_FOUND => Ok(ResolutionError::NotFound),
                ERROR_REPRESENTATION_NOT_SUPPORTED => {
                    Ok(ResolutionError::RepresentationNotSupported)
                }
                ERROR_METHOD_NOT_SUPPORTED => Ok(ResolutionError::MethodNotSupported),
                _ => Err(format!("Unknown error: {}", err)),
            }?;
            Some(error)
        } else {
            None
        };

        Ok(ResolutionMetadata { error })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_spruce_resolution_metadata(error: &str) -> SpruceResolutionMetadata {
        SpruceResolutionMetadata {
            error: Some(error.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            ResolutionMetadata::from_spruce(create_spruce_resolution_metadata(ERROR_INVALID_DID))
                .unwrap()
                .error,
            Some(ResolutionError::InvalidDid)
        );
        assert_eq!(
            ResolutionMetadata::from_spruce(create_spruce_resolution_metadata(ERROR_NOT_FOUND))
                .unwrap()
                .error,
            Some(ResolutionError::NotFound)
        );
        assert_eq!(
            ResolutionMetadata::from_spruce(create_spruce_resolution_metadata(
                ERROR_REPRESENTATION_NOT_SUPPORTED
            ))
            .unwrap()
            .error,
            Some(ResolutionError::RepresentationNotSupported)
        );
        assert_eq!(
            ResolutionMetadata::from_spruce(create_spruce_resolution_metadata(
                ERROR_METHOD_NOT_SUPPORTED
            ))
            .unwrap()
            .error,
            Some(ResolutionError::MethodNotSupported)
        );
    }
}
