mod did_jwk;
mod did_key;
mod error;

use crate::did_jwk::DidJwk;
use crate::did_key::DidKey;
use crate::error::DidsError;
use crypto_ffi::{KeyManager, KeyType};
use dids::method::jwk::DidJwkCreateOptions;
use dids::method::key::DidKeyCreateOptions;

uniffi::include_scaffolding!("dids_ffi");
