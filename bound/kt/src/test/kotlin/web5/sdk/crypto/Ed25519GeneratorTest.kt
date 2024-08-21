package web5.sdk.crypto

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Ed25519GeneratorTest {

    private val testSuite = UnitTestSuite("ed25519_generate")

    @AfterAll
    fun verifyAllTestsIncluded() {
        if (testSuite.tests.isNotEmpty()) {
            println("The following tests were not included or executed:")
            testSuite.tests.forEach { println(it) }
            fail("Not all tests were executed! ${this.testSuite.tests}")
        }
    }

    @Test
    fun test_must_set_alg() {
        this.testSuite.include()

        val jwk = Ed25519Generator.generate()
        assertEquals("Ed25519", jwk.alg)
    }

    @Test
    fun test_must_set_kty() {
        this.testSuite.include()

        val jwk = Ed25519Generator.generate()
        assertEquals("OKP", jwk.kty)
    }

    @Test
    fun test_must_set_crv() {
        this.testSuite.include()

        val jwk = Ed25519Generator.generate()
        assertEquals("Ed25519", jwk.crv)
    }

    @Test
    fun test_must_set_public_key_with_correct_length() {
        this.testSuite.include()

        val jwk = Ed25519Generator.generate()
        val publicKeyBytes = java.util.Base64.getUrlDecoder().decode(jwk.x)
        assertEquals(PUBLIC_KEY_LENGTH, publicKeyBytes.size)
    }

    @Test
    fun test_must_set_private_key_with_correct_length() {
        this.testSuite.include()

        val jwk = Ed25519Generator.generate()
        val privateKeyBytes = jwk.d ?: fail("Private key is missing")
        val decodedPrivateKeyBytes = java.util.Base64.getUrlDecoder().decode(privateKeyBytes)
        assertEquals(SECRET_KEY_LENGTH, decodedPrivateKeyBytes.size)
    }

    companion object {
        const val PUBLIC_KEY_LENGTH = 32
        const val SECRET_KEY_LENGTH = 32
    }
}
