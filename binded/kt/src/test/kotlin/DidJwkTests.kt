package web5.sdk

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.did.methods.jwk.DidJwk

class DidJwkTests {
    @Test
    fun `can create did jwk from rust`() {
        // Arrange
        // TODO Swap out for kotlin native after it is built
        val keyManager = InMemoryKeyManager()
        // TODO Swap out for kotlin native after it is built
        val jwkData = keyManager.generateKeyMaterial()


        // Act
        val didJwk = DidJwk(Jwk.fromBinded(jwkData))

        // Assert
        val rustDidJwk = web5.sdk.DidJwk.fromPublicKey(jwkData);
        assertEquals(rustDidJwk.getData().did.uri, didJwk.did.uri)
        assertEquals(rustDidJwk.getData().document.id, didJwk.document.id)
    }
}