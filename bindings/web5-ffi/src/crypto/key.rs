use ssi_jwk::JWK;
use std::sync::Arc;

#[derive(uniffi::Enum)]
pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

pub trait Key {
    fn alias(&self) -> KeyAlias;
}

pub type KeyAlias = String;

#[derive(uniffi::Object, Clone)]
pub struct PrivateKey(pub JWK);

#[uniffi::export]
impl Key for PrivateKey {
    fn alias(&self) -> KeyAlias {
        self.0.thumbprint().unwrap()
    }
}

#[uniffi::export]
impl PrivateKey {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    pub fn to_public_key(&self) -> Arc<PublicKey> {
        Arc::new(PublicKey(self.0.to_public()))
    }

    pub fn sign(&self, payload: &Vec<u8>) -> Vec<u8> {
        let algorithm = self
            .0
            .get_algorithm()
            .expect("Expected algorithm to be present");
        let signed_bytes = ssi_jws::sign_bytes(algorithm, &payload, &self.0)
            .expect("Signature not computed properly");

        signed_bytes
    }
}

#[derive(uniffi::Object, Clone)]
pub struct PublicKey(pub JWK);

#[uniffi::export]
impl Key for PublicKey {
    fn alias(&self) -> KeyAlias {
        self.0.thumbprint().unwrap()
    }
}

#[uniffi::export]
impl PublicKey {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    fn verify(&self, payload: &Vec<u8>, signature: &Vec<u8>) -> bool {
        let algorithm = self
            .0
            .get_algorithm()
            .expect("Expected algorithm to be present");
        let verify_result =
            ssi_jws::verify_bytes_warnable(algorithm, &payload, &self.0, &signature);
        match verify_result {
            Ok(warnings) => {
                if warnings.len() > 0 {
                    println!("VerificationWarnings: {:?}", warnings);
                }
                true
            }
            Err(e) => {
                println!("VerificationError: {:?}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_derivation() {
        // Creating a public key from a private key should produce a different key
        let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key = PrivateKey(jwk);
        let public_key = private_key.to_public_key();

        assert_ne!(private_key.0, public_key.0);
    }

    #[test]
    fn test_alias_consistency() {
        // An alias created computed from a public & private key, generated from the same JWK,
        // should be the same
        let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key = PrivateKey(jwk);
        let public_key = private_key.to_public_key();

        let private_key_alias = private_key.alias();
        let public_key_alias = public_key.alias();

        assert_eq!(private_key_alias, public_key_alias);
    }

    #[test]
    fn test_signing_consistency() {
        // Signing the same payload with the same key should produce the same signature
        let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key = PrivateKey(jwk);

        let payload = b"Hello, world!".to_vec();
        let signature_1 = private_key.sign(&payload);
        let signature_2 = private_key.sign(&payload);

        assert_eq!(signature_1, signature_2);
    }

    #[test]
    fn test_verification_success() {
        // Verifying a payload with the public key of the private key that signed it
        // should result in a successful verification
        let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key = PrivateKey(jwk);

        let payload = b"Hello, world!".to_vec();
        let signature = private_key.sign(&payload);

        let public_key = private_key.to_public_key();
        let verification_result = public_key.verify(&payload, &signature);

        assert!(verification_result);
    }

    #[test]
    fn test_verification_failure() {
        // Verifying a payload with a public key different from the private key that signed it
        // should result in a failed verification
        let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key = PrivateKey(jwk);

        let payload = b"Hello, world!".to_vec();
        let signature = private_key.sign(&payload);

        let jwk_2 = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key_2 = PrivateKey(jwk_2);
        let public_key_2 = private_key_2.to_public_key();

        let verification_result = public_key_2.verify(&payload, &signature);
        assert!(!verification_result);
    }

    #[test]
    fn test_verification_consistency() {
        // Verifying the same payload with the same key should produce the same result
        let jwk = JWK::generate_ed25519().expect("Failed to generate JWK");
        let private_key = PrivateKey(jwk.clone());
        let public_key = private_key.to_public_key();

        let payload = b"Hello, world!".to_vec();
        let signature = private_key.sign(&payload);
        let verification_1 = public_key.verify(&payload, &signature);
        let verification_2 = public_key.verify(&payload, &signature);

        assert_eq!(verification_1, verification_2);
    }
}
