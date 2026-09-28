/**
 * m1.c — Paramasiva: Mathematical DNA (.rodata + API)
 *
 * All matrix data lives in .rodata (const). No magic numbers.
 * Every value traces to the 16:9 foundational ratio.
 */

#include "m1.h"
#include "kernel.h"  /* kernel_resonance_index — the 72-space address (T2.11) */
#include "m3.h"      /* apply_epogdoon_compression — the 9:8 72→64 transduction (T2.11) */
#include "m0.h"      /* ARCHETYPE_LUT / ARCHETYPE_COORDINATE_LUT — seat binding (FR 2.1.10) */
#include "psychoid_numbers.h"
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


/* Header-remediated constants and former inline bodies. */

/* Public lookup tables relocated from the coordinate header. */
const uint8_t TOPOLOGICAL_ELEMENT_COUNT_LUT[12] = {
     1,  2,  2,  3,  4,  5,   /* Explicate phase (positions 0-5) */
     8, 10, 12,  6,  7, 11    /* Implicate phase (positions 6-11) */
};

const uint8_t SPANDA_CF_FOLD_COUNT[6] = { 4, 6, 8, 10, 12, 0 };

const Quaternion RING_QUATERNION_LUT[12] = {
    [0]  = { .w = 1.0f,    .x = 0.0f,    .y = 0.0f, .z = 0.0f },
    [1]  = { .w = 0.8660254f,  .x = 0.5f,    .y = 0.0f, .z = 0.0f },
    [2]  = { .w = 0.5f,    .x = 0.8660254f,  .y = 0.0f, .z = 0.0f },
    [3]  = { .w = 0.0f,    .x = 1.0f,    .y = 0.0f, .z = 0.0f },
    [4]  = { .w = -0.5f,   .x = 0.8660254f,  .y = 0.0f, .z = 0.0f },
    [5]  = { .w = -0.8660254f, .x = 0.5f,    .y = 0.0f, .z = 0.0f },
    [6]  = { .w = 0.8660254f,  .x = -0.5f,   .y = 0.0f, .z = 0.0f },
    [7]  = { .w = 0.5f,    .x = -0.8660254f, .y = 0.0f, .z = 0.0f },
    [8]  = { .w = 0.0f,    .x = -1.0f,   .y = 0.0f, .z = 0.0f },
    [9]  = { .w = -0.5f,   .x = -0.8660254f, .y = 0.0f, .z = 0.0f },
    [10] = { .w = -0.8660254f, .x = -0.5f,   .y = 0.0f, .z = 0.0f },
    [11] = { .w = -1.0f,   .x = 0.0f,    .y = 0.0f, .z = 0.0f },
};

const QL_Trig_Entry QL_TRIG_TABLE[6] = {
    [0] = { "sin", "sinθ",       0,          TRIG_UNITY, -1 }, /* P0 — generator pole 1       */
    [1] = { "tan", "sinθ/cosθ",  0,          5,          +1 }, /* P1 — ratio of two generators */
    [2] = { "sec", "1/cosθ",     TRIG_UNITY, 5,          +1 }, /* P2 — cos reciprocal          */
    [3] = { "cot", "cosθ/sinθ",  5,          0,          +1 }, /* P3 — inverse of tan          */
    [4] = { "csc", "1/sinθ",     TRIG_UNITY, 0,          +1 }, /* P4 — sin reciprocal          */
    [5] = { "cos", "cosθ",       5,          TRIG_UNITY, -1 }, /* P5 — generator pole 2       */
};

const uint8_t VALID_FOLDS[VALID_FOLD_COUNT] = {
    0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 16, 18, 24
};

