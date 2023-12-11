mod error;
mod key;
mod key_manager;

pub use crate::error::CryptoError;
pub use crate::key::private_key::PrivateKeyFfi as PrivateKey;
pub use crate::key::public_key::PublicKeyFfi as PublicKey;
pub use crate::key_manager::key_store::KeyStoreFfi as KeyStore;
pub use crate::key_manager::KeyManagerFfi as KeyManager;
pub use crypto::key::KeyType;

uniffi::include_scaffolding!("crypto_ffi");
