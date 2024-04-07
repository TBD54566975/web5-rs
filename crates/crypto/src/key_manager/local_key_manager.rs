use crate::key::{KeyType, PrivateKey, PublicKey};
use crate::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crate::key_manager::key_store::KeyStore;
use crate::key_manager::{KeyManager, KeyManagerError};
use std::sync::Arc;

/// Generalized implementation of the `KeyManager` trait that can work with any key types.
pub struct LocalKeyManager<T, U>
where
    T: PrivateKey<U> + 'static,
    U: PublicKey + 'static,
{
    key_store: Arc<dyn KeyStore<T, U>>,
}

impl<T, U> LocalKeyManager<T, U>
where
    T: PrivateKey<U> + Clone + Send + Sync + 'static, // Ensure T meets all required bounds
    U: PublicKey + Send + Sync + 'static,             // Ensure U meets required thread safety
{
    /// Constructs a new `LocalKeyManager` that stores keys in the provided `KeyStore`.
    pub fn new(key_store: Arc<dyn KeyStore<T, U>>) -> Self {
        Self { key_store }
    }

    pub fn new_in_memory() -> Self {
        Self {
            key_store: Arc::new(InMemoryKeyStore::<T, U>::new()),
        }
    }
}

impl<T, U> KeyManager<U> for LocalKeyManager<T, U>
where
    T: PrivateKey<U> + 'static,
    U: PublicKey + 'static,
{
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, KeyManagerError> {
        let private_key =
            T::generate(key_type).map_err(|_| KeyManagerError::KeyGenerationFailed)?;
        let public_key = private_key
            .to_public()
            .map_err(|_| KeyManagerError::KeyGenerationFailed)?;
        let key_alias = self.alias(&public_key)?;

        self.key_store.insert(&key_alias, private_key)?;

        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<U>, KeyManagerError> {
        if let Some(private_key) = self.key_store.get(key_alias)? {
            let public_key = private_key.to_public()?;
            Ok(Some(public_key))
        } else {
            Ok(None)
        }
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let private_key = self
            .key_store
            .get(key_alias)?
            .ok_or(KeyManagerError::SigningKeyNotFound)?;

        let signed_payload = private_key.sign(payload)?;

        Ok(signed_payload)
    }

    fn alias(&self, public_key: &U) -> Result<String, KeyManagerError> {
        let alias = public_key.alias().map_err(KeyManagerError::KeyError)?;
        Ok(alias)
    }
}
