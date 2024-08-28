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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{credentials::decode::decode, test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_SUITE: UnitTestSuite =
            UnitTestSuite::new("verifiable_credential_1_1_data_model_validation");
    }

    #[test]
    fn test_validate_dm_empty_id() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_empty_id = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6IiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbGNrSlhOamhNWldaVVlsaERTRnByTlV4bk5VWXlTelJqUmtWaU1WaE1iVVZrVVU1bFNtUkpXamROSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozMDo1My44NDQ2ODMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MzA6NTMuODQ0NjMwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCIsImp0aSI6IiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTg1MywiaWF0IjoxNzI0ODUxODUzLCJleHAiOjE4ODIwOTk4NTN9.X_jkleLbhdAo0vm7KtN0qr6nR6hvWrXxk08UslfZAhZCkDN2kqLvWhoHps3GNznmGAuhJxwhZ0SN60OV7pp1DQ"#;

        let vc = decode(vc_jwt_with_empty_id, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing id")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_empty_context() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_empty_context = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjFSbXRGTVdkblNGcENaME4yY1doa1VqRkJZelZqUkd0Q2IybDFRMnhOYm5CTVVFNDRYM1ZOVjBRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6W10sImlkIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4IiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUoxUm10Rk1XZG5TRnBDWjBOMmNXaGtVakZCWXpWalJHdENiMmwxUTJ4TmJuQk1VRTQ0WDNWTlYwUXdJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjMyOjM1LjI2MzM1NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKMVJtdEZNV2RuU0ZwQ1owTjJjV2hrVWpGQll6VmpSR3RDYjJsMVEyeE5ibkJNVUU0NFgzVk5WMFF3SW4wIiwianRpIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxOTU1LCJpYXQiOjE3MjQ4NTE5NTV9.2GaazffucPj-LfdnO9OtMwij0PQK9crDC7rMMcwV9nt50Q3ACc1UtYCruMWsfYMc_CKfl5g7m6-zwDW8SpDzAw"#;

        let vc = decode(vc_jwt_with_empty_context, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing context")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_context_base_mismatch() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_without_base_context = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJhIGNvbnRleHQiXSwiaWQiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzQ6MjcuODk4MDkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpxUlVkZlNVeE5WQzFVVDBoMk4yMXhSelJSVlZkTE1rNXVPRXBpVFVOTlpXSkNaV1Y0ZFRsSGJYVlpJbjAiLCJqdGkiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIwNjcsImlhdCI6MTcyNDg1MjA2N30.cgkGQF5CXqHw_C1KNaKLFeIzPzmBuWRzRk-7KgvEYc1jJzwoXoOWB6cn-8I3MjWAgd_NeM1Yt656lJ60gy0RAQ"#;
        let vc = decode(vc_jwt_without_base_context, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing context")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_empty_type() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_empty_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTJSV2Q2T0RZeGExRjNVMTh3U25SWVVqQlJha3BtWldOemQyVkJiRWN3UzBadGMwZGxUa0ZoZVdwQkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmNmNjQ1MTNiLTIwODQtNDliNC1iNWMzLTgxZTk1ODNjOTcyOCIsInR5cGUiOltdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUkyUldkNk9EWXhhMUYzVTE4d1NuUllVakJSYWtwbVpXTnpkMlZCYkVjd1MwWnRjMGRsVGtGaGVXcEJJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjM1OjM2LjkxMzQyNyswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMlJXZDZPRFl4YTFGM1UxOHdTblJZVWpCUmFrcG1aV056ZDJWQmJFY3dTMFp0YzBkbFRrRmhlV3BCSW4wIiwianRpIjoidXJuOnV1aWQ6Y2Y2NDUxM2ItMjA4NC00OWI0LWI1YzMtODFlOTU4M2M5NzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUyMTM2LCJpYXQiOjE3MjQ4NTIxMzZ9.EY1q2nZHnPk-hnzdScvf6QYA0ko_sshHWOnPxU9tkU-RhxdklRoO9JQgmoHZC1FdDgEfgs4nDFNUKyX-FlJPBw"#;

        let vc = decode(vc_jwt_with_empty_type, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing type")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_type_base_mismatch() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_without_base_type = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmQ1NTdkODY3LWRlNTgtNDE3Ny1iZjE4LTM1ZjQ3NDA5NDlmMSIsInR5cGUiOlsiYSB0eXBlIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzY6MzUuNDgwMTkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpSUzI1clQzb3dkRE5KUVhkalVrRmpkMk4xZFVsSlJGbE9jR1pYZFhSb2NtUlRORFZLYnpkRVNXbHJJbjAiLCJqdGkiOiJ1cm46dXVpZDpkNTU3ZDg2Ny1kZTU4LTQxNzctYmYxOC0zNWY0NzQwOTQ5ZjEiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIxOTUsImlhdCI6MTcyNDg1MjE5NX0.S3vchUrNfdgXTQFeu7HcI5F0ZdkQdYkd4IqAXF8_uhcOe_sX9joDWspBSxwP3BY6ESCPIpJoms_dPIp01RWABA"#;

        let vc = decode(vc_jwt_without_base_type, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing type")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_empty_issuer() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_empty_issuer = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnRPSEZRTVhoR1p6RndZMlpqZUY5UWVrUnJOMjFPYVhoak9YQTFWamN4WlVZelYwRTViSGwzTWpsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozOToxNS42MjMzOTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6IiIsImp0aSI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjM1NSwiaWF0IjoxNzI0ODUyMzU1fQ.mRYZKF3qNz_Vyg8xpemuBOipGLOliYy9xJ6b9ZqcMNjZbb8GtEyiaBv8rgF2jqmHreRT71wHaT3P6mV9GsQOCA"#;

        let vc = decode(vc_jwt_with_empty_issuer, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing issuer")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_empty_subject() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_empty_subject = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKek5qRXpTWGhsYVZaT1YxbzVjMFozU2t0TU5IQjZaR1pGVGxZdFZURnFiQzB5U0hwWldXaEJRbFYzSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0MToxMC4xNzM2NzIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiIifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCIsImp0aSI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInN1YiI6IiIsIm5iZiI6MTcyNDg1MjQ3MCwiaWF0IjoxNzI0ODUyNDcwfQ.Ek9NMfHyb8BzJ7GnV0JRQPVl-UQyMOCMZ2_ABMx9Cvh8d8T81wMjrYUPp6v57-veqKntYFO_WZciL2FC_VZWAw"#;

        let vc = decode(vc_jwt_with_empty_subject, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "missing credential subject")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_issuance_in_future() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_issuance_date_in_future = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKaGFrMU9OazVmTWs5VFZtWXlMV0Z3VWw5bVVtZHBUbU4xU1UxemFVYzNNWFozYVhWcFVIZ3hhMU5OSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyOS0wOC0yMlQxMzo0Mjo1Ni43OTA2OTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCIsImp0aSI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTg4MjEwMDU3NiwiaWF0IjoxNzI0ODUyNTc2fQ.QM4LHyJ8wW1_A0PcuhpsorI3FOA9NLX9-u7a6MkAMXrQoxwNFIfHZeHuwLGVBshmco2emUievVAfKWUQFpOvBQ"#;

        let vc = decode(vc_jwt_with_issuance_date_in_future, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "issuance date in future")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }

    #[test]
    fn test_validate_dm_credential_expired() {
        TEST_SUITE.include(test_name!());

        let vc_jwt_with_expired = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNXeHFVMlpmWDNsT1NVWkpNVXBpY1hCRVJUQm5VRkZPYVVrMGJWRmpXak42ZG1kVVVuaDZZMDFuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0NDoyNy45MTUwMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMTktMDktMDRUMTM6NDQ6MjcuOTE0ODY0KzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCIsImp0aSI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjY2NywiaWF0IjoxNzI0ODUyNjY3LCJleHAiOjE1Njc2MDQ2Njd9.pP_8QVzTqxuhUlIWpXDWQ3Py_VlDA4uX82xdD9GOdmRT2UK-K5Gn7A5qdUxBPhXifiRVnH_Q8NbWZCUQ8jZUBg"#;

        let vc = decode(vc_jwt_with_expired, true).unwrap();
        let result = validate_vc_data_model(&vc);

        match result {
            Err(CredentialError::DataModelValidationError(msg)) => {
                assert_eq!(msg, "credential expired")
            }
            _ => panic!(
                "Expected CredentialError::DataModelValidationError, but got: {:?}",
                result
            ),
        };
    }
}
