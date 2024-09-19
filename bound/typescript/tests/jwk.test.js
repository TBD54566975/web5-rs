import web5 from "../dist/index.js";
import { expect } from 'chai'

describe("test EC valid", async () => {
  console.log(web5)
  console.log(web5.WasmJwk)
  it('works', () => {
    expect(1).to.equal(1)
  })
});
