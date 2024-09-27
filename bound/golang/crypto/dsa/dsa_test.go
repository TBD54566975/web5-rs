package dsa_test

import (
	"encoding/base64"
	"fmt"
	"testing"
	"web5/crypto"
	"web5/crypto/dsa"
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

func Test_POCSignerFromGo(t *testing.T) {
	signer := POCGoSigner{}
	cSigner := web5c.RegisterSigner(signer.Sign)
	defer web5c.FreeSigner(cSigner.ID)

	web5c.POCSignerFromForeign(cSigner)
}
