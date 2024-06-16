use std::sync::Arc;
use web5::apid::credentials::verifiable_credential_11::VerifiableCredential as InnerVerifiableCredential;

use crate::{
    dsa::{RcbSigner, RcbVerifier},
    errors::RcbResult,
};

pub struct RcbVerifiableCredential(InnerVerifiableCredential);

impl RcbVerifiableCredential {
    pub fn new(verifiable_credential: InnerVerifiableCredential) -> Self {
        Self {
            0: verifiable_credential,
        }
    }

    pub fn verify(vcjwt: &str) -> RcbResult<Self> {
        let inner = InnerVerifiableCredential::verify(vcjwt).map_err(|e| Arc::new(e.into()))?;
        Ok(Self { 0: inner })
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn RcbVerifier>) -> RcbResult<Self> {
        let inner = InnerVerifiableCredential::verify_with_verifier(vcjwt, verifier.to_inner())
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self { 0: inner })
    }

    pub fn sign(&self, signer: Arc<dyn RcbSigner>) -> RcbResult<String> {
        self.0
            .sign(signer.to_inner())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> InnerVerifiableCredential {
        self.0.clone()
    }
}
