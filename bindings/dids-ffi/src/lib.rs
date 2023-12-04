mod did_jwk;
mod did_key;
mod error;

pub use crate::did_jwk::DidJwk;
pub use crate::did_key::DidKey;
pub use crate::error::DidsError;
pub use crypto_ffi::{KeyManager, KeyType};
pub use dids::did::{DidJwkCreateOptions, DidKeyCreateOptions};

uniffi::include_scaffolding!("dids_ffi");
