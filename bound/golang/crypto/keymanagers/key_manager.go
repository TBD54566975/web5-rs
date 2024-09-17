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
	cPrivateJWK := privateJWK.ToCJWK()

	cPublicJWK, err := k.cKeyManager.ImportPrivateJWK(cPrivateJWK)
	if err != nil {
		return nil, err
	}

	publicJWK := crypto.NewJWKFromCJWK(cPublicJWK)
	return publicJWK, nil
}

func (k *innerKeyManager) GetSigner(publicJWK *crypto.JWK) (dsa.Signer, error) {
	cPublicJWK := publicJWK.ToCJWK()

	cSigner, err := k.cKeyManager.GetSigner(cPublicJWK)
	if err != nil {
		return nil, err
	}

	signer := dsa.NewSignerFromCSigner(cSigner)
	return signer, nil
}

func NewKeyManagerFromCKeyManager(cKeyManager *web5c.CKeyManager) KeyManager {
	return &innerKeyManager{cKeyManager}
}

func ToCImportPrivateJWK(keyManager KeyManager) web5c.ImportFunc {
	return func(cPrivateJWK *web5c.CJWK) (*web5c.CJWK, error) {
		privateJWK := crypto.NewJWKFromCJWK(cPrivateJWK)
		publicJWK, err := keyManager.ImportPrivateJwk(privateJWK)
		if err != nil {
			return nil, err
		}
		cPublicJWK := publicJWK.ToCJWK()
		return cPublicJWK, nil
	}
}

func ToCGetSigner(keyManager KeyManager) web5c.GetSignerFunc {
	return func(cPublicJWK *web5c.CJWK) (*web5c.CSigner, error) {
		publicJWK := crypto.NewJWKFromCJWK(cPublicJWK)
		signer, err := keyManager.GetSigner(publicJWK)
		if err != nil {
			return nil, err
		}
		cSigner := web5c.RegisterSigner(signer.Sign)
		return cSigner, nil
	}
}
