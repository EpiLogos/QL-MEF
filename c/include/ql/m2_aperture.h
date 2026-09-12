#ifndef QL_M2_APERTURE_H
#define QL_M2_APERTURE_H
#include "ql/m_tree_live.h"
#ifdef __cplusplus
extern "C" {
#endif
#define QL_M2_APERTURE_COUNT 18u
#define QL_M2_STATIC_APERTURE_COUNT 16u
#define QL_M2_NO_NATIVE_APERTURE 255u
typedef enum { QL_M2_STATIC_APERTURE=0, QL_M2_VOID_APERTURE=1, QL_M2_FIBONACCI_APERTURE=2 } QL_M2_ApertureKind;
typedef struct {
    QL_M_NodeId coordinate, pair, reciprocal;
    QL_M2_ApertureKind kind;
    uint16_t division_half_degrees, segments, native_orientation_half_degrees;
    uint8_t native_index;
} QL_M2_Aperture;
/* Indices 0..15 preserve native ascending divisor order; 16 is void, 17 ground.
 * Pair-container/display order never redefines the native index. */
const QL_M2_Aperture *ql_m2_aperture_at(unsigned index);
const QL_M2_Aperture *ql_m2_aperture_by_id(QL_M_NodeId id);
int ql_m2_void_antipode(unsigned position, unsigned *out);
#ifdef __cplusplus
}
#endif
#endif
