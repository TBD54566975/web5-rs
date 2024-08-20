package web5.sdk.dids.methods.dht

import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.Jwk

import web5.sdk.rust.ed25519GeneratorGenerate as rustCoreEd25519GeneratorGenerate

class DidDhtTests {
    @Test
    fun `can create did dht`() {
        val jwk = rustCoreEd25519GeneratorGenerate()

        val didDht = DidDht(Jwk.fromRustCoreJwkData(jwk))

        assertNotNull(didDht.document.id)
    }
}