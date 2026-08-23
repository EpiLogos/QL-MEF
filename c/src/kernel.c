#include "ql/kernel.h"

#include <math.h>
#include <stdio.h>

#ifndef QL_C_SOURCE_REVISION
#define QL_C_SOURCE_REVISION "unversioned"
#endif

static const char QL_KERNEL_VERSION[] = "0.1.0";
static const char QL_KERNEL_SOURCE_REVISION[] = QL_C_SOURCE_REVISION;
static const char QL_KERNEL_CONTRACT_VERSION[] = QL_HOLOGRAPHIC_KERNEL_CONTRACT_VERSION;

static const QL_Kernel_Contract_Provenance QL_KERNEL_PROVENANCE = {
    .contract_version = QL_HOLOGRAPHIC_KERNEL_CONTRACT_VERSION,
    .historical_reference_revision = "daa660cbc1b8c5da83828698665a753852cb0287",
    .historical_pointer_web_blob = "3eeae6f9c8cc65c5a610df1a49143b3c65bdd320",
    .rust_pairing_version = "1.0.0",
    .rust_mef_rotation_version = "1.0.0",
    .rust_context_frame_version = "1.0.0"
};

static const char* const QL_KERNEL_RELATION_IDS[QL_KERNEL_RELATION_COUNT] = {
    "ql.kernel.position.identity/v1",
    "ql.kernel.family.same-position/v1",
    "ql.kernel.cross.same-position/v1",
    "ql.kernel.pair.A/v1",
    "ql.kernel.pair.B/v1",
    "ql.kernel.pair.C/v1",
    "ql.kernel.cross.transform/v1",
    "ql.kernel.cross.require/v1",
    "ql.kernel.cross.complete/v1",
    "ql.kernel.conjugate-invariance.A/v1",
    "ql.kernel.conjugate-invariance.B/v1",
    "ql.kernel.conjugate-invariance.C/v1",
    "ql.kernel.mirror.complement/v1",
    "ql.kernel.position.successor/v1",
    "ql.kernel.return.mobius/v1",
    "ql.kernel.lens.anchor/v1",
    "ql.kernel.context-frame/v1",
    "ql.kernel.nesting/v1",
    "ql.kernel.branching/v1",
    "ql.kernel.source.provenance/v1"
};

const char* ql_kernel_api_version(void) {
    return QL_KERNEL_VERSION;
}

const char* ql_kernel_build_source_revision(void) {
    return QL_KERNEL_SOURCE_REVISION;
}

const char* ql_kernel_contract_version(void) {
    return QL_KERNEL_CONTRACT_VERSION;
}

QL_Kernel_Contract_Provenance ql_kernel_contract_provenance(void) {
    return QL_KERNEL_PROVENANCE;
}

const char* ql_kernel_relation_id(QL_Kernel_Relation_Id relation) {
    return (unsigned)relation < QL_KERNEL_RELATION_COUNT
        ? QL_KERNEL_RELATION_IDS[(unsigned)relation]
        : NULL;
}

uint8_t ql_kernel_pitch_class(uint8_t position, QL_Coordinate_Face face) {
    if (position >= QL_POSITION_COUNT || face > QL_COORD_FACE_PRIME) return QL_INVALID_U8;
    return (uint8_t)(((2u * position) + (uint8_t)face) % 12u);
}

uint8_t ql_kernel_mirror_position(uint8_t position) {
    return position < QL_POSITION_COUNT ? (uint8_t)(5u - position) : QL_INVALID_U8;
}

uint8_t ql_kernel_mirror_interval_role(uint8_t position) {
    if (position >= QL_POSITION_COUNT) return QL_KERNEL_INTERVAL_NONE;
    uint8_t mirror = ql_kernel_mirror_position(position);
    uint8_t whole_tones = position > mirror
        ? (uint8_t)(position - mirror)
        : (uint8_t)(mirror - position);
    uint8_t semitones = (uint8_t)(2u * whole_tones);
    if (semitones == QL_KERNEL_INTERVAL_WHOLE_TONE) return QL_KERNEL_INTERVAL_WHOLE_TONE;
    if (semitones == QL_KERNEL_INTERVAL_TRITONE) return QL_KERNEL_INTERVAL_TRITONE;
    if (semitones == QL_KERNEL_INTERVAL_TOTALITY_16_9) return QL_KERNEL_INTERVAL_TOTALITY_16_9;
    return QL_KERNEL_INTERVAL_NONE;
}

