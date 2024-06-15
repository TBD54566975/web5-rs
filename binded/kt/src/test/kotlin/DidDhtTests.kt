package web5.sdk

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertEquals
//import web5.sdk.DidDht
//import web5.sdk.InMemoryKeyManager
import web5.sdk.testErr
import web5.sdk.UniFfiJwkException
import web5.sdk.UniffiWeb5Exception

class DidDhtTests {
    @Test
    fun `test JWK error details`() {
        val exception = assertThrows(UniffiWeb5Exception::class.java) {
            testJwkErr()
        }
        assertEquals("thumbprint computation failed testing inner string", exception.message())
        assertEquals("web5::apid::jwk::JwkError", exception.errorType())
        assertEquals("ThumbprintFailed", exception.errorVariant())
    }

    @Test
    fun `can do things`() {
//        val keyManager = InMemoryKeyManager()
//        val jwkData = keyManager.generateKeyMaterial()
//
//        val didDht = DidDht.fromIdentityKey(jwkData)
//        val signer = keyManager.getSigner(jwkData)
//        didDht.publish(signer)
//        didDht.deactivate(signer)
        val exception = assertThrows(UniFfiJwkException::class.java) {
            testErr()
        }
        println(exception)
        println(exception.message())
        println(exception.message)
        assert(exception.message?.contains("testing inner string") == true)
    }

    @Test
    fun `test Key Manager error details`() {
        val exception = assertThrows(UniffiWeb5Exception::class.java) {
            testKeyManagerErr()
        }
        assertEquals("Key not found: test_key", exception.message())
        assertEquals("web5::apid::key_manager::KeyManagerError", exception.errorType())
        assertEquals("KeyNotFound", exception.errorVariant())
    }
}