/**
 * engine.c — The Torus Engine
 *
 * Torus walk (6-step QL cycle), Spanda walk (12-step M1 tick cycle),
 * Lemniscate dive, double covering.
 *
 * Canonical walk vocabulary (spec: 00-canonical-invariants §6):
 *   engine_torus_walk  = 6-step QL cycle (#0-#5), coordinate-driven
 *   engine_spanda_walk = 12-step M1 spanda walk (tick12 0-11, 30°/tick)
 */

#include "engine.h"
#include "psychoid_numbers.h"
#include <stdio.h>

static const struct { uint8_t from; const Holographic_Coordinate* to; } torus_map[] = {
    { 0, &Psychoid_1 },
    { 1, &Psychoid_2 },
    { 2, &Psychoid_3 },
    { 3, &Psychoid_4 },
    { 4, &Psychoid_5 },
    { 5, &Psychoid_0 },
};

#define TORUS_MAP_SIZE 6

const Holographic_Coordinate* engine_next_coordinate(uint8_t position) {
    for (int i = 0; i < TORUS_MAP_SIZE; i++) {
        if (torus_map[i].from == position) return torus_map[i].to;
    }
    return NULL;
}

void engine_torus_walk(
    const Holographic_Coordinate* start,
    void* context_state,
    uint32_t steps
) {
    if (!start) return;
    const Holographic_Coordinate* current = start;
    uint32_t limit = (steps == 0) ? 6 : steps;

    for (uint32_t i = 0; i < limit; i++) {
        if (current->invoke_process) {
            current->invoke_process((Holographic_Coordinate*)current, context_state);
        }
        /* At #4, optionally dive into the Lemniscate */
        if (current->ql_position == 4 && current->cf != NULL) {
            engine_lemniscate_dive(current, context_state, 1);
        }
        const Holographic_Coordinate* next = engine_next_coordinate(current->ql_position);
        if (!next) break;
        current = next;
    }
}

void engine_lemniscate_dive(
    const Holographic_Coordinate* c4_entry,
    void* context_state,
    uint8_t depth
) {
    if (!c4_entry || depth == 0 || !c4_entry->cf) return;
    const Holographic_Coordinate* inner = GET_PTR(c4_entry->cf);
    if (inner->invoke_process) {
        inner->invoke_process((Holographic_Coordinate*)inner, context_state);
    }
    engine_lemniscate_dive(inner, context_state, depth - 1);
}

void engine_double_covering(
    const Holographic_Coordinate* start,
    void* context_state
) {
    if (!start) return;

    /* First 360°: normal covering */
    engine_torus_walk(start, context_state, 6);

    /* Phase transition: if context_state has Walk_Context, flip covering */
    if (context_state) {
        Walk_Context* wc = (Walk_Context*)context_state;
        wc->covering = 1;  /* Inverted phase */
    }

    /* Second 360°: inverted covering */
    engine_torus_walk(start, context_state, 6);

    /* Return to normal */
    if (context_state) {
        Walk_Context* wc = (Walk_Context*)context_state;
        wc->covering = 0;
    }
}


/* M1 Paramasiva Spanda walk — 12 discrete ticks of 30° each.
 * tick12 = 0-11; each tick = one spanda beat on the double-helix.
 * Spec: 00-canonical-invariants §6, 03-spanda-double-helix §12-fold
 */
void engine_spanda_walk(
    uint8_t tick12_start,
    uint8_t tick_count,
    void* context_state,
    void (*on_tick)(uint8_t tick12, void* ctx)
) {
    if (!on_tick || tick_count == 0) return;
    for (uint8_t i = 0; i < tick_count; i++) {
        uint8_t tick = (uint8_t)((tick12_start + i) % 12);
        on_tick(tick, context_state);
    }
}

/* =============================================================================
 * Walk_Mode dispatcher — engine_walk_by_mode()
 * ============================================================================= */

/* Adapter: wraps torus walk to emit position via Walk_Callback.
 * context_state->accumulator stores the Walk_Callback.
 * We use a small shim struct to carry both callback and user context. */

typedef struct {
    Walk_Callback cb;
    void*         user_ctx;
    uint32_t      emitted;
} WalkAdapterCtx;

static void spanda_to_walk_cb(uint8_t tick12, void* ctx) {
    WalkAdapterCtx* adapter = (WalkAdapterCtx*)ctx;
    if (adapter && adapter->cb) {
        adapter->cb(tick12, adapter->user_ctx);
    }
}

void engine_walk_by_mode(
    Walk_Mode mode,
    const Holographic_Coordinate* start,
    void* context_state,
    Walk_Callback callback,
    uint32_t steps
) {
    /* Handle NULL callback and zero steps safely */
    if (!callback) return;

    switch (mode) {
        case WALK_GROUND: {
            /* Emit start position once, then return */
            uint8_t pos = start ? start->ql_position : 0;
            callback(pos, context_state);
            break;
        }

        case WALK_TORUS: {
            /* Torus walk uses invoke_process — we need to temporarily install
             * our adapter as the invoke_process on each psychoid, then walk.
             * Simpler approach: manually step through torus map with callback. */
            if (!start) { callback(0, context_state); break; }
            uint32_t limit = (steps == 0) ? 6 : steps;
            const Holographic_Coordinate* current = start;
            for (uint32_t i = 0; i < limit; i++) {
                callback(current->ql_position, context_state);
                const Holographic_Coordinate* next = engine_next_coordinate(current->ql_position);
                if (!next) break;
                current = next;
            }
            break;
        }

        case WALK_FIBER: {
            /* Lemniscate dive — emit positions along cf chain */
            if (!start) { callback(0, context_state); break; }
            uint8_t depth = (steps == 0) ? 4 : (uint8_t)(steps > 255 ? 255 : steps);
            const Holographic_Coordinate* current = start;
            for (uint8_t d = 0; d < depth; d++) {
                callback(current->ql_position, context_state);
                if (!current->cf) break;
                const Holographic_Coordinate* inner = GET_PTR(current->cf);
                if (!inner) break;
                current = inner;
            }
            break;
        }

        case WALK_SPANDA: {
            /* 12-step spanda walk — use engine_spanda_walk with adapter */
            uint8_t tick_count = (steps == 0) ? 12 : (uint8_t)(steps > 255 ? 255 : steps);
            WalkAdapterCtx adapter = { .cb = callback, .user_ctx = context_state, .emitted = 0 };
            engine_spanda_walk(0, tick_count, &adapter, spanda_to_walk_cb);
            break;
        }
    }
}


/* =============================================================================
 * VAK INSTRUCTION DISPATCH — operates on the 6 reflective coordinates
 * ============================================================================= */

#include "vak.h"

/* Default handler table — all return VAK_ERR_FAMILY until M-branches register */
static int vak_default_handler(Holographic_Coordinate* s, VAK_Instruction i) {
    (void)s; (void)i;
    return VAK_ERR_FAMILY;
}

static VAK_Handler vak_handlers[VAK_FAMILY_COUNT] = {
    vak_default_handler, vak_default_handler, vak_default_handler,
    vak_default_handler, vak_default_handler, vak_default_handler
};

void vak_register_handler(uint8_t family, VAK_Handler handler) {
    if (family < VAK_FAMILY_COUNT && handler) {
        vak_handlers[family] = handler;
    }
}

int execute_vak_instruction(Holographic_Coordinate* session,
                            VAK_Instruction instr) {
    if (instr.vak_family >= VAK_FAMILY_COUNT) return VAK_ERR_FAMILY;
    if (!session) return VAK_ERR_SESSION;
    return vak_handlers[instr.vak_family](session, instr);
}
