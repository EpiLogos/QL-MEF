#ifndef QL_MEF_PRIMITIVE_H
#define QL_MEF_PRIMITIVE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define QL_C_API_VERSION_MAJOR 0u
#define QL_C_API_VERSION_MINOR 1u
#define QL_C_API_VERSION_PATCH 0u

#define QL_POSITION_COUNT 6u
#define QL_FACE_COUNT 2u
#define QL_TICK_COUNT 12u
#define QL_RELATION_COUNT 36u
#define QL_STATE6_COUNT 64u
#define QL_RESONANCE_COUNT 72u
#define QL_TRITONE_SQUARE_COUNT 3u
#define QL_INVALID_U8 0xFFu

typedef enum {
    QL_RING_DIRECT_HALF = 0,
    QL_RING_INVERTED_HALF = 1
} QL_Ring_Half;

typedef struct {
    uint64_t cycle;
    uint8_t sub_tick;
    QL_Ring_Half half;
    /* Historical Epi exposes two non-equivalent 0..5 projections on the
     * inverted half. Keep both explicit rather than silently choosing one. */
    uint8_t base_position;
    uint8_t traversal_stage;
    float harmonic_ratio;
} QL_Primitive_Tick;

const char* ql_c_api_version(void);
const char* ql_c_reference_provenance(void);

uint8_t ql_position_invert(uint8_t position);
uint8_t ql_ring_wrap(uint8_t tick);
QL_Ring_Half ql_ring_half(uint8_t tick);
uint8_t ql_ring_base_position(uint8_t tick);
uint8_t ql_ring_traversal_stage(uint8_t tick);

uint8_t ql_relation_index(uint8_t left_position, uint8_t right_position);
uint8_t ql_state6_complement(uint8_t state);
uint8_t ql_state6_line_change(uint8_t state, uint8_t line);

uint8_t ql_resonance_index(uint8_t lens, uint8_t face, uint8_t position);
uint8_t ql_tritone_square_for_lens(uint8_t lens);

float ql_epogdoon_ratio(void);
float ql_ratio_ascending_fourth(void);
float ql_ratio_descending_fourth(void);
float ql_ratio_descending_fifth(void);
float ql_ratio_ascending_fifth(void);
QL_Primitive_Tick ql_tick_from_epogdoon(uint64_t cycle, uint8_t sub_tick);

#ifdef __cplusplus
}
#endif

#endif /* QL_MEF_PRIMITIVE_H */
