use crate::crypto::jwk::Jwk;
use crate::errors::Result;
use crate::errors::Web5Error;
use base64::{engine::general_purpose, Engine as _};

pub struct Secp256k1Generator;

impl Secp256k1Generator {
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

pub fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;
    let decoded_y = general_purpose::URL_SAFE_NO_PAD.decode(
        jwk.y
            .as_ref()
            .ok_or(Web5Error::Parameter("missing y".to_string()))?,
    )?;

    let mut pk_bytes = Vec::with_capacity(1 + decoded_x.len() + decoded_y.len());
    pk_bytes.push(0x04); // Prefix 0x04 denotes public key is uncompressed
    pk_bytes.extend_from_slice(&decoded_x);
    pk_bytes.extend_from_slice(&decoded_y);

    Ok(pk_bytes)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    mod generate {
        use super::*;

        #[test]
        fn test_must_set_alg() {
            let jwk = Secp256k1Generator::generate();
            assert_eq!(jwk.alg, Some("ES256K".to_string()));
        }

        #[test]
        fn test_must_set_kty() {
            let jwk = Secp256k1Generator::generate();
            assert_eq!(jwk.kty, "EC".to_string());
        }

        #[test]
        fn test_must_set_crv() {
            let jwk = Secp256k1Generator::generate();
            assert_eq!(jwk.crv, "secp256k1");
        }

        #[test]
        fn test_must_set_public_key_with_correct_length() {
            let jwk = Secp256k1Generator::generate();
            let x_bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(&jwk.x)
                .expect("Failed to decode x coordinate");
            let y_bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(jwk.y.as_ref().expect("y coordinate is missing"))
                .expect("Failed to decode y coordinate");
            assert_eq!(x_bytes.len(), 32);
            assert_eq!(y_bytes.len(), 32);
        }

        #[test]
        fn test_must_set_private_key_with_correct_length() {
            let jwk = Secp256k1Generator::generate();
            let private_key_bytes = jwk.d.expect("Private key is missing");
            let decoded_private_key_bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(private_key_bytes)
                .expect("Failed to decode private key");
            assert_eq!(decoded_private_key_bytes.len(), 32);
        }
    }
}
