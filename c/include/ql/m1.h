#ifndef QL_M1_H
#define QL_M1_H
#include "ql/m_tree.h"
#ifdef __cplusplus
extern "C" {
#endif
/* Candidate K5 computation ABI. K4 acceptance and whole-M1 readiness are separate.
 * Every ID is resolved in the existing M registry. No new coordinate tree.
 * All checked functions return 1 on success, 0 on error, leave *out unchanged
 * on error, and accept no null output. No allocation or mutable global state.
 */
#define QL_M1_ENGINE_VERSION "0.1.0"
#define QL_M1_RETURN_REVISION "bb47ab9730f0ddadd4891666fb6f3e0a6d457330"
typedef struct {
    uint64_t cycle;
    uint32_t tick12, position6, phase, conjugate_tick12, conjugate_phase;
    uint32_t degree360, hopf_fiber, degree720, spanda_stage;
} QL_M1_Clock;
typedef struct {
    QL_M_NodeId coordinate, dr_coordinate;
    uint32_t family, row12, col12, scalar_valid, decimal10_valid;
    int32_t raw; /* unused (zero), NOT a scalar Quintessence, when scalar_valid=0 */
    uint32_t digit_root, decimal10;
    /* Ordered contributors: Bimba, Pratibimba, Sum, Difference A, Difference B.
     * Quintessence source tuple is {-1, raw_terms[0], raw_terms[2]}.
     * Difference A: raw -1, source-defined complement residue 9, decimal 9.
     */
    int32_t raw_terms[5];
    uint32_t dr_terms[5], decimal_terms[5];
    QL_M1_Clock clock;
} QL_M1_Cell;
typedef struct {
    QL_M_NodeId coordinate, substage_coordinate;
    uint32_t stage, substage, fold_count, dual_track;
    float weave_state;
    uint32_t inversion_state;
} QL_M1_Spanda;
typedef struct {
    QL_M_NodeId coordinate;
    uint32_t stage, next, inverse, numerator_position, denominator_position;
    int32_t signature; /* Cl(4,2): [-1,+1,+1,+1,+1,-1]; unity sentinel is 6. */
} QL_M1_Formal;
typedef struct {
    QL_M_NodeId coordinate;
    uint32_t tick12, element_count, legacy_return_stage;
    float legacy_ring_quaternion[4]; /* retained LUT, NOT the generated clock orbit */
} QL_M1_Topology;
typedef struct {
    QL_M_NodeId coordinate;
    double x, y, z; /* source embedding, fixed R=16/9, r=1 */
} QL_M1_Torus;
const char *ql_m1_engine_version(void);
const QL_M_Node *ql_m1_node(const char *coordinate);
int ql_m1_clock(uint64_t cycle, uint32_t tick12, QL_M1_Clock *out);
int ql_m1_cell(uint32_t family, uint32_t row12, uint32_t col12,
               uint64_t cycle, uint32_t tick12, QL_M1_Cell *out);
int ql_m1_spanda(uint32_t stage, uint32_t substage, QL_M1_Spanda *out);
int ql_m1_formal(uint32_t stage, QL_M1_Formal *out);
int ql_m1_topology(uint32_t tick12, QL_M1_Topology *out);
int ql_m1_torus(double theta1, double theta2, QL_M1_Torus *out);
int ql_m1_valid_fold(uint32_t fold);
#ifdef __cplusplus
}
#endif
#endif
