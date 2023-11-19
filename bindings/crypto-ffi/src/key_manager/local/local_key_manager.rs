use crate::key_manager::local::key_store::{KeyStore, KeyStoreWrapper};
use crypto::key_manager::local::LocalKeyManager as RustLocalKeyManager;
use std::sync::Arc;

pub struct LocalKeyManager {
    inner: RustLocalKeyManager,
}

impl LocalKeyManager {
    pub fn new(key_store: Arc<dyn KeyStore>) -> Self {
        let wrapper = Arc::new(KeyStoreWrapper(key_store));
        Self {
            inner: RustLocalKeyManager::new(wrapper),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            inner: RustLocalKeyManager::new_in_memory(),
        }
    }
}
