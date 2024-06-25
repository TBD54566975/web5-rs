package web5.sdk.vc.pex

import web5.sdk.rust.ConstraintsData as RustCoreConstraintsData
import web5.sdk.rust.FieldData as RustCoreFieldData
import web5.sdk.rust.FilterData as RustCoreFilterData
import web5.sdk.rust.Optionality as RustCoreOptionality
import web5.sdk.rust.InputDescriptorData as RustCoreInputDescriptorData

import web5.sdk.rust.PresentationDefinition as RustCorePresentationDefinition
import web5.sdk.rust.PresentationDefinitionData as RustCorePresentationDefinitionData

typealias ConstraintsData = RustCoreConstraintsData
typealias FieldData = RustCoreFieldData
typealias FilterData = RustCoreFilterData
typealias Optionality = RustCoreOptionality
typealias InputDescriptor = RustCoreInputDescriptorData

class PresentationDefinition {
    val id: String
    val name: String?
    val purpose: String?
    val inputDescriptors: List<InputDescriptor>

    internal val rustCorePresentationDefinition: RustCorePresentationDefinition

    constructor(id: String, name: String? = null, purpose: String? = null, inputDescriptors: List<InputDescriptor>) {
        this.id = id
        this.name = name
        this.purpose = purpose
        this.inputDescriptors = inputDescriptors

        this.rustCorePresentationDefinition = RustCorePresentationDefinition(
            RustCorePresentationDefinitionData(id, name, purpose, inputDescriptors)
        )
    }

    constructor(rustCorePresentationDefinitionData: RustCorePresentationDefinitionData) {
        this.id = rustCorePresentationDefinitionData.id
        this.name = rustCorePresentationDefinitionData.name
        this.purpose = rustCorePresentationDefinitionData.purpose
        this.inputDescriptors = rustCorePresentationDefinitionData.inputDescriptors

        this.rustCorePresentationDefinition = RustCorePresentationDefinition(rustCorePresentationDefinitionData)
    }

    fun selectCredentials(vcJwts: List<String>): List<String> {
        return this.rustCorePresentationDefinition.selectCredentials(vcJwts)
    }
}