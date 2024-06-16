use std::sync::Arc;
use web5::apid::{dids::methods::did_dht::DidDht as InnerDidDht, jwk::Jwk};

use crate::{dsa::RcbSigner, errors::RcbResult};

pub struct RcbDidDht(InnerDidDht);

impl RcbDidDht {
    pub fn from_identity_key(public_key: Jwk) -> RcbResult<Self> {
        let inner = InnerDidDht::from_identity_key(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(inner))
    }

    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let inner = InnerDidDht::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(inner))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn publish(&self, signer: Arc<dyn RcbSigner>) -> RcbResult<()> {
        self.0
            .publish(signer.to_inner())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn deactivate(&self, signer: Arc<dyn RcbSigner>) -> RcbResult<()> {
        self.0
            .deactivate(signer.to_inner())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> InnerDidDht {
        self.0.clone()
    }
}
