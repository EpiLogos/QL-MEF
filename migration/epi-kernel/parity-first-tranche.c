#include "kernel.h"
#include "m3.h"
#include "ql/primitive.h"

#include <math.h>
#include <stdint.h>
#include <stdio.h>

static unsigned failures = 0u;

#define CHECK(cond, op, detail) \
    do { \
        if (!(cond)) { \
            fprintf(stderr, "FAIL\t%s\t%s\n", (op), (detail)); \
            failures++; \
        } \
    } while (0)

static int exactf(float a, float b) {
    return a == b;
}

int main(void) {
    /* Cardinality/provenance anchors from the historical headers. */
    CHECK(QL_POSITION_COUNT == QL_POSITIONS, "cardinality.positions", "6");
    CHECK(QL_TICK_COUNT == RING_SIZE, "cardinality.tick12", "12");
    CHECK(QL_RELATION_COUNT == M2_36_BASE, "cardinality.relations", "36");
    CHECK(QL_STATE6_COUNT == M3_WORD, "cardinality.state6", "64");
    CHECK(QL_RESONANCE_COUNT == KERNEL_RESONANCE_DIM, "cardinality.resonance", "72");

    for (uint8_t p = 0u; p < QL_POSITION_COUNT; p++) {
        CHECK(ql_position_invert(p) == QL_INVERT[p], "position.invert", "reference QL_INVERT");
        CHECK(ql_position_invert(ql_position_invert(p)) == p,
              "position.invert.involution", "invert(invert(p)) == p");
    }
    CHECK(ql_position_invert(6u) == QL_INVALID_U8, "position.invert.invalid", "6 -> invalid");

    for (unsigned raw = 0u; raw <= 255u; raw++) {
        uint8_t tick = (uint8_t)raw;
        CHECK(ql_ring_wrap(tick) == RING_WRAP(tick), "ring.wrap", "uint8 exhaustive");
        CHECK((ql_ring_half(tick) == QL_RING_INVERTED_HALF) == IS_SHADOW_PHASE(RING_WRAP(tick)),
              "ring.half", "uint8 exhaustive");
        CHECK(ql_ring_base_position(tick) == GET_BASE_QL_POS(RING_WRAP(tick)),
              "ring.base_position", "uint8 exhaustive");
        CHECK(ql_ring_traversal_stage(tick) == ql_get_stage(RING_WRAP(tick)),
              "ring.traversal_stage", "uint8 exhaustive");
    }

    /* 6 x 6 relation sheet: native generalized addressing. The historical
     * source supplies the 36 cardinality but no single callable generic index. */
    uint8_t seen[QL_RELATION_COUNT] = {0};
    for (uint8_t left = 0u; left < QL_POSITION_COUNT; left++) {
        for (uint8_t right = 0u; right < QL_POSITION_COUNT; right++) {
            uint8_t idx = ql_relation_index(left, right);
            CHECK(idx < QL_RELATION_COUNT, "relation.index.range", "6x6 exhaustive");
            if (idx < QL_RELATION_COUNT) seen[idx]++;
        }
    }
    for (uint8_t idx = 0u; idx < QL_RELATION_COUNT; idx++) {
        CHECK(seen[idx] == 1u, "relation.index.bijection", "all 36 exactly once");
    }
    CHECK(ql_relation_index(6u, 0u) == QL_INVALID_U8, "relation.index.invalid", "left boundary");
    CHECK(ql_relation_index(0u, 6u) == QL_INVALID_U8, "relation.index.invalid", "right boundary");

    for (uint8_t state = 0u; state < QL_STATE6_COUNT; state++) {
        CHECK(ql_state6_complement(state) == m3_complement(state),
              "state6.complement", "64-state exhaustive");
        CHECK(ql_state6_complement(ql_state6_complement(state)) == state,
              "state6.complement.involution", "64-state exhaustive");
        for (uint8_t line = 0u; line < QL_POSITION_COUNT; line++) {
            CHECK(ql_state6_line_change(state, line) == m3_line_change(state, line),
                  "state6.line_change", "64x6 exhaustive");
            CHECK(ql_state6_line_change(ql_state6_line_change(state, line), line) == state,
                  "state6.line_change.involution", "64x6 exhaustive");
        }
    }
    CHECK(ql_state6_line_change(0u, 6u) == QL_INVALID_U8,
          "state6.line_change.invalid", "line boundary");

    for (uint8_t lens = 0u; lens < QL_POSITION_COUNT; lens++) {
        CHECK(ql_tritone_square_for_lens(lens) == kernel_tritone_square_for_lens(lens),
              "resonance.tritone_square", "6-lens exhaustive");
        for (uint8_t face = 0u; face < QL_FACE_COUNT; face++) {
            for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
                CHECK(ql_resonance_index(lens, face, position) ==
                      kernel_resonance_index(lens, face, position),
                      "resonance.index", "6x2x6 exhaustive");
            }
        }
    }
    CHECK(ql_tritone_square_for_lens(6u) == kernel_tritone_square_for_lens(6u),
          "resonance.tritone_square.invalid", "lens boundary");
    CHECK(ql_resonance_index(6u, 0u, 0u) == kernel_resonance_index(6u, 0u, 0u),
          "resonance.index.invalid", "lens boundary");
    CHECK(ql_resonance_index(0u, 2u, 0u) == kernel_resonance_index(0u, 2u, 0u),
          "resonance.index.invalid", "face boundary");
    CHECK(ql_resonance_index(0u, 0u, 6u) == kernel_resonance_index(0u, 0u, 6u),
          "resonance.index.invalid", "position boundary");

    CHECK(exactf(ql_epogdoon_ratio(), kernel_epogdoon_ratio()), "ratio.epogdoon", "exact float");
    CHECK(exactf(ql_ratio_ascending_fourth(), kernel_ratio_ascending_fourth()),
          "ratio.ascending_fourth", "exact float");
    CHECK(exactf(ql_ratio_descending_fourth(), kernel_ratio_descending_fourth()),
          "ratio.descending_fourth", "exact float");
    CHECK(exactf(ql_ratio_descending_fifth(), kernel_ratio_descending_fifth()),
          "ratio.descending_fifth", "exact float");
    CHECK(exactf(ql_ratio_ascending_fifth(), kernel_ratio_ascending_fifth()),
          "ratio.ascending_fifth", "exact float");

    for (unsigned raw = 0u; raw <= 255u; raw++) {
        uint8_t sub_tick = (uint8_t)raw;
        Kernel_Tick reference = kernel_tick_from_epogdoon(17u, sub_tick);
        QL_Tick native = ql_tick_from_epogdoon(17u, sub_tick);
        CHECK(native.cycle == reference.cycle, "tick.cycle", "uint8 exhaustive sub_tick");
        CHECK(native.sub_tick == reference.sub_tick, "tick.wrap", "uint8 exhaustive sub_tick");
        CHECK(native.base_position == reference.position6,
              "tick.kernel_position_projection", "reference position6 parity");
        CHECK(exactf(native.harmonic_ratio, reference.harmonic_ratio),
              "tick.harmonic_ratio", "exact float; uint8 exhaustive sub_tick");
        CHECK((native.half == QL_RING_DIRECT_HALF) == (reference.phase == KERNEL_PHASE_DESCENT),
              "tick.historical_kernel_phase", "labels preserved only as reference mapping");
    }

    /* Returned reality: these projections are intentionally not asserted equal.
     * Historical M1 says tick 6 -> stage 5; historical kernel says tick 6 ->
     * position6 0. Native API preserves both fields so migration does not
     * silently choose a semantic correction. */
    CHECK(ql_ring_traversal_stage(6u) == 5u, "discrepancy.m1_stage", "tick6 -> 5");
    CHECK(ql_ring_base_position(6u) == 0u, "discrepancy.kernel_projection", "tick6 -> 0");

    if (failures != 0u) {
        fprintf(stderr, "parity failures: %u\n", failures);
        return 1;
    }

    printf("first-tranche parity: PASS\n");
    printf("reference: EpiLogos/Epi-Logos-C-Experiments@daa660cbc1b8c5da83828698665a753852cb0287\n");
    printf("native-api: %s\n", ql_c_api_version());
    return 0;
}
