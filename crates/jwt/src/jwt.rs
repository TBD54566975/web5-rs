use dids::bearer::{BearerDid, BearerDidError, KeySelector};
use josekit::{
    jws::JwsHeader,
    jwt::{encode_with_signer, JwtPayload},
    JoseError,
};

// todo pub enum JwtError, and KeyError::MissingKey is nonsensical
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwtError {
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error("josekit error {0}")]
    JoseError(String),
}

impl From<JoseError> for JwtError {
    fn from(error: JoseError) -> Self {
        JwtError::JoseError(error.to_string())
    }
}

pub fn sign_jwt(
    bearer_did: BearerDid,
    key_selector: &KeySelector,
    claims: &JwtPayload,
    header: &mut JwsHeader,
) -> Result<String, JwtError> {
    let verification_method = bearer_did.get_verification_method(&key_selector)?;
    header.set_key_id(verification_method.id);

    let signer = bearer_did.get_jws_signer(key_selector)?;

    let jwt = encode_with_signer(claims, header, signer.clone().as_ref())?;

    Ok(jwt)
}
