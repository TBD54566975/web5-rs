use web5::dids::portable_did::PortableDid as InnerPortableDid;

use crate::errors::Result;

pub struct PortableDid(pub InnerPortableDid);

impl PortableDid {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_portable_did = InnerPortableDid::from_json_string(json)?;
        Ok(Self(inner_portable_did))
    }

    pub fn get_data(&self) -> InnerPortableDid {
        self.0.clone()
    }
}
