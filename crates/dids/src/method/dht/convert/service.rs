use crate::document::Service;

use pkarr::dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{ConvertError, DEFAULT_TTL};

impl Service {
    pub fn to_resource_record(&self, idx: u32) -> Result<ResourceRecord<'static>, ConvertError> {
        let service_id_fragment = self.id.split('#').last().ok_or(ConvertError::Service(
            "Service id missing fragment".to_string(),
        ))?;

        let parts = format!(
            "id={};t={};se={}",
            service_id_fragment, self.r#type, self.service_endpoint
        );

        let name = Name::new_unchecked(&format!("_s{}._did", idx)).into_owned();

        let txt_record = TXT::new().with_string(&parts)?.into_owned();

        Ok(ResourceRecord::new(
            name,
            pkarr::dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }
}
