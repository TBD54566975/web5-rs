package web5.sdk.vc

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
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

    @Test
    fun testDefaultContextAddedIfNotSupplied() {
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT)
        assertEquals(listOf("https://www.w3.org/2018/credentials/v1"), vc.context)
    }

    @Test
    fun testDefaultContextNotDuplicatedIfSupplied() {
        val options = VerifiableCredentialCreateOptions(
            context = listOf("https://www.w3.org/2018/credentials/v1")
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("https://www.w3.org/2018/credentials/v1"), vc.context)
    }

    @Test
    fun testDeveloperProvidedContextAppendedToDefault() {
        val customContext = "https://example.com/custom-context"
        val options = VerifiableCredentialCreateOptions(
            context = listOf(customContext)
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("https://www.w3.org/2018/credentials/v1", customContext), vc.context)
    }

    @Test
    fun testDefaultTypeAddedIfNotSupplied() {
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(listOf("VerifiableCredential"), vc.type)
    }

    @Test
    fun testDefaultTypeNotDuplicatedIfSupplied() {
        val options = VerifiableCredentialCreateOptions(
            type = listOf("VerifiableCredential")
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("VerifiableCredential"), vc.type)
    }

    @Test
    fun testDeveloperProvidedTypeAppendedToDefault() {
        val customType = "CustomType"
        val options = VerifiableCredentialCreateOptions(
            type = listOf(customType)
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(listOf("VerifiableCredential", customType), vc.type)
    }

    @Test
    fun testIdGeneratedIfNotSupplied() {
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        val uuidPattern = Pattern.compile("^urn:uuid:[0-9a-fA-F-]{36}$")
        assertTrue(uuidPattern.matcher(vc.id).matches())
    }

    @Test
    fun testIdMustBeSetIfSupplied() {
        val customId = "custom-id"
        val options = VerifiableCredentialCreateOptions(
            id = customId
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(customId, vc.id)
    }

    @Test
    fun testIssuerStringMustNotBeEmpty() {
        val emptyIssuer = Issuer.StringIssuer("")
        try {
            VerifiableCredential.create(emptyIssuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            fail("Expected an exception to be thrown")
        } catch (e: Web5Exception) {
            when (e) {
                is Web5Exception.Exception -> {
                    assertEquals(e.msg, "parameter error issuer id must not be empty")
                }
                else -> {
                    fail("Caught an unexpected exception type")
                }
            }
        }
    }

    @Test
    fun testIssuerStringMustBeSet() {
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(ISSUER, vc.issuer)
    }

    @Test
    fun testIssuerObjectIdMustNotBeEmpty() {
        val issuer = Issuer.ObjectIssuer("", "Example Name")

        try {
            VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            fail("Expected an exception to be thrown")
        } catch (e: Web5Exception) {
            when (e) {
                is Web5Exception.Exception -> {
                    assertEquals(e.msg, "parameter error issuer id must not be empty")
                }
                else -> {
                    fail("Caught an unexpected exception type")
                }
            }
        }
    }

    @Test
    fun testIssuerObjectNameMustNotBeEmpty() {
        val issuer = Issuer.ObjectIssuer(ISSUER_DID_URI, "")

        try {
            VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
            fail("Expected an exception to be thrown")
        } catch (e: Web5Exception) {
            when (e) {
                is Web5Exception.Exception -> {
                    assertEquals(e.msg, "parameter error named issuer name must not be empty")
                }
                else -> {
                    fail("Caught an unexpected exception type")
                }
            }
        }
    }

    @Test
    fun testIssuerObjectMustBeSet() {
        val issuer = Issuer.ObjectIssuer(ISSUER_DID_URI, "Example Name")

        val vc = VerifiableCredential.create(issuer, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(issuer, vc.issuer)
    }

    @Test
    fun testIssuerObjectSupportsAdditionalProperties() {
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
    fun testCredentialSubjectIdMustNotBeEmpty() {
        val credentialSubject = CredentialSubject("")

        try {
            VerifiableCredential.create(ISSUER, credentialSubject, VerifiableCredentialCreateOptions())
            fail("Expected an exception to be thrown")
        } catch (e: Web5Exception) {
            when (e) {
                is Web5Exception.Exception -> {
                    assertEquals(e.msg, "parameter error subject id must not be empty")
                }
                else -> {
                    fail("Caught an unexpected exception type")
                }
            }
        }
    }

    @Test
    fun testCredentialSubjectMustBeSet() {
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())
        assertEquals(CREDENTIAL_SUBJECT, vc.credentialSubject)
    }

    @Test
    fun testCredentialSubjectSupportsAdditionalProperties() {
        val additionalProperties = mapOf("extra_key" to "extra_value")

        val credentialSubject = CredentialSubject(
            SUBJECT_DID_URI,
            additionalProperties
        )

        val vc = VerifiableCredential.create(ISSUER, credentialSubject, VerifiableCredentialCreateOptions())
        assertEquals(additionalProperties, vc.credentialSubject.additionalProperties)
    }

    @Test
    fun testIssuanceDateMustBeSet() {
        val issuanceDate = Date()

        val options = VerifiableCredentialCreateOptions(
            issuanceDate = issuanceDate
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(issuanceDate, vc.issuanceDate)
    }

    @Test
    fun testIssuanceDateMustBeNowIfNotSupplied() {
        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, VerifiableCredentialCreateOptions())

        val now = Date()
        val oneSecondAgo = Date(now.time - 1000)

        assertTrue(vc.issuanceDate.after(oneSecondAgo) && vc.issuanceDate.before(now))
    }

    @Test
    fun testExpirationDateMustBeSetIfSupplied() {
        val expirationDate = Date()
        val options = VerifiableCredentialCreateOptions(
            expirationDate = expirationDate
        )

        val vc = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, options)
        assertEquals(expirationDate, vc.expirationDate)
    }
}