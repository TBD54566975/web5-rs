pub mod crypto;
pub mod dids;
pub mod jwk;
pub mod keys;

use ::crypto::{CryptoError, Curve};
use ::dids::{
    document::{Document, DocumentError, Service, VerificationMethod, VerificationMethodType},
    identifier::{Identifier, IdentifierError},
};
use ::jwk::{Jwk, JwkError};
use ::keys::key_manager::KeyManagerError;
use crypto::{ed25519_generate, ed25519_sign, ed25519_verify};
use dids::{
    document::{get_verification_method, KeySelector},
    identifier::identifier_parse,
};
use jwk::compute_thumbprint;
use keys::LocalJwkManager;

uniffi::include_scaffolding!("web5");
