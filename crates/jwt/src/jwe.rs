use crate::{Claims, JwtError};
use jws::JwsHeader;
use keys::key_manager::Signer;
use std::sync::Arc;

// A JWT can be implemented as either a JWS or JWE, this module is the implementation of a JWT as a JWE

// TODO implement https://github.com/TBD54566975/web5-rs/issues/174

pub struct JwtDecoded<T: Claims> {
    // TODO other properties for JWE
    pub claims: T,
    pub parts: Vec<String>,
}

pub struct Jwt;

impl Jwt {
    pub fn sign<T: Claims>(
        _signer: Arc<dyn Signer>,
        _header: Option<JwsHeader>,
        _claims: &T,
    ) -> Result<String, JwtError> {
        unimplemented!()
    }

    pub fn decode<T: Claims>(_jwt: &str) -> Result<JwtDecoded<T>, JwtError> {
        unimplemented!()
    }

    pub async fn verify<T: Claims>(_jwt: &str) -> Result<JwtDecoded<T>, JwtError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO tests
}
