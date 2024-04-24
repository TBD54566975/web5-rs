import Foundation
import Web5

let keyManager = LocalKeyManager.newInMemory()
let keyAlias = try keyManager.generatePrivateKey(curve: Curve.ed25519, keyAlias: nil)
let payload = "hello world".data(using: .utf8)!.map { $0 }
let signature = try keyManager.sign(keyAlias: keyAlias, payload: payload)
let publicKey = try keyManager.getPublicKey(keyAlias: keyAlias)
try publicKey.verify(payload: payload, signature: Array(signature))
print("Success!")