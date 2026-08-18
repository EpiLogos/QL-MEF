/**
 * ontology.h — The Master Blueprint
 *
 * The universal DNA of the Epi-Logos coordinate system.
 * Contains ONLY definitions and promises — no memory allocated here.
 *
 * What lives here:
 *   - Coordinate_Family enum (NONE for raw psychoids, P/S/T/M/L/C for families)
 *   - Tagged Pointer macros (baking ontology into addresses)
 *   - Forward declaration and Context_Execution_Operator signature
 *   - The Holographic_Coordinate struct (128 bytes, universal for all coordinates)
 *   - BIMBA / PRATIBIMBA type aliases
 *   - BEDROCK accessor macro
 *   - Status/flags byte definitions
 *
 * What does NOT live here:
 *   - Any actual psychoid data (.rodata belongs in archetypes.c)
 *   - Any function bodies (implementations belong in src/)
 */

#ifndef ONTOLOGY_H
#define ONTOLOGY_H

#include <stddef.h>
#include <stdint.h>
#include <stdalign.h>


/* =============================================================================
 * I. COORDINATE FAMILY ENUM
 *    Distinguishes raw psychoids (#0-#5) from family manifestations.
 *    FAMILY_NONE = raw psychoid (Layer 1, pre-categorical).
 *    P through C = family manifestations (Layer 2).
 * ============================================================================= */

typedef enum {
    FAMILY_C    = 0,   /* Category  — #0 Ontological foundation              */
    FAMILY_P    = 1,   /* Position  — #1 Functional semantics                */
    FAMILY_L    = 2,   /* Lens      — #2 Epistemic modes                     */
    FAMILY_S    = 3,   /* Stack     — #3 Technology / infrastructure layers  */
    FAMILY_T    = 4,   /* Thought   — #4 Artifacts / cognition               */
    FAMILY_M    = 5,   /* Map (Bimba) — #5 Consciousness domains             */
    FAMILY_NONE = 7    /* Raw psychoid — pre-categorical (no archetype resonance) */
} Coordinate_Family;


/* =============================================================================
 * II. TAGGED POINTER MACROS
 *    The operators baked into memory addresses.
 *
 *    Bit layout:
 *      63:    # (FLAG_INVERTED)   — the inversion act
 *      62:    . (FLAG_NESTING)    — inward fractal traversal
 *      61:    - (FLAG_BRANCHING)  — lateral relation
 *      60:    () (FLAG_EXECUTING) — execution context active
 *      59-56: FAMILY_BITS         — target coordinate family (4 bits)
 *      55-48: ARCH_BITS           — target ql_position (8 bits)
 *      47-0:  physical address
 *
 *    SAFETY CONTRACT: GET_PTR() before EVERY dereference. No exceptions.
 * ============================================================================= */

#define MASK_ADDRESS    0x0000FFFFFFFFFFFF
#define FLAG_INVERTED   0x8000000000000000
#define FLAG_NESTING    0x4000000000000000
#define FLAG_BRANCHING  0x2000000000000000
#define FLAG_EXECUTING  0x1000000000000000

/* Strip all flags — MUST call before dereferencing */
#define GET_PTR(tagged)  ((Holographic_Coordinate*)((uintptr_t)(tagged) & MASK_ADDRESS))

/* Inversion (#) — the fundamental operation */
#define SET_INVERTED(ptr)    ((Holographic_Coordinate*)((uintptr_t)(ptr) | FLAG_INVERTED))
#define IS_INVERTED(ptr)     ((uintptr_t)(ptr) & FLAG_INVERTED)

/* Nesting (.) — inward fractal traversal */
#define SET_NESTING(ptr)     ((Holographic_Coordinate*)((uintptr_t)(ptr) | FLAG_NESTING))
#define IS_NESTING(ptr)      ((uintptr_t)(ptr) & FLAG_NESTING)

/* Branching (-) — lateral relation */
#define SET_BRANCHING(ptr)   ((Holographic_Coordinate*)((uintptr_t)(ptr) | FLAG_BRANCHING))
#define IS_BRANCHING(ptr)    ((uintptr_t)(ptr) & FLAG_BRANCHING)