const Cl42_Basis_Entry CL42_BASIS[6] = {
    [0] = { .position = 0, .signature = -1, .trig_fn = TRIG_SIN }, /* P0 Ground      — sinθ (generator) */
    [1] = { .position = 1, .signature = +1, .trig_fn = TRIG_TAN }, /* P1 Definition  — tanθ = sin/cos   */
    [2] = { .position = 2, .signature = +1, .trig_fn = TRIG_SEC }, /* P2 Operation   — secθ = 1/cos     */
    [3] = { .position = 3, .signature = +1, .trig_fn = TRIG_COT }, /* P3 Pattern     — cotθ = cos/sin   */
    [4] = { .position = 4, .signature = +1, .trig_fn = TRIG_CSC }, /* P4 Context     — cscθ = 1/sin     */
    [5] = { .position = 5, .signature = -1, .trig_fn = TRIG_COS }, /* P5 Integration — cosθ (generator) */
};

const uint8_t QL_INVERT[6] = { 5u, 4u, 3u, 2u, 1u, 0u };

/* Per-family implicate/explicate signature for the M1-2 ananda-vortex perspex
 * cross-fade (M1-2-ANANDA-VORTEX-ARCHITECTURE.md §5.2/§5.4). Indexed by
 * Ananda_Matrix_Op: Bimba(0) and Quintessence(5) are implicate boundaries (-1,
 * cool indigo glass); Pratibimba(1)/Sum(2)/DiffA(3)/DiffB(4) are explicate (+1,
 * warm glass). The signature is forced by canon (m1.h enum boundaries), not a
 * visual choice — the played-torus renderer reads it to tint the six glass
 * layers rather than forking the value. */
const int8_t ANANDA_FAMILY_SIGNATURE[6] = { -1, +1, +1, +1, +1, -1 };
/* The +2 net Cl(4,2) signature is preserved across the family axis exactly as
 * on the position axis: four (+1) explicate families minus two (-1) implicate
 * boundaries = +2. (Verified at runtime by m1_verify, not _Static_assert,
 * since const-array element access is not a C constant expression.) */


/* Public helper bodies relocated from the coordinate header. */
uint8_t get_ananda_harmonic(
        const DR_Matrix_12x12* mat,
        uint8_t row_0_to_11,
        uint8_t col_0_to_11)
{
    uint8_t flat = (uint8_t)((row_0_to_11 * 12u) + col_0_to_11);
    uint8_t byte_val = mat->packed_cells[flat / 2u];
    return (flat % 2u == 0u) ? (uint8_t)(byte_val & 0x0Fu) : (uint8_t)(byte_val >> 4u);
}

uint8_t get_quint_bimba(uint8_t row_0_to_11, uint8_t col_0_to_11) {
    uint8_t flat = (uint8_t)((row_0_to_11 * 12u) + col_0_to_11);
    uint8_t byte_val = ANANDA_QUINTESSENCE.bimba[flat / 2u];
    return (flat % 2u == 0u) ? (uint8_t)(byte_val & 0x0Fu) : (uint8_t)(byte_val >> 4u);
}

uint8_t get_quint_sum(uint8_t row_0_to_11, uint8_t col_0_to_11) {
    uint8_t flat = (uint8_t)((row_0_to_11 * 12u) + col_0_to_11);
    uint8_t byte_val = ANANDA_QUINTESSENCE.sum[flat / 2u];
    return (flat % 2u == 0u) ? (uint8_t)(byte_val & 0x0Fu) : (uint8_t)(byte_val >> 4u);
}

int8_t get_quint_diff(uint8_t row, uint8_t col) {
    (void)row; (void)col;
    return (int8_t)M1_QUINT_DIFF;
}

bool is_valid_fold(uint8_t n) {
    for (int i = 0; i < VALID_FOLD_COUNT; i++)
        if (VALID_FOLDS[i] == n) return true;
    return false;
}

bool ql_is_ascending(QL_Tick tick) {
    return tick < (QL_Tick)RING_HALF;
}

uint8_t ql_get_stage(QL_Tick tick) {
    return ql_is_ascending(tick) ? tick : (uint8_t)(11u - tick);
}

float quat_norm_sq(Quaternion q) {
    return q.w*q.w + q.x*q.x + q.y*q.y + q.z*q.z;
}

