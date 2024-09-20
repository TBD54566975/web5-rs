import { expect } from 'chai';
import { Jwk } from '../../src/crypto/jwk';
import { Web5Error } from '../../src/errors';

describe('Jwk class', () => {
  it('should compute the correct thumbprint for EC key', async () => {
    const jwk = new Jwk('ES256', 'EC', 'secp256k1', undefined, 'x_value', 'y_value');
    const thumbprint = jwk.computeThumbprint();
    expect(thumbprint).to.equal('yiiszVT5Lwt6760MW19cHaJ61qJKIfe20sUW8dNxBv4');
  });

  it('should compute the correct thumbprint for OKP key', async () => {
    const jwk = new Jwk(undefined, 'OKP', 'Ed25519', undefined, 'x_value', undefined);
    const thumbprint = jwk.computeThumbprint();
    expect(thumbprint).to.equal('nDMRVZm4lpedGjuJGO4y3YVJJ0krDF0aSz4KhlncDdI');
  });

  it('should throw error for unsupported kty', async () => {
    try {
      const jwk = new Jwk(undefined, 'RSA', 'RS256', undefined, 'x_value', 'y_value');
      jwk.computeThumbprint();
    } catch (error: any) {
      expect(error.variant).to.equal('DataMember');
      expect(error.message).to.equal('data member error kty not supported RSA');
    }
  });

  it('should throw error when kty is empty', async () => {
    try {
      const jwk = new Jwk(undefined, '', 'Ed25519', undefined, 'x_value', undefined);
      jwk.computeThumbprint();
    } catch (error: any) {
      expect(error.variant).to.equal('DataMember');
      expect(error.message).to.equal('data member error kty cannot be empty');
    }
  });

  it('should throw error when x is empty', async () => {
    try {
      const jwk = new Jwk(undefined, 'OKP', 'Ed25519', undefined, '', undefined);
      jwk.computeThumbprint();
    } catch (error: any) {
      expect(error.variant).to.equal('DataMember');
      expect(error.message).to.equal('data member error x cannot be empty');
    }
  });

  it('should throw error when crv is empty', async () => {
    try {
      const jwk = new Jwk(undefined, 'EC', '', undefined, 'x_value', 'y_value');
      jwk.computeThumbprint();
    } catch (error: any) {
      expect(error.variant).to.equal('DataMember');
      expect(error.message).to.equal('data member error crv cannot be empty');
    }
  });

  it('should throw error when y is missing for EC key', async () => {
    try {
      const jwk = new Jwk(undefined, 'EC', 'P-256', undefined, 'x_value', undefined);
      jwk.computeThumbprint();
    } catch (error: any) {
      expect(error.variant).to.equal('DataMember');
      expect(error.message).to.equal('data member error missing y');
    }
  });

  it('should throw error when y is empty for EC key', async () => {
    try {
      const jwk = new Jwk(undefined, 'EC', 'P-256', undefined, 'x_value', '');
      jwk.computeThumbprint();
    } catch (error: any) {
      expect(error.variant).to.equal('DataMember');
      expect(error.message).to.equal('data member error y cannot be empty');
    }
  });
});