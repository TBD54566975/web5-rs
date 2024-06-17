pub mod ed25519;

use crate::errors::Result;
use std::sync::Arc;
use web5::apid::dsa::{Signer as InnerSigner, Verifier as InnerVerifier};

pub trait Signer: Send + Sync {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;
    fn to_signer(&self) -> Arc<dyn InnerSigner>;
}

pub trait Verifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool>;
    fn to_verifier(&self) -> Arc<dyn InnerVerifier>;
}
