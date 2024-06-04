use std::collections::HashMap;

use crate::dids::document::Service;

use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use url::Url;

use super::{
    rdata_encoder::{get_rdata_txt_value, record_rdata_to_hash_map},
    DocumentPacketError, DEFAULT_TTL,
};

const NAME_PREFIX: &str = "_s";
const NAME_SUFFIX: &str = "_did";

#[derive(Debug, PartialEq)]
struct ServiceRdata {
    pub id: String,
    pub se: Vec<String>,
    pub t: String,
}

impl TryFrom<HashMap<String, String>> for ServiceRdata {
    fn try_from(rdata_map: HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(ServiceRdata {
            id: get_rdata_txt_value(&rdata_map, "id")?,
            se: get_rdata_txt_value(&rdata_map, "se")?
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            t: get_rdata_txt_value(&rdata_map, "t")?,
        })
    }

    type Error = DocumentPacketError;
}

impl Service {
    pub fn is_service_record_with_index(record: &ResourceRecord, idx: u32) -> bool {
        let labels = record.name.get_labels();

        match labels.first() {
            None => return false,
            Some(subdomain) => {
                if subdomain.to_string() != format!("{}{}", NAME_PREFIX, idx) {
                    return false;
                }
            }
        };

        match labels.get(1) {
            None => false,
            Some(subdomain) => subdomain.to_string() == NAME_SUFFIX,
        }
    }

    pub fn record_name(idx: u32) -> String {
        format!("{}{}.{}", NAME_PREFIX, idx, NAME_SUFFIX)
    }

    pub fn to_resource_record(&self, idx: u32) -> Result<ResourceRecord, DocumentPacketError> {
        let url = Url::parse(&self.id)
            .map_err(|_| DocumentPacketError::MissingFragment(self.id.clone()))?;
        let service_id_fragment = url
            .fragment()
            .ok_or(DocumentPacketError::MissingFragment(self.id.clone()))?;

        let se = self.service_endpoint.join(",");

        let parts = format!("id={};t={};se={}", service_id_fragment, self.r#type, se);
        let name =
            Name::new_unchecked(&format!("{}{}.{}", NAME_PREFIX, idx, NAME_SUFFIX)).into_owned();
        let txt_record = TXT::new().with_string(&parts)?.into_owned();

        Ok(ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }

    pub fn from_resource_record(
        did_uri: &str,
        record: &ResourceRecord,
    ) -> Result<Self, DocumentPacketError> {
        let rdata_map = record_rdata_to_hash_map(record)?;
        let service_rdata: ServiceRdata = rdata_map.try_into()?;

        Ok(Service {
            id: format!("{}#{}", did_uri, service_rdata.id),
            r#type: service_rdata.t,
            service_endpoint: service_rdata.se,
        })
    }
}

#[cfg(test)]
mod tests {
    use simple_dns::rdata::A;

    use super::*;

    #[test]
    fn test_to_and_from_resource_record() {
        let did_uri = "did:dht:123";
        let id = "did:dht:123#0";

        let r#type = "some_type";
        let service_endpoint = "foo.tbd.website";
        let service = Service {
            id: id.to_string(),
            r#type: r#type.to_string(),
            service_endpoint: vec![service_endpoint.to_string()],
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Failed to convert Service to ResourceRecord");

        let service2 = Service::from_resource_record(did_uri, &resource_record)
            .expect("Failed to convert ResourceRecord to Service");

        assert_eq!(service, service2);
    }

    #[test]
    fn test_to_and_from_resource_record_many_service_endpoints() {
        let did_uri = "did:dht:123";
        let id = "did:dht:123#0";

        let r#type = "some_type";
        let service_endpoint = "foo.tbd.website";
        let service = Service {
            id: id.to_string(),
            r#type: r#type.to_string(),
            service_endpoint: vec![service_endpoint.to_string(), service_endpoint.to_string()],
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Failed to convert Service to ResourceRecord");

        let service2 = Service::from_resource_record(did_uri, &resource_record)
            .expect("Failed to convert ResourceRecord to Service");

        assert_eq!(service, service2);
    }

    #[test]
    fn test_to_record_resource_service_id_multiple_sharps() {
        // The URL spec seems to say that multiple "#" is invalid,
        // but after trying some actual URL implementations,
        // this behavior is more common.
        let did_uri = "did:dht:123";
        let id = "did:dht:123#hey#ya"; // multiple "#"
        let r#type = "some_type";
        let service_endpoint = "foo.tbd.website";
        let service = Service {
            id: id.to_string(),
            r#type: r#type.to_string(),
            service_endpoint: vec![service_endpoint.to_string()],
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Expected to create resource record from service");
        let service2 = Service::from_resource_record(did_uri, &resource_record)
            .expect("Expected to create service from resource record");
        assert_eq!(service, service2);
    }

    #[test]
    fn test_to_record_resource_missing_fragment() {
        let did_uri = "did:dht:123"; // missing "#0"
        let r#type = "some_type";
        let service_endpoint = "foo.tbd.website";
        let service = Service {
            id: did_uri.to_string(),
            r#type: r#type.to_string(),
            service_endpoint: vec![service_endpoint.to_string()],
        };

        let resource_record = service
            .to_resource_record(0)
            .expect_err("Expected error due to service.id missing fragment");
        match resource_record {
            DocumentPacketError::MissingFragment(id) => {
                assert_eq!(id, did_uri)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_from_record_resource_rdata_not_txt() {
        let name = Name::new_unchecked(&format!("_s{}._did", 0)).into_owned();

        let resource_record = ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::A(A { address: 0 }), // not RData::TXT
        );

        let error = Service::from_resource_record("did:ex:abc", &resource_record)
            .expect_err("Expected error because RData is not TXT");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!(),
        };
    }

    #[test]
    fn test_from_record_resource_rdata_txt_malformed() {
        let name = Name::new_unchecked(&format!("_s{}._did", 0)).into_owned();

        let txt = TXT::new().with_string("a=b=c;;;").unwrap();
        let resource_record = ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt), // Not ';' separated entries
        );

        let error = Service::from_resource_record("did:ex:abc", &resource_record)
            .expect_err("Expected error because RData TXT is malformed");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!(),
        };
    }

    #[test]
    fn test_from_record_resource_missing_se() {
        let name = Name::new_unchecked(&format!("_s{}._did", 0)).into_owned();

        let txt = TXT::new().with_string("id=foo;t=bar").unwrap();
        let resource_record = ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt), // Not ';' separated entries
        );

        let error = Service::from_resource_record("did:ex:abc", &resource_record)
            .expect_err("Expected error because missing se");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!(),
        };
    }
}