/* Executing (()) — context active */
#define SET_EXECUTING(ptr)   ((Holographic_Coordinate*)((uintptr_t)(ptr) | FLAG_EXECUTING))
#define IS_EXECUTING(ptr)    ((uintptr_t)(ptr) & FLAG_EXECUTING)

/* Utilities */
#define CLEAR_FLAGS(ptr)     GET_PTR(ptr)
#define TAG_FLAGS(ptr)       ((uintptr_t)(ptr) & ~MASK_ADDRESS)

/* --- FAMILY_BITS: bits 59-56 (4 bits, values 0-7 for Coordinate_Family) --- */
#define FAMILY_BITS_SHIFT  56
#define FAMILY_BITS_MASK   ((uintptr_t)0x0F00000000000000)

#define SET_FAMILY_BITS(ptr, fam) \
    ((Holographic_Coordinate*)( \
        ((uintptr_t)(ptr) & ~FAMILY_BITS_MASK) | \
        (((uintptr_t)(fam) & 0xF) << FAMILY_BITS_SHIFT) ))

#define GET_FAMILY_BITS(ptr) \
    ((Coordinate_Family)( ((uintptr_t)(ptr) >> FAMILY_BITS_SHIFT) & 0xF ))

/* --- ARCH_BITS: bits 55-48 (8 bits; 0-5 for psychoids, 0xFF for Hash) --- */
#define ARCH_BITS_SHIFT    48
#define ARCH_BITS_MASK     ((uintptr_t)0x00FF000000000000)

#define SET_ARCH_BITS(ptr, arch) \
    ((Holographic_Coordinate*)( \
        ((uintptr_t)(ptr) & ~ARCH_BITS_MASK) | \
        (((uintptr_t)(arch) & 0xFF) << ARCH_BITS_SHIFT) ))

#define GET_ARCH_BITS(ptr) \
    ((uint8_t)( ((uintptr_t)(ptr) >> ARCH_BITS_SHIFT) & 0xFF ))


/* =============================================================================
 * III. WEAVE-STATE SEMANTICS
 *    The identification edges (0.0, 0.5, 5.0, 5.5) define the topological
 *    boundary conditions. Everything between them is inter-identification
 *    process "frames". The integer part of weave_state is the parent position;
 *    the decimal part encodes the child/frame context.
 *
 *    Operator access rules:
 *      - #4 and identification edges have . (nesting) access
 *      - All other positions have - (branching) access
 *      - The SOURCE coordinate determines the operator, not the target
 * ============================================================================= */

/* Identification edges — topological boundary conditions */
#define IS_IDENTIFICATION_EDGE(w) \
    ((w) == 0.0f || (w) == 0.5f || (w) == 5.0f || (w) == 5.5f)

/* Does this coordinate have nesting (.) access?
 * Only #4 (Lemniscate) and identification edges can nest.
 * All others branch (-). */
#define HAS_NESTING_ACCESS(coord) \
    ((coord)->ql_position == 4 || IS_IDENTIFICATION_EDGE((coord)->weave_state))

/* Extract parent and child/frame from weave_state.
 * e.g. weave_state 3.2 → parent=#3, child=#2
 * e.g. weave_state 4.6 → parent=#4, child=C-family (6) */
#define WEAVE_PARENT(w)  ((uint8_t)(w))
#define WEAVE_CHILD(w)   ((uint8_t)(((w) - (float)((uint8_t)(w))) * 10.0f + 0.5f))

/* Apply the correct relational flag based on source coordinate's access */
#define TAG_RELATION(source, target_ptr) \
    (HAS_NESTING_ACCESS(source) \
        ? SET_NESTING(target_ptr) \
        : SET_BRANCHING(target_ptr))


/* =============================================================================
 * IV. FORWARD DECLARATION & EXECUTION OPERATOR
 * ============================================================================= */

typedef struct Holographic_Coordinate Holographic_Coordinate;

typedef void (*Context_Execution_Operator)(
    Holographic_Coordinate* self,
    void* context_state
);


/* =============================================================================
 * V. THE UNIVERSAL COORDINATE — 128 Bytes / 2 Cache Lines
 *
 *    Used for BOTH raw psychoids (#0-#5, family=FAMILY_NONE)
 *    and family manifestations (C0-C5, P0-P5, etc., family=FAMILY_*)
 *
 *    Layout:
 *      Identity          (8 bytes)
 *      Tensor Anchor     (8 bytes)
 *      Intra-Openness    (96 bytes = 12 tagged pointers)
 *      Execution         (8 bytes)
 *      Payload           (8 bytes)
 *      Total:            128 bytes
 * ============================================================================= */

