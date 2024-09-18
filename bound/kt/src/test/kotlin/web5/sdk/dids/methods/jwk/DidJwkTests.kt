package web5.sdk.dids.methods.jwk

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.crypto.Dsa
import web5.sdk.dids.ResolutionMetadataError

class DidJwkTests {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Create {
        @Test
        fun test_can_specify_key_manager() {

            val keyManager = InMemoryKeyManager(listOf())
            val bearerDid = DidJwk.create(DidJwkCreateOptions(keyManager))

            // TODO publicKeyJwk on the document should be of type Jwk
            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertDoesNotThrow {
                keyManager.getSigner(publicJwk)
            }
        }

        @Test
        fun test_can_specify_secp256k1() {

            val bearerDid = DidJwk.create(DidJwkCreateOptions(dsa = Dsa.SECP256K1))

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertEquals("ES256K", publicJwk.alg)
            assertEquals("EC", publicJwk.kty)
            assertEquals("secp256k1", publicJwk.crv)
        }

        @Test
        fun test_defaults_to_ed25519() {

            val bearerDid = DidJwk.create()

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertEquals("Ed25519", publicJwk.alg)
            assertEquals("OKP", publicJwk.kty)
            assertEquals("Ed25519", publicJwk.crv)
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Resolve {
        @Test
        fun test_invalid_did() {

            val resolutionResult = DidJwk.resolve("something invalid")
            assertEquals(ResolutionMetadataError.INVALID_DID, resolutionResult.resolutionMetadata.error)
        }

        @Test
        fun test_create_then_resolve() {

            val bearerDid = DidJwk.create()
            val resolutionResult = DidJwk.resolve(bearerDid.did.uri)
            assertEquals(bearerDid.document, resolutionResult.document)
        }
    }
}