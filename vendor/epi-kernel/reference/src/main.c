/**
 * main.c — The Ignition Switch
 *
 * Boots the system, verifies the coordinate web,
 * initializes arenas, instantiates families, runs the engine.
 */

#include <stdio.h>
#include <string.h>
#include "ontology.h"
#include "psychoid_numbers.h"
#include "engine.h"
#include "arena.h"
#include "m0.h"
#include "m1.h"
#include "m2.h"
#include "m3.h"
#include "m4.h"
#include "m5.h"

static int boot_verify_web(void) {
    int ok = 1;

    /* --- Core psychoid wiring --- */
    if (GET_PTR(Psychoid_0.c) != &Psychoid_0) {
        fprintf(stderr, "[boot] FAIL: #0.c does not self-reference.\n");
        ok = 0;
    }
    if (GET_PTR(Psychoid_5.c) != &Psychoid_0) {
        fprintf(stderr, "[boot] FAIL: #5.c does not return to #0 (Möbius broken).\n");
        ok = 0;
    }
    if (GET_PTR(Psychoid_4.cf) != &Psychoid_4) {
        fprintf(stderr, "[boot] FAIL: #4.cf does not self-reference (Lemniscate broken).\n");
        ok = 0;
    }
    if (GET_PTR(Psychoid_3.cf) != &Psychoid_4) {
        fprintf(stderr, "[boot] FAIL: #3.cf does not anchor to #4.\n");
        ok = 0;
    }

    /* --- Psychoid_Hash boundary psychoid --- */
    if (Psychoid_Hash.ql_position != 0xFF) {
        fprintf(stderr, "[boot] FAIL: Psychoid_Hash.ql_position != 0xFF.\n");
        ok = 0;
    }
    if (GET_PTR(Psychoid_Hash.c) != &Psychoid_0) {
        fprintf(stderr, "[boot] FAIL: Psychoid_Hash.c does not return to #0.\n");
        ok = 0;
    }
    if (GET_PTR(Psychoid_Hash.cf) != &Psychoid_4) {
        fprintf(stderr, "[boot] FAIL: Psychoid_Hash.cf does not anchor to #4.\n");
        ok = 0;
    }

    /* --- All 7 CF roots must anchor to Psychoid_4 via .cf --- */
    const Holographic_Coordinate* cf_roots[] = {
        &CF_0000, &CF_01, &CF_012, &CF_0123, &CF_4x, &CF_450, &CF_50
    };
    const char* cf_names[] = {
        "CF_0000", "CF_01", "CF_012", "CF_0123", "CF_4x", "CF_450", "CF_50"
    };
    for (int i = 0; i < 7; i++) {
        if (GET_PTR(cf_roots[i]->cf) != &Psychoid_4) {
            fprintf(stderr, "[boot] FAIL: %s.cf does not anchor to #4.\n", cf_names[i]);
            ok = 0;
        }
    }

    /* --- All BIMBA entities must have FLAG_BIMBA set --- */
    const Holographic_Coordinate* all_bimba[] = {
        &Psychoid_0, &Psychoid_1, &Psychoid_2, &Psychoid_3, &Psychoid_4, &Psychoid_5,
        &Psychoid_Hash,
        &Weave_0_0, &Weave_0_5, &Weave_5_0, &Weave_5_5,
        &CF_0000, &CF_01, &CF_012, &CF_0123, &CF_4x, &CF_450, &CF_50
    };
    for (int i = 0; i < 18; i++) {
        if (!(all_bimba[i]->flags & FLAG_BIMBA)) {
            fprintf(stderr, "[boot] FAIL: BIMBA entity %d missing FLAG_BIMBA.\n", i);
            ok = 0;
        }
        if (!(all_bimba[i]->flags & FLAG_STATUS_CANONICAL)) {
            fprintf(stderr, "[boot] FAIL: BIMBA entity %d missing FLAG_STATUS_CANONICAL.\n", i);
            ok = 0;
        }
    }

    if (ok) {
        printf("[boot] .rodata web: OK (18 BIMBA entities verified)\n");
        printf("[boot] Möbius return (#5 -> #0): OK\n");
        printf("[boot] Lemniscate anchor (#4.cf -> #4): OK\n");
        printf("[boot] Psychoid_Hash (#=0xFF): OK\n");
        printf("[boot] 7 CF roots anchored to #4: OK\n");
        printf("[boot] All BIMBA flags: OK\n");
    }
    return ok;
}

