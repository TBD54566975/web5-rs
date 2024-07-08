mod resolver;

use super::{MethodError, Result};
use crate::{crypto::jwk::Jwk, dids::{
    data_model::{document::Document, verification_method::VerificationMethod},
    did::Did,
    resolution::{
        resolution_metadata::{ResolutionMetadata, ResolutionMetadataError},
        resolution_result::ResolutionResult,
    },
}};
use resolver::Resolver;

#[derive(Clone)]
pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub fn new(domain: &str, public_jwk: Jwk) -> Result<Self> {
        let did = format!("did:web:{}", domain);

        let verification_method = VerificationMethod {
            id: format!("{}#key-0", did),
            r#type: "JsonWebKey2020".to_string(),
            controller: did.clone(),
            public_key_jwk: public_jwk,
        };

        let document = Document {
            id: did.clone(),
            context: Some(vec!["https://www.w3.org/ns/did/v1".to_string()]),
            verification_method: vec![verification_method],
            ..Default::default()
        };

        Ok(DidWeb {
            did: Did::new(&did)?,
            document
        })
    }

    pub async fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidWeb::resolve(uri);
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

    pub fn resolve(uri: &str) -> ResolutionResult {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(_) => {
                return ResolutionResult {
                    resolution_metadata: ResolutionMetadata {
                        error: Some(ResolutionMetadataError::InternalError),
                    },
                    ..Default::default()
                }
            }
        };

        let result: Result<ResolutionResult> = rt.block_on(async {
            let did = Did::new(uri).map_err(|_| ResolutionMetadataError::InvalidDid)?;
            let resolution_result = Resolver::new(did).await;
            Ok(match resolution_result {
                Err(e) => ResolutionResult {
                    resolution_metadata: ResolutionMetadata { error: Some(e) },
                    ..Default::default()
                },
                Ok(r) => r,
            })
        });

        match result {
            Ok(resolution_result) => resolution_result,
            Err(err) => ResolutionResult {
                resolution_metadata: ResolutionMetadata {
                    error: Some(match err {
                        MethodError::ResolutionError(e) => e,
                        _ => ResolutionMetadataError::InternalError,
                    }),
                },
                ..Default::default()
            },
        }
    }
}
