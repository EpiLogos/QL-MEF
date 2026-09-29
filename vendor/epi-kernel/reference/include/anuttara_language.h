/**
 * anuttara_language.h — Anuttara Coordinate-Language Registry (M0/M1 reach)
 *
 * Coordinate: S0-0 (M0 Anuttara kernel substrate)
 * Residency: Body/S/S0/epi-lib/include/anuttara_language.h
 * Position (#0): Ontological Foundation — the 128-entry coordinate-language
 *   registry the 0' Verifier checks membership against
 * Actualises: Tranche 01.T1.10 + [[M0-ARCHITECTURE]] §11 (109 M0 alphabet +
 *   19 QL closure = 128 typed-calculus reach); DR-VAK-7; Tranche 1.14a
 *   M0_IDENTITY_CHAINS (equational-theory membership tables, first landing)
 * Public surface: AnuttaraEntryKind, AnuttaraLanguageEntry,
 *   ANUTTARA_LANGUAGE_REGISTRY[], ANUTTARA_LANGUAGE_REGISTRY_COUNT,
 *   anuttara_language_is_member(), anuttara_language_lookup(),
 *   anuttara_language_lookup_packed(), M0IdentityChain, M0_IDENTITY_CHAINS[],
 *   M0_IDENTITY_CHAINS_COUNT, m0_identity_chain_find()
 * Does NOT own: reduction rules (m0_calculus.h), verifier report shape
 *   (m0_verifier.h), the graph-side c_1_* anuttara-language schema (S2
 *   graph-schema), the anuttara-symbolic-parse skill (S4)
 */

#ifndef ANUTTARA_LANGUAGE_H
#define ANUTTARA_LANGUAGE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Registry entry kinds: the 109 dataset nodes plus the 19-entry QL closure
 * (7 psychoid bases + 12 coordinate types) per M0-ARCHITECTURE §11. */
typedef enum {
    ANUTTARA_ENTRY_M0_ALPHABET     = 0,
    ANUTTARA_ENTRY_PSYCHOID_BASE   = 1,
    ANUTTARA_ENTRY_COORDINATE_TYPE = 2,
} AnuttaraEntryKind;

typedef struct {
    const char* coordinate; /* canonical address, e.g. "M0-2-9-0", "O#", "P'" */
    const char* symbol;     /* c_1_symbol verbatim; NULL = canonical absence */
    const char* name;       /* c_1_name */
    uint8_t     kind;       /* AnuttaraEntryKind */
    uint16_t    packed;     /* m0.h M0C packed form; 0xFFFF when unpackable */
} AnuttaraLanguageEntry;

#define ANUTTARA_LANGUAGE_QL_CLOSURE_COUNT 19u

extern const AnuttaraLanguageEntry ANUTTARA_LANGUAGE_REGISTRY[];
extern const size_t ANUTTARA_LANGUAGE_REGISTRY_COUNT;

/* Membership over the full 128 reach: exact match on coordinate first,
 * then on symbol (glyphs like "(@#)" resolve through symbol identity). */
bool anuttara_language_is_member(const char* coordinate_or_symbol);
const AnuttaraLanguageEntry* anuttara_language_lookup(const char* coordinate);
const AnuttaraLanguageEntry* anuttara_language_lookup_packed(uint16_t packed);

/* --- M0_IDENTITY_CHAINS — Law-3 equational theory (Tranche 1.14a seam) ---
 * The corpus's =-chains compiled as ordered backing walks toward the
 * M0-0/M0-1 roots. Principle-triad chains (M0-2-9-0/1/2) ground unbounded;
 * conjugate chains (M0-2-9-3..8) traverse the M0-3/M0-4 base spine. */

#define M0_IDENTITY_CHAIN_MAX_LINKS 10u

typedef struct {
    const char* coordinate;                          /* chain anchor */
    const char* links[M0_IDENTITY_CHAIN_MAX_LINKS];  /* ordered toward ground */
    uint8_t     link_count;
    uint8_t     is_principle; /* 1 = principle/meta (depth-unbounded walk) */
} M0IdentityChain;

extern const M0IdentityChain M0_IDENTITY_CHAINS[];
extern const size_t M0_IDENTITY_CHAINS_COUNT;

const M0IdentityChain* m0_identity_chain_find(const char* coordinate);

/* --- Law-3 =-chain equivalence classes (Tranche 1.14a) ----------------
 * The corpus's long `=` chains declare identity-classes, not computations
 * (`## = @ = (0/1)-(00)-00` is one node viewed three ways). The classes
 * are derived from the registry's own c_1_symbol chains — no second data
 * source. A member is usable for modulo-rewriting only when UNAMBIGUOUS
 * (it appears in exactly one class; `@` appears in dozens and never
 * rewrites). The canonical member is the chain head (the glyph). The
 * paraconsistent `=/≠` copula stays OUT of this classical surface. */

int m0_identity_class_find(const char* term);          /* class id, or -1 */
const char* m0_identity_class_canonical(int class_id); /* head member     */
size_t m0_identity_class_count(void);

#ifdef __cplusplus
}
#endif

#endif /* ANUTTARA_LANGUAGE_H */
