import SwiftUI
import crypto_ffiFFI
import dids_ffiFFI

func createKeyManager() -> KeyManager {
  let keyManager = KeyManager.newInMemory();
  return keyManager
}

func createCustomKeyManager() -> KeyManager {
  let keyManager = KeyManager(keyStore: SwiftKeyStore())
  return keyManager
}

var keyManager = createKeyManager()

struct ContentView: View {
  var body: some View {
    VStack(spacing: 10) {
      Button("Generate did:jwk") {
        Task {
          let didJwk = try! DidJwk(keyManager: keyManager, options: .init(keyType: .secp256k1))
          print("didJwk uri: \(didJwk.uri())")
        }
      }
      Button("Generate did:key") {
        Task {
          let didKey = try! DidKey(keyManager: keyManager, options: .init(keyType: .secp256k1))
          print("didKey uri: \(didKey.uri())")
        }
      }
      Button("Generate privateKey") {
        Task {
          let key_alias = try! keyManager.generatePrivateKey(keyType: .ed25519);
          print("generated private key with alias: \(key_alias)")
        }
      }
      Button("New In-Memory KeyManager") {
        keyManager = createKeyManager()
      }
      Button("New Custom Key Manager") {
        keyManager = createCustomKeyManager()
      }
    }
    .padding()
  }
}

class SwiftKeyStore: KeyStore {
  public private(set) var map = [String: PrivateKey]()

  func get(keyAlias: String) throws -> PrivateKey? {
    return self.map[keyAlias]
  }

  func insert(keyAlias: String, privateKey: PrivateKey) throws {
    self.map[keyAlias] = privateKey
    print("Inserted. Map now: \(self.map)")
  }
}
