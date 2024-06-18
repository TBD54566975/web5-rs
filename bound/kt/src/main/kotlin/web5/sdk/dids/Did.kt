package web5.sdk.dids

import web5.sdk.rust.DidData as RustCoreDidData

/**
 * Representation of a [DID Core Identifier](https://www.w3.org/TR/did-core/#identifiers).
 *
 * @property uri Represents the complete Decentralized Identifier (DID) URI.
 * @property url Represents the DID URI + A network location identifier for a specific resource.
 * @property method Specifies the DID method in the URI, indicating the underlying method-specific identifier scheme.
 * @property id The method-specific identifier in the DID URI.
 * @property params A map containing optional parameters present in the DID URI. These parameters are method-specific.
 * @property path An optional path component in the DID URI.
 * @property query An optional query component in the DID URI, used to express a request for a specific representation or resource related to the DID.
 * @property fragment An optional fragment component in the DID URI, used to reference a specific part of a DID document.
 */
data class Did(
    val uri: String,
    val url: String,
    val method: String,
    val id: String,
    val params: Map<String, String> = emptyMap(),
    val path: String? = null,
    val query: String? = null,
    val fragment: String? = null
) {
    /**
     * Converts the Did instance to a RustCoreDidData binding.
     *
     * @return RustCoreDidData the corresponding RustCoreDidData object.
     */
    fun toRustCore(): RustCoreDidData {
        return RustCoreDidData(
            uri = this.uri,
            url = this.url,
            method = this.method,
            id = this.id,
            params = this.params.ifEmpty { null },
            path = this.path,
            query = this.query,
            fragment = this.fragment
        )
    }

    companion object {
        /**
         * Creates a Did instance from a RustCoreDidData binding.
         *
         * @param didData the RustCoreDidData object.
         * @return Did the corresponding Did instance.
         */
        fun fromRustCore(didData: RustCoreDidData): Did {
            return Did(
                uri = didData.uri,
                url = didData.url,
                method = didData.method,
                id = didData.id,
                params = didData.params ?: emptyMap(),
                path = didData.path,
                query = didData.query,
                fragment = didData.fragment
            )
        }
    }
}
