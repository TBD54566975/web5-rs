import SwiftUI
import web5_uniffiFFI;

func createKeyManager() -> KeyManager {
  return KeyManager(keyStore: InMemoryKeyStore())
}

var keyManager = createKeyManager()

struct ContentView: View {
  var body: some View {
    VStack {
      Button("Generate Key Only") {
        let keyAlias = try! keyManager.generatePrivateKey(keyAlgorithm: .ed25519)
        let key = try! keyManager.getPublicKey(keyAlias: keyAlias)
        print("Generated Key: \(key!.toJson())")
      }
      Button("Generate did:jwk") {
        Task {
          let did = DidJwk(keyAlgorithm: .secp256k1, keyManager: keyManager)
          print("Generated did:jwk: \(did.getUri())")
          let resolution = try! await resolve(didUri: did.getUri())
          print("Resolved DIDDocument: \(resolution.didDocument)")
        }
      }
      Button("Generate did:key") {
        Task {
          let did = DidKey(keyAlgorithm: .secp256r1, keyManager: keyManager)
          print("Generated did:key: \(did.getUri())")
          let resolution = try! await resolve(didUri: did.getUri())
          print("Resolved DIDDocument: \(resolution.didDocument)")
        }

      }
      Button("Print KeyStore keys") {
        let allPrivateKeys = try! keyManager.getKeyStore().dump()
        for pk in allPrivateKeys {
          print("Key: \(pk.toJson())")
        }
        print("Total key count: \(allPrivateKeys.count)")
      }
      Button("Reset KeyManager") {
        keyManager = createKeyManager()
      }
    }
    .padding()
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

  deinit {
    print("SwiftKeyStore deallocated!")
  }
}
