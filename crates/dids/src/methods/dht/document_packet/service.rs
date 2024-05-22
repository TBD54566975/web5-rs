use std::collections::HashMap;

use crate::document::Service;

use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};
use ssi_core::one_or_many::OneOrMany;
use url::Url;

use super::{
    rdata_encoder::{get_rdata_txt_value, record_rdata_to_hash_map, to_one_or_many},
    DocumentPacketError, DEFAULT_TTL,
};

#[derive(Debug, PartialEq)]
struct ServiceRdata {
    pub id: String,
    pub se: OneOrMany<String>,
    pub t: OneOrMany<String>,
}

impl TryFrom<HashMap<String, String>> for ServiceRdata {
    fn try_from(rdata_map: HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(ServiceRdata {
            id: get_rdata_txt_value(&rdata_map, "id")?,
            se: to_one_or_many(get_rdata_txt_value(&rdata_map, "se")?),
            t: to_one_or_many(get_rdata_txt_value(&rdata_map, "t")?),
        })
    }

    type Error = DocumentPacketError;
}

impl Service {
    pub fn to_resource_record(
        &self,
        idx: u32,
    ) -> Result<ResourceRecord<'static>, DocumentPacketError> {
        let url = Url::parse(&self.id)
            .map_err(|_| DocumentPacketError::MissingFragment(self.id.clone()))?;
        let service_id_fragment = url
            .fragment()
            .ok_or(DocumentPacketError::MissingFragment(self.id.clone()))?;

        let t = match &self.r#type {
            OneOrMany::One(r#type) => r#type.clone(),
            OneOrMany::Many(r#types) => r#types.join(","),
        };
        let se = match &self.service_endpoint {
            OneOrMany::One(service_endpoint) => service_endpoint.clone(),
            OneOrMany::Many(service_endpoints) => service_endpoints.join(","),
        };

        let parts = format!("id={};t={};se={}", service_id_fragment, t, se);
        let name = Name::new_unchecked(&format!("_s{}._did", idx)).into_owned();
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
        record: ResourceRecord,
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
            r#type: OneOrMany::One(r#type.to_string()),
            service_endpoint: OneOrMany::One(service_endpoint.to_string()),
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Failed to convert Service to ResourceRecord");

        let service2 = Service::from_resource_record(did_uri, resource_record)
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
            r#type: OneOrMany::Many(vec![r#type.to_string(), r#type.to_string()]),
            service_endpoint: OneOrMany::Many(vec![
                service_endpoint.to_string(),
                service_endpoint.to_string(),
            ]),
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Failed to convert Service to ResourceRecord");

        let service2 = Service::from_resource_record(did_uri, resource_record)
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
            r#type: OneOrMany::One(r#type.to_string()),
            service_endpoint: OneOrMany::One(service_endpoint.to_string()),
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Expected to create resource record from service");
        let service2 = Service::from_resource_record(did_uri, resource_record).expect("Expected to create service from resource record");
        assert_eq!(service, service2);
    }

    #[test]
    fn test_to_record_resource_missing_fragment() {
        let did_uri = "did:dht:123"; // missing "#0"
        let r#type = "some_type";
        let service_endpoint = "foo.tbd.website";
        let service = Service {
            id: did_uri.to_string(),
            r#type: OneOrMany::One(r#type.to_string()),
            service_endpoint: OneOrMany::One(service_endpoint.to_string()),
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

        let error = Service::from_resource_record("did:ex:abc", resource_record).expect_err("");
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

        let error = Service::from_resource_record("did:ex:abc", resource_record).expect_err("");
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

        let error = Service::from_resource_record("did:ex:abc", resource_record).expect_err("");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!(),
        };
    }
}
