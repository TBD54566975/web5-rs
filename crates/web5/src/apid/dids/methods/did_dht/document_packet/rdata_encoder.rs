use std::collections::HashMap;

use simple_dns::{rdata::RData, ResourceRecord};

use super::DocumentPacketError;

pub fn text_from_record(record: &ResourceRecord) -> Result<String, DocumentPacketError> {
    let rdata_txt = match &record.rdata {
        RData::TXT(txt) => txt.clone(),
        _ => {
            return Err(DocumentPacketError::RDataError(
                "RData must have type TXT".to_owned(),
            ))
        }
    };
    String::try_from(rdata_txt)
        .map_err(|_| DocumentPacketError::RDataError("Failed to convert to string".to_owned()))
}

/// Gets the RData from the record. If RData is RData::TXT, get the text as a string.
/// Convert strings like "id=foo;t=bar;se=baz" into a hash map like { 'id': 'foo', 't': 'bar', 'se': 'baz' }
/// If there is any issue, return DocumentPacketError::RDataError
pub fn record_rdata_to_hash_map(
    record: &ResourceRecord,
) -> Result<HashMap<String, String>, DocumentPacketError> {
    let text = text_from_record(record)?;
    if text.is_empty() {
        return Ok(HashMap::new());
    }

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
pub fn get_rdata_txt_value(
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
