import Foundation
import UniFFI

let jwk = UniFFI.Jwk(alg: "ES256K", kty: "EC", crv: "secp256k1", d: "P3hRuve79GaggsVdQG_w-JpdM6dHXG33-1nwZ8Jw07g", x: "vA8umEbOhhQjFfk1-byvVxtJNRtwQSEE0UMVmxSN9K4", y: "A1qGUBx-wpznzVI0DLu8kEhDZ77ou533NKSCw90R33Q")

let thumbprint = try jwk.computeThumbprint()
print("Thumbprint: \(thumbprint)")

let ed25519 = UniFFI.Ed25199()
let ed25519Jwk = try ed25519.generate()
let ed25519Thumbprint = try ed25519Jwk.computeThumbprint()
print("Thumbprint (Ed25519): \(ed25519Thumbprint)")

let payload = Data("hello world".utf8)
let signature = try ed25519.sign(jwk: ed25519Jwk, payload: payload)
print("Signature: \(signature.base64EncodedString())")

try ed25519.verify(jwk: ed25519Jwk, payload: payload, signature: signature)
print("verify() passed as expected")

do {
    try ed25519.verify(jwk: ed25519Jwk, payload: payload, signature: Data("invalid sig".utf8))
} catch {
    print("verify() failed as expected")
}

let keyManager = UniFFI.LocalJwkManager()
let keyAlias = try keyManager.generatePrivateKey(curve: Curve.ed25519, keyAlias: nil)
print("key alias \(keyAlias)")
let publicKey = try keyManager.getPublicKey(keyAlias: keyAlias)
print("public key \(publicKey)")
let signature2 = try keyManager.sign(keyAlias: keyAlias, payload: payload)
try ed25519.verify(jwk: publicKey, payload: payload, signature: signature2)
print("signed & verified \(signature2.base64EncodedString())")
let privateKeys = try keyManager.exportPrivateKeys()
print("private keys \(privateKeys)")
try keyManager.importPrivateKeys(privateKeys: privateKeys)
print("imported private keys")

let identifier = try UniFFI.Identifier(didUri: "did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1")
print(identifier)
