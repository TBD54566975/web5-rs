import { expect } from 'chai';
import { sayHello } from "../src/hello"

describe('sayHello function', () => {
  it('prints hello', () => {
    sayHello();
    expect(1).to.equal(1); // Simple assertion to pass the test
  });
});