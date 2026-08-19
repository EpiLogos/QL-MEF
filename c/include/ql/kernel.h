#ifndef QL_MEF_KERNEL_H
#define QL_MEF_KERNEL_H

#include "ql/primitive.h"

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define QL_KERNEL_API_VERSION_MAJOR 0u
#define QL_KERNEL_API_VERSION_MINOR 1u
#define QL_KERNEL_API_VERSION_PATCH 0u
#define QL_KERNEL_RESONANCE_DIM QL_RESONANCE_COUNT
#define QL_KERNEL_TRITONE_SQUARES QL_TRITONE_SQUARE_COUNT

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

typedef struct {
    uint64_t cycle;
    uint8_t sub_tick;
    QL_Ring_Half half;
    QL_Kernel_Phase phase;
    QL_Kernel_Element element;
    uint8_t position6;       /* historical kernel projection: tick % 6 */
    uint8_t base_position;   /* explicit native name for position6 */
    uint8_t traversal_stage; /* distinct M1 inverted-half projection; #60 remains open */
    float harmonic_ratio;
} QL_Kernel_Tick;

const char* ql_kernel_api_version(void);
const char* ql_kernel_build_source_revision(void);

QL_Quaternion ql_quat_normalize(QL_Quaternion q);
QL_Quaternion ql_quat_conjugate(QL_Quaternion q);
QL_Quaternion ql_quat_multiply(QL_Quaternion a, QL_Quaternion b);
float ql_quat_distance_sq(QL_Quaternion a, QL_Quaternion b);

QL_Kernel_Bioquaternion ql_kernel_bioquaternion_init(QL_Quaternion q_b, QL_Quaternion q_p);
QL_Quaternion ql_kernel_slash_flip_bimba_prime(QL_Kernel_Bioquaternion state);

uint8_t ql_kernel_resonance_index(uint8_t lens, uint8_t face, uint8_t position);
uint8_t ql_kernel_tritone_square_for_lens(uint8_t lens);
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

#ifdef __cplusplus
}
#endif

#endif /* QL_MEF_KERNEL_H */
