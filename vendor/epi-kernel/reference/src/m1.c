/**
 * m1.c — Paramasiva: Mathematical DNA (.rodata + API)
 *
 * All matrix data lives in .rodata (const). No magic numbers.
 * Every value traces to the 16:9 foundational ratio.
 */

#include "m1.h"
#include "psychoid_numbers.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ===================================================================
 * ANANDA_BIMBA — #X+0 Original source matrix
 * 12x12, nibble-packed (72 bytes).
 * Row 0: all zeros (ground).
 * Rows 1-9: digital-root multiplication table (n*col mod 9, 0->9).
 * Rows 10-11: shadow extension (= rows 1-2).
 * =================================================================== */

const DR_Matrix_12x12 ANANDA_BIMBA = { .packed_cells = {
    0x00,0x00,0x00,0x00,0x00,0x00,  /* Row  0: 0,0,0,0,0,0,0,0,0,0,0,0 */
    0x10,0x32,0x54,0x76,0x98,0x21,  /* Row  1: 0,1,2,3,4,5,6,7,8,9,1,2 */
    0x20,0x64,0x18,0x53,0x97,0x42,  /* Row  2: 0,2,4,6,8,1,3,5,7,9,2,4 */
    0x30,0x96,0x63,0x39,0x96,0x63,  /* Row  3: 0,3,6,9,3,6,9,3,6,9,3,6 */
    0x40,0x38,0x27,0x16,0x95,0x84,  /* Row  4: 0,4,8,3,7,2,6,1,5,9,4,8 */
    0x50,0x61,0x72,0x83,0x94,0x15,  /* Row  5: 0,5,1,6,2,7,3,8,4,9,5,1 */
    0x60,0x93,0x36,0x69,0x93,0x36,  /* Row  6: 0,6,3,9,6,3,9,6,3,9,6,3 */
    0x70,0x35,0x81,0x46,0x92,0x57,  /* Row  7: 0,7,5,3,1,8,6,4,2,9,7,5 */
    0x80,0x67,0x45,0x23,0x91,0x78,  /* Row  8: 0,8,7,6,5,4,3,2,1,9,8,7 */
    0x90,0x99,0x99,0x99,0x99,0x99,  /* Row  9: 0,9,9,9,9,9,9,9,9,9,9,9 */
    0x10,0x32,0x54,0x76,0x98,0x21,  /* Row 10: shadow (= row 1) */
    0x20,0x64,0x18,0x53,0x97,0x42   /* Row 11: shadow (= row 2) */
}};

/* ===================================================================
 * ANANDA_PRATIBIMBA — #X+1 Offset reflection matrix
 * Base offset: each cell = DR((row+1) * col), rows start at 1.
 * Rows 10-11: shadow extension (= rows 1-2).
 * =================================================================== */

const DR_Matrix_12x12 ANANDA_PRATIBIMBA = { .packed_cells = {
    0x11,0x11,0x11,0x11,0x11,0x11,  /* Row  0: 1,1,1,1,1,1,1,1,1,1,1,1 */
    0x21,0x43,0x65,0x87,0x19,0x32,  /* Row  1: 1,2,3,4,5,6,7,8,9,1,2,3 */
    0x31,0x75,0x29,0x64,0x18,0x53,  /* Row  2: 1,3,5,7,9,2,4,6,8,1,3,5 */
    0x41,0x17,0x74,0x41,0x17,0x74,  /* Row  3: 1,4,7,1,4,7,1,4,7,1,4,7 */
    0x51,0x49,0x38,0x27,0x16,0x95,  /* Row  4: 1,5,9,4,8,3,7,2,6,1,5,9 */
    0x61,0x72,0x83,0x94,0x15,0x26,  /* Row  5: 1,6,2,7,3,8,4,9,5,1,6,2 */
    0x71,0x14,0x47,0x71,0x14,0x47,  /* Row  6: 1,7,4,1,7,4,1,7,4,1,7,4 */
    0x81,0x46,0x92,0x57,0x13,0x68,  /* Row  7: 1,8,6,4,2,9,7,5,3,1,8,6 */
    0x91,0x78,0x56,0x34,0x12,0x89,  /* Row  8: 1,9,8,7,6,5,4,3,2,1,9,8 */
    0x11,0x11,0x11,0x11,0x11,0x11,  /* Row  9: 1,1,1,1,1,1,1,1,1,1,1,1 */
    0x21,0x43,0x65,0x87,0x19,0x32,  /* Row 10: shadow (= row 1) */
    0x31,0x75,0x29,0x64,0x18,0x53   /* Row 11: shadow (= row 2) */
}};

