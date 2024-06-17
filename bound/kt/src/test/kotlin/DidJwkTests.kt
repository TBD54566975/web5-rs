package web5.sdk

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.dids.methods.jwk.DidJwk

import web5.sdk.rust.DidJwk as RustCoreDidJwk

class DidJwkTests {
    @Test
    fun `can create did jwk same as rust core`() {
        val keyManager = InMemoryKeyManager()
        val jwk = keyManager.generateKeyMaterial()

        val didJwk = DidJwk(jwk)

        val rustCoreDidJwk = RustCoreDidJwk.fromPublicJwk(jwk.toBinding());
        assertEquals(rustCoreDidJwk.getData().did.uri, didJwk.did.uri)
        assertEquals(rustCoreDidJwk.getData().document.id, didJwk.document.id)
    }

    @Test
    fun `can create did jwk`() {
        val keyManager = InMemoryKeyManager()
        val jwk = keyManager.generateKeyMaterial()

        val didJwk = DidJwk(jwk)

        print(didJwk.did.uri)

        assertNotNull(didJwk.document.id)
    }
}