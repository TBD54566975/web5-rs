import { generate_ed25519_key, generate_secp256k1_key, sign_ed25519, sign_secp256k1, verify_ed25519, verify_secp256k1 } from '../../../bindings/wasm/pkg';

describe('crypto', () => {
  test('should generate Ed25519 key', () => {
    const key = generate_ed25519_key();
    expect(key).toBeDefined();
    expect(key.alg).toBe('EdDSA');
    expect(key.kty).toBe('OKP');
    expect(key.crv).toBe('Ed25519');
    expect(key.d).toBeDefined();
    expect(key.x).toBeDefined();
    expect(key.y).toBeUndefined();
  });

  test('should generate Secp256k1 key', () => {
    const key = generate_secp256k1_key();
    expect(key).toBeDefined();
    expect(key.alg).toBe('ES256K');
    expect(key.kty).toBe('EC');
    expect(key.crv).toBe('secp256k1');
    expect(key.d).toBeDefined();
    expect(key.x).toBeDefined();
    expect(key.y).toBeDefined();
  });

  test('should sign and verify using Ed25519', () => {
    const privateKey = generate_ed25519_key();
    const payload = new TextEncoder().encode('hello world');

    const signature = sign_ed25519(privateKey, payload);
    expect(signature).toBeDefined();

    const publicKey = privateKey.to_public();
    const verified = verify_ed25519(publicKey, payload, signature);
    expect(verified).toBe(undefined);
  });

  test('should sign and verify using Secp256k1', () => {
    const privateKey = generate_secp256k1_key();
    const payload = new TextEncoder().encode('hello world');

    const signature = sign_secp256k1(privateKey, payload);
    expect(signature).toBeDefined();

    const publicKey = privateKey.to_public();
    const verified = verify_secp256k1(publicKey, payload, signature);
    expect(verified).toBe(undefined);
  });
});