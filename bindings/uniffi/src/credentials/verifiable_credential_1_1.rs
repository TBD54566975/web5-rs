use crate::{
    dsa::{Signer, Verifier},
    errors::Result,
};
use std::sync::Arc;
use web5::apid::credentials::verifiable_credential_1_1::VerifiableCredential as InnerVerifiableCredential;

pub struct VerifiableCredential(pub InnerVerifiableCredential);

impl VerifiableCredential {
    pub fn new(verifiable_credential: InnerVerifiableCredential) -> Self {
        Self(verifiable_credential)
    }

    pub fn verify(vcjwt: &str) -> Result<Self> {
        let vc = InnerVerifiableCredential::verify(vcjwt).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(vc))
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn Verifier>) -> Result<Self> {
        let vc = InnerVerifiableCredential::verify_with_verifier(vcjwt, verifier.to_verifier())
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(vc))
    }

    pub fn sign(&self, signer: Arc<dyn Signer>) -> Result<String> {
        self.0
            .sign(signer.to_signer())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> InnerVerifiableCredential {
        self.0.clone()
    }
}
