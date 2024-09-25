package com.example

import web5.sdk.dids.methods.jwk.DidJwk
import web5.sdk.vc.*
import web5.sdk.vc.pex.*
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue

fun main(args: Array<String>) {
    // Initialize the JSON mapper
    val jsonMapper = jacksonObjectMapper()

    // Step 1: Create an issuer DID (Decentralized Identifier)
    // Typically, this is the entity that issues the Verifiable Credential (VC)
    val issuer = DidJwk.create()
    val issuerUri = issuer.did.uri
    println("Issuer DID URI: $issuerUri")

    // Step 2: Define a Presentation Definition (PD)
    // This specifies the requirements that credentials must meet to be accepted
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
                        pattern = "^did:jwk:.*$" // Regex pattern to match DID JWK
                    ),
                    optional = false,
                    predicate = Optionality.Required
                )
            )
        )
    )

    // Create the Presentation Definition with the InputDescriptor
    val presentationDefinition = PresentationDefinition(
        id = "test_presentation_definition",
        name = "Test Presentation Definition",
        purpose = "Testing Presentation Exchange",
        inputDescriptors = listOf(inputDescriptor)
    )
    println("Presentation Definition created: $presentationDefinition")

    // Step 3: Create a Verifiable Credential (VC) that meets the PD criteria
    // The credentialSubject.id matches the pattern specified in the PD
    val vc = VerifiableCredential.create(
        Issuer.StringIssuer(issuerUri),
        CredentialSubject(issuerUri)
    )

    // Sign the VC using the issuer's DID
    val vcJwt = vc.sign(issuer)
    println("Verifiable Credential JWT created and signed:\n$vcJwt\n")

    // Step 4: Select credentials that match the PD's input descriptors
    val vcJwts = listOf(vcJwt)
    val presentationResult = presentationDefinition.createPresentationFromCredentials(vcJwts)
    println("Presentation Result after matching credentials:\n$presentationResult\n")

    // Step 5: Create a Verifiable Presentation (VP) with the selected credentials
    // The holder is the entity presenting the credentials
    val holder = DidJwk.create()
    val holderUri = holder.did.uri
    println("Holder DID URI: $holderUri")

    // Include the presentation submission data to link the presentation to the PD
    val additionalData = mapOf(
        "presentation_submission" to presentationResult.presentationSubmission
    )

    val vpCreateOptions = VerifiablePresentationCreateOptions(
        additionalProperties = additionalData
    )

    // Generate the VP with the matched credentials and additional data
    val vp = VerifiablePresentation.create(
        holderUri,
        presentationResult.matchedVcJwts,
        vpCreateOptions
    )
    println("Verifiable Presentation created:\n$vp\n")

    // Step 6: Sign the VP to generate a JWT format presentation
    val signedVpJwt = vp.sign(holder)
    println("Signed Verifiable Presentation JWT:\n$signedVpJwt\n")

    // Step 7: Decode and verify the signed VP to ensure correctness
    val decodedVp = VerifiablePresentation.fromVpJwt(signedVpJwt, true)
    println("Decoded Verifiable Presentation:\n$decodedVp\n")

    // Step 8: Print the holder URI to verify it matches the expected holder
    println("Decoded VP Holder URI: ${decodedVp.holder}\n")

    // Step 9: Print the Verifiable Credentials included in the presentation
    println("Verifiable Credentials in VP:")
    decodedVp.verifiableCredential.forEach { credential ->
        println(credential)
    }
    println()

    // Step 10: Retrieve the presentation_submission from the decoded VP's additional data
    val decodedPresentationSubmissionMap = decodedVp.additionalProperties?.get("presentation_submission") as? Map<*, *>
    println("Presentation Submission from decoded VP:\n$decodedPresentationSubmissionMap\n")

    // Step 11: Convert the map back to PresentationSubmission
    val jsonPresentationSubmission = jsonMapper.writeValueAsString(decodedPresentationSubmissionMap)
    val decodedPresentationSubmission = jsonMapper.readValue<PresentationSubmission>(jsonPresentationSubmission)
    println("Decoded Presentation Submission object:\n$decodedPresentationSubmission\n")

    // Step 12: Verify that the presentation_submission in additional_data matches the original one
    if (decodedPresentationSubmission == presentationResult.presentationSubmission) {
        println("Presentation submissions match.")
    } else {
        println("Presentation submissions do not match.")
    }
}
