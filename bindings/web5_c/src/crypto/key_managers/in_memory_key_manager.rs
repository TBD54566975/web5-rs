use super::{
    add_key_manager_to_registry, rust_key_manager_get_signer, rust_key_manager_import_private_jwk,
    CKeyManager,
};
use std::sync::Arc;
use web5::crypto::key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager};

#[no_mangle]
pub extern "C" fn new_in_memory_key_manager() -> *mut CKeyManager {
    let manager: Arc<dyn KeyManager> = Arc::new(InMemoryKeyManager::new());

    let manager_id = add_key_manager_to_registry(manager);

    Box::into_raw(Box::new(CKeyManager {
        manager_id,
        import_private_jwk: rust_key_manager_import_private_jwk,
        get_signer: rust_key_manager_get_signer,
    }))
}
