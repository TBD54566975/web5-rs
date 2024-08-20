use crate::errors::Result;
use web5::{
    dids::portable_did::PortableDid as InnerPortableDid,
    json::{FromJson, ToJson},
};

pub struct PortableDid(pub InnerPortableDid);

impl PortableDid {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_portable_did = InnerPortableDid::from_json_string(json)?;
        Ok(Self(inner_portable_did))
    }

    pub fn get_data(&self) -> InnerPortableDid {
        self.0.clone()
    }

    pub fn to_json_string(&self) -> Result<String> {
        let json_string = self.0.to_json_string()?;
        Ok(json_string)
    }
}
