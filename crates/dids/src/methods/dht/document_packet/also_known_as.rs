use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{rdata_encoder::text_from_record, DocumentPacketError, DEFAULT_TTL};

pub struct AlsoKnownAs {}

impl AlsoKnownAs {
    pub fn is_aka_record(record: &ResourceRecord) -> bool {
        match record.name.get_labels().first() {
            None => false,
            Some(domain) => domain.to_string() == "_aka",
        }
    }

    pub fn to_resource_record(
        also_known_as: &[String],
    ) -> Result<ResourceRecord, DocumentPacketError> {
        let name = Name::new_unchecked("_aka._did.");
        let txt_record = TXT::new()
            .with_string(&also_known_as.join(","))?
            .into_owned();

        Ok(ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }

    pub fn from_resource_record(
        record: &ResourceRecord,
    ) -> Result<Vec<String>, DocumentPacketError> {
        let text = text_from_record(record)?;
        let also_known_as = text
            .split(',')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(also_known_as)
    }
}
