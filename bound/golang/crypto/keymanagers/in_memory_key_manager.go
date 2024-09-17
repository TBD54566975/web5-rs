package keymanagers

import (
	"web5/crypto"
	"web5/crypto/dsa"
	"web5/web5c"
)

type InMemoryKeyManager struct {
	cManager *web5c.CInMemoryKeyManager
}

func NewInMemoryKeyManager() (*InMemoryKeyManager, error) {
	cManager, err := web5c.NewCInMemoryKeyManager()
	if err != nil {
		return nil, err
	}

	return &InMemoryKeyManager{cManager}, nil
}

func (m *InMemoryKeyManager) ImportPrivateJwk(privateJWK *crypto.JWK) (*crypto.JWK, error) {
	cJWK := privateJWK.ToCJWK()

	cPublicJWK, err := m.cManager.ImportPrivateJwk(cJWK)
	if err != nil {
		return nil, err
	}

	publicJWK := crypto.NewJWKFromCJWK(cPublicJWK)

	return publicJWK, nil
}

func (m *InMemoryKeyManager) GetSigner(publicJWK *crypto.JWK) (dsa.Signer, error) {
	cJWK := publicJWK.ToCJWK()

	cSigner, err := m.cManager.GetSigner(cJWK)
	if err != nil {
		return nil, err
	}

	signer := dsa.NewSignerFromCSigner(cSigner)
	return signer, nil
}
