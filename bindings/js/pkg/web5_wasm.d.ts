/* tslint:disable */
/* eslint-disable */
/**
* @returns {Jwk}
*/
export function generateEd25519Key(): Jwk;
/**
* @returns {Jwk}
*/
export function generateSecp256k1Key(): Jwk;
/**
* @param {Jwk} private_key
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
export function signSecp256k1(private_key: Jwk, payload: Uint8Array): Uint8Array;
/**
* @param {Jwk} private_key
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
export function signEd25519(private_key: Jwk, payload: Uint8Array): Uint8Array;
/**
* @param {Jwk} public_key
* @param {Uint8Array} payload
* @param {Uint8Array} signature
*/
export function verifySecp256k1(public_key: Jwk, payload: Uint8Array, signature: Uint8Array): void;
/**
* @param {Jwk} public_key
* @param {Uint8Array} payload
* @param {Uint8Array} signature
*/
export function verifyEd25519(public_key: Jwk, payload: Uint8Array, signature: Uint8Array): void;
/**
*/
export class Identifier {
  free(): void;
/**
* @param {string} did_uri
* @returns {Identifier}
*/
  static parse(did_uri: string): Identifier;
/**
*/
  readonly fragment: string | undefined;
/**
*/
  readonly id: string;
/**
*/
  readonly method: string;
/**
*/
  readonly params: any;
/**
*/
  readonly path: string | undefined;
/**
*/
  readonly query: string | undefined;
/**
*/
  readonly uri: string;
/**
*/
  readonly url: string;
}
/**
*/
export class Jwk {
  free(): void;
/**
* @param {string} alg
* @param {string} kty
* @param {string} crv
* @param {string | undefined} d
* @param {string} x
* @param {string | undefined} [y]
*/
  constructor(alg: string, kty: string, crv: string, d: string | undefined, x: string, y?: string);
/**
* @returns {string}
*/
  computeThumbprint(): string;
/**
* PublicKey implementations 
* @param {Uint8Array} payload
* @param {Uint8Array} signature
*/
  verify(payload: Uint8Array, signature: Uint8Array): void;
/**
* PrivateKey implementations 
* @returns {Jwk}
*/
  toPublic(): Jwk;
/**
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
  sign(payload: Uint8Array): Uint8Array;
/**
*/
  readonly alg: string;
/**
*/
  readonly crv: string;
/**
*/
  readonly d: string | undefined;
/**
*/
  readonly kty: string;
/**
*/
  readonly x: string;
/**
*/
  readonly y: string | undefined;
}
/**
*/
export class LocalJwkManager {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} curve
* @param {string | undefined} [key_alias]
* @returns {string}
*/
  generatePrivateKey(curve: string, key_alias?: string): string;
/**
* @param {string} key_alias
* @returns {Jwk}
*/
  getPublicKey(key_alias: string): Jwk;
/**
* @param {string} key_alias
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
  sign(key_alias: string, payload: Uint8Array): Uint8Array;
/**
* @returns {(Jwk)[]}
*/
  exportPrivateKeys(): (Jwk)[];
/**
* @param {(Jwk)[]} private_keys
*/
  importPrivateKeys(private_keys: (Jwk)[]): void;
}
