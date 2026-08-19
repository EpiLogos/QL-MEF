#include "ql/holographic.h"

#include <string.h>

static const QL_Holographic_Coordinate QL_PSYCHOID_BIMBA[QL_POSITION_COUNT] = {
    { .ql_position = 0u, .family = QL_FAMILY_NONE, .flags = QL_BIMBA_FLAGS, .weave_state = 0.0f },
    { .ql_position = 1u, .family = QL_FAMILY_NONE, .flags = QL_BIMBA_FLAGS, .weave_state = 1.0f },
    { .ql_position = 2u, .family = QL_FAMILY_NONE, .flags = QL_BIMBA_FLAGS, .weave_state = 2.0f },
    { .ql_position = 3u, .family = QL_FAMILY_NONE, .flags = QL_BIMBA_FLAGS, .weave_state = 3.0f },
    { .ql_position = 4u, .family = QL_FAMILY_NONE, .flags = QL_BIMBA_FLAGS, .weave_state = 4.0f },
    { .ql_position = 5u, .family = QL_FAMILY_NONE, .flags = QL_BIMBA_FLAGS, .weave_state = 5.0f }
};

static const QL_Holographic_Coordinate QL_HASH_BIMBA = {
    .ql_position = QL_HASH_POSITION,
    .family = QL_FAMILY_NONE,
    .flags = QL_BIMBA_FLAGS,
    .weave_state = 0.0f
};

static int ql_family_index(QL_Coordinate_Family family) {
    return family <= QL_FAMILY_M ? (int)family : -1;
}

static QL_Topology_Mode ql_default_topology(QL_Coordinate_Family family, uint8_t position) {
    if (family == QL_FAMILY_P && position == 4u) return QL_TOPO_LEMNISCATE;
    if (family == QL_FAMILY_C && (position == 0u || position == 5u)) return QL_TOPO_ZERO_SPHERE;
    return QL_TOPO_TORUS;
}

const QL_Bimba* ql_default_psychoid_bimba(uint8_t position) {
    return position < QL_POSITION_COUNT ? &QL_PSYCHOID_BIMBA[position] : NULL;
}

const QL_Bimba* ql_default_hash_bimba(void) {
    return &QL_HASH_BIMBA;
}

int ql_coordinate_init(
    QL_Holographic_Coordinate* coordinate,
    QL_Coordinate_Family family,
    uint8_t position
) {
    if (!coordinate || position >= QL_POSITION_COUNT ||
        (family != QL_FAMILY_NONE && ql_family_index(family) < 0)) {
        return -1;
    }
    memset(coordinate, 0, sizeof(*coordinate));
    coordinate->ql_position = position;
    coordinate->family = (uint8_t)family;
    coordinate->flags = QL_STATUS_CANONICAL;
    coordinate->weave_state = (float)position;
    ql_coordinate_set_topology(coordinate, ql_default_topology(family, position));
    return 0;
}

int ql_coordinate_materialize(
    const QL_Bimba* source,
    QL_Pratibimba* manifestation
) {
    if (!source || !manifestation) return -1;
    *manifestation = *source;
    manifestation->flags = (uint8_t)(manifestation->flags & (uint8_t)~QL_FLAG_BIMBA);
    manifestation->semantic_embedding = (float*)(uintptr_t)source;
    return 0;
}

const QL_Bimba* ql_coordinate_source(const QL_Holographic_Coordinate* coordinate) {
    if (!coordinate) return NULL;
    if (ql_coordinate_is_bimba(coordinate)) return coordinate;
    if (!coordinate->semantic_embedding) return NULL;
    return (const QL_Bimba*)(const void*)coordinate->semantic_embedding;
}

const QL_Bimba* ql_coordinate_bedrock(const QL_Holographic_Coordinate* coordinate) {
    if (!coordinate || coordinate->family == QL_FAMILY_NONE) return NULL;
    return ql_coordinate_source(coordinate);
}

