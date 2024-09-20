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

  tmp() {
    console.log('kw dbg begin')

    // function hello1() {
    //   console.log("hello 1 from javascript");
    // }
    // wasm.call_js_function(hello1);

    const obj = {
        hello1: function() {
            console.log("hello 1 from javascript");
        },
        hello2: function() {
            console.log("hello 2 from javascript");
        }
    };
    wasm.call_js_functions(obj);

    console.log('kw dbg end')
  }
}