struct Holographic_Coordinate {

    /* --- Identity (8 bytes) --- */
    uint8_t  ql_position;       /* 0-5: which psychoid position, 0xFF: Hash  */
    uint8_t  family;            /* Coordinate_Family: NONE for raw, P-C for family */
    uint8_t  inversion_state;   /* 0 = normal, 1 = inverted (result of #)     */
    uint8_t  flags;             /* Status/metadata byte — see flag definitions */
    float    weave_state;       /* 0.0, 0.5, 1.0, ... 5.5                    */

    /* --- Tensor Anchor (8 bytes) --- */
    float*   semantic_embedding; /* Points into Tensor Arena; for family coords,
                                  * dual-use as bedrock pointer via BEDROCK() */

    /* --- Intra-Openness: 6 Base + 6 Reflective (96 bytes) --- */
    Holographic_Coordinate* c;    /* #0 Category family link    */
    Holographic_Coordinate* p;    /* #1 Position family link    */
    Holographic_Coordinate* l;    /* #2 Lens family link        */
    Holographic_Coordinate* s;    /* #3 Stack family link       */
    Holographic_Coordinate* t;    /* #4 Thought family link     */
    Holographic_Coordinate* m;    /* #5 Map (Bimba) family link */

    Holographic_Coordinate* cpf;  /* Category-Position-Frame  */
    Holographic_Coordinate* ct;   /* Context-Type             */
    Holographic_Coordinate* cp;   /* Context-Position         */
    Holographic_Coordinate* cf;   /* Context-Frame (#4 anchor)*/
    Holographic_Coordinate* cfp;  /* Context-Frame-Position   */
    Holographic_Coordinate* cs;   /* Context-Sequence          */

    /* --- Execution (8 bytes) --- */
    Context_Execution_Operator invoke_process;

    /* --- Payload (8 bytes) --- */
    union {
        char*    meaning_bin;     /* Semantic content        */
        void*    process_state;   /* Runtime state           */
        uint64_t instance_id;     /* Unique instance handle  */
        float*   vector_anchor;   /* Alternate vector ref    */
    } payload;
};


/* =============================================================================
 * VI. COMPILE-TIME LAYOUT ENFORCEMENT
 * ============================================================================= */

_Static_assert(sizeof(Holographic_Coordinate) == 128,
    "Holographic_Coordinate must be exactly 128 bytes (2 cache lines)");
_Static_assert(offsetof(Holographic_Coordinate, semantic_embedding) == 8,
    "Tensor Anchor must begin at byte 8");
_Static_assert(offsetof(Holographic_Coordinate, c) == 16,
    "Intra-Openness must begin at byte 16");
_Static_assert(offsetof(Holographic_Coordinate, invoke_process) == 112,
    "Execution zone must begin at byte 112");
_Static_assert(offsetof(Holographic_Coordinate, payload) == 120,
    "Payload zone must begin at byte 120");


/* =============================================================================
 * VII. BIMBA / PRATIBIMBA TYPE ALIASES
 *
 *    BIMBA:      The Canonical Source — immutable, .rodata, can never be mutated.
 *    PRATIBIMBA:  The Manifest Instance — mutable, heap/stack, reflects its BIMBA.
 * ============================================================================= */

#define BIMBA       const Holographic_Coordinate
#define PRATIBIMBA  Holographic_Coordinate


/* =============================================================================
 * VIII. STATUS / FLAGS BYTE DEFINITIONS (Identity zone, offset 3)
 *
 *    Bits 7-6: TOPO_MODE — topological modality (see section IX-A)
 *    Bit 5:   FLAG_BIMBA — 1 if this coordinate is a .rodata canonical source
 *    Bits 4-2: ELEMENT_ID — encodes Mahabhuta elements (0-4), for M2 instances
 *    Bit 1:   STATUS_PROVISIONAL — asserted by Neo4j but unproven by C
 *    Bit 0:   STATUS_CANONICAL — C-compiler-proven and verified
 * ============================================================================= */

