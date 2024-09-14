package crypto_test

import (
	"testing"
	"web5/crypto"
)

func Test_ComputeThumbprint_ECValid(t *testing.T) {
	jwk := crypto.JWK{
		ALG: "",
		KTY: "EC",
		CRV: "secp256k1",
		D:   "",
		X:   "x_value",
		Y:   "y_value",
	}

	thumbprint, err := jwk.ComputeThumbprint()
	if err != nil {
		t.Fatalf("Failed to compute thumbprint: %v", err)
	}

	expected := "yiiszVT5Lwt6760MW19cHaJ61qJKIfe20sUW8dNxBv4"
	if thumbprint != expected {
		t.Errorf("Expected thumbprint %v, but got %v", expected, thumbprint)
	}
}

func Test_ComputeThumbprint_OKPValid(t *testing.T) {
	jwk := crypto.JWK{
		ALG: "",
		KTY: "OKP",
		CRV: "Ed25519",
		D:   "",
		X:   "x_value",
		Y:   "",
	}

	thumbprint, err := jwk.ComputeThumbprint()
	if err != nil {
		t.Fatalf("Failed to compute thumbprint: %v", err)
	}

	expected := "nDMRVZm4lpedGjuJGO4y3YVJJ0krDF0aSz4KhlncDdI"
	if thumbprint != expected {
		t.Errorf("Expected thumbprint %v, but got %v", expected, thumbprint)
	}
}