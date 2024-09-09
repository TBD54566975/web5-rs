use super::{
    credential_subject::CredentialSubject, issuer::Issuer,
    jwt_payload_vc::JwtPayloadVerifiableCredential,
    verifiable_credential_1_1::VerifiableCredential, VerificationError,
};
use crate::{errors::Result, jose::Jwt, json::FromJsonValue};

pub fn decode(vc_jwt: &str, verify_signature: bool) -> Result<VerifiableCredential> {
    let jwt = Jwt::from_compact_jws(vc_jwt, verify_signature)?;

    let jti = jwt
        .claims
        .jti
        .ok_or(VerificationError::MissingClaim("jti".to_string()))?;
    let iss = jwt
        .claims
        .iss
        .ok_or(VerificationError::MissingClaim("issuer".to_string()))?;
    let sub = jwt
        .claims
        .sub
        .ok_or(VerificationError::MissingClaim("subject".to_string()))?;
    let nbf = jwt
        .claims
        .nbf
        .ok_or(VerificationError::MissingClaim("not_before".to_string()))?;
    let exp = jwt.claims.exp;

    let vc_payload = JwtPayloadVerifiableCredential::from_json_value(
        jwt.claims
            .additional_properties
            .ok_or(VerificationError::MissingClaim("vc".to_string()))?
            .get("vc")
            .ok_or(VerificationError::MissingClaim("vc".to_string()))?,
    )?;

    if let Some(id) = vc_payload.id {
        if id != jti {
            return Err(VerificationError::ClaimMismatch("id".to_string()).into());
        }
    }

    if let Some(issuer) = &vc_payload.issuer {
        let vc_issuer = issuer.to_string();
        if iss != vc_issuer {
            return Err(VerificationError::ClaimMismatch("issuer".to_string()).into());
        }
    }

    if let Some(credential_subject) = &vc_payload.credential_subject {
        if sub != credential_subject.id {
            return Err(VerificationError::ClaimMismatch("subject".to_string()).into());
        }
    }

    if let Some(vc_payload_expiration_date) = vc_payload.expiration_date {
        match exp {
            None => {
                return Err(VerificationError::MisconfiguredExpirationDate(
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
                        VerificationError::ClaimMismatch("expiration_date".to_string()).into(),
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
