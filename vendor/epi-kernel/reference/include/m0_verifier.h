/**
 * m0_verifier.h — Verifier-Anuttara position 0' R-virtue constraint-checker.
 *
 * Coordinate: S0-0 (M0 Anuttara kernel substrate, 0' face)
 * Residency: Body/S/S0/epi-lib/include/m0_verifier.h
 * Position (#0'): the R-anchor face of the 4'/5'/0' unified act — the
 *   LLM-Nara at 4' speaks recognition, the EBM-Epii at 5' scores energy,
 *   the Verifier-Anuttara at 0' raises questions with backing chains
 * Actualises: Tranche 01.T1.10 (DR-MP-1, DR-MP-3); checks state against
 *   VIRTUE_LUT[9] + M0_CORE_RELATIONS[65] + the 128-entry coordinate-language
 *   registry (anuttara_language.h) + the Track 19.9 four syntax layers
 * Public surface: KernelState, M0TypedQuery, M0CoordinateRef, M0BackingChain,
 *   M0OwlValidationReport, M0RVirtueViolation, M0PrincipleTriadCompilation,
 *   M0VerifierReport, m0_verifier_check_state(), m0_verifier_emit_query(),
 *   m0_verifier_walk_backing(), m0_verifier_emit_question()
 * Does NOT own: the registry/identity-chain tables (anuttara_language.h),
 *   reduction rules (m0_calculus.h), EnergyDecomposition (kernel.h — the
 *   report composes alongside it, never replaces it), live n10s OWL
 *   validation (S2; kernel carries the slot), gateway dispatch
 *   (s0'.verifier.* routes live in S3 gateway-contract)
 *
 * The earlier coherence_score scalar is REMOVED per Tranche 1.10: scalar
 * collapse loses the typed-exception structure. The typed queries with
 * their backing chains + the act-route trajectory + the principle-triad
 * closure marker carry what scalars cannot.
 */

#ifndef M0_VERIFIER_H
#define M0_VERIFIER_H

#include "m0.h"
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define M0_VERIFIER_VIRTUE_COUNT 9u
#define M0_VERIFIER_MAX_UNSATISFIED 80u
#define M0_VERIFIER_COORDINATE_MAX 96u
#define M0_VERIFIER_VIRTUE_THRESHOLD 0.5f
#define M0_VERIFIER_MAX_TYPED_QUERIES 16u
#define M0_VERIFIER_MAX_BACKING_CHAIN 24u
#define M0_VERIFIER_MAX_ROUTE_IN 16u
#define M0_VERIFIER_MAX_ROUTE_OUT 24u
#define M0_VERIFIER_MAX_ENGAGED_COORDS 8u
#define M0_VERIFIER_QUERY_KIND_MAX 32u

/* Backing-walk depth: principle chains default unbounded; conjugate chains
 * stop at the configured depth ([anuttara.verifier] backing_chain_depth.*,
 * Tier-3 Class-C tunability per Track 38 — the gateway passes the configured
 * value; these are the kernel defaults). */
#define M0_VERIFIER_DEPTH_UNBOUNDED 0xFFu
#define M0_VERIFIER_DEFAULT_CONJUGATE_DEPTH 4u

#define M0_VERIFIER_SYNTAX_SPEECH       (1u << 0) /* Track 19.9 layer 3 */
#define M0_VERIFIER_SYNTAX_RELATIONSHIP (1u << 1) /* Track 19.9 layer 5 */
#define M0_VERIFIER_SYNTAX_ACTION       (1u << 2) /* Track 19.9 layer 7 */
#define M0_VERIFIER_SYNTAX_COMPLETION   (1u << 3) /* Track 19.9 layer 9 */
#define M0_VERIFIER_SYNTAX_ALL \
    (M0_VERIFIER_SYNTAX_SPEECH | M0_VERIFIER_SYNTAX_RELATIONSHIP | \
     M0_VERIFIER_SYNTAX_ACTION | M0_VERIFIER_SYNTAX_COMPLETION)

