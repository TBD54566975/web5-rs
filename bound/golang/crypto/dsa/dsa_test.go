package dsa_test

import (
	"encoding/base64"
	"fmt"
	"testing"
	"web5/crypto/dsa"
)

func Test_PocSignerFromGo(t *testing.T) {
	signer := dsa.InGoSigner{}
	dsa.PocSignerFromGo(&signer)
}

func Test_PocSignerFromRust(t *testing.T) {
	signer := dsa.PocSignerFromRust()
	result, _ := signer.Sign([]byte("Test message"))
	encoded := base64.RawURLEncoding.EncodeToString(result)
	fmt.Println("Base64 Encoded (from rust):", encoded)
}
