package web5.sdk.dids.methods.jwk

import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.Did
import web5.sdk.dids.DidDocument
import web5.sdk.dids.DidResolutionResult

import web5.sdk.rust.didJwkResolve as rustCoreDidJwkResolve
import web5.sdk.rust.DidJwk as RustCoreDidJwk

class DidJwk {
    val did: Did
    val document: DidDocument

    constructor(publicKey: Jwk) {
        val rustCoreDidJwk = RustCoreDidJwk.fromPublicJwk(publicKey.toBinding())

        this.did = Did.fromBinding(rustCoreDidJwk.getData().did)
        this.document = DidDocument.fromBinding(rustCoreDidJwk.getData().document)
    }

    constructor(uri: String) {
        val rustCoreDidJwk = RustCoreDidJwk.fromUri(uri)

        this.did = Did.fromBinding(rustCoreDidJwk.getData().did)
        this.document = DidDocument.fromBinding(rustCoreDidJwk.getData().document)
    }

    companion object {
        @JvmStatic
        fun resolve(uri: String): DidResolutionResult {
            val rustCoreResolutionObject = rustCoreDidJwkResolve(uri).getData()
            return DidResolutionResult.fromBinding(rustCoreResolutionObject);
        }
    }
}