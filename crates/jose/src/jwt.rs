// todo sign & verify & decode

use crate::{jws::Header, jws_signer::JwsSigner};
use josekit::{
    jws::JwsHeader as JosekitJwsHeader,
    jwt::{encode_with_signer, JwtPayload as JosekitJwtPayload},
};
use std::time::SystemTime;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Claims {
    pub issuer: Option<String>,
    pub jwt_id: Option<String>,
    pub subject: Option<String>,
    pub not_before: Option<SystemTime>,
    pub expires_at: Option<SystemTime>,
    pub vc: Option<serde_json::Value>,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwtError {
    #[error("error signing {0}")]
    SigningError(String),
    #[error("josekit JoseError {0}")]
    JoseError(String),
}

pub fn sign_jwt(signer: JwsSigner, claims: Claims, header: Header) -> Result<String, JwtError> {
    let mut josekit_claims = JosekitJwtPayload::new();

    if let Some(issuer) = claims.issuer {
        josekit_claims.set_issuer(issuer);
    }
    if let Some(jwt_id) = claims.jwt_id {
        josekit_claims.set_jwt_id(jwt_id);
    }
    if let Some(subject) = claims.subject {
        josekit_claims.set_subject(subject);
    }
    if let Some(not_before) = claims.not_before {
        josekit_claims.set_not_before(&not_before);
    }
    if let Some(expires_at) = claims.expires_at {
        josekit_claims.set_expires_at(&expires_at);
    }
    if claims.vc.is_some() {
        josekit_claims
            .set_claim("vc", claims.vc)
            .map_err(|e| JwtError::JoseError(e.to_string()))?;
    }

    let mut josekit_header = JosekitJwsHeader::new();
    if let Some(typ) = header.r#type {
        josekit_header.set_token_type(typ);
    }

    let jwt = encode_with_signer(&josekit_claims, &josekit_header, &signer)
        .map_err(|e| JwtError::JoseError(e.to_string()))?;

    Ok(jwt)
}
