pub mod error;
pub mod key;
pub mod key_manager;

pub use crate::error::CryptoError;
pub use crate::key::private_key::PrivateKey;
pub use crate::key::public_key::PublicKey;
pub use crate::key_manager::key_store::{KeyStore, KeyStoreTrait};
pub use crate::key_manager::{KeyManager, KeyManagerTrait};
pub use crypto::key::KeyType;

uniffi::include_scaffolding!("crypto_ffi");
