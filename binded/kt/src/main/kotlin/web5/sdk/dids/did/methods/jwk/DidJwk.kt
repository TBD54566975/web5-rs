package web5.sdk.dids.did.methods.jwk

import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.did.methods.Did
import web5.sdk.dids.did.methods.DidDocument
import web5.sdk.dids.did.methods.DidResolutionResult

import web5.sdk.DidJwk as RcbDidJwk

class DidJwk {
    val did: Did
    val document: DidDocument

    constructor(publicKey: Jwk) {
        val didJwk = RcbDidJwk.fromPublicKey(publicKey.toBinding());

        this.did = Did.fromBinding(didJwk.getData().did)
        this.document = DidDocument.fromBinding(didJwk.getData().document)
    }

    constructor(uri: String) {
        val resolutionResult = resolve(uri)
        val verificationMethod = resolutionResult.didDocument?.verificationMethod?.get(0)
        val publicKey = verificationMethod?.publicKeyJwk!!

        val rcbDidJwk = RcbDidJwk.fromPublicKey(publicKey.toBinding());

        this.did = Did.fromBinding(rcbDidJwk.getData().did)
        this.document = DidDocument.fromBinding(rcbDidJwk.getData().document)
    }

    companion object {
        @JvmStatic
        fun resolve(uri: String): DidResolutionResult {
            return TODO("Provide the return value")
        }
    }
}