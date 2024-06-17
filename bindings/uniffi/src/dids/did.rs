use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::dids::did::Did;

pub struct RcbDid(pub Did);

impl RcbDid {
    pub fn new(uri: &str) -> RcbResult<Self> {
        let did = Did::new(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did))
    }

    pub fn get_data(&self) -> Did {
        self.0.clone()
    }
}
