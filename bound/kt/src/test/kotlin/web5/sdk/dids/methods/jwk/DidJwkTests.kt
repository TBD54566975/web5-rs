package web5.sdk.dids.methods.jwk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.rust.RustCoreException

import web5.sdk.rust.DidJwk as RustCoreDidJwk

class DidJwkTests {
    @Test
    fun `can create did jwk same as rust core`() {
        val keyManager = InMemoryKeyManager()
        val jwk = keyManager.generateKeyMaterial()

        val didJwk = DidJwk(jwk)

        println(didJwk.document.id)

        val rustCoreDidJwk = RustCoreDidJwk.fromPublicJwk(jwk);
        assertEquals(rustCoreDidJwk.getData().did.uri, didJwk.did.uri)
        assertEquals(rustCoreDidJwk.getData().document.id, didJwk.document.id)
    }

    @Test
    fun `can resolve did jwk uri`() {
        val didUri = "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsImQiOm51bGwsIngiOiJPQ1RWd1pReWFkUWpnVnR4bHZ3aTZTNGFTeEF0OVg2dHl3NU5OZkRoeEtrIiwieSI6bnVsbH0"
        val resolvedDid = DidJwk.resolve(didUri)

        assertEquals(resolvedDid.document!!.id, didUri)
    }

    @Test
    fun `throws exception if did method is not jwk`() {
        // TODO: Should this behave differently - https://github.com/TBD54566975/web5-rs/issues/237
        assertThrows(RustCoreException::class.java) {
            DidJwk.resolve("did:example:123")
        }
    }
}