import { catchWeb5Error } from "../errors";
import wasm from "../wasm"

export class Jwk {
  private wasmJwk: wasm.WasmJwk;

  constructor(
    alg: string | undefined,
    kty: string,
    crv: string,
    d: string | undefined,
    x: string,
    y: string | undefined
  ) {
    this.wasmJwk = new wasm.WasmJwk(alg, kty, crv, d, x, y);
  }

  computeThumbprint(): string {
    try {
      return this.wasmJwk.compute_thumbprint();
    } catch (error) {
      throw catchWeb5Error(error)
    }
  }
}
