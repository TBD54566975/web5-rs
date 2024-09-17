package dsa

import (
	"errors"
	"web5/crypto"
	"web5/web5c"
)

type Ed25519Signer struct {
	cSigner *web5c.CEd25519Signer
}

func NewEd25519Signer(jwk crypto.JWK) (*Ed25519Signer, error) {
	cJWK := jwk.ToCJWK()

	signer, err := web5c.NewCEd25519Signer(cJWK)
	if err != nil {
		return nil, errors.New("failed to create Ed25519Signer")
	}

	return &Ed25519Signer{signer}, nil
}

func (s *Ed25519Signer) Sign(payload []byte) ([]byte, error) {
	return s.cSigner.Sign(payload)
}
