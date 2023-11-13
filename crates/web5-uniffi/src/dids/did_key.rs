use crate::crypto::key::KeyAlgorithm;
use crate::crypto::key_manager::KeyManager;
use did_method_key::DIDKey;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct DidKey {
    pub uri: String,
}

#[uniffi::export]
impl DidKey {
    #[uniffi::constructor]
    pub fn new(key_algorithm: KeyAlgorithm, key_manager: Arc<KeyManager>) -> Arc<Self> {
        // TODO: handle the error properly
        let key_alias = key_manager.generate_private_key(key_algorithm).unwrap();
        let private_key = key_manager
            .get_public_key(key_alias)
            .unwrap()
            .expect("public key not found immediately after creating the private key");
        let uri = DIDKey.generate(&Source::Key(&private_key.0)).unwrap();

        Self { uri }.into()
    }

    pub fn get_uri(&self) -> String {
        self.uri.clone()
    }
}
