package dsa_test

import (
	"encoding/base64"
	"testing"
	"web5/crypto"
	"web5/crypto/dsa"
)

func Test_Ed25519Sign(t *testing.T) {
	jwk := crypto.JWK{
		ALG: "Ed25519",
		KTY: "OKP",
		CRV: "Ed25519",
		D:   "UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g",
		X:   "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q",
		Y:   "",
	}
	signer, err := dsa.NewEd25519Signer(jwk)

	if err != nil {
		t.Fatalf("Failed to instantiate Ed25519Signer: %v", err)
	}

	payload := []byte("Test message")

	signature, err := signer.Sign(payload)
	if err != nil {
		t.Fatalf("Failed to sign: %v", err)
	}

	encodedSignature := base64.RawURLEncoding.EncodeToString(signature)

	expectedSignature := "OFVgQLhFq9_Xq4atZqZA47qUSKKuBcuiSJ8SUfU8Yx75AA2vwJS7MzJi-QzX765lxUg5WTuQBSjPtfCVeRYVBw"
	if encodedSignature != expectedSignature {
		t.Errorf("Expected signature %s, but got %s", expectedSignature, encodedSignature)
	}
}
