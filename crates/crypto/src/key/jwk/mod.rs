pub mod private_jwk;
pub mod public_jwk;

use self::private_jwk::PrivateJwk;
use super::{KeyError, KeyType};
use base64::{engine::general_purpose, Engine as _};
use josekit::{
    jwk::{
        alg::{ec::EcCurve, ed::EdCurve},
        Jwk,
    },
    jws::alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
};
use sha2::{Digest, Sha256};

// todo considering proposing adding a thumbprint to the josekit repo
fn compute_thumbprint(jwk: &Jwk) -> Result<String, KeyError> {
    let thumbprint_json_string = match jwk.key_type() {
        "EC" => format!(
            r#"{{"crv":"{}","kty":"EC","x":"{}","y":"{}"}}"#,
            jwk.curve().ok_or(KeyError::ThumprintFailed)?,
            jwk.parameter("x").ok_or(KeyError::ThumprintFailed)?,
            jwk.parameter("y").ok_or(KeyError::ThumprintFailed)?,
        ),
        "OKP" => format!(
            r#"{{"crv":"{}","kty":"OKP","x":"{}"}}"#,
            jwk.curve().ok_or(KeyError::ThumprintFailed)?,
            jwk.parameter("x").ok_or(KeyError::ThumprintFailed)?,
        ),
        _ => return Err(KeyError::ThumprintFailed),
    };
    let mut hasher = Sha256::new();
    hasher.update(thumbprint_json_string);
    let digest = hasher.finalize();
    let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(&digest);

    Ok(thumbprint)
}

pub fn generate_private_jwk(key_type: KeyType) -> Result<Box<PrivateJwk>, KeyError> {
    let mut jwk = match key_type {
        KeyType::Secp256k1 => Jwk::generate_ec_key(EcCurve::Secp256k1),
        KeyType::Ed25519 => Jwk::generate_ed_key(EdCurve::Ed25519),
    }?;

    let key_alias = compute_thumbprint(&jwk)?;
    jwk.set_key_id(&key_alias);
    jwk.set_algorithm(match key_type {
        KeyType::Secp256k1 => EcdsaJwsAlgorithm::Es256k.to_string(),
        KeyType::Ed25519 => EddsaJwsAlgorithm::Eddsa.to_string(),
    });

    Ok(Box::new(PrivateJwk(jwk)))
}
