/* K5 native seating of the retained M1 body, not another structural registry.
 * Arithmetic/clock return: Epi-Logos-C-Experiments@QL_M1_RETURN_REVISION,
 * Body/S/S0/epi-lib/src/m1_ananda_projection.c. Formal/Spanda/topology:
 * vendor/epi-kernel/reference/include/m1.h and src/m1.c. No legacy source edits.
 */
#include "ql/m1.h"
#include <math.h>
#include <stdio.h>

const char *ql_m1_engine_version(void) { return QL_M1_ENGINE_VERSION; }
const QL_M_Node *ql_m1_node(const char *coordinate) {
    const QL_M_Node *node = coordinate ? ql_m_resolve(coordinate) : NULL;
    return node && node->root_position == 1u ? node : NULL;
}
static QL_M_NodeId node_id(const char *coordinate) {
    const QL_M_Node *node = ql_m1_node(coordinate);
    return node ? node->id : QL_M_NO_ID;
}
static uint32_t dr(uint32_t n) { return n ? 1u + (n - 1u) % 9u : 0u; }
static uint32_t decimal(int32_t n) { return (uint32_t)((n % 10 + 10) % 10); }
int ql_m1_clock(uint64_t cycle, uint32_t tick, QL_M1_Clock *out) {
    QL_M1_Clock value;
    if (!out || tick >= 12u) return 0;
    value.cycle = cycle;
    value.tick12 = tick;
    value.position6 = tick % 6u;
    value.phase = tick / 6u;
    value.conjugate_tick12 = (tick + 6u) % 12u;
    value.conjugate_phase = 1u - value.phase;
    value.degree360 = tick * 30u;
    value.hopf_fiber = (uint32_t)(cycle & UINT64_C(1));
    value.degree720 = value.degree360 + value.hopf_fiber * 360u;
    value.spanda_stage = value.position6;
    *out = value;
    return 1;
}
int ql_m1_cell(uint32_t family, uint32_t row, uint32_t col,
               uint64_t cycle, uint32_t tick, QL_M1_Cell *out) {
    QL_M1_Cell value = {0};
    char ref[32];
    size_t i;
    if (!out || family >= 6u || row >= 12u || col >= 12u ||
        !ql_m1_clock(cycle, tick, &value.clock)) return 0;
    (void)snprintf(ref, sizeof(ref), "#1-2-%u", (unsigned)family);
    value.coordinate = node_id(ref);
    (void)snprintf(ref, sizeof(ref), "#1-2-%u-0", (unsigned)family);
    value.dr_coordinate = node_id(ref);
    if (!value.coordinate || !value.dr_coordinate) return 0;
    value.family = family;
    value.row12 = row;
    value.col12 = col;
    value.scalar_valid = family < 5u;
    value.decimal10_valid = row < 10u && col < 10u;
    value.raw_terms[0] = (int32_t)(row * col);
    value.raw_terms[1] = value.raw_terms[0] + 1;
    value.raw_terms[2] = 2 * value.raw_terms[0] + 1;
    value.raw_terms[3] = -1;
    value.raw_terms[4] = 1;
    for (i = 0; i < 5u; ++i) {
        value.dr_terms[i] = i == 3u ? 9u : dr((uint32_t)value.raw_terms[i]);
        if (value.decimal10_valid) value.decimal_terms[i] = decimal(value.raw_terms[i]);
    }
    if (value.scalar_valid) {
        value.raw = value.raw_terms[family];
        value.digit_root = value.dr_terms[family];
        value.decimal10 = value.decimal_terms[family];
    }
    *out = value;
    return 1;
}
static const uint32_t valid_folds[] = {0,1,2,3,4,5,6,8,9,10,12,16,18,24};
int ql_m1_valid_fold(uint32_t fold) {
    size_t i;
    for (i = 0; i < sizeof(valid_folds)/sizeof(valid_folds[0]); ++i)
        if (valid_folds[i] == fold) return 1;
    return 0;
}
int ql_m1_spanda(uint32_t stage, uint32_t substage, QL_M1_Spanda *out) {
    static const char *const refs[6] = {
        "#1-3-4.0000", "#1-3-4.0/1", "#1-3-4.0/1/2", "#1-3-4.0/1/2/3",
        "#1-3-4.4.0-4.4/5", "#1-3-4.5/0"
    };
    static const uint32_t folds[6] = {4,6,8,10,12,0};
    static const float weave[6] = {0.0f,1.0f,1.0f,1.5f,4.0f,5.0f};
    static const uint32_t inversion[6] = {0,0,1,0,0,1};
    QL_M1_Spanda value = {0};
    char ref[32];
    if (!out || stage >= 6u || substage >= 6u || (stage != 4u && substage != 0u)) return 0;
    (void)snprintf(ref, sizeof(ref), "#1-3-%u", (unsigned)stage);
    value.coordinate = node_id(ref);
    if (!value.coordinate) return 0;
    value.stage = stage;
    value.substage = substage;
    value.weave_state = weave[stage];
    value.inversion_state = inversion[stage];
    if (stage == 4u) {
        value.substage_coordinate = node_id(refs[substage]);
        if (!value.substage_coordinate) return 0;
        value.fold_count = folds[substage];
        value.dual_track = substage == 3u;
    }
    *out = value;
    return 1;
}
int ql_m1_formal(uint32_t stage, QL_M1_Formal *out) {
    static const uint32_t numerator[6] = {0,0,6,5,6,5};
    static const uint32_t denominator[6] = {6,5,5,0,0,6};
    QL_M1_Formal value;
    char ref[32];
    if (!out || stage >= 6u) return 0;
    (void)snprintf(ref, sizeof(ref), "#1-4.%u", (unsigned)stage);
    value.coordinate = node_id(ref);
    if (!value.coordinate) return 0;
    value.stage = stage;
    value.next = (stage + 1u) % 6u;
    value.inverse = 5u - stage;
    value.numerator_position = numerator[stage];
    value.denominator_position = denominator[stage];
    value.signature = stage == 0u || stage == 5u ? -1 : 1;
    *out = value;
    return 1;
}
int ql_m1_topology(uint32_t tick, QL_M1_Topology *out) {
    static const uint32_t elements[12] = {1,2,2,3,4,5,8,10,12,6,7,11};
    static const float ring[12][2] = {
        {1,0},{.8660254f,.5f},{.5f,.8660254f},{0,1},{-.5f,.8660254f},{-.8660254f,.5f},
        {.8660254f,-.5f},{.5f,-.8660254f},{0,-1},{-.5f,-.8660254f},{-.8660254f,-.5f},{-1,0}
    };
    QL_M1_Topology value = {0};
    if (!out || tick >= 12u) return 0;
    value.coordinate = node_id("#1-5");
    if (!value.coordinate) return 0;
    value.tick12 = tick;
    value.element_count = elements[tick];
    value.legacy_return_stage = tick < 6u ? tick : 11u - tick;
    value.legacy_ring_quaternion[0] = ring[tick][0];
    value.legacy_ring_quaternion[1] = ring[tick][1];
    *out = value;
    return 1;
}
int ql_m1_torus(double theta1, double theta2, QL_M1_Torus *out) {
    QL_M1_Torus value;
    double radius;
    if (!out || !isfinite(theta1) || !isfinite(theta2)) return 0;
    value.coordinate = node_id("#1-5-1");
    if (!value.coordinate) return 0;
    radius = 16.0 / 9.0 + cos(theta1);
    value.x = radius * cos(theta2);
    value.y = radius * sin(theta2);
    value.z = sin(theta1);
    *out = value;
    return 1;
}
