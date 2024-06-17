use super::{MethodError, Result};
use crate::apid::dids::{
    did::Did,
    document::Document,
    resolution_result::{ResolutionMetadataError, ResolutionResult},
};

#[derive(Clone)]
pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidWeb::resolve(uri)?;
        match resolution_result.document {
            None => Err(match resolution_result.resolution_metadata.error {
                None => MethodError::ResolutionError(ResolutionMetadataError::InternalError),
                Some(e) => MethodError::ResolutionError(e),
            }),
            Some(document) => {
                let identifer = Did::new(uri)?;
                Ok(Self {
                    did: identifer,
                    document,
                })
            }
        }
    }

    pub fn resolve(uri: &str) -> Result<ResolutionResult> {
        // 🚧 use existing PR which replaces spruce dep
        println!("DidWeb::resolve() called with {}", uri);
        Ok(ResolutionResult {
            ..Default::default()
        })
    }
}
