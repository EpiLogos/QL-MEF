#ifndef QL_M_TREE_H
#define QL_M_TREE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define QL_M_TREE_SCHEMA "ql.m-tree/v1"
#define QL_M_TREE_API_VERSION "1.0.0"
#define QL_M_NO_ID UINT64_C(0)
#define QL_M_NO_ROOT 255u

typedef uint64_t QL_M_NodeId;
typedef uint64_t QL_M_RelationId;
typedef enum { QL_M_MASTER_INDEX, QL_M_SOURCE_DECLARED } QL_M_StructuralStatus;
typedef enum { QL_M_PARENT_MASTER, QL_M_PARENT_AGGREGATE, QL_M_PARENT_SOURCE,
               QL_M_PARENT_PREFIX } QL_M_ParentBasis;
typedef enum { QL_M_DIRECTED, QL_M_UNDIRECTED, QL_M_UNSPECIFIED } QL_M_Orientation;
typedef enum { QL_M_COORDINATE_BOUND, QL_M_CROSS_COORDINATE,
               QL_M_INFRASTRUCTURAL } QL_M_Disposition;
typedef enum { QL_M_UNBOUND, QL_M_STRUCTURAL_INDEX_ONLY } QL_M_Readiness;

/* IDs are spelling-derived, stable across regeneration, and not storage offsets.
 * All returned objects/strings are immutable, process-lifetime, registry-owned.
 * M is the master index; # remains external meta-bedrock, not an alias for M.
 * M0/#0 (etc.) name the same source identity. Separators never canonicalise.
 * local_segment retains compound forms such as 0/1/2 rather than inventing
 * nodes for each slash. Lexical depth is distinct from source tree depth.
 */
typedef struct {
    QL_M_NodeId id, parent_id, root_id;
    const char *source_ref, *local_segment, *separator, *lexical_parent_source_ref;
    const char *source_parent_refs_json, *names_json, *aliases_json;
    uint32_t root_position, depth, lexical_depth;
    QL_M_ParentBasis parent_basis;
    QL_M_StructuralStatus structural_status;
    uint32_t aggregate, children_start, children_count, records_start, records_count, subtree_count;
} QL_M_Node;

typedef struct {
    const char *path, *git_blob, *sha256, *record_class;
    uint64_t bytes;
} QL_M_SourceFile;
typedef struct {
    uint32_t file_index, record_index;
    const char *payload_sha256, *property_keys_json;
} QL_M_SourceRecord;
typedef struct {
    QL_M_RelationId id;
    QL_M_NodeId from_id, to_id;
    const char *relation_ref, *source_kind, *from_ref, *to_ref;
    QL_M_Orientation orientation;
    uint32_t cross_m, record;
} QL_M_Relation;

/* A binding describes an implementation; it cannot create a structural node.
 * This ABI supplies K3/K4 disposition identities, not the full M-body census.
 * K2's built-in bindings only disclose the seven aggregate structural indexes.
 */
typedef struct {
    const char *identity, *module;
    QL_M_Disposition disposition;
    QL_M_NodeId coordinate_id;
    QL_M_RelationId relation_id;
    QL_M_Readiness readiness;
    const char *evidence;
} QL_M_Binding;

const char *ql_m_registry_revision(void);
const char *ql_m_source_revision(void);
const char *ql_m_source_repository(void);
size_t ql_m_node_count(void);
const QL_M_Node *ql_m_node_at(size_t index);
const QL_M_Node *ql_m_node_by_id(QL_M_NodeId id);
const QL_M_Node *ql_m_resolve(const char *source_or_m_ref);
const QL_M_Node *ql_m_master(void);
const QL_M_Node *ql_m_root(unsigned position);
const QL_M_Node *ql_m_parent(QL_M_NodeId id);
size_t ql_m_child_count(QL_M_NodeId id);
const QL_M_Node *ql_m_child_at(QL_M_NodeId id, size_t child_index);
size_t ql_m_subtree_count(QL_M_NodeId id);
size_t ql_m_node_record_index(QL_M_NodeId id, size_t record_index);
const QL_M_SourceRecord *ql_m_node_record_at(QL_M_NodeId id, size_t record_index);
size_t ql_m_source_file_count(void);
const QL_M_SourceFile *ql_m_source_file_at(size_t index);
size_t ql_m_source_record_count(void);
const QL_M_SourceRecord *ql_m_source_record_at(size_t index);
size_t ql_m_relation_count(void);
const QL_M_Relation *ql_m_relation_at(size_t index);
const QL_M_Relation *ql_m_relation_by_id(QL_M_RelationId id);
const QL_M_Relation *ql_m_relation_resolve(const char *relation_ref);
size_t ql_m_binding_count(void);
const QL_M_Binding *ql_m_binding_at(size_t index);
int ql_m_binding_valid(const QL_M_Binding *binding);

#ifdef __cplusplus
}
#endif
#endif
