package web5.sdk.vc

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.rust.Web5Exception
import java.util.Date
import java.util.regex.Pattern

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class VerifiableCredentialTest {
    companion object {
        const val ISSUER_DID_URI = "did:web:tbd.website"
        const val SUBJECT_DID_URI = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y"

        val ISSUER = Issuer.StringIssuer(ISSUER_DID_URI)
        val CREDENTIAL_SUBJECT = CredentialSubject(SUBJECT_DID_URI)
    }

    private val testSuite = UnitTestSuite("verifiable_credential_1_1_create")

    @AfterAll
    fun verifyAllTestsIncluded() {
        if (testSuite.tests.isNotEmpty()) {
            println("The following tests were not included or executed:")
            testSuite.tests.forEach { println(it) }
            fail("Not all tests were executed! ${this.testSuite.tests}")
        }
    }

    @Test
    fun test_default_context_added_if_not_supplied() {
        this.testSuite.include()
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT)
        assertEquals(listOf("https://www.w3.org/2018/credentials/v1"), vc.context)
    }

    @Test
    fun test_default_context_not_duplicated_if_supplied() {
        this.testSuite.include()
        val options = VerifiableCredentialCreateOptions(
            context = listOf("https://www.w3.org/2018/credentials/v1")
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("https://www.w3.org/2018/credentials/v1"), vc.context)
    }

    @Test
    fun test_developer_provided_context_appended_to_default() {
        this.testSuite.include()
        val customContext = "https://example.com/custom-context"
        val options = VerifiableCredentialCreateOptions(
            context = listOf(customContext)
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("https://www.w3.org/2018/credentials/v1", customContext), vc.context)
    }

    @Test
    fun test_default_type_added_if_not_supplied() {
        this.testSuite.include()
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(listOf("VerifiableCredential"), vc.type)
    }

    @Test
    fun test_default_type_not_duplicated_if_supplied() {
        this.testSuite.include()
        val options = VerifiableCredentialCreateOptions(
            type = listOf("VerifiableCredential")
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("VerifiableCredential"), vc.type)
    }

    @Test
    fun test_developer_provided_type_appended_to_default() {
        this.testSuite.include()
        val customType = "CustomType"
        val options = VerifiableCredentialCreateOptions(
            type = listOf(customType)
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("VerifiableCredential", customType), vc.type)
    }

    @Test
    fun test_id_generated_if_not_supplied() {
        this.testSuite.include()
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        val uuidPattern = Pattern.compile("^urn:uuid:[0-9a-fA-F-]{36}$")
        assertTrue(uuidPattern.matcher(vc.id).matches())
    }

    @Test
    fun test_id_must_be_set_if_supplied() {
        this.testSuite.include()
        val customId = "custom-id"
        val options = VerifiableCredentialCreateOptions(
            id = customId
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(customId, vc.id)
    }

    @Test
    fun test_issuer_string_must_not_be_empty() {
        this.testSuite.include()
        val emptyIssuer = Issuer.StringIssuer("")

        val exception = assertThrows<Web5Exception.Exception> {
            VerifiableCredential.create(emptyIssuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        }

        assertEquals("parameter error issuer id must not be empty", exception.msg)
    }

    @Test
    fun test_issuer_string_must_be_set() {
        this.testSuite.include()
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(ISSUER, vc.issuer)
    }

    @Test
    fun test_issuer_object_id_must_not_be_empty() {
        this.testSuite.include()
        val issuer = Issuer.ObjectIssuer("", "Example Name")

        val exception = assertThrows<Web5Exception.Exception> {
            VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        }

        assertEquals("parameter error issuer id must not be empty", exception.msg)
    }

    @Test
    fun test_issuer_object_name_must_not_be_empty() {
        this.testSuite.include()
        val issuer = Issuer.ObjectIssuer(ISSUER_DID_URI, "")

        val exception = assertThrows<Web5Exception.Exception> {
            VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        }

        assertEquals("parameter error named issuer name must not be empty", exception.msg)
    }

    @Test
    fun test_issuer_object_must_be_set() {
        this.testSuite.include()
        val issuer = Issuer.ObjectIssuer(ISSUER_DID_URI, "Example Name")

        val vc = VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(issuer, vc.issuer)
    }

    @Test
    fun test_issuer_object_supports_additional_properties() {
        this.testSuite.include()
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
        this.testSuite.include()
        val credentialSubject = CredentialSubject("")

        val exception = assertThrows<Web5Exception.Exception> {
            VerifiableCredential.create(ISSUER, credentialSubject, VerifiableCredentialCreateOptions())
        }

        assertEquals("parameter error subject id must not be empty", exception.msg)
    }

    @Test
    fun test_credential_subject_must_be_set() {
        this.testSuite.include()
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(CREDENTIAL_SUBJECT, vc.credentialSubject)
    }

    @Test
    fun test_credential_subject_supports_additional_properties() {
        this.testSuite.include()
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
        this.testSuite.include()
        val issuanceDate = Date()

        val options = VerifiableCredentialCreateOptions(
            issuanceDate = issuanceDate
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(issuanceDate, vc.issuanceDate)
    }

    @Test
    fun test_issuance_date_must_be_now_if_not_supplied() {
        this.testSuite.include()
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())

        val now = Date()
        val tenSecondsAgo = Date(now.time - 10000)
        val tenSecondsAhead = Date(now.time + 10000)

        assertTrue(vc.issuanceDate.after(tenSecondsAgo) && vc.issuanceDate.before(tenSecondsAhead))
    }

    @Test
    fun test_expiration_date_must_be_set_if_supplied() {
        this.testSuite.include()
        val expirationDate = Date()
        val options = VerifiableCredentialCreateOptions(
            expirationDate = expirationDate
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(expirationDate, vc.expirationDate)
    }
}
