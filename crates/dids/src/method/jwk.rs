use crate::bearer::BearerDid;
use crate::document::{DidDocument, VerificationMethod};
use crate::identifier::DidIdentifier;
use crate::method::{DidMethod, DidMethodError};
use crate::resolver::{DidResolutionError, DidResolutionResult};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use crypto::key::public_key::PublicKey;
use crypto::key::KeyType;
use crypto::key_manager::KeyManager;
use serde_json::{from_str, to_string};
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

        let serialized = to_string(&public_key).map_err(|e| {
            DidMethodError::DidCreationFailure(format!("Failed to serialize {}", e))
        })?;
        let encoded_id = general_purpose::URL_SAFE_NO_PAD.encode(&serialized);
        let uri = format!("did:jwk:{}", encoded_id);

        let identifier = DidIdentifier::parse(&uri).map_err(|e| {
            DidMethodError::DidCreationFailure(format!(
                "Failed to parse did:jwk uri {} {}",
                &uri, e
            ))
        })?;

        let bearer_did = BearerDid {
            identifier,
            key_manager,
            document: DidDocument {
                id: uri.clone(),
                verification_method: vec![VerificationMethod {
                    id: format!("{}#{}", uri, "0"),
                    r#type: "JsonWebKey".to_string(),
                    controller: uri,
                    public_key_jwk: public_key,
                }],
                ..Default::default()
            },
        };

        Ok(bearer_did)
    }

    async fn resolve_uri(did_uri: &str) -> DidResolutionResult {
        let identifier = match DidIdentifier::parse(did_uri) {
            Ok(identifier) => identifier,
            Err(_) => return DidResolutionResult::from_error(DidResolutionError::InvalidDid),
        };

        if identifier.method != Self::NAME {
            return DidResolutionResult::from_error(DidResolutionError::MethodNotSupported);
        }

        let decoded_id = match general_purpose::URL_SAFE_NO_PAD.decode(&identifier.id) {
            Ok(decoded_id) => decoded_id,
            Err(_) => return DidResolutionResult::from_error(DidResolutionError::InvalidDid),
        };

        let json_str = match String::from_utf8(decoded_id) {
            Ok(json_str) => json_str,
            Err(_) => return DidResolutionResult::from_error(DidResolutionError::InternalError),
        };

        let public_key: PublicKey = match from_str(&json_str) {
            Ok(public_key) => public_key,
            Err(_) => return DidResolutionResult::from_error(DidResolutionError::InternalError),
        };

        DidResolutionResult {
            did_document: Some(DidDocument {
                id: identifier.uri.clone(),
                verification_method: vec![VerificationMethod {
                    id: format!("{}#{}", identifier.uri, "0"),
                    r#type: "JsonWebKey".to_string(),
                    controller: identifier.uri,
                    public_key_jwk: public_key,
                }],
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::key_manager::local_key_manager::LocalKeyManager;

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
            Some(DidResolutionError::InvalidDid)
        );
    }
}
