/*
 * m1_ananda_projection.c — canonical raw12 / DR12 / decimal10 Ananda views
 *
 * The affine laws here are not a replacement dataset. They are the exact
 * executable form of the canonical Vortex Modulae source used by M1-2.
 */

#include "m1_ananda_projection.h"
#include <string.h>

uint8_t m1_ananda_true_digit_root(uint16_t value) {
    if (value == 0u) return 0u;
    return (uint8_t)(1u + ((value - 1u) % 9u));
}

uint8_t m1_ananda_decimal_mod10(int16_t value) {
    int16_t rem = (int16_t)(value % 10);
    if (rem < 0) rem = (int16_t)(rem + 10);
    return (uint8_t)rem;
}

int m1_ananda_oscillatory_address_from_clock(
        uint64_t cycle,
        uint8_t tick12,
        M1_Ananda_Oscillatory_Address* out)
{
    uint8_t position6;
    uint8_t fiber;
    uint16_t degree360;
    M1_Ananda_Direct_Prime_Phase phase;

    if (!out || tick12 >= 12u) return 0;

    position6 = (uint8_t)(tick12 % 6u);
    phase = tick12 < 6u ? M1_ANANDA_DIRECT_PHASE : M1_ANANDA_PRIME_PHASE;
    degree360 = (uint16_t)((uint16_t)tick12 * (uint16_t)DEGREE_PER_TICK);
    fiber = (uint8_t)(cycle & 1u);

    out->cycle = cycle;
    out->tick12 = tick12;
    out->degree360 = degree360;
    out->hopf_fiber = fiber;
    out->degree720 = (uint16_t)(degree360 + ((uint16_t)fiber * (uint16_t)FULL_CYCLE_DEG));
    out->position6 = position6;
    out->phase = phase;

    out->conjugate_tick12 = (uint8_t)((tick12 + 6u) % 12u);
    out->conjugate_position6 = position6;
    out->conjugate_phase = phase == M1_ANANDA_DIRECT_PHASE
        ? M1_ANANDA_PRIME_PHASE
        : M1_ANANDA_DIRECT_PHASE;

    /* Ananda family position and Spanda stage are a compile-time 1:1 track. */
    out->spanda_stage = ANANDA_TO_SPANDA_STAGE((Ananda_Matrix_Op)position6);
    return 1;
}

int m1_ananda_oscillatory_address_from_tick12(
        uint8_t tick12,
        M1_Ananda_Oscillatory_Address* out)
{
    return m1_ananda_oscillatory_address_from_clock(0u, tick12, out);
}

static int16_t _m1_ananda_raw(
        Ananda_Matrix_Op family,
        uint8_t row12,
        uint8_t col12)
{
    int16_t kp = (int16_t)((int16_t)row12 * (int16_t)col12);
    switch (family) {
        case MATRIX_BIMBA:        return kp;
        case MATRIX_PRATIBIMBA:   return (int16_t)(kp + 1);
        case MATRIX_SUM:          return (int16_t)((2 * kp) + 1);
        case MATRIX_DIFF_A:       return -1;
        case MATRIX_DIFF_B:       return 1;
        case MATRIX_QUINTESSENCE: return 0; /* tuple-bearing; no scalar authority */
        default:                  return 0;
    }
}

static uint8_t _m1_ananda_source_dr(
        Ananda_Matrix_Op family,
        int16_t raw)
{
    /* The canonical source fixes the signed/complement Difference A residue. */
    if (family == MATRIX_DIFF_A) return 9u;
    if (family == MATRIX_DIFF_B) return 1u;
    if (raw < 0) return 0u; /* no other negative scalar family is canonical */
    return m1_ananda_true_digit_root((uint16_t)raw);
}

static int _m1_ananda_address_valid(const M1_Ananda_Oscillatory_Address* osc) {
    M1_Ananda_Oscillatory_Address expected;

    if (!osc) return 0;
    if (!m1_ananda_oscillatory_address_from_clock(
            osc->cycle, osc->tick12, &expected)) return 0;

    return osc->cycle == expected.cycle
        && osc->degree360 == expected.degree360
        && osc->hopf_fiber == expected.hopf_fiber
        && osc->degree720 == expected.degree720
        && osc->position6 == expected.position6
        && osc->phase == expected.phase
        && osc->conjugate_tick12 == expected.conjugate_tick12
        && osc->conjugate_position6 == expected.conjugate_position6
        && osc->conjugate_phase == expected.conjugate_phase
        && osc->spanda_stage == expected.spanda_stage;
}

