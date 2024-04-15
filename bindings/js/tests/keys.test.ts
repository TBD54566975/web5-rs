import { LocalJwkManager } from '../pkg'

describe('LocalJwkManager', () => {
  let keyManager: LocalJwkManager

  beforeEach(() => {
    keyManager = new LocalJwkManager()
  })

  test('should override key_alias', () => {
    const keyAliasOverride = 'key-id-123'
    const keyAlias = keyManager.generate_private_key('Secp256k1', keyAliasOverride)
    expect(keyAlias).toEqual(keyAliasOverride)
  })

  test('should generate and retrieve Secp256k1 private key', () => {
    const keyAlias = keyManager.generate_private_key('Secp256k1')
    expect(typeof keyAlias).toBe('string')

    const publicKey = keyManager.get_public_key(keyAlias)
    expect(publicKey).toBeDefined()
    expect(publicKey.alg).toBe('ES256K')
    expect(publicKey.kty).toBe('EC')
    expect(publicKey.crv).toBe('secp256k1')
    expect(publicKey.x).toBeDefined()
    expect(publicKey.y).toBeDefined()
  })

  test('should generate and retrieve Ed25519 private key', () => {
    const keyAlias = keyManager.generate_private_key('Ed25519')
    expect(typeof keyAlias).toBe('string')

    const publicKey = keyManager.get_public_key(keyAlias)
    expect(publicKey).toBeDefined()
    expect(publicKey.alg).toBe('EdDSA')
    expect(publicKey.kty).toBe('OKP')
    expect(publicKey.crv).toBe('Ed25519')
    expect(publicKey.x).toBeDefined()
  })

  test('should sign and verify using Secp256k1', () => {
    const keyAlias = keyManager.generate_private_key('Secp256k1')
    const publicKey = keyManager.get_public_key(keyAlias)

    const payload = new TextEncoder().encode('test message')
    const signature = keyManager.sign(keyAlias, payload)
    expect(signature).toBeDefined()

    const verified = publicKey.verify(payload, signature)
    expect(verified).toBeUndefined()
  })

  test('should sign and verify using Ed25519', () => {
    const keyAlias = keyManager.generate_private_key('Ed25519')
    const publicKey = keyManager.get_public_key(keyAlias)

    const payload = new TextEncoder().encode('test message')
    const signature = keyManager.sign(keyAlias, payload)
    expect(signature).toBeDefined()

    const verified = publicKey.verify(payload, signature)
    expect(verified).toBeUndefined()
  })

  test('should export and import private keys', () => {
    const keyAlias1 = keyManager.generate_private_key('Secp256k1')
    const keyAlias2 = keyManager.generate_private_key('Ed25519')

    const exportedKeys = keyManager.export_private_keys()
    expect(exportedKeys.length).toBe(2)

    const newKeyManager = new LocalJwkManager()
    newKeyManager.import_private_keys(exportedKeys)

    const publicKey1 = newKeyManager.get_public_key(keyAlias1)
    expect(publicKey1).toBeDefined()

    const publicKey2 = newKeyManager.get_public_key(keyAlias2)
    expect(publicKey2).toBeDefined()
  })

  // todo implement get_signer()
  // test('should get signer function', () => {
  //   const keyAlias = keyManager.generate_private_key('Secp256k1');
  //   const signer = keyManager.get_signer(keyAlias);
  //   expect(typeof signer).toBe('function');

  //   const payload = new TextEncoder().encode('test message');
  //   const signature = signer(payload);
  //   expect(signature).toBeDefined();
  // });
})