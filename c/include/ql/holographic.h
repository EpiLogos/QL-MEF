#ifndef QL_MEF_HOLOGRAPHIC_H
#define QL_MEF_HOLOGRAPHIC_H

#include "ql/primitive.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define QL_COORDINATE_FAMILY_COUNT 6u
#define QL_KERNEL_ADDRESS_FAMILY_COUNT 7u
#define QL_HASH_POSITION QL_INVALID_U8

typedef enum {
    QL_FAMILY_C    = 0,
    QL_FAMILY_P    = 1,
    QL_FAMILY_L    = 2,
    QL_FAMILY_S    = 3,
    QL_FAMILY_T    = 4,
    QL_FAMILY_M    = 5,
    QL_FAMILY_NONE = 7
} QL_Coordinate_Family;

/* Face is a structural address dimension over the same family+position
 * substrate. It does not perform positional complement. The Bimba/Pratibimba
 * names are retained as source-compatible aliases for direct/prime. */
typedef enum {
    QL_COORD_FACE_DIRECT      = 0,
    QL_COORD_FACE_PRIME       = 1,
    QL_COORD_FACE_BIMBA       = QL_COORD_FACE_DIRECT,
    QL_COORD_FACE_PRATIBIMBA  = QL_COORD_FACE_PRIME
} QL_Coordinate_Face;

typedef struct {
    uint8_t family;
    uint8_t position;
    uint8_t face;
} QL_Coordinate_Label;

/* Native semantic address. The historical coordinate object remains a parity
 * substrate; direct and prime addresses may resolve to the same substrate
 * object while remaining distinct semantic addresses. */
typedef QL_Coordinate_Label QL_Kernel_Address;

typedef enum {
    QL_TOPO_TORUS       = 0x00u,
    QL_TOPO_LEMNISCATE  = 0x40u,
    QL_TOPO_KLEIN       = 0x80u,
    QL_TOPO_ZERO_SPHERE = 0xC0u
} QL_Topology_Mode;

#define QL_STATUS_CANONICAL   0x01u
#define QL_STATUS_PROVISIONAL 0x02u
#define QL_FLAG_BIMBA         0x20u
#define QL_TOPO_MODE_MASK     0xC0u
#define QL_BIMBA_FLAGS        (QL_STATUS_CANONICAL | QL_FLAG_BIMBA)

#define QL_TAG_ADDRESS_MASK UINT64_C(0x0000FFFFFFFFFFFF)
#define QL_TAG_INVERTED     UINT64_C(0x8000000000000000)
#define QL_TAG_NESTING      UINT64_C(0x4000000000000000)
#define QL_TAG_BRANCHING    UINT64_C(0x2000000000000000)
#define QL_TAG_EXECUTING    UINT64_C(0x1000000000000000)
#define QL_TAG_FAMILY_MASK  UINT64_C(0x0F00000000000000)
#define QL_TAG_POSITION_MASK UINT64_C(0x00FF000000000000)
#define QL_TAG_FAMILY_SHIFT 56u
#define QL_TAG_POSITION_SHIFT 48u

typedef struct QL_Holographic_Coordinate QL_Holographic_Coordinate;

typedef void (*QL_Context_Execution_Operator)(
    QL_Holographic_Coordinate* self,
    void* context_state
);

struct QL_Holographic_Coordinate {
    uint8_t ql_position;
    uint8_t family;
    uint8_t inversion_state; /* historical storage; native reading is coordinate face */
    uint8_t flags;
    float weave_state;

    float* semantic_embedding;

    QL_Holographic_Coordinate* c;
    QL_Holographic_Coordinate* p;
    QL_Holographic_Coordinate* l;
    QL_Holographic_Coordinate* s;
    QL_Holographic_Coordinate* t;
    QL_Holographic_Coordinate* m;

    QL_Holographic_Coordinate* cpf;
    QL_Holographic_Coordinate* ct;
    QL_Holographic_Coordinate* cp;
    QL_Holographic_Coordinate* cf;
    QL_Holographic_Coordinate* cfp;
    QL_Holographic_Coordinate* cs;

    QL_Context_Execution_Operator invoke_process;

    union {
        char* meaning_bin;
        void* process_state;
        uint64_t instance_id;
        float* vector_anchor;
    } payload;
};

_Static_assert(sizeof(QL_Holographic_Coordinate) == 128,
    "QL_Holographic_Coordinate must remain the historical 128-byte seed layout");
_Static_assert(offsetof(QL_Holographic_Coordinate, semantic_embedding) == 8,
    "semantic anchor offset must match the historical seed");
_Static_assert(offsetof(QL_Holographic_Coordinate, c) == 16,
    "base links must begin at byte 16");
