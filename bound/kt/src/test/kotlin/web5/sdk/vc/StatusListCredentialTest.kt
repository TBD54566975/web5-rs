package web5.sdk.vc

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*


import web5.sdk.rust.SystemTargetTest
import web5.sdk.rust.UniffiLib

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class StatusListCredentialTest {
    companion object {
        const val ISSUER_DID_URI = "did:web:tbd.website"
        const val SUBJECT_DID_URI = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y"

        val ISSUER = Issuer.StringIssuer(ISSUER_DID_URI)
        val CREDENTIAL_SUBJECT = CredentialSubject(SUBJECT_DID_URI)
    }

    @Test
    fun test_create_status_list_credential() {
        val statusPurpose = "revocation"

        val optionsVc1 = VerifiableCredentialCreateOptions(
            credentialStatus = CredentialStatus("vc-cred-status-id-1", "StatusList2021Entry", statusPurpose, "123", "status-list-credential-id"),
        )
        val vc1 = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, optionsVc1)

        val optionsVc2 = VerifiableCredentialCreateOptions(
            credentialStatus = CredentialStatus("vc-cred-status-id-2", "StatusList2021Entry", statusPurpose, "9999", "status-list-credential-id"),
        )
        val vc2 = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, optionsVc2)

        val optionsVc3 = VerifiableCredentialCreateOptions(
            credentialStatus = CredentialStatus("vc-cred-status-id-3", "StatusList2021Entry", statusPurpose, "876", "status-list-credential-id"),
        )
        val vc3 = VerifiableCredential.create(ISSUER, CREDENTIAL_SUBJECT, optionsVc3)

        val statusListCredential = StatusListCredential.create(ISSUER, statusPurpose, listOf(vc1, vc2))

        assertTrue(statusListCredential.isDisabled(vc1))
        assertTrue(statusListCredential.isDisabled(vc2))
        assertFalse(statusListCredential.isDisabled(vc3))
    }
}