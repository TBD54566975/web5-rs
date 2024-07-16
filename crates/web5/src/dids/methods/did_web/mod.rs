mod resolver;

use super::{MethodError, Result};
use crate::{
    crypto::jwk::Jwk,
    dids::{
        data_model::{document::Document, verification_method::VerificationMethod},
        did::Did,
        resolution::{
            resolution_metadata::{ResolutionMetadata, ResolutionMetadataError},
            resolution_result::ResolutionResult,
        },
    },
};
use resolver::Resolver;
use url::Url;

#[derive(Clone)]
pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub fn new(domain: &str, public_jwk: Jwk) -> Result<Self> {
        let domain = &domain.to_string();
        let valid_url = if domain.starts_with("http://") || domain.starts_with("https://") {
            let url = Url::parse(domain)
                .map_err(|e| MethodError::DidCreationFailure(format!("url parse failure {}", e)))?;

            // Ensure "http://" is only allowed for localhost or 127.0.0.1
            if url.scheme() == "http"
                && !(url.host_str() == Some("localhost") || url.host_str() == Some("127.0.0.1"))
            {
                return Err(MethodError::DidCreationFailure(
                    "only https is allowed except for localhost or 127.0.0.1 with http".to_string(),
                ));
            }

            // Get the trimmed URL string without the scheme
            let trimmed_url = url[url::Position::BeforeHost..].to_string();

            // Remove the scheme
            let normalized = if let Some(trimmed) = trimmed_url.strip_prefix("//") {
                trimmed
            } else {
                &trimmed_url
            };

            normalized.to_string()
        } else {
            Url::parse(&format!("https://{}", domain))
                .map_err(|e| MethodError::DidCreationFailure(format!("url parse failure {}", e)))?;
            domain.clone()
        };

        let mut normalized = valid_url.clone();
        if normalized.ends_with('/') {
            normalized = normalized.trim_end_matches('/').to_string()
        }
        if normalized.ends_with("/did.json") {
            normalized = normalized.trim_end_matches("/did.json").to_string()
        }
        if normalized.ends_with("/.well-known") {
            normalized = normalized.trim_end_matches("/.well-known").to_string()
        }

        let encoded_domain = normalized.replace(':', "%3A");
        let encoded_domain = encoded_domain.replace('/', ":");

        let did = format!("did:web:{}", encoded_domain);

        let verification_method = VerificationMethod {
            id: format!("{}#key-0", did),
            r#type: "JsonWebKey".to_string(),
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
            document,
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
