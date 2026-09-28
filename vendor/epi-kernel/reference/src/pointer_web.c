/**
 * pointer_web.c -- Derived harmonic pointer web implementation.
 */

#include "pointer_web.h"
#include <stddef.h>
#include <string.h>

static const Coordinate_Family HC_FAMILIES[HC_WEB_HELIX_SIZE] = {
    FAMILY_C, FAMILY_P, FAMILY_L, FAMILY_S, FAMILY_T, FAMILY_M
};

static const uint8_t HC_LENS_ANCHOR_PC[HC_WEB_RING_SIZE] = {
    0u, 2u, 4u, 6u, 8u, 10u,
    1u, 3u, 5u, 7u, 9u, 11u
};

static const char* HC_CF_NOTATION[CF_COUNT] = {
    "(00/00)",
    "(0/1)",
    "(0/1/2)",
    "(0/1/2/3)",
    "(4.0/1-4.4/5)",
    "(4.5/0)",
    "(5/0)"
};

static const uint8_t HC_CF_QL_POSITION[CF_COUNT] = {
    0u, 1u, 2u, 2u, 3u, 4u, 5u
};

static const uint8_t HC_CF_HELIX[CF_COUNT] = {
    HC_HELIX_BIMBA,
    HC_HELIX_BIMBA,
    HC_HELIX_BIMBA,
    HC_HELIX_PRATIBIMBA,
    HC_HELIX_PRATIBIMBA,
    HC_HELIX_PRATIBIMBA,
    HC_HELIX_PRATIBIMBA
};

static const Holographic_Coordinate* HC_RAW_PSYCHOIDS[HC_BEDROCK_POSITION_COUNT] = {
    &Psychoid_0, &Psychoid_1, &Psychoid_2,
    &Psychoid_3, &Psychoid_4, &Psychoid_5
};

typedef struct {
    uint8_t first;
    uint8_t second;
    HC_HarmonicFamily family;
} HC_HarmonicPairRow;

static const HC_HarmonicPairRow HC_HARMONIC_PAIR_ROWS[] = {
    {0u, 1u, HC_HARMONIC_FAMILY_A},
    {2u, 3u, HC_HARMONIC_FAMILY_A},
    {4u, 5u, HC_HARMONIC_FAMILY_A},
    {0u, 5u, HC_HARMONIC_FAMILY_B},
    {1u, 4u, HC_HARMONIC_FAMILY_B},
    {2u, 3u, HC_HARMONIC_FAMILY_B},
    {1u, 2u, HC_HARMONIC_FAMILY_C},
    {3u, 4u, HC_HARMONIC_FAMILY_C},
    {5u, 0u, HC_HARMONIC_FAMILY_C}
};

uint8_t hc_bimba_pitch_class(uint8_t ql_position) {
    return (uint8_t)((2u * (ql_position % HC_WEB_HELIX_SIZE)) % HC_WEB_RING_SIZE);
}

uint8_t hc_pratibimba_pitch_class(uint8_t ql_position) {
    return (uint8_t)((hc_bimba_pitch_class(ql_position) + 1u) % HC_WEB_RING_SIZE);
}

uint8_t hc_lens_bimba_pitch_class(uint8_t lens_anchor_pc, uint8_t ql_position) {
    return (uint8_t)(((lens_anchor_pc % HC_WEB_RING_SIZE) + hc_bimba_pitch_class(ql_position)) % HC_WEB_RING_SIZE);
}

uint8_t hc_lens_pratibimba_pitch_class(uint8_t lens_anchor_pc, uint8_t ql_position) {
    return (uint8_t)((hc_lens_bimba_pitch_class(lens_anchor_pc, ql_position) + 1u) % HC_WEB_RING_SIZE);
}

uint8_t hc_mirror_position(uint8_t ql_position) {
    return (uint8_t)(5u - (ql_position % HC_WEB_HELIX_SIZE));
}

uint8_t hc_mirror_interval_role(uint8_t ql_position) {
    uint8_t q = (uint8_t)(ql_position % HC_WEB_HELIX_SIZE);
    uint8_t mirror = hc_mirror_position(q);
    uint8_t whole_tones = (q > mirror) ? (uint8_t)(q - mirror) : (uint8_t)(mirror - q);
    uint8_t semitones = (uint8_t)(2u * whole_tones);

    if (semitones == HC_INTERVAL_WHOLE_TONE) return HC_INTERVAL_WHOLE_TONE;
    if (semitones == HC_INTERVAL_TRITONE) return HC_INTERVAL_TRITONE;
    if (semitones == HC_INTERVAL_TOTALITY_16_9) return HC_INTERVAL_TOTALITY_16_9;
    return HC_INTERVAL_NONE;
}

