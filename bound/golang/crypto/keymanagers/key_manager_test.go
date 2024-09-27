package keymanagers_test

import (
	"encoding/base64"
	"fmt"
	"testing"
	"web5/crypto"
	"web5/crypto/dsa"
	"web5/crypto/keymanagers"
	"web5/web5c"
)

type POCGoSigner struct{}

func (s *POCGoSigner) Sign(payload []byte) ([]byte, error) {
	jwk := crypto.JWK{
		ALG: "Ed25519",
		KTY: "OKP",
		CRV: "Ed25519",
		D:   "UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g",
		X:   "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q",
		Y:   "",
	}
	signer, _ := dsa.NewEd25519Signer(jwk)
	signature, _ := signer.Sign(payload)

	encoded := base64.RawURLEncoding.EncodeToString(signature)
	fmt.Println("Base64 Encoded (from golang):", encoded)
	return payload, nil
}

type POCGoKeyManager struct{}

func (k *POCGoKeyManager) ImportPrivateJWK(privateJWK *crypto.JWK) (*crypto.JWK, error) {
	return privateJWK, nil
}

func (k *POCGoKeyManager) GetSigner(publicJWK *crypto.JWK) (dsa.Signer, error) {
	return &POCGoSigner{}, nil
}

func Test_POCKeyManagerFromGo(t *testing.T) {
	keyManager := POCGoKeyManager{}
	importFunc := keymanagers.ToCImportPrivateJWK(&keyManager)
	getSignerFunc := keymanagers.ToCGetSigner(&keyManager)
	cKeyManager := web5c.RegisterKeyManager(importFunc, getSignerFunc)
	defer web5c.FreeKeyManager(cKeyManager.ID)
	web5c.POCKeyManagerFromForeign(cKeyManager)
}
