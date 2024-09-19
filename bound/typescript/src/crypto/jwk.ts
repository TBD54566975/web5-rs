import { WasmJwk } from "../web5_wasm";

export class Jwk {
  private wasmJwk: WasmJwk;

  constructor(
    kty: string,
    crv: string,
    x: string,
    y?: string,
    alg?: string,
    d?: string
  ) {
    this.wasmJwk = new WasmJwk(alg, kty, crv, d, x, y);
  }

  async computeThumbprint(): Promise<string> {
    return this.wasmJwk.compute_thumbprint();
  }
}