package web5.sdk.core

import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class VerifiableCredentialTests {
    @Test
    fun `can sign VC`() = runBlocking {
        val keyManager = LocalKeyManager.newInMemory()
        val bearerDid = createDidJwk(keyManager, DidJwkCreateOptions(Curve.ED25519))

        val verifiableCredential = VerifiableCredential(
            listOf("some context"),
            "some-id",
            listOf("some type"),
            "some-issuer",
            0,
            0,
            CredentialSubject("some-cred-sub-id", mapOf(Pair("something", "something else")))
        )

        val signedJwt = verifiableCredential.sign(bearerDid, KeySelector.MethodType(VerificationMethodType.VERIFICATION_METHOD))
        assertNotEquals(0, signedJwt.length)
    }
}