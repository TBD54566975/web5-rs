use crate::{
    dids::bearer_did::BearerDid,
    errors::{Result, Web5Error},
    json::JsonObject,
};

pub struct Jwt {
    pub kid: String,
    pub parts: Vec<String>,
    pub header: JsonObject,
    pub claims: JsonObject,
    pub signature: Vec<u8>,
    pub compact_jws: String,
}

impl Jwt {
    pub fn from_claims(
        claims: &JsonObject,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<Self> {
        let verification_method_id = verification_method_id
            .unwrap_or_else(|| bearer_did.document.verification_method[0].id.clone());

        let is_assertion_method =
            if let Some(assertion_methods) = &bearer_did.document.assertion_method {
                assertion_methods.contains(&verification_method_id)
            } else {
                false
            };

        if !is_assertion_method {
            return Err(Web5Error::Parameter(format!(
                "verification_method_id {} is not an assertion_method",
                verification_method_id
            )));
        }

        unimplemented!()
    }

    pub fn from_compact_jws(compact_jws: &str, verify: bool) -> Result<Self> {
        unimplemented!()
    }
}
