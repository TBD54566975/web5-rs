import wasm from "../../wasm";
import { Jwk } from "../jwk";
import { catchWeb5Error } from "../../errors";

export class Ed25519Generator {
  /**
   * Generates a new Ed25519 key pair and returns it as a JWK.
   *
   * @returns A `Jwk` object containing the generated Ed25519 key pair.
   */
  static generate(): Jwk {
    try {
      const wasmJwk = wasm.generate_ed25519_key(); 
      return Jwk.fromWasmJwk(wasmJwk); 
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }
}

export class Ed25519Signer {
  private wasmSigner: wasm.WasmSigner;

  /**
   * Creates a new Ed25519Signer with the given JWK.
   *
   * @param jwk - The JWK containing the Ed25519 private key.
   */
  constructor(jwk: Jwk) {
    try {
      this.wasmSigner = wasm.new_ed25519_signer(jwk.toWasmJwk());
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }

  /**
   * Signs the given payload using the Ed25519 private key.
   *
   * @param payload - The data to be signed as a Uint8Array.
   * @returns The signature as a Uint8Array.
   */
  sign(payload: Uint8Array): Uint8Array {
    try {
      return this.wasmSigner.sign(payload);
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }
}