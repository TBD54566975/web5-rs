use ::credentials::vc::{CredentialError, CredentialSubject, VerifiableCredential};
use ::crypto::Curve;
use ::dids::{
    bearer::{BearerDid, BearerDidError},
    document::{KeySelector, VerificationMethodType},
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

pub async fn verify_vcjwt(jwt: &str) -> Result<Arc<VerifiableCredential>, CredentialError> {
    let vc = VerifiableCredential::verify(jwt).await?;
    Ok(Arc::new(vc))
}

pub fn decode_vcjwt(jwt: &str) -> Result<Arc<VerifiableCredential>, CredentialError> {
    let vc = VerifiableCredential::decode(jwt)?;
    Ok(Arc::new(vc))
}

uniffi::include_scaffolding!("web5");
