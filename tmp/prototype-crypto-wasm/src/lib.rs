use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{SigningKey, SECRET_KEY_LENGTH};
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        SigningKey as k256SigningKey, VerifyingKey,
    },
    EncodedPoint,
};
use rand::{rngs::OsRng, RngCore};
use secp256k1::Secp256k1;
use serde_json::json;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn prove_bitcoin_secp256k1() {
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

#[wasm_bindgen]
pub fn prove_secp256k1() {
    // Generate a new signing key (private key)
    let signing_key = k256SigningKey::random(&mut rand::thread_rng());
    let verifying_key = signing_key.verifying_key();

    // Serialize the public key in uncompressed form
    let serialized_pub_key = verifying_key.to_encoded_point(false); // false to get uncompressed

    // Extract x and y coordinates
    let bytes = serialized_pub_key.as_bytes();
    let x_bytes = &bytes[1..33]; // Skip the first byte (0x04 for uncompressed)
    let y_bytes = &bytes[33..65];

    // // Encode x, y, and d to base64 URL safe without padding
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x_bytes);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y_bytes);
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(signing_key.to_bytes().as_slice());

    // Create a JWK JSON object
    let jwk = json!({
        "kty": "EC",
        "crv": "secp256k1",
        "x": x_b64,
        "y": y_b64,
        "d": d_b64
    });

    println!("JWK: {}", jwk.to_string());
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

    #[test]
    fn bitcoin_secp256k1() {
        prove_bitcoin_secp256k1()
    }
}
