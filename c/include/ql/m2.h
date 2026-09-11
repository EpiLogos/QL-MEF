#ifndef QL_M2_H
#define QL_M2_H
/* Paraśakti native structural/finite-operation ABI. No arena or Neo4j runtime.
 * Descriptor correspondence is retained-source standing, not canon promotion.
 * coordinate_id==0 is an unresolved/branch law, NEVER an invented K2 node.
 * On failure all caller outputs are unchanged. Inputs and outputs may alias.
 */
#include <stddef.h>
#include <stdint.h>
#include "ql/m_tree.h"
#ifdef __cplusplus
extern "C" {
#endif
#define QL_M2_ENGINE_CONTRACT "ql.m2-engine/v1"
#define QL_M2_CARRIER_SIZE 72u
#define QL_M2_FORM_SIZE 64u
#define QL_M2_MAX_COMPONENT INT64_C(1000000)
#define QL_M2_MAX_COLUMNS 11u

typedef enum {
    QL_M2_OK=0, QL_M2_INVALID=1, QL_M2_BOUNDARY=2, QL_M2_NONFINITE=3, QL_M2_UNAVAILABLE=4
} QL_M2_Result;
typedef enum {
    QL_M2_MEF=0, QL_M2_TATTVA=1, QL_M2_DECAN=2, QL_M2_SHEM=3
} QL_M2_Register72;
typedef enum {
    QL_M2_CARRIER_TABLE=0, QL_M2_MEF_TABLE, QL_M2_TATTVA_TABLE,
    QL_M2_DECAN_TABLE, QL_M2_PLANET_TABLE, QL_M2_CHAKRA_TABLE,
    QL_M2_SHEM_TABLE, QL_M2_RATIO_TABLE, QL_M2_MAQAM_TABLE,
    QL_M2_STATION_TABLE, QL_M2_ASMA_TABLE, QL_M2_MANTRA_TABLE,
    QL_M2_ELEMENT_TABLE, QL_M2_DET_TABLE, QL_M2_RESONANCE_TABLE,
    QL_M2_ROUTING_TABLE, QL_M2_TABLE_COUNT
} QL_M2_TableId;
typedef struct {
    QL_M_NodeId coordinate_id;
    uint64_t values[QL_M2_MAX_COLUMNS];
} QL_M2_Record;
typedef struct {
    const char *name, *source_symbol, *scope;
    size_t row_count, column_count;
    const char *columns[QL_M2_MAX_COLUMNS];
    const QL_M2_Record *rows;
} QL_M2_Table;
typedef struct { int64_t re, im; } QL_M2_Amplitude;
typedef struct { uint8_t type; double angle, orb; } QL_M2_Aspect;
typedef struct {
    int8_t phi_polynomial[3]; /* ascending: -1 - x + x^2 */
    uint8_t tattva_divisions[3], mef[2], tattva[2], decan[4], shem[2];
    uint8_t material[2], form[2], asma[2], mantra[2], station[2];
} QL_M2_Ground;
/* Numerical Vimarsha seeds. M3 pose legality belongs to the shared M3 API. */
typedef struct { uint8_t tick12, lens, musical_mode, codon, rotation; uint16_t ratio_num, ratio_den; } QL_M2_VimarshaSeed;
typedef struct { uint8_t ql_position, helix, m, n; } QL_M2_NodalConstraint;
typedef struct { float audio_octet_hz[8]; QL_M2_NodalConstraint nodal_quartet[4]; } QL_M2_VimarshaReading;
QL_M2_Result ql_m2_vimarsha(const QL_M2_VimarshaSeed *in, QL_M2_VimarshaReading *out);
const QL_M2_Ground *ql_m2_ground(void);
const char *ql_m2_registry_revision(void);
const QL_M2_Table *ql_m2_table(unsigned table);
const QL_M2_Record *ql_m2_record(unsigned table, size_t row);
/* Counts exact retained-record associations only, not whole-coordinate readiness. */
size_t ql_m2_exact_record_count(QL_M_NodeId coordinate);
QL_M2_Result ql_m2_flatten(unsigned reading, unsigned a, unsigned b, unsigned c, unsigned d, uint8_t *out);
QL_M2_Result ql_m2_unflatten(unsigned reading, unsigned index, uint8_t out[4]);
QL_M2_Result ql_m2_signature(unsigned element, unsigned chakra, unsigned phase, uint8_t *out);
QL_M2_Result ql_m2_unpack_signature(unsigned signature, uint8_t out[3]);
QL_M2_Result ql_m2_tattva_step(unsigned principle, unsigned phase, uint8_t *out);
/* Decan group order F/E/A/W differs from Templateure fibre order E/F/W/A. */
QL_M2_Result ql_m2_decan_to_fibre(unsigned index, uint8_t *out);
QL_M2_Result ql_m2_fibre_target(unsigned index, uint8_t *out);
/* Historical scalar address shadow and retained OR mask are distinct laws. */
QL_M2_Result ql_m2_scalar_compress(unsigned index, uint8_t *out);
QL_M2_Result ql_m2_scalar_expand(unsigned index, uint8_t *out);
QL_M2_Result ql_m2_legacy_det(const uint8_t *indices, size_t count, uint64_t *out);
QL_M2_Result ql_m2_asma_route(unsigned index, uint8_t *out); /* 0 internal, 1 projective */
QL_M2_Result ql_m2_planet_preempted(unsigned index, uint8_t *out);
uint8_t ql_m2_digital_root(uint64_t value);
QL_M2_Result ql_m2_aspect(double a, double b, QL_M2_Aspect *out);
QL_M2_Result ql_m2_maqam_pitch(unsigned mode, unsigned degree, double root_hz, double *out);
/* Exact bounded coefficients. Power is conserved by Q, not by coherent folding. */
QL_M2_Result ql_m2_modal_quadrature(const QL_M2_Amplitude in[72], QL_M2_Amplitude out[72]);
QL_M2_Result ql_m2_modal_transduce(const QL_M2_Amplitude in[72], QL_M2_Amplitude out[64]);
QL_M2_Result ql_m2_modal_power(const QL_M2_Amplitude *in, size_t count, uint64_t *out);
/* Source-attributed musical/planetary/chakral correspondence projection.
 * role: 0 tonic, 1 dominant; fibre: EFWA 0..3 or 255 (beyond/space).
 * Missing colour is NULL, not a rainbow default. Repeated source assertion IDs
 * and literal/property provenance remain in ql.m2-correspondences/v1.
 */
typedef struct {
    QL_M_NodeId maqam_id, planet_id, chakra_id, tattva_id;
    QL_M_RelationId musical_relation_id, planetary_relation_id;
    uint8_t maqam_index, role, planet_index, chakra_index, fibre, spelled_supported;
    uint8_t spelled_steps24[8];
    const char *colour_name, *element_literal, *planetary_mode_literal, *interval_literal;
} QL_M2_Correspondence;
size_t ql_m2_correspondence_count(void);
const QL_M2_Correspondence *ql_m2_correspondence_at(size_t index);
const QL_M2_Correspondence *ql_m2_correspondence(uint8_t maqam_index, uint8_t role);
/* tuning: 0 retained-C 24-TET; 1 explicit Bimba note-spelling 24-TET policy.
 * These are separately named readings, never an equivalence assertion.
 */
QL_M2_Result ql_m2_correspondence_pitch(uint8_t maqam_index, uint8_t role,
    uint8_t tuning, uint8_t degree, double tonic_hz, double *out);
#ifdef __cplusplus
}
#endif
#endif
