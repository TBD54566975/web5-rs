use crate::{dids::resolution_result::ResolutionResult, dsa::Signer, errors::Result};
use std::sync::Arc;
use web5::apid::{dids::methods::did_dht::DidDht as InnerDidDht, jwk::Jwk};

pub struct DidDht(pub InnerDidDht);

pub fn did_dht_resolve(uri: &str) -> Result<Arc<ResolutionResult>> {
    let resolution_result = InnerDidDht::resolve(uri).map_err(|e| Arc::new(e.into()))?;
    Ok(Arc::new(ResolutionResult(resolution_result)))
}

impl DidDht {
    pub fn from_identity_key(public_key: Jwk) -> Result<Self> {
        let did_dht = InnerDidDht::from_identity_key(public_key).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_dht))
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let did_dht = InnerDidDht::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_dht))
    }

    pub fn publish(&self, signer: Arc<dyn Signer>) -> Result<()> {
        self.0
            .publish(signer.to_inner())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn deactivate(&self, signer: Arc<dyn Signer>) -> Result<()> {
        self.0
            .deactivate(signer.to_inner())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> InnerDidDht {
        self.0.clone()
    }
}
