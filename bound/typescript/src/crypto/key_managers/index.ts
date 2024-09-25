import { catchWeb5Error } from "../../errors";
import wasm from "../../wasm";
import { Signer, TypescriptSigner } from "../dsa";
import { Jwk } from "../jwk";

export interface KeyManager {
  importPrivateJwk(privateJwk: Jwk): Jwk
  getSigner(publicJwk: Jwk): Signer
}

export class TypescriptKeyManager implements KeyManager {
  private wasmKeyManager: wasm.WasmKeyManager;

  constructor(wasmKeyManager: wasm.WasmKeyManager) {
    this.wasmKeyManager = wasmKeyManager
  }

  importPrivateJwk(privateJwk: Jwk): Jwk {
    try {
      const wasmJwk = this.wasmKeyManager.import_private_jwk(privateJwk.toWasmJwk());
      return Jwk.fromWasmJwk(wasmJwk);
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }

  getSigner(publicJwk: Jwk): Signer {
    try {
      const wasmSigner = this.wasmKeyManager.get_signer(publicJwk.toWasmJwk());
      return new TypescriptSigner(wasmSigner)
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }
}