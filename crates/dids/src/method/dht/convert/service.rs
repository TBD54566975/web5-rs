use crate::document::Service;

use pkarr::dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{ConvertError, DEFAULT_TTL};

impl Service {
    pub fn to_resource_record(self, idx: u32) -> Result<ResourceRecord<'static>, ConvertError> {
        let service_id_fragment =
            self.id
                .split("#")
                .last()
                .ok_or(ConvertError::ServiceConvertError(
                    "Service id missing fragment".to_string(),
                ))?;

        let parts = format!(
            "id={};t={};se={}",
            service_id_fragment, self.r#type, self.service_endpoint
        );

        let mut txt_record = TXT::new();
        txt_record.add_string(&parts)?;

        Ok(ResourceRecord::new(
            Name::new_unchecked(&format!("_s{}._did", idx)),
            pkarr::dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }
}
