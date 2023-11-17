import SwiftUI
import web5_ffiFFI;

func createKeyManager() -> KeyManager {
  return KeyManager.inMemory()
//  return KeyManager.keyStore(keyStore: SwiftKeyStore())
}

var keyManager = createKeyManager()

struct ContentView: View {
  var body: some View {
    VStack(spacing: 10) {
//      Button("Generate a private key") {
//        let _ = try! keyManager.generatePrivateKey(keyAlgorithm: .secp256k1)
//      }
//      Button("Generate Key Only") {
//        let keyAlias = try! keyManager.generatePrivateKey(keyAlgorithm: .ed25519)
//        let key = try! keyManager.getPublicKey(keyAlias: keyAlias)
//        print("Generated Key: \(key!.toJson())")
//      }
      Button("Generate did:jwk") {
        Task {
          let didJwk = try! DidJwk(keyManager: keyManager, options: .init(keyAlgorithm: .ed25519))
          print("didJwk uri: \(didJwk.uri())")
        }
      }
      Button("Generate did:key") {
        Task {
          let didKey = try! DidKey(keyManager: keyManager, options: .init(keyAlgorithm: .secp256k1))
          print("didKey uri: \(didKey.uri())")
        }
      }
//      Button("Sign & Verify with new Key") {
//        let keyAlias = try! keyManager.generatePrivateKey(keyAlgorithm: .ed25519)
//
//        let payload = "Hello, world!".data(using: .utf8)!
//        let signature = keyManager.sign(keyAlias: keyAlias, payload: payload)
//
//        let publicKey = try! keyManager.getPublicKey(keyAlias: keyAlias)!;
//        let result = publicKey.verify(payload: payload, signature: signature)
//        print("Verification Result: \(result)")
//      }
//      Button("Sign & Verify with new different keys") {
//        let keyAlias = try! keyManager.generatePrivateKey(keyAlgorithm: .ed25519)
//        let payload = "Hello, world!".data(using: .utf8)!
//        let signature = keyManager.sign(keyAlias: keyAlias, payload: payload)
//
//        let keyAlias2 = try! keyManager.generatePrivateKey(keyAlgorithm: .ed25519)
//        let publicKey2 = try! keyManager.getPublicKey(keyAlias: keyAlias2)!;
//        let result = publicKey2.verify(payload: payload, signature: signature)
//        print("Verification Result: \(result)")
//      }
//      Button("Print KeyStore keys") {
//        let allPrivateKeys = try! keyManager.getKeyStore().dump()
//        for pk in allPrivateKeys {
//          print("Key: \(pk.toJson())")
//        }
//        print("Total key count: \(allPrivateKeys.count)")
//      }
      Button("Reset KeyManager") {
        keyManager = createKeyManager()
      }
    }
    .padding()
  }
}

//extension InMemoryKeyStore: KeyStore {}
//
//class SwiftKeyStore: KeyStore {
//  var map = [String: PrivateKey]()
//
//  func get(key: String) throws -> PrivateKey? {
//    return map[key]
//  }
//
//  func insert(value: PrivateKey) throws -> String {
//    let key = value.alias()
//    map[key] = value
//    return key
//  }
//
//  func dump() throws -> [PrivateKey] {
//    return Array(map.values)
//  }
//
//  deinit {
//    print("SwiftKeyStore deallocated!")
//  }
//}

class SwiftKeyStore: KeyStore {
  var map = [String: PrivateKey]()

  func getPrivateKey(keyAlias: String) throws -> PrivateKey? {
    return self.map[keyAlias]!
  }
  
  func insertPrivateKey(keyAlias: String, privateKey: PrivateKey) throws {
    self.map[keyAlias] = privateKey
    print("Map size: \(map.count)")
  }

  deinit {
    print("Deinit called")
  }

}
