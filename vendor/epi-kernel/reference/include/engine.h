/**
 * engine.h — The Engine API
 *
 * Torus walk, Lemniscate dive, double covering.
 * Separated from implementation for clean include graph.
 */

#ifndef ENGINE_H
#define ENGINE_H

#include "ontology.h"
#include <stdint.h>

/* Single QL Torus cycle: #0 → #1 → ... → #5 → #0 (6-step walk).
 * For the canonical 12-step M1 Spanda walk (30°/step), use engine_spanda_walk(). */
void engine_torus_walk(
    const Holographic_Coordinate* start,
    void* context_state,
    uint32_t steps
);

/* M1 Paramasiva Spanda walk: 12 discrete ticks of 30° each (tick12 = 0–11).
 * Maps to the WALK_SPANDA mode in the cosmic-clock spec.
 * Spec: 00-canonical-invariants §6, 03-spanda-double-helix §12-fold
 * tick12_start: first tick to execute (0–11).
 * tick_count: number of ticks to walk (wraps mod 12). */
void engine_spanda_walk(
    uint8_t tick12_start,
    uint8_t tick_count,
    void* context_state,
    void (*on_tick)(uint8_t tick12, void* ctx)
);

/* Lemniscate recursion from #4 via .cf chain */
void engine_lemniscate_dive(
    const Holographic_Coordinate* c4_entry,
    void* context_state,
    uint8_t depth
);

/* Full 720° double covering directed by cs */
void engine_double_covering(
    const Holographic_Coordinate* start,
    void* context_state
);

/* Map lookup: position → next coordinate */
const Holographic_Coordinate* engine_next_coordinate(uint8_t current_position);


/* ===================================================================
 * Walk_Mode — Quaternion-directed walk selection
 *
 * The dominant quaternion component selects the walk mode:
 *   w (scalar/ground)   → WALK_GROUND  (emit start, return)
 *   x (fire/latitude)   → WALK_TORUS   (6-step QL cycle)
 *   y (water/longitude)  → WALK_FIBER   (lemniscate recursion)
 *   z (air/interaction)  → WALK_SPANDA  (12-step M1 tick cycle)
 * =================================================================== */

#include <math.h>

typedef enum {
    WALK_GROUND = 0,  /* Emit start position, return */
    WALK_TORUS  = 1,  /* QL 6-step torus cycle       */
    WALK_FIBER  = 2,  /* Lemniscate recursion via cf  */
    WALK_SPANDA = 3   /* M1 12-step spanda walk      */
} Walk_Mode;

/* Walk callback: position index + opaque context */
typedef void (*Walk_Callback)(uint8_t position, void* ctx);

/* Select walk mode from quaternion — argmax of |w|,|x|,|y|,|z| */
static inline Walk_Mode walk_mode_from_quaternion(float w, float x, float y, float z) {
    float aw = fabsf(w), ax = fabsf(x), ay = fabsf(y), az = fabsf(z);
    Walk_Mode mode = WALK_GROUND;
    float best = aw;
    if (ax > best) { best = ax; mode = WALK_TORUS; }
    if (ay > best) { best = ay; mode = WALK_FIBER; }
    if (az > best) { mode = WALK_SPANDA; }
    (void)best;
    return mode;
}

/* Bifurcation parameter: magnitude of imaginary part sqrt(x²+y²+z²) */
static inline float walk_bifurcation_param(float w, float x, float y, float z) {
    (void)w;
    return sqrtf(x * x + y * y + z * z);
}

/* Resolution level: quantize lambda into 4 bands */
static inline uint8_t walk_resolution_level(float lambda) {
    if (lambda < 0.25f) return 0;
    if (lambda < 0.50f) return 1;
    if (lambda < 0.75f) return 2;
    return 3;
}

/* Unified walk dispatcher — selects engine function by Walk_Mode.
 * callback receives position indices as walk progresses.
 * start: HC for torus/fiber modes (may be NULL for ground/spanda).
 * steps: iteration count (0 = mode default). */
void engine_walk_by_mode(
    Walk_Mode mode,
    const Holographic_Coordinate* start,
    void* context_state,
    Walk_Callback callback,
    uint32_t steps
);


/* ===================================================================
 * Walk_Type — 9 structural walk types (SEPARATE from Walk_Mode)
 *
 * Walk_Mode = quaternion-selected traversal strategy.
 * Walk_Type = structural geometry of the walk (step count / ring).
 * =================================================================== */

typedef enum {
    WALK_TYPE_DEGREE       = 0,  /* 360 steps */
    WALK_TYPE_AMINO        = 1,  /* 24 steps  */
    WALK_TYPE_ZODIAC       = 2,  /* 12 steps  */
    WALK_TYPE_SPANDA       = 3,  /* 12 steps  */
    WALK_TYPE_DECAN        = 4,  /* 36 steps  */
    WALK_TYPE_HEXAGRAM     = 5,  /* 64 steps  */
    WALK_TYPE_ENNEADIC     = 6,  /* 9 steps   */
    WALK_TYPE_SEASONAL     = 7,  /* 4 steps   */
    WALK_TYPE_LINE_CHANGE  = 8   /* 384 steps */
} Walk_Type;
#define WALK_TYPE_COUNT 9u

static const uint16_t WALK_TYPE_STEPS[WALK_TYPE_COUNT] = {
    360, 24, 12, 12, 36, 64, 9, 4, 384
};


#include "vak.h"  /* VAK instruction dispatch */

#endif /* ENGINE_H */
