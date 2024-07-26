package web5.sdk.dids.methods.web

import io.mockk.coEvery
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import web5.sdk.dids.Document
import web5.sdk.rust.DidJwk
import web5.sdk.rust.RustCoreException

class DidWebTests {

    // TODO: Did web resolution errors - https://github.com/TBD54566975/web5-rs/issues/272
//    @Test
//    fun `can create did web from uri`() = runBlocking {
//        val didUri = "did:web:localhost:1234"
//        val resolvedDid = DidWeb(didUri)
//
//        assertEquals("did:web:localhost:1234", resolvedDid.document!!.id)
//    }

//    @Test
//    fun `can create did web from url`() {
//        val didDocument = Document(
//            id = "did:web:localhost:1234",
//            context = listOf("https://www.w3.org/ns/did/v1"),
//            verificationMethod = listOf(
//                VerificationMethod(
//                    id = "did:web:www.tbd.website#key-0",
//                    type = "JsonWebKey",
//                    controller = "did:web:www.tbd.website",
//                    publicKeyJwk = PublicKeyJwk(
//                        alg = "Ed25519",
//                        kty = "OKP",
//                        crv = "Ed25519",
//                        x = "gNFtgCZhOYv00p48FHQYt4edkoBPOyw0oGAB20LrT0c"
//                    )
//                )
//            )
//        )
//
//        // TODO: Construct a resolution result the correct way
//        val resolutionResult = web5.sdk.dids.methods.jwk.DidJwk.resolve("did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsImQiOiJwR0tScHBWczlqYkhWYkZjNm5MTmZQZEN0RG9lOHV1VVNaNVMzX3VOWXpvIiwieCI6IjV5V0FBMkZBM25pMnJaSjh3NVVKMkxMdFZHN2lsYU0wRkc1ZVYwXzZ4U3cifQ")
//
//        mockkStatic("web5.sdk.dids.methods.web.DidWeb")
//        coEvery { DidWeb.resolve("did:web:localhost:1234") } returns resolutionResult
//
//        val didWebResolutionResult = DidWeb.resolve("did:web:localhost:1234")
//        assertEquals("did:web:localhost:1234", didWebResolutionResult.document!!.id)
//    }
}
