package web5.sdk

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.dids.did.methods.jwk.DidJwk

import web5.sdk.DidJwk as RcbDidJwk

//import web5.sdk.crypto.keys.Jwk
//import web5.sdk.dids.did.methods.jwk.DidJwk

class DidJwkTests {
    @Test
    fun `can create did jwk`() {
        // Arrange
        val keyManager = InMemoryKeyManager()
        val jwk = keyManager.generateKeyMaterial()

        // Act
        val didJwk = DidJwk(jwk)

        // Assert
        val rustDidJwk = RcbDidJwk.fromPublicKey(jwk.toBinding());
        assertEquals(rustDidJwk.getData().did.uri, didJwk.did.uri)
        assertEquals(rustDidJwk.getData().document.id, didJwk.document.id)
    }
}