uint8_t ql_kernel_mirror_ratio_role(uint8_t position) {
    uint8_t interval = ql_kernel_mirror_interval_role(position);
    if (interval == QL_KERNEL_INTERVAL_WHOLE_TONE) return QL_KERNEL_RATIO_EPOGDOON;
    if (interval == QL_KERNEL_INTERVAL_TOTALITY_16_9) return QL_KERNEL_RATIO_TOTALITY;
    return QL_KERNEL_RATIO_NONE;
}

static uint8_t ql_pair_target(QL_Kernel_Relation_Id relation, uint8_t position) {
    static const uint8_t pair_a[QL_POSITION_COUNT] = {1u, 0u, 3u, 2u, 5u, 4u};
    static const uint8_t pair_b[QL_POSITION_COUNT] = {5u, 2u, 1u, 4u, 3u, 0u};
    static const uint8_t pair_c[QL_POSITION_COUNT] = {5u, 4u, 3u, 2u, 1u, 0u};
    if (position >= QL_POSITION_COUNT) return QL_INVALID_U8;
    switch (relation) {
        case QL_KERNEL_REL_PAIR_A:
        case QL_KERNEL_REL_CONJUGATE_INVARIANCE_A:
            return pair_a[position];
        case QL_KERNEL_REL_PAIR_B:
        case QL_KERNEL_REL_CONJUGATE_INVARIANCE_B:
            return pair_b[position];
        case QL_KERNEL_REL_PAIR_C:
        case QL_KERNEL_REL_CONJUGATE_INVARIANCE_C:
            return pair_c[position];
        default:
            return QL_INVALID_U8;
    }
}

static void ql_relation_harmonic_roles(
    QL_Kernel_Relation_Id relation,
    uint8_t source_position,
    QL_Kernel_Relation_Ref* out
) {
    out->interval_role = QL_KERNEL_INTERVAL_NONE;
    out->ratio_role = QL_KERNEL_RATIO_NONE;
    switch (relation) {
        case QL_KERNEL_REL_POSITION_IDENTITY:
        case QL_KERNEL_REL_FAMILY_SAME_POSITION:
        case QL_KERNEL_REL_LENS_ANCHOR:
            out->ratio_role = QL_KERNEL_RATIO_UNISON;
            break;
        case QL_KERNEL_REL_CROSS_SAME_POSITION:
            out->interval_role = QL_KERNEL_INTERVAL_SEMITONE;
            break;
        case QL_KERNEL_REL_POSITION_SUCCESSOR:
            out->interval_role = QL_KERNEL_INTERVAL_WHOLE_TONE;
            out->ratio_role = QL_KERNEL_RATIO_EPOGDOON;
            break;
        case QL_KERNEL_REL_MOBIUS_RETURN:
            out->interval_role = QL_KERNEL_INTERVAL_OCTAVE;
            out->ratio_role = QL_KERNEL_RATIO_OCTAVE;
            break;
        case QL_KERNEL_REL_MIRROR_COMPLEMENT:
            out->interval_role = ql_kernel_mirror_interval_role(source_position);
            out->ratio_role = ql_kernel_mirror_ratio_role(source_position);
            break;
        default:
            break;
    }
}

