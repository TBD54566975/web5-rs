package web5.sdk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import kotlin.math.sign

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

    @Test
    fun `can sign and verify`() {
        assertDoesNotThrow {
            val privateJwk = ed25519Generate()
            val payload = "hello world".toByteArray().map { it.toUByte() }
            val signature = ed25519Sign(privateJwk, payload)
            val publicJwk = privateJwk.toPublic().jwk()
            ed25519Verify(publicJwk, payload, signature.map { it.toUByte() })
        }
    }
}