import {
  generateEd25519Key,
  generateSecp256k1Key,
  signSecp256k1,
  signEd25519,
  verifySecp256k1,
  verifyEd25519 } from '../pkg'

describe('crypto', () => {
  test('should generate Ed25519 key', () => {
    const key = generateEd25519Key()
    expect(key).toBeDefined()
    expect(key.alg).toBe('Ed25519')
    expect(key.kty).toBe('OKP')
    expect(key.crv).toBe('Ed25519')
    expect(key.d).toBeDefined()
    expect(key.x).toBeDefined()
    expect(key.y).toBeUndefined()
  })

  test('should generate Secp256k1 key', () => {
    const key = generateSecp256k1Key()
    expect(key).toBeDefined()
    expect(key.alg).toBe('ES256K')
    expect(key.kty).toBe('EC')
    expect(key.crv).toBe('secp256k1')
    expect(key.d).toBeDefined()
    expect(key.x).toBeDefined()
    expect(key.y).toBeDefined()
  })

  test('should sign and verify using Ed25519', () => {
    const privateKey = generateEd25519Key()
    const payload = new TextEncoder().encode('hello world')

    const signature = signEd25519(privateKey, payload)
    expect(signature).toBeDefined()

    const publicKey = privateKey.toPublic()
    const verified = verifyEd25519(publicKey, payload, signature)
    expect(verified).toBe(undefined)
  })

  test('should sign and verify using Secp256k1', () => {
    const privateKey = generateSecp256k1Key()
    const payload = new TextEncoder().encode('hello world')

    const signature = signSecp256k1(privateKey, payload)
    expect(signature).toBeDefined()

    const publicKey = privateKey.toPublic()
    const verified = verifySecp256k1(publicKey, payload, signature)
    expect(verified).toBe(undefined)
  })
})