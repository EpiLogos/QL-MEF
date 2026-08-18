/**
 * psychoid_numbers.h — The Promises of the Bedrock
 *
 * Extern declarations for the seven psychoid-numbers (#0-#5 + Hash),
 * four weave interleaves, and seven context frame roots.
 * These are the pre-categorical foundation — FAMILY_NONE.
 * C0-C5 are C-family MANIFESTATIONS, not these.
 */

#ifndef PSYCHOID_NUMBERS_H
#define PSYCHOID_NUMBERS_H

#include "ontology.h"

/* =============================================================================
 * I. THE SEVEN PSYCHOID-NUMBERS — Extern Promises
 *    These live in .rodata: immutable, transcendent, Siva.
 *    family = FAMILY_NONE — pre-categorical.
 * ============================================================================= */

extern BIMBA Psychoid_0;     /* Ground      — #0 */
extern BIMBA Psychoid_1;     /* Form        — #1 */
extern BIMBA Psychoid_2;     /* Operation   — #2 */
extern BIMBA Psychoid_3;     /* Pattern     — #3 */
extern BIMBA Psychoid_4;     /* Context     — #4 (Lemniscate) */
extern BIMBA Psychoid_5;     /* Integration — #5 (Möbius) */
extern BIMBA Psychoid_Hash;  /* The # Inversion Act — boundary psychoid */

/* =============================================================================
 * II. THE FOUR INTERLACED WEAVE STATES
 * ============================================================================= */

extern BIMBA Weave_0_0;    /* Pure Ground implicate (0.0) */
extern BIMBA Weave_0_5;
extern BIMBA Weave_5_0;
extern BIMBA Weave_5_5;

/* =============================================================================
 * III. THE SEVEN CONTEXT FRAME ROOTS
 *    Processual execution contexts — all anchored to Psychoid_4 via .cf
 * ============================================================================= */

extern BIMBA CF_0000;   /* Receptive Dynamism  (00/00) Mod%   */
extern BIMBA CF_01;     /* Non-Dual Binary     (0/1)  Mod 2  */
extern BIMBA CF_012;    /* The Trika           (0/1/2) Mod 3  */
extern BIMBA CF_0123;   /* Three-Plus-One      (0/1/2/3) Mod 4 */
extern BIMBA CF_4x;     /* Fractal Doubling    (4.0/1-4.4/5): '.' nests sub-pos in #4 */
extern BIMBA CF_450;    /* Möbius Synthesis    (4.5/0): '.' = #4 nests #5; '/' = return */
extern BIMBA CF_50;     /* Total Synthesis      (5/0) Mod 6   */

/* =============================================================================
 * III-B. CANONICAL CF_TABLE — Index by CF_Id
 *
 *    Single .rodata lookup table for all context frames.
 *    M-branch modules MUST reference context frames via CF_TABLE[CF_*]
 *    rather than naming CF_ entities directly — this keeps them rooted in
 *    the canonical context frame system and lets the CLI dispatch by index.
 *
 *    All 7 entries point to BIMBA (.rodata) entities above.
 *    All entries are anchored to Psychoid_4 via their .cf field.
 * ============================================================================= */

typedef enum {
    CF_VOID        = 0,  /* (00/00) Mod%      — Receptive Dynamism        */
    CF_BINARY      = 1,  /* (0/1)   Mod 2     — Non-Dual Binary           */
    CF_TRIKA       = 2,  /* (0/1/2) Mod 3     — The Trika                 */
    CF_QUATERNAL   = 3,  /* (0/1/2/3) Mod 4   — Three-Plus-One            */
    CF_FRACTAL     = 4,  /* (4.0/1-4.4/5)     — Fractal Doubling; '.' nests sub-pos in #4 */
    CF_SYNTHESIS   = 5,  /* (4.5/0)            — Möbius Synthesis; '.' = #4 nests #5     */
    CF_MOBIUS      = 6,  /* (5/0)   Mod 6     — Total Synthesis           */
    CF_COUNT       = 7
} CF_Id;

/* Canonical pointer table — defined in psychoid_numbers.c */
extern const Holographic_Coordinate* CF_TABLE[CF_COUNT];

/* Lookup by CF_Id — safe, bounds-checked */
static inline const Holographic_Coordinate* cf_get(CF_Id id) {
    return (id < CF_COUNT) ? CF_TABLE[id] : NULL;
}

/* M-branch HC anchoring helpers:
 *
 *   HC_LINK(hc, m_struct):
 *     Bidirectionally bind an HC to its M-branch payload struct.
 *     (hc)->payload.process_state = (m_struct)
 *     (m_struct)->hc = (hc)
 *     M-branch structs MUST declare 'Holographic_Coordinate* hc;' as first field.
 *
 *   HC_UNLINK(hc):
 *     Clear the payload link on teardown.
 */
#define HC_LINK(hc, m_struct) \
    do { (hc)->payload.process_state = (m_struct); \
         (m_struct)->hc = (hc); } while(0)

#define HC_UNLINK(hc) \
    do { (hc)->payload.process_state = NULL; } while(0)

/* =============================================================================
 * IV. WALK CONTEXT — Passed through the entire Torus cycle
 * ============================================================================= */

typedef struct {
    uint8_t  current_position;    /* Where we are in the walk      */
    uint8_t  covering;            /* 0 = normal, 1 = inverted      */
    uint32_t step_count;          /* Total steps taken             */
    uint32_t cycle_count;         /* Complete 360° cycles          */
    void*    accumulator;         /* Generic payload for processing */
} Walk_Context;

/* =============================================================================
 * V. EXECUTE FUNCTION PROTOTYPES
 *    The () operator implementations for each psychoid position.
 * ============================================================================= */

void Execute_Ground(Holographic_Coordinate* self, void* context_state);
void Execute_Form(Holographic_Coordinate* self, void* context_state);
void Execute_Entity(Holographic_Coordinate* self, void* context_state);
void Execute_Process(Holographic_Coordinate* self, void* context_state);
void Execute_Lemniscate(Holographic_Coordinate* self, void* context_state);
void Execute_Integration(Holographic_Coordinate* self, void* context_state);
void Execute_Hash(Holographic_Coordinate* self, void* context_state);

/* CF root execution stubs */
void Execute_CF_Void(Holographic_Coordinate* self, void* context_state);
void Execute_CF_Binary(Holographic_Coordinate* self, void* context_state);
void Execute_CF_Trika(Holographic_Coordinate* self, void* context_state);
void Execute_CF_Quaternal(Holographic_Coordinate* self, void* context_state);
void Execute_CF_Fractal(Holographic_Coordinate* self, void* context_state);
void Execute_CF_Synthesis(Holographic_Coordinate* self, void* context_state);
void Execute_CF_Mobius(Holographic_Coordinate* self, void* context_state);


#endif /* PSYCHOID_NUMBERS_H */