#define FLAG_STATUS_CANONICAL    0x01
#define FLAG_STATUS_PROVISIONAL  0x02
#define FLAG_BIMBA               0x20

/* Mahabhuta element IDs (bits 4-2) */
#define ELEMENT_AKASHA   0x00   /* Ether / Space */
#define ELEMENT_VAYU     0x04   /* Air           */
#define ELEMENT_AGNI     0x08   /* Fire          */
#define ELEMENT_APAS     0x0C   /* Water         */
#define ELEMENT_PRITHVI  0x10   /* Earth         */

#define FLAGS_ELEMENT_MASK  0x1C
#define GET_ELEMENT(coord)  ( ((coord)->flags & FLAGS_ELEMENT_MASK) >> 2 )

/* Combined flags for all BIMBA (.rodata) entities */
#define BIMBA_FLAGS  (FLAG_STATUS_CANONICAL | FLAG_BIMBA)  /* = 0x21 */


/* =============================================================================
 * IX-A. TOPOLOGICAL MODALITY — bits 7-6 of flags byte
 *
 * The inner reality of the P-family (Position) coordinate family and its
 * inversion P'. These four modes ARE the HC's topological self-understanding:
 *
 *   TOPO_TORUS       — P (inversion_state=0), outward 6-position walk.
 *                      Genus-1 surface: 4g+2g=6, χ=0, π₁=Z⊕Z (abelian),
 *                      orientable, 4-colour map. The single Torus cover.
 *
 *   TOPO_KLEIN       — P' (inversion_state=1), inward return phase.
 *                      P + P' together = full Klein double-cover (12 positions,
 *                      720°). P' completes the Klein topology. Non-orientable,
 *                      6-colour map (Heawood) = the 6 QL positions.
 *
 *   TOPO_LEMNISCATE  — P at ql_position==4 specifically (genus-2 incubation).
 *                      The figure-eight nesting anchor at #4. Already embedded
 *                      in pointer logic (cf → &Psychoid_4, HAS_NESTING_ACCESS).
 *                      Also the entry-point to C' (reflective coordinates:
 *                      cpf/ct/cp/cf/cfp/cs — the VAK language / context frames).
 *
 *   TOPO_ZERO_SPHERE — The Spanda seed: S⁰ = two disconnected points = (0/1)
 *                      as pre-differential binary ground. Applies to the six
 *                      boundary/seed coordinates: P0, P5, P0', P5', C0, C5.
 *                      "0 and 1 = 5 and 0" — the binary poles ARE the ground
 *                      and synthesis of the Torus, which ARE Bimba and Pratibimba.
 *                      See SPANDA_SEED_TOTALIZATION_INVARIANT in m1.h.
 *
 * Bits 5-0 of flags are unchanged (ELEMENT_ID, PROVISIONAL, CANONICAL, BIMBA).
 * ============================================================================= */

#define TOPO_MODE_SHIFT       6u
#define TOPO_MODE_MASK        0xC0u

#define TOPO_MODE_TORUS       0x00u   /* T² genus-1: P outward phase (default)          */
#define TOPO_MODE_LEMNISCATE  0x40u   /* T² genus-2: P at #4, cf-chain entry            */
#define TOPO_MODE_KLEIN       0x80u   /* Klein: P' return phase, completes double-cover  */
#define TOPO_MODE_ZERO_SPHERE 0xC0u   /* S⁰: Spanda seed, P0/P5/P0'/P5'/C0/C5 boundary */

#define GET_TOPO_MODE(coord) \
    ((coord)->flags & TOPO_MODE_MASK)

#define SET_TOPO_MODE(coord, mode) \
    ((coord)->flags = (uint8_t)(((coord)->flags & ~TOPO_MODE_MASK) | (uint8_t)(mode)))


/* =============================================================================
 * IX-B. BEDROCK ACCESSOR
 *
 *    For family coordinates (family != FAMILY_NONE), semantic_embedding
 *    is cast to point at the .rodata psychoid this coordinate manifests.
 *    For raw psychoids, returns NULL.
 * ============================================================================= */

#define BEDROCK(coord) \
    ( (coord)->family == FAMILY_NONE \
      ? NULL \
      : (Holographic_Coordinate*)((coord)->semantic_embedding) )


#endif /* ONTOLOGY_H */

