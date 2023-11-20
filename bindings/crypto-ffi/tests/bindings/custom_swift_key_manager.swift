#if canImport(crypto_ffi)
    import crypto_ffi
#endif

class SwiftKeyStore: KeyStore {
  public private(set) var map = [String: PrivateKey]()

  func getPrivateKey(keyAlias: String) throws -> PrivateKey? {
    return self.map[keyAlias]
  }

  func insertPrivateKey(keyAlias: String, privateKey: PrivateKey) throws {
    self.map[keyAlias] = privateKey
  }
}

let keyManager = KeyManager(keyStore: SwiftKeyStore())