/* ===================================================================
 * ANANDA_SUM — (#X+0)+(#X+1) synthesis matrix
 * Each cell = DR(BIMBA[r][c] + PRATIBIMBA[r][c]).
 * Rows 10-11: shadow extension (= rows 1-2).
 * =================================================================== */

const DR_Matrix_12x12 ANANDA_SUM = { .packed_cells = {
    0x11,0x11,0x11,0x11,0x11,0x11,  /* Row  0: 1,1,1,1,1,1,1,1,1,1,1,1 */
    0x31,0x75,0x29,0x64,0x18,0x53,  /* Row  1: 1,3,5,7,9,2,4,6,8,1,3,5 */
    0x51,0x49,0x38,0x27,0x16,0x95,  /* Row  2: 1,5,9,4,8,3,7,2,6,1,5,9 */
    0x71,0x14,0x47,0x71,0x14,0x47,  /* Row  3: 1,7,4,1,7,4,1,7,4,1,7,4 */
    0x91,0x78,0x56,0x34,0x12,0x89,  /* Row  4: 1,9,8,7,6,5,4,3,2,1,9,8 */
    0x21,0x43,0x65,0x87,0x19,0x32,  /* Row  5: 1,2,3,4,5,6,7,8,9,1,2,3 */
    0x41,0x17,0x74,0x41,0x17,0x74,  /* Row  6: 1,4,7,1,4,7,1,4,7,1,4,7 */
    0x61,0x72,0x83,0x94,0x15,0x26,  /* Row  7: 1,6,2,7,3,8,4,9,5,1,6,2 */
    0x81,0x46,0x92,0x57,0x13,0x68,  /* Row  8: 1,8,6,4,2,9,7,5,3,1,8,6 */
    0x11,0x11,0x11,0x11,0x11,0x11,  /* Row  9: 1,1,1,1,1,1,1,1,1,1,1,1 */
    0x31,0x75,0x29,0x64,0x18,0x53,  /* Row 10: shadow (= row 1) */
    0x51,0x49,0x38,0x27,0x16,0x95   /* Row 11: shadow (= row 2) */
}};

/* ===================================================================
 * ANANDA_QUINTESSENCE — Dyadic {bimba_dr, sum_dr} per cell
 * C5 synthesis: source (C0/Bimba) + sum (C5/Pratibimba) together.
 * DIFF_A = -1 invariant; no storage needed (M1_QUINT_DIFF).
 * =================================================================== */

const Quintessence_Matrix ANANDA_QUINTESSENCE = {
    .bimba = {
        0x00,0x00,0x00,0x00,0x00,0x00,
        0x10,0x32,0x54,0x76,0x98,0x21,
        0x20,0x64,0x18,0x53,0x97,0x42,
        0x30,0x96,0x63,0x39,0x96,0x63,
        0x40,0x38,0x27,0x16,0x95,0x84,
        0x50,0x61,0x72,0x83,0x94,0x15,
        0x60,0x93,0x36,0x69,0x93,0x36,
        0x70,0x35,0x81,0x46,0x92,0x57,
        0x80,0x67,0x45,0x23,0x91,0x78,
        0x90,0x99,0x99,0x99,0x99,0x99,
        0x10,0x32,0x54,0x76,0x98,0x21,
        0x20,0x64,0x18,0x53,0x97,0x42
    },
    .sum = {
        0x11,0x11,0x11,0x11,0x11,0x11,
        0x31,0x75,0x29,0x64,0x18,0x53,
        0x51,0x49,0x38,0x27,0x16,0x95,
        0x71,0x14,0x47,0x71,0x14,0x47,
        0x91,0x78,0x56,0x34,0x12,0x89,
        0x21,0x43,0x65,0x87,0x19,0x32,
        0x41,0x17,0x74,0x41,0x17,0x74,
        0x61,0x72,0x83,0x94,0x15,0x26,
        0x81,0x46,0x92,0x57,0x13,0x68,
        0x11,0x11,0x11,0x11,0x11,0x11,
        0x31,0x75,0x29,0x64,0x18,0x53,
        0x51,0x49,0x38,0x27,0x16,0x95
    }
};

