package web5.sdk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class LocalKeyManagerTests {
    @Test
    fun `can generate a Ed25519 key`() {
        val keyManager = LocalKeyManager.newInMemory()
        val keyAlias = keyManager.generatePrivateKey(Curve.ED25519, null)
        val publicKey = keyManager.getPublicKey(keyAlias)
        val jwk = publicKey.jwk()
        assertEquals("EdDSA", jwk.getAlg())
        assertEquals("OKP", jwk.getKty())
        assertEquals("Ed25519", jwk.getCrv())
    }

    @Test
    fun `can generate a secp256k1 key`() {
        val keyManager = LocalKeyManager.newInMemory()
        val keyAlias = keyManager.generatePrivateKey(Curve.SECP256K1, null)
        val publicKey = keyManager.getPublicKey(keyAlias)
        val jwk = publicKey.jwk()
        assertEquals("ES256K", jwk.getAlg())
        assertEquals("EC", jwk.getKty())
        assertEquals("secp256k1", jwk.getCrv())
    }

    @Test
    fun `can sign and verify`() {
        val keyManager = LocalKeyManager.newInMemory()
        val keyAlias = keyManager.generatePrivateKey(Curve.ED25519, null)

        val payload = "hello world".toByteArray().map { it.toUByte() }
        val signature = keyManager.sign(keyAlias, payload)
        assertNotEquals(0, signature.size)

        val publicKey = keyManager.getPublicKey(keyAlias)
        assertDoesNotThrow {
            publicKey.verify(payload, signature.map { it.toUByte() })
        }
    }

    @Test
    fun `can export and import keys`() {
        val keyManager = LocalKeyManager.newInMemory()
        keyManager.generatePrivateKey(Curve.SECP256K1, null)

        val exportedPrivateKeys = keyManager.exportPrivateKeys()
        assertEquals(1, exportedPrivateKeys.size)

        assertDoesNotThrow {
            keyManager.importPrivateKeys(exportedPrivateKeys)
        }
    }
}