package web5.sdk.vc.pex

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import web5.sdk.Json
import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.vc.*

class PresentationDefinitionTest {

  @Test
  fun `test basic presentation definition`() {
    val inputDescriptor = InputDescriptor(
      id = "test_input",
      name = "Test Input",
      purpose = "For testing",
      constraints = Constraints(
        fields = listOf(
          Field(
            id = "field1",
            name = "Field 1",
            path = listOf("$.field1"),
            purpose = "Test field",
            filter = Filter(
              type = "string",
              pattern = "^[a-zA-Z]+$",
            ),
            optional = false,
            predicate = Optionality.Required
          )
        )
      )
    )

    val presentationDefinition = PresentationDefinition(
      id = "test_presentation",
      name = "Test Presentation",
      purpose = "For testing purposes",
      inputDescriptors = listOf(inputDescriptor)
    )

    assertEquals("test_presentation", presentationDefinition.id)
    assertEquals("Test Presentation", presentationDefinition.name)
    assertEquals("For testing purposes", presentationDefinition.purpose)
    assertEquals(1, presentationDefinition.inputDescriptors.size)

    val firstInputDescriptor = presentationDefinition.inputDescriptors[0]
    assertEquals("test_input", firstInputDescriptor.id)
    assertEquals("Test Input", firstInputDescriptor.name)
    assertEquals("For testing", firstInputDescriptor.purpose)

    val field = firstInputDescriptor.constraints.fields[0]
    assertEquals("field1", field.id)
    assertEquals("Field 1", field.name)
    assertEquals(listOf("$.field1"), field.path)
    assertEquals("Test field", field.purpose)
    assertFalse(field.optional ?: true)
    assertEquals(Optionality.Required, field.predicate)

    val filter = field.filter
    assertNotNull(filter)
    assertEquals("string", filter?.type)
    assertEquals("^[a-zA-Z]+$", filter?.pattern)
    assertNull(filter?.const)
  }

