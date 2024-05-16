use std::sync::Arc;

use crate::{Claims, JwtError};
use ::jws::{CompactJws, JwsHeader};
use keys::key_manager::Signer;

// A JWT can be implemented as either a JWS or JWE, this module is the implementation of a JWT as a JWS

pub struct JwtDecoded<T: Claims> {
    pub header: JwsHeader,
    pub claims: T,
    pub signature: String,
    pub parts: Vec<String>,
}

pub struct Jwt;

impl Jwt {
    pub fn sign<T: Claims>(
        signer: Arc<dyn Signer>,
        jws_header: JwsHeader,
        claims: &T,
    ) -> Result<String, JwtError> {
        let serialized_claims = serde_json::to_string(claims)?.into_bytes();
        let jwt = CompactJws::sign(signer, &jws_header, &serialized_claims)?;
        Ok(jwt)
    }

    pub fn decode<T: Claims>(jwt: &str) -> Result<JwtDecoded<T>, JwtError> {
        let jws_decoded = CompactJws::decode(jwt)?;

        let claims = serde_json::from_slice::<T>(&jws_decoded.payload)?;

        Ok(JwtDecoded {
            header: jws_decoded.header,
            claims,
            signature: jws_decoded.signature,
            parts: jws_decoded.parts,
        })
    }

    pub async fn verify<T: Claims>(jwt: &str) -> Result<JwtDecoded<T>, JwtError> {
        let jws_decoded = CompactJws::verify(jwt).await?;

        let claims = serde_json::from_slice::<T>(&jws_decoded.payload)?;

        Ok(JwtDecoded {
            header: jws_decoded.header,
            claims,
            signature: jws_decoded.signature,
            parts: jws_decoded.parts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RegisteredClaims;
    use crypto::Curve;
    use dids::{
        document::{KeyIdFragment, KeySelector},
        methods::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Create,
        },
    };
    use keys::key_manager::{local_key_manager::LocalKeyManager, KeyManager};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_sign_and_verify() {
        let key_manager = Arc::new(LocalKeyManager::new());
        let bearer_did = DidJwk::create(
            key_manager.clone(),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let claims = RegisteredClaims {
            issuer: Some(bearer_did.identifier.uri.clone()),
            ..Default::default()
        };

        let key_id = bearer_did.document.verification_method[0].id.clone();
        let key_alias = KeyIdFragment(key_id.clone()).splice_key_alias();
        let signer = key_manager.get_signer(&key_alias).unwrap();

        let jws_header = JwsHeader::from_did_document(
            &bearer_did.document,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
        )
        .unwrap();
        let jwt = Jwt::sign(signer, jws_header, &claims).unwrap();

        let jwt_decoded = Jwt::verify::<RegisteredClaims>(&jwt).await.unwrap();

        // default JwsHeader
        assert_eq!("JWT".to_string(), jwt_decoded.header.typ);
        assert_eq!(key_id, jwt_decoded.header.kid);
        assert_eq!(
            bearer_did.document.verification_method[0]
                .public_key_jwk
                .alg,
            jwt_decoded.header.alg
        );

        // claims
        assert_eq!(claims.issuer, jwt_decoded.claims.issuer);
    }
}
