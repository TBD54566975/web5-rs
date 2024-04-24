pub mod any;
pub mod credentials;

use ::credentials::vc::{verify_vcjwt, CredentialError, CredentialSubject, VerifiableCredential};
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
use any::{Any, AnyError};
use credentials::new_verifiable_credential;
use std::sync::Arc;

pub async fn bearer_did_from_key_manager(
    did_uri: &str,
    key_manager: Arc<dyn KeyManager>,
) -> Result<Arc<BearerDid>, BearerDidError> {
    let bearer_did = BearerDid::from_key_manager(did_uri, key_manager).await?;
    Ok(Arc::new(bearer_did))
}

uniffi::include_scaffolding!("web5");