Quaternion quat_mul(Quaternion a, Quaternion b) {
    return (Quaternion){
        .w = a.w*b.w - a.x*b.x - a.y*b.y - a.z*b.z,
        .x = a.w*b.x + a.x*b.w + a.y*b.z - a.z*b.y,
        .y = a.w*b.y - a.x*b.z + a.y*b.w + a.z*b.x,
        .z = a.w*b.z + a.x*b.y - a.y*b.x + a.z*b.w
    };
}

Quaternion quat_conj(Quaternion q) {
    return (Quaternion){ .w = q.w, .x = -q.x, .y = -q.y, .z = -q.z };
}

Quaternion quat_neg(Quaternion q) {
    return (Quaternion){ .w = -q.w, .x = -q.x, .y = -q.y, .z = -q.z };
}

Quaternion quat_normalize(Quaternion q) {
    float norm_sq = quat_norm_sq(q);
    if (norm_sq <= 0.0f) {
        return q;
    }
    float scale = 1.0f / sqrtf(norm_sq);
    return (Quaternion){
        .w = q.w * scale,
        .x = q.x * scale,
        .y = q.y * scale,
        .z = q.z * scale
    };
}

Quaternion quat_rotate(Quaternion q, Quaternion v) {
    return quat_mul(quat_mul(q, v), quat_conj(q));
}

Quaternion quat_slerp(Quaternion a, Quaternion b, float t) {
    float dot = a.w*b.w + a.x*b.x + a.y*b.y + a.z*b.z;
    if (dot < 0.0f) {
        b = quat_neg(b);
        dot = -dot;
    }
    if (dot > 0.9995f) {
        Quaternion lerp = {
            .w = a.w + t * (b.w - a.w),
            .x = a.x + t * (b.x - a.x),
            .y = a.y + t * (b.y - a.y),
            .z = a.z + t * (b.z - a.z)
        };
        return quat_normalize(lerp);
    }
    if (dot < -1.0f) dot = -1.0f;
    if (dot > 1.0f) dot = 1.0f;
    float theta = acosf(dot);
    float sin_theta = sinf(theta);
    if (fabsf(sin_theta) < 0.0001f) {
        return quat_normalize(a);
    }
    float wa = sinf((1.0f - t) * theta) / sin_theta;
    float wb = sinf(t * theta) / sin_theta;
    return (Quaternion){
        .w = wa*a.w + wb*b.w,
        .x = wa*a.x + wb*b.x,
        .y = wa*a.y + wb*b.y,
        .z = wa*a.z + wb*b.z
    };
}

Quaternion quat_from_ring_pos(QL_Tick tick) {
    return RING_QUATERNION_LUT[tick % RING_SIZE];
}

uint8_t get_topological_element_count(uint8_t ring_pos) {
    return TOPOLOGICAL_ELEMENT_COUNT_LUT[RING_WRAP(ring_pos)];
}

uint16_t hopf_project(uint16_t exact_degree_720) {
    return exact_degree_720 % FULL_CYCLE_DEG;
}

uint8_t hopf_fiber(uint16_t exact_degree_720) {
    return (exact_degree_720 >= FULL_CYCLE_DEG) ? 1u : 0u;
}

uint8_t hopf_tick12(uint16_t exact_degree_720) {
    return (uint8_t)(hopf_project(exact_degree_720) / DEGREE_PER_TICK);
}

bool quat_is_unit(Quaternion q) {
    float norm_sq = quat_norm_sq(q);
    return fabsf(1.0f - norm_sq) < 1e-4f;
}

uint8_t get_ananda_diff_a(uint8_t row, uint8_t col) {
    (void)row; (void)col;
    return ANANDA_DIFF_A_CONSTANT;
}

uint8_t get_ananda_diff_b(uint8_t row, uint8_t col) {
    (void)row; (void)col;
    return ANANDA_DIFF_B_CONSTANT;
}