uint8_t hc_mirror_ratio_role(uint8_t ql_position) {
    uint8_t interval = hc_mirror_interval_role(ql_position);
    if (interval == HC_INTERVAL_WHOLE_TONE) return HC_RATIO_EPOGDOON;
    if (interval == HC_INTERVAL_TRITONE) return HC_RATIO_NONE;
    if (interval == HC_INTERVAL_TOTALITY_16_9) return HC_RATIO_TOTALITY;
    return HC_RATIO_NONE;
}

uint8_t hc_harmonic_families_for_pair(
    uint8_t first_ql_position,
    uint8_t second_ql_position,
    uint8_t* out,
    uint8_t out_capacity
) {
    if (!out || out_capacity == 0u) return 0u;

    uint8_t first = (uint8_t)(first_ql_position % HC_WEB_HELIX_SIZE);
    uint8_t second = (uint8_t)(second_ql_position % HC_WEB_HELIX_SIZE);
    uint8_t count = 0u;
    uint8_t row_count = (uint8_t)(sizeof(HC_HARMONIC_PAIR_ROWS) / sizeof(HC_HARMONIC_PAIR_ROWS[0]));
    for (uint8_t i = 0u; i < row_count && count < out_capacity; i++) {
        const HC_HarmonicPairRow* row = &HC_HARMONIC_PAIR_ROWS[i];
        if (row->first == first && row->second == second) {
            out[count++] = row->family;
        }
    }
    return count;
}

uint8_t hc_harmonic_depth_for_d_face(uint8_t face) {
    switch (face) {
        case HC_D_FACE_LEFT:
        case HC_D_FACE_RIGHT:
            return 3u;
        case HC_D_FACE_BOTH:
            return 4u;
        case HC_D_FACE_NONE:
        default:
            return 2u;
    }
}

uint8_t hc_harmonic_d_face(uint8_t first_is_prime, uint8_t second_is_prime) {
    if (first_is_prime && second_is_prime) return HC_D_FACE_BOTH;
    if (first_is_prime) return HC_D_FACE_LEFT;
    if (second_is_prime) return HC_D_FACE_RIGHT;
    return HC_D_FACE_NONE;
}

uint8_t hc_harmonic_register_for_family(uint8_t family) {
    switch (family) {
        case HC_HARMONIC_FAMILY_A:
            return HC_HARMONIC_REGISTER_BEING;
        case HC_HARMONIC_FAMILY_B:
            return HC_HARMONIC_REGISTER_BECOMING;
        case HC_HARMONIC_FAMILY_C:
            return HC_HARMONIC_REGISTER_KNOWING_UNKNOWING;
        case HC_HARMONIC_FAMILY_D:
            return HC_HARMONIC_REGISTER_INVERSION;
        case HC_HARMONIC_FAMILY_NONE:
        default:
            return HC_HARMONIC_REGISTER_NONE;
    }
}

uint8_t hc_harmonic_relation_type(uint8_t family, uint8_t face) {
    switch (face) {
        case HC_D_FACE_LEFT:
            return HC_REL_INVERTS_THROUGH_FIRST;
        case HC_D_FACE_RIGHT:
            return HC_REL_INVERTS_THROUGH_SECOND;
        case HC_D_FACE_BOTH:
            return HC_REL_INVERTS_THROUGH_PAIR;
        case HC_D_FACE_NONE:
        default:
            break;
    }

    switch (family) {
        case HC_HARMONIC_FAMILY_A:
            return HC_REL_ADJACENTLY_ARTICULATES;
        case HC_HARMONIC_FAMILY_B:
            return HC_REL_MIRRORS_COMPLEMENT;
        case HC_HARMONIC_FAMILY_C:
            return HC_REL_CROSSES_KNOWING_LIMIT;
        default:
            return HC_REL_HARMONIC_NONE;
    }
}

const char* hc_harmonic_family_name(uint8_t family) {
    switch (family) {
        case HC_HARMONIC_FAMILY_A:
            return "A";
        case HC_HARMONIC_FAMILY_B:
            return "B";
        case HC_HARMONIC_FAMILY_C:
            return "C";
        case HC_HARMONIC_FAMILY_D:
            return "D";
        case HC_HARMONIC_FAMILY_NONE:
        default:
            return "NONE";
    }
}

