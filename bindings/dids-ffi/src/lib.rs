mod error;
mod method;

use crate::error::DidsError;
use crate::method::jwk::DidJwkFfi as DidJwk;
use crate::method::key::DidKeyFfi as DidKey;
use crypto_ffi::{KeyManager, KeyType};
use dids::method::jwk::DidJwkCreateOptions;
use dids::method::key::DidKeyCreateOptions;

uniffi::include_scaffolding!("dids_ffi");