static void _m1_ananda_fill_quintessence(
        uint8_t row12,
        uint8_t col12,
        M1_Ananda_Quintessence_Projection* q)
{
    int16_t bimba = _m1_ananda_raw(MATRIX_BIMBA, row12, col12);
    int16_t pratibimba = _m1_ananda_raw(MATRIX_PRATIBIMBA, row12, col12);
    int16_t sum = _m1_ananda_raw(MATRIX_SUM, row12, col12);

    q->bimba_raw = bimba;
    q->pratibimba_raw = pratibimba;
    q->sum_raw = sum;
    q->difference_a_raw = -1;
    q->difference_b_raw = 1;

    q->bimba_dr = _m1_ananda_source_dr(MATRIX_BIMBA, bimba);
    q->pratibimba_dr = _m1_ananda_source_dr(MATRIX_PRATIBIMBA, pratibimba);
    q->sum_dr = _m1_ananda_source_dr(MATRIX_SUM, sum);
    q->difference_a_dr = 9u;
    q->difference_b_dr = 1u;

    q->decimal10_valid = (row12 < 10u && col12 < 10u);
    if (q->decimal10_valid) {
        q->bimba_decimal10 = m1_ananda_decimal_mod10(bimba);
        q->pratibimba_decimal10 = m1_ananda_decimal_mod10(pratibimba);
        q->sum_decimal10 = m1_ananda_decimal_mod10(sum);
        q->difference_a_decimal10 = 9u;
        q->difference_b_decimal10 = 1u;
    }
}

int m1_ananda_project_cell(
        Ananda_Matrix_Op family,
        uint8_t row12,
        uint8_t col12,
        const M1_Ananda_Oscillatory_Address* oscillatory,
        M1_Ananda_Cell_Projection* out)
{
    int16_t raw;

    if (!out) return 0;
    if ((uint8_t)family > (uint8_t)MATRIX_QUINTESSENCE) return 0;
    if (row12 >= 12u || col12 >= 12u) return 0;
    if (!_m1_ananda_address_valid(oscillatory)) return 0;

    memset(out, 0, sizeof(*out));
    out->family = family;
    out->row12 = row12;
    out->col12 = col12;
    out->oscillatory = *oscillatory;
    out->source_ref = M1_ANANDA_SOURCE_REF;
    out->derivation_ref = M1_ANANDA_DERIVATION_REF;
    out->phase_ref = M1_ANANDA_PHASE_REF;

    if (family == MATRIX_QUINTESSENCE) {
        out->scalar_valid = false;
        out->decimal10_valid = false;
        _m1_ananda_fill_quintessence(row12, col12, &out->quintessence);
        return 1;
    }

    raw = _m1_ananda_raw(family, row12, col12);
    out->scalar_valid = true;
    out->raw12_value = raw;
    out->digit_root12_value = _m1_ananda_source_dr(family, raw);
    out->decimal10_valid = (row12 < 10u && col12 < 10u);
    if (out->decimal10_valid)
        out->decimal10_value = m1_ananda_decimal_mod10(raw);

    return 1;
}

static uint8_t _m1_ananda_stored_dr(
        Ananda_Matrix_Op family,
        uint8_t row12,
        uint8_t col12)
{
    switch (family) {
        case MATRIX_BIMBA:
            return get_ananda_harmonic(&ANANDA_BIMBA, row12, col12);
        case MATRIX_PRATIBIMBA:
            return get_ananda_harmonic(&ANANDA_PRATIBIMBA, row12, col12);
        case MATRIX_SUM:
            return get_ananda_harmonic(&ANANDA_SUM, row12, col12);
        case MATRIX_DIFF_A:
            return 9u;
        case MATRIX_DIFF_B:
            return 1u;
        default:
            return 0u;
    }
}

int m1_ananda_verify_dr12_luts(void) {
    for (uint8_t row = 0u; row < 12u; ++row) {
        for (uint8_t col = 0u; col < 12u; ++col) {
            for (uint8_t f = (uint8_t)MATRIX_BIMBA;
                 f <= (uint8_t)MATRIX_DIFF_B;
                 ++f)
            {
                Ananda_Matrix_Op family = (Ananda_Matrix_Op)f;
                int16_t raw = _m1_ananda_raw(family, row, col);
                uint8_t expected = _m1_ananda_source_dr(family, raw);
                if (_m1_ananda_stored_dr(family, row, col) != expected)
                    return 0;
            }

            /* Quintessence source tuple {-1, bimba, sum} must match storage. */
            {
                int16_t bimba = _m1_ananda_raw(MATRIX_BIMBA, row, col);
                int16_t sum = _m1_ananda_raw(MATRIX_SUM, row, col);
                if (get_quint_diff(row, col) != -1) return 0;
                if (get_quint_bimba(row, col) !=
                    _m1_ananda_source_dr(MATRIX_BIMBA, bimba)) return 0;
                if (get_quint_sum(row, col) !=
                    _m1_ananda_source_dr(MATRIX_SUM, sum)) return 0;
            }
        }
    }
    return 1;
}
