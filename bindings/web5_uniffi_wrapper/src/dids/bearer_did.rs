use crate::{
    crypto::{
        dsa::{Signer, ToOuterSigner},
        key_manager::{KeyManager, ToInnerKeyManager, ToOuterKeyManager},
    },
    errors::Result,
};
use std::sync::Arc;
use web5::dids::{
    bearer_did::BearerDid as InnerBearerDid, data_model::document::Document, did::Did,
};

use super::portable_did::PortableDid;

pub struct BearerDidData {
    pub did: Did,
    pub document: Document,
    pub key_manager: Arc<dyn KeyManager>,
}

pub struct BearerDid(pub InnerBearerDid);

impl BearerDid {
    pub fn new(uri: &str, key_manager: Arc<dyn KeyManager>) -> Result<Self> {
        let inner_key_manager = Arc::new(ToInnerKeyManager(key_manager));
        let inner = InnerBearerDid::new(uri, inner_key_manager)?;
        Ok(Self(inner))
    }

    pub fn from_portable_did(portable_did: Arc<PortableDid>) -> Result<Self> {
        let inner_bearer_did = InnerBearerDid::from_portable_did(portable_did.get_data())?;
        Ok(Self(inner_bearer_did))
    }

    pub fn get_data(&self) -> BearerDidData {
        let outer_key_manager = ToOuterKeyManager(self.0.key_manager.clone());

        BearerDidData {
            did: self.0.did.clone(),
            document: self.0.document.clone(),
            key_manager: Arc::new(outer_key_manager),
        }
    }

    pub fn get_signer(&self, key_id: String) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(key_id)?;
        let outer_signer = ToOuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }
}
