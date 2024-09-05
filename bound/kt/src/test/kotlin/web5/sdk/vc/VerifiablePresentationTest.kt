package web5.sdk.vc

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.api.Assertions.*
import java.util.Date
import web5.sdk.Web5Exception
import web5.sdk.dids.methods.jwk.DidJwk

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class VerifiablePresentationTest {
    companion object {
        const val HOLDER_DID_URI = "did:web:example.com"
        const val VERIFIABLE_CREDENTIAL = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlpSamRxVTBGaFpqVlVMVnBXYlcwMWVHeExVVXBqVlc5R1JrRklkbGRJT0ZGc2JVdElNVUkxUm5SbkluMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjc2YmY4YTIwLWEzYzAtNDRlNy05NGFhLWI3YTY2NGYwMDNlZiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKWlJqZHFVMEZoWmpWVUxWcFdiVzAxZUd4TFVVcGpWVzlHUmtGSWRsZElPRkZzYlV0SU1VSTFSblJuSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0wNFQyMToyMjowNy40MjUwNDErMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpaUmpkcVUwRmhaalZVTFZwV2JXMDFlR3hMVVVwalZXOUdSa0ZJZGxkSU9GRnNiVXRJTVVJMVJuUm5JbjAifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlpSamRxVTBGaFpqVlVMVnBXYlcwMWVHeExVVXBqVlc5R1JrRklkbGRJT0ZGc2JVdElNVUkxUm5SbkluMCIsImp0aSI6InVybjp1dWlkOjc2YmY4YTIwLWEzYzAtNDRlNy05NGFhLWI3YTY2NGYwMDNlZiIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSlpSamRxVTBGaFpqVlVMVnBXYlcwMWVHeExVVXBqVlc5R1JrRklkbGRJT0ZGc2JVdElNVUkxUm5SbkluMCIsIm5iZiI6MTcyNTQ4NDkyNywiaWF0IjoxNzI1NDg0OTI3fQ.UlunzuF6bIii2b3DUX_clZ4UHLkjR87na5x_UXQZJ0DEhPu2Mv-W6CkEupU92eZOYeK70BkVpeph0EccRonuBg"
    }

    @Test
    fun test_create_verifiable_presentation() {
        val holder = HOLDER_DID_URI
        val verifiableCredential = listOf(VERIFIABLE_CREDENTIAL)

        val options = VerifiablePresentationCreateOptions(
            id = "urn:uuid:12345678-1234-5678-1234-567812345678",
            context = listOf("https://www.w3.org/2018/credentials/v1"),
            type = listOf("VerifiablePresentation"),
            issuanceDate = Date(),
            expirationDate = Date(System.currentTimeMillis() + 86400000) // 1 day later
        )

        val vp = VerifiablePresentation.create(holder, verifiableCredential, options)

        assertEquals(holder, vp.holder)
        assertEquals(options.id, vp.id)
        assertEquals(options.context, vp.context)
        assertEquals(options.type, vp.type)
        assertEquals(verifiableCredential, vp.verifiableCredential)
        assertNotNull(vp.issuanceDate)
        assertNotNull(vp.expirationDate)
    }

    @Test
    fun test_create_verifiable_presentation_with_missing_required_fields() {
        val holder = HOLDER_DID_URI
        val verifiableCredential = listOf(VERIFIABLE_CREDENTIAL)

        // No options provided, defaults will be used
        val vp = VerifiablePresentation.create(holder, verifiableCredential, null)

        assertEquals(holder, vp.holder)
        assertEquals(verifiableCredential, vp.verifiableCredential)
        assertNotNull(vp.issuanceDate)
    }

    @Test
    fun test_create_verifiable_presentation_with_invalid_data() {
        val invalidVcJwt = "invalid_jwt_string"

        assertThrows<Web5Exception> {
            VerifiablePresentation.create(HOLDER_DID_URI, listOf(invalidVcJwt), null)
        }
    }

    @Test
    fun test_sign_verifiable_presentation() {
        val bearerDid = DidJwk.create()
        val holder = bearerDid.did.uri
        val verifiableCredential = listOf(VERIFIABLE_CREDENTIAL)

        val vp = VerifiablePresentation.create(holder, verifiableCredential, null)

        val signedVpJwt = vp.sign(bearerDid)

        assertNotNull(signedVpJwt)
    }

    @Test
    fun test_from_vp_jwt() {
        val bearerDid = DidJwk.create()
        val holder = bearerDid.did.uri

        val verifiableCredential = listOf(VERIFIABLE_CREDENTIAL)

        val vp = VerifiablePresentation.create(holder, verifiableCredential, null)
        val signedVpJwt = vp.sign(bearerDid)

        val decodedVp = VerifiablePresentation.fromVpJwt(signedVpJwt, true)

        assertEquals(vp.holder, decodedVp.holder)
        assertEquals(vp.context, decodedVp.context)
        assertEquals(vp.type, decodedVp.type)
        assertEquals(vp.verifiableCredential, decodedVp.verifiableCredential)
    }
}
