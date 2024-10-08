package web5.sdk.dids.methods.web

import web5.sdk.Web5Exception
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.crypto.Dsa
import web5.sdk.crypto.keys.InMemoryKeyManager
import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.Service
import web5.sdk.dids.VerificationMethod
import web5.sdk.dids.ResolutionMetadataError

class DidWebTests {
    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Create {
        @Test
        fun test_can_specify_key_manager() {

            val keyManager = InMemoryKeyManager(listOf())
            val bearerDid = DidWeb.create("localhost", DidWebCreateOptions(keyManager = keyManager))

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertDoesNotThrow {
                keyManager.getSigner(publicJwk)
            }
        }

        @Test
        fun test_can_specify_secp256k1() {

            val bearerDid = DidWeb.create("localhost", DidWebCreateOptions(dsa = Dsa.SECP256K1))

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertEquals("ES256K", publicJwk.alg)
            assertEquals("EC", publicJwk.kty)
            assertEquals("secp256k1", publicJwk.crv)
        }

        @Test
        fun test_defaults_to_ed25519() {

            val bearerDid = DidWeb.create("localhost")

            val publicJwk = bearerDid.document.verificationMethod.first().publicKeyJwk
            assertEquals("Ed25519", publicJwk.alg)
            assertEquals("OKP", publicJwk.kty)
            assertEquals("Ed25519", publicJwk.crv)
        }

        @Test
        fun test_invalid_domain() {

            val exception = assertThrows<Web5Exception> {
                DidWeb.create("invalid domain")
            }

            assertTrue(exception.message.contains("url parse failure") ?: false)
            assertEquals("Parameter", exception.variant)
        }

        @Test
        fun test_should_allow_http_for_localhost() {

            assertDoesNotThrow {
                DidWeb.create("http://localhost")
            }

            assertDoesNotThrow {
                DidWeb.create("http://127.0.0.1")
            }

            val exception = assertThrows<Web5Exception> {
                DidWeb.create("http://example.com")
            }
            assertEquals(
                "parameter error only https is allowed except for localhost or 127.0.0.1 with http",
                exception.message
            )
            assertEquals("Parameter", exception.variant)
        }

        @Test
        fun test_must_be_https() {

            val exception = assertThrows<Web5Exception> {
                DidWeb.create("http://example.com")
            }
            assertEquals(
                "parameter error only https is allowed except for localhost or 127.0.0.1 with http",
                exception.message
            )
            assertEquals("Parameter", exception.variant)

            assertDoesNotThrow {
                DidWeb.create("https://example.com")
            }
        }

        @Test
        fun test_should_trim_did_json() {

            val bearerDid = DidWeb.create("https://example.com/did.json")
            assertEquals("did:web:example.com", bearerDid.did.uri)
        }

        @Test
        fun test_should_trim_well_known() {

            val bearerDid = DidWeb.create("https://example.com/.well-known/did.json")
            assertEquals("did:web:example.com", bearerDid.did.uri)
        }

        @Test
        fun test_should_percent_encode_colons() {

            val bearerDid = DidWeb.create("https://example.com:8080")
            assertEquals("did:web:example.com%3A8080", bearerDid.did.uri)
        }

        @Test
        fun test_should_replace_path_with_colons() {

            val bearerDid = DidWeb.create("https://example.com/path/to/resource")
            assertEquals("did:web:example.com:path:to:resource", bearerDid.did.uri)
        }

        @Test
        fun test_should_add_optional_verification_methods() {

            val additionalVerificationMethod = VerificationMethod(
                id = "did:web:example.com#key-1",
                type = "JsonWebKey",
                controller = "did:web:example.com",
                publicKeyJwk = Jwk(
                    kty = "OKP",
                    crv = "Ed25519",
                    x = "some pub value",
                    alg = null,
                    y = null,
                    d = null
                )
            )

            val bearerDid = DidWeb.create(
                "https://example.com",
                DidWebCreateOptions(verificationMethod = listOf(additionalVerificationMethod))
            )

            assertEquals(2, bearerDid.document.verificationMethod.size)
            assertEquals(additionalVerificationMethod, bearerDid.document.verificationMethod[1])
        }

        @Test
        fun test_should_add_optional_services() {

            val service = Service(
                id = "did:web:example.com#service-0",
                type = "SomeService",
                serviceEndpoint = listOf("https://example.com/service")
            )

            val bearerDid = DidWeb.create(
                "https://example.com",
                DidWebCreateOptions(service = listOf(service))
            )

            assertEquals(service, bearerDid.document.service!!.first())
        }

        @Test
        fun test_should_add_optional_also_known_as() {

            val alsoKnownAs = listOf("https://alias.example.com")

            val bearerDid = DidWeb.create(
                "https://example.com",
                DidWebCreateOptions(alsoKnownAs = alsoKnownAs)
            )

            assertEquals(alsoKnownAs, bearerDid.document.alsoKnownAs)
        }

        @Test
        fun test_should_add_optional_controllers() {

            val controllers = listOf("did:web:controller.example.com")

            val bearerDid = DidWeb.create(
                "https://example.com",
                DidWebCreateOptions(controller = controllers)
            )

            assertEquals(controllers, bearerDid.document.controller)
        }
    }

    @Nested
    @TestInstance(TestInstance.Lifecycle.PER_CLASS)
    inner class Resolve {

        @Test
        fun test_invalid_did() {

            val resolutionResult = DidWeb.resolve("something invalid")
            assertEquals(ResolutionMetadataError.INVALID_DID, resolutionResult.resolutionMetadata.error)
        }

        @Test
        fun test_create_then_resolve() {

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

            val resolveResult = DidWeb.resolve(bearerDid.did.uri)

            assertNull(resolveResult.resolutionMetadata.error)
            assertNotNull(resolveResult.document)
            assertEquals(bearerDid.document, resolveResult.document)

            mockWebServer.shutdown()
        }
    }
}