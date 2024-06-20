use crate::{dsa::Signer, errors::Result};
use std::sync::Arc;
use web5::apid::crypto::{jwk::Jwk, key_managers::key_manager::KeyManager as InnerKeyManager};

pub trait KeyManager: Send + Sync {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
    fn to_inner(&self) -> Arc<dyn InnerKeyManager>;
}
