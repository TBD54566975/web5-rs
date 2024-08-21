package web5.sdk.crypto.verifiers

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.crypto.Ed25519Generator
import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Ed25519Signer
import web5.sdk.rust.Web5Exception

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Ed25519VerifierTest {

    private val testSuite = UnitTestSuite("ed25519_verify")

    @AfterAll
    fun verifyAllTestsIncluded() {
        if (testSuite.tests.isNotEmpty()) {
            println("The following tests were not included or executed:")
            testSuite.tests.forEach { println(it) }
            fail("Not all tests were executed! ${this.testSuite.tests}")
        }
    }

    private fun generateKeys(): Pair<Jwk, Jwk> {
        val privateJwk = Ed25519Generator.generate()
        val publicJwk = privateJwk.copy(d = null)
        return Pair(publicJwk, privateJwk)
    }

    @Test
    fun test_with_valid_signature() {
        this.testSuite.include()

        val (publicJwk, privateJwk) = generateKeys()
        val signer = Ed25519Signer(privateJwk)
        val verifier = Ed25519Verifier(publicJwk)

        val message = "Test message".toByteArray()
        val signature = signer.sign(message)

        val verifyResult = runCatching { verifier.verify(message, signature) }

        assertTrue(verifyResult.isSuccess, "Verification should succeed with a valid signature")
    }

    @Test
    fun test_with_private_key() {
        this.testSuite.include()

        val (_, privateJwk) = generateKeys()
        val verifier = Ed25519Verifier(privateJwk) // this is not allowed

        val message = "Test message".toByteArray()
        val invalidSignature = ByteArray(SIGNATURE_LENGTH - 1) // invalid length

        val exception = assertThrows<Web5Exception.Exception> {
            verifier.verify(message, invalidSignature)
        }

        assertEquals("cryptography error provided verification key cannot contain private key material", exception.msg)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_invalid_signature() {
        this.testSuite.include()

        val (publicJwk, _) = generateKeys()
        val verifier = Ed25519Verifier(publicJwk)

        val message = "Test message".toByteArray()
        val invalidSignature = ByteArray(SIGNATURE_LENGTH) // an obviously invalid signature

        val exception = assertThrows<Web5Exception.Exception> {
            verifier.verify(message, invalidSignature)
        }

        assertEquals("cryptography error cryptographic verification failure", exception.msg)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_invalid_public_key() {
        this.testSuite.include()

        val (publicJwk, privateJwk) = generateKeys()
        val invalidPublicJwk = publicJwk.copy(x = java.util.Base64.getUrlEncoder().withoutPadding().encodeToString(ByteArray(PUBLIC_KEY_LENGTH - 1)))

        val signer = Ed25519Signer(privateJwk)
        val verifier = Ed25519Verifier(invalidPublicJwk)

        val message = "Test message".toByteArray()
        val signature = signer.sign(message)

        val exception = assertThrows<Web5Exception.Exception> {
            verifier.verify(message, signature)
        }

        assertEquals("cryptography error invalid public key length ${PUBLIC_KEY_LENGTH - 1} must be $PUBLIC_KEY_LENGTH", exception.msg)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_invalid_signature_length() {
        this.testSuite.include()

        val (publicJwk, _) = generateKeys()
        val verifier = Ed25519Verifier(publicJwk)

        val message = "Test message".toByteArray()
        val invalidSignature = ByteArray(SIGNATURE_LENGTH - 1) // invalid length

        val exception = assertThrows<Web5Exception.Exception> {
            verifier.verify(message, invalidSignature)
        }

        assertEquals("cryptography error invalid signature length ${SIGNATURE_LENGTH - 1} must be $SIGNATURE_LENGTH", exception.msg)
        assertEquals("Crypto", exception.variant)
    }

    companion object {
        const val SIGNATURE_LENGTH = 64
        const val PUBLIC_KEY_LENGTH = 32
    }
}
