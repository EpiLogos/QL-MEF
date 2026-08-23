#include "ql/holographic.h"

#include <stdio.h>
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

static bool ql_address_family_valid(QL_Coordinate_Family family) {
    return family == QL_FAMILY_NONE || ql_family_index(family) >= 0;
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
    if (!coordinate || position >= QL_POSITION_COUNT || !ql_address_family_valid(family)) {
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
    if (!coordinate) return NULL;
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

QL_Coordinate_Label ql_coordinate_label(
    QL_Coordinate_Family family,
    uint8_t position,
    QL_Coordinate_Face face
) {
    QL_Coordinate_Label label = {
        .family = (uint8_t)family,
        .position = position,
        .face = (uint8_t)face
    };
    if (!ql_coordinate_label_valid(label)) {
        label.family = QL_INVALID_U8;
        label.position = QL_INVALID_U8;
        label.face = (uint8_t)QL_COORD_FACE_DIRECT;
    }
    return label;
}

bool ql_coordinate_label_valid(QL_Coordinate_Label label) {
    return ql_address_family_valid((QL_Coordinate_Family)label.family) &&
           label.position < QL_POSITION_COUNT &&
           label.face < QL_FACE_COUNT;
}

QL_Coordinate_Label ql_coordinate_label_other_face(QL_Coordinate_Label label) {
    if (!ql_coordinate_label_valid(label)) return label;
    label.face = (uint8_t)(label.face == (uint8_t)QL_COORD_FACE_DIRECT
        ? QL_COORD_FACE_PRIME
        : QL_COORD_FACE_DIRECT);
    return label;
}

QL_Coordinate_Face ql_coordinate_face(const QL_Holographic_Coordinate* coordinate) {
    return coordinate && coordinate->inversion_state
        ? QL_COORD_FACE_PRIME
        : QL_COORD_FACE_DIRECT;
}

int ql_coordinate_set_face(QL_Holographic_Coordinate* coordinate, QL_Coordinate_Face face) {
    if (!coordinate || face > QL_COORD_FACE_PRIME) return -1;
    coordinate->inversion_state = (uint8_t)face;

    /* Only P/P' has a topology change asserted by the current coordinate account.
     * L/L' is a refractive face distinction and is not assigned a new topology here. */
    if (coordinate->family == QL_FAMILY_P) {
        if (face == QL_COORD_FACE_PRIME) {
            ql_coordinate_set_topology(coordinate, QL_TOPO_KLEIN);
        } else {
            ql_coordinate_set_topology(coordinate,
                coordinate->ql_position == 4u ? QL_TOPO_LEMNISCATE : QL_TOPO_TORUS);
        }
    }
    return 0;
}

QL_Kernel_Address ql_kernel_hash_address(void) {
    return (QL_Kernel_Address){
        .family = (uint8_t)QL_FAMILY_NONE,
        .position = QL_HASH_POSITION,
        .face = (uint8_t)QL_COORD_FACE_DIRECT
    };
}

QL_Kernel_Address ql_kernel_position_address(uint8_t position, QL_Coordinate_Face face) {
    return ql_kernel_family_address(QL_FAMILY_NONE, position, face);
}

QL_Kernel_Address ql_kernel_family_address(
    QL_Coordinate_Family family,
    uint8_t position,
    QL_Coordinate_Face face
) {
    return ql_coordinate_label(family, position, face);
}

bool ql_kernel_address_is_hash(QL_Kernel_Address address) {
    return address.family == (uint8_t)QL_FAMILY_NONE &&
           address.position == QL_HASH_POSITION &&
           address.face == (uint8_t)QL_COORD_FACE_DIRECT;
}

bool ql_kernel_address_valid(QL_Kernel_Address address) {
    return ql_kernel_address_is_hash(address) || ql_coordinate_label_valid(address);
}

bool ql_kernel_address_is_bedrock(QL_Kernel_Address address) {
    return ql_kernel_address_is_hash(address) ||
           (ql_coordinate_label_valid(address) && address.family == (uint8_t)QL_FAMILY_NONE);
}

const char* ql_kernel_family_code(QL_Coordinate_Family family) {
    switch (family) {
        case QL_FAMILY_C: return "C";
        case QL_FAMILY_P: return "P";
        case QL_FAMILY_L: return "L";
        case QL_FAMILY_S: return "S";
        case QL_FAMILY_T: return "T";
        case QL_FAMILY_M: return "M";
        case QL_FAMILY_NONE: return "NONE";
        default: return NULL;
    }
}

const char* ql_kernel_face_code(QL_Coordinate_Face face) {
    switch (face) {
        case QL_COORD_FACE_DIRECT: return "direct";
        case QL_COORD_FACE_PRIME: return "prime";
        default: return NULL;
    }
}

int ql_kernel_address_format(QL_Kernel_Address address, char* out, size_t out_size) {
    if (!out || out_size == 0u || !ql_kernel_address_valid(address)) return -1;

    int written;
    if (ql_kernel_address_is_hash(address)) {
        written = snprintf(out, out_size, "#");
    } else if (address.family == (uint8_t)QL_FAMILY_NONE) {
        written = snprintf(out, out_size, "#%u%s", (unsigned)address.position,
            address.face == (uint8_t)QL_COORD_FACE_PRIME ? "'" : "");
    } else {
        const char* family = ql_kernel_family_code((QL_Coordinate_Family)address.family);
        if (!family) return -1;
        written = snprintf(out, out_size, "%s%u%s", family, (unsigned)address.position,
            address.face == (uint8_t)QL_COORD_FACE_PRIME ? "'" : "");
    }
    return written >= 0 && (size_t)written < out_size ? 0 : -1;
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

const QL_Holographic_Coordinate* ql_holographic_field_resolve(
    const QL_Holographic_Field* field,
    QL_Kernel_Address address
) {
    if (!ql_kernel_address_valid(address)) return NULL;
    if (ql_kernel_address_is_hash(address)) return ql_default_hash_bimba();
    if (address.family == (uint8_t)QL_FAMILY_NONE) {
        return ql_default_psychoid_bimba(address.position);
    }
    return ql_holographic_field_get_const(
        field,
        (QL_Coordinate_Family)address.family,
        address.position
    );
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
            /* cpf/ct/cp/cfp remain source-layout witness slots. Their stable
             * semantic relations are exposed through the native kernel field. */
        }
    }
    return 0;
}

void ql_coordinate_execute(QL_Holographic_Coordinate* coordinate, void* context_state) {
    if (coordinate && coordinate->invoke_process) {
        coordinate->invoke_process(coordinate, context_state);
    }
}
