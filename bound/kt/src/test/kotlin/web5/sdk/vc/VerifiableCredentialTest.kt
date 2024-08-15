package web5.sdk.vc

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertNull

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
}