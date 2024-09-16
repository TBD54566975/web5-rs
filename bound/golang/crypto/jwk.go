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
	cJwk := web5c.NewCJwk(j.ALG, j.KTY, j.CRV, j.D, j.X, j.Y)
	defer web5c.FreeCJwk(cJwk)

	thumbprint := web5c.CJwkComputeThumbprint(cJwk)
	return thumbprint, nil
}

func NewJWKFromCJwk(cJwk *web5c.CJwk) *JWK {
	return &JWK{
		ALG: cJwk.GetALG(),
		KTY: cJwk.GetKTY(),
		CRV: cJwk.GetCRV(),
		D:   cJwk.GetD(),
		X:   cJwk.GetX(),
		Y:   cJwk.GetY(),
	}
}
