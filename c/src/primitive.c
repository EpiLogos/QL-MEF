#include "ql/primitive.h"

static const char QL_C_VERSION[] = "0.1.0";
static const char QL_C_PROVENANCE[] =
    "EpiLogos/Epi-Logos-C-Experiments:Body/S/S0/epi-lib@"
    "daa660cbc1b8c5da83828698665a753852cb0287; QL-MEF#54 native C tranche";

const char* ql_c_api_version(void) {
    return QL_C_VERSION;
}

const char* ql_c_reference_provenance(void) {
    return QL_C_PROVENANCE;
}

uint8_t ql_position_invert(uint8_t position) {
    return position < QL_POSITION_COUNT
        ? (uint8_t)((QL_POSITION_COUNT - 1u) - position)
        : QL_INVALID_U8;
}

uint8_t ql_ring_wrap(uint8_t tick) {
    return (uint8_t)(tick % QL_TICK_COUNT);
}

QL_Ring_Half ql_ring_half(uint8_t tick) {
    return ql_ring_wrap(tick) < QL_POSITION_COUNT
        ? QL_RING_DIRECT_HALF
        : QL_RING_INVERTED_HALF;
}

uint8_t ql_ring_base_position(uint8_t tick) {
    return (uint8_t)(ql_ring_wrap(tick) % QL_POSITION_COUNT);
}

uint8_t ql_ring_traversal_stage(uint8_t tick) {
    uint8_t wrapped = ql_ring_wrap(tick);
    return wrapped < QL_POSITION_COUNT
        ? wrapped
        : (uint8_t)((QL_TICK_COUNT - 1u) - wrapped);
}

uint8_t ql_relation_index(uint8_t left_position, uint8_t right_position) {
    if (left_position >= QL_POSITION_COUNT || right_position >= QL_POSITION_COUNT) {
        return QL_INVALID_U8;
    }
    return (uint8_t)(left_position * QL_POSITION_COUNT + right_position);
}

uint8_t ql_state6_complement(uint8_t state) {
    return (uint8_t)((state & 0x3Fu) ^ 0x3Fu);
}

uint8_t ql_state6_line_change(uint8_t state, uint8_t line) {
    if (line >= QL_POSITION_COUNT) {
        return QL_INVALID_U8;
    }
    return (uint8_t)((state & 0x3Fu) ^ (uint8_t)(1u << line));
}

uint8_t ql_resonance_index(uint8_t lens, uint8_t face, uint8_t position) {
    if (lens >= QL_POSITION_COUNT || face >= QL_FACE_COUNT || position >= QL_POSITION_COUNT) {
        return QL_INVALID_U8;
    }
    return (uint8_t)(lens * QL_TICK_COUNT + face * QL_POSITION_COUNT + position);
}

uint8_t ql_tritone_square_for_lens(uint8_t lens) {
    if (lens >= QL_POSITION_COUNT) {
        return QL_INVALID_U8;
    }
    if (lens == 0u || lens == 5u) return 0u;
    if (lens == 1u || lens == 4u) return 1u;
    return 2u;
}

float ql_epogdoon_ratio(void) {
    return 9.0f / 8.0f;
}

float ql_ratio_ascending_fourth(void) {
    return 4.0f / 3.0f;
}

float ql_ratio_descending_fourth(void) {
    return 3.0f / 4.0f;
}

float ql_ratio_descending_fifth(void) {
    return 2.0f / 3.0f;
}

float ql_ratio_ascending_fifth(void) {
    return 3.0f / 2.0f;
}

QL_Tick ql_tick_from_epogdoon(uint64_t cycle, uint8_t sub_tick) {
    static const float ratios[QL_TICK_COUNT] = {
        1.0f,
        4.0f / 3.0f,
        3.0f / 4.0f,
        9.0f / 8.0f,
        1.0f,
        2.0f / 3.0f,
        2.0f / 3.0f,
        3.0f / 4.0f,
        9.0f / 8.0f,
        3.0f / 2.0f,
        3.0f / 2.0f,
        9.0f / 8.0f
    };
    uint8_t tick = ql_ring_wrap(sub_tick);
    return (QL_Tick){
        .cycle = cycle,
        .sub_tick = tick,
        .half = ql_ring_half(tick),
        .base_position = ql_ring_base_position(tick),
        .traversal_stage = ql_ring_traversal_stage(tick),
        .harmonic_ratio = ratios[tick]
    };
}
