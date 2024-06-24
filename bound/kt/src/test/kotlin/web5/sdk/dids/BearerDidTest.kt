package web5.sdk.dids

import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.rust.ed25519GeneratorGenerate

class BearerDidTest {

    @Test
    fun `test basic bearer did creation`() {
        val privateJwk = ed25519GeneratorGenerate()

        val keyManager = InMemoryKeyManager(listOf())
        val publicJwk = keyManager.importPrivateJwk(privateJwk)

        val didJwk = DidJwk(publicJwk)

        // TODO: This is throwing an error on the rust side
         val bearerDid = BearerDid(didJwk.did.uri, keyManager)
    }
}