bool ql_coordinate_is_bimba(const QL_Holographic_Coordinate* coordinate) {
    return coordinate && (coordinate->flags & QL_FLAG_BIMBA) != 0u;
}

QL_Topology_Mode ql_coordinate_topology(const QL_Holographic_Coordinate* coordinate) {
    if (!coordinate) return QL_TOPO_TORUS;
    return (QL_Topology_Mode)(coordinate->flags & QL_TOPO_MODE_MASK);
}

void ql_coordinate_set_topology(QL_Holographic_Coordinate* coordinate, QL_Topology_Mode mode) {
    if (!coordinate) return;
    coordinate->flags = (uint8_t)((coordinate->flags & (uint8_t)~QL_TOPO_MODE_MASK) | (uint8_t)mode);
}

bool ql_weave_is_identification_edge(float weave_state) {
    return weave_state == 0.0f || weave_state == 0.5f ||
           weave_state == 5.0f || weave_state == 5.5f;
}

bool ql_coordinate_has_nesting_access(const QL_Holographic_Coordinate* coordinate) {
    return coordinate && (coordinate->ql_position == 4u || ql_weave_is_identification_edge(coordinate->weave_state));
}

uint8_t ql_weave_parent(float weave_state) {
    return (uint8_t)weave_state;
}

uint8_t ql_weave_child(float weave_state) {
    uint8_t parent = (uint8_t)weave_state;
    return (uint8_t)((weave_state - (float)parent) * 10.0f + 0.5f);
}

QL_Holographic_Coordinate* ql_relation_tag(
    const QL_Holographic_Coordinate* source,
    QL_Holographic_Coordinate* target,
    uint64_t extra_flags
) {
    if (!source || !target) return NULL;
    uintptr_t raw = (uintptr_t)target;
    uintptr_t tagged = raw & (uintptr_t)QL_TAG_ADDRESS_MASK;
    tagged |= ql_coordinate_has_nesting_access(source)
        ? (uintptr_t)QL_TAG_NESTING
        : (uintptr_t)QL_TAG_BRANCHING;
    tagged |= (uintptr_t)(extra_flags & (QL_TAG_INVERTED | QL_TAG_EXECUTING));
    tagged |= ((uintptr_t)target->family & 0x0Fu) << QL_TAG_FAMILY_SHIFT;
    tagged |= ((uintptr_t)target->ql_position & 0xFFu) << QL_TAG_POSITION_SHIFT;
    return (QL_Holographic_Coordinate*)tagged;
}

QL_Holographic_Coordinate* ql_relation_target(QL_Holographic_Coordinate* tagged) {
    return (QL_Holographic_Coordinate*)((uintptr_t)tagged & (uintptr_t)QL_TAG_ADDRESS_MASK);
}

const QL_Holographic_Coordinate* ql_relation_target_const(const QL_Holographic_Coordinate* tagged) {
    return (const QL_Holographic_Coordinate*)((uintptr_t)tagged & (uintptr_t)QL_TAG_ADDRESS_MASK);
}

uint64_t ql_relation_flags(const QL_Holographic_Coordinate* tagged) {
    return (uint64_t)((uintptr_t)tagged & (uintptr_t)(QL_TAG_INVERTED | QL_TAG_NESTING | QL_TAG_BRANCHING | QL_TAG_EXECUTING));
}

QL_Coordinate_Family ql_relation_family(const QL_Holographic_Coordinate* tagged) {
    return (QL_Coordinate_Family)(((uintptr_t)tagged >> QL_TAG_FAMILY_SHIFT) & 0x0Fu);
}

uint8_t ql_relation_position(const QL_Holographic_Coordinate* tagged) {
    return (uint8_t)(((uintptr_t)tagged >> QL_TAG_POSITION_SHIFT) & 0xFFu);
}

QL_Holographic_Coordinate* ql_holographic_field_get(
    QL_Holographic_Field* field,
    QL_Coordinate_Family family,
    uint8_t position
) {
    int index = ql_family_index(family);
    if (!field || index < 0 || position >= QL_POSITION_COUNT) return NULL;
    return &field->coordinates[index][position];
}

