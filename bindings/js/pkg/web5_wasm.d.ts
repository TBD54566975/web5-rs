/* tslint:disable */
/* eslint-disable */
/**
* @returns {Jwk}
*/
export function generate_ed25519_key(): Jwk;
/**
* @returns {Jwk}
*/
export function generate_secp256k1_key(): Jwk;
/**
* @param {Jwk} private_key
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
export function sign_secp256k1(private_key: Jwk, payload: Uint8Array): Uint8Array;
/**
* @param {Jwk} private_key
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
export function sign_ed25519(private_key: Jwk, payload: Uint8Array): Uint8Array;
/**
* @param {Jwk} public_key
* @param {Uint8Array} payload
* @param {Uint8Array} signature
*/
export function verify_secp256k1(public_key: Jwk, payload: Uint8Array, signature: Uint8Array): void;
/**
* @param {Jwk} public_key
* @param {Uint8Array} payload
* @param {Uint8Array} signature
*/
export function verify_ed25519(public_key: Jwk, payload: Uint8Array, signature: Uint8Array): void;
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
  compute_thumbprint(): string;
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
  to_public(): Jwk;
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
  generate_private_key(curve: string, key_alias?: string): string;
/**
* @param {string} key_alias
* @returns {Jwk}
*/
  get_public_key(key_alias: string): Jwk;
/**
* @param {string} key_alias
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
  sign(key_alias: string, payload: Uint8Array): Uint8Array;
/**
* @returns {(Jwk)[]}
*/
  export_private_keys(): (Jwk)[];
/**
* @param {(Jwk)[]} private_keys
*/
  import_private_keys(private_keys: (Jwk)[]): void;
}
