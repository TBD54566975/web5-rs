use std::str::FromStr;

use crypto::Curve;
use pkarr::dns::{rdata::{RData, TXT}, Name, ResourceRecord};

use crate::document::VerificationMethod;

use super::{ConvertError, DEFAULT_TTL};

fn key_type_index_to_crv(key_type_idx: char) -> Result<String, ConvertError> {
  let crv = match key_type_idx {
    '0' => Curve::Ed25519,
    '1' => Curve::Secp256k1,
    _ => return Err(ConvertError::VerificationMethodConvertError("Unrecognized curve key type index".to_string())),
  };

  Ok(crv.to_string())
}

impl VerificationMethod {
  pub fn to_resource_record(self, idx: u32) -> Result<ResourceRecord<'static>, ConvertError> {
    let vm_id_fragment = self.id.split("#").last().ok_or(
      ConvertError::VerificationMethodConvertError("verification method id missing fragment".to_string())
    )?;

    let crv: Curve = Curve::from_str(&self.public_key_jwk.crv).map_err(|_| -> ConvertError {
      ConvertError::VerificationMethodConvertError(format!("Unsupported crv: {}", self.public_key_jwk.crv))
    })?;
    let key_type_idx = match crv {
      Curve::Ed25519 => '0',
      Curve::Secp256k1 => '1',
    };

    let jwk_string = serde_json::to_string(&self.public_key_jwk).map_err(|_| {
      ConvertError::VerificationMethodConvertError("failed to serialize verification method public jwk".to_string())
    })?;
  
    let parts = format!("id={};t={};k={}",
      vm_id_fragment,
      key_type_idx,
      jwk_string
    );
  
    let mut txt_record = TXT::new();
    txt_record.add_string(&parts)?;
  
    Ok(ResourceRecord::new(
      Name::new_unchecked(&format!("_k{}._did", idx)),
      pkarr::dns::CLASS::IN,
      DEFAULT_TTL,
      RData::TXT(txt_record),
    ))
  }
}