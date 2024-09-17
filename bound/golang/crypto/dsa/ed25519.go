package dsa

import (
	"web5/crypto"
	"web5/web5c"
)

type Ed25519Signer struct {
	cSigner *web5c.CSigner
}

func NewEd25519Signer(jwk crypto.JWK) (*Ed25519Signer, error) {
	cJWK := jwk.ToCJWK()
	cSigner, err := web5c.NewCEd25519Signer(cJWK)
	if err != nil {
		return nil, err
	}
	signer := Ed25519Signer{cSigner}
	return &signer, nil
}

func (s *Ed25519Signer) Sign(payload []byte) ([]byte, error) {
	return s.cSigner.Sign(payload)
}
