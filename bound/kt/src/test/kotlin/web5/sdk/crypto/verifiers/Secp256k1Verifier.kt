package web5.sdk.crypto.verifiers

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.crypto.Secp256k1Generator
import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Secp256k1Signer
import web5.sdk.Web5Exception
import java.util.Base64

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Secp256k1VerifierTest {

    private fun generateKeys(): Pair<Jwk, Jwk> {
        val privateJwk = Secp256k1Generator.generate()
        val publicJwk = privateJwk.copy(d = null)
        return Pair(publicJwk, privateJwk)
    }

    @Test
    fun test_with_valid_signature() {
        val (publicJwk, privateJwk) = generateKeys()
        val signer = Secp256k1Signer(privateJwk)
        val verifier = Secp256k1Verifier(publicJwk)

        val message = "Test message".toByteArray()
        val signature = signer.sign(message)

        val verifyResult = runCatching { verifier.verify(message, signature) }

        assertTrue(verifyResult.isSuccess, "Verification should succeed with a valid signature")
    }

    @Test
    fun test_with_private_key() {
        val (_, privateJwk) = generateKeys()
        val verifier = Secp256k1Verifier(privateJwk) // this is not allowed

        val message = "Test message".toByteArray()
        val invalidSignature = ByteArray(SIGNATURE_LENGTH) // Invalid length, but valid shape

        val exception = assertThrows<Web5Exception> {
            verifier.verify(message, invalidSignature)
        }

        assertEquals("cryptography error provided verification key cannot contain private key material", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_invalid_signature() {
        val (publicJwk, privateJwk) = generateKeys()
        val signer = Secp256k1Signer(privateJwk)
        val verifier = Secp256k1Verifier(publicJwk)

        val message = "Test message".toByteArray()

        // Create a valid signature and mutate the last byte
        val validSignature = signer.sign(message).toMutableList()
        validSignature[validSignature.size - 1] = (validSignature.last().toInt() xor 0x01).toByte() // Flip the last bit

        val exception = assertThrows<Web5Exception> {
            verifier.verify(message, validSignature.toByteArray())
        }

        assertEquals("cryptography error cryptographic verification failure", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_invalid_public_key() {
        val (publicJwk, privateJwk) = generateKeys()
        val invalidPublicJwk = publicJwk.copy(
            x = Base64.getUrlEncoder().withoutPadding().encodeToString(ByteArray(PUBLIC_KEY_LENGTH - 1))
        )

        val signer = Secp256k1Signer(privateJwk)
        val verifier = Secp256k1Verifier(invalidPublicJwk)

        val message = "Test message".toByteArray()
        val signature = signer.sign(message)

        val exception = assertThrows<Web5Exception> {
            verifier.verify(message, signature)
        }

        assertEquals("cryptography error unable to instantiate verifying key", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_invalid_signature_length() {
        val (publicJwk, _) = generateKeys()
        val verifier = Secp256k1Verifier(publicJwk)

        val message = "Test message".toByteArray()
        val invalidSignature = ByteArray(SIGNATURE_LENGTH - 1) // Invalid length

        val exception = assertThrows<Web5Exception> {
            verifier.verify(message, invalidSignature)
        }

        assertEquals("cryptography error invalid signature", exception.message)
        assertEquals("Crypto", exception.variant)
    }

    companion object {
        const val SIGNATURE_LENGTH = 64 // Secp256k1 signature length (r + s, each 32 bytes)
        const val PUBLIC_KEY_LENGTH = 32 // Secp256k1 public key length in bytes
    }
}