const char* hc_harmonic_register_name(uint8_t harmonic_register) {
    switch (harmonic_register) {
        case HC_HARMONIC_REGISTER_BEING:
            return "Being";
        case HC_HARMONIC_REGISTER_BECOMING:
            return "Becoming";
        case HC_HARMONIC_REGISTER_KNOWING_UNKNOWING:
            return "KnowingUnknowing";
        case HC_HARMONIC_REGISTER_INVERSION:
            return "Inversion";
        case HC_HARMONIC_REGISTER_NONE:
        default:
            return "None";
    }
}

const char* hc_harmonic_d_face_name(uint8_t face) {
    switch (face) {
        case HC_D_FACE_LEFT:
            return "D_LEFT";
        case HC_D_FACE_RIGHT:
            return "D_RIGHT";
        case HC_D_FACE_BOTH:
            return "D_BOTH";
        case HC_D_FACE_NONE:
        default:
            return "NONE";
    }
}

const char* hc_harmonic_relation_type_name(uint8_t relation_type) {
    switch (relation_type) {
        case HC_REL_ADJACENTLY_ARTICULATES:
            return "ADJACENTLY_ARTICULATES";
        case HC_REL_MIRRORS_COMPLEMENT:
            return "MIRRORS_COMPLEMENT";
        case HC_REL_CROSSES_KNOWING_LIMIT:
            return "CROSSES_KNOWING_LIMIT";
        case HC_REL_INVERTS_THROUGH_FIRST:
            return "INVERTS_THROUGH_FIRST";
        case HC_REL_INVERTS_THROUGH_SECOND:
            return "INVERTS_THROUGH_SECOND";
        case HC_REL_INVERTS_THROUGH_PAIR:
            return "INVERTS_THROUGH_PAIR";
        case HC_REL_HARMONIC_NONE:
        default:
            return "NONE";
    }
}

static HC_BedrockRef make_bedrock_ref(
    const Holographic_Coordinate* target,
    uint8_t index,
    uint8_t ql_position,
    uint8_t bedrock_role,
    uint8_t relation_role,
    uint8_t interval_role,
    uint8_t ratio_role,
    uint8_t pitch_class
) {
    HC_BedrockRef ref;
    ref.target = target;
    ref.index = index;
    ref.ql_position = ql_position;
    ref.bedrock_role = bedrock_role;
    ref.relation_role = relation_role;
    ref.interval_role = interval_role;
    ref.ratio_role = ratio_role;
    ref.pitch_class = pitch_class;
    return ref;
}

int hc_bedrock_web7_fill(HC_BedrockWeb7* out) {
    if (!out) return -1;

    memset(out, 0, sizeof(*out));
    out->hash = make_bedrock_ref(
        &Psychoid_Hash,
        0u,
        Psychoid_Hash.ql_position,
        HC_BEDROCK_HASH_OPERATOR,
        HC_REL_INVERSION_SPANDA,
        HC_INTERVAL_SEMITONE,
        HC_RATIO_NONE,
        0u
    );

    for (uint8_t q = 0u; q < HC_BEDROCK_POSITION_COUNT; q++) {
        const Holographic_Coordinate* psychoid = HC_RAW_PSYCHOIDS[q];
        uint8_t successor_q = (uint8_t)((q + 1u) % HC_BEDROCK_POSITION_COUNT);
        uint8_t successor_role = (q == 5u) ? HC_REL_MOBIUS_RETURN : HC_REL_EPOGDOON_TICK;
        uint8_t successor_interval = (q == 5u) ? HC_INTERVAL_OCTAVE : HC_INTERVAL_WHOLE_TONE;
        uint8_t successor_ratio = (q == 5u) ? HC_RATIO_OCTAVE : HC_RATIO_EPOGDOON;

        out->psychoid[q] = make_bedrock_ref(
            psychoid,
            q,
            psychoid->ql_position,
            HC_BEDROCK_PSYCHOID_NUMBER,
            HC_REL_POSITION_IDENTITY,
            HC_INTERVAL_NONE,
            HC_RATIO_UNISON,
            hc_bimba_pitch_class(q)
        );
        out->successor[q] = make_bedrock_ref(
            HC_RAW_PSYCHOIDS[successor_q],
            q,
            successor_q,
            HC_BEDROCK_PSYCHOID_NUMBER,
            successor_role,
            successor_interval,
            successor_ratio,
            (successor_role == HC_REL_MOBIUS_RETURN)
                ? hc_pratibimba_pitch_class(successor_q)
                : hc_bimba_pitch_class(successor_q)
        );
        out->inversion[q] = make_bedrock_ref(
            psychoid,
            q,
            psychoid->ql_position,
            HC_BEDROCK_INVERTED_PSYCHOID,
            HC_REL_INVERSION_SPANDA,
            HC_INTERVAL_SEMITONE,
            HC_RATIO_NONE,
            hc_pratibimba_pitch_class(q)
        );
    }

    return 0;
}

