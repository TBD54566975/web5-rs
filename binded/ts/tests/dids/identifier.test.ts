import { Identifier } from '../../pkg'

describe('Identifier', () => {
  test('should parse a valid DID URI', () => {
    const uri = 'did:example:123456789abcdefghi'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.params).toBeUndefined()
    expect(identifier.path).toBeUndefined()
    expect(identifier.query).toBeUndefined()
    expect(identifier.fragment).toBeUndefined()
  })

  test('should parse a DID URI with params', () => {
    const uri = 'did:example:123456789abcdefghi;foo=bar;baz=qux'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi;foo=bar;baz=qux')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.path).toBeUndefined()
    expect(identifier.query).toBeUndefined()
    expect(identifier.fragment).toBeUndefined()

    // Check the params property
    expect(identifier.params).toBeDefined()
    expect(identifier.params.foo).toEqual('bar')
    expect(identifier.params.baz).toEqual('qux')
  })

  test('should parse a DID URI with query', () => {
    const uri = 'did:example:123456789abcdefghi?foo=bar&baz=qux'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi?foo=bar&baz=qux')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.path).toBeUndefined()
    expect(identifier.query).toEqual('foo=bar&baz=qux')
    expect(identifier.fragment).toBeUndefined()
  })

  test('should parse a DID URI with fragment', () => {
    const uri = 'did:example:123456789abcdefghi#keys-1'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi#keys-1')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.path).toBeUndefined()
    expect(identifier.query).toBeUndefined()
    expect(identifier.fragment).toEqual('keys-1')
  })

  test('should parse a DID URI with query and fragment', () => {
    const uri = 'did:example:123456789abcdefghi?foo=bar&baz=qux#keys-1'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi?foo=bar&baz=qux#keys-1')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.path).toBeUndefined()
    expect(identifier.query).toEqual('foo=bar&baz=qux')
    expect(identifier.fragment).toEqual('keys-1')
  })

  test('should parse a DID URI with params, query, and fragment', () => {
    const uri = 'did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.path).toBeUndefined()
    expect(identifier.query).toEqual('foo=bar&baz=qux')
    expect(identifier.fragment).toEqual('keys-1')
  })

  test('should parse a DID URI with path', () => {
    const uri = 'did:example:123456789abcdefghi/path/to/resource'
    const identifier = Identifier.parse(uri)

    expect(identifier).toBeInstanceOf(Identifier)
    expect(identifier.uri).toEqual('did:example:123456789abcdefghi')
    expect(identifier.url).toEqual('did:example:123456789abcdefghi/path/to/resource')
    expect(identifier.method).toEqual('example')
    expect(identifier.id).toEqual('123456789abcdefghi')
    expect(identifier.path).toEqual('/path/to/resource')
    expect(identifier.query).toBeUndefined()
    expect(identifier.fragment).toBeUndefined()
  })

  test('should throw an error for invalid DID URIs', () => {
    const invalidUris = [
      '',
      'did:',
      'did:uport',
      'did:uport:',
      'did:uport:1234_12313***',
      '2nQtiQG6Cgm1GYTBaaKAgr76uY7iSexUkqX',
      'did:method:%12%1',
      'did:method:%1233%Ay',
      'did:CAP:id',
      'did:method:id::anotherid%r9',
    ]

    invalidUris.forEach((uri) => {
      expect(() => Identifier.parse(uri)).toThrow()
    })
  })
})