bool verify_m1_m0_crosslink(void) {
    for (int i = 0; i < 12; i++) {
        if (M1_M0_CROSSLINK[i] == NULL) return false;
    }
    return true;
}


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
    0x70,0x35,0x81,0x46,0x92,0x57,  /* Row  7: DR face 0,7,5,3,1,8,6,4,2,9,7,5; raw 7X+0 face 0,7,14,21,28,35,42,49,56,63,70,77 */
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
 * T2.11: SPANDA DUAL-OSCILLATOR — THE TICK FLOOR (M1-3)
 * Contract in m1.h. The continuous stratum beneath the integer ring;
 * the ring's M_PI ban is a ring-traversal law and does not bind the
 * field layer that generates what the ring reads out.
 * =================================================================== */

Spanda_HKB_Params spanda_hkb_params_default(void) {
    /* Derived, not guessed (m1.h contract): a normalizes the fundamental;
     * b/a = 9/16 is the 16/9 generative gap inverted (Base Frame equation:
     * 100% = 64+36 → 16/9 → 4+2); Δω = 0 — the poles are co-original;
     * 2.5 Hz = conserved-delta band centre (see citation fn). */
    return (Spanda_HKB_Params){
        .delta_omega  = 0.0,
        .a            = 1.0,
        .b            = 9.0 / 16.0,
        .base_freq_hz = 2.5,
    };
}

void spanda_hkb_frequency_band(double* lo_hz, double* hi_hz) {
    if (lo_hz) *lo_hz = 1.5;
    if (hi_hz) *hi_hz = 4.0;
}

const char* spanda_hkb_frequency_citation(void) {
    return "Conserved-delta band ~1.5-4.0 Hz, centre ~2.5 Hz: Buzsaki, "
           "Logothetis & Singer 2013 + Mizuseki 2014 (oscillation-frequency "
           "hierarchy preserved across a 17,000-fold brain-volume range, "
           "non-allometric); PLOS Biology 2026 (98-species acoustic rhythm "
           "converges on 2.7 Hz, non-allometric). A clean 2.0 Hz is "
           "human-only spontaneous motor tempo - cite as such if used.";
}

double spanda_hkb_drift(double phi, const Spanda_HKB_Params* p) {
    return p->delta_omega - p->a * sin(phi) - 2.0 * p->b * sin(2.0 * phi);
}

double spanda_hkb_potential(double phi, const Spanda_HKB_Params* p) {
    return -p->a * cos(phi) - p->b * cos(2.0 * phi);
}

double spanda_hkb_curvature(double phi, const Spanda_HKB_Params* p) {
    /* V″(φ) = a·cos φ + 4b·cos 2φ — V″(0) = a+4b (always > 0 for positive
     * coupling: identity unconditional); V″(π) = 4b−a (> 0 ⇔ b/a > 1/4:
     * difference conditional). */
    return p->a * cos(phi) + 4.0 * p->b * cos(2.0 * phi);
}

/* Wrap an angle to (−π, π]. */
static double spanda_wrap_phase(double phi) {
    double wrapped = fmod(phi + M_PI, 2.0 * M_PI);
    if (wrapped <= 0.0) wrapped += 2.0 * M_PI;
    return wrapped - M_PI;
}

double spanda_hkb_settle(double phi0, double dt, uint32_t steps,
                         const Spanda_HKB_Params* p) {
    double phi = phi0;
    for (uint32_t i = 0u; i < steps; i++) {
        phi += dt * spanda_hkb_drift(phi, p);
    }
    return spanda_wrap_phase(phi);
}

double spanda_pole_wave(double x, double t, uint8_t pole, bool pole_swapped) {
    if (pole == 0u) {
        return cos(x - t);                       /* bimba (0/1), rightward */
    }
    double wave = cos(x + t);                    /* pratibimba (1/0), leftward */
    /* The half-turn on the field: π polarity flip of the reflection pole
     * (6 ticks × 30°/tick = 180°). */
    return pole_swapped ? -wave : wave;
}

double spanda_superposition(double x, double t, bool pole_swapped) {
    return spanda_pole_wave(x, t, 0u, pole_swapped)
         + spanda_pole_wave(x, t, 1u, pole_swapped);
}

