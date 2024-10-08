package web5.sdk.crypto.signers

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.crypto.Ed25519Generator
import web5.sdk.Web5Exception

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Ed25519SignerTest {

    @Test
    fun test_with_valid_key() {

        val jwk = Ed25519Generator.generate()
        val signer = Ed25519Signer(jwk)

        val message = "Test message".toByteArray()

        assertDoesNotThrow {
            val signature = signer.sign(message)
            assertEquals(SIGNATURE_LENGTH, signature.size, "Signature length should match the expected Ed25519 signature length")
        }
    }

    @Test
    fun test_with_invalid_private_key() {

        val jwk = Ed25519Generator.generate()
        val invalidJwk = jwk.copy(d = java.util.Base64.getUrlEncoder().withoutPadding().encodeToString(ByteArray(SECRET_KEY_LENGTH - 1)))

        val signer = Ed25519Signer(invalidJwk)
        val message = "Test message".toByteArray()
        val exception = assertThrows<Web5Exception> {
            signer.sign(message)
        }

        assertEquals("cryptography error invalid private key length ${SECRET_KEY_LENGTH - 1} must be $SECRET_KEY_LENGTH", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_missing_private_key() {

        val jwk = Ed25519Generator.generate()
        val missingKeyJwk = jwk.copy(d = null)

        val signer = Ed25519Signer(missingKeyJwk)
        val message = "Test message".toByteArray()
        val exception = assertThrows<Web5Exception> {
            signer.sign(message)
        }

        assertEquals("cryptography error private key material must be set", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    companion object {
        const val SIGNATURE_LENGTH = 64
        const val SECRET_KEY_LENGTH = 32
    }
}
