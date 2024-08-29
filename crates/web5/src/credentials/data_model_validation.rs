use std::time::SystemTime;

use super::{
    verifiable_credential_1_1::{VerifiableCredential, BASE_CONTEXT, BASE_TYPE},
    CredentialError,
};

pub fn validate_vc_data_model(
    vc: &VerifiableCredential,
) -> std::result::Result<(), CredentialError> {
    // Required fields ["@context", "id", "type", "issuer", "issuanceDate", "credentialSubject"]
    if vc.id.is_empty() {
        return Err(CredentialError::DataModelValidationError(
            "missing id".to_string(),
        ));
    }

    if vc.context.is_empty() || vc.context[0] != BASE_CONTEXT {
        return Err(CredentialError::DataModelValidationError(
            "missing context".to_string(),
        ));
    }

    if vc.r#type.is_empty() || vc.r#type[0] != BASE_TYPE {
        return Err(CredentialError::DataModelValidationError(
            "missing type".to_string(),
        ));
    }

    if vc.issuer.to_string().is_empty() {
        return Err(CredentialError::DataModelValidationError(
            "missing issuer".to_string(),
        ));
    }

    if vc.credential_subject.id.is_empty() {
        return Err(CredentialError::DataModelValidationError(
            "missing credential subject".to_string(),
        ));
    }

    let now = SystemTime::now();
    if vc.issuance_date > now {
        return Err(CredentialError::DataModelValidationError(
            "issuance date in future".to_string(),
        ));
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = &vc.expiration_date {
        if expiration_date < &now {
            return Err(CredentialError::DataModelValidationError(
                "credential expired".to_string(),
            ));
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
}
