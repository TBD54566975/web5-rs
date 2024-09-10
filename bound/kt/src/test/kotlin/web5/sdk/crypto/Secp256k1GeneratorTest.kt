package web5.sdk.crypto

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import java.util.Base64
import web5.sdk.UnitTestSuite

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Secp256k1GeneratorTest {

    private val testSuite = UnitTestSuite("secp256k1_generate")

    @AfterAll
    fun verifyAllTestsIncluded() {
        if (testSuite.tests.isNotEmpty()) {
            println("The following tests were not included or executed:")
            testSuite.tests.forEach { println(it) }
            fail("Not all tests were executed! ${testSuite.tests}")
        }
    }

    @Test
    fun test_must_set_alg() {
        testSuite.include()

        val jwk = Secp256k1Generator.generate()
        assertEquals("ES256K", jwk.alg)
    }

    @Test
    fun test_must_set_kty() {
        testSuite.include()

        val jwk = Secp256k1Generator.generate()
        assertEquals("EC", jwk.kty)
    }

    @Test
    fun test_must_set_crv() {
        testSuite.include()

        val jwk = Secp256k1Generator.generate()
        assertEquals("secp256k1", jwk.crv)
    }

    @Test
    fun test_must_set_public_key_with_correct_length() {
        testSuite.include()

        val jwk = Secp256k1Generator.generate()
        val xBytes = Base64.getUrlDecoder().decode(jwk.x)
        val yBytes = jwk.y?.let { Base64.getUrlDecoder().decode(it) } ?: fail("y coordinate is missing")
        assertEquals(32, xBytes.size)
        assertEquals(32, yBytes.size)
    }

    @Test
    fun test_must_set_private_key_with_correct_length() {
        testSuite.include()

        val jwk = Secp256k1Generator.generate()
        val privateKeyBytes = jwk.d ?: fail("Private key is missing")
        val decodedPrivateKeyBytes = Base64.getUrlDecoder().decode(privateKeyBytes)
        assertEquals(32, decodedPrivateKeyBytes.size)
    }
}