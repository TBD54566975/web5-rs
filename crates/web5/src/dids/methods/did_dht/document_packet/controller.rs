use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{rdata_encoder::text_from_record, DocumentPacketError, DEFAULT_TTL};

pub struct Controller {}

impl Controller {
    pub fn is_cnt_record(record: &ResourceRecord) -> bool {
        match record.name.get_labels().first() {
            None => false,
            Some(domain) => domain.to_string() == "_cnt",
        }
    }

    pub fn to_resource_record(
        controllers: &[String],
    ) -> Result<ResourceRecord, DocumentPacketError> {
        let name = Name::new_unchecked("_cnt._did.");
        let txt_record = TXT::new().with_string(&controllers.join(","))?.into_owned();

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
        let controllers = text
            .split(',')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(controllers)
    }
}

#[cfg(test)]
mod tests {
    use crate::dids::methods::did_dht::document_packet::also_known_as::AlsoKnownAs;

    use super::*;

    #[test]
    fn to_and_from_resource_record() {
        let controllers = vec![];
        let record = Controller::to_resource_record(&controllers)
            .expect("Expected to create controller resource record");
        let controllers2 = Controller::from_resource_record(&record)
            .expect("Expected to get a list of strings from resource record");
        assert_eq!(controllers, controllers2);

        let controllers = vec!["did:dht:123".to_string(), "did:dht:456".to_string()];
        let record = Controller::to_resource_record(&controllers)
            .expect("Expected to create controller resource record");
        let controllers2 = Controller::from_resource_record(&record)
            .expect("Expected to get a list of strings from resource record");
        assert_eq!(controllers, controllers2);
    }

    #[test]
    fn test_is_cnt_record() {
        let also_known_as = vec!["Dave".to_string(), "Bartholemoop".to_string()];
        let record = Controller::to_resource_record(&also_known_as)
            .expect("expected to convert to DNS record");

        assert!(Controller::is_cnt_record(&record));

        let controllers = vec!["Jerb".to_string(), "Carrie Bradshaw".to_string()];
        let record = AlsoKnownAs::to_resource_record(&controllers).unwrap();
        assert!(!Controller::is_cnt_record(&record));
    }
}
