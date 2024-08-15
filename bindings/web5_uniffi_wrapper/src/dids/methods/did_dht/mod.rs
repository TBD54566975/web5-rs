use crate::{
    crypto::dsa::{Signer, ToInnerSigner},
    dids::resolution::resolution_result::ResolutionResult,
    errors::ResultOld,
};
use std::sync::Arc;
use web5::{crypto::jwk::Jwk, dids::methods::did_dht::DidDht as InnerDidDht};

pub struct DidDht(pub InnerDidDht);

pub fn did_dht_resolve(uri: &str) -> ResultOld<Arc<ResolutionResult>> {
    let resolution_result = InnerDidDht::resolve(uri);
    Ok(Arc::new(ResolutionResult(resolution_result)))
}

impl DidDht {
    pub fn from_identity_key(public_key: Jwk) -> ResultOld<Self> {
        let did_dht = InnerDidDht::from_identity_key(public_key)?;
        Ok(Self(did_dht))
    }

    pub fn from_uri(uri: &str) -> ResultOld<Self> {
        let did_dht = InnerDidDht::from_uri(uri)?;
        Ok(Self(did_dht))
    }

    pub fn publish(&self, signer: Arc<dyn Signer>) -> ResultOld<()> {
        let inner_signer = Arc::new(ToInnerSigner(signer));
        Ok(self.0.publish(inner_signer)?)
    }

    pub fn deactivate(&self, signer: Arc<dyn Signer>) -> ResultOld<()> {
        let inner_signer = Arc::new(ToInnerSigner(signer));
        Ok(self.0.deactivate(inner_signer)?)
    }

    pub fn get_data(&self) -> InnerDidDht {
        self.0.clone()
    }
}
