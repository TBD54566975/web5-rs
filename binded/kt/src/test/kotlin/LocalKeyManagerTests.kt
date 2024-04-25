package web5.sdk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
//import web5.sdk.LocalKeyManager
import web5.sdk.helloWorld
import web5.sdk.newJwk

class LocalKeyManagerTests {
    @Test
    fun test() {
        helloWorld()
    }

    @Test
    fun jwk() {
        val jwk = newJwk("Ed25519", "OKP")
        println(jwk)
        println(jwk.getAlg())

        val thumbprint = jwk.computeThumbprint()
        println(thumbprint)

        val jwk2 = Jwk("Ed25519", "OKP")
        println(jwk2)
        println(jwk2.getAlg())
        println(jwk2.computeThumbprint())
    }


//    @Test
//    fun `can generate a Ed25519 key`() {
//        val keyManager = LocalKeyManager.newInMemory()
//        val keyAlias = keyManager.generatePrivateKey(Curve.ED25519, null)
//        assertNotEquals(0, keyAlias.length)
//    }
//
//    @Test
//    fun `can generate a secp256k1 key`() {
//        val keyManager = LocalKeyManager.newInMemory()
//        val keyAlias = keyManager.generatePrivateKey(Curve.SECP256K1, null)
//        assertNotEquals(0, keyAlias.length)
//    }
//
//    @Test
//    fun `can sign and verify`() {
//        val keyManager = LocalKeyManager.newInMemory()
//        val keyAlias = keyManager.generatePrivateKey(Curve.ED25519, null)
//        assertNotEquals(0, keyAlias.length)
//
//        val payload = "hello world".toByteArray().map { it.toUByte() }
//        val signature = keyManager.sign(keyAlias, payload)
//        assertNotEquals(0, signature.size)
//
//        val publicKey = keyManager.getPublicKey(keyAlias)
//        assertDoesNotThrow {
//            publicKey.verify(payload, signature.map { it.toUByte() })
//        }
//    }
//
//    @Test
//    fun `can export and import keys`() {
//        val keyManager = LocalKeyManager.newInMemory()
//        keyManager.generatePrivateKey(Curve.SECP256K1, null)
//
//        val exportedPrivateKeys = keyManager.exportPrivateKeys()
//        assertEquals(1, exportedPrivateKeys.size)
//
//        assertDoesNotThrow {
//            keyManager.importPrivateKeys(exportedPrivateKeys)
//        }
//    }
}