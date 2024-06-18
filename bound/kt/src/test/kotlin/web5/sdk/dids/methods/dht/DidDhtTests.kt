package web5.sdk.dids.methods.dht

import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager

class DidDhtTests {
    @Test
    fun `can create did dht`() {
        val keyManager = InMemoryKeyManager()
        val jwk = keyManager.generateKeyMaterial()

        val didDht = DidDht(jwk)

        assertNotNull(didDht.document.id)
    }
}