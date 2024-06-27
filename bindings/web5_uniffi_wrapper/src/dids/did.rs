use crate::errors::Result;
use std::sync::Arc;
use web5::dids::did::Did as InnerDid;

pub struct Did(pub InnerDid);

impl Did {
    pub fn new(uri: &str) -> Result<Self> {
        let did = InnerDid::new(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did))
    }

    pub fn get_data(&self) -> InnerDid {
        self.0.clone()
    }
}
