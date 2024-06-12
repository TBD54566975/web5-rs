package web5.sdk.dids.did.methods

import web5.sdk.DidData

public data class Did(
    val uri: String,
    val url: String,
    val method: String,
    val id: String,
    val params: Map<String, String> = emptyMap(),
    val path: String? = null,
    val query: String? = null,
    val fragment: String? = null
) {
    fun toBinded(): DidData {
        return DidData(
            uri = this.uri,
            url = this.url,
            method = this.method,
            id = this.id,
            params = if (this.params.isEmpty()) null else this.params,
            path = this.path,
            query = this.query,
            fragment = this.fragment
        )
    }

    companion object {
        fun fromBinded(didData: DidData): Did {
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