use crate::{dids::bearer_did::BearerDid, errors::Result};
use std::{sync::Arc, time::SystemTime};
use web5::{
    credentials::{
        credential_schema::CredentialSchema,
        verifiable_credential_1_1::{
            VerifiableCredential as InnerVerifiableCredential,
            VerifiableCredentialCreateOptions as InnerVerifiableCredentialCreateOptions,
        },
        CredentialSubject, Issuer,
    },
    json::{FromJson as _, JsonObject},
};

#[derive(Default)]
pub struct VerifiableCredentialCreateOptions {
    pub id: Option<String>,
    pub context: Option<Vec<String>>,
    pub r#type: Option<Vec<String>>,
    pub issuance_date: Option<SystemTime>,
    pub expiration_date: Option<SystemTime>,
    pub credential_schema: Option<CredentialSchema>,
    pub json_serialized_evidence: Option<String>,
}

pub struct VerifiableCredential {
    inner_vc: InnerVerifiableCredential,
    json_serialized_issuer: String,
    json_serialized_credential_subject: String,
}

impl VerifiableCredential {
    pub fn new(
        context: Vec<String>,
        r#type: Vec<String>,
        id: String,
        json_serialized_issuer: String,
        json_serialized_credential_subject: String,
        issuance_date: SystemTime,
        expiration_date: Option<SystemTime>,
        credential_schema: Option<CredentialSchema>,
        json_serialized_evidence: Option<String>,
    ) -> Result<Self> {
        let issuer = Issuer::from_json_string(&json_serialized_issuer)?;
        let credential_subject =
            CredentialSubject::from_json_string(&json_serialized_credential_subject)?;

        let evidence = match json_serialized_evidence {
            Some(evidence_string) => {
                Some(serde_json::from_str::<Vec<JsonObject>>(&evidence_string)?)
            }
            None => None,
        };

        Ok(Self {
            inner_vc: InnerVerifiableCredential {
                context,
                r#type,
                id,
                issuer,
                credential_subject,
                issuance_date,
                expiration_date,
                credential_schema,
                evidence,
            },
            json_serialized_issuer,
            json_serialized_credential_subject,
        })
    }

    pub fn create(
        json_serialized_issuer: String,
        json_serialized_credential_subject: String,
        options: Option<VerifiableCredentialCreateOptions>,
    ) -> Result<Self> {
        let issuer = Issuer::from_json_string(&json_serialized_issuer)?;
        let credential_subject =
            CredentialSubject::from_json_string(&json_serialized_credential_subject)?;

        let options = options.unwrap_or_default();
        let evidence = match options.json_serialized_evidence {
            Some(evidence_string) => {
                Some(serde_json::from_str::<Vec<JsonObject>>(&evidence_string)?)
            }
            None => None,
        };
        let inner_options = InnerVerifiableCredentialCreateOptions {
            id: options.id,
            context: options.context,
            r#type: options.r#type,
            issuance_date: options.issuance_date,
            expiration_date: options.expiration_date,
            credential_schema: options.credential_schema,
            evidence,
        };

        let inner_vc =
            InnerVerifiableCredential::create(issuer, credential_subject, Some(inner_options))?;

        Ok(Self {
            inner_vc,
            json_serialized_issuer,
            json_serialized_credential_subject,
        })
    }

    pub fn get_data(&self) -> Result<VerifiableCredentialData> {
        let json_serialized_evidence = match &self.inner_vc.evidence {
            Some(e) => Some(serde_json::to_string(e)?),
            None => None,
        };

        Ok(VerifiableCredentialData {
            context: self.inner_vc.context.clone(),
            id: self.inner_vc.id.clone(),
            r#type: self.inner_vc.r#type.clone(),
            json_serialized_issuer: self.json_serialized_issuer.clone(),
            json_serialized_credential_subject: self.json_serialized_credential_subject.clone(),
            issuance_date: self.inner_vc.issuance_date,
            expiration_date: self.inner_vc.expiration_date,
            credential_schema: self.inner_vc.credential_schema.clone(),
            json_serialized_evidence,
        })
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
    pub credential_schema: Option<CredentialSchema>,
    pub json_serialized_evidence: Option<String>,
}
