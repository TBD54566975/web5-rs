package web5.sdk.dids.methods.web

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import web5.sdk.rust.ed25519GeneratorGenerate

class DidWebTests {

    @Test
    fun `can create did web`() {
        val domain = "example.com"
        val jwk = ed25519GeneratorGenerate();

        val didWeb = DidWeb(domain, jwk)
        assertEquals(didWeb.did.uri, "did:web:example.com")
        assertEquals(didWeb.document.verificationMethod.get(0).publicKeyJwk, jwk)
    }

    @Test
    fun `can resolve did web uri`() {
//        Works if you host a local did web document You must host this json at http://localhost:1234/.well-known/did.json
//        val didDocumentJson = """
//            {
//               "id":"did:web.tbd.website",
//               "@context":[
//                  "https://www.w3.org/ns/did/v1"
//               ],
//               "verificationMethod":[
//                  {
//                     "id":"did:web:www.tbd.website#key-0",
//                     "type":"JsonWebKey",
//                     "controller":"did:web:www.tbd.website",
//                     "publicKeyJwk":{
//                        "alg":"Ed25519",
//                        "kty":"OKP",
//                        "crv":"Ed25519",
//                        "x":"gNFtgCZhOYv00p48FHQYt4edkoBPOyw0oGAB20LrT0c"
//                     }
//                  }
//               ]
//            }
//        """.trimIndent()
//
//        val didUri = "did:web:localhost%3A1234"
//        val resolvedDid = DidWeb.resolve(didUri)
//
//        assertEquals(resolvedDid.document!!.id, "did:web:www.tbd.website")
    }
}