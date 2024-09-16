#ifndef WEB5_C_H
#define WEB5_C_H

#include <stdlib.h>

void free_string(char *s);

/** jwk */
typedef struct
{
	const char *alg;
	const char *kty;
	const char *crv;
	const char *d;
	const char *x;
	const char *y;
} CJwk;
char *jwk_compute_thumbprint(const CJwk *jwk);
/** --- */

/** dsa signer */
typedef unsigned char *(*signFunc)(int signer_id, const unsigned char *payload, size_t payload_len, size_t *out_len);
typedef struct
{
	int signer_id;
	signFunc sign;
} CSigner;
extern unsigned char *foreign_signer_sign(int signer_id, const unsigned char *payload, size_t payload_len, size_t *out_len);
unsigned char *call_sign(CSigner *signer, const unsigned char *payload, size_t payload_len, size_t *out_len);

void poc_signer_from_foreign(const CSigner *signer);
CSigner *poc_signer_from_rust();

typedef struct CEd25519Signer CEd25519Signer;
CEd25519Signer *ed25519_signer_new(const CJwk *jwk);
unsigned char *ed25519_signer_sign(CEd25519Signer *signer, const unsigned char *payload, size_t payload_len, size_t *out_len);
void ed25519_signer_free(CEd25519Signer *signer);
/** --- */

/** key managers */
typedef struct CInMemoryKeyManager CInMemoryKeyManager;

// Create a new InMemoryKeyManager instance
CInMemoryKeyManager *in_memory_key_manager_new();
CJwk *in_memory_key_manager_import_private_jwk(CInMemoryKeyManager *manager, const CJwk *private_jwk);
CSigner *in_memory_key_manager_get_signer(CInMemoryKeyManager *manager, const CJwk *public_jwk);
void in_memory_key_manager_free(CInMemoryKeyManager *manager);
/** --- */

#endif // WEB5_C_H