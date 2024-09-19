import wasm from "./wasm"

export function sayHello() {
  console.log("Hello from TypeScript!");

  wasm.loadWasmSync();
  const jwk = new wasm.WasmJwk(
    undefined,  
    'EC',       
    'secp256k1',
    undefined,  
    'x_value',  
    'y_value'   
  );

  const thumbprint = jwk.compute_thumbprint();
  console.log('WasmJwk thumbprint:', thumbprint);
}