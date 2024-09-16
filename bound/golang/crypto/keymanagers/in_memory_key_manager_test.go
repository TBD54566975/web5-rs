package keymanagers_test

import (
	"encoding/base64"
	"testing"
	"web5/crypto"
	"web5/crypto/keymanagers"
)

func Test_HappyPath(t *testing.T) {
	manager, _ := keymanagers.NewInMemoryKeyManager()

	privateJWK := &crypto.JWK{
		ALG: "Ed25519",
		KTY: "OKP",
		CRV: "Ed25519",
		D:   "UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g",
		X:   "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q",
		Y:   "",
	}

	publicJWK, _ := manager.ImportPrivateJwk(privateJWK)

	signer, _ := manager.GetSigner(publicJWK)

	payload := []byte("Test message")
	signature, _ := signer.Sign(payload)

	encodedSignature := base64.RawURLEncoding.EncodeToString(signature)

	expectedSignature := "OFVgQLhFq9_Xq4atZqZA47qUSKKuBcuiSJ8SUfU8Yx75AA2vwJS7MzJi-QzX765lxUg5WTuQBSjPtfCVeRYVBw"
	if encodedSignature != expectedSignature {
		t.Errorf("Expected signature %s, but got %s", expectedSignature, encodedSignature)
	}
}
