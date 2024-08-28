package web5.sdk.vc

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.rust.Web5Exception
import java.util.Date
import java.util.regex.Pattern

class VerifiableCredentialTest {
    companion object {
        const val ISSUER_DID_URI = "did:web:tbd.website"
        const val SUBJECT_DID_URI = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y"

        val ISSUER = Issuer.StringIssuer(ISSUER_DID_URI)
        val CREDENTIAL_SUBJECT = CredentialSubject(SUBJECT_DID_URI)
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Create {
        private val testSuite = UnitTestSuite("verifiable_credential_1_1_create")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_default_context_added_if_not_supplied() {
            testSuite.include()
            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT)
            assertEquals(listOf("https://www.w3.org/2018/credentials/v1"), vc.context)
        }

        @Test
        fun test_default_context_not_duplicated_if_supplied() {
            testSuite.include()
            val options = VerifiableCredentialCreateOptions(
                context = listOf("https://www.w3.org/2018/credentials/v1")
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(listOf("https://www.w3.org/2018/credentials/v1"), vc.context)
        }

        @Test
        fun test_developer_provided_context_appended_to_default() {
            testSuite.include()
            val customContext = "https://example.com/custom-context"
            val options = VerifiableCredentialCreateOptions(
                context = listOf(customContext)
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(listOf("https://www.w3.org/2018/credentials/v1", customContext), vc.context)
        }

        @Test
        fun test_default_type_added_if_not_supplied() {
            testSuite.include()
            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            assertEquals(listOf("VerifiableCredential"), vc.type)
        }

        @Test
        fun test_default_type_not_duplicated_if_supplied() {
            testSuite.include()
            val options = VerifiableCredentialCreateOptions(
                type = listOf("VerifiableCredential")
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(listOf("VerifiableCredential"), vc.type)
        }

        @Test
        fun test_developer_provided_type_appended_to_default() {
            testSuite.include()
            val customType = "CustomType"
            val options = VerifiableCredentialCreateOptions(
                type = listOf(customType)
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(listOf("VerifiableCredential", customType), vc.type)
        }

        @Test
        fun test_id_generated_if_not_supplied() {
            testSuite.include()
            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            val uuidPattern = Pattern.compile("^urn:uuid:[0-9a-fA-F-]{36}$")
            assertTrue(uuidPattern.matcher(vc.id).matches())
        }

        @Test
        fun test_id_must_be_set_if_supplied() {
            testSuite.include()
            val customId = "custom-id"
            val options = VerifiableCredentialCreateOptions(
                id = customId
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(customId, vc.id)
        }

        @Test
        fun test_issuer_string_must_not_be_empty() {
            testSuite.include()
            val emptyIssuer = Issuer.StringIssuer("")

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.create(emptyIssuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            }

            assertEquals("parameter error issuer id must not be empty", exception.msg)
        }

        @Test
        fun test_issuer_string_must_be_set() {
            testSuite.include()
            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            assertEquals(ISSUER, vc.issuer)
        }

        @Test
        fun test_issuer_string_must_be_valid_did() {
            testSuite.include()

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.create(Issuer.StringIssuer("did:something-invalid"), CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            }

            assertEquals("parameter error issuer must be a valid DID URI", exception.msg)
        }

        @Test
        fun test_issuer_object_id_must_not_be_empty() {
            testSuite.include()
            val issuer = Issuer.ObjectIssuer("", "Example Name")

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            }

            assertEquals("parameter error issuer id must not be empty", exception.msg)
        }

        @Test
        fun test_issuer_object_id_must_be_valid_did() {
            testSuite.include()

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.create(Issuer.ObjectIssuer("did:something-invalid", "some name"), CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            }

            assertEquals("parameter error issuer must be a valid DID URI", exception.msg)
        }

        @Test
        fun test_issuer_object_name_must_not_be_empty() {
            testSuite.include()
            val issuer = Issuer.ObjectIssuer(ISSUER_DID_URI, "")

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            }

            assertEquals("parameter error named issuer name must not be empty", exception.msg)
        }

        @Test
        fun test_issuer_object_must_be_set() {
            testSuite.include()
            val issuer = Issuer.ObjectIssuer(ISSUER_DID_URI, "Example Name")

            val vc = VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            assertEquals(issuer, vc.issuer)
        }

        @Test
        fun test_issuer_object_supports_additional_properties() {
            testSuite.include()
            val additionalProperties = mapOf("extra_key" to "extra_value")

            val issuer = Issuer.ObjectIssuer(
                ISSUER_DID_URI,
                "Example Name",
                additionalProperties
            )

            val vc = VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())

            if (vc.issuer is Issuer.ObjectIssuer) {
                assertEquals(additionalProperties, (vc.issuer as Issuer.ObjectIssuer).additionalProperties)
            } else {
                fail("Issuer is not an ObjectIssuer")
            }
        }

        @Test
        fun test_credential_subject_id_must_not_be_empty() {
            testSuite.include()
            val credentialSubject = CredentialSubject("")

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.create(ISSUER, credentialSubject, VerifiableCredentialCreateOptions())
            }

            assertEquals("parameter error subject id must not be empty", exception.msg)
        }

        @Test
        fun test_credential_subject_must_be_set() {
            testSuite.include()
            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            assertEquals(CREDENTIAL_SUBJECT, vc.credentialSubject)
        }

        @Test
        fun test_credential_subject_supports_additional_properties() {
            testSuite.include()
            val additionalProperties = mapOf("extra_key" to "extra_value")

            val credentialSubject = CredentialSubject(
                SUBJECT_DID_URI,
                additionalProperties
            )

            val vc = VerifiableCredential.create(ISSUER, credentialSubject, VerifiableCredentialCreateOptions())
            assertEquals(additionalProperties, vc.credentialSubject.additionalProperties)
        }

        @Test
        fun test_issuance_date_must_be_set() {
            testSuite.include()
            val issuanceDate = Date()

            val options = VerifiableCredentialCreateOptions(
                issuanceDate = issuanceDate
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(issuanceDate, vc.issuanceDate)
        }

        @Test
        fun test_issuance_date_must_be_now_if_not_supplied() {
            testSuite.include()
            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())

            val now = Date()
            val tenSecondsAgo = Date(now.time - 10000)
            val tenSecondsAhead = Date(now.time + 10000)

            assertTrue(vc.issuanceDate.after(tenSecondsAgo) && vc.issuanceDate.before(tenSecondsAhead))
        }

        @Test
        fun test_expiration_date_must_be_set_if_supplied() {
            testSuite.include()
            val expirationDate = Date()
            val options = VerifiableCredentialCreateOptions(
                expirationDate = expirationDate
            )

            val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
            assertEquals(expirationDate, vc.expirationDate)
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class FromVcJwt {
        private val testSuite = UnitTestSuite("verifiable_credential_1_1_from_vc_jwt")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_missing_kid_jose_header() {
            testSuite.include()

            val vcJwtWithMissingKid = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmEzYzY3NGI5LTliNGUtNGE2OS1hYzUwLWM3N2JhYzY0OTg2MCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKclZ6YzNjbnA2VTNGUExYSkxUekpIY1hwd04zRk9TRGhEYWtORmJFMHpabmhMUmtWcVdFTmtaRWxWSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yN1QyMDo0NzoyNS4xMTk2MjQrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJWemMzY25wNlUzRlBMWEpMVHpKSGNYcHdOM0ZPU0RoRGFrTkZiRTB6Wm5oTFJrVnFXRU5rWkVsVkluMCIsImp0aSI6InVybjp1dWlkOmEzYzY3NGI5LTliNGUtNGE2OS1hYzUwLWM3N2JhYzY0OTg2MCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDc5MTY0NSwiaWF0IjoxNzI0NzkxNjQ1fQ.ocOyYhqFwz4Jvkdwpa69oFDXCOr2n-_IXSHg5elFebOM0T_lx3Cs6DgQJ7YLLk--mAOvPqrH05bh92BSaLB_DQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingKid, true)
            }

            assertEquals("missing kid jose header", exception.msg)
        }

        @Test
        fun test_empty_kid_jose_header() {
            testSuite.include()

            val vcJwtWithEmptyKid = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IiJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmE5MzcwYTZjLWFmNDAtNDU3Zi1iNDNiLWM0YmYzMzcwZTg1OSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKNE5XODFYM3BHTTE5a1QzaDBlbFZzTUVjNGREQnZkVlUyZFRsVFFVdEpiVkZXZFRaa1lsSlpiMUJqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yN1QyMDozODo0Ni45MjcxMzYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjROVzgxWDNwR00xOWtUM2gwZWxWc01FYzRkREJ2ZFZVMmRUbFRRVXRKYlZGV2RUWmtZbEpaYjFCakluMCIsImp0aSI6InVybjp1dWlkOmE5MzcwYTZjLWFmNDAtNDU3Zi1iNDNiLWM0YmYzMzcwZTg1OSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDc5MTEyNiwiaWF0IjoxNzI0NzkxMTI2fQ.0LzNrPzFY4CsEWRqYdo8pogGDonZqjRqfx9k30NEoWASw8pas6YC-mlDSAQ-4qQaE-otQ6p7zoMeopfw9M1CCQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithEmptyKid, true)
            }

            assertEquals("missing kid jose header", exception.msg)
        }

