use crypto::Curve;
use keys::key_manager::{
    local_key_manager::LocalKeyManager as InternalLocalKeyManager, KeyManager, KeyManagerError,
};

pub struct LocalJwkManager(InternalLocalKeyManager);

impl LocalJwkManager {
    pub fn new() -> Self {
        Self(InternalLocalKeyManager::new_in_memory())
    }

    pub fn generate_private_key(
        &self,
        curve: Curve,
        key_alias: Option<String>,
    ) -> Result<String, KeyManagerError> {
        self.0
            .generate_private_key(curve, key_alias.as_ref().map(|x| x.as_str()))
    }
}
