package crypto

import (
	"web5/web5c"
)

type JWK struct {
	ALG string `json:"alg,omitempty"`
	KTY string `json:"kty,omitempty"`
	CRV string `json:"crv,omitempty"`
	D   string `json:"d,omitempty"`
	X   string `json:"x,omitempty"`
	Y   string `json:"y,omitempty"`
}

func (j JWK) ComputeThumbprint() (string, error) {
	cJWK := j.ToCJWK()
	thumbprint := cJWK.ComputeThumbprint()
	return thumbprint, nil
}

func (j JWK) ToCJWK() *web5c.CJWK {
	return &web5c.CJWK{
		ALG: j.ALG,
		KTY: j.KTY,
		CRV: j.CRV,
		D:   j.D,
		X:   j.X,
		Y:   j.Y,
	}
}

func NewJWKFromCJWK(cJWK *web5c.CJWK) *JWK {
	return &JWK{
		ALG: cJWK.ALG,
		KTY: cJWK.KTY,
		CRV: cJWK.CRV,
		D:   cJWK.D,
		X:   cJWK.X,
		Y:   cJWK.Y,
	}
}
