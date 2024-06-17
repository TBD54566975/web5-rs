package web5.sdk.dids.methods.dht

import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Signer
import web5.sdk.dids.Did
import web5.sdk.dids.DidDocument
import web5.sdk.dids.DidResolutionResult

import web5.sdk.rust.didDhtResolve as rustCoreDidDhtResolve
import web5.sdk.rust.DidDht as RustCoreDidDht

class DidDht {
    val did: Did
    val document: DidDocument

    private val rustCoreDidDht: RustCoreDidDht

    constructor(identityKey: Jwk) {
        rustCoreDidDht = RustCoreDidDht.fromIdentityKey(identityKey.toBinding())

        this.did = Did.fromBinding(rustCoreDidDht.getData().did)
        this.document = DidDocument.fromBinding(rustCoreDidDht.getData().document)
    }

    constructor(uri: String) {
        rustCoreDidDht = RustCoreDidDht.fromUri(uri)

        this.did = Did.fromBinding(rustCoreDidDht.getData().did)
        this.document = DidDocument.fromBinding(rustCoreDidDht.getData().document)
    }

    fun publish(signer: Signer) {
        // TODO: Implement publish method
    }

    fun deactivate(signer: Signer) {
        // TODO: Implement deactivate method
    }

    companion object {
        @JvmStatic
        fun resolve(uri: String): DidResolutionResult {
            val rustCoreResolutionObject = rustCoreDidDhtResolve(uri).getData()
            return DidResolutionResult.fromBinding(rustCoreResolutionObject)
        }
    }
}