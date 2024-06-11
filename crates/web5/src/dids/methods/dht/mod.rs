use std::sync::Arc;

use reqwest::Client;

use crate::crypto::ed25519::Ed25519;
use crate::crypto::Curve;
use crate::dids::document::{VerificationMethod, VerificationPurposes};
use crate::dids::identifier::Identifier;
use crate::dids::resolver::ResolutionResult;
use crate::jwk::Jwk;
use crate::keys::key_manager::KeyManager;
use crate::dids::{bearer::BearerDid, document::{Document, Service}};
use crate::keys::KeyError;

use self::bep44::Bep44Message;

use super::MethodError;

pub mod bep44;
pub mod document_packet;

const JSON_WEB_KEY: &str = "JsonWebKey";

pub struct VerificationMethodOpts {
  // TODO: id
  pub public_key: Jwk,
  pub purposes: Vec<VerificationPurposes>,
  pub controller: Option<String>,
}

pub enum RegisteredDidType {
  Discoverable,
  Organization,
  GovernmentOrganization,
  Corporation,
  LocalBusiness,
  SoftwarePackage,
  WebApp,
  FinancialInstitution,
}

pub struct DidDhtOptions {
  pub also_known_as: Vec<String>,
  pub controller: Vec<String>,
  pub service: Option<Vec<Service>>,
  pub identity_key_jwk: Jwk,
  pub additional_verification_methods: Option<Vec<VerificationMethodOpts>>,
}

// We should put this somewhere else after that API design doc is out
pub trait Signer: Send + Sync {
  fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, MethodError>;
}

fn create_identifier(identity_key_jwk: &Jwk) -> String {
  let pubkey_bytes = Ed25519::extract_public_key(identity_key_jwk).unwrap(); // TODO: don't unwrap
  let suffix = zbase32::encode_full_bytes(&pubkey_bytes);
  format!("did:dht:{}", suffix)
}

// TODO: Implement Method, Create, and Resolve traits and add update method
/// Concrete implementation for a did:dht DID
pub struct DidDht {
  did: Identifier,
  document: Document,
}

impl DidDht {
  pub fn create_document(
    options: DidDhtOptions,
  ) -> Result<Document, MethodError> {
    // TODO maybe: verify identity key is ed25519

    let did_uri = create_identifier(&options.identity_key_jwk);

    // Add identity key to purposes except key_agreement
    let identity_key_verification_method = VerificationMethod {
        id: format!("{}#0", &did_uri),
        r#type: JSON_WEB_KEY.to_string(),
        controller: did_uri.clone(),
        public_key_jwk: options.identity_key_jwk,
    };
    let mut capability_delegation = vec![identity_key_verification_method.id.clone()];
    let mut capability_invocation = vec![identity_key_verification_method.id.clone()];
    let mut authentication = vec![identity_key_verification_method.id.clone()];
    let mut assertion_method = vec![identity_key_verification_method.id.clone()];
    let mut key_agreement = vec![];
    let mut verification_methods = vec![identity_key_verification_method];

    // Add additional verification methods and purposes
    if let Some(additional_verification_methods) =  options.additional_verification_methods {
      for vm_opts in additional_verification_methods {
        let verification_method = VerificationMethod {
          id: format!("{}#{}", did_uri, &vm_opts.public_key.compute_thumbprint().unwrap()), // TODO: don't unwrap
          r#type: JSON_WEB_KEY.to_string(),
          controller: "foo".to_string(),
          public_key_jwk: vm_opts.public_key,
        };
  
        for purpose in vm_opts.purposes {
          match purpose {
              VerificationPurposes::Authentication => authentication.push(verification_method.id.clone()),
              VerificationPurposes::AssertionMethod => assertion_method.push(verification_method.id.clone()),
              VerificationPurposes::CapabilityInvocation => capability_invocation.push(verification_method.id.clone()),
              VerificationPurposes::CapabilityDelegation => capability_delegation.push(verification_method.id.clone()),
              VerificationPurposes::KeyAgreement => key_agreement.push(verification_method.id.clone()),
          }
        }
  
        verification_methods.push(verification_method);
      }
    }

    Ok(Document {
      id: did_uri.clone(),
      verification_method: verification_methods,
      capability_delegation: Some(capability_delegation),
      capability_invocation: Some(capability_invocation),
      authentication: Some(authentication),
      assertion_method: Some(assertion_method),
      key_agreement: Some(key_agreement),
      service: options.service,
      ..Default::default()
    })
  }

  pub fn publish_blocking(&self, identity_key_signer: &dyn Signer) -> Result<(), MethodError> {
    let packet = document.to_dns_packet().unwrap(); // TODO: don't unwrap
    let packet_bytes = packet
                .build_bytes_vec_compressed()
                .unwrap(); // TODO: don't unwrap

    let bep44_message = Bep44Message::new(&packet_bytes, |payload| -> Result<Vec<u8>, KeyError> {
        identity_key_signer.sign(&payload)
      })
      .unwrap() // TODO: don't unwrap
      .encode()
      .unwrap(); // TODO: don't unwrap
      
      let client = Client::new();
  
    // TODO: support custom gateway
    let gateway = "https://diddht.tbddev.org";

    Ok(())
  }

  // TODO
  // pub fn publish(document: &Document, identity_key_signer: &dyn Signer) -> Result<(), MethodError> {}

  // TODO
  // pub fn resolve(uri: &str) -> ResolutionResult {}
}

/*

let myresolution_result = DidDht::resolve("did:dht:123");
let myDidDht = DidDht::resolve("did:dht:123").try_into().unwrap();

*/