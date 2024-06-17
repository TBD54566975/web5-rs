package web5.sdk.dids

import web5.sdk.rust.DidData as RustCoreDidData

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
    fun toBinding(): RustCoreDidData {
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
        fun fromBinding(didData: RustCoreDidData): Did {
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