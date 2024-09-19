// src/crypto/jwk.ts

export interface JWK {
  kty: string;
  alg: string;
  use?: string;
  kid?: string;
  n?: string;
  e?: string;
  d?: string;
  p?: string;
  q?: string;
  dp?: string;
  dq?: string;
  qi?: string;
}

export function validateJWK(jwk: JWK): boolean {
  if (!jwk.kty || !jwk.alg) {
    return false;
  }
  // Add more validation logic if necessary
  return true;
}