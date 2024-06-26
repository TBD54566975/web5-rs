use web5::dids::portable_did::PortableDid as InnerPortableDid;

use crate::errors::Result;

pub struct PortableDid(pub InnerPortableDid);

impl PortableDid {
    pub fn new(json: &str) -> Result<Self> {
        let inner_portable_did = InnerPortableDid::new(json)?;
        Ok(Self(inner_portable_did))
    }

    pub fn get_data(&self) -> InnerPortableDid {
        self.0.clone()
    }
}
