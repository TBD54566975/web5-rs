mod resolver;

use super::{MethodError, Result};
use crate::apid::dids::{
    data_model::document::Document,
    did::Did,
    resolution::{
        resolution_metadata::{ResolutionMetadata, ResolutionMetadataError},
        resolution_result::ResolutionResult,
    },
};
use resolver::Resolver;

#[derive(Clone)]
pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub async fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidWeb::resolve(uri).await?;
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

    pub async fn resolve(uri: &str) -> Result<ResolutionResult> {
        let did = Did::new(uri)?;
        let resolution_result = Resolver::new(did).await;

        Ok(match resolution_result {
            Err(e) => ResolutionResult {
                resolution_metadata: ResolutionMetadata { error: Some(e) },
                ..Default::default()
            },
            Ok(r) => r,
        })
    }
}
