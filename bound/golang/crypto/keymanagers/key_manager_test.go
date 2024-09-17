package keymanagers_test

import (
	"encoding/base64"
	"fmt"
	"testing"
	"web5/crypto"
	"web5/crypto/keymanagers"
	"web5/web5c"
)

func Test_POCKeyManagerFromRust(t *testing.T) {
	cKeyManager := web5c.POCKeyManagerFromRust()
	keyManager := keymanagers.NewKeyManagerFromCKeyManager(cKeyManager)

	privateJWK := crypto.JWK{
		ALG: "Ed25519",
		KTY: "OKP",
		CRV: "Ed25519",
		D:   "UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g",
		X:   "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q",
		Y:   "",
	}

	publicJWK, err := keyManager.ImportPrivateJwk(&privateJWK)
	if err != nil {
		t.Fatalf("Failed to import private jwk: %v", err)
	}

	signer, err := keyManager.GetSigner(publicJWK)
	if err != nil {
		t.Fatalf("Failed to get signer: %v", err)
	}

	result, err := signer.Sign([]byte("Test message"))
	if err != nil {
		t.Fatalf("Failed to sign: %v", err)
	}

	encoded := base64.RawURLEncoding.EncodeToString(result)
	fmt.Println("Base64 Encoded (from rust):", encoded)

	expectedSignature := "OFVgQLhFq9_Xq4atZqZA47qUSKKuBcuiSJ8SUfU8Yx75AA2vwJS7MzJi-QzX765lxUg5WTuQBSjPtfCVeRYVBw"
	if encoded != expectedSignature {
		t.Errorf("Expected signature %s, but got %s", expectedSignature, encoded)
	}
}