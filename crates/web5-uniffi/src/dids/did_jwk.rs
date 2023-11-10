use crate::crypto::key::KeyAlgorithm;
use crate::crypto::key_manager::KeyManager;
use did_jwk::DIDJWK;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct DidJwk {
    pub uri: String,
}

impl DidJwk {
    #[uniffi::constructor]
    pub fn new(key_algorithm: KeyAlgorithm, key_manager: Arc<dyn KeyManager>) -> Arc<Self> {
        // TODO: handle the error properly
        let private_key = key_manager.generate_private_key(key_algorithm).unwrap();
        let uri = DIDJWK.generate(&Source::Key(&private_key.0)).unwrap();

        Self { uri }.into()
    }
}
