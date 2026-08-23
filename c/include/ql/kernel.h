#ifndef QL_MEF_KERNEL_H
#define QL_MEF_KERNEL_H

#include "ql/holographic.h"
#include "ql/primitive.h"

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define QL_KERNEL_API_VERSION_MAJOR 0u
#define QL_KERNEL_API_VERSION_MINOR 1u
#define QL_KERNEL_API_VERSION_PATCH 0u
#define QL_HOLOGRAPHIC_KERNEL_CONTRACT_VERSION "1.0.0"
#define QL_KERNEL_RESONANCE_DIM QL_RESONANCE_COUNT
#define QL_KERNEL_TRITONE_SQUARES QL_TRITONE_SQUARE_COUNT
#define QL_KERNEL_CONTEXT_FRAME_COUNT 7u
#define QL_KERNEL_RELATION_COUNT 26u

typedef enum {
    QL_KERNEL_PHASE_DESCENT = 0,
    QL_KERNEL_PHASE_ASCENT  = 1
} QL_Kernel_Phase;

typedef enum {
    QL_KERNEL_ELEMENT_BIMBA_ENCODING        = 0,
    QL_KERNEL_ELEMENT_PRATIBIMBA_PREHENSION = 1,
    QL_KERNEL_ELEMENT_MOBIUS_DESCENT        = 2,
    QL_KERNEL_ELEMENT_SLASH_FLIP            = 3,
    QL_KERNEL_ELEMENT_PRATIBIMBA_AS_BIMBA   = 4,
    QL_KERNEL_ELEMENT_DOUBLED_PREHENSION    = 5,
    QL_KERNEL_ELEMENT_INVERSE_MOBIUS        = 6,
    QL_KERNEL_ELEMENT_ENRICHED_RETURN       = 7
} QL_Kernel_Element;

typedef enum {
    QL_KERNEL_INTERVAL_NONE          = 0,
    QL_KERNEL_INTERVAL_SEMITONE      = 1,
    QL_KERNEL_INTERVAL_WHOLE_TONE    = 2,
    QL_KERNEL_INTERVAL_TRITONE       = 6,
    QL_KERNEL_INTERVAL_TOTALITY_16_9 = 10,
    QL_KERNEL_INTERVAL_OCTAVE        = 12
} QL_Kernel_Interval_Role;

typedef enum {
    QL_KERNEL_RATIO_NONE      = 0,
    QL_KERNEL_RATIO_UNISON    = 1,
    QL_KERNEL_RATIO_EPOGDOON  = 2,
    QL_KERNEL_RATIO_FOURTH    = 3,
    QL_KERNEL_RATIO_FIFTH     = 4,
    QL_KERNEL_RATIO_TOTALITY  = 5,
    QL_KERNEL_RATIO_OCTAVE    = 6
} QL_Kernel_Ratio_Role;

/* Stable semantic relation identities. The D1/D2/D3 historical/software names
 * remain provenance/aliases on the Rust side; the shared kernel contract uses
 * the unambiguous relation meaning established by #39.
 *
 * VAK relations name the six historical reflective C' slots in ontology.h.
 * VAK_CF is deliberately distinct from CONTEXT_FRAME: VAK_CF is the historical
 * pointer slot, while CONTEXT_FRAME addresses the seven canonical MEF/CF cuts. */
