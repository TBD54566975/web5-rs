use std::collections::HashMap;

use crate::document::Service;

use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{DocumentPacketError, DEFAULT_TTL};

/// Gets the RData from the record. If RData is RData::TXT, get the text as a string.
/// Convert strings like "id=foo;t=bar;se=baz" into a map like { 'id': 'foo', 't': 'bar', 'se': 'baz' }
/// If there is any issue, return DocumentPacketError::RDataError
fn record_rdata_to_hash_map(
    record: ResourceRecord,
) -> Result<HashMap<String, String>, DocumentPacketError> {
    // Get RData text as String
    let rdata_txt = match record.rdata {
        RData::TXT(txt) => txt,
        _ => {
            return Err(DocumentPacketError::RDataError(
                "RData must have type TXT".to_owned(),
            ))
        }
    };
    let text = match String::try_from(rdata_txt) {
        Ok(text) => text,
        Err(_) => {
            return Err(DocumentPacketError::RDataError(
                "Failed to convert to string".to_owned(),
            ))
        }
    };

    // Parse key-value pairs:
    //   Split string by ";" to get entries
    //   Split each entry by "=" to get key and value
    let mut attributes = HashMap::new();
    for entry in text.split(';') {
        let k_v: Vec<&str> = entry.split('=').collect();
        if k_v.len() != 2 {
            return Err(DocumentPacketError::RDataError(
                "Could not get values from RData text".to_owned(),
            ));
        }

        let k = k_v[0].trim().to_string();
        let v = k_v[1].trim().to_string();

        attributes.insert(k, v);
    }

    Ok(attributes)
}

/// Get value from the RData HashMap created by record_rdata_to_hash_map().
/// Convert `None` into DocumentPacketError
fn get_rdata_txt_value(
    rdata_map: &HashMap<String, String>,
    key: &str,
) -> Result<String, DocumentPacketError> {
    let val = rdata_map
        .get(key)
        .ok_or(DocumentPacketError::RDataError(format!(
            "Could not extract {} from RData",
            key
        )))?;
    Ok(val.to_string())
}

impl Service {
    pub fn to_resource_record(
        &self,
        idx: u32,
    ) -> Result<ResourceRecord<'static>, DocumentPacketError> {
        let service_id_fragment = self
            .id
            .split('#')
            .last()
            .ok_or(DocumentPacketError::MissingFragment(self.id.to_string()))?;

        let parts = format!(
            "id={};t={};se={}",
            service_id_fragment, self.r#type, self.service_endpoint
        );
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

        let id = get_rdata_txt_value(&rdata_map, "id")?;
        let t = get_rdata_txt_value(&rdata_map, "t")?;
        let se = get_rdata_txt_value(&rdata_map, "se")?;

        Ok(Service {
            id: format!("{}#{}", did_uri, id),
            r#type: t,
            service_endpoint: se, // TODO: support service endpoints as array or map
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
            service_endpoint: service_endpoint.to_string(),
        };

        let resource_record = service
            .to_resource_record(0)
            .expect("Failed to convert Service to ResourceRecord");

        let service2 = Service::from_resource_record(did_uri, resource_record)
            .expect("Failed to convert ResourceRecord to Service");

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
            service_endpoint: service_endpoint.to_string(),
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
