use std::sync::Arc;

use crate::identifier::Identifier;
use crate::method::{MethodError, Resolve};
use crate::resolver::ResolutionResult;
use crate::{bearer::BearerDid, method::Create};
use crate::{
    document::{Document, VerificationMethod},
    method::Method,
};
use crypto::Curve;
use did_jwk::DIDJWK as SpruceDidJwkMethod;
use keys::key_manager::KeyManager;
use serde_json::from_str;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use ssi_dids::{DIDMethod, Source};
use ssi_jwk::JWK as SpruceJwk;

/// Concrete implementation for a did:jwk DID
pub struct DidJwk;

/// Options that can be used to create a did:jwk DID
pub struct DidJwkCreateOptions {
    pub curve: Curve,
}

impl Method for DidJwk {
    const NAME: &'static str = "jwk";
}

impl Create<DidJwkCreateOptions> for DidJwk {
    fn create(
        key_manager: Arc<dyn KeyManager>,
        options: DidJwkCreateOptions,
    ) -> Result<BearerDid, MethodError> {
        let key_alias = key_manager.generate_private_key(options.curve, Some("0".to_string()))?;
        let public_key = key_manager.get_public_key(&key_alias)?;
        let public_jwk = public_key.jwk()?;
        let jwk_string = serde_json::to_string(public_jwk.as_ref()).map_err(|_| {
            MethodError::DidCreationFailure("failed to serialize public jwk".to_string())
        })?;
        let spruce_jwk: SpruceJwk =
            from_str(&jwk_string).map_err(|e| MethodError::DidCreationFailure(e.to_string()))?;

        let uri = SpruceDidJwkMethod
            .generate(&Source::Key(&spruce_jwk))
            .ok_or(MethodError::DidCreationFailure(
                "Failed to generate did:jwk".to_string(),
            ))?;

        let identifier = Identifier::parse(&uri).map_err(|e| {
            MethodError::DidCreationFailure(format!("Failed to parse did:jwk uri {} {}", &uri, e))
        })?;

        let verification_method_id = format!("{}#0", uri);

        let verification_method = VerificationMethod {
            id: verification_method_id.clone(),
            r#type: "JsonWebKey".to_string(),
            controller: uri.clone(),
            public_key_jwk: public_jwk.as_ref().clone(),
        };

        let document = Document {
            id: uri.clone(),
            verification_method: vec![verification_method.clone()],
            authentication: Some(vec![verification_method_id.clone()]),
            assertion_method: Some(vec![verification_method_id.clone()]),
            capability_invocation: Some(vec![verification_method_id.clone()]),
            capability_delegation: Some(vec![verification_method_id.clone()]),
            ..Default::default()
        };

        let bearer_did = BearerDid {
            identifier,
            key_manager,
            document,
        };

        Ok(bearer_did)
    }
}

impl Resolve for DidJwk {
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
    use super::*;
    use crate::resolver::ResolutionError;
    use keys::key_manager::local_key_manager::LocalKeyManager;

    fn create_did_jwk() -> BearerDid {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };

        DidJwk::create(key_manager, options).unwrap()
    }

    #[test]
    fn create_produces_correct_uri() {
        let bearer_did = create_did_jwk();
        assert!(bearer_did.identifier.uri.starts_with("did:jwk:"));
    }

    #[test]
    fn create_produces_correct_did_document() {
        let bearer_did = create_did_jwk();

        let verification_method_id = bearer_did.document.verification_method[0].id.clone();
        assert_eq!(
            bearer_did.document.authentication.unwrap()[0],
            verification_method_id
        );
        assert_eq!(
            bearer_did.document.assertion_method.unwrap()[0],
            verification_method_id
        );
        assert_eq!(
            bearer_did.document.capability_invocation.unwrap()[0],
            verification_method_id
        );
        assert_eq!(
            bearer_did.document.capability_delegation.unwrap()[0],
            verification_method_id
        );
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

        let verification_method_id = did_document.verification_method[0].id.clone();
        assert_eq!(
            did_document.authentication.unwrap()[0],
            verification_method_id
        );
        assert_eq!(
            did_document.assertion_method.unwrap()[0],
            verification_method_id
        );
        assert_eq!(
            did_document.capability_invocation.unwrap()[0],
            verification_method_id
        );
        assert_eq!(
            did_document.capability_delegation.unwrap()[0],
            verification_method_id
        );
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