typedef enum {
    QL_KERNEL_REL_POSITION_IDENTITY = 0,
    QL_KERNEL_REL_FAMILY_SAME_POSITION = 1,
    QL_KERNEL_REL_CROSS_SAME_POSITION = 2,
    QL_KERNEL_REL_PAIR_A = 3,
    QL_KERNEL_REL_PAIR_B = 4,
    QL_KERNEL_REL_PAIR_C = 5,
    QL_KERNEL_REL_CROSS_TRANSFORM = 6,
    QL_KERNEL_REL_CROSS_REQUIRE = 7,
    QL_KERNEL_REL_CROSS_COMPLETE = 8,
    QL_KERNEL_REL_CONJUGATE_INVARIANCE_A = 9,
    QL_KERNEL_REL_CONJUGATE_INVARIANCE_B = 10,
    QL_KERNEL_REL_CONJUGATE_INVARIANCE_C = 11,
    QL_KERNEL_REL_MIRROR_COMPLEMENT = 12,
    QL_KERNEL_REL_POSITION_SUCCESSOR = 13,
    QL_KERNEL_REL_MOBIUS_RETURN = 14,
    QL_KERNEL_REL_LENS_ANCHOR = 15,
    QL_KERNEL_REL_CONTEXT_FRAME = 16,
    QL_KERNEL_REL_VAK_CPF = 17,
    QL_KERNEL_REL_VAK_CT = 18,
    QL_KERNEL_REL_VAK_CP = 19,
    QL_KERNEL_REL_VAK_CF = 20,
    QL_KERNEL_REL_VAK_CFP = 21,
    QL_KERNEL_REL_VAK_CS = 22,
    QL_KERNEL_REL_NESTING = 23,
    QL_KERNEL_REL_BRANCHING = 24,
    QL_KERNEL_REL_SOURCE_PROVENANCE = 25
} QL_Kernel_Relation_Id;

typedef struct {
    QL_Kernel_Relation_Id relation;
    QL_Kernel_Address source;
    QL_Kernel_Address target;
    uint8_t interval_role;
    uint8_t ratio_role;
    uint8_t pitch_class;
} QL_Kernel_Relation_Ref;

typedef struct {
    float w;
    float x;
    float y;
    float z;
} QL_Quaternion;

typedef struct {
    QL_Quaternion q_b;
    QL_Quaternion q_p;
} QL_Kernel_Bioquaternion;

typedef struct {
    float values[QL_KERNEL_RESONANCE_DIM];
} QL_Kernel_Resonance_Vector;

typedef struct {
    float bimba_pratibimba_energy;
    float lens_energy;
    float r_energy;
    float total_energy;
} QL_Kernel_Energy;

/* Keep the native kernel tick aligned with the historical computational
 * substrate. Coordinate interpretation is supplied separately by mapping
 * helpers below rather than by injecting M1 traversal semantics here. */
typedef struct {
    uint64_t cycle;
    uint8_t sub_tick;
    QL_Kernel_Phase phase;
    QL_Kernel_Element element;
    uint8_t position6;
    float harmonic_ratio;
} QL_Kernel_Tick;

/* 72-fold resonance is 6 lens anchors x 2 conjugate lens faces x 6 inner
 * positions. The legacy map is retained as a compact dynamics-facing view. */
typedef struct {
    QL_Coordinate_Label lens;
    uint8_t inner_position;
    uint8_t resonance_index;
    uint8_t tritone_square;
} QL_Kernel_Resonance_Map;

/* Native MEF address over the same L/L' kernel field used by the resonance
 * body and Rust PR #19. `absolute_position = (lens + local) mod 6`. */
typedef struct {
    QL_Kernel_Address lens;
    uint8_t local_position;
    uint8_t absolute_position;
    uint8_t resonance_index;
    uint8_t tritone_square;
    uint8_t pitch_class;
} QL_Kernel_MEF_Address;

typedef enum {
    QL_KERNEL_MEF_UNIT_NAME = 0,
    QL_KERNEL_MEF_UNIT_POWER = 1
} QL_Kernel_MEF_Unit_Face;

typedef enum {
    QL_KERNEL_MEF_GRAIN_INNER_FOUR = 0,
    QL_KERNEL_MEF_GRAIN_OUTER_TWO = 1
} QL_Kernel_MEF_Grain;

typedef enum {
    QL_KERNEL_CF1 = 0,
    QL_KERNEL_CF2 = 1,
    QL_KERNEL_CF3 = 2,
    QL_KERNEL_CF4 = 3,
    QL_KERNEL_CF5 = 4,
    QL_KERNEL_CF6 = 5,
    QL_KERNEL_CF7 = 6
} QL_Kernel_Context_Frame_Id;

