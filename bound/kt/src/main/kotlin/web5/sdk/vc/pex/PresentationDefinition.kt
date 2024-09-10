package web5.sdk.vc.pex

import web5.sdk.Json
import web5.sdk.Web5Exception
import web5.sdk.rust.PresentationDefinition as RustCorePresentationDefinition
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

data class PresentationDefinition(
    val id: String,
    val name: String?,
    val purpose: String?,
    val inputDescriptors: List<InputDescriptor>,
val submissionRequirements: List<SubmissionRequirement>? = null
) {
    internal val rustCorePresentationDefinition = RustCorePresentationDefinition(
        Json.stringify(this)
    )

    fun selectCredentials(vcJwts: List<String>): List<String> {
        try {
            return this.rustCorePresentationDefinition.selectCredentials(vcJwts)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }

    fun createPresentationFromCredentials(vcJwts: List<String>): PresentationResult {
        try {
            val rustCoreJsonSerializedPresentationResult = this.rustCorePresentationDefinition.createPresentationFromCredentials(vcJwts)
            return Json.jsonMapper.readValue(rustCoreJsonSerializedPresentationResult, PresentationResult::class.java)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}

data class InputDescriptor(
    val id: String,
    val name: String? = null,
    val purpose: String? = null,
    val constraints: Constraints,
)

data class Constraints(
    val fields: List<Field>
)

data class Field(
    val id: String? = null,
    val name: String? = null,
    val path: List<String>,
    val purpose: String? = null,
    val filter: Filter? = null,
    val optional: Boolean? = false,
    val predicate: Optionality? = null
)

enum class Optionality {
    Required,
    Preferred
}

data class Filter(
    val type: String? = null,
    val pattern: String? = null,
    val const: String? = null,
    val contains: Filter? = null
)

data class SubmissionRequirement(
    val rule: SubmissionRequirementRule,
    val from: String? = null,
    val fromNested: List<SubmissionRequirement>? = null,
    val name: String? = null,
    val purpose: String? = null,
    val count: Int? = null,
    val min: Int? = null,
    val max: Int? = null
)

enum class SubmissionRequirementRule {
    All,
    Pick
}

data class PresentationResult(
    val presentationSubmission: PresentationSubmission,
    val matchedVcJwts: List<String>
)

data class PresentationSubmission(
    val id: String,
    val definitionId: String,
    val descriptorMap: List<InputDescriptorMapping>
)

data class InputDescriptorMapping(
    val id: String,
    val format: String,
    val path: String,
    val pathNested: InputDescriptorMapping? = null
)