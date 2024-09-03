use crate::{
    crypto::{
        dsa::{Signer, ToOuterSigner},
        key_exporter::{KeyExporter, ToInnerKeyExporter},
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
    pub fn new(did: Did, document: Document, key_manager: Arc<dyn KeyManager>) -> Self {
        let inner_key_manager = Arc::new(ToInnerKeyManager(key_manager));
        Self(InnerBearerDid {
            did,
            document,
            key_manager: inner_key_manager,
        })
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

    pub fn get_signer(&self, verification_method_id: String) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(&verification_method_id)?;
        let outer_signer = ToOuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }

    pub fn to_portable_did(&self, key_exporter: Arc<dyn KeyExporter>) -> Result<Arc<PortableDid>> {
        let inner_key_exporter = Arc::new(ToInnerKeyExporter(key_exporter));
        let inner_portable_did = self.0.to_portable_did(inner_key_exporter)?;
        Ok(Arc::new(PortableDid(inner_portable_did)))
    }
}
