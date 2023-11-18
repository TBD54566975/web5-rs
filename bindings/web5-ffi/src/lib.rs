uniffi::include_scaffolding!("web5_ffi");

use std::sync::Arc;

pub use web5::crypto::key::KeyAlgorithm;
pub use web5::crypto::key::{PrivateKey, PublicKey};
pub use web5::crypto::key_store::InMemoryKeyStore;
pub use web5::did::did::Did;
pub use web5::error::Web5Error;
use web5::result::Web5Result;

// Super hacky way to get pure Rust trait exposed as a foreign implementation trait.
// I have tried multiple other ways, with no success. I would love if someone could
// show me a better way to do this.
//
// What the below does:
// * Rename the pure rust trait to RustKeyStore
// * Define a uniffi-compatible KeyStore trait, with supported params & return types
// * Create a wrapper struct, which will execute the foreign language implementation when
//   the pure rust trait is called.
//
// This means we're creating some additional memory, because it's cloning the values that
// come in from the foreign language implementation.
//
// :shrug: Tradeoffs :shrug:

pub use web5::crypto::key_store::KeyStore as RustKeyStore;

pub trait KeyStore: Send + Sync {
    fn get_private_key(&self, key_alias: String) -> Result<Option<Arc<PrivateKey>>, KeyStoreError>;
    fn insert_private_key(
        &self,
        key_alias: String,
        private_key: Arc<PrivateKey>,
    ) -> Result<(), KeyStoreError>;
}

pub struct KeyStoreWrapper(Arc<dyn KeyStore>);

impl RustKeyStore for KeyStoreWrapper {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, RustKeyStoreError> {
        let private_key = self
            .0
            .get_private_key(key_alias.to_string())
            .map_err(|e| e.into())?;

        if let Some(private_key) = private_key {
            Ok(Some((*private_key).clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), RustKeyStoreError> {
        let key_alias = key_alias.to_string();
        let private_key = Arc::new(private_key);
        self.0
            .insert_private_key(key_alias, private_key)
            .map_err(|e| e.into())
    }
}

// Foreign languages can throw any arbitrary error. We need handle those!
// In order to do this, the Error type exposed to foreign languages must implement
// From<uniffi::UnexpectedUniFFICallbackError>. Unfortunately, we can't directly
// do this for RustKeyStoreError, because it's defined in an external crate.
//
// So, in order to support this, we define our own KeyStoreError, which DOES implement
// the required trait. Then, we implement Into<RustKeyStoreError> for our KeyStoreError
// to get BACK into pure-Rust land, with an error that it understand.

pub use web5::crypto::key_store::KeyStoreError as RustKeyStoreError;

#[derive(thiserror::Error, Debug)]
pub enum KeyStoreError {
    #[error("{message}")]
    InternalKeyStoreError { message: String },
}

impl From<uniffi::UnexpectedUniFFICallbackError> for KeyStoreError {
    fn from(value: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::InternalKeyStoreError {
            message: value.reason,
        }
    }
}

impl Into<RustKeyStoreError> for KeyStoreError {
    fn into(self) -> RustKeyStoreError {
        match self {
            KeyStoreError::InternalKeyStoreError { message } => {
                RustKeyStoreError::InternalKeyStoreError { message }
            }
        }
    }
}

// Cleaner KeyManager interface to foreign languages

pub use web5::crypto::key_manager::KeyManager as RustKeyManager;
use web5::crypto::key_manager::KeyManagerError;
use web5::crypto::key_manager::LocalKeyManager;

pub struct KeyManager(Arc<dyn RustKeyManager>);

impl KeyManager {
    pub fn in_memory() -> Self {
        let key_store = InMemoryKeyStore::new();
        let key_manager = LocalKeyManager::new(Arc::new(key_store));
        Self(Arc::new(key_manager))
    }

    pub fn key_store(key_store: Arc<dyn KeyStore>) -> Self {
        let wrapper = KeyStoreWrapper(key_store);
        let key_manager = LocalKeyManager::new(Arc::new(wrapper));
        Self(Arc::new(key_manager))
    }
}

impl RustKeyManager for KeyManager {
    fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, KeyManagerError> {
        self.0.generate_private_key(key_algorithm)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError> {
        self.0.get_public_key(key_alias)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        self.0.sign(key_alias, payload)
    }

    fn get_deterministic_alias(&self, public_key: PublicKey) -> Result<String, KeyManagerError> {
        self.0.get_deterministic_alias(public_key)
    }
}

// DID JWK
// Reasons I need to do the below:
// 1. key_manager comes in as an Arc from uniffi
// 2. the key manager from uniffi is NOT the RustKeyManager
// 3. uri needs to return a new String, not a string slice

pub use web5::did::did_method::did_jwk::{DidJwk as RustDidJwk, DidJwkCreateOptions};

pub struct DidJwk {
    inner: RustDidJwk,
}

impl DidJwk {
    fn new(key_manager: Arc<KeyManager>, options: DidJwkCreateOptions) -> Web5Result<Self> {
        let inner = RustDidJwk::new(key_manager.0.clone(), options)?;
        Ok(Self { inner })
    }

    fn uri(&self) -> String {
        self.inner.uri.to_string()
    }
}

// DID Key
// Reasons I need to do the below:
// 1. key_manager comes in as an Arc from uniffi
// 2. the key manager from uniffi is NOT the RustKeyManager
// 3. uri needs to return a new String, not a string slice

pub use web5::did::did_method::did_key::{DidKey as RustDidKey, DidKeyCreateOptions};

pub struct DidKey {
    inner: RustDidKey,
}

impl DidKey {
    fn new(key_manager: Arc<KeyManager>, options: DidKeyCreateOptions) -> Web5Result<Self> {
        let inner = RustDidKey::new(key_manager.0.clone(), options)?;
        Ok(Self { inner })
    }

    fn uri(&self) -> String {
        self.inner.uri.to_string()
    }
}
