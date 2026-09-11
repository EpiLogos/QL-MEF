#ifndef QL_M3_H
#define QL_M3_H
#include "ql/m_tree.h"
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif

/* K7 finite-engine ABI. Semantic projections retain source standing; this is
 * not a blanket declaration of graph, instrument or experiential readiness. */
#define QL_M3_ENGINE_VERSION "ql.m3-engine/v1"
#define QL_M3_CLOCK_FIELDS 26u
#define QL_M3_NO_VALUE 255u

typedef enum { QL_M3_OK=0, QL_M3_INVALID=1, QL_M3_GAP=2 } QL_M3_Result;
typedef enum {
 QL_M3_ROOT, QL_M3_NUCLEOTIDE, QL_M3_PAIR, QL_M3_CODON,
 QL_M3_TRIGRAM, QL_M3_HEXAGRAM, QL_M3_MATRIX, QL_M3_MINOR,
 QL_M3_MAJOR, QL_M3_DEGREE, QL_M3_BACKBONE, QL_M3_PHASE
} QL_M3_NodeKind;
/* Stable field positions in the retained C clock record. DOUBLED_DEGREE means
 * 2*d, not d+360. Recorded zeros are NOT recognized symbolic values. */
typedef enum {
 QL_M3_DEGREE_360, QL_M3_DOUBLED_DEGREE, QL_M3_ZODIAC_SIGN,
 QL_M3_ZODIAC_DEGREE, QL_M3_DECAN, QL_M3_DECAN_POSITION,
 QL_M3_BACKBONE_FLAG, QL_M3_HEXAGRAM_ESTIMATE, QL_M3_LINE_ESTIMATE,
 QL_M3_NONDUAL_ESTIMATE, QL_M3_CLASS_ESTIMATE, QL_M3_UPPER_PAIR_ESTIMATE,
 QL_M3_LOWER_PAIR_ESTIMATE, QL_M3_TAROT_PLACEHOLDER,
 QL_M3_DECAN_PLANET_RECORD, QL_M3_DECAN_ELEMENT_RECORD,
 QL_M3_DECAN_CHAKRA_RECORD, QL_M3_TICK12_RECORD, QL_M3_STRAND_RECORD,
 QL_M3_DR_RING_RECORD, QL_M3_ANANDA_PLACEHOLDER, QL_M3_ARCHETYPE_PLACEHOLDER,
 QL_M3_SHADOW_DEGREE, QL_M3_POLAR_DEGREE, QL_M3_CHAMBER_RECORD,
 QL_M3_CHAMBER_FACE_RECORD
} QL_M3_ClockField;

typedef struct {
 uint8_t address, nucleotides[3], values[3], class_id, state_count;
 uint8_t complement, nuclear, anticodon, amino_record, is_stop, rna_changes;
 int8_t charges[4]; /* pp, mm, mp, pm */
 QL_M_NodeId coordinate;
} QL_M3_CodonRecord;
typedef struct { float w,x,y,z; } QL_M3_Quaternion;
typedef struct { uint8_t card_id; const char *name; uint8_t chromosome_pair, amino_acid_index; } QL_M3_Major;
typedef struct {
 uint8_t pair1,pair2,codon,positive,slot,nondual;
 uint16_t degrees;
 int8_t value;
} QL_M3_Rotation;
typedef struct {
 uint64_t steps, completed_double_covers;
 uint16_t degree720, degree360, polar720;
 uint8_t layer,tick12,decan_phase,uniform_hexagram_estimate;
 QL_M_NodeId degree_node,backbone_node,clockwise_node,polar_node;
} QL_M3_Clock;

const char *ql_m3_registry_revision(void);
const char *ql_m3_source_digest(void);
size_t ql_m3_group_count(QL_M3_NodeKind kind);
const QL_M_Node *ql_m3_node(QL_M3_NodeKind kind, size_t ordinal);
/* Every M3 coordinate and original source relation remains in ql_m_tree. This
 * accessor gives the existing source body, not a claim that each is computed. */
size_t ql_m3_coordinate_count(void);
const QL_M_Node *ql_m3_coordinate_at(size_t ordinal);
QL_M3_Result ql_m3_codon(unsigned address, QL_M3_CodonRecord *out);
QL_M3_Result ql_m3_pair(unsigned pair, int8_t out[2]);
QL_M3_Result ql_m3_trigram(unsigned id, uint16_t out[7]);
QL_M3_Result ql_m3_hexagram(unsigned id, uint8_t out[5]);
QL_M3_Result ql_m3_line_change(unsigned codon, unsigned line, uint8_t *out);
QL_M3_Result ql_m3_matrix_partner(unsigned matrix, unsigned nucleotide, uint8_t *out);
QL_M3_Result ql_m3_apply_matrix(unsigned matrix, unsigned codon, uint8_t *out);
QL_M3_Result ql_m3_rotational_profile(unsigned codon, uint8_t out[5]);
QL_M3_Result ql_m3_rotations(unsigned codon, QL_M3_Rotation out[8]);
QL_M3_Result ql_m3_pose_ordinal(unsigned codon, unsigned slot, uint16_t *out);
QL_M3_Result ql_m3_minor(unsigned card, uint8_t out[4]);
const QL_M3_Major *ql_m3_major(unsigned card);
QL_M3_Result ql_m3_quaternion(unsigned codon, unsigned state, QL_M3_Quaternion *out);
QL_M3_Result ql_m3_active_state(QL_M3_Quaternion environment, unsigned codon, uint8_t *out);
QL_M3_Result ql_m3_transcribe(unsigned codon, unsigned rna, char out[4]);
/* Recorded amino slots, NOT an asserted standard biological translation.
 * STOP is slot 10. T->U changes the alphabet, never the nucleotide polarity. */
QL_M3_Result ql_m3_transduce(const uint8_t *indices, size_t count, uint64_t *out);
QL_M3_Result ql_m3_epogdoon(unsigned index, uint8_t *compressed, uint8_t *roundtrip_gap);
QL_M3_Result ql_m3_clock(uint64_t steps, QL_M3_Clock *out);
QL_M3_Result ql_m3_clock_advance(uint64_t steps, uint64_t delta, QL_M3_Clock *out);
QL_M3_Result ql_m3_clock_record(unsigned degree, uint16_t out[QL_M3_CLOCK_FIELDS]);
/* Deliberately zero: no symbolic clock column is promoted by numeric parity. */
uint32_t ql_m3_clock_reconciled_symbolic_fields(void);
#ifdef __cplusplus
}
#endif
#endif
