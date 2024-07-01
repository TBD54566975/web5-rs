use crate::{
    crypto::dsa::{Signer, ToInnerSigner, ToInnerVerifier, Verifier},
    dids::bearer_did::BearerDid,
    errors::{Result, RustCoreError},
};
use std::sync::{Arc, RwLock};
use web5::credentials::verifiable_credential_1_1::VerifiableCredential as InnerVerifiableCredential;

pub struct VerifiableCredential(pub Arc<RwLock<InnerVerifiableCredential>>);

impl VerifiableCredential {
    pub fn new(verifiable_credential: data::VerifiableCredential) -> Result<Self> {
        let inner_verifiable_credential = verifiable_credential.to_inner()?;

        Ok(Self(Arc::new(RwLock::new(inner_verifiable_credential))))
    }

    pub fn verify(vcjwt: &str) -> Result<Self> {
        let inner_verifiable_credential = InnerVerifiableCredential::verify(vcjwt)?;

        Ok(Self(Arc::new(RwLock::new(inner_verifiable_credential))))
    }

    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn Verifier>) -> Result<Self> {
        let inner_verifier = Arc::new(ToInnerVerifier(verifier));
        let inner_verifiable_credential =
            InnerVerifiableCredential::verify_with_verifier(vcjwt, inner_verifier)?;

        Ok(Self(Arc::new(RwLock::new(inner_verifiable_credential))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<String> {
        let inner_verifiable_credential = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(inner_verifiable_credential.sign(&bearer_did.0)?)
    }

    pub fn sign_with_signer(&self, key_id: &str, signer: Arc<dyn Signer>) -> Result<String> {
        let inner_verifiable_credential = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        let inner_signer = Arc::new(ToInnerSigner(signer));
        Ok(inner_verifiable_credential.sign_with_signer(key_id, inner_signer)?)
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
    use std::time::SystemTime;

    #[derive(Clone)]
    pub struct VerifiableCredential {
        pub context: Vec<String>,
        pub id: String,
        pub r#type: Vec<String>,
        pub json_serialized_issuer: String, // JSON serialized
        pub issuance_date: SystemTime,
        pub expiration_date: Option<SystemTime>,
        pub json_serialized_credential_subject: String, // JSON serialized
    }

    impl VerifiableCredential {
        pub fn from_inner(inner_verifiable_credential: InnerVerifiableCredential) -> Result<Self> {
            Ok(Self {
                context: inner_verifiable_credential.context.clone(),
                id: inner_verifiable_credential.id.clone(),
                r#type: inner_verifiable_credential.r#type.clone(),
                json_serialized_issuer: serde_json::to_string(&inner_verifiable_credential.issuer)?,
                issuance_date: inner_verifiable_credential.issuance_date,
                expiration_date: inner_verifiable_credential.expiration_date,
                json_serialized_credential_subject: serde_json::to_string(
                    &inner_verifiable_credential.credential_subject,
                )?,
            })
        }

        pub fn to_inner(&self) -> Result<InnerVerifiableCredential> {
            Ok(InnerVerifiableCredential {
                context: self.context.clone(),
                id: self.id.clone(),
                r#type: self.r#type.clone(),
                issuer: serde_json::from_str(&self.json_serialized_issuer)?,
                issuance_date: self.issuance_date,
                expiration_date: self.expiration_date,
                credential_subject: serde_json::from_str(&self.json_serialized_credential_subject)?,
            })
        }
    }
}