  @Test
  fun `test select credentials`() {
    // Define a mock InputDescriptor with constraints
    val inputDescriptor = InputDescriptor(
      id = "test_input",
      name = "Test Input",
      purpose = "For testing",
      constraints = Constraints(
        fields = listOf(
          Field(
            id = "field1",
            name = "Field 1",
            path = listOf("$.credentialSubject.id"),
            purpose = "Test field",
            filter = Filter(
              type = "string",
              // Matching value for 1st of 2 DID JWK's in VC-JWTs below
              const = "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJHWXFJa0xiN3ZuYktmbUhIVkNwcDJOQndOVEJQOXdUZW4tdkZReWhLbnp3In0"
            ),
            optional = false,
            predicate = Optionality.Required
          )
        )
      )
    )

    // Create a PresentationDefinition with the InputDescriptor
    val presentationDefinition = PresentationDefinition(
      id = "test_presentation",
      name = "Test Presentation",
      purpose = "For testing purposes",
      inputDescriptors = listOf(inputDescriptor)
    )

    val vcJwt1 = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhXWEZKYTB4aU4zWnVZa3RtYlVoSVZrTndjREpPUW5kT1ZFSlFPWGRVWlc0dGRrWlJlV2hMYm5wM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRjZjQ2Y2ZkLTAwMGQtNDJiZi1iZWYyLTEwOGFkYzNlMTBmZSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFdYRkphMHhpTjNadVlrdG1iVWhJVmtOd2NESk9RbmRPVkVKUU9YZFVaVzR0ZGtaUmVXaExibnAzSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xMFQxODowMTo0M1oiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpIV1hGSmEweGlOM1p1WWt0bWJVaElWa053Y0RKT1FuZE9WRUpRT1hkVVpXNHRka1pSZVdoTGJucDNJbjAifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhXWEZKYTB4aU4zWnVZa3RtYlVoSVZrTndjREpPUW5kT1ZFSlFPWGRVWlc0dGRrWlJlV2hMYm5wM0luMCIsImp0aSI6InVybjp1dWlkOjRjZjQ2Y2ZkLTAwMGQtNDJiZi1iZWYyLTEwOGFkYzNlMTBmZSIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhXWEZKYTB4aU4zWnVZa3RtYlVoSVZrTndjREpPUW5kT1ZFSlFPWGRVWlc0dGRrWlJlV2hMYm5wM0luMCIsIm5iZiI6MTcyNTk5MTMwMywiaWF0IjoxNzI1OTkxMzAzfQ.mKMKH1XoqXPFVLREAXspb8JTrxSNDHCpJZF23uB7CSSzbKUtQBVk_wUIMPnOTCj6W5YcoF9Gsz4oWl_TlzfRDA"
    val vcJwt2 = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJbVJSUm04eWJVbzFlbFJSVW5kbFRUZElTME5PYVROMlV6bGtORTFMWjJKNE1IWlBVRGh6U0VKSk9FVWlmUSMwIiwidHlwIjoiSldUIn0.eyJpc3MiOiJkaWQ6andrOmV5SnJkSGtpT2lKUFMxQWlMQ0pqY25ZaU9pSkZaREkxTlRFNUlpd2llQ0k2SW1SUlJtOHliVW8xZWxSUlVuZGxUVGRJUzBOT2FUTjJVemxrTkUxTFoySjRNSFpQVURoelNFSkpPRVVpZlEiLCJqdGkiOiJ1cm46dmM6dXVpZDo2NDIzOTIxYy03NjEzLTQzYjAtOWVhMi1iOTA2MTNhMDMwNTciLCJuYmYiOjE3MTIyMzY3MjYsInN1YiI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJbVJSUm04eWJVbzFlbFJSVW5kbFRUZElTME5PYVROMlV6bGtORTFMWjJKNE1IWlBVRGh6U0VKSk9FVWlmUSIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIl0sInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJUQkRldmVsb3BlckNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpyZEhraU9pSlBTMUFpTENKamNuWWlPaUpGWkRJMU5URTVJaXdpZUNJNkltUlJSbTh5YlVvMWVsUlJVbmRsVFRkSVMwTk9hVE4yVXpsa05FMUxaMko0TUhaUFVEaHpTRUpKT0VVaWZRIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SnJkSGtpT2lKUFMxQWlMQ0pqY25ZaU9pSkZaREkxTlRFNUlpd2llQ0k2SW1SUlJtOHliVW8xZWxSUlVuZGxUVGRJUzBOT2FUTjJVemxrTkUxTFoySjRNSFpQVURoelNFSkpPRVVpZlEiLCJ1c2VybmFtZSI6Im5pdHJvIn0sImlkIjoidXJuOnZjOnV1aWQ6NjQyMzkyMWMtNzYxMy00M2IwLTllYTItYjkwNjEzYTAzMDU3IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNC0wNFQxMzoxODo0NloifX0.AahszhMp2ZpKyr-Mtt_CtNGFxAfhxptUN88AdsANKkAQ824qblkeocIhGokWnGQY8-W59cm8Va9wYhmkVv9oAg"

    val vcJwts = listOf(vcJwt1, vcJwt2)

    // Test the selectCredentials function
    val selectedCredentials = presentationDefinition.selectCredentials(vcJwts)

    // Assert that only the valid JWT is selected
    assertEquals(1, selectedCredentials.size)
    assertTrue(selectedCredentials.contains(vcJwt1))
  }