typedef struct {
    QL_Kernel_Context_Frame_Id frame;
    QL_Kernel_MEF_Address mef;
    QL_Kernel_MEF_Unit_Face unit_face;
    QL_Kernel_MEF_Grain grain;
    const char* notation;
} QL_Kernel_Context_Frame_Address;

typedef struct {
    const char* contract_version;
    const char* historical_reference_revision;
    const char* historical_pointer_web_blob;
    const char* rust_pairing_version;
    const char* rust_mef_rotation_version;
    const char* rust_context_frame_version;
} QL_Kernel_Contract_Provenance;

const char* ql_kernel_api_version(void);
const char* ql_kernel_build_source_revision(void);
const char* ql_kernel_contract_version(void);
QL_Kernel_Contract_Provenance ql_kernel_contract_provenance(void);

const char* ql_kernel_relation_id(QL_Kernel_Relation_Id relation);
uint8_t ql_kernel_pitch_class(uint8_t position, QL_Coordinate_Face face);
uint8_t ql_kernel_mirror_position(uint8_t position);
uint8_t ql_kernel_mirror_interval_role(uint8_t position);
uint8_t ql_kernel_mirror_ratio_role(uint8_t position);
int ql_kernel_relation_resolve(
    QL_Kernel_Relation_Id relation,
    QL_Kernel_Address source,
    QL_Coordinate_Family family_target,
    QL_Kernel_Relation_Ref* out
);

QL_Quaternion ql_quat_normalize(QL_Quaternion q);
QL_Quaternion ql_quat_conjugate(QL_Quaternion q);
QL_Quaternion ql_quat_multiply(QL_Quaternion a, QL_Quaternion b);
float ql_quat_distance_sq(QL_Quaternion a, QL_Quaternion b);

QL_Kernel_Bioquaternion ql_kernel_bioquaternion_init(QL_Quaternion q_b, QL_Quaternion q_p);
QL_Quaternion ql_kernel_slash_flip_bimba_prime(QL_Kernel_Bioquaternion state);

uint8_t ql_kernel_resonance_index(uint8_t lens, uint8_t face, uint8_t position);
uint8_t ql_kernel_tritone_square_for_lens(uint8_t lens);
int ql_kernel_resonance_map(
    uint8_t lens,
    uint8_t face,
    uint8_t position,
    QL_Kernel_Resonance_Map* out
);
int ql_kernel_mef_address(
    uint8_t lens_position,
    QL_Coordinate_Face lens_face,
    uint8_t local_position,
    QL_Kernel_MEF_Address* out
);
int ql_kernel_mef_address_format(
    const QL_Kernel_MEF_Address* address,
    char* out,
    size_t out_size
);
int ql_kernel_context_frame_address(
    QL_Kernel_Context_Frame_Id frame,
    uint8_t lens_position,
    QL_Coordinate_Face lens_face,
    QL_Kernel_Context_Frame_Address* out
);
void ql_kernel_resonance_square_emphasis(
    const QL_Kernel_Resonance_Vector* vector,
    float out_square_emphasis[QL_KERNEL_TRITONE_SQUARES]
);

QL_Kernel_Energy ql_kernel_energy_evaluate(
    QL_Kernel_Bioquaternion state,
    const QL_Kernel_Resonance_Vector* observed,
    const QL_Kernel_Resonance_Vector* target,
    float r_energy
);

float ql_kernel_epogdoon_log(void);
QL_Kernel_Tick ql_kernel_tick_from_epogdoon(uint64_t cycle, uint8_t sub_tick);
QL_Coordinate_Label ql_kernel_tick_position_label(const QL_Kernel_Tick* tick);

#ifdef __cplusplus
}
#endif

#endif /* QL_MEF_KERNEL_H */
