use std::str::FromStr;

use crypto::Curve;
use pkarr::dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use crate::document::VerificationMethod;

use super::{ConvertError, DEFAULT_TTL};

impl VerificationMethod {
    pub fn to_resource_record(&self, idx: u32) -> Result<ResourceRecord<'static>, ConvertError> {
        let vm_id_fragment = self
            .id
            .split('#')
            .last()
            .ok_or(ConvertError::VerificationMethod(
                "verification method id missing fragment".to_string(),
            ))?;

        let crv: Curve =
            Curve::from_str(&self.public_key_jwk.crv).map_err(|_| -> ConvertError {
                ConvertError::VerificationMethod(format!(
                    "Unsupported crv: {}",
                    self.public_key_jwk.crv
                ))
            })?;
        let key_type_idx = match crv {
            Curve::Ed25519 => '0',
            Curve::Secp256k1 => '1',
        };

        let jwk_string = serde_json::to_string(&self.public_key_jwk).map_err(|_| {
            ConvertError::VerificationMethod(
                "failed to serialize verification method public jwk".to_string(),
            )
        })?;

        let name = Name::new_unchecked(&format!("_k{}._did", idx)).into_owned();

        let parts = format!("id={};t={};k={}", vm_id_fragment, key_type_idx, jwk_string);
        let txt_record = TXT::new().with_string(&parts)?.into_owned();

        Ok(ResourceRecord::new(
            name,
            pkarr::dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }
}
