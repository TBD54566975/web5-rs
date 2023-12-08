import SwiftUI
import crypto_ffiFFI
//import dids_ffiFFI

func createKeyManager() -> KeyManager {
  let keyStore = KeyStore.inMemory()
  let keyManager = KeyManager.withKeyStore(keyStore: keyStore)
  return keyManager
}

var keyManager = createKeyManager()

struct ContentView: View {
  var body: some View {
    VStack(spacing: 10) {
//      Button("Generate did:jwk") {
//        Task {
//          let didJwk = try! DidJwk(keyManager: keyManager, options: .init(keyAlgorithm: .secp256k1))
//          didJwk.uri().hasPrefix("did:jwk:")
//          print("didJwk uri: \(didJwk.uri())")
//        }
//      }
//      Button("Generate did:key") {
//        Task {
//          let didKey = try! DidKey(keyManager: keyManager, options: .init(keyAlgorithm: .secp256k1))
//          print("didKey uri: \(didKey.uri())")
//        }
//      }
      Button("Test") {
        print("testing: \(keyManager.test())")
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
