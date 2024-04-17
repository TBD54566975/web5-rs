pub mod crypto;
pub mod jwk;

use ::crypto::CryptoError;
use ::jwk::JwkError;
use ::keys::key_manager::KeyManagerError;
use crypto::Ed25199;
use jwk::Jwk;

uniffi::include_scaffolding!("web5");