int ql_kernel_relation_resolve(
    QL_Kernel_Relation_Id relation,
    QL_Kernel_Address source,
    QL_Coordinate_Family family_target,
    QL_Kernel_Relation_Ref* out
) {
    if (!out || !ql_kernel_address_valid(source) || !ql_kernel_relation_id(relation)) return -1;

    QL_Kernel_Address target = source;
    if (ql_kernel_address_is_hash(source)) {
        if (relation != QL_KERNEL_REL_POSITION_IDENTITY &&
            relation != QL_KERNEL_REL_SOURCE_PROVENANCE) {
            return -1;
        }
    } else {
        uint8_t target_position;
        switch (relation) {
            case QL_KERNEL_REL_POSITION_IDENTITY:
            case QL_KERNEL_REL_CONTEXT_FRAME:
            case QL_KERNEL_REL_NESTING:
            case QL_KERNEL_REL_BRANCHING:
            case QL_KERNEL_REL_SOURCE_PROVENANCE:
                break;
            case QL_KERNEL_REL_FAMILY_SAME_POSITION:
                target = ql_kernel_family_address(
                    family_target,
                    source.position,
                    (QL_Coordinate_Face)source.face
                );
                break;
            case QL_KERNEL_REL_CROSS_SAME_POSITION:
                target = ql_coordinate_label_other_face(source);
                break;
            case QL_KERNEL_REL_PAIR_A:
            case QL_KERNEL_REL_PAIR_B:
            case QL_KERNEL_REL_PAIR_C:
                target_position = ql_pair_target(relation, source.position);
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    target_position,
                    (QL_Coordinate_Face)source.face
                );
                break;
            case QL_KERNEL_REL_CROSS_TRANSFORM:
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    (uint8_t)((source.position + 1u) % QL_POSITION_COUNT),
                    source.face == (uint8_t)QL_COORD_FACE_DIRECT
                        ? QL_COORD_FACE_PRIME : QL_COORD_FACE_DIRECT
                );
                break;
            case QL_KERNEL_REL_CROSS_REQUIRE:
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    (uint8_t)((source.position + QL_POSITION_COUNT - 1u) % QL_POSITION_COUNT),
                    source.face == (uint8_t)QL_COORD_FACE_DIRECT
                        ? QL_COORD_FACE_PRIME : QL_COORD_FACE_DIRECT
                );
                break;
            case QL_KERNEL_REL_CROSS_COMPLETE:
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    ql_kernel_mirror_position(source.position),
                    source.face == (uint8_t)QL_COORD_FACE_DIRECT
                        ? QL_COORD_FACE_PRIME : QL_COORD_FACE_DIRECT
                );
                break;
            case QL_KERNEL_REL_CONJUGATE_INVARIANCE_A:
            case QL_KERNEL_REL_CONJUGATE_INVARIANCE_B:
            case QL_KERNEL_REL_CONJUGATE_INVARIANCE_C:
                if (source.face != (uint8_t)QL_COORD_FACE_PRIME) return -1;
                target_position = ql_pair_target(relation, source.position);
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    target_position,
                    QL_COORD_FACE_PRIME
                );
                break;
            case QL_KERNEL_REL_MIRROR_COMPLEMENT:
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    ql_kernel_mirror_position(source.position),
                    (QL_Coordinate_Face)source.face
                );
                break;
            case QL_KERNEL_REL_POSITION_SUCCESSOR:
                if (source.position == 5u) return -1;
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    (uint8_t)(source.position + 1u),
                    (QL_Coordinate_Face)source.face
                );
                break;
            case QL_KERNEL_REL_MOBIUS_RETURN:
                if (source.position != 5u) return -1;
                target = ql_kernel_family_address(
                    (QL_Coordinate_Family)source.family,
                    0u,
                    source.face == (uint8_t)QL_COORD_FACE_DIRECT
                        ? QL_COORD_FACE_PRIME : QL_COORD_FACE_DIRECT
                );
                break;
            case QL_KERNEL_REL_LENS_ANCHOR:
                target = ql_kernel_family_address(
                    QL_FAMILY_L,
                    source.position,
                    (QL_Coordinate_Face)source.face
                );
                break;
            default:
                return -1;
        }
    }

    if (!ql_kernel_address_valid(target)) return -1;
    *out = (QL_Kernel_Relation_Ref){
        .relation = relation,
        .source = source,
        .target = target,
        .interval_role = QL_KERNEL_INTERVAL_NONE,
        .ratio_role = QL_KERNEL_RATIO_NONE,
        .pitch_class = ql_kernel_address_is_hash(target)
            ? 0u
            : ql_kernel_pitch_class(target.position, (QL_Coordinate_Face)target.face)
    };
    ql_relation_harmonic_roles(relation,
        ql_kernel_address_is_hash(source) ? 0u : source.position,
        out);
    return 0;
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

int ql_kernel_resonance_map(
    uint8_t lens,
    uint8_t face,
    uint8_t position,
    QL_Kernel_Resonance_Map* out
) {
    if (!out || lens >= QL_POSITION_COUNT || face >= QL_FACE_COUNT ||
        position >= QL_POSITION_COUNT) {
        return -1;
    }
    uint8_t index = ql_kernel_resonance_index(lens, face, position);
    if (index == QL_INVALID_U8) return -1;

    out->lens = ql_kernel_family_address(
        QL_FAMILY_L,
        lens,
        face == 0u ? QL_COORD_FACE_DIRECT : QL_COORD_FACE_PRIME
    );
    out->inner_position = position;
    out->resonance_index = index;
    out->tritone_square = ql_kernel_tritone_square_for_lens(lens);
    return 0;
}