double spanda_standing_envelope(double x, bool pole_swapped) {
    /* cos(x−t) + cos(x+t) = 2·cos x·cos t → envelope 2|cos x|;
     * cos(x−t) − cos(x+t) = 2·sin x·sin t → envelope 2|sin x|.
     * The swap exchanges node and antinode — the ≠ made audible only
     * in co-presence. */
    return 2.0 * (pole_swapped ? fabs(sin(x)) : fabs(cos(x)));
}

double spanda_pole_rms(uint8_t pole, bool pole_swapped) {
    /* RMS over one full period at x = 0, computed (not asserted): the
     * polarity flip of a lone wave leaves every solo observable intact. */
    const uint32_t samples = 4096u;
    double sum_sq = 0.0;
    for (uint32_t i = 0u; i < samples; i++) {
        double t = (2.0 * M_PI * (double)i) / (double)samples;
        double v = spanda_pole_wave(0.0, t, pole, pole_swapped);
        sum_sq += v * v;
    }
    return sqrt(sum_sq / (double)samples);
}

uint8_t spanda_half_turn_index(uint8_t n) {
    return (uint8_t)((n + RING_HALF) % RING_SIZE);
}

uint8_t spanda_intrinsic_twelvefold(void) {
    /* Generation, cross-checked — the twelvefold is the terminal of the
     * flowering's own +2 progression from the 4-fold static seed, never an
     * independent 12 literal. Sub-stage 5 is the meta/percentile return
     * (fold 0) and is not part of the climb. */
    uint8_t fold = SPANDA_CF_FOLD_COUNT[0];              /* the 4-fold seed */
    if (fold != 4u) return 0u;
    for (uint8_t substage = 1u; substage <= 4u; substage++) {
        fold = (uint8_t)(fold + 2u);                     /* contextual flowering: the dyad enters */
        if (fold != SPANDA_CF_FOLD_COUNT[substage]) return 0u;
        if (fold != SPANDA_CF_SUBSTAGE_LUT[substage].fold_count) return 0u;
    }
    return fold;                                         /* 12 — flowered, not sampled */
}

uint8_t spanda_ql_positions_derived(void) {
    /* QL derives FROM the twelvefold: the double cover halves to 6. */
    return (uint8_t)(spanda_intrinsic_twelvefold() / 2u);
}

uint8_t spanda_tick12_readout(double cycle_phase) {
    uint8_t twelvefold = spanda_intrinsic_twelvefold();
    if (twelvefold == 0u) return 0u;
    double norm = fmod(cycle_phase, 2.0 * M_PI);
    if (norm < 0.0) norm += 2.0 * M_PI;
    uint8_t tick = (uint8_t)(norm / (2.0 * M_PI) * (double)twelvefold);
    return (uint8_t)(tick % twelvefold);
}

