import SwiftUI
import web5_uniffiFFI;

struct ContentView: View {
  private let keyManager = LocalKeyManager(keyStore: SwiftLocalKeyStore())

  @State var didUri: String = ""

  var body: some View {
    VStack {
      Button("Generate did:jwk") {
        let did = DidJwk(keyAlgorithm: .secp256k1, keyManager: keyManager as! KeyManager)
        
      }
    }
    .padding()
  }
}

class SwiftLocalKeyStore: LocalKeyStore {
  var store = [String: PrivateKey]()

  func get(key: String) throws -> PrivateKey? {
    return store[key]
  }

  func insert(key: String, value: PrivateKey) throws {
    store[key] = value
  }
}
