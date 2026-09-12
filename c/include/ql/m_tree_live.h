#ifndef QL_M_TREE_LIVE_H
#define QL_M_TREE_LIVE_H
#include "ql/m_tree.h"
#ifdef __cplusplus
extern "C" {
#endif
#define QL_M_LIVE_TREE_SCHEMA "ql.m-tree/v2"
#define QL_M_LIVE_TREE_API_VERSION "2.0.0"
/* Current projection over the preserved accepted v1 view. Same spelling-derived
 * IDs; only reviewed additions. The v1 ABI and its original proof remain usable.
 * All pointers are immutable, process-lifetime, and registry-owned.
 * source_repository/source_revision describe the inherited corpus. Resolve each
 * file's actual authority with source_origin_at; promotion files are not falsely
 * attributed to the historical C-Experiments commit.
 */
typedef struct { const char *repository; const char *revision; } QL_M_SourceOrigin;
const QL_M_SourceOrigin *ql_m_live_source_origin_at(size_t file_index);
const char *ql_m_live_base_registry_revision(void);
int ql_m_live_accepts_base(const char *registry_revision);
const char *ql_m_live_registry_revision(void);
const char *ql_m_live_source_revision(void);
const char *ql_m_live_source_repository(void);
size_t ql_m_live_node_count(void);
const QL_M_Node *ql_m_live_node_at(size_t index);
const QL_M_Node *ql_m_live_node_by_id(QL_M_NodeId id);
const QL_M_Node *ql_m_live_resolve(const char *source_or_m_ref);
const QL_M_Node *ql_m_live_master(void);
const QL_M_Node *ql_m_live_root(unsigned position);
const QL_M_Node *ql_m_live_parent(QL_M_NodeId id);
size_t ql_m_live_child_count(QL_M_NodeId id);
const QL_M_Node *ql_m_live_child_at(QL_M_NodeId id, size_t child_index);
size_t ql_m_live_subtree_count(QL_M_NodeId id);
size_t ql_m_live_node_record_index(QL_M_NodeId id, size_t record_index);
const QL_M_SourceRecord *ql_m_live_node_record_at(QL_M_NodeId id, size_t record_index);
size_t ql_m_live_source_file_count(void);
const QL_M_SourceFile *ql_m_live_source_file_at(size_t index);
size_t ql_m_live_source_record_count(void);
const QL_M_SourceRecord *ql_m_live_source_record_at(size_t index);
size_t ql_m_live_relation_count(void);
const QL_M_Relation *ql_m_live_relation_at(size_t index);
const QL_M_Relation *ql_m_live_relation_by_id(QL_M_RelationId id);
const QL_M_Relation *ql_m_live_relation_resolve(const char *relation_ref);
size_t ql_m_live_binding_count(void);
const QL_M_Binding *ql_m_live_binding_at(size_t index);
int ql_m_live_binding_valid(const QL_M_Binding *binding);

#ifdef __cplusplus
}
#endif
#endif
