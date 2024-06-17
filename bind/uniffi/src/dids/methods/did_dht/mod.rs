use std::sync::Arc;
use web5::apid::{dids::methods::did_dht::DidDht, jwk::Jwk};

use crate::{dsa::RcbSigner, errors::RcbResult};

pub struct RcbDidDht(DidDht);

impl RcbDidDht {
    pub fn from_identity_key(public_key: Jwk) -> RcbResult<Self> {
        let did_dht = DidDht::from_identity_key(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_dht))
    }

    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let did_dht = DidDht::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_dht))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn publish(&self, signer: Arc<dyn RcbSigner>) -> RcbResult<()> {
        self.0
            .publish(signer.to_signer())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn deactivate(&self, signer: Arc<dyn RcbSigner>) -> RcbResult<()> {
        self.0
            .deactivate(signer.to_signer())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> DidDht {
        self.0.clone()
    }
}
