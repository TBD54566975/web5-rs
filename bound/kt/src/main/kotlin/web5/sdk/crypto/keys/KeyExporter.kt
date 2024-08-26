package web5.sdk.crypto.keys

interface KeyExporter {
    fun exportPrivateJwks(): List<Jwk>
}

internal class ToInnerKeyExporter(private val keyExporter: KeyExporter) : web5.sdk.rust.KeyExporter {
    override fun exportPrivateJwks(): List<web5.sdk.rust.JwkData> {
        return keyExporter.exportPrivateJwks().map { it.rustCoreJwkData }
    }
}