        @Test
        fun test_kid_invalid_did_uri() {
            testSuite.include()

            val vcJwtWithInvalidDidUri = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImludmFsaWQgZGlkIHVyaSJ9.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFhYzFmN2M5LTIzOWQtNGE4OC05NDBiLTEwOTk3NmViNWYyNCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiaW52YWxpZCBkaWQgdXJpIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMjozMDowOC41OTAxOTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImludmFsaWQgZGlkIHVyaSIsImp0aSI6InVybjp1dWlkOmFhYzFmN2M5LTIzOWQtNGE4OC05NDBiLTEwOTk3NmViNWYyNCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg0ODIwOCwiaWF0IjoxNzI0ODQ4MjA4fQ.YdmnfP0wIK5HDu8Lft52UFdZCzfdFO0rclAOF-mWt6Y1vqAgoyuOn7AnX1Lx782-iWaekKApCGqCTaXepzj4CQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithInvalidDidUri, true)
            }

            assertEquals("parameter error identifier regex match failure invalid did uri", exception.msg)
        }

        @Test
        fun test_kid_fail_to_resolve_did() {
            testSuite.include()

            val vcJwtWithInvalidDidResolution = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6c29tZXRoaW5nLWludmFsaWQifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjhiNGM1NmI5LTM1ODgtNGM0Mi1iOTg3LWEwZTAxNDFmNzA2YSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDpzb21ldGhpbmctaW52YWxpZCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTI6MzQ6NDguMzMzMjg5KzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6ZGh0OnNvbWV0aGluZy1pbnZhbGlkIiwianRpIjoidXJuOnV1aWQ6OGI0YzU2YjktMzU4OC00YzQyLWI5ODctYTBlMDE0MWY3MDZhIiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODQ4NDg4LCJpYXQiOjE3MjQ4NDg0ODh9.hXbWLVU8ef38O5SY-HshVhXPM1RadFEAGRj0ds5Yjw1_lweWxe1-CNJxLmo0D4BiRCo4T4hCWP_bkwRoteImBA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithInvalidDidResolution, true)
            }

            assertEquals("The DID Document does not have a valid public key.", exception.msg)
        }

        @Test
        fun test_kid_missing_verification_method() {
            testSuite.include()

            val vcJwtWithMissingVerificationMethod = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmZNblZ0WDJFM2EwMU1OM1p0TXpGdlRWbGhjVE5WWWpGTWRWbzBhazFUTjNaT2NsTndlbWxvVWpkWkluMCMwLWludmFsaWQifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmM5ZjUzNTY0LTdkMjYtNGE1NS1iN2E4LTk2MTU4ZTBhNWVhNSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKZk1uVnRYMkUzYTAxTU4zWnRNekZ2VFZsaGNUTlZZakZNZFZvMGFrMVROM1pPY2xOd2VtbG9VamRaSW4wIzAtaW52YWxpZCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTI6NDA6NDIuMjk2Njc4KzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpmTW5WdFgyRTNhMDFNTjNadE16RnZUVmxoY1ROVllqRk1kVm8wYWsxVE4zWk9jbE53ZW1sb1VqZFpJbjAjMC1pbnZhbGlkIiwianRpIjoidXJuOnV1aWQ6YzlmNTM1NjQtN2QyNi00YTU1LWI3YTgtOTYxNThlMGE1ZWE1Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODQ4ODQyLCJpYXQiOjE3MjQ4NDg4NDJ9.g-KcBy9jJ87PvIZkBUDPkBVF-dlnSTsLUVxOxB4az5q64aIDFJNTffVETD3Cq0fjXKX3tZq3QpfzmNoiTo4xBQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingVerificationMethod, true)
            }

            assertEquals("not found error verification method not found", exception.msg)
        }

        @Test
        fun test_fails_cryptographic_verification() {
            testSuite.include()

            val vcJwtWithInvalidSignature = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhWelpGVERsSVRUbHRkSGx5Y0dsWWRGRlVNR3B4Wms1MmFXTm5RVGxCVkRnME1IWTFZMDh5YjFSckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjZmYTQ2MDVjLWFlZGItNGQ2NC05NzdiLTFmY2NmYTU1ZTM1ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEyOjQyOjI3Ljc3Mjg4OSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJqdGkiOiJ1cm46dXVpZDo2ZmE0NjA1Yy1hZWRiLTRkNjQtOTc3Yi0xZmNjZmE1NWUzNWYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NDg5NDcsImlhdCI6MTcyNDg0ODk0N30.-JwIGYZ9HlJASYxdRBWY5KlwP0iJUxWUOU6BsOR74VeC-zKgZb9WWZR08OVD-wv0X8KD5--0K5Dr9r5fL3B0Aw-invalid-signature"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithInvalidSignature, true)
            }

            assertTrue(exception.msg.contains("cryptography error vc-jwt failed cryptographic verification"))
        }

        @Test
        fun test_passes_cryptographic_verification() {
            testSuite.include()

            val vcJwtValid = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhWelpGVERsSVRUbHRkSGx5Y0dsWWRGRlVNR3B4Wms1MmFXTm5RVGxCVkRnME1IWTFZMDh5YjFSckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjZmYTQ2MDVjLWFlZGItNGQ2NC05NzdiLTFmY2NmYTU1ZTM1ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEyOjQyOjI3Ljc3Mjg4OSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJqdGkiOiJ1cm46dXVpZDo2ZmE0NjA1Yy1hZWRiLTRkNjQtOTc3Yi0xZmNjZmE1NWUzNWYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NDg5NDcsImlhdCI6MTcyNDg0ODk0N30.-JwIGYZ9HlJASYxdRBWY5KlwP0iJUxWUOU6BsOR74VeC-zKgZb9WWZR08OVD-wv0X8KD5--0K5Dr9r5fL3B0Aw"

            val vc = VerifiableCredential.fromVcJwt(vcJwtValid, true)
            assertEquals(Issuer.StringIssuer("did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJHVzZFTDlITTltdHlycGlYdFFUMGpxZk52aWNnQTlBVDg0MHY1Y08yb1RrIn0#0"), vc.issuer)
        }

        @Test
        fun test_can_skip_cryptographic_verification() {
            testSuite.include()

            val vcJwtWithInvalidSignature = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhWelpGVERsSVRUbHRkSGx5Y0dsWWRGRlVNR3B4Wms1MmFXTm5RVGxCVkRnME1IWTFZMDh5YjFSckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjZmYTQ2MDVjLWFlZGItNGQ2NC05NzdiLTFmY2NmYTU1ZTM1ZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEyOjQyOjI3Ljc3Mjg4OSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFZ6WkZURGxJVFRsdGRIbHljR2xZZEZGVU1HcHhaazUyYVdOblFUbEJWRGcwTUhZMVkwOHliMVJySW4wIzAiLCJqdGkiOiJ1cm46dXVpZDo2ZmE0NjA1Yy1hZWRiLTRkNjQtOTc3Yi0xZmNjZmE1NWUzNWYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NDg5NDcsImlhdCI6MTcyNDg0ODk0N30.-JwIGYZ9HlJASYxdRBWY5KlwP0iJUxWUOU6BsOR74VeC-zKgZb9WWZR08OVD-wv0X8KD5--0K5Dr9r5fL3B0Aw-invalid-signature"

            val vc = VerifiableCredential.fromVcJwt(vcJwtWithInvalidSignature, false)
            assertEquals("urn:uuid:6fa4605c-aedb-4d64-977b-1fccfa55e35f", vc.id)
        }

        @Test
        fun test_can_skip_data_model_validation() {
            testSuite.include()

            // expired would throw an error, but since verify=false it doesn't
            val vcJwtWithExpired = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNXeHFVMlpmWDNsT1NVWkpNVXBpY1hCRVJUQm5VRkZPYVVrMGJWRmpXak42ZG1kVVVuaDZZMDFuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0NDoyNy45MTUwMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMTktMDktMDRUMTM6NDQ6MjcuOTE0ODY0KzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCIsImp0aSI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjY2NywiaWF0IjoxNzI0ODUyNjY3LCJleHAiOjE1Njc2MDQ2Njd9.pP_8QVzTqxuhUlIWpXDWQ3Py_VlDA4uX82xdD9GOdmRT2UK-K5Gn7A5qdUxBPhXifiRVnH_Q8NbWZCUQ8jZUBg"

            val vc = VerifiableCredential.fromVcJwt(vcJwtWithExpired, false)

            assertEquals("urn:uuid:ad70f7f6-a157-4f1f-af29-7cbbd44f89f0", vc.id)
        }

        @Test
        fun test_issuer_string() {
            testSuite.include()

            val vcJwtWithIssuerAsString = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnlkMmhYU1VOWWNsSjNiMFphUm1SMU0wbHNOaTFCTkdVdGRqazNRbE14UmtaUmFWRTRhV05tV2t0ckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjc0NTY5ZmIzLWMyZTktNGZiMy1hOThkLWY3NGFjNzVjYTg5NSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKeWQyaFhTVU5ZY2xKM2IwWmFSbVIxTTBsc05pMUJOR1V0ZGprM1FsTXhSa1pSYVZFNGFXTm1Xa3RySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxNjozNjoyOS4zNDc4ODArMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnlkMmhYU1VOWWNsSjNiMFphUm1SMU0wbHNOaTFCTkdVdGRqazNRbE14UmtaUmFWRTRhV05tV2t0ckluMCIsImp0aSI6InVybjp1dWlkOjc0NTY5ZmIzLWMyZTktNGZiMy1hOThkLWY3NGFjNzVjYTg5NSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg2Mjk4OSwiaWF0IjoxNzI0ODYyOTg5fQ.0DSZ2XbPtjtrtxNKo3tImoByb1-jlQxZQN11lsngaFSe4lhy4mYmaxGAby4wIl-c_cLEkgBULfF3Qa_dlNSTCw"

            val vc = VerifiableCredential.fromVcJwt(vcJwtWithIssuerAsString, false)

            when (vc.issuer) {
                is Issuer.StringIssuer -> {
                    assertEquals(
                        "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJyd2hXSUNYclJ3b0ZaRmR1M0lsNi1BNGUtdjk3QlMxRkZRaVE4aWNmWktrIn0",
                        (vc.issuer as Issuer.StringIssuer).value
                    )
                }
                is Issuer.ObjectIssuer -> fail("issuer should be string")
            }
        }

        @Test
        fun test_issuer_object() {
            testSuite.include()

            val vcJwtWithIssuerObject = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTFVazF5YVVNMVZsaHVielpTVkRoTVdWVnJibnBKWm5OamFUUXlZbXhCYVdsTFdrcENaR2huVm5WQkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjcwNWM0MTZiLTU1ODYtNDUzMS1hMmRmLWI3YzdhNTMxMGY5NiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMVVrMXlhVU0xVmxodWJ6WlNWRGhNV1ZWcmJucEpabk5qYVRReVlteEJhV2xMV2twQ1pHaG5WblZCSW4wIiwibmFtZSI6InNvbWUgbmFtZSJ9LCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDE2OjQwOjExLjUwNDIyMCswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMVVrMXlhVU0xVmxodWJ6WlNWRGhNV1ZWcmJucEpabk5qYVRReVlteEJhV2xMV2twQ1pHaG5WblZCSW4wIiwianRpIjoidXJuOnV1aWQ6NzA1YzQxNmItNTU4Ni00NTMxLWEyZGYtYjdjN2E1MzEwZjk2Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODYzMjExLCJpYXQiOjE3MjQ4NjMyMTF9.Mv-wlUcnj0w-OWuoMBCciaQXrAogXL3qqgZnthTRI9f55S5PidYiSapWFxFqc4SzxTVSpe64H2vF7kfGU-QpBw"

            val vc = VerifiableCredential.fromVcJwt(vcJwtWithIssuerObject, false)

            when (vc.issuer) {
                is Issuer.ObjectIssuer -> {
                    assertEquals(
                        "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiI1Uk1yaUM1VlhubzZSVDhMWVVrbnpJZnNjaTQyYmxBaWlLWkpCZGhnVnVBIn0",
                        (vc.issuer as Issuer.ObjectIssuer).id
                    )
                    assertEquals("some name", (vc.issuer as Issuer.ObjectIssuer).name)
                }
                is Issuer.StringIssuer -> fail("issuer should be object")
            }
        }

        @Test
        fun test_missing_vc_claim() {
            testSuite.include()

            val vcJwtWithMissingVcClaim = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlNSbkZSVlVWS1RFOVhlbXh3T1ZaRk1rdEtSalp6UjBwT00yVnpaWHBsY0hSSE0ySTFlbTh4YjAwNEluMCMwIn0.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpTUm5GUlZVVktURTlYZW14d09WWkZNa3RLUmpaelIwcE9NMlZ6WlhwbGNIUkhNMkkxZW04eGIwMDRJbjAiLCJqdGkiOiJ1cm46dXVpZDozNmU0ZjllNi0yYzdjLTQ0NGMtOTI4OS0zNDhmY2IxNDZlYjYiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTA1MjIsImlhdCI6MTcyNDg1MDUyMn0.SqwZC0q9RuHp9hAtFmE6sBYeJ1uHuuq1hyijF0NmW9nksSBqtDpfNroNlitK_Tl-CLWtwbTpK3b3JduTfzGEAw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingVcClaim, true)
            }

            assertEquals("missing claim: vc", exception.msg)
        }

        @Test
        fun test_missing_jti_claim() {
            testSuite.include()

            val vcJwtWithMissingJtiClaim = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm5jMjlTZGsxUFlXMHliMlJQTlY4NWVqbExlV2xzV1VzM1Yzb3RZa1owWW5wdlVrWm1iVTlUTVRJNEluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjEwODM2MzgwLWI2MmMtNGVmZC04YmU0LTZhNzJiMDZjYWI4NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbmMyOVNkazFQWVcweWIyUlBOVjg1ZWpsTGVXbHNXVXMzVjNvdFlrWjBZbnB2VWtabWJVOVRNVEk0SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMDo1NS4yMDYwOTIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSm5jMjlTZGsxUFlXMHliMlJQTlY4NWVqbExlV2xzV1VzM1Yzb3RZa1owWW5wdlVrWm1iVTlUTVRJNEluMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MDY1NSwiaWF0IjoxNzI0ODUwNjU1fQ.1XDmdvB1GDsCHw9Qwp0HA5r8W-JnZB4lz9Yqo0C2V_EEe-uk88bQSl8P9HV8ViNyBC_YaYatLiPTD4jBZY77DA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingJtiClaim, true)
            }

            assertEquals("missing claim: jti", exception.msg)
        }

        @Test
        fun test_missing_issuer_claim() {
            testSuite.include()

            val vcJwtWithMissingIssuerClaim = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjamREVWtVek1HbzNjVVU0Y2taVVJYQXdSbFJzYnpKVVVXVmlZa1ZHTVVvelJHaHRTVWhaVTNFd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRjYzU0NWU0LWI5ZDgtNDdkNS04Zjk0LTA4MmM0ZGViNzAyZCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNqZERVa1V6TUdvM2NVVTRja1pVUlhBd1JsUnNiekpVVVdWaVlrVkdNVW96UkdodFNVaFpVM0V3SW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMTo1Mi4zMjg4MTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImp0aSI6InVybjp1dWlkOjRjYzU0NWU0LWI5ZDgtNDdkNS04Zjk0LTA4MmM0ZGViNzAyZCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MDcxMiwiaWF0IjoxNzI0ODUwNzEyfQ.hwR6edt6ItlN0HHkDcxzhE3N5hLk-5-VYDLrqkalUoTKB41vsfaPvGnt_UQK3EAuekQgrTQ0SuCq-6ut0EdlBw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingIssuerClaim, true)
            }

            assertEquals("missing claim: issuer", exception.msg)
        }

        @Test
        fun test_missing_subject_claim() {
            testSuite.include()

            val vcJwtWithMissingSubjectClaim = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSldiRFprTjFFMWRXOTNSMVk1TWxsRlVWSkxOMnROWkdRM1lYcFJiMGxsU0hac1FXaFNSMVJmTlRJMEluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjBmYTE0MTgxLTllMWYtNDk0ZC05ZmVmLWMwYjgxZDE1ZGJiYiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKV2JEWmtOMUUxZFc5M1IxWTVNbGxGVVZKTE4ydE5aR1EzWVhwUmIwbGxTSFpzUVdoU1IxUmZOVEkwSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMjo0NS40NTg4MjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSldiRFprTjFFMWRXOTNSMVk1TWxsRlVWSkxOMnROWkdRM1lYcFJiMGxsU0hac1FXaFNSMVJmTlRJMEluMCIsImp0aSI6InVybjp1dWlkOjBmYTE0MTgxLTllMWYtNDk0ZC05ZmVmLWMwYjgxZDE1ZGJiYiIsIm5iZiI6MTcyNDg1MDc2NSwiaWF0IjoxNzI0ODUwNzY1fQ.61IFQhdASbbcYKUzMfhO7WPmikBd8AoE468FTlqRysxXck7kNa3bAAow3jK2uhYrIWLyRu3kuBp7JyYhLavjBw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingSubjectClaim, true)
            }

            assertEquals("missing claim: subject", exception.msg)
        }

        @Test
        fun test_missing_nbf_claim() {
            testSuite.include()

            val vcJwtWithMissingNbfClaim = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdOR1ZzZGxGdlJWbDBZbEJIT0RsWlVtaGpTR2RJT1cwMlMzSjZiRVkyUWpGUldrZGxOR2RGUjJKakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjk3OGZhZTIxLTVmMDYtNDBmNy1iZTJmLTM4MzRmZGMwZDY0NSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd05HVnNkbEZ2UlZsMFlsQkhPRGxaVW1oalNHZElPVzAyUzNKNmJFWTJRakZSV2tkbE5HZEZSMkpqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxMzoyNi4zMzQzNjYrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdOR1ZzZGxGdlJWbDBZbEJIT0RsWlVtaGpTR2RJT1cwMlMzSjZiRVkyUWpGUldrZGxOR2RGUjJKakluMCIsImp0aSI6InVybjp1dWlkOjk3OGZhZTIxLTVmMDYtNDBmNy1iZTJmLTM4MzRmZGMwZDY0NSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsImlhdCI6MTcyNDg1MDgwNn0.ZXfuZmvddH1nvmub8WDpQ2UEOhuiLaN6WL2q3XDhn0eouM_bNVa7vmCUCUZc3sfJ1YCtnAGCJOlJxSGnD3tOCw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMissingNbfClaim, true)
            }

            assertEquals("missing claim: not_before", exception.msg)
        }

        @Test
        fun test_claim_mismatch_id() {
            testSuite.include()

            val vcJwtWithMismatchId = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnZTWHBSUjJkTmNGTlNPSEpRWTNkd1IxZEJTRnBaV0hwUFdYRlRiMFkyTWtoM09HTlJRamRJUzIxM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InNvbWV0aGluZyBpbnZhbGlkIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUp2U1hwUlIyZE5jRk5TT0hKUVkzZHdSMWRCU0ZwWldIcFBXWEZUYjBZMk1raDNPR05SUWpkSVMyMTNJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjE2OjAwLjcyMjgxOSswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKdlNYcFJSMmROY0ZOU09ISlFZM2R3UjFkQlNGcFpXSHBQV1hGVGIwWTJNa2gzT0dOUlFqZElTMjEzSW4wIiwianRpIjoidXJuOnV1aWQ6ZGFkM2Y2MjktMzFiMS00NDcxLWFhYTMtMWE4MGZjN2I1YmU2Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUwOTYwLCJpYXQiOjE3MjQ4NTA5NjB9.P8-Z3KsMxIk7-Dz9a5odVhbGJZtWsWp4mDVYLlVxuZTNJl-Km-j2S1KusTjRTDkg1DqQoiVvp2Is0kr5WoAFBA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMismatchId, true)
            }

            assertEquals("claim mismatch: id", exception.msg)
        }

        @Test
        fun test_claim_mismatch_issuer() {
            testSuite.include()

            val vcJwtWithMismatchIssuer = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXpWRVZsYWs0emIzSXpUbXR4WkZWVllYQjZaMVZ5TFcxblZFTkNkWEZRWVZkT1JWcE9lRXcwWkhRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiNzQzNWY0LWU0YjctNGQyZC1iN2M2LTVkOTE5ODRlNDlhOCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoic29tZXRoaW5nIGludmFsaWQiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjE3OjQ1LjI4ODk2NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJelZFVmxhazR6YjNJelRtdHhaRlZWWVhCNloxVnlMVzFuVkVOQ2RYRlFZVmRPUlZwT2VFdzBaSFF3SW4wIiwianRpIjoidXJuOnV1aWQ6MmI3NDM1ZjQtZTRiNy00ZDJkLWI3YzYtNWQ5MTk4NGU0OWE4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxMDY1LCJpYXQiOjE3MjQ4NTEwNjV9.x0UY38J4lEwmrXR4qrzhnk58btjZfMf8DVhdgBoj9M0JOgJqCDFCzwcS5weVCpNAv3gN72Qo32RH9Tx0eYyoDA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMismatchIssuer, true)
            }

            assertEquals("claim mismatch: issuer", exception.msg)
        }

        @Test
        fun test_claim_mismatch_subject() {
            testSuite.include()

            val vcJwtWithMismatchSubject = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSXdVRmh0UkVNMlNIWnVia1E0Vmw5QkxWbDVSelZ1TWtSa2IxQkdTVFkxY2tkb2MwVTVZWFZsWW5CckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjAwNDJiYTQ4LWU0ZGYtNGVhMS04ZmJjLWJjYmI4ODY3ZjFhMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd1VGaHRSRU0yU0hadWJrUTRWbDlCTFZsNVJ6VnVNa1JrYjFCR1NUWTFja2RvYzBVNVlYVmxZbkJySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoxOToxMC4xNjM0ODkrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJzb21ldGhpbmcgaW52YWxpZCJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJd1VGaHRSRU0yU0hadWJrUTRWbDlCTFZsNVJ6VnVNa1JrYjFCR1NUWTFja2RvYzBVNVlYVmxZbkJySW4wIiwianRpIjoidXJuOnV1aWQ6MDA0MmJhNDgtZTRkZi00ZWExLThmYmMtYmNiYjg4NjdmMWEwIiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxMTUwLCJpYXQiOjE3MjQ4NTExNTB9.bAm9kKJX2-Rcw679VS7cUPbqg9awuq5Lwu9wiZoGcE0TCSc59rQTIP4nvxlP22o3V-VVs_DbfpJU-qB4duDSCA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMismatchSubject, true)
            }

            assertEquals("claim mismatch: subject", exception.msg)
        }

        @Test
        fun test_claim_misconfigured_exp() {
            testSuite.include()

            val vcJwtWithMisconfiguredExp = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJkWFI2V21WM01EVTBMVlUwUVRBM2FsYzJZbkkxUlV4NU1UQlpOSGxPVTFCaVkyOTNXakJ3TjJWakluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjYxZjgwM2I4LWUxMDQtNDdhOC04YWE1LTk4YzQ1ZTFiOGUzMSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKcmRYUjZXbVYzTURVMExWVTBRVEEzYWxjMlluSTFSVXg1TVRCWk5IbE9VMUJpWTI5M1dqQndOMlZqSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoyMzo0My45NDg4MzQrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MjM6NDMuOTQ4NzYwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnJkWFI2V21WM01EVTBMVlUwUVRBM2FsYzJZbkkxUlV4NU1UQlpOSGxPVTFCaVkyOTNXakJ3TjJWakluMCIsImp0aSI6InVybjp1dWlkOjYxZjgwM2I4LWUxMDQtNDdhOC04YWE1LTk4YzQ1ZTFiOGUzMSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTQyMywiaWF0IjoxNzI0ODUxNDIzfQ.AWYyvLRISXwLH5gAXb5CcwBXNwaRKwacGqstXjnk-xIHx9gmm5xj8zGONvcKE2Xx0t9j3pNHicrhkp5wcOkABQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMisconfiguredExp, true)
            }

            assertEquals("misconfigured expiration date: VC has expiration date but no exp in registered claims", exception.msg)
        }

        @Test
        fun test_claim_mismatch_exp() {
            testSuite.include()

            val vcJwtWithMismatchExp = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSk1lWEJmUjJVelVEZGtjbVZhYTJSV1VsTnJZbmROVldkcVkxUTRhMHd6VUVVMk1Hc3pZMGgzVTJ0ckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRhMjA2YmMzLWZmOTYtNDMwNS1iMzM4LTJiZGQ1ODRiYzkyOSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKTWVYQmZSMlV6VURka2NtVmFhMlJXVWxOclluZE5WV2RxWTFRNGEwd3pVRVUyTUdzelkwaDNVMnRySW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzoyNzozMy40Mjg1NjMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6Mjc6MzMuNDI4NDgyKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSk1lWEJmUjJVelVEZGtjbVZhYTJSV1VsTnJZbmROVldkcVkxUTRhMHd6VUVVMk1Hc3pZMGgzVTJ0ckluMCIsImp0aSI6InVybjp1dWlkOjRhMjA2YmMzLWZmOTYtNDMwNS1iMzM4LTJiZGQ1ODRiYzkyOSIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTY1MywiaWF0IjoxNzI0ODUxNjUzLCJleHAiOjE4ODUxMjM2NTN9.lAaTG8RhL2D92iNI6psZrv1uhtHYAO0m0AacGIQrW0XIThg-Livef36_CN9t4Lz2Ta5US2Be2VP6D3lCA-z1DQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithMismatchExp, true)
            }

            assertEquals("claim mismatch: expiration_date", exception.msg)
        }

        @Test
        fun test_validate_dm_empty_id() {
            testSuite.include()

            val vcJwtWithEmptyId = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6IiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKbGNrSlhOamhNWldaVVlsaERTRnByTlV4bk5VWXlTelJqUmtWaU1WaE1iVVZrVVU1bFNtUkpXamROSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozMDo1My44NDQ2ODMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMjktMDgtMjJUMTM6MzA6NTMuODQ0NjMwKzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmxja0pYTmpoTVpXWlVZbGhEU0Zwck5VeG5OVVl5U3pSalJrVmlNVmhNYlVWa1VVNWxTbVJKV2pkTkluMCIsImp0aSI6IiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MTg1MywiaWF0IjoxNzI0ODUxODUzLCJleHAiOjE4ODIwOTk4NTN9.X_jkleLbhdAo0vm7KtN0qr6nR6hvWrXxk08UslfZAhZCkDN2kqLvWhoHps3GNznmGAuhJxwhZ0SN60OV7pp1DQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithEmptyId, true)
            }

            assertEquals("data model validation error: missing id", exception.msg)
        }

        @Test
        fun test_validate_dm_empty_context() {
            testSuite.include()

            val vcJwtWithEmptyContext = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSjFSbXRGTVdkblNGcENaME4yY1doa1VqRkJZelZqUkd0Q2IybDFRMnhOYm5CTVVFNDRYM1ZOVjBRd0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6W10sImlkIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4IiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUoxUm10Rk1XZG5TRnBDWjBOMmNXaGtVakZCWXpWalJHdENiMmwxUTJ4TmJuQk1VRTQ0WDNWTlYwUXdJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjMyOjM1LjI2MzM1NiswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKMVJtdEZNV2RuU0ZwQ1owTjJjV2hrVWpGQll6VmpSR3RDYjJsMVEyeE5ibkJNVUU0NFgzVk5WMFF3SW4wIiwianRpIjoidXJuOnV1aWQ6ODVmM2MzNWUtYTI5Yi00YmQ2LTk1MmMtNzhlYWJiOTIzNzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUxOTU1LCJpYXQiOjE3MjQ4NTE5NTV9.2GaazffucPj-LfdnO9OtMwij0PQK9crDC7rMMcwV9nt50Q3ACc1UtYCruMWsfYMc_CKfl5g7m6-zwDW8SpDzAw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithEmptyContext, true)
            }

            assertEquals("data model validation error: missing context", exception.msg)
        }

        @Test
        fun test_validate_dm_context_base_mismatch() {
            testSuite.include()

            val vcJwtWithoutBaseContext = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJhIGNvbnRleHQiXSwiaWQiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnFSVWRmU1V4TlZDMVVUMGgyTjIxeFJ6UlJWVmRMTWs1dU9FcGlUVU5OWldKQ1pXVjRkVGxIYlhWWkluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzQ6MjcuODk4MDkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpxUlVkZlNVeE5WQzFVVDBoMk4yMXhSelJSVlZkTE1rNXVPRXBpVFVOTlpXSkNaV1Y0ZFRsSGJYVlpJbjAiLCJqdGkiOiJ1cm46dXVpZDo4N2VmMDI1MC0yYWE2LTQyNTctYjIxMi0xYzAyMWFhZDY2Y2YiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIwNjcsImlhdCI6MTcyNDg1MjA2N30.cgkGQF5CXqHw_C1KNaKLFeIzPzmBuWRzRk-7KgvEYc1jJzwoXoOWB6cn-8I3MjWAgd_NeM1Yt656lJ60gy0RAQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithoutBaseContext, true)
            }

            assertEquals("data model validation error: missing context", exception.msg)
        }

        @Test
        fun test_validate_dm_empty_type() {
            testSuite.include()

            val vcJwtWithEmptyType = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSTJSV2Q2T0RZeGExRjNVMTh3U25SWVVqQlJha3BtWldOemQyVkJiRWN3UzBadGMwZGxUa0ZoZVdwQkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmNmNjQ1MTNiLTIwODQtNDliNC1iNWMzLTgxZTk1ODNjOTcyOCIsInR5cGUiOltdLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUkyUldkNk9EWXhhMUYzVTE4d1NuUllVakJSYWtwbVpXTnpkMlZCYkVjd1MwWnRjMGRsVGtGaGVXcEJJbjAiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA4LTI4VDEzOjM1OjM2LjkxMzQyNyswMDowMCIsImV4cGlyYXRpb25EYXRlIjpudWxsLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSJ9fSwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lJMlJXZDZPRFl4YTFGM1UxOHdTblJZVWpCUmFrcG1aV056ZDJWQmJFY3dTMFp0YzBkbFRrRmhlV3BCSW4wIiwianRpIjoidXJuOnV1aWQ6Y2Y2NDUxM2ItMjA4NC00OWI0LWI1YzMtODFlOTU4M2M5NzI4Iiwic3ViIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5IiwibmJmIjoxNzI0ODUyMTM2LCJpYXQiOjE3MjQ4NTIxMzZ9.EY1q2nZHnPk-hnzdScvf6QYA0ko_sshHWOnPxU9tkU-RhxdklRoO9JQgmoHZC1FdDgEfgs4nDFNUKyX-FlJPBw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithEmptyType, true)
            }

            assertEquals("data model validation error: missing type", exception.msg)
        }

        @Test
        fun test_validate_dm_type_base_mismatch() {
            testSuite.include()

            val vcJwtWithoutBaseType = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmQ1NTdkODY3LWRlNTgtNDE3Ny1iZjE4LTM1ZjQ3NDA5NDlmMSIsInR5cGUiOlsiYSB0eXBlIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlJTMjVyVDNvd2RETkpRWGRqVWtGamQyTjFkVWxKUkZsT2NHWlhkWFJvY21SVE5EVktiemRFU1dsckluMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjQtMDgtMjhUMTM6MzY6MzUuNDgwMTkwKzAwOjAwIiwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmRodDpxZ21tcHlqdzVod25xZmd6bjd3bXJtMzNhZHk4Z2I4ejlpZGVpYjZtOWdqNHlzNndueTh5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpSUzI1clQzb3dkRE5KUVhkalVrRmpkMk4xZFVsSlJGbE9jR1pYZFhSb2NtUlRORFZLYnpkRVNXbHJJbjAiLCJqdGkiOiJ1cm46dXVpZDpkNTU3ZDg2Ny1kZTU4LTQxNzctYmYxOC0zNWY0NzQwOTQ5ZjEiLCJzdWIiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkiLCJuYmYiOjE3MjQ4NTIxOTUsImlhdCI6MTcyNDg1MjE5NX0.S3vchUrNfdgXTQFeu7HcI5F0ZdkQdYkd4IqAXF8_uhcOe_sX9joDWspBSxwP3BY6ESCPIpJoms_dPIp01RWABA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithoutBaseType, true)
            }

            assertEquals("data model validation error: missing type", exception.msg)
        }

        @Test
        fun test_validate_dm_empty_issuer() {
            testSuite.include()

            val vcJwtWithEmptyIssuer = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnRPSEZRTVhoR1p6RndZMlpqZUY5UWVrUnJOMjFPYVhoak9YQTFWamN4WlVZelYwRTViSGwzTWpsckluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzozOToxNS42MjMzOTMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6IiIsImp0aSI6InVybjp1dWlkOmFhMmNjNWNkLTg4N2QtNDFkMi1iZTM3LTIzMjMxMGVkODdjMiIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjM1NSwiaWF0IjoxNzI0ODUyMzU1fQ.mRYZKF3qNz_Vyg8xpemuBOipGLOliYy9xJ6b9ZqcMNjZbb8GtEyiaBv8rgF2jqmHreRT71wHaT3P6mV9GsQOCA"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithEmptyIssuer, true)
            }

            assertEquals("data model validation error: missing issuer", exception.msg)
        }

        @Test
        fun test_validate_dm_empty_subject() {
            testSuite.include()

            val vcJwtWithEmptySubject = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKek5qRXpTWGhsYVZaT1YxbzVjMFozU2t0TU5IQjZaR1pGVGxZdFZURnFiQzB5U0hwWldXaEJRbFYzSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0MToxMC4xNzM2NzIrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiIifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpOakV6U1hobGFWWk9WMW81YzBaM1NrdE1OSEI2WkdaRlRsWXRWVEZxYkMweVNIcFpXV2hCUWxWM0luMCIsImp0aSI6InVybjp1dWlkOjNkNGMzMjQxLWU0NDUtNGE2Ny1hYmE0LTIxYjBmM2NkMmMxYyIsInN1YiI6IiIsIm5iZiI6MTcyNDg1MjQ3MCwiaWF0IjoxNzI0ODUyNDcwfQ.Ek9NMfHyb8BzJ7GnV0JRQPVl-UQyMOCMZ2_ABMx9Cvh8d8T81wMjrYUPp6v57-veqKntYFO_WZciL2FC_VZWAw"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithEmptySubject, true)
            }

            assertEquals("data model validation error: missing credential subject", exception.msg)
        }

        @Test
        fun test_validate_dm_issuance_in_future() {
            testSuite.include()

            val vcJwtWithIssuanceDateInFuture = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKaGFrMU9OazVmTWs5VFZtWXlMV0Z3VWw5bVVtZHBUbU4xU1UxemFVYzNNWFozYVhWcFVIZ3hhMU5OSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyOS0wOC0yMlQxMzo0Mjo1Ni43OTA2OTcrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSmhhazFPTms1Zk1rOVRWbVl5TFdGd1VsOW1VbWRwVG1OMVNVMXphVWMzTVhaM2FYVnBVSGd4YTFOTkluMCIsImp0aSI6InVybjp1dWlkOjJiODhmMDhmLTVkOGItNDJiYS1iYmY0LTg4MjU1MjlmOGE2NyIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTg4MjEwMDU3NiwiaWF0IjoxNzI0ODUyNTc2fQ.QM4LHyJ8wW1_A0PcuhpsorI3FOA9NLX9-u7a6MkAMXrQoxwNFIfHZeHuwLGVBshmco2emUievVAfKWUQFpOvBQ"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithIssuanceDateInFuture, true)
            }

            assertEquals("data model validation error: issuance date in future", exception.msg)
        }

        @Test
        fun test_validate_dm_credential_expired() {
            testSuite.include()

            val vcJwtWithExpired = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKemNXeHFVMlpmWDNsT1NVWkpNVXBpY1hCRVJUQm5VRkZPYVVrMGJWRmpXak42ZG1kVVVuaDZZMDFuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOC0yOFQxMzo0NDoyNy45MTUwMjUrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6IjIwMTktMDktMDRUMTM6NDQ6MjcuOTE0ODY0KzAwOjAwIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0OnFnbW1weWp3NWh3bnFmZ3puN3dtcm0zM2FkeThnYjh6OWlkZWliNm05Z2o0eXM2d255OHkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSnpjV3hxVTJaZlgzbE9TVVpKTVVwaWNYQkVSVEJuVUZGT2FVazBiVkZqV2pONmRtZFVVbmg2WTAxbkluMCIsImp0aSI6InVybjp1dWlkOmFkNzBmN2Y2LWExNTctNGYxZi1hZjI5LTdjYmJkNDRmODlmMCIsInN1YiI6ImRpZDpkaHQ6cWdtbXB5anc1aHducWZnem43d21ybTMzYWR5OGdiOHo5aWRlaWI2bTlnajR5czZ3bnk4eSIsIm5iZiI6MTcyNDg1MjY2NywiaWF0IjoxNzI0ODUyNjY3LCJleHAiOjE1Njc2MDQ2Njd9.pP_8QVzTqxuhUlIWpXDWQ3Py_VlDA4uX82xdD9GOdmRT2UK-K5Gn7A5qdUxBPhXifiRVnH_Q8NbWZCUQ8jZUBg"

            val exception = assertThrows<Web5Exception.Exception> {
                VerifiableCredential.fromVcJwt(vcJwtWithExpired, true)
            }

            assertEquals("data model validation error: credential expired", exception.msg)
        }
    }
}