  @Test
  fun `test create presentation from credentials`() {
    // Define a mock InputDescriptor with constraints
    val inputDescriptor = InputDescriptor(
      id = "test_input",
      name = "Test Input",
      purpose = "For testing",
      constraints = Constraints(
        fields = listOf(
          Field(
            id = "field1",
            name = "Field 1",
            path = listOf("$.credentialSubject.id"),
            purpose = "Test field",
            filter = Filter(
              type = "string",
              const = "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJHWXFJa0xiN3ZuYktmbUhIVkNwcDJOQndOVEJQOXdUZW4tdkZReWhLbnp3In0",
            ),
            optional = false,
            predicate = Optionality.Required
          )
        )
      )
    )

    // Create a PresentationDefinition with the InputDescriptor
    val presentationDefinition = PresentationDefinition(
      id = "test_presentation",
      name = "Test Presentation",
      purpose = "For testing purposes",
      inputDescriptors = listOf(inputDescriptor)
    )

    val vcJwt1 = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhXWEZKYTB4aU4zWnVZa3RtYlVoSVZrTndjREpPUW5kT1ZFSlFPWGRVWlc0dGRrWlJlV2hMYm5wM0luMCMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOjRjZjQ2Y2ZkLTAwMGQtNDJiZi1iZWYyLTEwOGFkYzNlMTBmZSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKSFdYRkphMHhpTjNadVlrdG1iVWhJVmtOd2NESk9RbmRPVkVKUU9YZFVaVzR0ZGtaUmVXaExibnAzSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xMFQxODowMTo0M1oiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpIV1hGSmEweGlOM1p1WWt0bWJVaElWa053Y0RKT1FuZE9WRUpRT1hkVVpXNHRka1pSZVdoTGJucDNJbjAifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhXWEZKYTB4aU4zWnVZa3RtYlVoSVZrTndjREpPUW5kT1ZFSlFPWGRVWlc0dGRrWlJlV2hMYm5wM0luMCIsImp0aSI6InVybjp1dWlkOjRjZjQ2Y2ZkLTAwMGQtNDJiZi1iZWYyLTEwOGFkYzNlMTBmZSIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkRJMU5URTVJaXdpYTNSNUlqb2lUMHRRSWl3aVkzSjJJam9pUldReU5UVXhPU0lzSW5naU9pSkhXWEZKYTB4aU4zWnVZa3RtYlVoSVZrTndjREpPUW5kT1ZFSlFPWGRVWlc0dGRrWlJlV2hMYm5wM0luMCIsIm5iZiI6MTcyNTk5MTMwMywiaWF0IjoxNzI1OTkxMzAzfQ.mKMKH1XoqXPFVLREAXspb8JTrxSNDHCpJZF23uB7CSSzbKUtQBVk_wUIMPnOTCj6W5YcoF9Gsz4oWl_TlzfRDA"
    val vcJwt2 = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJbVJSUm04eWJVbzFlbFJSVW5kbFRUZElTME5PYVROMlV6bGtORTFMWjJKNE1IWlBVRGh6U0VKSk9FVWlmUSMwIiwidHlwIjoiSldUIn0.eyJpc3MiOiJkaWQ6andrOmV5SnJkSGtpT2lKUFMxQWlMQ0pqY25ZaU9pSkZaREkxTlRFNUlpd2llQ0k2SW1SUlJtOHliVW8xZWxSUlVuZGxUVGRJUzBOT2FUTjJVemxrTkUxTFoySjRNSFpQVURoelNFSkpPRVVpZlEiLCJqdGkiOiJ1cm46dmM6dXVpZDo2NDIzOTIxYy03NjEzLTQzYjAtOWVhMi1iOTA2MTNhMDMwNTciLCJuYmYiOjE3MTIyMzY3MjYsInN1YiI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJbVJSUm04eWJVbzFlbFJSVW5kbFRUZElTME5PYVROMlV6bGtORTFMWjJKNE1IWlBVRGh6U0VKSk9FVWlmUSIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIl0sInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJUQkRldmVsb3BlckNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpyZEhraU9pSlBTMUFpTENKamNuWWlPaUpGWkRJMU5URTVJaXdpZUNJNkltUlJSbTh5YlVvMWVsUlJVbmRsVFRkSVMwTk9hVE4yVXpsa05FMUxaMko0TUhaUFVEaHpTRUpKT0VVaWZRIiwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SnJkSGtpT2lKUFMxQWlMQ0pqY25ZaU9pSkZaREkxTlRFNUlpd2llQ0k2SW1SUlJtOHliVW8xZWxSUlVuZGxUVGRJUzBOT2FUTjJVemxrTkUxTFoySjRNSFpQVURoelNFSkpPRVVpZlEiLCJ1c2VybmFtZSI6Im5pdHJvIn0sImlkIjoidXJuOnZjOnV1aWQ6NjQyMzkyMWMtNzYxMy00M2IwLTllYTItYjkwNjEzYTAzMDU3IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNC0wNFQxMzoxODo0NloifX0.AahszhMp2ZpKyr-Mtt_CtNGFxAfhxptUN88AdsANKkAQ824qblkeocIhGokWnGQY8-W59cm8Va9wYhmkVv9oAg"

    val vcJwts = listOf(vcJwt1, vcJwt2)

    // Test the createPresentationFromCredentials function
    val presentationResult = presentationDefinition.createPresentationFromCredentials(vcJwts)

    // Validate the results
    assertNotNull(presentationResult, "PresentationResult should not be null")

    // Validate that a PresentationSubmission was created
    val submission = presentationResult.presentationSubmission
    assertNotNull(submission, "PresentationSubmission should not be null")
    assertEquals("test_presentation", submission.definitionId)

    // Validate the descriptor map
    val descriptorMap = submission.descriptorMap
    assertEquals(1, descriptorMap.size, "There should be one descriptor in the map")
    assertEquals("test_input", descriptorMap[0].id)
    assertEquals("jwt_vc", descriptorMap[0].format)
    assertEquals("$.verifiableCredential[0]", descriptorMap[0].path)

    // Validate the matched VCs
    val matchedVcJwts = presentationResult.matchedVcJwts
    assertEquals(1, matchedVcJwts.size, "Only one VC should have been matched")
    assertTrue(matchedVcJwts.contains(vcJwt1))
  }