static Holographic_Coordinate* arena_family_target(
    const Coordinate_Arena* arena,
    Coordinate_Family family,
    uint8_t ql_position
) {
    if (!arena || !arena->slots || arena->count < 42u || ql_position >= HC_WEB_HELIX_SIZE) {
        return NULL;
    }
    if (family == FAMILY_NONE) {
        return &arena->slots[ql_position];
    }
    if (family > FAMILY_M) {
        return NULL;
    }
    return &arena->slots[6u + ((uint32_t)family * HC_WEB_HELIX_SIZE) + ql_position];
}

static Holographic_Coordinate* tag_for_helix(
    Holographic_Coordinate* target,
    uint8_t helix
) {
    if (!target) return NULL;
    return (helix == HC_HELIX_PRATIBIMBA) ? SET_INVERTED(target) : target;
}

static uint8_t position_relation_role(uint8_t source_q, uint8_t target_q, uint8_t helix) {
    if (target_q == source_q && helix == HC_HELIX_BIMBA) {
        return HC_REL_POSITION_IDENTITY;
    }
    if (target_q == source_q && helix == HC_HELIX_PRATIBIMBA) {
        return HC_REL_INVERSION_SPANDA;
    }
    if (source_q == 5u && target_q == 0u && helix == HC_HELIX_PRATIBIMBA) {
        return HC_REL_MOBIUS_RETURN;
    }
    if (target_q == hc_mirror_position(source_q) && helix == HC_HELIX_BIMBA) {
        return HC_REL_MIRROR_XY5;
    }
    if (target_q == (uint8_t)((source_q + 1u) % HC_WEB_HELIX_SIZE) && helix == HC_HELIX_BIMBA) {
        return HC_REL_EPOGDOON_TICK;
    }
    return HC_REL_POSITION_PROJECT;
}

static uint8_t relation_interval_role(uint8_t relation_role, uint8_t source_q) {
    switch (relation_role) {
        case HC_REL_INVERSION_SPANDA:
            return HC_INTERVAL_SEMITONE;
        case HC_REL_EPOGDOON_TICK:
            return HC_INTERVAL_WHOLE_TONE;
        case HC_REL_MIRROR_XY5:
            return hc_mirror_interval_role(source_q);
        case HC_REL_MOBIUS_RETURN:
            return HC_INTERVAL_OCTAVE;
        default:
            return HC_INTERVAL_NONE;
    }
}

static uint8_t relation_ratio_role(uint8_t relation_role, uint8_t source_q) {
    switch (relation_role) {
        case HC_REL_POSITION_IDENTITY:
        case HC_REL_FAMILY_LINK:
        case HC_REL_LENS_ANCHOR:
            return HC_RATIO_UNISON;
        case HC_REL_EPOGDOON_TICK:
            return HC_RATIO_EPOGDOON;
        case HC_REL_MIRROR_XY5:
            return hc_mirror_ratio_role(source_q);
        case HC_REL_MOBIUS_RETURN:
            return HC_RATIO_OCTAVE;
        default:
            return HC_RATIO_NONE;
    }
}

static HC_PointerRef make_ref(
    Holographic_Coordinate* target,
    uint8_t ring,
    uint8_t index,
    uint8_t ql_position,
    uint8_t helix,
    uint8_t relation_role,
    uint8_t interval_role,
    uint8_t ratio_role,
    uint8_t pitch_class
) {
    HC_PointerRef ref;
    ref.target = target;
    ref.ring = ring;
    ref.index = index;
    ref.ql_position = ql_position;
    ref.helix = helix;
    ref.relation_role = relation_role;
    ref.interval_role = interval_role;
    ref.ratio_role = ratio_role;
    ref.pitch_class = pitch_class;
    return ref;
}