/* ===================================================================
 * DR RINGS — dual-track architecture
 * Mahamaya:   {1,2,4,8,7,5} — doubling ring (64-bit word)
 * Parashakti: {3,6,9,3,6,9} — tripling ring (72-name space)
 * =================================================================== */

const uint8_t DR_RING_MAHAMAYA[6]   = { 1, 2, 4, 8, 7, 5 };
const uint8_t DR_RING_PARASHAKTI[6] = { 3, 6, 9, 3, 6, 9 };

/* ===================================================================
 * M1_BRANCH_QL_CATEGORY — 6-entry .rodata category assignments
 * Indexed 0-5 = #1-0 (Bimba) through #1-5 (Toroidal).
 * =================================================================== */

const M1_QL_Category M1_BRANCH_QL_CATEGORY[6] = {
    M1_QL_CAT_IMPLICATE,            /* #1-0 Bimba */
    M1_QL_CAT_IMPLICATE_EXPLICATE,  /* #1-1 Pratibimba */
    M1_QL_CAT_EXPLICATE_1,          /* #1-2 Ananda */
    M1_QL_CAT_EXPLICATE_4,          /* #1-3 Spanda */
    M1_QL_CAT_EXPLICATE_3,          /* #1-4 QL Flowering */
    M1_QL_CAT_IMPLICATE_BOUNDARY,   /* #1-5 Toroidal */
};

/* ===================================================================
 * M1_M0_CROSSLINK — 12-entry pointer table linking Ananda ring to M0
 * Entries 0-5: ascending  (Psychoid_0 through Psychoid_5)
 * Entries 6-11: descending Möbius return (Psychoid_5 through Psychoid_0)
 * =================================================================== */

const Holographic_Coordinate* const M1_M0_CROSSLINK[12] = {
    &Psychoid_0, &Psychoid_1, &Psychoid_2,
    &Psychoid_3, &Psychoid_4, &Psychoid_5,   /* 0-5: ascending  */
    &Psychoid_5, &Psychoid_4, &Psychoid_3,
    &Psychoid_2, &Psychoid_1, &Psychoid_0    /* 6-11: descending (Möbius) */
};

/* ===================================================================
 * SPANDA_CF_SUBSTAGE_LUT — 6 sub-stages of SPANDA_FLOWERING (#1-3-4)
 *
 * Each sub-stage is a depth of contextual flowering within stage 4.
 * The element_count/fold_count traces the genus-0→genus-1 puncture:
 *   4  → first stable quaternary sides of torus
 *   6  → minimal circulation (Siva's 5 acts + Svatantrya)
 *   8  → nested resolution increase (O(2,4), genus-2 preview)
 *   10 → dual-track quantum superposition (T1/T2 double-slit)
 *   12 → complete double-torus blueprint (O(2,6) or O(3,4))
 *   0  → meta/percentile: 100% = 64+36; Möbius return completes
 * =================================================================== */

const CF_Substage_Entry SPANDA_CF_SUBSTAGE_LUT[6] = {
    {
        0, 4, 4, 0, "(4.0000)",
        "{0/0,0/1,1/0,1/1} — 4-fold static: Void-Void,Void-Form,Form-Void,Form-Form"
    },
    {
        1, 6, 6, 0, "(4.0/1)",
        "0=(0/0)->(0/1)->(1/0)->(1/1)=1 — 6-fold dynamic; O(1,5)xN(0); Siva 5+Svatantrya"
    },
    {
        2, 8, 8, 0, "(4.0/1/2)",
        "0/(0/1)+(1/0)/0 — 8-fold nested states; O(2,4) pixel density; genus-2 preview"
    },
    {
        3, 10, 10, 1, "(4.0/1/2/3)",
        "[0/0,((0/1)/(1/0)),{T1:0/(0/1),0/1},{T2:(1/0)/0,1/0},(1/0+0/1),1/1] — 10-fold dual-track"
    },
    {
        4, 12, 12, 0, "(4.4.0-4.4/5)",
        "infinite differential potential — O(2,6)/O(3,4) 12-fold; double-torus blueprint"
    },
    {
        5, 0, 0, 0, "(4.5/0)",
        "Mobius return: synthesis=void=one-point; genus-1 torus completes; 100%=64+36=16/9"
    },
};

