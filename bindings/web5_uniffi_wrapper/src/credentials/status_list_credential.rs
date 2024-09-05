use crate::credentials::verifiable_credential_1_1::VerifiableCredential;
use crate::errors::Result;
use std::sync::Arc;
use web5::credentials::Issuer;
use web5::{
    credentials::StatusListCredential as InnerStatusListCredential,
    credentials::VerifiableCredential as InnerVerifiableCredential, json::FromJson,
};

pub struct StatusListCredential(pub InnerStatusListCredential);

impl StatusListCredential {
    pub fn create(
        json_serialized_issuer: String,
        status_purpose: String,
        credentials_to_disable: Option<Vec<Arc<VerifiableCredential>>>,
    ) -> Result<Self> {
        let issuer = Issuer::from_json_string(&json_serialized_issuer)?;

        let inner_vcs: Option<Vec<InnerVerifiableCredential>> =
            credentials_to_disable.map(|credentials| {
                credentials
                    .into_iter()
                    .map(|vc| vc.inner_vc.clone())
                    .collect()
            });

        Ok(Self(InnerStatusListCredential::create(
            issuer,
            status_purpose,
            inner_vcs,
        )?))
    }

    pub fn get_base(&self) -> Result<Arc<VerifiableCredential>> {
        let vc = VerifiableCredential::from_inner(&self.0.base)?;
        Ok(Arc::new(vc))
    }

    pub fn is_disabled(&self, credential: Arc<VerifiableCredential>) -> Result<bool> {
        Ok(self.0.is_disabled(&credential.inner_vc.clone())?)
    }
}
