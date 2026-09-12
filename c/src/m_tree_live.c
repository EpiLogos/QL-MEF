/* Compile the same structural index implementation over its current generated
 * source projection. This is version compatibility, not another authored tree. */
#include "ql/m_tree_live.h"
#define ql_m_registry_revision ql_m_live_registry_revision
#define ql_m_source_revision ql_m_live_source_revision
#define ql_m_source_repository ql_m_live_source_repository
#define ql_m_node_count ql_m_live_node_count
#define ql_m_node_at ql_m_live_node_at
#define ql_m_node_by_id ql_m_live_node_by_id
#define ql_m_resolve ql_m_live_resolve
#define ql_m_master ql_m_live_master
#define ql_m_root ql_m_live_root
#define ql_m_parent ql_m_live_parent
#define ql_m_child_count ql_m_live_child_count
#define ql_m_child_at ql_m_live_child_at
#define ql_m_subtree_count ql_m_live_subtree_count
#define ql_m_node_record_index ql_m_live_node_record_index
#define ql_m_node_record_at ql_m_live_node_record_at
#define ql_m_source_file_count ql_m_live_source_file_count
#define ql_m_source_file_at ql_m_live_source_file_at
#define ql_m_source_record_count ql_m_live_source_record_count
#define ql_m_source_record_at ql_m_live_source_record_at
#define ql_m_relation_count ql_m_live_relation_count
#define ql_m_relation_at ql_m_live_relation_at
#define ql_m_relation_by_id ql_m_live_relation_by_id
#define ql_m_relation_resolve ql_m_live_relation_resolve
#define ql_m_binding_count ql_m_live_binding_count
#define ql_m_binding_at ql_m_live_binding_at
#define ql_m_binding_valid ql_m_live_binding_valid
#define QL_M_TREE_DATA_INCLUDE "m_tree_live_data.inc"
#include "m_tree.c"
#include "m_tree_live_origins.inc"
const QL_M_SourceOrigin *ql_m_live_source_origin_at(size_t i) {
    return i < COUNT(m_live_origins) ? &m_live_origins[i] : NULL;
}
const char *ql_m_live_base_registry_revision(void) { return m_live_base_revision; }
int ql_m_live_accepts_base(const char *revision) {
    return revision && (!strcmp(revision,m_live_base_revision) || !strcmp(revision,m_revision));
}
