use super::Signer;
use super::Verifier;
use crate::crypto::jwk::Jwk;
use crate::errors::Result;
use crate::errors::Web5Error;
use base64::{engine::general_purpose, Engine as _};
use k256::ecdsa::signature::{Signer as K256Signer, Verifier as K256Verifier};
use k256::ecdsa::Signature;

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

#[derive(Clone)]
pub struct Secp256k1Signer {
    private_jwk: Jwk,
}

impl Secp256k1Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self { private_jwk }
    }
}

impl Signer for Secp256k1Signer {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        let d = self.private_jwk.d.as_ref().ok_or(Web5Error::Crypto(
            "private key material must be set".to_string(),
        ))?;

        let decoded_d = general_purpose::URL_SAFE_NO_PAD.decode(d)?;

        let signing_key = k256::ecdsa::SigningKey::from_slice(&decoded_d)
            .map_err(|_| Web5Error::Crypto("invalid private key".to_string()))?;

        let signature: Signature = signing_key.sign(payload);

        Ok(signature.to_vec())
    }
}

#[derive(Clone)]
pub struct Secp256k1Verifier {
    public_jwk: Jwk,
}

impl Secp256k1Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self { public_jwk }
    }
}

impl Verifier for Secp256k1Verifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<()> {
        if let Some(d) = &self.public_jwk.d {
            if !d.is_empty() {
                return Err(Web5Error::Crypto(
                    "provided verification key cannot contain private key material".to_string(),
                ));
            }
        }

        let public_key_bytes = public_jwk_extract_bytes(&self.public_jwk)?;

        let verifying_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_bytes)
            .map_err(|_| Web5Error::Crypto("unable to instantiate verifying key".to_string()))?;

        let signature = k256::ecdsa::Signature::from_slice(signature)
            .map_err(|_| Web5Error::Crypto("invalid signature".to_string()))?;

        verifying_key
            .verify(payload, &signature)
            .map_err(|_| Web5Error::Crypto("cryptographic verification failure".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::UnitTestSuite;
    use crate::test_name;
    use lazy_static::lazy_static;

    mod generate {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("secp256k1_generate");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_must_set_alg() {
            TEST_SUITE.include(test_name!());

            let jwk = Secp256k1Generator::generate();
            assert_eq!(jwk.alg, Some("ES256K".to_string()));
        }

        #[test]
        fn test_must_set_kty() {
            TEST_SUITE.include(test_name!());

            let jwk = Secp256k1Generator::generate();
            assert_eq!(jwk.kty, "EC".to_string());
        }

        #[test]
        fn test_must_set_crv() {
            TEST_SUITE.include(test_name!());

            let jwk = Secp256k1Generator::generate();
            assert_eq!(jwk.crv, "secp256k1");
        }

        #[test]
        fn test_must_set_public_key_with_correct_length() {
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

            let jwk = Secp256k1Generator::generate();
            let private_key_bytes = jwk.d.expect("Private key is missing");
            let decoded_private_key_bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(private_key_bytes)
                .expect("Failed to decode private key");
            assert_eq!(decoded_private_key_bytes.len(), 32);
        }
    }

    mod sign {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("secp256k1_sign");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_with_valid_key() {
            TEST_SUITE.include(test_name!());

            let jwk = Secp256k1Generator::generate();
            let signer = Secp256k1Signer::new(jwk);

            let message = b"Test message";
            let signature_result = signer.sign(message);

            assert!(
                signature_result.is_ok(),
                "Signing should succeed with a valid key"
            );

            let signature = signature_result.unwrap();
            assert_eq!(
                signature.len(),
                64, // Expected length for Secp256k1 signature (r + s, each 32 bytes)
                "Signature length should match the expected Secp256k1 signature length"
            );
        }

        #[test]
        fn test_with_invalid_private_key() {
            TEST_SUITE.include(test_name!());

            let mut jwk = Secp256k1Generator::generate();

            // Set an invalid private key (wrong length)
            jwk.d = Some(general_purpose::URL_SAFE_NO_PAD.encode(&[0u8; 31])); // One byte too short

            let signer = Secp256k1Signer::new(jwk);
            let message = b"Test message";
            let signature_result = signer.sign(message);

            assert!(
                signature_result.is_err(),
                "Signing should fail with an invalid private key"
            );
            assert_eq!(
                signature_result.unwrap_err(),
                Web5Error::Crypto("invalid private key".to_string())
            );
        }

        #[test]
        fn test_with_missing_private_key() {
            TEST_SUITE.include(test_name!());

            let mut jwk = Secp256k1Generator::generate();

            // Remove the private key
            jwk.d = None;

            let signer = Secp256k1Signer::new(jwk);
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

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("secp256k1_verify");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        fn generate_keys() -> (Jwk, Jwk) {
            let private_jwk = Secp256k1Generator::generate();
            let public_jwk = to_public_jwk(&private_jwk);
            (public_jwk, private_jwk)
        }

        #[test]
        fn test_with_valid_signature() {
            TEST_SUITE.include(test_name!());

            let (public_jwk, private_jwk) = generate_keys();
            let signer = Secp256k1Signer::new(private_jwk);
            let verifier = Secp256k1Verifier::new(public_jwk);

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
            TEST_SUITE.include(test_name!());

            let (_, private_jwk) = generate_keys();
            let verifier = Secp256k1Verifier::new(private_jwk); // Should not use a private key for verification

            let message = b"Test message";
            let invalid_signature = vec![0u8; 64]; // Invalid length

            let verify_result = verifier.verify(message, &invalid_signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail when a private key is used"
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
            TEST_SUITE.include(test_name!());

            let (public_jwk, private_jwk) = generate_keys();
            let signer = Secp256k1Signer::new(private_jwk);
            let verifier = Secp256k1Verifier::new(public_jwk);

            let message = b"Test message";

            let mut valid_signature = signer.sign(message).expect("Signing failed");
            let last_bit = valid_signature.len() - 1;
            valid_signature[last_bit] ^= 0x01; // Flip the last bit

            let verify_result = verifier.verify(message, &valid_signature);

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
            TEST_SUITE.include(test_name!());

            let (mut public_jwk, private_jwk) = generate_keys();
            public_jwk.x = general_purpose::URL_SAFE_NO_PAD.encode(&[0u8; 31]); // Invalid length

            let signer = Secp256k1Signer::new(private_jwk);
            let verifier = Secp256k1Verifier::new(public_jwk);

            let message = b"Test message";
            let signature = signer.sign(message).expect("Signing failed");

            let verify_result = verifier.verify(message, &signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail with an invalid public key"
            );
            assert_eq!(
                verify_result.unwrap_err(),
                Web5Error::Crypto("unable to instantiate verifying key".to_string())
            );
        }

        #[test]
        fn test_with_invalid_signature_length() {
            TEST_SUITE.include(test_name!());

            let (public_jwk, _) = generate_keys();
            let verifier = Secp256k1Verifier::new(public_jwk);

            let message = b"Test message";
            let invalid_signature = vec![0u8; 63]; // Invalid length (should be 64 bytes)

            let verify_result = verifier.verify(message, &invalid_signature);

            assert!(
                verify_result.is_err(),
                "Verification should fail with a signature of incorrect length"
            );
            assert_eq!(
                verify_result.unwrap_err(),
                Web5Error::Crypto("invalid signature".to_string())
            );
        }
    }
}
