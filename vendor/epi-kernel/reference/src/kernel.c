/**
 * kernel.c — Epi-Logos bioquaternionic kernel primitives
 */

#include "kernel.h"
#include <math.h>
#include <stddef.h>

static Quaternion kernel_unit_or_identity(Quaternion q) {
    float norm_sq = quat_norm_sq(q);
    if (norm_sq <= 0.0f) {
        return (Quaternion){ .w = 1.0f, .x = 0.0f, .y = 0.0f, .z = 0.0f };
    }
    return quat_normalize(q);
}

float kernel_epogdoon_ratio(void) {
    return (float)KERNEL_EPOGDOON_NUM / (float)KERNEL_EPOGDOON_DEN;
}

float kernel_epogdoon_log(void) {
    return logf(kernel_epogdoon_ratio());
}

float kernel_ratio_ascending_fourth(void) {
    return 4.0f / 3.0f;
}

float kernel_ratio_descending_fourth(void) {
    return 3.0f / 4.0f;
}

float kernel_ratio_descending_fifth(void) {
    return 2.0f / 3.0f;
}

float kernel_ratio_ascending_fifth(void) {
    return 3.0f / 2.0f;
}

Kernel_Bioquaternion kernel_bioquaternion_init(Quaternion q_b, Quaternion q_p) {
    return (Kernel_Bioquaternion){
        .q_b = kernel_unit_or_identity(q_b),
        .q_p = kernel_unit_or_identity(q_p)
    };
}

float kernel_quat_distance_sq(Quaternion a, Quaternion b) {
    float dw = a.w - b.w;
    float dx = a.x - b.x;
    float dy = a.y - b.y;
    float dz = a.z - b.z;
    return dw*dw + dx*dx + dy*dy + dz*dz;
}

Quaternion kernel_slash_flip_bimba_prime(Kernel_Bioquaternion state) {
    return kernel_unit_or_identity(quat_conj(state.q_p));
}

uint8_t kernel_resonance_index(uint8_t lens, uint8_t helix, uint8_t position) {
    if (lens >= MEF_BASE_LENSES || helix >= 2u || position >= QL_POSITIONS) {
        return 0xFFu;
    }
    return (uint8_t)((lens * 12u) + (helix * 6u) + position);
}

uint8_t kernel_tritone_square_for_lens(uint8_t lens) {
    switch (lens) {
        case 0u:
        case 5u:
            return 0u;
        case 1u:
        case 4u:
            return 1u;
        case 2u:
        case 3u:
            return 2u;
        default:
            return 0xFFu;
    }
}

void kernel_resonance_square_emphasis(
    const Kernel_Resonance_Vector* vector,
    float out_square_emphasis[KERNEL_TRITONE_SQUARES]
) {
    if (!out_square_emphasis) return;
    out_square_emphasis[0] = 0.0f;
    out_square_emphasis[1] = 0.0f;
    out_square_emphasis[2] = 0.0f;
    if (!vector) return;

    uint8_t counts[KERNEL_TRITONE_SQUARES] = {0u, 0u, 0u};
    for (uint8_t lens = 0u; lens < MEF_BASE_LENSES; lens++) {
        uint8_t square = kernel_tritone_square_for_lens(lens);
        if (square >= KERNEL_TRITONE_SQUARES) continue;
        for (uint8_t helix = 0u; helix < 2u; helix++) {
            for (uint8_t position = 0u; position < QL_POSITIONS; position++) {
                uint8_t idx = kernel_resonance_index(lens, helix, position);
                out_square_emphasis[square] += vector->values[idx];
                counts[square]++;
            }
        }
    }
    for (uint8_t square = 0u; square < KERNEL_TRITONE_SQUARES; square++) {
        if (counts[square] > 0u) {
            out_square_emphasis[square] /= (float)counts[square];
        }
    }
}

Kernel_Energy kernel_energy_evaluate(
    Kernel_Bioquaternion state,
    const Kernel_Resonance_Vector* observed,
    const Kernel_Resonance_Vector* target,
    float r_energy
) {
    Kernel_Energy energy = {
        .bimba_pratibimba_energy = kernel_quat_distance_sq(state.q_b, state.q_p),
        .lens_energy = 0.0f,
        .r_energy = r_energy,
        .total_energy = 0.0f
    };

    if (observed && target) {
        float sum = 0.0f;
        for (uint8_t i = 0u; i < KERNEL_RESONANCE_DIM; i++) {
            float delta = observed->values[i] - target->values[i];
            sum += delta * delta;
        }
        energy.lens_energy = sum / (float)KERNEL_RESONANCE_DIM;
    }

    energy.total_energy =
        energy.bimba_pratibimba_energy + energy.lens_energy + energy.r_energy;
    return energy;
}

Kernel_Tick kernel_tick_from_epogdoon(uint64_t cycle, uint8_t sub_tick) {
    static const Kernel_Element elements[12] = {
        KERNEL_ELEMENT_BIMBA_ENCODING,
        KERNEL_ELEMENT_PRATIBIMBA_PREHENSION,
        KERNEL_ELEMENT_MOBIUS_DESCENT,
        KERNEL_ELEMENT_MOBIUS_DESCENT,
        KERNEL_ELEMENT_SLASH_FLIP,
        KERNEL_ELEMENT_PRATIBIMBA_AS_BIMBA,
        KERNEL_ELEMENT_DOUBLED_PREHENSION,
        KERNEL_ELEMENT_INVERSE_MOBIUS,
        KERNEL_ELEMENT_INVERSE_MOBIUS,
        KERNEL_ELEMENT_ENRICHED_RETURN,
        KERNEL_ELEMENT_ENRICHED_RETURN,
        KERNEL_ELEMENT_ENRICHED_RETURN
    };
    static const float ratios[12] = {
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

    uint8_t tick = (uint8_t)(sub_tick % RING_SIZE);
    return (Kernel_Tick){
        .cycle = cycle,
        .sub_tick = tick,
        .phase = tick < 6u ? KERNEL_PHASE_DESCENT : KERNEL_PHASE_ASCENT,
        .element = elements[tick],
        .position6 = (uint8_t)(tick % QL_POSITIONS),
        .harmonic_ratio = ratios[tick]
    };
}
