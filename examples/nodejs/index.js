const { JWK } = require('../../bindings/wasm/pkg');

async function main() {
  const jwk = new JWK(); // Instantiates the JWK object

  // Setting properties of jwk
  jwk.set_alg("");
  jwk.set_kty("EC");
  jwk.set_crv("secp256k1");
  jwk.set_d("");
  jwk.set_x("IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY");
  jwk.set_y("UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA");

  // Optionally set properties of jwk here, if setter methods are implemented
  const thumbprint = jwk.compute_thumbprint();
  const expected = 'bgEObpJ1QzKa0jhWUkMSQKDOSDKEmwIw77ewaYpyduk';
  console.log(`Thumbprint: ${thumbprint}`);
  console.log(`Expected: ${expected} (${thumbprint === expected})`)
}

main().catch(console.error);
