pub mod crypto;
pub mod dids;
pub mod jwk;
pub mod keys;

use ::crypto::{CryptoError, Curve};
use ::dids::identifier::IdentifierError;
use ::jwk::JwkError;
use ::keys::key_manager::KeyManagerError;
use crypto::Ed25199;
use dids::identifier::Identifier;
use jwk::Jwk;
use keys::LocalJwkManager;

uniffi::include_scaffolding!("web5");
