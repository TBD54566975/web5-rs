package crypto

type JWK struct {
	ALG string `json:"alg,omitempty"`
	KTY string `json:"kty,omitempty"`
	CRV string `json:"crv,omitempty"`
	D   string `json:"d,omitempty"`
	X   string `json:"x,omitempty"`
	Y   string `json:"y,omitempty"`
}

func (j JWK) ComputeThumbprint() (string, error) {
	// TODO 
	
	return "todo", nil
}