/* ===================================================================
 * QL_FLOWERING — 6-stage mod-6 ring
 * Each stage: {index, name, formulation, next, inverse}
 * Formulations sourced from Spanda Genesis canonical document.
 * =================================================================== */

const QL_Stage QL_FLOWERING[6] = {
    { 0, "Ground",      "genus-0 sphere: (0/1) seed; 16/9=4^2/3^2 generative ratio",    1, 5 },
    { 1, "Form",        "4+2 frame: 4 explicate + 2 implicate = 6-fold torus necessity", 2, 4 },
    { 2, "Entity",      "12-fold ring: P*P'=36(Parashakti), P/P'=64(Mahamaya); 100%",   3, 3 },
    { 3, "Pattern",     "bi-12 double-cover: Mahamaya 64-bit x Parashakti 72-space",     4, 2 },
    { 4, "Context",     "7/8/9/10-fold nesting variants; lemniscate CF sub-staging",     5, 1 },
    { 5, "Integration", "1-24-fold meta-sieve; Mobius 5/0 return; ceiling=24",           0, 0 },
};

/* ===================================================================
 * SPANDA MUTATORS — static, not exported; referenced only via
 * SPANDA_COMPILER_PASSES[].
 * Each pass writes weave_state and inversion_state onto the target HC.
 * =================================================================== */

static void spanda_pass_seed(PRATIBIMBA* hc) {
    hc->weave_state     = 0.0f;
    hc->inversion_state = 0;
}
static void spanda_pass_pole_a(PRATIBIMBA* hc) {
    hc->weave_state     = 1.0f;
    hc->inversion_state = 0;
}
static void spanda_pass_pole_b(PRATIBIMBA* hc) {
    hc->weave_state     = 1.0f;
    hc->inversion_state = 1;
}
static void spanda_pass_trika(PRATIBIMBA* hc) {
    hc->weave_state     = 1.5f;
    hc->inversion_state = 0;
}
static void spanda_pass_flower(PRATIBIMBA* hc) {
    hc->weave_state     = 4.0f;
    hc->inversion_state = 0;
}
static void spanda_pass_meta(PRATIBIMBA* hc) {
    hc->weave_state     = 5.0f;
    hc->inversion_state = 1;
}

/* ===================================================================
 * SPANDA_COMPILER_PASSES — 6 typed mutators in Spanda stage order
 * Lives in .rodata: function pointers are const.
 * =================================================================== */

const Spanda_Mutator SPANDA_COMPILER_PASSES[6] = {
    spanda_pass_seed,
    spanda_pass_pole_a,
    spanda_pass_pole_b,
    spanda_pass_trika,
    spanda_pass_flower,
    spanda_pass_meta,
};

/* ===================================================================
 * PUBLIC API
 * =================================================================== */

bool m1_verify(void) {
    if (get_ananda_harmonic(&ANANDA_BIMBA, 1, 1) != 1u) return false;
    if (get_ananda_harmonic(&ANANDA_BIMBA, 9, 1) != 9u) return false;
    if (get_ananda_harmonic(&ANANDA_PRATIBIMBA, 0, 0) != 1u) return false;
    if (get_ananda_harmonic(&ANANDA_PRATIBIMBA, 3, 3) != 1u) return false;
    if (get_ananda_harmonic(&ANANDA_SUM, 1, 2) != 5u) return false;
    if (get_quint_bimba(1, 1) != get_ananda_harmonic(&ANANDA_BIMBA, 1, 1)) return false;
    if (get_quint_sum(1, 2) != get_ananda_harmonic(&ANANDA_SUM, 1, 2)) return false;
    if (DR_RING_MAHAMAYA[0] != 1u || DR_RING_MAHAMAYA[3] != 8u) return false;
    if (DR_RING_PARASHAKTI[0] != 3u || DR_RING_PARASHAKTI[2] != 9u) return false;
    return verify_m1_m0_crosslink();
}

