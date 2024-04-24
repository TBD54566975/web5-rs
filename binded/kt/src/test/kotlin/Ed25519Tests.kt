package web5.sdk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class Ed25519Tests {
    @Test
    fun `can generate`() {
        assertDoesNotThrow {
            val jwk = ed25519Generate()
            assertEquals("EdDSA", jwk.getAlg())
            assertEquals("OKP", jwk.getKty())
            assertEquals("Ed25519", jwk.getCrv())
            assertNotNull(jwk.getD())
            assertNotEquals(0, jwk.getD()?.length)
            assertNotEquals(0, jwk.getX().length)
            assertNull(jwk.getY())
        }
    }
}