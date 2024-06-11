

// impl ResolutionResult {
//     pub fn resolve(_uri: &str) -> Self {
//         unimplemented!()
//     }
// }

// pub struct DidJwk {
//     pub did: Did,
//     pub document: Document,
// }

// impl DidJwk {
//     pub fn from_public_key(_public_key: Jwk) -> Self {
//         unimplemented!()
//     }

//     pub fn from_uri(_uri: &str) -> Self {
//         unimplemented!()
//     }

//     pub fn resolve(_uri: &str) -> ResolutionResult {
//         unimplemented!()
//     }
// }

// pub struct DidWeb {
//     pub did: Did,
//     pub document: Document,
// }

// impl DidWeb {
//     pub fn from_uri(_uri: &str) -> Self {
//         unimplemented!()
//     }

//     pub fn resolve(_uri: &str) -> ResolutionResult {
//         unimplemented!()
//     }
// }

// #[derive(Default)]
// pub struct DidDht {
//     pub did: Did,
//     pub document: Document,
// }

// impl DidDht {
//     pub fn from_identity_key(_identity_key: Jwk) -> Self {
//         println!("from_identity_key");
//         Self {
//             ..Default::default()
//         }
//     }

//     pub fn from_uri(_uri: &str) -> Self {
//         println!("from_uri");
//         Self {
//             ..Default::default()
//         }
//     }

//     pub fn publish(&self, _signer: Arc<dyn Signer>) {
//         println!("publish");
//     }

//     pub fn deactivate(&self, _signer: Arc<dyn Signer>) {
//         println!("deactivate");
//     }

//     pub fn resolve(_uri: &str) -> ResolutionResult {
//         ResolutionResult {
//             ..Default::default()
//         }
//     }
// }

// pub struct VerifiableCredential {
//     pub context: Vec<String>,
//     pub id: String,
//     pub r#type: Vec<String>,
//     pub issuer: String, // 🚧
//     pub issuance_date: String,
//     pub expiration_date: Option<String>,
//     pub credential_subject: String, // 🚧
// }

// impl VerifiableCredential {
//     pub fn sign(&self, _signer: Arc<dyn Signer>) -> String {
//         unimplemented!()
//     }

//     pub fn verify(_vcjwt: String) -> Self {
//         unimplemented!()
//     }

//     pub fn verify_with_verifier(_vcjwt: String, _verifier: Arc<dyn Verifier>) -> Self {
//         unimplemented!()
//     }
// }