typedef struct KernelState {
    uint16_t committed_virtue_mask; /* 0 means the canonical nine are committed. */
    float    virtue_evidence[M0_VERIFIER_VIRTUE_COUNT];
    uint16_t observed_core_relation_count;
    uint16_t syntax_layer_mask;
    uint8_t  active_archetype;
    uint8_t  active_tct_position;
    uint8_t  slot_privacy_boundary_compliance;
    /* Act-route trajectory input (Archetype 7 face): the RFactorPathStep[]
     * the execution stamped onto its trace (oracle cast, walk step,
     * transform stage, session close, canon promotion). */
    uint8_t  route_step_count;
    RFactorPathStep route_steps[M0_VERIFIER_MAX_ROUTE_IN];
    /* Coordinates the state engages, checked against the 128 registry. */
    uint8_t  engaged_coordinate_count;
    char     engaged_coordinates[M0_VERIFIER_MAX_ENGAGED_COORDS]
                                 [M0_VERIFIER_COORDINATE_MAX];
} KernelState;

/* The full 7-laws vocabulary — the typed-exception surface is NOT restricted
 * to Law 6's minimal (%, ?/!, ?!/!?) set. Open enum: every law of the
 * Anuttara grammar populates it. */
typedef enum {
    M0_ANUTTARA_LAW_CHIRALITY      = 1,
    M0_ANUTTARA_LAW_CONTAINMENT    = 2,
    M0_ANUTTARA_LAW_IDENTITY_CHAIN = 3,
    M0_ANUTTARA_LAW_CONNECTIVE     = 4,
    M0_ANUTTARA_LAW_EIGHT_PLUS_ONE = 5,
    M0_ANUTTARA_LAW_INTERROGATIVE  = 6,
    M0_ANUTTARA_LAW_DERIVATION     = 7,
} M0AnuttaraLaw;

typedef struct {
    uint8_t law_family; /* M0AnuttaraLaw, 1..7 */
    char    query_kind[M0_VERIFIER_QUERY_KIND_MAX];
    char    symbolic_coordinate_string[M0_VERIFIER_COORDINATE_MAX];
} M0TypedQuery;

typedef struct {
    char     coordinate[M0_VERIFIER_COORDINATE_MAX];
    uint16_t packed; /* m0.h M0C packed form; 0xFFFF when symbolic-only */
} M0CoordinateRef;

typedef struct {
    uint8_t depth;
    uint8_t grounded; /* 1 when the walk reached an M0-0/M0-1 root */
    uint8_t link_count;
    M0CoordinateRef links[M0_VERIFIER_MAX_BACKING_CHAIN];
} M0BackingChain;

/* Kernel-static ontology validation over M0_CORE_RELATIONS. Live n10s OWL
 * validation runs at S2 and is routed through s0'.verifier.owl_query; the
 * kernel carries the slot and validates what the kernel can actually see. */
typedef enum {
    M0_OWL_LIVE_DEFERRED           = 0,
    M0_OWL_KERNEL_STATIC_PASS      = 1,
    M0_OWL_KERNEL_STATIC_VIOLATION = 2,
} M0OwlValidationStatus;

typedef struct {
    uint8_t  status;             /* M0OwlValidationStatus */
    uint8_t  live_n10s_deferred; /* always 1 kernel-side */
    uint16_t checked_relation_count;
    uint16_t violation_count;
    uint16_t unresolved_endpoint_count; /* M0-branch endpoints absent from registry */
} M0OwlValidationReport;

typedef struct {
    uint8_t virtue_index; /* 0..8 into VIRTUE_LUT */
    uint8_t r_factor;     /* VIRTUE_LUT r_factor; 0xFF for principle/meta */
    float   evidence;
    char    coordinate[M0_VERIFIER_COORDINATE_MAX]; /* M0-2-9-n address */
} M0RVirtueViolation;