int ql_kernel_mef_address(
    uint8_t lens_position,
    QL_Coordinate_Face lens_face,
    uint8_t local_position,
    QL_Kernel_MEF_Address* out
) {
    if (!out || lens_position >= QL_POSITION_COUNT ||
        lens_face > QL_COORD_FACE_PRIME || local_position >= QL_POSITION_COUNT) {
        return -1;
    }
    uint8_t resonance = ql_kernel_resonance_index(
        lens_position,
        (uint8_t)lens_face,
        local_position
    );
    if (resonance == QL_INVALID_U8) return -1;

    uint8_t lens_anchor_pc = ql_kernel_pitch_class(lens_position, lens_face);
    uint8_t local_pc = ql_kernel_pitch_class(local_position, QL_COORD_FACE_DIRECT);
    *out = (QL_Kernel_MEF_Address){
        .lens = ql_kernel_family_address(QL_FAMILY_L, lens_position, lens_face),
        .local_position = local_position,
        .absolute_position = (uint8_t)((lens_position + local_position) % QL_POSITION_COUNT),
        .resonance_index = resonance,
        .tritone_square = ql_kernel_tritone_square_for_lens(lens_position),
        .pitch_class = (uint8_t)((lens_anchor_pc + local_pc) % 12u)
    };
    return 0;
}

int ql_kernel_mef_address_format(
    const QL_Kernel_MEF_Address* address,
    char* out,
    size_t out_size
) {
    if (!address || !out || out_size == 0u ||
        !ql_kernel_address_valid(address->lens) ||
        address->lens.family != (uint8_t)QL_FAMILY_L ||
        address->local_position >= QL_POSITION_COUNT) {
        return -1;
    }
    int written = snprintf(
        out,
        out_size,
        "mef:sublens:L%u%s.%u@1",
        (unsigned)address->lens.position,
        address->lens.face == (uint8_t)QL_COORD_FACE_PRIME ? "'" : "",
        (unsigned)address->local_position
    );
    return written >= 0 && (size_t)written < out_size ? 0 : -1;
}

int ql_kernel_context_frame_address(
    QL_Kernel_Context_Frame_Id frame,
    uint8_t lens_position,
    QL_Coordinate_Face lens_face,
    QL_Kernel_Context_Frame_Address* out
) {
    static const uint8_t local_positions[QL_KERNEL_CONTEXT_FRAME_COUNT] = {
        0u, 1u, 2u, 2u, 3u, 4u, 5u
    };
    static const QL_Kernel_MEF_Unit_Face unit_faces[QL_KERNEL_CONTEXT_FRAME_COUNT] = {
        QL_KERNEL_MEF_UNIT_NAME,
        QL_KERNEL_MEF_UNIT_NAME,
        QL_KERNEL_MEF_UNIT_NAME,
        QL_KERNEL_MEF_UNIT_POWER,
        QL_KERNEL_MEF_UNIT_POWER,
        QL_KERNEL_MEF_UNIT_POWER,
        QL_KERNEL_MEF_UNIT_POWER
    };
    static const QL_Kernel_MEF_Grain grains[QL_KERNEL_CONTEXT_FRAME_COUNT] = {
        QL_KERNEL_MEF_GRAIN_OUTER_TWO,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_OUTER_TWO
    };
    static const char* const notations[QL_KERNEL_CONTEXT_FRAME_COUNT] = {
        "(00/00)",
        "(0/1)",
        "(0/1/2)",
        "(0/1/2/3)",
        "(4.0/1-4.4/5)",
        "(4.5/0)",
        "(5/0)"
    };

    if (!out || (unsigned)frame >= QL_KERNEL_CONTEXT_FRAME_COUNT) return -1;
    QL_Kernel_MEF_Address mef;
    if (ql_kernel_mef_address(
            lens_position,
            lens_face,
            local_positions[(unsigned)frame],
            &mef) != 0) {
        return -1;
    }
    *out = (QL_Kernel_Context_Frame_Address){
        .frame = frame,
        .mef = mef,
        .unit_face = unit_faces[(unsigned)frame],
        .grain = grains[(unsigned)frame],
        .notation = notations[(unsigned)frame]
    };
    return 0;
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
        .phase = primitive.sub_tick < QL_POSITION_COUNT ? QL_KERNEL_PHASE_DESCENT : QL_KERNEL_PHASE_ASCENT,
        .element = elements[primitive.sub_tick],
        .position6 = primitive.base_position,
        .harmonic_ratio = primitive.harmonic_ratio
    };
}

QL_Coordinate_Label ql_kernel_tick_position_label(const QL_Kernel_Tick* tick) {
    if (!tick || tick->sub_tick >= QL_TICK_COUNT || tick->position6 >= QL_POSITION_COUNT) {
        return ql_coordinate_label(QL_FAMILY_P, QL_INVALID_U8, QL_COORD_FACE_DIRECT);
    }
    return ql_kernel_family_address(
        QL_FAMILY_P,
        tick->position6,
        tick->sub_tick < QL_POSITION_COUNT
            ? QL_COORD_FACE_DIRECT
            : QL_COORD_FACE_PRIME
    );
}
