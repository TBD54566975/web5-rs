package web5.sdk

import org.junit.jupiter.api.Test

import web5.sdk.rust.InMemoryKeyManager as RustCoreInMemoryKeyManager
import web5.sdk.rust.DidDht as RustCoreDidDht

class DidDhtTests {
    @Test
    fun `can do things`() {
        val rustCoreKeyManager = RustCoreInMemoryKeyManager()

        val jwkData = rustCoreKeyManager.generateKeyMaterial()

        val rustCoreDidDht = RustCoreDidDht.fromIdentityKey(jwkData)
        val signer = rustCoreKeyManager.getSigner(jwkData)

        rustCoreDidDht.publish(signer)
        rustCoreDidDht.deactivate(signer)
    }
}