uint8_t spanda_codon_advance(Quaternion rot, uint64_t cycle) {
    /* (1) Rotational state: atan2 over (x, w) reads the SU(2) half-angle
     * WITH its sign — antipodal states q and −q (the same SO(3) face,
     * reflection-related arcs n and 11−n) resolve to arcs 6 apart. A bare
     * tick12 integer carries no sign and cannot make this distinction. */
    double half_angle = atan2((double)rot.x, (double)rot.w);
    if (half_angle < 0.0) half_angle += 2.0 * M_PI;
    uint8_t arc = (uint8_t)(half_angle / (M_PI / 6.0) + 0.5) % RING_SIZE;
    uint8_t helix    = (uint8_t)(arc >= RING_HALF ? 1u : 0u);
    uint8_t position = (uint8_t)(arc % RING_HALF);
    /* (2) Clock: the cycle selects the lens class of the 72-space. */
    uint8_t lens = (uint8_t)(cycle % (uint64_t)MEF_BASE_LENSES);
    uint8_t addr72 = kernel_resonance_index(lens, helix, position);
    if (addr72 >= PARASHAKTI_TOTAL) return 0u;
    /* (3) Epogdoon: the canonical 9:8 transduction 72 → 64 (M2 → M3). */
    return apply_epogdoon_compression(addr72);
}

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
    /* Ananda family signature: implicate boundaries at Bimba(0)/Quintessence(5);
     * +2 net Cl(4,2) signature across the family axis (cf. perspex tint §5.4). */
    if (ANANDA_FAMILY_SIGNATURE[0] != -1 || ANANDA_FAMILY_SIGNATURE[5] != -1) return false;
    int8_t family_sig_net = 0;
    for (int i = 0; i < 6; i++) family_sig_net = (int8_t)(family_sig_net + ANANDA_FAMILY_SIGNATURE[i]);
    if (family_sig_net != 2) return false;
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
 * ANANDA #1-2 MATRICES — canonical 12×12 Vortex Modulae, dual-faced
 *
 * RAW face ("No digi-rooting" block of the canonical CSV): closed-form
 * affine values, un-modded. DR face ("Digi-rooting" block): routes to
 * the canonical nibble-packed .rodata matrices (ANANDA_BIMBA /
 * ANANDA_PRATIBIMBA / ANANDA_SUM) — one source, never a parallel table.
 * The former 10×10 %10 "operational core" is retired (Tranche 10.10).
 * ------------------------------------------------------------------ */

/* RAW face. Scalar non-negative families:
 *   0 Bimba       (rX+0)         : r*c            (max 121)
 *   1 Pratibimba  (rX+1)         : r*c + 1        (max 122)
 *   2 Sum         (rX+0)+(rX+1)  : 2*r*c + 1      (max 243)
 *   4 DiffB       (rX+1)-(rX+0)  : +1 constant
 * Non-scalar-representable families return 0 here:
 *   3 DiffA       (rX+0)-(rX+1)  : constant -1 (signed; DR face = 9)
 *   5 NonDual rule (rX+0/1)+/-() : "-1/0/1" tuple — see get_quint_* */
uint8_t m1_ananda_get(uint8_t matrix_idx, uint8_t row, uint8_t col) {
    if (matrix_idx >= 6 || row >= 12 || col >= 12) return 0;
    uint16_t rc = (uint16_t)row * (uint16_t)col;
    switch (matrix_idx) {
        case 0:  return (uint8_t)rc;
        case 1:  return (uint8_t)(rc + 1u);
        case 2:  return (uint8_t)(2u * rc + 1u);
        case 4:  return 1u;
        default: return 0u;   /* 3: signed -1; 5: rule tuple */
    }
}

/* DR face — the digi-root mirrors, read from the canonical .rodata
 * matrices (FR 2.1.1 / FR 2.1.9), never recomputed from the raw face. */
uint8_t m1_ananda_dr_get(uint8_t matrix_idx, uint8_t row, uint8_t col) {
    if (matrix_idx >= 6 || row >= 12 || col >= 12) return 0;
    switch (matrix_idx) {
        case 0:  return get_ananda_harmonic(&ANANDA_BIMBA, row, col);
        case 1:  return get_ananda_harmonic(&ANANDA_PRATIBIMBA, row, col);
        case 2:  return get_ananda_harmonic(&ANANDA_SUM, row, col);
        case 3:  return 9u;   /* DR of constant -1 (FR 2.1.9) */
        case 4:  return 1u;
        default: return 0u;   /* 5: dyadic — get_quint_bimba/get_quint_sum */
    }
}

/* Ananda Axiom on the RAW face: Pratibimba - Bimba == +1 exactly,
 * across the full canonical 12×12 grid (no modular wrap). */
int m1_ananda_verify_axiom(void) {
    for (uint8_t i = 0; i < 12; i++)
        for (uint8_t j = 0; j < 12; j++)
            if ((int)m1_ananda_get(1, i, j) - (int)m1_ananda_get(0, i, j) != 1)
                return 0;
    return 1;
}

