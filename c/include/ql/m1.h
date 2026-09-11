#ifndef QL_M1_H
#define QL_M1_H
#include "ql/m_tree.h"
#ifdef __cplusplus
extern "C" {
#endif
/* K5 computation ABI over the accepted K4 coordinate field.
 * Every ID is resolved in the existing M registry. No new coordinate tree.
 * All checked functions return 1 on success, 0 on error, leave *out unchanged
 * on error, and accept no null output. No allocation or mutable global state.
 */
#define QL_M1_ENGINE_VERSION "1.0.0"
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
/* Literals retain source notation verbatim; they are NOT numeric arithmetic.
 * Pointers have process lifetime. No caller ownership or allocation. */
typedef struct {
    uint32_t family, row12, col12;
    const char *raw_literal, *digit_root_literal;
    uint32_t csv_raw_row, csv_dr_row, csv_column;
} QL_M1_SourceCell;
typedef struct {
    QL_M_NodeId coordinate, subject, source_ground, reflection_ground;
    uint32_t phase, conjugate_phase;
} QL_M1_Reflection;
typedef struct {
    QL_M_NodeId coordinates[6]; /* #1-4.0 through #1-4.5, not trig aliases */
    uint32_t ratio_num, ratio_den, explicate, processual, decimal_frame;
    uint32_t inversion[6], ring_positions, nesting[4];
    uint32_t binary_states, relation_states, resonance_states, cosmic_degrees;
    uint32_t retained_one_total, genus, euler_characteristic;
} QL_M1_Grammar;
typedef struct {
    QL_M_NodeId coordinate; /* #1-5-0, Hamilton product of two phase circles */
    float quaternion[4];
} QL_M1_Rotor;
typedef struct {
    QL_M_NodeId coordinates[6]; /* #1-5-0 through #1-5-5 */
    QL_M1_Clock clock;
    float spinor[4]; /* generated; legacy ring remains separately available */
    double quadrature[2], opposite_quadrature[2];
    uint32_t m2_carrier_count, genus, explicate_edges, identification_slots;
} QL_M1_Carrier;
/* The two 64-state words are supplied independently. A prime face is not
 * silently inferred by complementing the direct word. pair_index is a storage
 * ordinal over 64 x 64, not a new coordinate or a M3 codon identity. */
typedef struct {
    QL_M_NodeId coordinate;
    uint32_t left_position, right_position, direct_word, prime_word;
    uint32_t relation_index, pair_index, direct_face_index, prime_face_index;
    uint32_t bitwise_complement, binary_states, two_face_states, paired_states;
} QL_M1_FiniteField;
int ql_m1_finite_field(uint32_t left, uint32_t right, uint32_t direct_word,
                       uint32_t prime_word, QL_M1_FiniteField *out);
typedef struct {
    QL_M_NodeId coordinates[6];
    uint32_t branch_categories[6], mahamaya_ring[6], parashakti_ring[6];
    uint32_t unary_mask, binary_mask, relational_mask;
} QL_M1_SourceTraits;
int ql_m1_source_traits(QL_M1_SourceTraits *out);
int ql_m1_source_cell(uint32_t family, uint32_t row, uint32_t col, QL_M1_SourceCell *out);
int ql_m1_reflection(QL_M_NodeId subject, uint32_t phase, QL_M1_Reflection *out);
int ql_m1_grammar(QL_M1_Grammar *out);
int ql_m1_rotor(double phase1_radians, double phase2_radians, QL_M1_Rotor *out);
int ql_m1_carrier(uint64_t cycle, uint32_t tick12, QL_M1_Carrier *out);
int ql_m1_clock_advance(uint64_t cycle, uint32_t tick12, uint64_t ticks, QL_M1_Clock *out);
size_t ql_m1_relation_count(QL_M_NodeId coordinate);
const QL_M_Relation *ql_m1_relation_at(QL_M_NodeId coordinate, size_t index);
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
