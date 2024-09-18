package web5.sdk.dids

import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.dids.methods.web.DidWeb

class ResolutionResultTests {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Resolve {
        @Test
        fun test_invalid_did() {

            val resolutionResult = ResolutionResult.resolve("something invalid")
            assertEquals(ResolutionMetadataError.INVALID_DID, resolutionResult.resolutionMetadata.error)
        }

        @Test
        fun test_did_jwk() {

            val bearerDid = DidJwk.create()

            val resolutionResult = ResolutionResult.resolve(bearerDid.did.uri)
            assertNull(resolutionResult.resolutionMetadata.error)
            assertEquals(bearerDid.document, resolutionResult.document)
        }

        @Test
        fun test_did_web() {

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val url = mockWebServer.url("")

            val bearerDid = DidWeb.create(url.toString())

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(200)
                    .addHeader("Content-Type", "application/json")
                    .setBody(bearerDid.document.toJsonString())
            )

            val resolutionResult = ResolutionResult.resolve(bearerDid.did.uri)

            assertNull(resolutionResult.resolutionMetadata.error)
            assertNotNull(resolutionResult.document)
            assertEquals(bearerDid.document, resolutionResult.document)

            mockWebServer.shutdown()
        }

        @Test
        fun test_method_not_supported() {

            val resolutionResult = ResolutionResult.resolve("did:example:123")
            assertEquals(ResolutionMetadataError.METHOD_NOT_SUPPORTED, resolutionResult.resolutionMetadata.error)
        }
    }
}