M1_Root* m1_init(Coordinate_Arena* arena, Holographic_Coordinate* hc) {
    (void)arena;
    if (!hc) return NULL;
    M1_Root* root = (M1_Root*)malloc(sizeof(M1_Root));
    if (!root) return NULL;
    memset(root, 0, sizeof(M1_Root));
    root->hc         = hc;
    root->active_cf  = cf_get(CF_BINARY);
    root->spanda.stage       = SPANDA_SEED;
    root->spanda.state_bits  = SPANDA_SEED_BITS;
    root->spanda.track       = 0;
    root->spanda.cf_substage = 0;
    root->torus_pos  = 0;
    root->ananda     = &ANANDA_BIMBA;
    HC_LINK(hc, root);
    return root;
}

void m1_teardown(M1_Root* root) {
    if (!root) return;
    if (root->hc) HC_UNLINK(root->hc);
    free(root);
}

/* ------------------------------------------------------------------ *
 * ANANDA #1-2 MATRICES
 * ------------------------------------------------------------------ */

static uint8_t _ananda_core[6][10][10];
static uint8_t _ananda_dr[6][10][10];
static int _ananda_initialized = 0;

static uint8_t _ananda_digital_root(uint8_t n) {
    if (n == 0) return 0;
    uint8_t r = n % 9;
    return (r == 0) ? 9u : r;
}

static void _ananda_init(void) {
    if (_ananda_initialized) return;
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            _ananda_core[0][i][j] = (uint8_t)((i * j) % 10);
            _ananda_core[1][i][j] = (uint8_t)(((i * j) + 1) % 10);
            _ananda_core[2][i][j] = (uint8_t)(((2 * i * j) + 1) % 10);
            _ananda_core[3][i][j] = 9u;
            _ananda_core[4][i][j] = 1u;
            _ananda_core[5][i][j] = (uint8_t)(((i * j) ^ ((i * j) + 1)) % 10);
        }
    }
    for (int m = 0; m < 6; m++)
        for (int i = 0; i < 10; i++)
            for (int j = 0; j < 10; j++)
                _ananda_dr[m][i][j] = _ananda_digital_root(_ananda_core[m][i][j]);
    _ananda_initialized = 1;
}

uint8_t m1_ananda_get(uint8_t matrix_idx, uint8_t row, uint8_t col) {
    _ananda_init();
    if (matrix_idx >= 6 || row >= 10 || col >= 10) return 0;
    return _ananda_core[matrix_idx][row][col];
}

uint8_t m1_ananda_dr_get(uint8_t matrix_idx, uint8_t row, uint8_t col) {
    _ananda_init();
    if (matrix_idx >= 6 || row >= 10 || col >= 10) return 0;
    return _ananda_dr[matrix_idx][row][col];
}

int m1_ananda_verify_axiom(void) {
    _ananda_init();
    for (int i = 0; i < 10; i++)
        for (int j = 0; j < 10; j++) {
            uint8_t diff = (uint8_t)((_ananda_core[1][i][j] - _ananda_core[0][i][j] + 10) % 10);
            if (diff != 1) return 0;
        }
    return 1;
}

