use super::{Signer, Verifier};
use crate::{
    crypto::jwk::Jwk,
    errors::{Result, Web5Error},
};
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{
    Signature, Signer as DalekSigner, SigningKey, Verifier as DalekVerifier, VerifyingKey,
    PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH,
};
use rand::rngs::OsRng;

/// A key generator for Ed25519, used for creating JWKs with Ed25519 key pairs.
pub struct Ed25519Generator;

impl Ed25519Generator {
    /// Generates a new Ed25519 key pair and returns it as a JWK.
    ///
    /// The method creates a new Ed25519 private key and derives the public key from it. Both the private
    /// key (`d`) and the public key (`x`) are encoded in base64url format and returned as a JWK.
    ///
    /// # Returns
    /// A `Jwk` object containing the generated Ed25519 key pair.
    pub fn generate() -> Jwk {
        let signing_key = SigningKey::generate(&mut OsRng {});
        let verifying_key = signing_key.verifying_key();

        let private_key_bytes = signing_key.to_bytes();
        let public_key_bytes = verifying_key.to_bytes();

        Jwk {
            alg: Some("Ed25519".to_string()),
            kty: "OKP".to_string(),
            crv: "Ed25519".to_string(),
            x: general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes),
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(private_key_bytes)),
            ..Default::default()
        }
    }
}

/// Creates a JWK from raw Ed25519 public key bytes.
///
/// This function takes the raw bytes of an Ed25519 public key and constructs a corresponding JWK. The
/// public key is encoded in base64url format and stored in the `x` field of the JWK.
///
/// # Arguments
/// * `public_key` - A byte slice containing the raw public key.
///
/// # Returns
/// A `Result` containing the constructed JWK, or an error if the key length is incorrect.
pub(crate) fn public_jwk_from_bytes(public_key: &[u8]) -> Result<Jwk> {
    if public_key.len() != PUBLIC_KEY_LENGTH {
        return Err(Web5Error::Parameter(format!(
            "Public key has incorrect length {}",
            PUBLIC_KEY_LENGTH
        )));
    }

    Ok(Jwk {
        alg: Some("Ed25519".to_string()),
        kty: "OKP".to_string(),
        crv: "Ed25519".to_string(),
        x: general_purpose::URL_SAFE_NO_PAD.encode(public_key),
        ..Default::default()
    })
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

/// Extracts the public key bytes from a JWK.
///
/// This function decodes the base64url-encoded `x` value of a JWK and returns it as raw bytes.
///
/// # Arguments
/// * `jwk` - The JWK containing the public key.
///
/// # Returns
/// A `Result` containing the decoded public key bytes, or an error if the length is incorrect.
pub(crate) fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;

    if decoded_x.len() != PUBLIC_KEY_LENGTH {
        return Err(Web5Error::Parameter(format!(
            "public key invalid length {}",
            PUBLIC_KEY_LENGTH
        )));
    }

    Ok(decoded_x)
}

/// A signer for Ed25519 keys.
///
/// The `Ed25519Signer` is responsible for signing messages using the Ed25519 private key material contained
/// in a JWK.
#[derive(Clone)]
pub struct Ed25519Signer {
    private_jwk: Jwk,
}

/// A signer for Ed25519 keys.
///
/// The `Ed25519Signer` is responsible for signing messages using the Ed25519 private key material contained
/// in a JWK.
impl Ed25519Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self { private_jwk }
    }
}

impl Signer for Ed25519Signer {
    /// Signs the given payload using the Ed25519 private key.
    ///
    /// The private key is extracted from the JWK and used to sign the payload. The resulting signature
    /// is returned as a vector of bytes.
    ///
    /// # Arguments
    /// * `payload` - The data to be signed.
    ///
    /// # Returns
    /// A `Result` containing the signature as a vector of bytes, or an error if signing fails.
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        let d = self.private_jwk.d.as_ref().ok_or(Web5Error::Crypto(
            "private key material must be set".to_string(),
        ))?;
        let decoded_d = general_purpose::URL_SAFE_NO_PAD.decode(d)?;
        if decoded_d.len() != SECRET_KEY_LENGTH {
            return Err(Web5Error::Crypto(format!(
                "invalid private key length {} must be {}",
                decoded_d.len(),
                SECRET_KEY_LENGTH
            )));
        }
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&decoded_d);
        let signing_key = SigningKey::from_bytes(&key_array);
        let signature = signing_key.sign(payload);
        Ok(signature.to_vec())
    }
}

