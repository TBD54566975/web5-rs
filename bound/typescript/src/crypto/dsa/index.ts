import { catchWeb5Error } from "../../errors";
import wasm from "../../wasm";

export interface Signer {
  sign(payload: Uint8Array): Uint8Array;
}

export class TypescriptSigner implements Signer {
  private wasmSigner: wasm.WasmSigner;

  constructor(wasmSigner: wasm.WasmSigner) {
    this.wasmSigner = wasmSigner
  }

  sign(payload: Uint8Array): Uint8Array {
    try {
      return this.wasmSigner.sign(payload);
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }
}