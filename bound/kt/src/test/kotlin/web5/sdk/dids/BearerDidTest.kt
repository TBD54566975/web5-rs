package web5.sdk.dids

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.rust.ed25519GeneratorGenerate

class BearerDidTest {

    @Test
    fun `test basic bearer did creation`() {
        val privateJwk = ed25519GeneratorGenerate()

        val keyManager = InMemoryKeyManager(listOf())
        val publicJwk = keyManager.importPrivateJwk(Jwk.fromRustCoreJwkData(privateJwk))

        val didJwk = DidJwk(publicJwk)

        val bearerDid = BearerDid(didJwk.did.uri, keyManager)

        assertEquals(bearerDid.document.id, didJwk.document.id)
    }

    @Test
    fun `test bearer did sign`() {
        val privateJwk = ed25519GeneratorGenerate()

        val keyManager = InMemoryKeyManager(listOf())
        val publicJwk = keyManager.importPrivateJwk(Jwk.fromRustCoreJwkData(privateJwk))

        val didJwk = DidJwk(publicJwk)

        val bearerDid = BearerDid(didJwk.did.uri, keyManager)

        val signedPayload = bearerDid.getSigner().sign("hi".toByteArray())
        assertNotNull(signedPayload)
    }
}