/// A verifier for Ed25519 keys.
///
/// The `Ed25519Verifier` is responsible for verifying signatures using the Ed25519 public key material
/// contained in a JWK.
#[derive(Clone)]
pub struct Ed25519Verifier {
    public_jwk: Jwk,
}

impl Ed25519Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self { public_jwk }
    }
}

impl Verifier for Ed25519Verifier {
    /// Verifies the given signature using the Ed25519 public key.
    ///
    /// The public key is extracted from the JWK and used to verify the signature against the provided payload.
    ///
    /// # Arguments
    /// * `payload` - The data that was signed.
    /// * `signature` - The signature to verify.
    ///
    /// # Returns
    /// A `Result` indicating whether the signature is valid.
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<()> {
        if let Some(d) = &self.public_jwk.d {
            if !d.is_empty() {
                return Err(Web5Error::Crypto(
                    "provided verification key cannot contain private key material".to_string(),
                ));
            }
        }

        let mut public_key_bytes = [0u8; PUBLIC_KEY_LENGTH];
        let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&self.public_jwk.x)?;

        if decoded_x.len() != PUBLIC_KEY_LENGTH {
            return Err(Web5Error::Crypto(format!(
                "invalid public key length {} must be {}",
                decoded_x.len(),
                PUBLIC_KEY_LENGTH
            )));
        }

