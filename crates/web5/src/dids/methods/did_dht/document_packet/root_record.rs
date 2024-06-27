use std::collections::HashMap;

use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{rdata_encoder::record_rdata_to_hash_map, DocumentPacketError, DEFAULT_TTL};

const NAME_PREFIX: &str = "_did";

#[derive(Debug, PartialEq)]
pub struct RootRecord {
    pub did_id: String,
    pub vm: Vec<u32>,
    pub svc: Vec<u32>,
    pub inv: Vec<u32>,
    pub del: Vec<u32>,
    pub auth: Vec<u32>,
    pub agm: Vec<u32>,
    pub asm: Vec<u32>,
}

/// Parse RData value as Vec<u32> or return empty vec![] if key is absent
fn parse_rdata_list(
    rdata_map: &HashMap<String, String>,
    key: &str,
    prefix: &str,
) -> Result<Vec<u32>, DocumentPacketError> {
    // Step 1: Get the data string associated with the key
    let data = match rdata_map.get(key) {
        Some(data) => data,
        None => return Ok(vec![]),
    };

    // Step 2: Split the data by commas and trim each part
    let parts: Vec<&str> = data.split(',').map(|s| s.trim()).collect();

    // Step 3: Strip the prefix from each part
    let stripped_parts: Result<Vec<&str>, DocumentPacketError> = parts
        .into_iter()
        .map(|s| {
            s.strip_prefix(prefix).ok_or_else(|| {
                DocumentPacketError::RDataError(format!(
                    "Missing prefix {} for value of {}",
                    prefix, key
                ))
            })
        })
        .collect();

    let stripped_parts = stripped_parts?;

    // Step 4: Parse the stripped parts into u32
    stripped_parts
        .into_iter()
        .map(|s| {
            s.parse::<u32>().map_err(|_| {
                DocumentPacketError::RDataError(
                    format!("Could not parse root record entry {} into u32's", key).to_string(),
                )
            })
        })
        .collect()
}

impl RootRecord {
    pub fn is_root_record(record: &ResourceRecord) -> bool {
        match record.name.get_labels().first() {
            None => false,
            Some(subdomain) => subdomain.to_string() == NAME_PREFIX,
        }
    }

    pub fn new(did_id: &str) -> Self {
        RootRecord {
            did_id: did_id.to_string(),
            vm: Vec::new(),
            svc: Vec::new(),
            inv: Vec::new(),
            del: Vec::new(),
            auth: Vec::new(),
            agm: Vec::new(),
            asm: Vec::new(),
        }
    }

    pub fn to_resource_record(&self) -> Result<ResourceRecord, DocumentPacketError> {
        let fields = [
            ("vm", "k", &self.vm),
            ("asm", "k", &self.asm),
            ("inv", "k", &self.inv),
            ("del", "k", &self.del),
            ("auth", "k", &self.auth),
            ("agm", "k", &self.agm),
            ("svc", "s", &self.svc),
        ];

        let mut parts: Vec<String> = vec![];
        // Add each non-empty field to `parts`
        fields.iter().for_each(|(key, prefix, vals)| {
            if !vals.is_empty() {
                let prefixed_vals: Vec<String> = vals
                    .iter()
                    .map(|idx| format!("{}{}", prefix, idx))
                    .collect();
                let entry = format!("{}={}", key, prefixed_vals.join(","));
                parts.push(entry);
            }
        });
        let parts = parts.join(";");

        let name = Name::new_unchecked(&format!("{}.{}", NAME_PREFIX, self.did_id)).into_owned();
        let txt_record = TXT::new().with_string(&parts)?.into_owned();

        Ok(ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }

    pub fn from_resource_record(record: &ResourceRecord) -> Result<Self, DocumentPacketError> {
        // todo maybe: pass did_uri as param and verify it against self.id

        let labels = record.name.get_labels();
        if labels.len() != 2 {
            return Err(DocumentPacketError::RootRecord(
                "Root record name must have form \"_did.<did_id>\"".to_string(),
            ));
        }
        if labels[0].to_string() != "_did" {
            return Err(DocumentPacketError::RootRecord(
                "Root record did not have top level domain \"_did\"".to_string(),
            ));
        }
        let did_id = labels[1].to_string();

        let rdata_map: HashMap<String, String> = record_rdata_to_hash_map(record)?;

        Ok(RootRecord {
            did_id,
            vm: parse_rdata_list(&rdata_map, "vm", "k")?,
            svc: parse_rdata_list(&rdata_map, "svc", "s")?,
            inv: parse_rdata_list(&rdata_map, "inv", "k")?,
            del: parse_rdata_list(&rdata_map, "del", "k")?,
            auth: parse_rdata_list(&rdata_map, "auth", "k")?,
            agm: parse_rdata_list(&rdata_map, "agm", "k")?,
            asm: parse_rdata_list(&rdata_map, "asm", "k")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_from_resource_record_no_txt() {
        let did_id = "123";
        let root_record = RootRecord::new(&did_id);
        let resource_record = root_record
            .to_resource_record()
            .expect("Expected to create resource record");
        let root_record2 = RootRecord::from_resource_record(&resource_record)
            .expect("Expected to reconstitute root record");
        assert_eq!(root_record, root_record2);
    }

    #[test]
    fn to_and_from_resource_record_many_values() {
        let did_id = "123";
        let mut root_record = RootRecord::new(&did_id);
        root_record.vm.push(0);
        root_record.agm.push(0);
        root_record.asm.push(1);
        root_record.asm.push(0);
        root_record.auth.push(0);
        root_record.del.push(0);
        root_record.del.push(2);
        root_record.inv.push(0);
        root_record.svc.push(0);
        root_record.svc.push(10);

        let resource_record = root_record
            .to_resource_record()
            .expect("Expected to create resource record");
        let root_record2 = RootRecord::from_resource_record(&resource_record)
            .expect("Expected to reconstitute root record");
        assert_eq!(root_record, root_record2);
    }
}
