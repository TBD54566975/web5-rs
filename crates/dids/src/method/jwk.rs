use crate::bearer::BearerDid;
use crate::document::Document;
use crate::identifier::Identifier;
use crate::method::{DidMethod, DidMethodError};
use crate::resolver::DidResolutionResult;
use async_trait::async_trait;
use crypto::key::{Key, KeyType};
use crypto::key_manager::KeyManager;
use did_jwk::DIDJWK as SpruceDidJwkMethod;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

/// Concrete implementation for a did:jwk DID
pub struct DidJwk;

/// Options that can be used to create a did:jwk DID
pub struct DidJwkCreateOptions {
    pub key_type: KeyType,
}

#[async_trait]
impl DidMethod<DidJwkCreateOptions> for DidJwk {
    const NAME: &'static str = "jwk";

    fn create<T: KeyManager>(
        key_manager: Arc<T>,
        options: DidJwkCreateOptions,
    ) -> Result<BearerDid<T>, DidMethodError> {
        let key_alias = key_manager.generate_private_key(options.key_type)?;
        let public_key =
            key_manager
                .get_public_key(&key_alias)?
                .ok_or(DidMethodError::DidCreationFailure(
                    "PublicKey not found".to_string(),
                ))?;

        let uri = SpruceDidJwkMethod
            .generate(&Source::Key(public_key.jwk()))
            .ok_or(DidMethodError::DidCreationFailure(
                "Failed to generate did:jwk".to_string(),
            ))?;

        let identifier = Identifier::parse(&uri).map_err(|e| {
            DidMethodError::DidCreationFailure(format!("Failed to parse did:jwk uri {} {}", uri, e))
        })?;

        let bearer_did = BearerDid {
            identifier,
            key_manager,
            document: Document {
                // todo
                ..Default::default()
            },
        };

        Ok(bearer_did)
    }

    async fn resolve_uri(did_uri: &str) -> DidResolutionResult {
        let input_metadata = ResolutionInputMetadata::default();
        let (did_resolution_metadata, did_document, did_document_metadata) =
            SpruceDidJwkMethod.resolve(did_uri, &input_metadata).await;

        DidResolutionResult {
            did_resolution_metadata,
            did_document,
            did_document_metadata,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::key_manager::local_key_manager::LocalKeyManager;
    use ssi_dids::did_resolve::ERROR_INVALID_DID;

    fn create_did_jwk() -> BearerDid<LocalKeyManager> {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            key_type: KeyType::Ed25519,
        };

        DidJwk::create(key_manager, options).unwrap()
    }

    #[test]
    fn create_produces_correct_uri() {
        let bearer_did = create_did_jwk();
        assert!(bearer_did.identifier.uri.starts_with("did:jwk:"));
    }

    // #[tokio::test]
    // async fn instance_resolve() {
    //     let did = create_did_jwk();
    //     let result = did.resolve().await;
    //     assert!(result.did_resolution_metadata.error.is_none());

    //     let did_document = result.did_document.unwrap();
    //     assert_eq!(did_document.id, did.uri);
    // }

    #[tokio::test]
    async fn resolve_uri_success() {
        let bearer_did = create_did_jwk();
        let result = DidJwk::resolve_uri(&bearer_did.identifier.uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.unwrap();
        assert_eq!(did_document.id, bearer_did.identifier.uri);
    }

    #[tokio::test]
    async fn resolve_uri_failure() {
        let result = DidJwk::resolve_uri("did:jwk:does-not-exist").await;
        assert_eq!(
            result.did_resolution_metadata.error,
            Some(ERROR_INVALID_DID.to_string())
        );
    }
}
