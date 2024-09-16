package keymanagers

import (
	"web5/crypto"
	"web5/crypto/dsa"
	"web5/web5c"
)

type InMemoryKeyManager struct {
	manager *web5c.CInMemoryKeyManager
}

func NewInMemoryKeyManager() (*InMemoryKeyManager, error) {
	cManager, err := web5c.NewCInMemoryKeyManager()
	if err != nil {
		return nil, err
	}

	return &InMemoryKeyManager{cManager}, nil
}

func (m *InMemoryKeyManager) ImportPrivateJwk(privateJWK *crypto.JWK) (*crypto.JWK, error) {
	cJwk := web5c.NewCJwk(privateJWK.ALG, privateJWK.KTY, privateJWK.CRV, privateJWK.D, privateJWK.X, privateJWK.Y)
	defer web5c.FreeCJwk(cJwk)

	cPublicJwk, err := web5c.CInMemoryKeyManagerImportPrivateJwk(m.manager, cJwk)
	if err != nil {
		return nil, err
	}

	publicJWK := crypto.NewJWKFromCJwk(cPublicJwk)

	return publicJWK, nil
}

func (m *InMemoryKeyManager) GetSigner(publicJWK *crypto.JWK) (dsa.Signer, error) {
	cJwk := web5c.NewCJwk(publicJWK.ALG, publicJWK.KTY, publicJWK.CRV, publicJWK.D, publicJWK.X, publicJWK.Y)
	defer web5c.FreeCJwk(cJwk)

	cSigner, err := web5c.CInMemoryKeyManagerGetSigner(m.manager, cJwk)
	if err != nil {
		return nil, err
	}

	signer := dsa.NewSignerFromCSigner(cSigner)
	return signer, nil
}
