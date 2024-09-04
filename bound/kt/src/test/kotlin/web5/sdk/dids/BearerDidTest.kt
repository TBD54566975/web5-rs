package web5.sdk.dids

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.dids.methods.jwk.DidJwkCreateOptions
import web5.sdk.Web5Exception

class BearerDidTest {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class FromPortableDid {
        private val testSuite = UnitTestSuite("bearer_did_from_portable_did")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_can_instantiate_successfully() {
            testSuite.include()

            val portableDid = PortableDid.fromJsonString("""
                {"uri":"did:web:tbd.website%3A9002:alice","document":{"id":"did:web:tbd.website%3A9002:alice","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:web:tbd.website%3A9002:alice#key-0","type":"JsonWebKey","controller":"did:web:tbd.website%3A9002:alice","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"NNoVSv_v34ombmylF572t9HYYDiJtMgfckRT1W0vW0g"}}]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"SwuWbL-Fm64OUFy6x3FBt3RiB79RcnZZrllGT24m4BA","x":"NNoVSv_v34ombmylF572t9HYYDiJtMgfckRT1W0vW0g"}]}
            """.trimIndent())

            assertDoesNotThrow {
                BearerDid.fromPortableDid(portableDid)
            }
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class GetSigner {
        private val testSuite = UnitTestSuite("bearer_did_get_signer")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_verification_method_id_empty() {
            testSuite.include()

            val bearerDid = DidJwk.create()

            val exception = assertThrows<Web5Exception> {
                bearerDid.getSigner("")
            }

            assertEquals("parameter error verification_method_id cannot be empty", exception.message)
            assertEquals("Parameter", exception.variant)
        }

        @Test
        fun test_not_found_by_verification_method_id() {
            testSuite.include()

            val bearerDid = DidJwk.create()

            val exception = assertThrows<Web5Exception> {
                bearerDid.getSigner("invalid_id")
            }

            assertEquals("not found error verification method not found", exception.message)
            assertEquals("NotFound", exception.variant)
        }

        @Test
        fun test_found_by_verification_method_id() {
            testSuite.include()

            val bearerDid = DidJwk.create()

            assertDoesNotThrow {
                bearerDid.getSigner(bearerDid.document.verificationMethod[0].id)
            }
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class ToPortableDid {
        private val testSuite = UnitTestSuite("bearer_did_to_portable_did")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_can_export() {
            testSuite.include()

            val inMemoryKeyManager = InMemoryKeyManager(listOf())
            val bearerDid = DidJwk.create(DidJwkCreateOptions(keyManager = inMemoryKeyManager))

            assertDoesNotThrow {
                val portableDid = bearerDid.toPortableDid(inMemoryKeyManager)
                assertEquals(bearerDid.did.uri, portableDid.didUri)
            }
        }
    }
}