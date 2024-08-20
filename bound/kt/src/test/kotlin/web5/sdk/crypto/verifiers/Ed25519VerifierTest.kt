package web5.sdk.crypto.verifiers

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Assertions.assertFalse
import web5.sdk.crypto.signers.Ed25519Signer
import web5.sdk.rust.ed25519GeneratorGenerate as rustCoreEd25519GeneratorGenerate

class Ed25519VerifierTest {

    @Test
    fun `test verifier with valid signature`() {
        val privateJwk = rustCoreEd25519GeneratorGenerate()
        val ed25519Signer = Ed25519Signer(privateJwk)

        val message = "abc".toByteArray()
        val signature = ed25519Signer.sign(message)

        val ed25519Verifier = Ed25519Verifier(privateJwk)
        val isValid = ed25519Verifier.verify(message, signature)

        assertTrue(isValid, "Signature should be valid")
    }

    @Test
    fun `test verifier with invalid signature`() {
        val privateJwk = rustCoreEd25519GeneratorGenerate()
        val ed25519Signer = Ed25519Signer(privateJwk)

        val message = "abc".toByteArray()
        val signature = ed25519Signer.sign(message)

        val modifiedMessage = "abcd".toByteArray()

        val ed25519Verifier = Ed25519Verifier(privateJwk)
        val isValid = ed25519Verifier.verify(modifiedMessage, signature)

        assertFalse(isValid, "Signature should be invalid")
    }
}
