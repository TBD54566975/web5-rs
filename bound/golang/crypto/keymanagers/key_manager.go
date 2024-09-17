package keymanagers

import (
	"web5/crypto"
	"web5/crypto/dsa"
	"web5/web5c"
)

type KeyManager interface {
	ImportPrivateJwk(privateJWK *crypto.JWK) (*crypto.JWK, error)
	GetSigner(publicJWK *crypto.JWK) (dsa.Signer, error)
}

type innerKeyManager struct {
	cKeyManager *web5c.CKeyManager
}

func (k *innerKeyManager) ImportPrivateJwk(privateJWK *crypto.JWK) (*crypto.JWK, error) {
	cPrivateJWK := web5c.NewCJwk(privateJWK.ALG, privateJWK.KTY, privateJWK.CRV, privateJWK.D, privateJWK.X, privateJWK.Y)
	defer web5c.FreeCJwk(cPrivateJWK)

	cPublicJWK, err := web5c.CKeyManagerImportPrivateJWK(k.cKeyManager, cPrivateJWK)
	if err != nil {
		return nil, err
	}

	publicJWK := crypto.NewJWKFromCJwk(cPublicJWK)
	return publicJWK, nil
}

func (k *innerKeyManager) GetSigner(publicJWK *crypto.JWK) (dsa.Signer, error) {
	cJwk := web5c.NewCJwk(publicJWK.ALG, publicJWK.KTY, publicJWK.CRV, publicJWK.D, publicJWK.X, publicJWK.Y)
	defer web5c.FreeCJwk(cJwk)

	cSigner, err := web5c.CKeyManagerGetSigner(k.cKeyManager, cJwk)
	if err != nil {
		return nil, err
	}

	signer := dsa.NewSignerFromCSigner(cSigner)
	return signer, nil
}

func NewKeyManagerFromCKeyManager(cKeyManager *web5c.CKeyManager) KeyManager {
	return &innerKeyManager{cKeyManager}
}
