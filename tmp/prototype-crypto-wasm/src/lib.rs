use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{SigningKey, SECRET_KEY_LENGTH};
use rand::{rngs::OsRng, RngCore};
use secp256k1::Secp256k1;
use serde_json::json;

pub fn prove_ed25519() {
    let mut csprng = OsRng {};
    let mut secret_key_bytes = [0u8; SECRET_KEY_LENGTH];
    csprng.fill_bytes(&mut secret_key_bytes); // Fill the byte array with random numbers

    let signing_key = SigningKey::from_bytes(&secret_key_bytes);
    let verifying_key = signing_key.verifying_key();

    let private_key_bytes = signing_key.to_bytes(); // Obtain the private key bytes
    let public_key_bytes = verifying_key.to_bytes(); // Obtain the public key bytes

    let jwk = json!({
        "kty": "OKP",                          // Key Type: Octet Key Pair
        "crv": "Ed25519",                      // Curve: Ed25519
        "d": general_purpose::URL_SAFE_NO_PAD.encode(&private_key_bytes),  // Private key
        "x": general_purpose::URL_SAFE_NO_PAD.encode(&public_key_bytes)    // Public key
    });

    println!("JWK: {}", jwk.to_string());
}

pub fn prove_secp256k1() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    // Serialize public key in uncompressed form
    let serialized_pub_key = public_key.serialize_uncompressed(); // 65 bytes: 0x04, x (32 bytes), y (32 bytes)

    let jwk = json!({
        "kty": "EC",
        "crv": "secp256k1",
        "x": general_purpose::URL_SAFE_NO_PAD.encode(&serialized_pub_key[1..33]), // Skip the first byte (0x04)
        "y": general_purpose::URL_SAFE_NO_PAD.encode(&serialized_pub_key[33..65]),
        "d": general_purpose::URL_SAFE_NO_PAD.encode(&secret_key.secret_bytes())
    });

    println!("JWK: {}", jwk);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ed25519() {
        prove_ed25519()
    }

    #[test]
    fn secp256k1() {
        prove_secp256k1()
    }
}