import SwiftUI
import crypto_ffiFFI
import dids_ffiFFI

func createKeyManager() -> KeyManager {
  let keyStore = KeyStore.newInMemory()
  let keyManager = KeyManager.newWithKeyStore(keyStore: keyStore)
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
      Button("Reset KeyManager") {
        keyManager = createKeyManager()
      }
    }
    .padding()
  }
}

class SwiftKeyStore: KeyStore {
  public private(set) var map = [String: PrivateKey]()

  func getPrivateKey(keyAlias: String) throws -> PrivateKey? {
    return self.map[keyAlias]
  }
  
  func insertPrivateKey(keyAlias: String, privateKey: PrivateKey) throws {
    self.map[keyAlias] = privateKey
  }
}
