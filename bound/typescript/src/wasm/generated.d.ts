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
*/
export class WasmBearerDid {
  free(): void;
/**
* @param {WasmDid} did
* @param {WasmDocument} document
* @param {WasmKeyManager} key_manager
*/
  constructor(did: WasmDid, document: WasmDocument, key_manager: WasmKeyManager);
/**
* @param {WasmPortableDid} portable_did
* @returns {WasmBearerDid}
*/
  static from_portable_did(portable_did: WasmPortableDid): WasmBearerDid;
/**
* @param {string} verification_method_id
* @returns {WasmSigner}
*/
  get_signer(verification_method_id: string): WasmSigner;
/**
*/
  readonly did: WasmDid;
/**
*/
  readonly document: WasmDocument;
/**
*/
  readonly key_manager: WasmKeyManager;
}
/**
*/
export class WasmDid {
  free(): void;
/**
* @param {string} uri
* @param {string} url
* @param {string} method
* @param {string} id
* @param {any} params
* @param {string | undefined} [path]
* @param {string | undefined} [query]
* @param {string | undefined} [fragment]
*/
  constructor(uri: string, url: string, method: string, id: string, params: any, path?: string, query?: string, fragment?: string);
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
export class WasmDocument {
  free(): void;
/**
* @param {string} id
* @param {(string)[] | undefined} context
* @param {(string)[] | undefined} controller
* @param {(string)[] | undefined} also_known_as
* @param {(WasmVerificationMethod)[]} verification_method
* @param {(string)[] | undefined} [authentication]
* @param {(string)[] | undefined} [assertion_method]
* @param {(string)[] | undefined} [key_agreement]
* @param {(string)[] | undefined} [capability_invocation]
* @param {(string)[] | undefined} [capability_delegation]
* @param {(WasmService)[] | undefined} [service]
*/
  constructor(id: string, context: (string)[] | undefined, controller: (string)[] | undefined, also_known_as: (string)[] | undefined, verification_method: (WasmVerificationMethod)[], authentication?: (string)[], assertion_method?: (string)[], key_agreement?: (string)[], capability_invocation?: (string)[], capability_delegation?: (string)[], service?: (WasmService)[]);
/**
* @param {string} json
* @returns {WasmDocument}
*/
  static from_json_string(json: string): WasmDocument;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
*/
  readonly also_known_as: (string)[] | undefined;
/**
*/
  readonly assertion_method: (string)[] | undefined;
/**
*/
  readonly authentication: (string)[] | undefined;
/**
*/
  readonly capability_delegation: (string)[] | undefined;
/**
*/
  readonly capability_invocation: (string)[] | undefined;
/**
*/
  readonly context: (string)[] | undefined;
/**
*/
  readonly controller: (string)[] | undefined;
/**
*/
  readonly id: string;
/**
*/
  readonly key_agreement: (string)[] | undefined;
/**
*/
  readonly service: (WasmService)[] | undefined;
/**
*/
  readonly verification_method: (WasmVerificationMethod)[];
}
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
export class WasmPortableDid {
  free(): void;
/**
* @param {string} did_uri
* @param {WasmDocument} document
* @param {(WasmJwk)[]} private_keys
*/
  constructor(did_uri: string, document: WasmDocument, private_keys: (WasmJwk)[]);
/**
* @param {string} json
* @returns {WasmPortableDid}
*/
  static from_json_string(json: string): WasmPortableDid;
/**
* @returns {string}
*/
  to_json_string(): string;
/**
*/
  readonly did_uri: string;
/**
*/
  readonly document: WasmDocument;
/**
*/
  readonly private_keys: (WasmJwk)[];
}
/**
*/
export class WasmService {
  free(): void;
/**
* @param {string} id
* @param {string} type
* @param {(string)[]} service_endpoint
*/
  constructor(id: string, type: string, service_endpoint: (string)[]);
/**
*/
  readonly id: string;
/**
*/
  readonly service_endpoint: (string)[];
/**
*/
  readonly type: string;
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
export class WasmVerificationMethod {
  free(): void;
/**
* @param {string} id
* @param {string} type
* @param {string} controller
* @param {WasmJwk} public_key_jwk
*/
  constructor(id: string, type: string, controller: string, public_key_jwk: WasmJwk);
/**
*/
  readonly controller: string;
/**
*/
  readonly id: string;
/**
*/
  readonly public_key_jwk: WasmJwk;
/**
*/
  readonly type: string;
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