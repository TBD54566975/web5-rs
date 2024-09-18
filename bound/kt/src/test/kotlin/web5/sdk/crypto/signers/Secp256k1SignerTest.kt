package web5.sdk.crypto.signers

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.crypto.Secp256k1Generator
import web5.sdk.Web5Exception
import java.util.Base64

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Secp256k1SignerTest {

    @Test
    fun test_with_valid_key() {
        val jwk = Secp256k1Generator.generate()
        val signer = Secp256k1Signer(jwk)

        val message = "Test message".toByteArray()

        assertDoesNotThrow {
            val signature = signer.sign(message)
            assertEquals(SIGNATURE_LENGTH, signature.size, "Signature length should match the expected Secp256k1 signature length")
        }
    }

    @Test
    fun test_with_invalid_private_key() {
        val jwk = Secp256k1Generator.generate()
        val invalidJwk = jwk.copy(d = Base64.getUrlEncoder().withoutPadding().encodeToString(ByteArray(SECRET_KEY_LENGTH - 1)))

        val signer = Secp256k1Signer(invalidJwk)
        val message = "Test message".toByteArray()
        val exception = assertThrows<Web5Exception> {
            signer.sign(message)
        }

        assertEquals("cryptography error invalid private key", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_missing_private_key() {
        val jwk = Secp256k1Generator.generate()
        val missingKeyJwk = jwk.copy(d = null)

        val signer = Secp256k1Signer(missingKeyJwk)
        val message = "Test message".toByteArray()
        val exception = assertThrows<Web5Exception> {
            signer.sign(message)
        }

        assertEquals("cryptography error private key material must be set", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    companion object {
        const val SIGNATURE_LENGTH = 64  // Expected length for Secp256k1 signature (r + s, each 32 bytes)
        const val SECRET_KEY_LENGTH = 32 // Secp256k1 private key length in bytes
    }
}