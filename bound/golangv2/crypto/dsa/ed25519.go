package dsa

import (
	"errors"
	"web5/crypto"
	"web5/web5c"
)

type Ed25519Signer struct {
	signer *web5c.CEd25519Signer
}

func NewEd25519Signer(jwk crypto.JWK) (*Ed25519Signer, error) {
	cJwk := web5c.NewCJwk(jwk.ALG, jwk.KTY, jwk.CRV, jwk.D, jwk.X, jwk.Y)
	defer web5c.FreeCJwk(cJwk)

	signer, err := web5c.NewCEd25519Signer(cJwk)
	if err != nil {
		return nil, errors.New("failed to create Ed25519Signer")
	}

	return &Ed25519Signer{signer}, nil
}

func (s *Ed25519Signer) Sign(payload []byte) ([]byte, error) {
	return web5c.CEd25519SignerSign(s.signer, payload)
}