int m1_cli_dispatch(int argc, char** argv, M1_Root* root) {
    if (argc < 2) {
        printf("[m1] Paramasiva — Mathematical DNA (CF: 0/1 Binary)\n");
        printf("[m1] torus_pos: %u | spanda_stage: %u | track: %u\n",
               root->torus_pos, (uint8_t)root->spanda.stage, root->spanda.track);
        printf("[m1] Commands: ananda, pratibimba, sum, quint, spanda, ring, ql\n");
        return 0;
    }
    if (strcmp(argv[1], "ananda") == 0) {
        printf("[m1] ANANDA_BIMBA (12x12 digi-rooted):\n");
        for (int r = 0; r < 12; r++) {
            for (int c = 0; c < 12; c++)
                printf("%2u ", get_ananda_harmonic(&ANANDA_BIMBA, (uint8_t)r, (uint8_t)c));
            printf("\n");
        }
        return 0;
    }
    if (strcmp(argv[1], "pratibimba") == 0) {
        printf("[m1] ANANDA_PRATIBIMBA (12x12 digi-rooted):\n");
        for (int r = 0; r < 12; r++) {
            for (int c = 0; c < 12; c++)
                printf("%2u ", get_ananda_harmonic(&ANANDA_PRATIBIMBA, (uint8_t)r, (uint8_t)c));
            printf("\n");
        }
        return 0;
    }
    if (strcmp(argv[1], "sum") == 0) {
        printf("[m1] ANANDA_SUM (12x12 digi-rooted):\n");
        for (int r = 0; r < 12; r++) {
            for (int c = 0; c < 12; c++)
                printf("%2u ", get_ananda_harmonic(&ANANDA_SUM, (uint8_t)r, (uint8_t)c));
            printf("\n");
        }
        return 0;
    }
    if (strcmp(argv[1], "quint") == 0) {
        printf("[m1] ANANDA_QUINTESSENCE (DIFF_A=%d constant):\n", M1_QUINT_DIFF);
        printf("[m1] Bimba layer (C0 source):\n");
        for (int r = 0; r < 12; r++) {
            for (int c = 0; c < 12; c++)
                printf("%2u ", get_quint_bimba((uint8_t)r, (uint8_t)c));
            printf("\n");
        }
        printf("[m1] Sum layer (C5 integration):\n");
        for (int r = 0; r < 12; r++) {
            for (int c = 0; c < 12; c++)
                printf("%2u ", get_quint_sum((uint8_t)r, (uint8_t)c));
            printf("\n");
        }
        return 0;
    }
    if (strcmp(argv[1], "spanda") == 0) {
        printf("[m1] Spanda stage: %u | track: %u | cf_sub: %u | state_bits: 0x%02X | dual_track: %u\n",
               (uint8_t)root->spanda.stage, root->spanda.track,
               root->spanda.cf_substage, root->spanda.state_bits,
               root->spanda.dual_track_active);
        printf("[m1] 100%% = %u (Mahamaya %u + Parashakti %u) = 16/9 = 4^2/3^2\n",
               QL_PERCENTILE_TOTAL, QL_PERCENTILE_MAHAMAYA, QL_PERCENTILE_PARASHAKTI);
        printf("[m1] SPANDA_FLOWERING CF sub-stages (#1-3-4):\n");
        for (int i = 0; i < 6; i++) {
            const CF_Substage_Entry* e = &SPANDA_CF_SUBSTAGE_LUT[i];
            printf("[m1]  [%d] %-16s fold=%u elem=%u dual=%u  %s\n",
                   e->substage, e->cf_notation,
                   e->fold_count, e->element_count, e->dual_track,
                   e->formulation);
        }
        return 0;
    }
    if (strcmp(argv[1], "ring") == 0) {
        printf("[m1] Mahamaya  ring (doubling, 64-bit): {");
        for (int i = 0; i < 6; i++)
            printf("%u%s", DR_RING_MAHAMAYA[i], i < 5 ? "," : "}\n");
        printf("[m1] Parashakti ring (tripling, 72-name): {");
        for (int i = 0; i < 6; i++)
            printf("%u%s", DR_RING_PARASHAKTI[i], i < 5 ? "," : "}\n");
        return 0;
    }
    if (strcmp(argv[1], "ql") == 0) {
        printf("[m1] QL_FLOWERING (6-stage mod-6 ring):\n");
        for (int i = 0; i < 6; i++)
            printf("[m1]  [%d] %-12s  next=%u inv=%u  %s\n",
                   QL_FLOWERING[i].stage, QL_FLOWERING[i].name,
                   QL_FLOWERING[i].next, QL_FLOWERING[i].inverse,
                   QL_FLOWERING[i].formulation);
        return 0;
    }
    fprintf(stderr, "[m1] Unknown command: %s\n", argv[1]);
    return 1;
}
