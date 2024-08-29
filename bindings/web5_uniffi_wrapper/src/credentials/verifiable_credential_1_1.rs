use crate::{dids::bearer_did::BearerDid, errors::Result};
use std::{sync::Arc, time::SystemTime};
use web5::{
    credentials::{
        verifiable_credential_1_1::{
            VerifiableCredential as InnerVerifiableCredential, VerifiableCredentialCreateOptions,
        },
        CredentialSubject, Issuer,
    },
    json::FromJson as _,
};

pub struct VerifiableCredential {
    inner_vc: InnerVerifiableCredential,
    json_serialized_issuer: String,
    json_serialized_credential_subject: String,
}

impl VerifiableCredential {
    pub fn create(
        json_serialized_issuer: String,
        json_serialized_credential_subject: String,
        options: Option<VerifiableCredentialCreateOptions>,
    ) -> Result<Self> {
        let issuer = Issuer::from_json_string(&json_serialized_issuer)?;
        let credential_subject =
            CredentialSubject::from_json_string(&json_serialized_credential_subject)?;

        let inner_vc = InnerVerifiableCredential::create(issuer, credential_subject, options)?;

        Ok(Self {
            inner_vc,
            json_serialized_issuer,
            json_serialized_credential_subject,
        })
    }

    pub fn get_data(&self) -> VerifiableCredentialData {
        VerifiableCredentialData {
            context: self.inner_vc.context.clone(),
            id: self.inner_vc.id.clone(),
            r#type: self.inner_vc.r#type.clone(),
            json_serialized_issuer: self.json_serialized_issuer.clone(),
            json_serialized_credential_subject: self.json_serialized_credential_subject.clone(),
            issuance_date: self.inner_vc.issuance_date,
            expiration_date: self.inner_vc.expiration_date,
        }
    }

    pub fn from_vc_jwt(vc_jwt: String, verify: bool) -> Result<Self> {
        let inner_vc = InnerVerifiableCredential::from_vc_jwt(&vc_jwt, verify)?;
        let json_serialized_issuer = serde_json::to_string(&inner_vc.issuer)?;
        let json_serialized_credential_subject =
            serde_json::to_string(&inner_vc.credential_subject)?;

        Ok(Self {
            inner_vc,
            json_serialized_issuer,
            json_serialized_credential_subject,
        })
    }

    pub fn sign(
        &self,
        bearer_did: Arc<BearerDid>,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        let vc_jwt = self.inner_vc.sign(&bearer_did.0, verification_method_id)?;
        Ok(vc_jwt)
    }
}

#[derive(Clone)]
pub struct VerifiableCredentialData {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub json_serialized_issuer: String,
    pub json_serialized_credential_subject: String,
    pub issuance_date: SystemTime,
    pub expiration_date: Option<SystemTime>,
}
