import { catchWeb5Error } from "../errors";
import wasm from "../wasm"

export class Jwk {
  public alg: string | undefined;
  public kty: string;
  public crv: string;
  public d: string | undefined;
  public x: string;
  public y: string | undefined;

  static fromWasmJwk(wasmJwk: wasm.WasmJwk): Jwk {
    return new Jwk(
      wasmJwk.alg,
      wasmJwk.kty,
      wasmJwk.crv,
      wasmJwk.d,
      wasmJwk.x,
      wasmJwk.y
    );
  }

  constructor(
    alg: string | undefined,
    kty: string,
    crv: string,
    d: string | undefined,
    x: string,
    y: string | undefined
  ) {
    this.alg = alg;
    this.kty = kty;
    this.crv = crv;
    this.d = d;
    this.x = x;
    this.y = y;
  }

  toWasmJwk(): wasm.WasmJwk {
    return new wasm.WasmJwk(this.alg, this.kty, this.crv, this.d, this.x, this.y);
  }

  computeThumbprint(): string {
    try {
      const wasmJwk = this.toWasmJwk();
      return wasmJwk.compute_thumbprint();
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }
}