/* Archetype-7 closure law: a closing trajectory compiles back to the
 * principle triad (##) and (R#) and (#R). NOT_CLOSING = None (in-progress
 * sessions / partial dispatches). */
typedef enum {
    M0_TRIAD_NOT_CLOSING = 0,
    M0_TRIAD_COMPILES    = 1,
    M0_TRIAD_INCOHERENT  = 2,
} M0PrincipleTriadStatus;

#define M0_TRIAD_BIT_TRUTH    (1u << 0) /* ## — steps conform to R_FACTOR_ROUTE_TABLE */
#define M0_TRIAD_BIT_FREEDOM  (1u << 1) /* R# — operative act precedes the R5 close */
#define M0_TRIAD_BIT_OPENNESS (1u << 2) /* #R — band flip passes through (@#) */

typedef struct {
    uint8_t status;     /* M0PrincipleTriadStatus */
    uint8_t triad_bits; /* M0_TRIAD_BIT_* that compiled */
} M0PrincipleTriadCompilation;

typedef struct M0VerifierReport {
    /* Archetype 9 face — the witness vector (one bit per virtue) */
    uint16_t virtue_witness_vector;
    float    virtue_scores[M0_VERIFIER_VIRTUE_COUNT];
    uint16_t unsatisfied_count;
    char     unsatisfied_constraints[M0_VERIFIER_MAX_UNSATISFIED]
                                      [M0_VERIFIER_COORDINATE_MAX];
    uint8_t  slot_privacy_boundary_compliance;
    uint8_t  act_face;
    uint8_t  witness_face;
    /* 4-bit Archetype 3/5/7/9 calculus witness (m0_calculus, Tranche 1.13) */
    uint8_t  syntax_witness_vector;
    /* Archetype 7 face — act-route trajectory with the (@#) turn marker */
    uint8_t  route_step_count;
    uint8_t  band_turn_index; /* index of the (@#) step; 0xFF = no turn */
    RFactorPathStep r_factor_route[M0_VERIFIER_MAX_ROUTE_OUT];
    /* Typed-query exceptions, full 7-laws vocabulary */
    uint16_t typed_query_count;
    M0TypedQuery typed_queries[M0_VERIFIER_MAX_TYPED_QUERIES];
    /* Ontological-integrity trace via M0_IDENTITY_CHAINS */
    uint16_t backing_chain_count;
    M0CoordinateRef backing_chain[M0_VERIFIER_MAX_BACKING_CHAIN];
    uint8_t  canonical_membership; /* engaged coords all in the 128 registry */
    M0OwlValidationReport owl_validation;
    uint16_t r_virtue_violation_count;
    M0RVirtueViolation r_virtue_violations[M0_VERIFIER_VIRTUE_COUNT];
    M0PrincipleTriadCompilation closure_marker;
} M0VerifierReport;

int m0_verifier_check_state(const KernelState* state, M0VerifierReport* out);

/* DR-MP-3: the Verifier raises questions, not pass/fail. Emits the highest-
 * priority typed query for the state. Returns 0 when a query was emitted,
 * 1 when the state raises no question, -1 on null input. */
int m0_verifier_emit_query(const KernelState* state, M0TypedQuery* out);

/* Walks the anchor coordinate's identity chain (M0_IDENTITY_CHAINS) toward
 * the M0-0/M0-1 roots. Principle chains walk to ground regardless of depth;
 * conjugate chains stop at `depth` (M0_VERIFIER_DEPTH_UNBOUNDED walks all). */
int m0_verifier_walk_backing(
    const M0TypedQuery* q,
    const M0CoordinateRef* anchor,
    uint8_t depth,
    M0BackingChain* out
);

/* Renders the first open question of a report as a symbolic-coordinate
 * string (Tranche 1.11 EBNF). */
int m0_verifier_emit_question(
    const M0VerifierReport* report,
    char* out_buf,
    size_t buf_len
);

#ifdef __cplusplus
}
#endif

#endif /* M0_VERIFIER_H */
