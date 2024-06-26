use crate::{
    dids::bearer_did::BearerDid,
    dsa::{Signer, Verifier},
    errors::{Result, RustCoreError},
};
use std::sync::{Arc, RwLock};
use web5::apid::credentials::verifiable_credential_1_1::VerifiableCredential as InnerVerifiableCredential;

pub struct VerifiableCredential(pub Arc<RwLock<InnerVerifiableCredential>>);

impl VerifiableCredential {
    pub fn new(verifiable_credential: data::VerifiableCredential) -> Result<Self> {
        let inner_verifiable_credential = verifiable_credential.to_inner()?;

        Ok(Self(Arc::new(RwLock::new(inner_verifiable_credential))))
    }

    pub fn verify(vcjwt: &str) -> Result<Self> {
        let inner_verifiable_credential =
            InnerVerifiableCredential::verify(vcjwt).map_err(|e| Arc::new(e.into()))?;

        Ok(Self(Arc::new(RwLock::new(inner_verifiable_credential))))
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn Verifier>) -> Result<Self> {
        let inner_verifiable_credential =
            InnerVerifiableCredential::verify_with_verifier(vcjwt, verifier.to_inner())
                .map_err(|e| Arc::new(e.into()))?;

        Ok(Self(Arc::new(RwLock::new(inner_verifiable_credential))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<String> {
        let inner_verifiable_credential = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        inner_verifiable_credential
            .sign(&bearer_did.0)
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn sign_with_signer(&self, key_id: &str, signer: Arc<dyn Signer>) -> Result<String> {
        let inner_verifiable_credential = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        inner_verifiable_credential
            .sign_with_signer(key_id, signer.to_inner())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> Result<data::VerifiableCredential> {
        let inner_verifiable_credential = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        data::VerifiableCredential::from_inner(inner_verifiable_credential.clone())
    }
}

pub mod data {
    use super::*;

    #[derive(Clone)]
    pub struct VerifiableCredential {
        pub context: Vec<String>,
        pub id: String,
        pub r#type: Vec<String>,
        pub issuer: String, // JSON serialized
        pub issuance_date: String,
        pub expiration_date: Option<String>,
        pub credential_subject: String, // JSON serialized
    }

    impl VerifiableCredential {
        pub fn from_inner(inner_verifiable_credential: InnerVerifiableCredential) -> Result<Self> {
            Ok(Self {
                context: inner_verifiable_credential.context.clone(),
                id: inner_verifiable_credential.id.clone(),
                r#type: inner_verifiable_credential.r#type.clone(),
                issuer: serde_json::to_string(&inner_verifiable_credential.issuer)
                    .map_err(|e| Arc::new(e.into()))?,
                issuance_date: inner_verifiable_credential.issuance_date.clone(),
                expiration_date: inner_verifiable_credential.expiration_date.clone(),
                credential_subject: serde_json::to_string(
                    &inner_verifiable_credential.credential_subject,
                )
                .map_err(|e| Arc::new(e.into()))?,
            })
        }

        pub fn to_inner(&self) -> Result<InnerVerifiableCredential> {
            Ok(InnerVerifiableCredential {
                context: self.context.clone(),
                id: self.id.clone(),
                r#type: self.r#type.clone(),
                issuer: serde_json::from_str(&self.issuer).map_err(|e| Arc::new(e.into()))?,
                issuance_date: self.issuance_date.clone(),
                expiration_date: self.expiration_date.clone(),
                credential_subject: serde_json::from_str(&self.credential_subject)
                    .map_err(|e| Arc::new(e.into()))?,
            })
        }
    }
}