int hc_pointer_web36_fill(
    const Coordinate_Arena* arena,
    const Holographic_Coordinate* source,
    HC_PointerWeb36* out
) {
    if (!arena || !source || !out || source->ql_position >= HC_WEB_HELIX_SIZE) {
        return -1;
    }

    memset(out, 0, sizeof(*out));
    uint8_t source_q = source->ql_position;
    Coordinate_Family source_family = (Coordinate_Family)source->family;
    if (source_family > FAMILY_M && source_family != FAMILY_NONE) {
        source_family = FAMILY_NONE;
    }

    for (uint8_t q = 0u; q < HC_WEB_HELIX_SIZE; q++) {
        uint8_t prime_index = (uint8_t)(q + HC_WEB_HELIX_SIZE);

        Coordinate_Family family = HC_FAMILIES[q];
        Holographic_Coordinate* family_target = arena_family_target(arena, family, source_q);
        out->family[q] = make_ref(
            family_target,
            HC_POINTER_RING_FAMILY,
            q,
            source_q,
            HC_HELIX_BIMBA,
            HC_REL_FAMILY_LINK,
            HC_INTERVAL_NONE,
            HC_RATIO_UNISON,
            hc_bimba_pitch_class(source_q)
        );
        out->family[prime_index] = make_ref(
            tag_for_helix(family_target, HC_HELIX_PRATIBIMBA),
            HC_POINTER_RING_FAMILY,
            prime_index,
            source_q,
            HC_HELIX_PRATIBIMBA,
            HC_REL_INVERSION_SPANDA,
            HC_INTERVAL_SEMITONE,
            HC_RATIO_NONE,
            hc_pratibimba_pitch_class(source_q)
        );

        Holographic_Coordinate* position_target = arena_family_target(arena, source_family, q);
        uint8_t bimba_role = position_relation_role(source_q, q, HC_HELIX_BIMBA);
        uint8_t prime_role = position_relation_role(source_q, q, HC_HELIX_PRATIBIMBA);
        out->position[q] = make_ref(
            position_target,
            HC_POINTER_RING_POSITION,
            q,
            q,
            HC_HELIX_BIMBA,
            bimba_role,
            relation_interval_role(bimba_role, source_q),
            relation_ratio_role(bimba_role, source_q),
            hc_bimba_pitch_class(q)
        );
        out->position[prime_index] = make_ref(
            tag_for_helix(position_target, HC_HELIX_PRATIBIMBA),
            HC_POINTER_RING_POSITION,
            prime_index,
            q,
            HC_HELIX_PRATIBIMBA,
            prime_role,
            relation_interval_role(prime_role, source_q),
            relation_ratio_role(prime_role, source_q),
            hc_pratibimba_pitch_class(q)
        );

        Holographic_Coordinate* lens_target = arena_family_target(arena, FAMILY_L, q);
        out->lens[q] = make_ref(
            lens_target,
            HC_POINTER_RING_LENS,
            q,
            q,
            HC_HELIX_BIMBA,
            HC_REL_LENS_ANCHOR,
            HC_INTERVAL_NONE,
            HC_RATIO_UNISON,
            HC_LENS_ANCHOR_PC[q]
        );
        out->lens[prime_index] = make_ref(
            tag_for_helix(lens_target, HC_HELIX_PRATIBIMBA),
            HC_POINTER_RING_LENS,
            prime_index,
            q,
            HC_HELIX_PRATIBIMBA,
            HC_REL_INVERSION_SPANDA,
            HC_INTERVAL_SEMITONE,
            HC_RATIO_NONE,
            HC_LENS_ANCHOR_PC[prime_index]
        );
    }

    return 0;
}

int hc_context_frame_web7_fill(HC_ContextFrameWeb7* out) {
    if (!out) return -1;

    memset(out, 0, sizeof(*out));
    for (uint8_t i = 0u; i < CF_COUNT; i++) {
        uint8_t q = HC_CF_QL_POSITION[i];
        uint8_t helix = HC_CF_HELIX[i];
        out->frame[i].target = cf_get((CF_Id)i);
        out->frame[i].cf_index = i;
        out->frame[i].notation = HC_CF_NOTATION[i];
        out->frame[i].diatonic_degree = (uint8_t)(i + 1u);
        out->frame[i].mode_anchor = i;
        out->frame[i].ql_position = q;
        out->frame[i].helix = helix;
        out->frame[i].relation_role = HC_REL_CONTEXT_FRAME;
        out->frame[i].pitch_class = (helix == HC_HELIX_BIMBA)
            ? hc_bimba_pitch_class(q)
            : hc_pratibimba_pitch_class(q);
    }

    return 0;
}
