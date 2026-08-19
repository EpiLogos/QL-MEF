#include "ql/kernel.h"

#include <math.h>

#ifndef QL_C_SOURCE_REVISION
#define QL_C_SOURCE_REVISION "unversioned"
#endif

static const char QL_KERNEL_VERSION[] = "0.1.0";
static const char QL_KERNEL_SOURCE_REVISION[] = QL_C_SOURCE_REVISION;

const char* ql_kernel_api_version(void) {
    return QL_KERNEL_VERSION;
}

const char* ql_kernel_build_source_revision(void) {
    return QL_KERNEL_SOURCE_REVISION;
}

static float ql_quat_norm_sq(QL_Quaternion q) {
    return q.w*q.w + q.x*q.x + q.y*q.y + q.z*q.z;
}

QL_Quaternion ql_quat_normalize(QL_Quaternion q) {
    float norm_sq = ql_quat_norm_sq(q);
    if (norm_sq <= 0.0f) {
        return (QL_Quaternion){ .w = 1.0f, .x = 0.0f, .y = 0.0f, .z = 0.0f };
    }
    float scale = 1.0f / sqrtf(norm_sq);
    return (QL_Quaternion){
        .w = q.w * scale,
        .x = q.x * scale,
        .y = q.y * scale,
        .z = q.z * scale
    };
}

QL_Quaternion ql_quat_conjugate(QL_Quaternion q) {
    return (QL_Quaternion){ .w = q.w, .x = -q.x, .y = -q.y, .z = -q.z };
}

QL_Quaternion ql_quat_multiply(QL_Quaternion a, QL_Quaternion b) {
    return (QL_Quaternion){
        .w = a.w*b.w - a.x*b.x - a.y*b.y - a.z*b.z,
        .x = a.w*b.x + a.x*b.w + a.y*b.z - a.z*b.y,
        .y = a.w*b.y - a.x*b.z + a.y*b.w + a.z*b.x,
        .z = a.w*b.z + a.x*b.y - a.y*b.x + a.z*b.w
    };
}

float ql_quat_distance_sq(QL_Quaternion a, QL_Quaternion b) {
    float dw = a.w - b.w;
    float dx = a.x - b.x;
    float dy = a.y - b.y;
    float dz = a.z - b.z;
    return dw*dw + dx*dx + dy*dy + dz*dz;
}

QL_Kernel_Bioquaternion ql_kernel_bioquaternion_init(QL_Quaternion q_b, QL_Quaternion q_p) {
    return (QL_Kernel_Bioquaternion){
        .q_b = ql_quat_normalize(q_b),
        .q_p = ql_quat_normalize(q_p)
    };
}

QL_Quaternion ql_kernel_slash_flip_bimba_prime(QL_Kernel_Bioquaternion state) {
    return ql_quat_normalize(ql_quat_conjugate(state.q_p));
}

uint8_t ql_kernel_resonance_index(uint8_t lens, uint8_t face, uint8_t position) {
    return ql_resonance_index(lens, face, position);
}

uint8_t ql_kernel_tritone_square_for_lens(uint8_t lens) {
    return ql_tritone_square_for_lens(lens);
}

void ql_kernel_resonance_square_emphasis(
    const QL_Kernel_Resonance_Vector* vector,
    float out_square_emphasis[QL_KERNEL_TRITONE_SQUARES]
) {
    if (!out_square_emphasis) return;
    for (uint8_t square = 0u; square < QL_KERNEL_TRITONE_SQUARES; square++) {
        out_square_emphasis[square] = 0.0f;
    }
    if (!vector) return;

    uint8_t counts[QL_KERNEL_TRITONE_SQUARES] = {0u, 0u, 0u};
    for (uint8_t lens = 0u; lens < QL_POSITION_COUNT; lens++) {
        uint8_t square = ql_kernel_tritone_square_for_lens(lens);
        if (square >= QL_KERNEL_TRITONE_SQUARES) continue;
        for (uint8_t face = 0u; face < QL_FACE_COUNT; face++) {
            for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
                uint8_t index = ql_kernel_resonance_index(lens, face, position);
                out_square_emphasis[square] += vector->values[index];
                counts[square]++;
            }
        }
    }
    for (uint8_t square = 0u; square < QL_KERNEL_TRITONE_SQUARES; square++) {
        if (counts[square]) {
            out_square_emphasis[square] /= (float)counts[square];
        }
    }
}

QL_Kernel_Energy ql_kernel_energy_evaluate(
    QL_Kernel_Bioquaternion state,
    const QL_Kernel_Resonance_Vector* observed,
    const QL_Kernel_Resonance_Vector* target,
    float r_energy
) {
    QL_Kernel_Energy energy = {
        .bimba_pratibimba_energy = ql_quat_distance_sq(state.q_b, state.q_p),
        .lens_energy = 0.0f,
        .r_energy = r_energy,
        .total_energy = 0.0f
    };

    if (observed && target) {
        float sum = 0.0f;
        for (uint8_t i = 0u; i < QL_KERNEL_RESONANCE_DIM; i++) {
            float delta = observed->values[i] - target->values[i];
            sum += delta * delta;
        }
        energy.lens_energy = sum / (float)QL_KERNEL_RESONANCE_DIM;
    }
    energy.total_energy = energy.bimba_pratibimba_energy + energy.lens_energy + energy.r_energy;
    return energy;
}

float ql_kernel_epogdoon_log(void) {
    return logf(ql_epogdoon_ratio());
}

QL_Kernel_Tick ql_kernel_tick_from_epogdoon(uint64_t cycle, uint8_t sub_tick) {
    static const QL_Kernel_Element elements[QL_TICK_COUNT] = {
        QL_KERNEL_ELEMENT_BIMBA_ENCODING,
        QL_KERNEL_ELEMENT_PRATIBIMBA_PREHENSION,
        QL_KERNEL_ELEMENT_MOBIUS_DESCENT,
        QL_KERNEL_ELEMENT_MOBIUS_DESCENT,
        QL_KERNEL_ELEMENT_SLASH_FLIP,
        QL_KERNEL_ELEMENT_PRATIBIMBA_AS_BIMBA,
        QL_KERNEL_ELEMENT_DOUBLED_PREHENSION,
        QL_KERNEL_ELEMENT_INVERSE_MOBIUS,
        QL_KERNEL_ELEMENT_INVERSE_MOBIUS,
        QL_KERNEL_ELEMENT_ENRICHED_RETURN,
        QL_KERNEL_ELEMENT_ENRICHED_RETURN,
        QL_KERNEL_ELEMENT_ENRICHED_RETURN
    };
    QL_Primitive_Tick primitive = ql_tick_from_epogdoon(cycle, sub_tick);
    return (QL_Kernel_Tick){
        .cycle = primitive.cycle,
        .sub_tick = primitive.sub_tick,
        .half = primitive.half,
        .phase = primitive.sub_tick < QL_POSITION_COUNT ? QL_KERNEL_PHASE_DESCENT : QL_KERNEL_PHASE_ASCENT,
        .element = elements[primitive.sub_tick],
        .position6 = primitive.base_position,
        .base_position = primitive.base_position,
        .traversal_stage = primitive.traversal_stage,
        .harmonic_ratio = primitive.harmonic_ratio
    };
}
