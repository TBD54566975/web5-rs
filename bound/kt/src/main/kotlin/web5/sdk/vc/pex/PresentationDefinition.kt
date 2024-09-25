package web5.sdk.vc.pex

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.Json
import web5.sdk.Web5Exception
import web5.sdk.rust.PresentationDefinition as RustCorePresentationDefinition
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents a DIF Presentation Definition.
 *
 * The `PresentationDefinition` outlines the criteria that Verifiable Credentials (VCs) must meet
 * for a presentation exchange. It includes input descriptors, submission requirements, and additional metadata.
 *
 * @property id The unique identifier for the Presentation Definition.
 * @property name The name of the Presentation Definition (optional).
 * @property purpose The purpose of the Presentation Definition (optional).
 * @property inputDescriptors A list of input descriptors that define the criteria for acceptable VCs.
 * @property submissionRequirements A list of submission requirements (optional).
 */
data class PresentationDefinition(
    val id: String,
    val name: String?,
    val purpose: String?,
    @JsonProperty("input_descriptors")
    val inputDescriptors: List<InputDescriptor>,
    @JsonProperty("submission_requirements")
    val submissionRequirements: List<SubmissionRequirement>? = null
) {
    internal val rustCorePresentationDefinition = RustCorePresentationDefinition(
        Json.stringify(this)
    )

    /**
     * Selects Verifiable Credentials (VCs) that match the input descriptors of the Presentation Definition.
     *
     * @param vcJwts A list of VC JWTs to validate against the input descriptors.
     * @return A list of VC JWTs that satisfy the input descriptors.
     * @throws Web5Exception If an error occurs during selection.
     *
     * @example
     * ```
     * val matchedVcs = presentationDefinition.selectCredentials(vcJwtList)
     * println(matchedVcs)  // Output: List of matched VC JWTs
     * ```
     */
    fun selectCredentials(vcJwts: List<String>): List<String> {
        try {
            return this.rustCorePresentationDefinition.selectCredentials(vcJwts)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }

    /**
     * Creates a presentation submission from the selected Verifiable Credentials (VCs).
     *
     * @param vcJwts A list of VC JWTs to create the presentation from.
     * @return A `PresentationResult` containing the presentation submission and matched VCs.
     * @throws Web5Exception If an error occurs during the creation process.
     *
     * @example
     * ```
     * val presentationResult = presentationDefinition.createPresentationFromCredentials(vcJwtList)
     * println(presentationResult.presentationSubmission.id)  // Output: Presentation submission ID
     * ```
     */
    fun createPresentationFromCredentials(vcJwts: List<String>): PresentationResult {
        try {
            val rustCoreJsonSerializedPresentationResult = this.rustCorePresentationDefinition.createPresentationFromCredentials(vcJwts)
            return Json.jsonMapper.readValue(rustCoreJsonSerializedPresentationResult, PresentationResult::class.java)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}

/**
 * Represents an input descriptor, which specifies the criteria for acceptable Verifiable Credentials (VCs).
 *
 * @property id The unique identifier for the input descriptor.
 * @property name The name of the input descriptor (optional).
 * @property purpose The purpose of the input descriptor (optional).
 * @property constraints The constraints that define acceptable fields and filters for the VCs.
 */
data class InputDescriptor(
    val id: String,
    val name: String? = null,
    val purpose: String? = null,
    val constraints: Constraints,
)

/**
 * Contains the constraints for a given input descriptor.
 *
 * @property fields A list of fields that define the acceptable values and structure for the Verifiable Credentials (VCs).
 */
data class Constraints(
    val fields: List<Field>
)

/**
 * Represents a field within a Verifiable Credential (VC) that must match certain criteria.
 *
 * @property id The unique identifier for the field (optional).
 * @property name The name of the field (optional).
 * @property path The JSON path to the field within the VC.
 * @property purpose The purpose of the field (optional).
 * @property filter A filter that defines acceptable values for the field (optional).
 * @property optional Indicates if the field is optional (defaults to false).
 * @property predicate Indicates if the field is required or preferred.
 */
data class Field(
    val id: String? = null,
    val name: String? = null,
    val path: List<String>,
    val purpose: String? = null,
    val filter: Filter? = null,
    val optional: Boolean? = false,
    val predicate: Optionality? = null
)

/**
 * Defines whether a field is required or preferred.
 */
enum class Optionality {
    Required,
    Preferred
}

/**
 * Represents a filter applied to a field within a Verifiable Credential (VC).
 *
 * @property type The type of the field (e.g., string, integer) (optional).
 * @property pattern A regular expression pattern that the field's value must match (optional).
 * @property const A constant value that the field's value must match (optional).
 * @property contains A nested filter applied to the field (optional).
 */
data class Filter(
    val type: String? = null,
    val pattern: String? = null,
    val const: String? = null,
    val contains: Filter? = null
)

/**
 * Represents a submission requirement, which defines how input descriptors must be satisfied.
 *
 * @property rule The rule that defines how input descriptors must be selected (e.g., All or Pick).
 * @property from The source of the input descriptors (optional).
 * @property fromNested A nested list of submission requirements (optional).
 * @property name The name of the submission requirement (optional).
 * @property purpose The purpose of the submission requirement (optional).
 * @property count The exact number of input descriptors required (optional).
 * @property min The minimum number of input descriptors required (optional).
 * @property max The maximum number of input descriptors allowed (optional).
 */
data class SubmissionRequirement(
    val rule: SubmissionRequirementRule,
    val from: String? = null,
    @JsonProperty("from_nested")
    val fromNested: List<SubmissionRequirement>? = null,
    val name: String? = null,
    val purpose: String? = null,
    val count: Int? = null,
    val min: Int? = null,
    val max: Int? = null
)

/**
 * Defines the selection rule for input descriptors.
 */
enum class SubmissionRequirementRule {
    All,
    Pick
}

/**
 * Represents the result of a presentation submission.
 *
 * @property presentationSubmission The `PresentationSubmission` object containing the submission details.
 * @property matchedVcJwts A list of matched VC JWTs that satisfy the input descriptors.
 */
data class PresentationResult(
    val presentationSubmission: PresentationSubmission,
    val matchedVcJwts: List<String>
)

/**
 * Represents the presentation submission, which links input descriptors to the matched Verifiable Credentials (VCs).
 *
 * @property id The unique identifier for the presentation submission.
 * @property definitionId The identifier of the Presentation Definition.
 * @property descriptorMap A list of mappings between input descriptors and Verifiable Credentials (VCs).
 */
data class PresentationSubmission(
    val id: String,
    @JsonProperty("definition_id")
    val definitionId: String,
    @JsonProperty("descriptor_map")
    val descriptorMap: List<InputDescriptorMapping>
)

/**
 * Maps input descriptors to Verifiable Credentials (VCs) in a presentation submission.
 *
 * @property id The unique identifier of the input descriptor.
 * @property format The format of the Verifiable Credential (e.g., jwt_vc).
 * @property path The JSON path to the Verifiable Credential within the presentation submission.
 * @property pathNested A nested mapping for deeper structures (optional).
 */
data class InputDescriptorMapping(
    val id: String,
    val format: String,
    val path: String,
    @JsonProperty("path_nested")
    val pathNested: InputDescriptorMapping? = null
)
