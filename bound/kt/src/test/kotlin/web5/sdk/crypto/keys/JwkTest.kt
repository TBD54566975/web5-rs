package web5.sdk.crypto.keys

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.Web5Exception

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class JwkTest {

    private val testSuite = UnitTestSuite("jwk_compute_thumbprint")

    @AfterAll
    fun verifyAllTestsIncluded() {
        if (testSuite.tests.isNotEmpty()) {
            println("The following tests were not included or executed:")
            testSuite.tests.forEach { println(it) }
            fail("Not all tests were executed! ${testSuite.tests}")
        }
    }

    @Test
    fun test_ec_valid() {
        testSuite.include()
        val jwk = Jwk(
            kty = "EC",
            crv = "secp256k1",
            x = "x_value",
            y = "y_value"
        )

        val thumbprint = jwk.computeThumbprint()
        assertEquals("yiiszVT5Lwt6760MW19cHaJ61qJKIfe20sUW8dNxBv4", thumbprint)
    }

    @Test
    fun test_okp_valid() {
        testSuite.include()
        val jwk = Jwk(
            kty = "OKP",
            crv = "Ed25519",
            x = "x_value"
        )

        val thumbprint = jwk.computeThumbprint()
        assertEquals("nDMRVZm4lpedGjuJGO4y3YVJJ0krDF0aSz4KhlncDdI", thumbprint)
    }

    @Test
    fun test_unsupported_kty() {
        testSuite.include()
        val jwk = Jwk(
            kty = "RSA",
            crv = "RS256",
            x = "x_value",
            y = "y_value"
        )

        val exception = assertThrows<Web5Exception> {
            jwk.computeThumbprint()
        }

        assertEquals("data member error kty not supported RSA", exception.message)
    }

    @Test
    fun test_empty_kty() {
        testSuite.include()
        val jwk = Jwk(
            kty = "",
            crv = "Ed25519",
            x = "x_value"
        )

        val exception = assertThrows<Web5Exception> {
            jwk.computeThumbprint()
        }

        assertEquals("data member error kty cannot be empty", exception.message)
    }

    @Test
    fun test_empty_x() {
        testSuite.include()
        val jwk = Jwk(
            kty = "OKP",
            crv = "Ed25519",
            x = ""
        )

        val exception = assertThrows<Web5Exception> {
            jwk.computeThumbprint()
        }

        assertEquals("data member error x cannot be empty", exception.message)
    }

    @Test
    fun test_empty_crv() {
        testSuite.include()
        val jwk = Jwk(
            kty = "EC",
            crv = "",
            x = "x_value",
            y = "y_value"
        )

        val exception = assertThrows<Web5Exception> {
            jwk.computeThumbprint()
        }

        assertEquals("data member error crv cannot be empty", exception.message)
    }

    @Test
    fun test_ec_missing_y() {
        testSuite.include()
        val jwk = Jwk(
            kty = "EC",
            crv = "P-256",
            x = "x_value"
        )

        val exception = assertThrows<Web5Exception> {
            jwk.computeThumbprint()
        }

        assertEquals("data member error missing y", exception.message)
    }

    @Test
    fun test_ec_empty_y() {
        testSuite.include()
        val jwk = Jwk(
            kty = "EC",
            crv = "P-256",
            x = "x_value",
            y = ""
        )

        val exception = assertThrows<Web5Exception> {
            jwk.computeThumbprint()
        }

        assertEquals("data member error y cannot be empty", exception.message)
    }
}
