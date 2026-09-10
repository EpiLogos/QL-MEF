/* Execute the real native registry and emit every public descriptor for Rust.
 * No second C fixture tree or coordinate list is authored in this probe. */
#include "ql/m_tree.h"
#include <inttypes.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CHECK(x) do { ++checks; if (!(x)) { fprintf(stderr, "FAIL line %d: %s\n", __LINE__, #x); return 1; } } while (0)
static size_t checks;

static void string(const char *s) {
    if (!s) { fputs("null", stdout); return; }
    putchar('"');
    for (const unsigned char *p = (const unsigned char *)s; *p; ++p) {
        if (*p == '"' || *p == '\\') { putchar('\\'); putchar(*p); }
        else if (*p < 32) printf("\\u%04x", (unsigned)*p);
        else putchar(*p);
    }
    putchar('"');
}
static void id(uint64_t value) {
    if (value) printf("\"%016" PRIx64 "\"", value); else fputs("null", stdout);
}
static void key(const char *name, const char *value) { printf(",\"%s\":", name); string(value); }
static void key_id(const char *name, uint64_t value) { printf(",\"%s\":", name); id(value); }
static const char *boolean(unsigned value) { return value ? "true" : "false"; }

int main(int argc, char **argv) {
    int emit = argc == 1;
    if (argc > 2 || (argc == 2 && strcmp(argv[1], "--check"))) return 2;
    const QL_M_Node *master = ql_m_master();
    CHECK(master && ql_m_resolve("M") == master);
    CHECK(master->children_count == 6 && master->subtree_count == ql_m_node_count());
    CHECK(ql_m_parent(master->id) == NULL);
    CHECK(!ql_m_resolve(NULL) && !ql_m_resolve("") && !ql_m_resolve("#"));
    const char *invalid[] = {"#-0", "M6", "#6", "#0-", "#0/", "#0..1", "#0-4.0", "#3-5-5", "#2-999999", " #0", "#0 ", "#0\n", "#0-01", "m0"};
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); ++i) CHECK(!ql_m_resolve(invalid[i]));
    CHECK(!ql_m_root(6) && !ql_m_root(256) && !ql_m_root(UINT_MAX));
    CHECK(!ql_m_node_by_id(0) && !ql_m_node_at(SIZE_MAX) && !ql_m_parent(0));
    CHECK(!ql_m_child_at(master->id, SIZE_MAX) && !ql_m_child_at(0, 0));
    CHECK(ql_m_child_count(0) == 0 && ql_m_subtree_count(0) == 0);
    CHECK(!ql_m_node_record_at(0, 0) && !ql_m_node_record_at(master->id, SIZE_MAX));
    CHECK(ql_m_node_record_index(0, 0) == SIZE_MAX);
    CHECK(!ql_m_source_file_at(SIZE_MAX) && !ql_m_source_record_at(SIZE_MAX));
    CHECK(!ql_m_relation_at(SIZE_MAX) && !ql_m_relation_by_id(0));
    CHECK(!ql_m_relation_resolve(NULL) && !ql_m_relation_resolve(""));
    CHECK(!ql_m_binding_at(SIZE_MAX) && !ql_m_binding_valid(NULL));
    const QL_M_Node *compound = ql_m_resolve("#0-4.0/1/2");
    CHECK(compound && ql_m_parent(compound->id) == ql_m_resolve("#0-4"));
    CHECK(!strcmp(compound->local_segment, "0/1/2") && !strcmp(compound->separator, "."));
    CHECK(compound->id != ql_m_resolve("#0-4.0/1-2")->id);
    CHECK(ql_m_resolve("#4.5-0")->id != ql_m_resolve("#4.5.0")->id);

    if (emit) {
        fputs("{\"kind\":\"registry\"", stdout);
        key("schema", QL_M_TREE_SCHEMA); key("registry_revision", ql_m_registry_revision());
        key("source_revision", ql_m_source_revision()); key("source_repository", ql_m_source_repository());
        printf(",\"nodes\":%zu,\"relations\":%zu,\"files\":%zu,\"records\":%zu,\"bindings\":%zu}\n",
               ql_m_node_count(), ql_m_relation_count(), ql_m_source_file_count(), ql_m_source_record_count(), ql_m_binding_count());
    }
    const char *bases[] = {"master", "aggregate-root", "source-parent", "existing-source-prefix"};
    size_t root_total = 0;
    for (unsigned i = 0; i < 6; ++i) {
        const QL_M_Node *r = ql_m_root(i);
        CHECK(r && r->root_position == i && r->root_id == r->id && ql_m_parent(r->id) == master);
        root_total += ql_m_subtree_count(r->id);
    }
    CHECK(root_total + 1 == ql_m_node_count());
    for (size_t i = 0; i < ql_m_node_count(); ++i) {
        const QL_M_Node *n = ql_m_node_at(i);
        CHECK(ql_m_node_by_id(n->id) == n && ql_m_resolve(n->source_ref) == n);
        char *alias = malloc(strlen(n->source_ref) + 1);
        CHECK(alias != NULL);
        strcpy(alias, n->source_ref); alias[0] = 'M';
        CHECK(ql_m_resolve(alias) == n); free(alias);
        CHECK(ql_m_child_count(n->id) == n->children_count);
        size_t count = 1;
        for (size_t j = 0; j < n->children_count; ++j) {
            const QL_M_Node *c = ql_m_child_at(n->id, j);
            CHECK(c && ql_m_parent(c->id) == n && c->depth == n->depth + 1);
            count += c->subtree_count;
        }
        CHECK(count == n->subtree_count && !ql_m_child_at(n->id, n->children_count));
        CHECK(!ql_m_node_record_at(n->id, n->records_count));
        if (emit) {
            fputs("{\"kind\":\"node\",\"id\":", stdout); id(n->id);
            key("source_ref", n->source_ref); key_id("parent_id", n->parent_id); key_id("root_id", n->root_id);
            fputs(",\"root_position\":", stdout);
            if (n->root_position == QL_M_NO_ROOT) fputs("null", stdout); else printf("%u", n->root_position);
            key("local_segment", n->local_segment); key("separator", n->separator);
            printf(",\"depth\":%u,\"lexical_depth\":%u", n->depth, n->lexical_depth);
            key("lexical_parent_source_ref", n->lexical_parent_source_ref);
            key("parent_basis", bases[n->parent_basis]);
            key("structural_status", n->structural_status == QL_M_MASTER_INDEX ? "master-index" : "source-declared");
            printf(",\"aggregate\":%s,\"source_parent_refs\":%s,\"names\":%s,\"aliases\":%s,\"children\":[",
                   boolean(n->aggregate), n->source_parent_refs_json, n->names_json, n->aliases_json);
            for (size_t j = 0; j < n->children_count; ++j) { if (j) putchar(','); id(ql_m_child_at(n->id, j)->id); }
            fputs("],\"records\":[", stdout);
        }
        for (size_t j = 0; j < n->records_count; ++j) {
            size_t index = ql_m_node_record_index(n->id, j);
            CHECK(ql_m_node_record_at(n->id, j) == ql_m_source_record_at(index));
            if (emit) { if (j) putchar(','); printf("%zu", index); }
        }
        if (emit) printf("],\"subtree_count\":%u}\n", n->subtree_count);
    }
    for (size_t i = 0; i < ql_m_source_file_count(); ++i) {
        const QL_M_SourceFile *f = ql_m_source_file_at(i);
        if (emit) {
            fputs("{\"kind\":\"file\"", stdout); key("path", f->path); key("git_blob", f->git_blob);
            key("sha256", f->sha256); key("record_class", f->record_class);
            printf(",\"bytes\":%" PRIu64 "}\n", f->bytes);
        }
    }
    for (size_t i = 0; i < ql_m_source_record_count(); ++i) {
        const QL_M_SourceRecord *r = ql_m_source_record_at(i);
        CHECK(ql_m_source_file_at(r->file_index));
        if (emit) {
            printf("{\"kind\":\"record\",\"file\":%u,\"record_index\":%u", r->file_index, r->record_index);
            key("payload_sha256", r->payload_sha256);
            printf(",\"property_keys\":%s}\n", r->property_keys_json);
        }
    }
    const char *orientations[] = {"directed", "undirected", "unspecified"};
    for (size_t i = 0; i < ql_m_relation_count(); ++i) {
        const QL_M_Relation *r = ql_m_relation_at(i);
        CHECK(ql_m_relation_by_id(r->id) == r && ql_m_relation_resolve(r->relation_ref) == r);
        CHECK(!r->from_id || ql_m_resolve(r->from_ref) == ql_m_node_by_id(r->from_id));
        CHECK(!r->to_id || ql_m_resolve(r->to_ref) == ql_m_node_by_id(r->to_id));
        CHECK(ql_m_source_record_at(r->record));
        if (emit) {
            fputs("{\"kind\":\"relation\",\"id\":", stdout); id(r->id);
            key("relation_ref", r->relation_ref); key("class", "bimba-source"); key("source_kind", r->source_kind);
            key("from_ref", r->from_ref); key("to_ref", r->to_ref);
            key_id("from_id", r->from_id); key_id("to_id", r->to_id);
            key("orientation", orientations[r->orientation]);
            printf(",\"cross_m\":%s,\"record\":%u}\n", boolean(r->cross_m), r->record);
        }
    }
    const char *dispositions[] = {"COORDINATE-BOUND", "CROSS-COORDINATE", "INFRASTRUCTURAL"};
    for (size_t i = 0; i < ql_m_binding_count(); ++i) {
        const QL_M_Binding *b = ql_m_binding_at(i);
        CHECK(ql_m_binding_valid(b));
        if (emit) {
            fputs("{\"kind\":\"binding\"", stdout); key("identity", b->identity); key("module", b->module);
            key("disposition", dispositions[b->disposition]); key_id("coordinate_id", b->coordinate_id);
            key_id("relation_id", b->relation_id); key("readiness", b->readiness == QL_M_UNBOUND ? "unbound" : "structural-index-only");
            key("evidence", b->evidence); puts("}");
        }
    }
    QL_M_Binding binding = *ql_m_binding_at(0);
    binding.coordinate_id = 0; CHECK(!ql_m_binding_valid(&binding));
    binding = *ql_m_binding_at(0); binding.readiness = (QL_M_Readiness)256; CHECK(!ql_m_binding_valid(&binding));
    binding = *ql_m_binding_at(0); binding.disposition = (QL_M_Disposition)256; CHECK(!ql_m_binding_valid(&binding));
    binding = *ql_m_binding_at(0); binding.evidence = NULL; CHECK(!ql_m_binding_valid(&binding));
    binding.readiness = QL_M_UNBOUND; CHECK(ql_m_binding_valid(&binding));
    binding.coordinate_id = 0; binding.relation_id = ql_m_relation_at(0)->id; binding.disposition = QL_M_CROSS_COORDINATE;
    CHECK(ql_m_binding_valid(&binding)); binding.relation_id = 0; CHECK(!ql_m_binding_valid(&binding));
    binding.disposition = QL_M_INFRASTRUCTURAL; CHECK(ql_m_binding_valid(&binding));
    CHECK(ql_m_resolve("#3-2-1") != NULL); /* no computational binding is required */
    CHECK(!ferror(stdout));
    fprintf(stderr, "K2 native registry: %zu checks passed; %zu nodes; %zu source relations\n", checks, ql_m_node_count(), ql_m_relation_count());
    return 0;
}
