package web5.sdk.vc.pex

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.Json
import web5.sdk.rust.PresentationDefinition as RustCorePresentationDefinition

data class PresentationDefinition(
    val id: String,
    val name: String?,
    val purpose: String?,
    @JsonProperty("input_descriptors")
    val inputDescriptors: List<InputDescriptor>
) {
    internal val rustCorePresentationDefinition = RustCorePresentationDefinition(
        Json.stringify(this)
    )

    fun selectCredentials(vcJwts: List<String>): List<String> {
        return this.rustCorePresentationDefinition.selectCredentials(vcJwts)
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