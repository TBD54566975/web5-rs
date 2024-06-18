package web5.sdk.crypto.keys

import web5.sdk.rust.JwkData as RustCoreJwkData

/**
 * Partial representation of a [JSON Web Key as per RFC7517](https://tools.ietf.org/html/rfc7517).
 * Note that this is a subset of the spec.
 */
typealias Jwk = RustCoreJwkData