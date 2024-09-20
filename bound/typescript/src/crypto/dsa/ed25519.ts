import wasm from "../../wasm";
import { Jwk } from "../jwk";
import { catchWeb5Error } from "../../errors";
import { Signer, TypescriptSigner } from ".";

export class Ed25519Generator {
  static generate(): Jwk {
    try {
      const wasmJwk = wasm.generate_ed25519_key(); 
      return Jwk.fromWasmJwk(wasmJwk); 
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }
}

export class Ed25519Signer implements Signer {
  private signer: TypescriptSigner;

  constructor(jwk: Jwk) {
    try {
      this.signer = new TypescriptSigner(wasm.new_ed25519_signer(jwk.toWasmJwk()));
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }

  sign(payload: Uint8Array): Uint8Array {
    return this.signer.sign(payload);
  }
}