/* ===================================================================
 * FR 2.1.10 — ANANDA SEAT SEMANTICS + DUAL-BASE ACCOUNTING
 * The M0-3 number-dozen binding: matrix positions 0-9 are the
 * archetypal numbers; 10 = (0/1) Non-Dual Binary; 11 = (-) Mirror.
 * Coordinate/symbol/polarity delegate to the compiled m0 authority.
 * =================================================================== */

const uint8_t ANANDA_SEAT_TO_ARCHETYPE_IDX[12] =
    { 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 0 };

const uint8_t ANANDA_MASCULINE_OCTET[8]  = { 0, 1, 2, 4, 6, 8, 10, 11 };
const uint8_t ANANDA_FEMININE_QUARTET[4] = { 3, 5, 7, 9 };
_Static_assert(sizeof(ANANDA_MASCULINE_OCTET) + sizeof(ANANDA_FEMININE_QUARTET) == 12,
               "masculine octet + feminine quartet must partition the dozen");

Ananda_Seat_Kind m1_ananda_seat_kind(uint8_t position12) {
    if (position12 == 10u) return ANANDA_SEAT_NONDUAL_BINARY;
    if (position12 == 11u) return ANANDA_SEAT_MIRROR;
    return ANANDA_SEAT_NUMBER;
}

const char* m1_ananda_seat_coordinate(uint8_t position12) {
    if (position12 >= 12u) return NULL;
    return ARCHETYPE_COORDINATE_LUT[ANANDA_SEAT_TO_ARCHETYPE_IDX[position12]];
}

const char* m1_ananda_seat_symbol(uint8_t position12) {
    if (position12 >= 12u) return NULL;
    return ARCHETYPE_LUT[ANANDA_SEAT_TO_ARCHETYPE_IDX[position12]].symbol;
}

/* EVE polarity (odd archetypal numbers 3,5,7,9) -> feminine quartet;
 * ADAM + NEUTRAL (evens + the four-fold zero) -> masculine octet.
 * Single source: the m0 ARCHETYPE_LUT polarity field. */
Ananda_Bus_Role m1_ananda_seat_bus_role(uint8_t position12) {
    if (position12 >= 12u) return ANANDA_BUS_OCTET;
    const uint8_t polarity =
        ARCHETYPE_LUT[ANANDA_SEAT_TO_ARCHETYPE_IDX[position12]].polarity;
    return (polarity == 1u) ? ANANDA_BUS_QUARTET : ANANDA_BUS_OCTET;
}

int16_t m1_ananda_row_sum(uint8_t matrix_idx, uint8_t row, uint8_t frame12) {
    if (matrix_idx >= 5u || row >= 12u) return 0;
    const uint8_t limit = frame12 ? 12u : 10u;
    int16_t acc = 0;
    for (uint8_t p = 0; p < limit; p++) {
        const int16_t kp = (int16_t)((int16_t)row * (int16_t)p);
        switch (matrix_idx) {
            case 0: acc = (int16_t)(acc + kp);                      break;
            case 1: acc = (int16_t)(acc + kp + 1);                  break;
            case 2: acc = (int16_t)(acc + (int16_t)(2 * kp) + 1);   break;
            case 3: acc = (int16_t)(acc - 1);                       break;
            case 4: acc = (int16_t)(acc + 1);                       break;
            default: break;
        }
    }
    return acc;
}