  @Test
  fun `test full presentation exchange flow`() {
    // Step 1: Create an issuer (typically the entity that issued the credential)
    val issuer = DidJwk.create()
    val issuerUri = issuer.did.uri

    // Step 2: Define a Presentation Definition (PD) that specifies the required input descriptors
    val inputDescriptor = InputDescriptor(
      id = "test_input",
      name = "Test Input",
      purpose = "Testing Input",
      constraints = Constraints(
        fields = listOf(
          Field(
            id = "field1",
            name = "Field 1",
            path = listOf("$.credentialSubject.id"),
            purpose = "Must match DID JWK pattern",
            filter = Filter(
              type = "string",
              pattern = "^did:jwk:.*$" // Matching pattern for DID JWK
            ),
            optional = false,
            predicate = Optionality.Required
          )
        )
      )
    )

    // Create a Presentation Definition with the InputDescriptor
    val presentationDefinition = PresentationDefinition(
      id = "test_presentation_definition",
      name = "Test Presentation Definition",
      purpose = "Testing Presentation Exchange",
      inputDescriptors = listOf(inputDescriptor)
    )

    // Step 3: Create a Verifiable Credential (VC) with a credential subject matching the PD criteria
    val vc = VerifiableCredential.create(
      Issuer.StringIssuer(issuerUri),
      CredentialSubject(issuerUri)
    )

    val vcJwt = vc.sign(issuer)

    // Step 4: Select the credentials that match the PD's input descriptors
    val vcJwts = listOf(vcJwt)
    val presentationResult = presentationDefinition.createPresentationFromCredentials(vcJwts)

    // Step 5: Create the Verifiable Presentation (VP) with the selected credentials
    val holder = DidJwk.create() // The holder of the Verifiable Presentation
    val holderUri = holder.did.uri

    // Additional data includes the presentation submission, which links the presentation to the PD
    val additionalData = mapOf(
      "presentation_submission" to presentationResult.presentationSubmission
    )

    val vpCreateOptions = VerifiablePresentationCreateOptions(
      additionalProperties = additionalData
    )

    // Generate the Verifiable Presentation
    val vp = VerifiablePresentation.create(
      holderUri,
      presentationResult.matchedVcJwts,
      vpCreateOptions
    )

    // Step 6: Sign the VP to generate a JWT format
    val signedVpJwt = vp.sign(holder)

    // Step 7: Decode and verify the signed VP to ensure correctness
    val decodedVp = VerifiablePresentation.fromVpJwt(signedVpJwt, true)

    // Step 8: Verify the holder matches the expected holder
    assertEquals(holderUri, decodedVp.holder)

    // Step 9: Verify that the correct Verifiable Credential was included in the presentation
    assertEquals(1, decodedVp.verifiableCredential.size)
    assertEquals(vcJwt, decodedVp.verifiableCredential[0])

    // Step 10: Retrieve the presentation_submission from the decoded VP's additional data
    val decodedPresentationSubmissionMap = decodedVp.additionalProperties?.get("presentation_submission") as? Map<*, *>

    // Step 11: Convert the map back to PresentationSubmission
    val jsonPresentationSubmission = Json.jsonMapper.writeValueAsString(decodedPresentationSubmissionMap)
    val decodedPresentationSubmission = Json.jsonMapper.readValue(jsonPresentationSubmission, PresentationSubmission::class.java)

    // Step 12: Verify that the presentation_submission in additional_data matches the original one
    assertEquals(presentationResult.presentationSubmission, decodedPresentationSubmission)
  }
}