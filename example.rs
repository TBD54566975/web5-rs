// Import necessary modules and traits
use web5::{Jwk, DidJwk, KeySigner, InMemoryKeySigner, VerifiableCredential};

/**
 * 1. existing private keys
 * 2. existing did dht
 * 3. sign vc
 */
let private_jwks = serde_json::from_string("{...your JWK with a d in it}");
let key_signer = InMemoryKeySigner::from_private_jwks(private_jwks);

let did_uri = "did:dht:123456";
let resolution = DidDht::resolve(did_uri).await;
let public_jwk = resolution.document.verification_method[0].public_key_jwk;

// ---

let key_signer = InMemoryKeySigner::new();
let public_jwk = key_signer.generate_private_key();

let did_dht = DidDht::create(key_signer, public_jwk, DidDhtCreateOptions {});
let universal_resolution = Resolution::resolve(did_dht.identifier.uri);
let resolution = DidDht::resolve(did_dht.identifier.uri);

// let did_jwk = DidJwk::create(public_jwk);
// could resolve that too

let vc = VerifiableCredential {
    context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
    id: "http://example.edu/credentials/1872".to_string(),
    type_: vec!["VerifiableCredential".to_string()],
    issuer: did_jwk.identifier.uri.clone(),
    issuance_date: "2024-05-23T00:00:00Z".to_string(),
    credential_subject: HashMap::new(),
    proof: None,
};
let jws_signer = key_signer.get_jws_signer(public_jwk);
let vcjwt = vc.sign(jws_signer);

let jws_verifier = key_signer.get_jws_verifier(public_jwk);
let vc = VerifiableCredential::verify_with_verifier(vcjwt, jws_verifier).await;

// // Step 1: Create a JWK
// let jwk = Jwk {
//     alg: "EdDSA".to_string(),
//     kty: "OKP".to_string(),
//     crv: "Ed25519".to_string(),
//     d: None,
//     x: "base64-url-encoded-public-key".to_string(),
//     y: None,
// };

// // Step 2: Create a DID from the JWK
// let did_jwk = DidJwk::create(&jwk);

// // Print the DID
// println!("DID: {}", did_jwk.identifier.uri);

// // Step 3: Create a Verifiable Credential
// let mut vc = VerifiableCredential {
//     context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
//     id: "http://example.edu/credentials/1872".to_string(),
//     type_: vec!["VerifiableCredential".to_string()],
//     issuer: did_jwk.identifier.uri.clone(),
//     issuance_date: "2024-05-23T00:00:00Z".to_string(),
//     credential_subject: HashMap::new(),
//     proof: None,
// };

// // Step 4: Initialize the InMemoryKeySigner
// let key_signer = InMemoryKeySigner::from_private_jwks(&vec![jwk]);

// // Step 5: Sign the Verifiable Credential
// let jws_signer = key_signer.get_jws_signer(&jwk);
// let signed_vc = vc.sign(&jws_signer);

// // Print the signed VC
// println!("Signed VC: {}", signed_vc);

// // Step 6: Verify the Verifiable Credential
// let jws_verifier = key_signer.get_jws_verifier(&jwk); // Implement get_jws_verifier method
// let verified_vc = VerifiableCredential::verify_with_verifier(&signed_vc, &jws_verifier);

// // Print the verification result
// match verified_vc {
//     Ok(vc) => println!("Verified VC: {:?}", vc),
//     Err(err) => println!("Verification failed: {}", err),
// }