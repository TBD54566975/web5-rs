uniffi::include_scaffolding!("web5_ffi");

use std::sync::Arc;

pub use web5::crypto::key::KeyAlgorithm;
pub use web5::crypto::key::PrivateKey;
use web5::crypto::key_manager::KeyManager;
pub use web5::crypto::key_store::{KeyStore, KeyStoreError};