int m1_ananda_verify_dual_base(void) {
    int32_t bimba_total = 0, pratibimba_total = 0;
    for (uint8_t k = 0; k < 12u; k++) {
        const int16_t bc = m1_ananda_row_sum(0, k, 0);
        const int16_t bf = m1_ananda_row_sum(0, k, 1);
        if (bc != (int16_t)(ANANDA_CORE_SUM_UNSHIFTED * k)) return 0;
        if (bf != (int16_t)(ANANDA_FRAME_SUM_UNSHIFTED * k)) return 0;
        if (m1_ananda_row_sum(1, k, 0) != (int16_t)(bc + 10)) return 0;
        if (m1_ananda_row_sum(1, k, 1) != (int16_t)(bf + 12)) return 0;
        if (m1_ananda_row_sum(3, k, 0) != -10 || m1_ananda_row_sum(3, k, 1) != -12) return 0;
        if (m1_ananda_row_sum(4, k, 0) !=  10 || m1_ananda_row_sum(4, k, 1) !=  12) return 0;
        bimba_total      += bf;
        pratibimba_total += m1_ananda_row_sum(1, k, 1);
    }
    if (bimba_total != 66 * 66) return 0;            /* 4356 = (Σk)(Σp)    */
    if (pratibimba_total != 66 * 66 + 144) return 0; /* +1 per frame cell  */
    if (m1_ananda_row_sum(1, 1, 0) != ANANDA_CORE_SUM_SHIFTED) return 0;
    if (m1_ananda_row_sum(1, 1, 1) != ANANDA_FRAME_SUM_SHIFTED) return 0;
    return 1;
}

static uint8_t ananda_digit_root_u16(uint16_t v) {
    if (v == 0u) return 0u;
    const uint16_t m = v % 9u;
    return (uint8_t)(m == 0u ? 9u : m);
}

int m1_ananda_rule_face(uint8_t row, uint8_t col, int dr_face,
                        char* buf, uint32_t buflen) {
    if (row >= 12u || col >= 12u || buf == NULL || buflen == 0u) return -1;
    const uint16_t kp = (uint16_t)row * (uint16_t)col;
    if (dr_face) {
        if (kp == 0u) return snprintf(buf, (size_t)buflen, "0/1");
        return snprintf(buf, (size_t)buflen, "1/%u",
                        (unsigned)ananda_digit_root_u16((uint16_t)(2u * kp + 1u)));
    }
    if (kp == 0u) return snprintf(buf, (size_t)buflen, "-1/0/1");
    return snprintf(buf, (size_t)buflen, "-1/1/%u", (unsigned)(2u * kp + 1u));
}

/* ---- Annex accounting (CSV rows 52-71, "8_9fold" grids) ---- */

int32_t m1_ananda_grand_total(uint8_t matrix_idx, uint8_t frame12,
                              uint8_t exclude_seed_row) {
    if (matrix_idx >= 5u) return 0;
    /* The core total is the true 10×10 SQUARE (rows 0-9 × cols 0-9 —
     * Bimba 45² = 2025); the frame total is the full 12×12 (66² = 4356). */
    const uint8_t row_limit = frame12 ? 12u : 10u;
    int32_t total = 0;
    for (uint8_t k = exclude_seed_row ? 1u : 0u; k < row_limit; k++)
        total += (int32_t)m1_ananda_row_sum(matrix_idx, k, frame12);
    return total;
}

int m1_ananda_cumulative_dr_trace(uint8_t matrix_idx, uint8_t reverse,
                                  uint8_t trace[12]) {
    if (matrix_idx >= 5u || trace == NULL) return 0;
    const int32_t grand = m1_ananda_grand_total(matrix_idx, 1u, 0u);
    int32_t running = 0;
    for (uint8_t k = 0; k < 12u; k++) {
        const int32_t before = running;
        running += (int32_t)m1_ananda_row_sum(matrix_idx, k, 1u);
        /* forward: DR of the partial through row k;
         * reverse: DR of the remaining total from row k inclusive. */
        const int32_t partial = reverse ? (grand - before) : running;
        const uint32_t mag = (uint32_t)(partial < 0 ? -partial : partial);
        trace[k] = (mag == 0u) ? 0u
                                : (uint8_t)((mag % 9u == 0u) ? 9u : (mag % 9u));
    }
    return 1;
}

int m1_ananda_mirror_pair(uint8_t n_3_to_10, uint8_t* dr_n,
                          uint8_t* complement11) {
    if (n_3_to_10 < 3u || n_3_to_10 > 10u || dr_n == NULL || complement11 == NULL)
        return 0;
    *dr_n = ananda_digit_root_u16(n_3_to_10);
    *complement11 = (uint8_t)(11u - n_3_to_10);
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
