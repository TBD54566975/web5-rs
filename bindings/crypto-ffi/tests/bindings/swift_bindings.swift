import crypto_ffi

//class SwiftKeyStore: KeyStoreTrait {
//  public private(set) var map = [String: PrivateKey]()
//
//  func get(keyAlias: String) throws -> PrivateKey? {
//    return self.map[keyAlias]
//  }
//
//  func insert(keyAlias: String, privateKey: PrivateKey) throws {
//    self.map[keyAlias] = privateKey
//  }
//}

//let keyStore = KeyStore(trait: SwiftKeyStore())
//keyStore.insert(keyAlias: "key1", privateKey: PrivateKey())

let keyManager = KeyManager.inMemory();
let key_alias = keyManager.generatePrivateKey(keyType: KeyType.secp256k1);
print(key_alias);