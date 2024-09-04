use std::sync::Arc;

use crate::{
    crypto::dsa::ed25519::Ed25519Verifier,
    dids::{
        data_model::document::FindVerificationMethodOptions,
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    errors::{Result, Web5Error},
};

use super::{
    credential_subject::CredentialSubject,
    issuer::Issuer,
    josekit::{JoseVerifier, JoseVerifierAlwaysTrue},
    jwt_payload_vc::JwtPayloadVerifiableCredential,
    verifiable_credential_1_1::VerifiableCredential,
    CredentialError,
};

pub fn decode(vc_jwt: &str, verify_signature: bool) -> Result<VerifiableCredential> {
    let header = josekit::jwt::decode_header(vc_jwt)
        .map_err(|_| Web5Error::Parameter("failed to decode vc-jwt jose header".to_string()))?;

    let kid = header
        .claim("kid")
        .and_then(serde_json::Value::as_str)
        .ok_or(CredentialError::MissingKid)?;

    if kid.is_empty() {
        return Err(CredentialError::MissingKid.into());
    }

    let jwt_payload = if verify_signature {
        let did = Did::parse(kid)?;

        let resolution_result = ResolutionResult::resolve(&did.uri);
        if let Some(err) = resolution_result.resolution_metadata.error.clone() {
            return Err(err.into());
        }

        let public_key_jwk = resolution_result
            .document
            .ok_or(ResolutionMetadataError::InternalError)?
            .find_verification_method(FindVerificationMethodOptions {
                verification_method_id: Some(kid.to_string()),
            })?
            .public_key_jwk;

        let jose_verifier = &JoseVerifier {
            kid: kid.to_string(),
            verifier: Arc::new(Ed25519Verifier::new(public_key_jwk)),
        };

        let (jwt_payload, _) =
            josekit::jwt::decode_with_verifier(vc_jwt, jose_verifier).map_err(|e| {
                Web5Error::Crypto(format!("vc-jwt failed cryptographic verification {}", e))
            })?;

        jwt_payload
    } else {
        let (jwt_payload, _) = josekit::jwt::decode_with_verifier(
            vc_jwt,
            &JoseVerifierAlwaysTrue {
                kid: kid.to_string(),
            },
        )
        .map_err(|e| Web5Error::Crypto(format!("vc-jwt failed to decode payload {}", e)))?;

        jwt_payload
    };

    let vc_claim = jwt_payload
        .claim("vc")
        .ok_or(CredentialError::MissingClaim("vc".to_string()))?;
    let vc_payload = serde_json::from_value::<JwtPayloadVerifiableCredential>(vc_claim.clone())?;

    // registered claims checks
    let jti = jwt_payload
        .jwt_id()
        .ok_or(CredentialError::MissingClaim("jti".to_string()))?;
    let iss = jwt_payload
        .issuer()
        .ok_or(CredentialError::MissingClaim("issuer".to_string()))?;
    let sub = jwt_payload
        .subject()
        .ok_or(CredentialError::MissingClaim("subject".to_string()))?;
    let nbf = jwt_payload
        .not_before()
        .ok_or(CredentialError::MissingClaim("not_before".to_string()))?;
    let exp = jwt_payload.expires_at();

    if let Some(id) = &vc_payload.id {
        if id != jti {
            return Err(CredentialError::ClaimMismatch("id".to_string()).into());
        }
    }

    if let Some(issuer) = &vc_payload.issuer {
        let vc_issuer = issuer.to_string();
        if iss != vc_issuer {
            return Err(CredentialError::ClaimMismatch("issuer".to_string()).into());
        }
    }

    if let Some(credential_subject) = &vc_payload.credential_subject {
        if sub != credential_subject.id {
            return Err(CredentialError::ClaimMismatch("subject".to_string()).into());
        }
    }

    if let Some(vc_payload_expiration_date) = vc_payload.expiration_date {
        match exp {
            None => {
                return Err(CredentialError::MisconfiguredExpirationDate(
                    "VC has expiration date but no exp in registered claims".to_string(),
                )
                .into());
            }
            Some(exp) => {
                let difference = vc_payload_expiration_date
                    .duration_since(exp)
                    .unwrap_or_else(|_| exp.duration_since(vc_payload_expiration_date).unwrap());

                if difference.as_secs() > 0 {
                    return Err(
                        CredentialError::ClaimMismatch("expiration_date".to_string()).into(),
                    );
                }
            }
        }
    }

    let vc_issuer = vc_payload.issuer.unwrap_or(Issuer::String(iss.to_string()));

    let vc_credential_subject = vc_payload.credential_subject.unwrap_or(CredentialSubject {
        id: sub.to_string(),
        additional_properties: None,
    });

    Ok(VerifiableCredential {
        context: vc_payload.context,
        id: jti.to_string(),
        r#type: vc_payload.r#type,
        issuer: vc_issuer,
        issuance_date: nbf,
        expiration_date: exp,
        credential_status: vc_payload.credential_status,
        credential_subject: vc_credential_subject,
        credential_schema: vc_payload.credential_schema,
        evidence: vc_payload.evidence,
    })
}