int main(int argc, char** argv) {
    printf("=== Epi-Logos C — System Boot ===\n\n");

    /* Phase 1: Verify .rodata bedrock */
    if (!boot_verify_web()) {
        fprintf(stderr, "[boot] Aborting: .rodata web malformed.\n");
        return 1;
    }

    /* Phase 2: Initialize arena (Shakti) */
    Coordinate_Arena arena;
    if (arena_init(&arena, 64) != 0) {
        fprintf(stderr, "[boot] Aborting: arena init failed.\n");
        return 1;
    }
    printf("[boot] Arena initialized (64 slots).\n");

    /* Phase 3: Create mutable mirrors of #0-#5 */
    Holographic_Coordinate* mirrors[6];
    const Holographic_Coordinate* raw[] = {
        &Psychoid_0, &Psychoid_1, &Psychoid_2,
        &Psychoid_3, &Psychoid_4, &Psychoid_5
    };
    for (int i = 0; i < 6; i++) {
        mirrors[i] = arena_alloc(&arena);
        mirrors[i]->ql_position = raw[i]->ql_position;
        mirrors[i]->family = FAMILY_NONE;
        mirrors[i]->weave_state = raw[i]->weave_state;
        mirrors[i]->invoke_process = raw[i]->invoke_process;
        mirrors[i]->c = (i == 0) ? mirrors[0] : mirrors[i - 1];
    }
    /* Möbius: mirror #5.c -> mirror #0 */
    mirrors[5]->c = mirrors[0];
    /* Lemniscate: mirror #4.cf -> mirror #4 */
    mirrors[4]->cf = mirrors[4];
    mirrors[3]->cf = mirrors[4];
    printf("[boot] Mutable mirrors created.\n");

    /* Phase 4: Instantiate families */
    if (families_init(&arena, mirrors) != 0) {
        fprintf(stderr, "[boot] Aborting: family init failed.\n");
        arena_destroy(&arena);
        return 1;
    }
    families_crosslink(&arena);
    families_wire_reflective(&arena);
    printf("[boot] All 6 families instantiated and cross-linked (%u slots used).\n", arena.count);

    /* Phase 4.5: Initialize M0 (Anuttara) */
    M0_Root* m0 = m0_init(&arena, mirrors[0]);
    if (!m0) {
        fprintf(stderr, "[boot] Aborting: M0 init failed.\n");
        arena_destroy(&arena);
        return 1;
    }
    if (!m0_verify_holographic_registry()) {
        fprintf(stderr, "[boot] FAIL: M0 holographic registry broken.\n");
        arena_destroy(&arena);
        return 1;
    }
    printf("[boot] M0 (Anuttara) initialized. CF=VOID. 12-fold archetypes loaded.\n");

    /* Phase 4.6: Initialize M1 (Paramasiva) */
    M1_Root* m1 = m1_init(&arena, mirrors[1]);
    if (!m1) {
        fprintf(stderr, "[boot] Aborting: M1 init failed.\n");
        m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    if (!m1_verify()) {
        fprintf(stderr, "[boot] FAIL: M1 verification failed.\n");
        m1_teardown(m1);
        m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    printf("[boot] M1 (Paramasiva) initialized. CF=BINARY. Ananda+Spanda+QL loaded.\n");

    /* Phase 4.7: Initialize M2 (Parashakti) */
    M2_Root* m2 = m2_init(&arena, mirrors[2]);
    if (!m2) {
        fprintf(stderr, "[boot] Aborting: M2 init failed.\n");
        m1_teardown(m1);
        m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    if (!m2_verify()) {
        fprintf(stderr, "[boot] FAIL: M2 verification failed.\n");
        m2_teardown(m2);
        m1_teardown(m1);
        m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    printf("[boot] M2 (Parashakti) initialized. CF=TRIKA. 72-space loaded.\n");

    /* Phase 4.8: Initialize M3 (Mahamaya) */
    M3_Root* m3 = m3_init(&arena, mirrors[3]);
    if (!m3) {
        fprintf(stderr, "[boot] Aborting: M3 init failed.\n");
        m2_teardown(m2);
        m1_teardown(m1);
        m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    if (!m3_verify()) {
        fprintf(stderr, "[boot] FAIL: M3 verification failed.\n");
        m3_teardown(m3);
        m2_teardown(m2);
        m1_teardown(m1);
        m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    printf("[boot] M3 (Mahamaya) initialized. CF=QUATERNAL. 64-fold transcription loaded.\n");

    /* Phase 4.9: Initialize M4 (Nara) */
    M4_Root* m4 = m4_init(&arena, mirrors[4]);
    if (!m4) {
        fprintf(stderr, "[boot] Aborting: M4 init failed.\n");
        m3_teardown(m3); m2_teardown(m2); m1_teardown(m1); m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    if (!m4_verify()) {
        fprintf(stderr, "[boot] FAIL: M4 verification failed.\n");
        m4_teardown(m4); m3_teardown(m3); m2_teardown(m2); m1_teardown(m1); m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    printf("[boot] M4 (Nara) initialized. CF=FRACTAL. Vtable[6] + PCO loaded.\n");

    /* Phase 5.0: Initialize M5 (Epii) */
    M5_Root* m5 = m5_init(&arena, mirrors[5]);
    if (!m5) {
        fprintf(stderr, "[boot] Aborting: M5 init failed.\n");
        m4_teardown(m4); m3_teardown(m3); m2_teardown(m2); m1_teardown(m1); m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    if (!m5_verify()) {
        fprintf(stderr, "[boot] FAIL: M5 verification failed.\n");
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2); m1_teardown(m1); m0_teardown(m0);
        arena_destroy(&arena);
        return 1;
    }
    printf("[boot] M5 (Epii) initialized. CF=MOBIUS. Logos FSM + QV self-API loaded.\n");

    /* CLI dispatch */
    if (argc > 1 && strcmp(argv[1], "m0") == 0) {
        int rc = m0_cli_dispatch(argc - 1, argv + 1, m0);
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2);
        m1_teardown(m1); m0_teardown(m0); arena_destroy(&arena);
        return rc;
    }
    if (argc > 1 && strcmp(argv[1], "m1") == 0) {
        int rc = m1_cli_dispatch(argc - 1, argv + 1, m1);
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2);
        m1_teardown(m1); m0_teardown(m0); arena_destroy(&arena);
        return rc;
    }
    if (argc > 1 && strcmp(argv[1], "m2") == 0) {
        int rc = m2_cli_dispatch(argc - 1, argv + 1, m2);
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2);
        m1_teardown(m1); m0_teardown(m0); arena_destroy(&arena);
        return rc;
    }
    if (argc > 1 && strcmp(argv[1], "m3") == 0) {
        int rc = m3_cli_dispatch(argc - 1, argv + 1, m3);
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2);
        m1_teardown(m1); m0_teardown(m0); arena_destroy(&arena);
        return rc;
    }
    if (argc > 1 && strcmp(argv[1], "m4") == 0) {
        int rc = m4_cli_dispatch(argc - 1, argv + 1, m4);
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2);
        m1_teardown(m1); m0_teardown(m0); arena_destroy(&arena);
        return rc;
    }
    if (argc > 1 && strcmp(argv[1], "m5") == 0) {
        int rc = m5_cli_dispatch(argc - 1, argv + 1, m5);
        m5_teardown(m5); m4_teardown(m4); m3_teardown(m3); m2_teardown(m2);
        m1_teardown(m1); m0_teardown(m0); arena_destroy(&arena);
        return rc;
    }

    /* Phase 6: Run double covering (720°) */
    Walk_Context wc = {0};
    printf("\n[engine] Starting double covering (720°)...\n");
    engine_double_covering(mirrors[0], &wc);
    printf("[engine] Double covering complete: %u steps, %u cycles.\n",
           wc.step_count, wc.cycle_count);

    /* Cleanup */
    m5_teardown(m5);
    m4_teardown(m4);
    m3_teardown(m3);
    m2_teardown(m2);
    m1_teardown(m1);
    m0_teardown(m0);
    arena_destroy(&arena);

    printf("\n=== Möbius return: #5 -> #0. Tomorrow's ground is richer. ===\n");
    return 0;
}
