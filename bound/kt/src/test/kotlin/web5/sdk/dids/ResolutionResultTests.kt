package web5.sdk.dids

import okhttp3.mockwebserver.Dispatcher
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import okhttp3.mockwebserver.RecordedRequest
import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.Json
import web5.sdk.UnitTestSuite
import web5.sdk.dids.methods.dht.DidDht
import web5.sdk.dids.methods.dht.DidDhtCreateOptions
import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.dids.methods.web.DidWeb
import web5.sdk.rust.ResolutionMetadataError

class ResolutionResultTests {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Resolve {
        private val testSuite = UnitTestSuite("resolution_result_resolve")

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

            val resolutionResult = ResolutionResult.resolve("something invalid")
            assertEquals(ResolutionMetadataError.INVALID_DID, resolutionResult.resolutionMetadata.error)
        }

        @Test
        fun test_did_jwk() {
            testSuite.include()

            val bearerDid = DidJwk.create()

            val resolutionResult = ResolutionResult.resolve(bearerDid.did.uri)
            assertNull(resolutionResult.resolutionMetadata.error)
            assertEquals(bearerDid.document, resolutionResult.document)
        }

        @Test
        fun test_did_web() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val url = mockWebServer.url("")

            val bearerDid = DidWeb.create(url.toString())

            // temporarily removing @context from document, need to rework Document type
            bearerDid.document.context = null

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(200)
                    .addHeader("Content-Type", "application/json")
                    .setBody(Json.stringify(bearerDid.document))
            )

            val resolutionResult = ResolutionResult.resolve(bearerDid.did.uri)

            assertNull(resolutionResult.resolutionMetadata.error)
            assertNotNull(resolutionResult.document)
            assertEquals(bearerDid.document, resolutionResult.document)

            mockWebServer.shutdown()
        }

        @Test
        fun test_method_not_supported() {
            testSuite.include()

            val resolutionResult = ResolutionResult.resolve("did:example:123")
            assertEquals(ResolutionMetadataError.METHOD_NOT_SUPPORTED, resolutionResult.resolutionMetadata.error)
        }
    }
}