const QL_Holographic_Coordinate* ql_holographic_field_get_const(
    const QL_Holographic_Field* field,
    QL_Coordinate_Family family,
    uint8_t position
) {
    int index = ql_family_index(family);
    if (!field || index < 0 || position >= QL_POSITION_COUNT) return NULL;
    return &field->coordinates[index][position];
}

int ql_holographic_field_init(QL_Holographic_Field* field) {
    if (!field) return -1;
    memset(field, 0, sizeof(*field));

    for (uint8_t family = 0u; family < QL_COORDINATE_FAMILY_COUNT; family++) {
        for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
            QL_Holographic_Coordinate* coordinate = &field->coordinates[family][position];
            if (ql_coordinate_init(coordinate, (QL_Coordinate_Family)family, position) != 0) return -1;
            coordinate->weave_state = (float)position + (float)family * 0.1f;
            coordinate->semantic_embedding = (float*)(uintptr_t)ql_default_psychoid_bimba(position);
            ql_coordinate_set_topology(coordinate, ql_default_topology((QL_Coordinate_Family)family, position));
        }
    }

    for (uint8_t family = 0u; family < QL_COORDINATE_FAMILY_COUNT; family++) {
        for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
            QL_Holographic_Coordinate* coordinate = &field->coordinates[family][position];
            coordinate->c = ql_relation_tag(coordinate, &field->coordinates[QL_FAMILY_C][position], 0u);
            coordinate->p = ql_relation_tag(coordinate, &field->coordinates[QL_FAMILY_P][position], 0u);
            coordinate->l = ql_relation_tag(coordinate, &field->coordinates[QL_FAMILY_L][position], 0u);
            coordinate->s = ql_relation_tag(coordinate, &field->coordinates[QL_FAMILY_S][position], 0u);
            coordinate->t = ql_relation_tag(coordinate, &field->coordinates[QL_FAMILY_T][position], 0u);
            coordinate->m = ql_relation_tag(coordinate, &field->coordinates[QL_FAMILY_M][position], 0u);

            QL_Holographic_Coordinate* cf_target =
                position == 4u ? coordinate :
                position == 3u ? &field->coordinates[family][4u] :
                (QL_Holographic_Coordinate*)(uintptr_t)ql_default_psychoid_bimba(4u);
            coordinate->cf = ql_relation_tag(coordinate, cf_target, 0u);
            coordinate->cs = ql_relation_tag(coordinate, &field->coordinates[family][5u], 0u);
            /* cpf/ct/cp/cfp remain intentionally unresolved here. They are
             * reflective link slots, not a second Context Frame authority. */
        }
    }
    return 0;
}

void ql_coordinate_toggle_cover(QL_Holographic_Coordinate* coordinate) {
    if (!coordinate) return;
    coordinate->inversion_state = (uint8_t)(1u - (coordinate->inversion_state ? 1u : 0u));
    if (coordinate->family == QL_FAMILY_P) {
        if (coordinate->inversion_state) {
            ql_coordinate_set_topology(coordinate, QL_TOPO_KLEIN);
        } else {
            ql_coordinate_set_topology(coordinate,
                coordinate->ql_position == 4u ? QL_TOPO_LEMNISCATE : QL_TOPO_TORUS);
        }
    }
}

int ql_coordinate_conjugate(
    const QL_Holographic_Coordinate* source,
    QL_Holographic_Coordinate* conjugate
) {
    if (!source || !conjugate || source->ql_position >= QL_POSITION_COUNT) return -1;
    *conjugate = *source;
    conjugate->ql_position = ql_position_invert(source->ql_position);
    ql_coordinate_toggle_cover(conjugate);
    return 0;
}

void ql_coordinate_execute(QL_Holographic_Coordinate* coordinate, void* context_state) {
    if (coordinate && coordinate->invoke_process) {
        coordinate->invoke_process(coordinate, context_state);
    }
}
