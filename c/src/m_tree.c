#include "ql/m_tree.h"
#include <string.h>

#ifndef QL_M_TREE_DATA_INCLUDE
#define QL_M_TREE_DATA_INCLUDE "m_tree_data.inc"
#endif
#include QL_M_TREE_DATA_INCLUDE

#define COUNT(a) (sizeof(a) / sizeof((a)[0]))

const char *ql_m_registry_revision(void) { return m_revision; }
const char *ql_m_source_revision(void) { return m_source_revision; }
const char *ql_m_source_repository(void) { return m_source_repository; }
size_t ql_m_node_count(void) { return COUNT(m_nodes); }
const QL_M_Node *ql_m_node_at(size_t i) { return i < COUNT(m_nodes) ? &m_nodes[i] : NULL; }
const QL_M_Node *ql_m_master(void) { return &m_nodes[0]; }
const QL_M_Node *ql_m_root(unsigned p) { return p < COUNT(m_roots) ? ql_m_node_by_id(m_roots[p]) : NULL; }

const QL_M_Node *ql_m_node_by_id(QL_M_NodeId id) {
    size_t lo = 0, hi = COUNT(m_nodes_by_id);
    if (!id) return NULL;
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        const QL_M_Node *n = &m_nodes[m_nodes_by_id[mid]];
        if (n->id == id) return n;
        if (n->id < id) lo = mid + 1; else hi = mid;
    }
    return NULL;
}

const QL_M_Node *ql_m_resolve(const char *ref) {
    size_t lo = 1, hi = COUNT(m_nodes);
    if (!ref) return NULL;
    if (strcmp(ref, "M") == 0) return ql_m_master();
    if ((ref[0] != '#' && ref[0] != 'M') || ref[1] < '0' || ref[1] > '5') return NULL;
    /* Compare after the namespace character: M spelling is an explicit facade,
     * never a numeric reparse that would erase ., -, / or leading zeroes. */
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        int order = strcmp(m_nodes[mid].source_ref + 1, ref + 1);
        if (!order) return &m_nodes[mid];
        if (order < 0) lo = mid + 1; else hi = mid;
    }
    return NULL;
}

const QL_M_Node *ql_m_parent(QL_M_NodeId id) {
    const QL_M_Node *n = ql_m_node_by_id(id);
    return n ? ql_m_node_by_id(n->parent_id) : NULL;
}
size_t ql_m_child_count(QL_M_NodeId id) {
    const QL_M_Node *n = ql_m_node_by_id(id);
    return n ? n->children_count : 0;
}
const QL_M_Node *ql_m_child_at(QL_M_NodeId id, size_t i) {
    const QL_M_Node *n = ql_m_node_by_id(id);
    return n && i < n->children_count ? &m_nodes[m_children[n->children_start + i]] : NULL;
}
size_t ql_m_subtree_count(QL_M_NodeId id) {
    const QL_M_Node *n = ql_m_node_by_id(id);
    return n ? n->subtree_count : 0;
}
size_t ql_m_node_record_index(QL_M_NodeId id, size_t i) {
    const QL_M_Node *n = ql_m_node_by_id(id);
    return n && i < n->records_count ? m_node_records[n->records_start + i] : SIZE_MAX;
}
const QL_M_SourceRecord *ql_m_node_record_at(QL_M_NodeId id, size_t i) {
    const QL_M_Node *n = ql_m_node_by_id(id);
    return n && i < n->records_count ? &m_records[m_node_records[n->records_start + i]] : NULL;
}
size_t ql_m_source_file_count(void) { return COUNT(m_files); }
const QL_M_SourceFile *ql_m_source_file_at(size_t i) { return i < COUNT(m_files) ? &m_files[i] : NULL; }
size_t ql_m_source_record_count(void) { return COUNT(m_records); }
const QL_M_SourceRecord *ql_m_source_record_at(size_t i) { return i < COUNT(m_records) ? &m_records[i] : NULL; }
size_t ql_m_relation_count(void) { return COUNT(m_relations); }
const QL_M_Relation *ql_m_relation_at(size_t i) { return i < COUNT(m_relations) ? &m_relations[i] : NULL; }
const QL_M_Relation *ql_m_relation_by_id(QL_M_RelationId id) {
    size_t lo = 0, hi = COUNT(m_relations_by_id);
    if (!id) return NULL;
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        const QL_M_Relation *r = &m_relations[m_relations_by_id[mid]];
        if (r->id == id) return r;
        if (r->id < id) lo = mid + 1; else hi = mid;
    }
    return NULL;
}
const QL_M_Relation *ql_m_relation_resolve(const char *ref) {
    size_t lo = 0, hi = COUNT(m_relations);
    if (!ref) return NULL;
    while (lo < hi) {
        size_t mid = lo + (hi - lo) / 2;
        int order = strcmp(m_relations[mid].relation_ref, ref);
        if (!order) return &m_relations[mid];
        if (order < 0) lo = mid + 1; else hi = mid;
    }
    return NULL;
}
size_t ql_m_binding_count(void) { return COUNT(m_bindings); }
const QL_M_Binding *ql_m_binding_at(size_t i) { return i < COUNT(m_bindings) ? &m_bindings[i] : NULL; }
int ql_m_binding_valid(const QL_M_Binding *b) {
    if (!b || !b->identity || !b->identity[0] || !b->module || !b->module[0]) return 0;
    if (b->readiness != QL_M_UNBOUND && b->readiness != QL_M_STRUCTURAL_INDEX_ONLY) return 0;
    if (b->readiness == QL_M_STRUCTURAL_INDEX_ONLY && (!b->evidence || !b->evidence[0])) return 0;
    switch (b->disposition) {
        case QL_M_COORDINATE_BOUND:
            return b->relation_id == 0 && ql_m_node_by_id(b->coordinate_id) != NULL;
        case QL_M_CROSS_COORDINATE:
            return b->coordinate_id == 0 && ql_m_relation_by_id(b->relation_id) != NULL;
        case QL_M_INFRASTRUCTURAL:
            return b->coordinate_id == 0 && b->relation_id == 0;
        default: return 0;
    }
}
