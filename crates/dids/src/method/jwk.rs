use crate::bearer::BearerDid;
use crate::document::{Document, VerificationMethod};
use crate::identifier::Identifier;
use crate::method::{Method, MethodError};
use crate::resolver::ResolutionResult;
use async_trait::async_trait;
use crypto::key::KeyType;
use crypto::key_manager::KeyManager;
use did_jwk::DIDJWK as SpruceDidJwkMethod;
use serde_json::from_str;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use ssi_dids::{DIDMethod, Source};
use ssi_jwk::JWK as SpruceJwk;

/// Concrete implementation for a did:jwk DID
pub struct DidJwk;

/// Options that can be used to create a did:jwk DID
pub struct DidJwkCreateOptions {
    pub key_type: KeyType,
}

#[async_trait]
impl Method<DidJwkCreateOptions> for DidJwk {
    const NAME: &'static str = "jwk";

    fn create(
        key_manager: Box<dyn KeyManager>,
        options: DidJwkCreateOptions,
    ) -> Result<BearerDid, MethodError> {
        let key_alias = key_manager.generate_private_key(options.key_type)?;
        let public_key =
            key_manager
                .get_public_key(&key_alias)?
                .ok_or(MethodError::DidCreationFailure(
                    "PublicKey not found".to_string(),
                ))?;

        let josekit_jwk_string = public_key.jwk().to_string();
        let spruce_jwk: SpruceJwk = from_str(&josekit_jwk_string)
            .map_err(|e| MethodError::DidCreationFailure(e.to_string()))?;

        let uri = SpruceDidJwkMethod
            .generate(&Source::Key(&spruce_jwk))
            .ok_or(MethodError::DidCreationFailure(
                "Failed to generate did:jwk".to_string(),
            ))?;

        let identifier = Identifier::parse(&uri).map_err(|e| {
            MethodError::DidCreationFailure(format!("Failed to parse did:jwk uri {} {}", &uri, e))
        })?;

        let bearer_did = BearerDid {
            identifier,
            key_manager,
            document: Document {
                id: uri.clone(),
                verification_method: vec![VerificationMethod {
                    id: format!("{}#{}", uri, key_alias),
                    r#type: "JsonWebKey".to_string(),
                    controller: uri,
                    public_key_jwk: public_key.jwk().clone(),
                }],
                ..Default::default()
            },
        };

        Ok(bearer_did)
    }

    async fn resolve(did_uri: &str) -> ResolutionResult {
        let input_metadata = ResolutionInputMetadata::default();
        let (spruce_resolution_metadata, spruce_document, spruce_document_metadata) =
            SpruceDidJwkMethod.resolve(did_uri, &input_metadata).await;

        match ResolutionResult::from_spruce(
            spruce_resolution_metadata,
            spruce_document,
            spruce_document_metadata,
        ) {
            Ok(r) => r,
            Err(e) => ResolutionResult::from_error(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resolver::ResolutionError;

    use super::*;
    use crypto::key_manager::local_key_manager::LocalKeyManager;

    fn create_did_jwk() -> BearerDid {
        let key_manager = Box::new(LocalKeyManager::new_in_memory());
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

    #[tokio::test]
    async fn instance_resolve() {
        let did = create_did_jwk();
        let result = DidJwk::resolve(&did.identifier.uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.unwrap();
        assert_eq!(did_document.id, did.identifier.uri);
    }

    #[tokio::test]
    async fn resolve_uri_success() {
        let bearer_did = create_did_jwk();
        let result = DidJwk::resolve(&bearer_did.identifier.uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.unwrap();
        assert_eq!(did_document.id, bearer_did.identifier.uri);
    }

    #[tokio::test]
    async fn resolve_uri_failure() {
        let result = DidJwk::resolve("did:jwk:does-not-exist").await;
        assert_eq!(
            result.did_resolution_metadata.error,
            Some(ResolutionError::InvalidDid)
        );
    }
}
