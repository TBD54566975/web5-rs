pub mod key_store;

use crate::key_manager::key_store::KeyStoreFfi;
use crypto::key_manager::local_key_manager::LocalKeyManager;
use crypto::key_manager::KeyManager;
use std::sync::Arc;

pub struct KeyManagerFfi(Box<dyn KeyManager>);

impl KeyManagerFfi {
    pub fn new(key_store: Arc<KeyStoreFfi>) -> Self {
        Self(Box::new(LocalKeyManager::new(key_store)))
    }
}
