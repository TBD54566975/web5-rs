use crate::{
    document::{Document, DocumentError},
    identifier::{Identifier, IdentifierError},
    resolver::{ResolutionError, Resolver},
};
use keys::{
    key::KeyError,
    key_manager::{KeyManager, KeyManagerError},
};
use std::sync::Arc;

pub struct BearerDid {
    pub identifier: Identifier,
    pub key_manager: Arc<dyn KeyManager>,
    pub document: Document,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BearerDidError {
    #[error("verfication method not found")]
    VerificationMethodNotFound,
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),
    #[error(transparent)]
    IdentifierError(#[from] IdentifierError),
}

impl BearerDid {
    pub async fn from_key_manager(
        did_uri: &str,
        key_manager: Arc<dyn KeyManager>,
    ) -> Result<Self, BearerDidError> {
        let resolution_result = Resolver::resolve_uri(did_uri).await;
        if let Some(err) = resolution_result.did_resolution_metadata.error {
            return Err(err)?;
        }

        Ok(BearerDid {
            identifier: Identifier::parse(did_uri)?,
            key_manager,
            document: resolution_result
                .did_document
                .ok_or(ResolutionError::NotFound)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::methods::{
        jwk::{DidJwk, DidJwkCreateOptions},
        Create,
    };
    use crypto::Curve;
    use keys::key_manager::local_key_manager::LocalKeyManager;

    #[tokio::test]
    async fn test_from_key_manager() {
        let key_manager = Arc::new(LocalKeyManager::new());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let did_jwk_bearer_did = DidJwk::create(key_manager.clone(), options).unwrap();
        let private_keys = key_manager.export_private_keys().unwrap();

        let bearer_did =
            BearerDid::from_key_manager(&did_jwk_bearer_did.identifier.uri, key_manager.clone())
                .await
                .unwrap();
        let bearer_did_private_keys = key_manager.export_private_keys().unwrap();

        assert_eq!(bearer_did.identifier.uri, did_jwk_bearer_did.identifier.uri);
        assert_eq!(private_keys.len(), bearer_did_private_keys.len());
        assert_eq!(
            private_keys[0].jwk().unwrap().d,
            bearer_did_private_keys[0].jwk().unwrap().d
        );
    }
}
