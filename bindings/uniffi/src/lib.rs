use ::credentials::vc::{verify_vcjwt, CredentialError, CredentialSubject, VerifiableCredential};
use ::crypto::Curve;
use ::dids::{
    bearer::{BearerDid, BearerDidError},
    document::{KeySelector, VerificationMethodType},
    method::{
        jwk::{DidJwk, DidJwkCreateOptions},
        Method, MethodError,
    },
};
use ::jwk::{Jwk, JwkError};
use ::jwt::{sign_jwt, verify_jwt, Claims, JwtError};
use ::keys::{
    key::{Key, KeyError, PrivateKey, PublicKey},
    key_manager::{
        key_store::{in_memory_key_store::InMemoryKeyStore, KeyStore, KeyStoreError},
        local_key_manager::LocalKeyManager,
        KeyManager, KeyManagerError,
    },
};
use std::sync::Arc;

pub async fn bearer_did_from_key_manager(
    did_uri: &str,
    key_manager: Arc<dyn KeyManager>,
) -> Result<Arc<BearerDid>, BearerDidError> {
    let bearer_did = BearerDid::from_key_manager(did_uri, key_manager).await?;
    Ok(Arc::new(bearer_did))
}

pub fn create_did_jwk(
    key_manager: Arc<LocalKeyManager>,
    options: DidJwkCreateOptions,
) -> Result<Arc<BearerDid>, MethodError> {
    let bearer_did = DidJwk::create(key_manager, options)?;
    Ok(Arc::new(bearer_did))
}

pub fn private_key_from_jwk(jwk: Arc<Jwk>) -> Arc<dyn PrivateKey> {
    return jwk;
}

uniffi::include_scaffolding!("web5");
