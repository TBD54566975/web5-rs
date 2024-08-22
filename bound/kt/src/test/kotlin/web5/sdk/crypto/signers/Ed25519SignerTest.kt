package web5.sdk.crypto.signers

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.crypto.Ed25519Generator
import web5.sdk.rust.Web5Exception

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Ed25519SignerTest {

    private val testSuite = UnitTestSuite("ed25519_sign")

    @AfterAll
    fun verifyAllTestsIncluded() {
        if (testSuite.tests.isNotEmpty()) {
            println("The following tests were not included or executed:")
            testSuite.tests.forEach { println(it) }
            fail("Not all tests were executed! ${testSuite.tests}")
        }
    }

    @Test
    fun test_with_valid_key() {
        testSuite.include()

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
        testSuite.include()

        val jwk = Ed25519Generator.generate()
        val invalidJwk = jwk.copy(d = java.util.Base64.getUrlEncoder().withoutPadding().encodeToString(ByteArray(SECRET_KEY_LENGTH - 1)))

        val signer = Ed25519Signer(invalidJwk)
        val message = "Test message".toByteArray()
        val exception = assertThrows<Web5Exception.Exception> {
            signer.sign(message)
        }

        assertEquals("cryptography error invalid private key length ${SECRET_KEY_LENGTH - 1} must be $SECRET_KEY_LENGTH", exception.msg)
        assertEquals("Crypto", exception.variant)
    }

    @Test
    fun test_with_missing_private_key() {
        testSuite.include()

        val jwk = Ed25519Generator.generate()
        val missingKeyJwk = jwk.copy(d = null)

        val signer = Ed25519Signer(missingKeyJwk)
        val message = "Test message".toByteArray()
        val exception = assertThrows<Web5Exception.Exception> {
            signer.sign(message)
        }

        assertEquals("cryptography error private key material must be set", exception.msg)
        assertEquals("Crypto", exception.variant)
    }

    companion object {
        const val SIGNATURE_LENGTH = 64
        const val SECRET_KEY_LENGTH = 32
    }
}
