package web5.sdk

import org.junit.jupiter.api.Test
import web5.sdk.DidDht
import web5.sdk.InMemoryKeyManager

class DidDhtTests {
    @Test
    fun `can do things`() {
        val keyManager = InMemoryKeyManager()
        val jwkData = keyManager.generateKeyMaterial()

        val didDht = DidDht.fromIdentityKey(jwkData)
        val signer = keyManager.getSigner(jwkData)
        didDht.publish(signer)
        didDht.deactivate(signer)
    }
}