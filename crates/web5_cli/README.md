# `web5` CLI

🚧 This is under construction 🚧

```shell
web5 -h
```

```shell
web5 did -h
```

```shell
web5 vc -h
```

## Examples

Certain operations, such as those that utilize the `InMemoryKeyManager`, may require root privileges. Ensure that you are running the `did create` command with the appropriate permissions. 

### Create a `did:dht`

```shell
➜ web5 did create dht
{
  "uri": "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo",
  "document": {
    "id": "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo",
    "verificationMethod": [
      {
        "id": "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo#0",
        "type": "JsonWebKey",
        "controller": "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo",
        "publicKeyJwk": {
          "alg": "Ed25519",
          "kty": "OKP",
          "crv": "Ed25519",
          "x": "FrobTImviZSWWggigv1z_y8BubdpLLGa6uPcq9RBZxc"
        }
      }
    ],
    "authentication": [
      "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo#0"
    ],
    "assertionMethod": [
      "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo#0"
    ],
    "capabilityInvocation": [
      "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo#0"
    ],
    "capabilityDelegation": [
      "did:dht:n47bsurji6r3jf14bytef9mu9hzodqpzprsmdgzkhxqkzinbchmo#0"
    ]
  },
  "privateKeys": [
    {
      "alg": "Ed25519",
      "kty": "OKP",
      "crv": "Ed25519",
      "d": "jRPQWN61KmTFQfo8glxu9T1GJkYpzz3-jNPJ9k0MkVs",
      "x": "FrobTImviZSWWggigv1z_y8BubdpLLGa6uPcq9RBZxc"
    }
  ]
}
```
### Create a `did:jwk`

```shell
➜ web5 did create jwk
{
  "uri": "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0",
  "document": {
    "id": "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0",
    "@context": [
      "https://www.w3.org/ns/did/v1"
    ],
    "verificationMethod": [
      {
        "id": "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0#0",
        "type": "JsonWebKey",
        "controller": "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0",
        "publicKeyJwk": {
          "alg": "Ed25519",
          "kty": "OKP",
          "crv": "Ed25519",
          "x": "PjGOpHaQ0l65M71LNFEMoPJIPui3Z_n8Tr3XL3aCVXI"
        }
      }
    ],
    "authentication": [
      "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0#0"
    ],
    "assertionMethod": [
      "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0#0"
    ],
    "capabilityInvocation": [
      "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0#0"
    ],
    "capabilityDelegation": [
      "did:jwk:eyJhbGciOiJFZDI1NTE5Iiwia3R5IjoiT0tQIiwiY3J2IjoiRWQyNTUxOSIsIngiOiJQakdPcEhhUTBsNjVNNzFMTkZFTW9QSklQdWkzWl9uOFRyM1hMM2FDVlhJIn0#0"
    ]
  },
  "privateKeys": [
    {
      "alg": "Ed25519",
      "kty": "OKP",
      "crv": "Ed25519",
      "d": "Z2TSOicWegPpYJghDn9UVKsVWAexgsgFBjr2Cl5hQ9Y",
      "x": "PjGOpHaQ0l65M71LNFEMoPJIPui3Z_n8Tr3XL3aCVXI"
    }
  ]
}
```

### Resolve a DID URI

```shell
#/bin/bash

web5 did resolve did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o
{
  "id": "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o",
  "verificationMethod": [
    {
      "id": "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o#0",
      "type": "JsonWebKey",
      "controller": "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o",
      "publicKeyJwk": {
        "alg": "Ed25519",
        "kty": "OKP",
        "crv": "Ed25519",
        "x": "DQnPlbziDckkWecR7frzAV7hr1_NobJri2xRIBdb9Tc"
      }
    }
  ],
  "authentication": [
    "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o#0"
  ],
  "assertionMethod": [
    "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o#0"
  ],
  "capabilityInvocation": [
    "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o#0"
  ],
  "capabilityDelegation": [
    "did:dht:bwrh9fphhegh1jn3hhe656zuyfxqdm493so5r4hmpte1yf456w5o#0"
  ]
}
```

### Create a VC & sign it

```shell
#/bin/bash 

export PORTABLE_DID=$(web5 did create dht --no-indent)

web5 vc create "alice" --portable-did $PORTABLE_DID
```

### Verify a VC

```shell
#/bin/bash

web5 vc verify eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpkaHQ6OXFnOGgxc3Jvd2hzZHNla3hwODk4eTU0MXhndGZ4Ym1ybW5oaGdzanlobXRtOHRjb253byMwIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjlkMDhhNjAzLWMyNTMtNGQyNC05M2MzLWIzYzAwMzg2NjM5MCIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmRodDo5cWc4aDFzcm93aHNkc2VreHA4OTh5NTQxeGd0ZnhibXJtbmhoZ3NqeWhtdG04dGNvbndvIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNi0yOFQxMzoxOTo1OS45OTY2MzMrMDA6MDAiLCJleHBpcmF0aW9uRGF0ZSI6bnVsbCwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJhbGljZSJ9fSwiaXNzIjoiZGlkOmRodDo5cWc4aDFzcm93aHNkc2VreHA4OTh5NTQxeGd0ZnhibXJtbmhoZ3NqeWhtdG04dGNvbndvIiwianRpIjoidXJuOnZjOnV1aWQ6OWQwOGE2MDMtYzI1My00ZDI0LTkzYzMtYjNjMDAzODY2MzkwIiwic3ViIjoiYWxpY2UiLCJuYmYiOjE3MTk1ODA3OTksImlhdCI6MTcxOTU4MDgwMH0.PJbb9EidggoqHL3IkfcglcTNzp_obBqbZjE0M4mL2XlecdLKNusZ3i4Hm0BtnzJ0ME7zYAvdIwg4shW4U884Bg
```