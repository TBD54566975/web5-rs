package web5.sdk.dids.methods.dht

import okhttp3.mockwebserver.Dispatcher
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import okhttp3.mockwebserver.RecordedRequest
import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.*

class DidDhtTests {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Create {
        private val testSuite = UnitTestSuite("did_dht_create")

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
            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false,
                    keyManager = keyManager
                )
            )

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertDoesNotThrow {
                keyManager.getSigner(Jwk.fromRustCoreJwkData(publicJwk))
            }
        }

        @Test
        fun test_can_specify_publish_and_gateway_url() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val gatewayUrl = mockWebServer.url("")

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(200)
                    .addHeader("Content-Type", "application/octet-stream")
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = true,
                    gatewayUrl = gatewayUrl.toString()
                )
            )

            mockWebServer.takeRequest().apply {
                assertEquals("/${bearerDid.did.uri.removePrefix("did:dht:")}", path)
                assertEquals("PUT", method)
                assertEquals("application/octet-stream", headers["Content-Type"])
            }

            mockWebServer.shutdown()
        }

        @Test
        fun test_should_add_optional_verification_methods() {
            testSuite.include()

            val additionalVerificationMethod = VerificationMethodData(
                id = "did:web:example.com#key-1",
                type = "JsonWebKey",
                controller = "did:web:example.com",
                publicKeyJwk = JwkData(
                    kty = "OKP",
                    crv = "Ed25519",
                    x = "some pub value",
                    alg = null,
                    y = null,
                    d = null
                )
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false,
                    verificationMethod = listOf(additionalVerificationMethod)
                )
            )

            assertEquals(2, bearerDid.document.verificationMethod.size)
            assertEquals(additionalVerificationMethod, bearerDid.document.verificationMethod[1])
        }

        @Test
        fun test_should_add_optional_services() {
            testSuite.include()

            val service = ServiceData(
                id = "did:web:example.com#service-0",
                type = "SomeService",
                serviceEndpoint = listOf("https://example.com/service")
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false,
                    service = listOf(service)
                )
            )

            assertEquals(service, bearerDid.document.service!!.first())
        }

        @Test
        fun test_should_add_optional_also_known_as() {
            testSuite.include()

            val alsoKnownAs = listOf("https://alias.example.com")

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false,
                    alsoKnownAs = alsoKnownAs
                )
            )

            assertEquals(alsoKnownAs, bearerDid.document.alsoKnownAs)
        }

        @Test
        fun test_should_add_optional_controllers() {
            testSuite.include()

            val controllers = listOf("did:web:controller.example.com")

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false,
                    controller = controllers
                )
            )

            assertEquals(controllers, bearerDid.document.controller)
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Publish {
        private val testSuite = UnitTestSuite("did_dht_publish")

        @AfterAll
        fun verifyAllTestsIncluded() {
            if (testSuite.tests.isNotEmpty()) {
                println("The following tests were not included or executed:")
                testSuite.tests.forEach { println(it) }
                fail("Not all tests were executed! ${testSuite.tests}")
            }
        }

        @Test
        fun test_can_specify_gateway_url() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val gatewayUrl = mockWebServer.url("")

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(200)
                    .addHeader("Content-Type", "application/octet-stream")
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false
                )
            )

            DidDht.publish(
                bearerDid,
                DidDhtPublishOptions(
                    gatewayUrl = gatewayUrl.toString()
                )
            )

            val request = mockWebServer.takeRequest()
            assertEquals("PUT", request.method)
            assertEquals("/${bearerDid.did.uri.removePrefix("did:dht:")}", request.path)
            assertEquals("application/octet-stream", request.getHeader("Content-Type"))

            mockWebServer.shutdown()
        }

        @Test
        fun test_can_handle_network_error() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val gatewayUrl = mockWebServer.url("")

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(500)
                    .addHeader("Content-Type", "application/octet-stream")
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false
                )
            )

            val exception = assertThrows<Web5Exception.Exception> {
                DidDht.publish(
                    bearerDid,
                    DidDhtPublishOptions(
                        gatewayUrl = gatewayUrl.toString()
                    )
                )
            }

            assertEquals("network error failed to PUT DID to mainline", exception.msg)
            assertEquals("Network", exception.variant)

            mockWebServer.shutdown()
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Resolve {
        private val testSuite = UnitTestSuite("did_dht_resolve")

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

            val resolutionResult = DidDht.resolve("something invalid")

            assertEquals(
                ResolutionMetadataError.INVALID_DID,
                resolutionResult.resolutionMetadata.error
            )
        }

        @Test
        fun test_method_not_supported() {
            testSuite.include()

            val resolutionResult = DidDht.resolve("did:web:example")

            assertEquals(
                ResolutionMetadataError.METHOD_NOT_SUPPORTED,
                resolutionResult.resolutionMetadata.error
            )
        }

        @Test
        fun test_not_found() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val gatewayUrl = mockWebServer.url("")

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(404)
                    .addHeader("Content-Type", "application/octet-stream")
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false
                )
            )

            val resolutionResult = DidDht.resolve(
                bearerDid.did.uri,
                DidDhtResolveOptions(
                    gatewayUrl = gatewayUrl.toString()
                )
            )

            assertEquals(
                ResolutionMetadataError.NOT_FOUND,
                resolutionResult.resolutionMetadata.error
            )

            mockWebServer.shutdown()
        }

        @Test
        fun test_internal_error() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            val gatewayUrl = mockWebServer.url("")

            mockWebServer.enqueue(
                MockResponse()
                    .setResponseCode(500)
                    .addHeader("Content-Type", "application/octet-stream")
            )

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = false
                )
            )

            val resolutionResult = DidDht.resolve(
                bearerDid.did.uri,
                DidDhtResolveOptions(
                    gatewayUrl = gatewayUrl.toString()
                )
            )

            assertEquals(
                ResolutionMetadataError.INTERNAL_ERROR,
                resolutionResult.resolutionMetadata.error
            )

            mockWebServer.shutdown()
        }

        @Test
        fun test_can_create_then_resolve() {
            testSuite.include()

            val mockWebServer = MockWebServer()
            mockWebServer.start()

            // Capture the body of the published DID Document
            val publishedBody = mutableListOf<ByteArray>()

            mockWebServer.dispatcher = object : Dispatcher() {
                override fun dispatch(request: RecordedRequest): MockResponse {
                    return when {
                        request.method == "PUT" -> {
                            // Capture the published body
                            publishedBody.add(request.body.readByteArray())
                            MockResponse()
                                .setResponseCode(200)
                                .addHeader("Content-Type", "application/octet-stream")
                        }

                        request.method == "GET" -> {
                            MockResponse()
                                .setResponseCode(200)
                                .addHeader("Content-Type", "application/octet-stream")
                                .setBody(okio.Buffer().write(publishedBody.first()))
                        }

                        else -> MockResponse().setResponseCode(404)
                    }
                }
            }

            val gatewayUrl = mockWebServer.url("")

            val bearerDid = DidDht.create(
                DidDhtCreateOptions(
                    publish = true,
                    gatewayUrl = gatewayUrl.toString()
                )
            )

            val resolutionResult = DidDht.resolve(
                bearerDid.did.uri,
                DidDhtResolveOptions(
                    gatewayUrl = gatewayUrl.toString()
                )
            )

            assertNull(resolutionResult.resolutionMetadata.error)
            assertNotNull(resolutionResult.document)
            assertEquals(bearerDid.document, resolutionResult.document)

            mockWebServer.shutdown()
        }
    }
}