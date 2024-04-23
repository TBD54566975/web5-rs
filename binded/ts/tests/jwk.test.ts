import { Jwk } from '../pkg'

describe('Jwk', () => {
  test('should create a new Jwk instance', () => {
    const alg = 'ES256'
    const kty = 'EC'
    const crv = 'P-256'
    const x = 'MKBCTNIcKUSDii11ySs3526iDZ8AiTo7Tu6KPAqv7D4'
    const y = '4Etl6SRW2YiLUrN5vfvVHuhp7x8PxltmWWlbbM4IFyM'

    const jwk = new Jwk(alg, kty, crv, undefined, x, y)

    expect(jwk).toBeInstanceOf(Jwk)
    expect(jwk.alg).toEqual(alg)
    expect(jwk.kty).toEqual(kty)
    expect(jwk.crv).toEqual(crv)
    expect(jwk.x).toEqual(x)
    expect(jwk.y).toEqual(y)
    expect(jwk.d).toBeUndefined()
  })

  test('should compute thumbprint', () => {
    const alg = 'ES256'
    const kty = 'EC'
    const crv = 'P-256'
    const x = 'MKBCTNIcKUSDii11ySs3526iDZ8AiTo7Tu6KPAqv7D4'
    const y = '4Etl6SRW2YiLUrN5vfvVHuhp7x8PxltmWWlbbM4IFyM'

    const jwk = new Jwk(alg, kty, crv, undefined, x, y)
    const thumbprint = jwk.computeThumbprint()

    expect(typeof thumbprint).toBe('string')
  })
})