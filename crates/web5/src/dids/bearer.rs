use crate::dids::{
    document::{Document, DocumentError, KeyIdFragment, KeySelector},
    identifier::{Identifier, IdentifierError},
    resolver::{ResolutionError, Resolver},
};
use crate::keys::{
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

    pub fn sign(
        &self,
        key_selector: &KeySelector,
        payload: &[u8],
    ) -> Result<Vec<u8>, BearerDidError> {
        let verification_method = self.document.get_verification_method(key_selector)?;
        let key_alias = KeyIdFragment(verification_method.id.clone()).splice_key_alias();
        let signature = self.key_manager.sign(&key_alias, payload)?;
        Ok(signature)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::crypto::Curve;
    use crate::dids::{
        document::VerificationMethodType,
        methods::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Create,
        },
    };
    use crate::keys::{key::PublicKey, key_manager::local_key_manager::LocalKeyManager};

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

    #[test]
    fn test_sign() {
        let key_manager = Arc::new(LocalKeyManager::new());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager.clone(), options).unwrap();

        let payload = b"hello world";
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let signature = bearer_did.sign(&key_selector, payload).unwrap();

        assert_ne!(0, signature.len());

        let vm = bearer_did
            .document
            .get_verification_method(&key_selector)
            .unwrap();
        vm.public_key_jwk.verify(payload, &signature).unwrap();
    }
}
