/* tslint:disable */
/* eslint-disable */
/**
* @returns {WasmKeyManager}
*/
export function new_in_memory_key_manager(): WasmKeyManager;
/**
* @param {{ importPrivateJwk: (privateJwk: WasmJwk) => WasmJwk, getSigner: (publicJwk: WasmJwk) => WasmSigner }} key_manager
* @returns {WasmSigner}
*/
export function poc_key_manager_from_foreign(key_manager: { importPrivateJwk: (privateJwk: WasmJwk) => WasmJwk, getSigner: (publicJwk: WasmJwk) => WasmSigner }): WasmSigner;
/**
* @returns {WasmJwk}
*/
export function generate_ed25519_key(): WasmJwk;
/**
* @returns {WasmJwk}
*/
export function generate_secp256k1_key(): WasmJwk;
/**
* @param {WasmJwk} jwk
* @returns {WasmSigner}
*/
export function new_ed25519_signer(jwk: WasmJwk): WasmSigner;
/**
* @param {WasmJwk} jwk
* @returns {WasmSigner}
*/
export function new_secp256k1_signer(jwk: WasmJwk): WasmSigner;
/**
* @param {{hello1: Function, hello2: Function}} obj
*/
export function call_js_functions(obj: {hello1: Function, hello2: Function}): void;
/**
*/
export class WasmJwk {
  free(): void;
/**
* @param {string | undefined} alg
* @param {string} kty
* @param {string} crv
* @param {string | undefined} d
* @param {string} x
* @param {string | undefined} [y]
*/
  constructor(alg: string | undefined, kty: string, crv: string, d: string | undefined, x: string, y?: string);
/**
* @returns {string}
*/
  compute_thumbprint(): string;
/**
*/
  readonly alg: string | undefined;
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
export class WasmKeyManager {
  free(): void;
/**
* @param {WasmJwk} private_jwk
* @returns {WasmJwk}
*/
  import_private_jwk(private_jwk: WasmJwk): WasmJwk;
/**
* @param {WasmJwk} public_jwk
* @returns {WasmSigner}
*/
  get_signer(public_jwk: WasmJwk): WasmSigner;
}
/**
*/
export class WasmSigner {
  free(): void;
/**
* @param {Uint8Array} payload
* @returns {Uint8Array}
*/
  sign(payload: Uint8Array): Uint8Array;
}
/**
*/
export class WasmWeb5Error {
  free(): void;
/**
*/
  readonly is_web5_error: boolean;
/**
*/
  readonly message: string;
/**
*/
  readonly variant: string;
}
/**
 * Load the WebAssembly module in the background, if it has not already been loaded.
 *
 * Returns a promise which will resolve once the other methods are ready.
 *
 * @returns {Promise<void>}
 */
export function loadWasmAsync(): Promise<void>;

export function loadWasmSync(): void;