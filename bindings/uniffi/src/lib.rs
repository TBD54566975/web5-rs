use ::crypto::Curve;
use ::dids::{
    bearer::{BearerDid, BearerDidError},
    document::{KeySelector, VerificationMethodType},
};
use ::jwk::{Jwk, JwkError};
use ::keys::{
    key::{Key, KeyError, PrivateKey, PublicKey},
    key_manager::{
        key_store::{in_memory_key_store::InMemoryKeyStore, KeyStore, KeyStoreError},
        local_key_manager::LocalKeyManager,
        KeyManager, KeyManagerError,
    },
};
use jws::{JwsCompactSerialized, JwsDecoded, JwsError, JwsHeader};
use std::sync::Arc;

pub async fn bearer_did_from_key_manager(
    did_uri: &str,
    key_manager: Arc<dyn KeyManager>,
) -> Result<Arc<BearerDid>, BearerDidError> {
    let bearer_did = BearerDid::from_key_manager(did_uri, key_manager).await?;
    Ok(Arc::new(bearer_did))
}

pub fn compactjws_sign(
    bearer_did: &BearerDid,
    key_selector: &KeySelector,
    header: &JwsHeader,
    payload: &[u8], // JSON string as a byte array, TODO add a doc comment for this
) -> Result<String, JwsError> {
    JwsCompactSerialized::sign(bearer_did, key_selector, header, payload)
}

pub fn compactjws_decode(compact_jws: &str) -> Result<JwsDecoded, JwsError> {
    JwsCompactSerialized::decode(compact_jws)
}

pub async fn compactjws_verify(compact_jws: &str) -> Result<JwsDecoded, JwsError> {
    JwsCompactSerialized::verify(compact_jws).await
}

uniffi::include_scaffolding!("web5");