_Static_assert(offsetof(QL_Holographic_Coordinate, invoke_process) == 112,
    "execution hook must begin at byte 112");
_Static_assert(offsetof(QL_Holographic_Coordinate, payload) == 120,
    "payload must begin at byte 120");

typedef const QL_Holographic_Coordinate QL_Bimba;
typedef QL_Holographic_Coordinate QL_Pratibimba;

typedef struct {
    QL_Holographic_Coordinate coordinates[QL_COORDINATE_FAMILY_COUNT][QL_POSITION_COUNT];
} QL_Holographic_Field;

const QL_Bimba* ql_default_psychoid_bimba(uint8_t position);
const QL_Bimba* ql_default_hash_bimba(void);

int ql_coordinate_init(
    QL_Holographic_Coordinate* coordinate,
    QL_Coordinate_Family family,
    uint8_t position
);
int ql_coordinate_materialize(
    const QL_Bimba* source,
    QL_Pratibimba* manifestation
);
const QL_Bimba* ql_coordinate_source(const QL_Holographic_Coordinate* coordinate);
const QL_Bimba* ql_coordinate_bedrock(const QL_Holographic_Coordinate* coordinate);

bool ql_coordinate_is_bimba(const QL_Holographic_Coordinate* coordinate);
QL_Topology_Mode ql_coordinate_topology(const QL_Holographic_Coordinate* coordinate);
void ql_coordinate_set_topology(QL_Holographic_Coordinate* coordinate, QL_Topology_Mode mode);

QL_Coordinate_Label ql_coordinate_label(
    QL_Coordinate_Family family,
    uint8_t position,
    QL_Coordinate_Face face
);
bool ql_coordinate_label_valid(QL_Coordinate_Label label);
QL_Coordinate_Label ql_coordinate_label_other_face(QL_Coordinate_Label label);
QL_Coordinate_Face ql_coordinate_face(const QL_Holographic_Coordinate* coordinate);
int ql_coordinate_set_face(QL_Holographic_Coordinate* coordinate, QL_Coordinate_Face face);

/* Tap-root address constructors. `#` is NONE/invalid-position/direct; raw
 * #0..#5 are NONE family addresses. Family addresses occupy the same positions. */
QL_Kernel_Address ql_kernel_hash_address(void);
QL_Kernel_Address ql_kernel_position_address(uint8_t position, QL_Coordinate_Face face);
QL_Kernel_Address ql_kernel_family_address(
    QL_Coordinate_Family family,
    uint8_t position,
    QL_Coordinate_Face face
);
bool ql_kernel_address_valid(QL_Kernel_Address address);
bool ql_kernel_address_is_hash(QL_Kernel_Address address);
bool ql_kernel_address_is_bedrock(QL_Kernel_Address address);
const char* ql_kernel_family_code(QL_Coordinate_Family family);
const char* ql_kernel_face_code(QL_Coordinate_Face face);
int ql_kernel_address_format(QL_Kernel_Address address, char* out, size_t out_size);

bool ql_weave_is_identification_edge(float weave_state);
bool ql_coordinate_has_nesting_access(const QL_Holographic_Coordinate* coordinate);
uint8_t ql_weave_parent(float weave_state);
uint8_t ql_weave_child(float weave_state);

QL_Holographic_Coordinate* ql_relation_tag(
    const QL_Holographic_Coordinate* source,
    QL_Holographic_Coordinate* target,
    uint64_t extra_flags
);
QL_Holographic_Coordinate* ql_relation_target(QL_Holographic_Coordinate* tagged);
const QL_Holographic_Coordinate* ql_relation_target_const(const QL_Holographic_Coordinate* tagged);
uint64_t ql_relation_flags(const QL_Holographic_Coordinate* tagged);
QL_Coordinate_Family ql_relation_family(const QL_Holographic_Coordinate* tagged);
uint8_t ql_relation_position(const QL_Holographic_Coordinate* tagged);

int ql_holographic_field_init(QL_Holographic_Field* field);
QL_Holographic_Coordinate* ql_holographic_field_get(
    QL_Holographic_Field* field,
    QL_Coordinate_Family family,
    uint8_t position
);
const QL_Holographic_Coordinate* ql_holographic_field_get_const(
    const QL_Holographic_Field* field,
    QL_Coordinate_Family family,
    uint8_t position
);
const QL_Holographic_Coordinate* ql_holographic_field_resolve(
    const QL_Holographic_Field* field,
    QL_Kernel_Address address
);

void ql_coordinate_execute(QL_Holographic_Coordinate* coordinate, void* context_state);

#ifdef __cplusplus
}
#endif

#endif /* QL_MEF_HOLOGRAPHIC_H */
