import SwiftUI
import web5_uniffiFFI;

let keyManager = KeyManager(keyStore: SwiftKeyStore())

struct ContentView: View {
  var body: some View {
    VStack {
      Button("Generate Key Only") {
        let keyAlias = try! keyManager.generatePrivateKey(keyAlgorithm: .ed25519)
        let key = try! keyManager.getPublicKey(keyAlias: keyAlias)
        print("Generated Key: \(key!.toJson())")
      }
      Button("Generate did:jwk") {
        let did = DidJwk(keyAlgorithm: .secp256k1, keyManager: keyManager)
        print("Generated did:jwk: \(did.getUri())")
      }
      Button("Generate did:key") {
        let did = DidKey(keyAlgorithm: .secp256r1, keyManager: keyManager)
        print("Generated did:key: \(did.getUri())")
      }
      Button("Print KeyStore keys") {
        let allPrivateKeys = try! keyManager.getKeyStore().dump()
        for pk in allPrivateKeys {
          print("Key: \(pk.toJson())")
        }
        print("Total key count: \(allPrivateKeys.count)")
      }
    }
    .padding()
    .task {
      let didJwk = DidJwk(keyAlgorithm: .ed25519, keyManager: keyManager)
      let resolution = try! await resolve(didUri: didJwk.getUri())
      print("Resolution: \(resolution)")
    }
  }
}

extension InMemoryKeyStore: KeyStore {}

class SwiftKeyStore: KeyStore {
  var map = [String: PrivateKey]()

  func get(key: String) throws -> PrivateKey? {
    return map[key]
  }

  func insert(value: PrivateKey) throws -> String {
    let key = value.alias()
    map[key] = value
    return key
  }

  func dump() throws -> [PrivateKey] {
    return Array(map.values)
  }
}
