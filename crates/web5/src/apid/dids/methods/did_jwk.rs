use crate::apid::{
    dids::{
        did::Did,
        document::{Document, VerificationMethod},
        resolution_result::ResolutionResult,
    },
    jwk::Jwk,
};
use base64::{engine::general_purpose, Engine as _};

use super::Result;

#[derive(Clone)]
pub struct DidJwk {
    pub did: Did,
    pub document: Document,
}

impl DidJwk {
    pub fn from_public_jwk(public_jwk: Jwk) -> Result<Self> {
        let jwk_string = serde_json::to_string(&public_jwk)?;
        let method_specific_id = general_purpose::URL_SAFE_NO_PAD.encode(jwk_string);

        let uri = format!("did:jwk:{}", method_specific_id);

        let did = Did::new(&uri)?;

        let verification_method_id = format!("{}#0", uri);

        let verification_method = VerificationMethod {
            id: verification_method_id.clone(),
            r#type: "JsonWebKey".to_string(),
            controller: uri.clone(),
            public_key_jwk: public_jwk.clone(),
        };

        let document = Document {
            id: uri.clone(),
            verification_method: vec![verification_method.clone()],
            authentication: Some(vec![verification_method_id.clone()]),
            assertion_method: Some(vec![verification_method_id.clone()]),
            capability_invocation: Some(vec![verification_method_id.clone()]),
            capability_delegation: Some(vec![verification_method_id.clone()]),
            ..Default::default()
        };

        Ok(Self { did, document })
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidJwk::resolve(uri)?;

        match resolution_result.document {
            None => panic!(),
            Some(document) => {
                let did = Did::new(uri)?;
                Ok(Self { did, document })
            }
        }
    }

    pub fn resolve(uri: &str) -> Result<ResolutionResult> {
        let identifier = Did::new(uri)?;
        let decoded_jwk = general_purpose::URL_SAFE_NO_PAD.decode(identifier.id)?;
        let public_jwk = serde_json::from_slice(&decoded_jwk)?;

        let did_jwk = DidJwk::from_public_jwk(public_jwk)?;

        Ok(ResolutionResult {
            document: Some(did_jwk.document),
            ..Default::default()
        })
    }
}
