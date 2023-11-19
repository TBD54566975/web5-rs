mod error;
mod key;
mod key_manager;

pub use crate::error::CryptoError;
pub use crate::key::PrivateKey;
pub use crate::key_manager::local::key_store::KeyStore;
pub use crate::key_manager::KeyManager;
pub use crypto::key::KeyAlgorithm;

uniffi::include_scaffolding!("crypto_ffi");
