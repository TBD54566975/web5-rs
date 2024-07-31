package web5.sdk.dids

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.assertDoesNotThrow
import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertEquals
import web5.sdk.rust.RustCoreException

class PortableDidTest {
    @Test
    fun `can instantiate from json string`() {
        val jsonString = """
            {"uri":"did:web:tbd.website%3A9002:alice","document":{"id":"did:web:tbd.website%3A9002:alice","@context":["https://www.w3.org/ns/did/v1"],"verificationMethod":[{"id":"did:web:tbd.website%3A9002:alice#key-0","type":"JsonWebKey","controller":"did:web:tbd.website%3A9002:alice","publicKeyJwk":{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","x":"NNoVSv_v34ombmylF572t9HYYDiJtMgfckRT1W0vW0g"}}]},"privateKeys":[{"alg":"Ed25519","kty":"OKP","crv":"Ed25519","d":"SwuWbL-Fm64OUFy6x3FBt3RiB79RcnZZrllGT24m4BA","x":"NNoVSv_v34ombmylF572t9HYYDiJtMgfckRT1W0vW0g"}]}
        """.trimIndent()

        assertDoesNotThrow {
            val portableDid = PortableDid(jsonString)
            assertEquals("did:web:tbd.website%3A9002:alice", portableDid.didUri)
        }
    }

    @Test
    fun `instantiation from json string throws with invalid json string`() {
        val invalidJsonString = "something not valid"
        assertThrows(RustCoreException::class.java) {
            PortableDid(invalidJsonString)
        }
    }
}