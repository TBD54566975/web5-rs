package web5.sdk.core

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import web5.sdk.core.Jwk

class JwkTests {
    @Test
    fun `can compute thumbprint`() {
        val alg = "Ed25519"
        val kty = "OKP"
        val crv = "Ed25519"
        val d = "some-random-value-d"
        val x = "some-random-value-x"
        val y = "some-random-value-y"
        val jwk = Jwk(alg, kty, crv, d, x, y)

        assertEquals(alg, jwk.getAlg())
        assertEquals(kty, jwk.getKty())
        assertEquals(crv, jwk.getCrv())
        assertEquals(d, jwk.getD())
        assertEquals(x, jwk.getX())
        assertEquals(y, jwk.getY())

        assertDoesNotThrow {
            val thumbprint = jwk.computeThumbprint()
            assertNotEquals(0, thumbprint.length)
        }
    }
}