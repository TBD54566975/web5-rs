use crate::{
    crypto::{
        dsa::{OuterSigner, Signer},
        key_manager::{KeyManager, OuterKeyManager},
    },
    errors::Result,
};
use std::sync::Arc;
use web5::dids::{
    bearer_did::BearerDid as InnerBearerDid, data_model::document::Document, did::Did,
};

pub struct BearerDidData {
    pub did: Did,
    pub document: Document,
    pub key_manager: Arc<dyn KeyManager>,
}

pub struct BearerDid(pub InnerBearerDid);

impl BearerDid {
    pub fn new(uri: &str, key_manager: Arc<dyn KeyManager>) -> Result<Self> {
        let inner =
            InnerBearerDid::new(uri, key_manager.to_inner()).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(inner))
    }

    pub fn get_data(&self) -> BearerDidData {
        let outer_key_manager = OuterKeyManager(self.0.key_manager.clone());

        BearerDidData {
            did: self.0.did.clone(),
            document: self.0.document.clone(),
            key_manager: Arc::new(outer_key_manager),
        }
    }

    pub fn get_signer(&self, key_id: String) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(key_id).map_err(|e| Arc::new(e.into()))?;
        let outer_signer = OuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }
}
