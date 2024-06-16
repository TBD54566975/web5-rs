pub mod ed25519;

use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::dsa::{Signer as InnerSigner, Verifier as InnerVerifier};

pub trait RcbSigner: Send + Sync {
    fn sign(&self, payload: &[u8]) -> RcbResult<Vec<u8>>;
    fn to_inner(&self) -> Arc<dyn InnerSigner>;
}

pub trait RcbVerifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> RcbResult<bool>;
    fn to_inner(&self) -> Arc<dyn InnerVerifier>;
}