        public_key_bytes.copy_from_slice(&decoded_x);
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|e| Web5Error::Crypto(format!("unable to instantiate verifying key {}", e)))?;

        if signature.len() != SIGNATURE_LENGTH {
            return Err(Web5Error::Crypto(format!(
                "invalid signature length {} must be {}",
                signature.len(),
                SIGNATURE_LENGTH
            )));
        }

        let mut signature_bytes = [0u8; SIGNATURE_LENGTH];
        signature_bytes.copy_from_slice(signature);
        let verify_result = verifying_key.verify(payload, &Signature::from_bytes(&signature_bytes));

        match verify_result {
            Ok(_) => Ok(()),
            Err(_) => Err(Web5Error::Crypto(
                "cryptographic verification failure".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use general_purpose::URL_SAFE_NO_PAD;

    mod generate {
        use super::*;

        #[test]
        fn test_must_set_alg() {
            let jwk = Ed25519Generator::generate();
            assert_eq!(jwk.alg, Some("Ed25519".to_string()));
        }

        #[test]
        fn test_must_set_kty() {
            let jwk = Ed25519Generator::generate();
            assert_eq!(jwk.kty, "OKP".to_string());
        }

        #[test]
        fn test_must_set_crv() {
            let jwk = Ed25519Generator::generate();
            assert_eq!(jwk.crv, "Ed25519");
        }

        #[test]
        fn test_must_set_public_key_with_correct_length() {
            let jwk = Ed25519Generator::generate();
            let public_key_bytes = URL_SAFE_NO_PAD
                .decode(&jwk.x)
                .expect("Failed to decode public key");
            assert_eq!(public_key_bytes.len(), PUBLIC_KEY_LENGTH);
        }

        #[test]
        fn test_must_set_private_key_with_correct_length() {
            let jwk = Ed25519Generator::generate();
            let private_key_bytes = jwk.d.expect("Private key is missing");
            let decoded_private_key_bytes = URL_SAFE_NO_PAD
                .decode(private_key_bytes)
                .expect("Failed to decode private key");
            assert_eq!(decoded_private_key_bytes.len(), SECRET_KEY_LENGTH);
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn test_with_valid_key() {
            let jwk = Ed25519Generator::generate();
            let signer = Ed25519Signer::new(jwk);

            let message = b"Test message";
            let signature_result = signer.sign(message);

            assert!(
                signature_result.is_ok(),
                "Signing should succeed with a valid key"
            );

            let signature = signature_result.unwrap();
            assert_eq!(
                signature.len(),
                SIGNATURE_LENGTH,
                "Signature length should match the expected Ed25519 signature length"
            );
        }

        #[test]
        fn test_with_invalid_private_key() {
            let mut jwk = Ed25519Generator::generate();

            // Set an invalid private key (wrong length)
            jwk.d = Some(URL_SAFE_NO_PAD.encode(&[0u8; SECRET_KEY_LENGTH - 1]));

            let signer = Ed25519Signer::new(jwk);
            let message = b"Test message";
            let signature_result = signer.sign(message);

            assert!(
                signature_result.is_err(),
                "Signing should fail with an invalid private key"
            );
            assert_eq!(
                signature_result.unwrap_err(),
                Web5Error::Crypto(format!(
                    "invalid private key length {} must be {}",
                    SECRET_KEY_LENGTH - 1,
                    SECRET_KEY_LENGTH
                ))
            );
        }

        #[test]
        fn test_with_missing_private_key() {
            let mut jwk = Ed25519Generator::generate();

            // Remove the private key
            jwk.d = None;

            let signer = Ed25519Signer::new(jwk);
            let message = b"Test message";
            let signature_result = signer.sign(message);

            assert!(
                signature_result.is_err(),
                "Signing should fail if the private key is missing"
            );
            assert_eq!(
                signature_result.unwrap_err(),
                Web5Error::Crypto("private key material must be set".to_string())
            );
        }
    }

    mod verify {
        use super::*;

        fn generate_keys() -> (Jwk, Jwk) {
            let private_jwk = Ed25519Generator::generate();
            let mut public_jwk = private_jwk.clone();
            public_jwk.d = None;
            (public_jwk, private_jwk)
        }

        #[test]
        fn test_with_valid_signature() {
            let (public_jwk, private_jwk) = generate_keys();
            let signer = Ed25519Signer::new(private_jwk);
            let verifier = Ed25519Verifier::new(public_jwk);

            let message = b"Test message";
            let signature = signer.sign(message).expect("Signing failed");

            let verify_result = verifier.verify(message, &signature);

            assert!(
                verify_result.is_ok(),
                "Verification should succeed with a valid signature"
            );
        }

        #[test]
        fn test_with_private_key() {
            let (_, private_jwk) = generate_keys();
            let verifier = Ed25519Verifier::new(private_jwk); // this is not allowed

            let message = b"Test message";
            let invalid_signature = vec![0u8; SIGNATURE_LENGTH - 1]; // invalid length

            let verify_result = verifier.verify(message, &invalid_signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail with private key not allowed"
            );
            assert_eq!(
                verify_result.unwrap_err(),
                Web5Error::Crypto(
                    "provided verification key cannot contain private key material".to_string()
                )
            );
        }

        #[test]
        fn test_with_invalid_signature() {
            let (public_jwk, _) = generate_keys();
            let verifier = Ed25519Verifier::new(public_jwk);

            let message = b"Test message";
            let invalid_signature = vec![0u8; SIGNATURE_LENGTH]; // an obviously invalid signature

            let verify_result = verifier.verify(message, &invalid_signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail with an invalid signature"
            );
            assert_eq!(
                verify_result.unwrap_err(),
                Web5Error::Crypto("cryptographic verification failure".to_string())
            );
        }

        #[test]
        fn test_with_invalid_public_key() {
            let (mut public_jwk, private_jwk) = generate_keys();
            public_jwk.x = URL_SAFE_NO_PAD.encode(&[0u8; PUBLIC_KEY_LENGTH - 1]);

            let signer = Ed25519Signer::new(private_jwk);
            let verifier = Ed25519Verifier::new(public_jwk);

            let message = b"Test message";
            let signature = signer.sign(message).expect("Signing failed");

            let verify_result = verifier.verify(message, &signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail with an invalid public key"
            );
            assert_eq!(
                verify_result.unwrap_err(),
                Web5Error::Crypto(format!(
                    "invalid public key length {} must be {}",
                    PUBLIC_KEY_LENGTH - 1,
                    PUBLIC_KEY_LENGTH
                ))
            );
        }

        #[test]
        fn test_with_invalid_signature_length() {
            let (public_jwk, _) = generate_keys();
            let verifier = Ed25519Verifier::new(public_jwk);

            let message = b"Test message";
            let invalid_signature = vec![0u8; SIGNATURE_LENGTH - 1]; // invalid length

            let verify_result = verifier.verify(message, &invalid_signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail with a signature of incorrect length"
            );
            assert_eq!(
                verify_result.unwrap_err(),
                Web5Error::Crypto(format!(
                    "invalid signature length {} must be {}",
                    SIGNATURE_LENGTH - 1,
                    SIGNATURE_LENGTH
                ))
            );
        }
    }
}
