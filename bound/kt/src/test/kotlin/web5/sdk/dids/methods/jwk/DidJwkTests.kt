package web5.sdk.dids.methods.jwk

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Dsa
import web5.sdk.rust.ResolutionMetadataError

class DidJwkTests {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Create {
        private val testSuite = UnitTestSuite("did_jwk_create")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_can_specify_key_manager() {
            testSuite.include()

            val keyManager = InMemoryKeyManager(listOf())
            val bearerDid = DidJwk.create(DidJwkCreateOptions(keyManager))

            // TODO publicKeyJwk on the document should be of type Jwk
            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertDoesNotThrow {
                keyManager.getSigner(Jwk.fromRustCoreJwkData(publicJwk))
            }
        }

        @Test
        fun test_can_specify_secp256k1() {
            testSuite.include()

            val bearerDid = DidJwk.create(DidJwkCreateOptions(dsa = Dsa.SECP256K1))

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertEquals("ES256K", publicJwk.alg)
            assertEquals("EC", publicJwk.kty)
            assertEquals("secp256k1", publicJwk.crv)
        }

        @Test
        fun test_defaults_to_ed25519() {
            testSuite.include()

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
        private val testSuite = UnitTestSuite("did_jwk_resolve")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_invalid_did() {
            testSuite.include()

            val resolutionResult = DidJwk.resolve("something invalid")
            assertEquals(ResolutionMetadataError.INVALID_DID, resolutionResult.resolutionMetadata.error)
        }

        @Test
        fun test_create_then_resolve() {
            testSuite.include()

            val bearerDid = DidJwk.create()
            val resolutionResult = DidJwk.resolve(bearerDid.did.uri)
            assertEquals(bearerDid.document, resolutionResult.document)
        }
    }
}