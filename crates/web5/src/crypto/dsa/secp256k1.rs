#[cfg(test)]
use super::{DsaError, Result};
#[cfg(test)]
use crate::crypto::jwk::Jwk;
#[cfg(test)]
use base64::{engine::general_purpose, Engine as _};

#[cfg(test)]
pub struct Secp256k1Generator;

#[cfg(test)]
impl Secp256k1Generator {
    #[cfg(test)]
    pub fn generate() -> Jwk {
        let signing_key = k256::ecdsa::SigningKey::random(&mut rand::thread_rng());
        let verifying_key = signing_key.verifying_key();
        let serialized_pub_key = verifying_key.to_encoded_point(false);
        let bytes = serialized_pub_key.as_bytes();
        let x_bytes = &bytes[1..33];
        let y_bytes = &bytes[33..65];

        Jwk {
            alg: Some("ES256K".to_string()),
            kty: "EC".to_string(),
            crv: "secp256k1".to_string(),
            x: general_purpose::URL_SAFE_NO_PAD.encode(x_bytes),
            y: Some(general_purpose::URL_SAFE_NO_PAD.encode(y_bytes)),
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(signing_key.to_bytes().as_slice())),
        }
    }
}

#[cfg(test)]
pub fn to_public_jwk(jwk: &Jwk) -> Jwk {
    Jwk {
        alg: jwk.alg.clone(),
        kty: jwk.kty.clone(),
        crv: jwk.crv.clone(),
        x: jwk.x.clone(),
        y: jwk.y.clone(),
        ..Default::default()
    }
}

#[cfg(test)]
pub fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;
    let decoded_y = general_purpose::URL_SAFE_NO_PAD.decode(
        jwk.y
            .as_ref()
            .ok_or(DsaError::PublicKeyFailure("missing y".to_string()))?,
    )?;

    let mut pk_bytes = Vec::with_capacity(1 + decoded_x.len() + decoded_y.len());
    pk_bytes.push(0x04); // Prefix 0x04 denotes public key is uncompressed
    pk_bytes.extend_from_slice(&decoded_x);
    pk_bytes.extend_from_slice(&decoded_y);

    Ok(pk_bytes)
}

#[cfg(test)]
pub fn public_jwk_from_bytes(public_key: &[u8]) -> Result<Jwk> {
    let x_bytes = &public_key[1..33];
    let y_bytes = &public_key[33..65];
    Ok(Jwk {
        alg: Some("ES256K".to_string()),
        kty: "EC".to_string(),
        crv: "secp256k1".to_string(),
        x: general_purpose::URL_SAFE_NO_PAD.encode(x_bytes),
        y: Some(general_purpose::URL_SAFE_NO_PAD.encode(y_bytes)),
        ..Default::default()
    })
}
