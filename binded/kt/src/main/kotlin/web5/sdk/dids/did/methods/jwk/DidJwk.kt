package web5.sdk.dids.did.methods.jwk

import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.did.methods.Did
import web5.sdk.dids.did.methods.DidDocument
import web5.sdk.dids.did.methods.DidResolutionResult

//CLASS DidJwk
//PUBLIC DATA did: Did
//PUBLIC DATA document: Document
//CONSTRUCTOR(public_key: Jwk)
//CONSTRUCTOR(uri: string)
//STATIC METHOD resolve(uri: string): ResolutionResult

public class DidJwk {
    public val did: Did
    public val document: DidDocument

    constructor(publicKey: Jwk) {
        val didJwk = web5.sdk.DidJwk.fromPublicKey(publicKey.toBinded());

        this.did = Did.fromBinded(didJwk.getData().did)
        this.document = DidDocument.fromBinded(didJwk.getData().document)
    }

//    constructor(uri: String) {
//        val resolutionResult = resolve(uri)
//        this.did = resolutionResult.
//        this.document = resolutionResult.didDocument!!
//    }

    companion object {
        @JvmStatic
        fun resolve(uri: String): DidResolutionResult {
            return TODO("Provide the return value")
        }
    }
}