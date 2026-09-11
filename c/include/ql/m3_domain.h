#ifndef QL_M3_DOMAIN_H
#define QL_M3_DOMAIN_H
#include "ql/m3.h"
#ifdef __cplusplus
extern "C" {
#endif
/* Source-reading accessors. These retain Bimba identities and qualification;
 * their presence does not promote historical claims to current structural law.
 * Returned pointers have static lifetime and are immutable. Zero IDs are absent.
 */
typedef struct { QL_M_NodeId id; uint32_t source_record; const char *role; } QL_M3_SourceNode;
typedef struct {
    QL_M_RelationId id;
    QL_M_NodeId from, to;
    uint32_t source_record;
    const char *kind;
    const char *properties_json;
} QL_M3_SourceRelation;
typedef struct {
    QL_M_NodeId backbone, codon, hexagram;
    uint8_t codon_address, hexagram_address;
    uint32_t source_record;
} QL_M3_BackboneProjection;
typedef struct {
    QL_M_NodeId id, hexagram;
    QL_M_RelationId resolves;
    uint8_t family, address;
    uint16_t link_offset, pair_count, codon_count;
} QL_M3_MatrixCell;
/* Exact discrete form projection in tenths of a degree, EFWA element order.
 * This grammar is not an identity hash, identity/live quaternion or bioquaternion.
 */
typedef struct {
    uint8_t address, pose, state_count, aperture, reciprocal_aperture, axis;
    uint8_t pair_xy, pair_yz, hinge, site_elements[3], elemental_counts[4];
    uint16_t pose_ordinal, fibonacci_phase60;
    int32_t angles_deg10[3], velocities_deg10[3];
    int32_t pair_angles_deg10[2], lens_division_deg10, lens_reciprocal_deg10;
    int32_t void_ring_orientation_deg10;
} QL_M3_Form;
const char *ql_m3_domain_revision(void);
size_t ql_m3_source_node_count(void);
const QL_M3_SourceNode *ql_m3_source_node_at(size_t ordinal);
size_t ql_m3_source_relation_count(void);
const QL_M3_SourceRelation *ql_m3_source_relation_at(size_t ordinal);
size_t ql_m3_matrix_cell_count(void);
const QL_M3_MatrixCell *ql_m3_matrix_cell_at(size_t ordinal);
const QL_M3_SourceRelation *ql_m3_matrix_cell_relation(size_t cell, size_t link);
/* Source-recorded backbone prototype; deliberately separate from current form,
 * uniform clock estimate, and the retained 26-column LUT. */
const QL_M3_BackboneProjection *ql_m3_backbone_projection(QL_M_NodeId backbone);
QL_M3_Result ql_m3_clock_projection(uint64_t steps, QL_M3_BackboneProjection *out);
QL_M3_Result ql_m3_form(unsigned address, unsigned pose, unsigned aperture,
                        unsigned fibonacci_phase60, unsigned axis, QL_M3_Form *out);
QL_M3_Result ql_m3_form_apply_matrix(const QL_M3_Form *form, unsigned matrix, QL_M3_Form *out);
QL_M3_Result ql_m3_cast(const int32_t angles[3], const int32_t velocities[3], uint8_t *address);
#ifdef __cplusplus
}
#endif
#endif
