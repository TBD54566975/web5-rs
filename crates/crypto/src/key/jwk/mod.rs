pub mod private_jwk;
pub mod public_jwk;

use super::KeyError;
use base64::{engine::general_purpose, Engine as _};
use josekit::jwk::Jwk;
use sha2::{Digest, Sha256};

fn compute_thumbprint(jwk: &Jwk) -> Result<String, KeyError> {
    // todo considering proposing adding a thumbprint to the josekit repo

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
