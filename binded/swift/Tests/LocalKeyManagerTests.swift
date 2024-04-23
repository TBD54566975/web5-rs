import XCTest
@testable import Web5

final class LocalKeyManagerTests: XCTestCase {
    func testCanGenerateEd25519Key() throws {
        let keyManager = LocalKeyManager.newInMemory()
        let keyAlias = try keyManager.generatePrivateKey(curve: Curve.ed25519, keyAlias: nil)
        XCTAssertNotEqual(keyAlias.count, 0)
    }
    
    func testCanGenerateSecp256k1Key() throws {
        let keyManager = LocalKeyManager.newInMemory()
        let keyAlias = try keyManager.generatePrivateKey(curve: Curve.secp256k1, keyAlias: nil)
        XCTAssertNotEqual(keyAlias.count, 0)
    }
    
    func testCanSignAndVerify() throws {
        let keyManager = LocalKeyManager.newInMemory()
        let keyAlias = try keyManager.generatePrivateKey(curve: Curve.ed25519, keyAlias: nil)
        XCTAssertNotEqual(keyAlias.count, 0)
        
        let payload = "hello world".data(using: .utf8)!.map { $0 }
        let signature = try keyManager.sign(keyAlias: keyAlias, payload: payload)
        XCTAssertNotEqual(signature.count, 0)
        
        let publicKey = try keyManager.getPublicKey(keyAlias: keyAlias)
        XCTAssertNoThrow(try publicKey.verify(payload: payload, signature: Array(signature)))
    }
    
    func testCanExportAndImportKeys() throws {
        let keyManager = LocalKeyManager.newInMemory()
        _ = try keyManager.generatePrivateKey(curve: Curve.secp256k1, keyAlias: nil)
        
        let exportedPrivateKeys = try keyManager.exportPrivateKeys()
        XCTAssertEqual(exportedPrivateKeys.count, 1)
        
        XCTAssertNoThrow(try keyManager.importPrivateKeys(privateKeys: exportedPrivateKeys))
    }
}