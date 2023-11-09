use crate::{KeyAlgorithm, KeyManager};
use did_jwk::DIDJWK;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct DidJwk {
    uri: String,
}

impl DidJwk {
    #[uniffi::constructor]
    pub fn new(key_algorithm: KeyAlgorithm, key_manager: KeyManager) -> Arc<Self> {
        let private_key = key_manager.generate_private_key(key_algorithm);
        let uri = DIDJWK.generate(&Source::Key(&private_key.0)).unwrap();

        Self { uri }.into()
    }
}
