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

#[cfg(test)]
mod test {
    use crate::dids::methods::did_dht::document_packet::controller::Controller;

    use super::*;

    #[test]
    fn test_to_and_from_resource_record() {
        let also_known_as = vec!["Dave".to_string(), "Bartholemoop".to_string()];
        let record = AlsoKnownAs::to_resource_record(&also_known_as)
            .expect("expected to convert to DNS record");
        let also_known_as2 = AlsoKnownAs::from_resource_record(&record)
            .expect("Expected to convert from DNS record");
        assert_eq!(also_known_as, also_known_as2);
    }

    #[test]
    fn test_is_aka_record() {
        let also_known_as = vec!["Dave".to_string(), "Bartholemoop".to_string()];
        let record = AlsoKnownAs::to_resource_record(&also_known_as)
            .expect("expected to convert to DNS record");

        assert!(AlsoKnownAs::is_aka_record(&record));

        let controllers = vec!["Jerb".to_string(), "Carrie Bradshaw".to_string()];
        let record = Controller::to_resource_record(&controllers).unwrap();
        assert!(!AlsoKnownAs::is_aka_record(&record));
    }
}
