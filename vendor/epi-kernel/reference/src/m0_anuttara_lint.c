/*
 * m0_anuttara_lint.c -- M0 Anuttara hardcoded-relation audit loop.
 *
 * This utility scans source files for numeric literals that look like hidden
 * relation knobs: const/default values, business-logic magic numbers, and
 * hardcoded timeout/retry/threshold settings. It emits review candidates only;
 * schema migration remains a manual tunable.toml authoring pass.
 */

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

#define M0_LINT_PATH_MAX 512
#define M0_LINT_LITERAL_MAX 64
#define M0_LINT_CONTEXT_MAX 256
#define M0_LINT_REASON_MAX 160
#define M0_LINT_REPORT_GROUP_LIMIT 25u

typedef enum M0LintClassification {
    M0_LINT_GENUINELY_TUNABLE = 1,
    M0_LINT_STRUCTURAL_INVARIANT = 2,
    M0_LINT_REVIEW_REQUIRED = 3
} M0LintClassification;

typedef struct M0LintCandidate {
    char path[M0_LINT_PATH_MAX];
    size_t line;
    char literal[M0_LINT_LITERAL_MAX];
    char context[M0_LINT_CONTEXT_MAX];
    char reason[M0_LINT_REASON_MAX];
    M0LintClassification classification;
} M0LintCandidate;

typedef struct M0LintCandidateSet {
    M0LintCandidate* items;
    size_t count;
    size_t capacity;
} M0LintCandidateSet;

static int m0_lint_scan_path(const char* path, M0LintCandidateSet* out);

void m0_lint_candidate_set_init(M0LintCandidateSet* set) {
    if (!set) return;
    set->items = NULL;
    set->count = 0u;
    set->capacity = 0u;
}

void m0_lint_candidate_set_free(M0LintCandidateSet* set) {
    if (!set) return;
    free(set->items);
    set->items = NULL;
    set->count = 0u;
    set->capacity = 0u;
}

static void copy_truncated(char* dst, size_t dst_len, const char* src) {
    if (!dst || dst_len == 0u) return;
    if (!src) {
        dst[0] = '\0';
        return;
    }
    (void)snprintf(dst, dst_len, "%s", src);
}

static int reserve_candidate(M0LintCandidateSet* set) {
    if (set->count < set->capacity) return 0;

    const size_t new_capacity = set->capacity == 0u ? 32u : set->capacity * 2u;
    M0LintCandidate* next =
        (M0LintCandidate*)realloc(set->items, new_capacity * sizeof(*next));
    if (!next) return -1;

    set->items = next;
    set->capacity = new_capacity;
    return 0;
}

static int add_candidate(
    M0LintCandidateSet* set,
    const char* path,
    size_t line,
    const char* literal,
    const char* context,
    M0LintClassification classification,
    const char* reason
) {
    if (!set || !path || !literal || !context || !reason) return -1;
    if (reserve_candidate(set) != 0) return -1;

    M0LintCandidate* item = &set->items[set->count++];
    memset(item, 0, sizeof(*item));
    copy_truncated(item->path, sizeof(item->path), path);
    item->line = line;
    copy_truncated(item->literal, sizeof(item->literal), literal);
    copy_truncated(item->context, sizeof(item->context), context);
    copy_truncated(item->reason, sizeof(item->reason), reason);
    item->classification = classification;
    return 0;
}

static void lower_copy(char* dst, size_t dst_len, const char* src) {
    size_t i = 0u;
    if (!dst || dst_len == 0u) return;
    if (!src) {
        dst[0] = '\0';
        return;
    }
    for (; src[i] != '\0' && i + 1u < dst_len; i++) {
        dst[i] = (char)tolower((unsigned char)src[i]);
    }
    dst[i] = '\0';
}

static bool contains_any(const char* s, const char* const* needles, size_t count) {
    for (size_t i = 0u; i < count; i++) {
        if (strstr(s, needles[i]) != NULL) return true;
    }
    return false;
}

static bool line_has_assignment_or_definition(const char* lower) {
    return strstr(lower, "const") != NULL ||
           strstr(lower, "default") != NULL ||
           strstr(lower, "#define") != NULL ||
           strchr(lower, '=') != NULL;
}

static bool line_has_business_logic(const char* lower) {
    return strstr(lower, "if ") != NULL ||
           strstr(lower, "if(") != NULL ||
           strstr(lower, "while ") != NULL ||
           strstr(lower, "while(") != NULL ||
           strstr(lower, "for ") != NULL ||
           strstr(lower, "for(") != NULL ||
           strstr(lower, "return ") != NULL ||
           strstr(lower, ">") != NULL ||
           strstr(lower, "<") != NULL ||
           strstr(lower, "==") != NULL ||
           strstr(lower, "!=") != NULL;
}

static bool literal_is_ignored_small_value(const char* literal) {
    char buf[M0_LINT_LITERAL_MAX];
    size_t n = 0u;
    for (; literal[n] != '\0' && n + 1u < sizeof(buf); n++) {
        buf[n] = (char)tolower((unsigned char)literal[n]);
    }
    buf[n] = '\0';

    while (n > 0u) {
        const char c = buf[n - 1u];
        if (c == 'u' || c == 'l' || c == 'f') {
            buf[--n] = '\0';
        } else {
            break;
        }
    }

    return strcmp(buf, "0") == 0 ||
           strcmp(buf, "0.0") == 0 ||
           strcmp(buf, "1") == 0 ||
           strcmp(buf, "1.0") == 0;
}

static M0LintClassification classify_line(
    const char* lower,
    char* reason,
    size_t reason_len
) {
    static const char* const structural_terms[] = {
        "ring", "count", "size", "capacity", "dimension", "axis",
        "family", "position", "ql_", "bytes", "byte", "mask", "flag", "bit"
    };
    static const char* const strong_structural_terms[] = {
        "_static_assert", "static_assert", "sizeof", "enum", "hc_interval", "hc_rel"
    };
    static const char* const tunable_terms[] = {
        "retry", "retries", "threshold", "limit", "window",
        "frequency", "ceiling", "weight", "backoff"
    };
    const bool has_timeout_term =
        strstr(lower, "timeout") != NULL ||
        strstr(lower, "millis") != NULL ||
        strstr(lower, "duration_ms") != NULL ||
        strstr(lower, "delay_ms") != NULL;
    const bool has_default_value_term =
        strstr(lower, "default_") != NULL ||
        strstr(lower, "_default") != NULL ||
        (strstr(lower, "default") != NULL && strchr(lower, '=') != NULL);

    if (contains_any(
            lower,
            strong_structural_terms,
            sizeof(strong_structural_terms) / sizeof(*strong_structural_terms)
        )) {
        copy_truncated(
            reason,
            reason_len,
            "structural invariant shape; review before declaring tunable"
        );
        return M0_LINT_STRUCTURAL_INVARIANT;
    }

    if (line_has_business_logic(lower) &&
        !line_has_assignment_or_definition(lower) &&
        !has_timeout_term &&
        strstr(lower, "delay") == NULL) {
        copy_truncated(reason, reason_len, "business-logic magic number candidate");
        return M0_LINT_REVIEW_REQUIRED;
    }

    if (has_timeout_term) {
        copy_truncated(reason, reason_len, "hardcoded timeout/duration candidate");
        return M0_LINT_GENUINELY_TUNABLE;
    }

    if (has_default_value_term) {
        copy_truncated(reason, reason_len, "default value candidate");
        return M0_LINT_GENUINELY_TUNABLE;
    }

    if (contains_any(lower, tunable_terms, sizeof(tunable_terms) / sizeof(*tunable_terms))) {
        copy_truncated(reason, reason_len, "policy threshold/limit candidate");
        return M0_LINT_GENUINELY_TUNABLE;
    }

    if (contains_any(lower, structural_terms, sizeof(structural_terms) / sizeof(*structural_terms))) {
        copy_truncated(
            reason,
            reason_len,
            "structural invariant shape; review before declaring tunable"
        );
        return M0_LINT_STRUCTURAL_INVARIANT;
    }

    if (line_has_assignment_or_definition(lower)) {
        copy_truncated(reason, reason_len, "const/default-shaped numeric literal");
        return M0_LINT_REVIEW_REQUIRED;
    }

    if (line_has_business_logic(lower)) {
        copy_truncated(reason, reason_len, "business-logic magic number candidate");
        return M0_LINT_REVIEW_REQUIRED;
    }

    copy_truncated(reason, reason_len, "numeric literal requires manual review");
    return M0_LINT_REVIEW_REQUIRED;
}

static void strip_comments(const char* input, char* output, size_t output_len, bool* in_block) {
    size_t o = 0u;
    bool in_string = false;
    bool in_char = false;
    bool escaped = false;

    for (size_t i = 0u; input[i] != '\0' && o + 1u < output_len; i++) {
        const char c = input[i];
        const char next = input[i + 1u];

        if (*in_block) {
            if (c == '*' && next == '/') {
                *in_block = false;
                i++;
            }
            output[o++] = ' ';
            continue;
        }

        if (!in_string && !in_char && c == '/' && next == '/') break;
        if (!in_string && !in_char && c == '/' && next == '*') {
            *in_block = true;
            output[o++] = ' ';
            i++;
            continue;
        }

        if (!in_char && c == '"' && !escaped) in_string = !in_string;
        if (!in_string && c == '\'' && !escaped) in_char = !in_char;

        output[o++] = (in_string || in_char) ? ' ' : c;
        escaped = (c == '\\' && !escaped);
        if (c != '\\') escaped = false;
    }
    output[o] = '\0';
}

static bool is_numeric_start(const char* line, size_t i) {
    if (!isdigit((unsigned char)line[i])) return false;
    if (i > 0u) {
        const char prev = line[i - 1u];
        if (isalnum((unsigned char)prev) || prev == '_') return false;
    }
    return true;
}

static size_t read_numeric_literal(const char* line, size_t start, char* out, size_t out_len) {
    size_t i = start;
    size_t o = 0u;

    if (line[i] == '0' && (line[i + 1u] == 'x' || line[i + 1u] == 'X')) {
        while (isxdigit((unsigned char)line[i]) || line[i] == 'x' || line[i] == 'X' ||
               line[i] == 'u' || line[i] == 'U' || line[i] == 'l' || line[i] == 'L') {
            if (o + 1u < out_len) out[o++] = line[i];
            i++;
        }
    } else {
        while (isdigit((unsigned char)line[i]) || line[i] == '.' ||
               line[i] == 'e' || line[i] == 'E' || line[i] == '+' ||
               line[i] == '-' || line[i] == 'f' || line[i] == 'F' ||
               line[i] == 'u' || line[i] == 'U' || line[i] == 'l' ||
               line[i] == 'L') {
            if ((line[i] == '+' || line[i] == '-') && i > start &&
                line[i - 1u] != 'e' && line[i - 1u] != 'E') {
                break;
            }
            if (o + 1u < out_len) out[o++] = line[i];
            i++;
        }
    }

    out[o] = '\0';
    return i;
}

static bool line_should_report(const char* lower) {
    return line_has_assignment_or_definition(lower) || line_has_business_logic(lower);
}

static bool literal_is_bracket_index(const char* line, size_t start, size_t end) {
    size_t before = start;
    while (before > 0u && isspace((unsigned char)line[before - 1u])) before--;
    size_t after = end;
    while (line[after] != '\0' && isspace((unsigned char)line[after])) after++;
    return before > 0u && line[before - 1u] == '[' && line[after] == ']';
}

int m0_lint_scan_file(const char* path, M0LintCandidateSet* out) {
    if (!path || !out) return -1;

    FILE* f = fopen(path, "r");
    if (!f) return -1;

    char raw[1024];
    char clean[1024];
    bool in_block_comment = false;
    size_t line_no = 0u;
    int status = 0;

    while (fgets(raw, sizeof(raw), f) != NULL) {
        line_no++;
        strip_comments(raw, clean, sizeof(clean), &in_block_comment);

        char lower[sizeof(clean)];
        lower_copy(lower, sizeof(lower), clean);
        if (!line_should_report(lower)) continue;

        for (size_t i = 0u; clean[i] != '\0'; i++) {
            if (!is_numeric_start(clean, i)) continue;

            char literal[M0_LINT_LITERAL_MAX];
            const size_t start = i;
            const size_t next = read_numeric_literal(clean, i, literal, sizeof(literal));
            i = next == 0u ? i : next - 1u;

            if (literal[0] == '\0' || literal_is_ignored_small_value(literal)) continue;

            char reason[M0_LINT_REASON_MAX];
            M0LintClassification classification = M0_LINT_REVIEW_REQUIRED;
            if (literal_is_bracket_index(clean, start, next)) {
                classification = M0_LINT_STRUCTURAL_INVARIANT;
                copy_truncated(
                    reason,
                    sizeof(reason),
                    "array index/cardinality shape; review before declaring tunable"
                );
            } else {
                classification = classify_line(lower, reason, sizeof(reason));
            }
            if (add_candidate(out, path, line_no, literal, clean, classification, reason) != 0) {
                status = -1;
                break;
            }
        }
        if (status != 0) break;
    }

    if (ferror(f)) status = -1;
    if (fclose(f) != 0) status = -1;
    return status;
}

static const char* classification_heading(M0LintClassification classification) {
    switch (classification) {
        case M0_LINT_GENUINELY_TUNABLE: return "Genuinely Tunable Candidates";
        case M0_LINT_STRUCTURAL_INVARIANT: return "Structural Invariant Candidates";
        case M0_LINT_REVIEW_REQUIRED: return "Manual Review Candidates";
        default: return "Unknown Candidates";
    }
}

static size_t count_classification(
    const M0LintCandidateSet* set,
    M0LintClassification classification
) {
    size_t count = 0u;
    for (size_t i = 0u; i < set->count; i++) {
        if (set->items[i].classification == classification) count++;
    }
    return count;
}

static void write_candidate_group(
    FILE* f,
    const M0LintCandidateSet* set,
    M0LintClassification classification
) {
    const size_t total = count_classification(set, classification);
    (void)fprintf(f, "## %s\n\n", classification_heading(classification));
    if (total > M0_LINT_REPORT_GROUP_LIMIT) {
        (void)fprintf(
            f,
            "Showing first %u of %zu candidates in scan order.\n\n",
            (unsigned)M0_LINT_REPORT_GROUP_LIMIT,
            total
        );
    }

    bool any = false;
    size_t shown = 0u;
    for (size_t i = 0u; i < set->count; i++) {
        const M0LintCandidate* item = &set->items[i];
        if (item->classification != classification) continue;
        if (shown >= M0_LINT_REPORT_GROUP_LIMIT) break;
        any = true;
        shown++;
        (void)fprintf(
            f,
            "- `%s` at `%s:%zu` — %s. Context: `%s`\n",
            item->literal,
            item->path,
            item->line,
            item->reason,
            item->context
        );
    }

    if (!any) (void)fprintf(f, "- None detected.\n");
    if (total > shown) {
        (void)fprintf(f, "\n_Omitted %zu additional candidates in this group._\n", total - shown);
    }
    (void)fprintf(f, "\n");
}

static void write_triage_queue(FILE* f, const M0LintCandidateSet* set) {
    (void)fprintf(f, "## Cycle Audit Triage Queue\n\n");
    (void)fprintf(
        f,
        "First-pass queue capped at %u candidates. Treat each row as a manual "
        "decision point before authoring `tunable.toml`.\n\n",
        (unsigned)M0_LINT_REPORT_GROUP_LIMIT
    );

    size_t shown = 0u;
    for (size_t i = 0u; i < set->count && shown < M0_LINT_REPORT_GROUP_LIMIT; i++) {
        const M0LintCandidate* item = &set->items[i];
        if (item->classification != M0_LINT_GENUINELY_TUNABLE) continue;
        shown++;
        (void)fprintf(
            f,
            "%zu. `%s` at `%s:%zu` — proposed action: review as a tunable knob; "
            "reject if this is coordinate cardinality or dataset law.\n",
            shown,
            item->literal,
            item->path,
            item->line
        );
    }

    for (size_t i = 0u; i < set->count && shown < M0_LINT_REPORT_GROUP_LIMIT; i++) {
        const M0LintCandidate* item = &set->items[i];
        if (item->classification != M0_LINT_REVIEW_REQUIRED) continue;
        shown++;
        (void)fprintf(
            f,
            "%zu. `%s` at `%s:%zu` — proposed action: inspect before promotion; "
            "only author `tunable.toml` if this proves to be runtime policy rather "
            "than relation law.\n",
            shown,
            item->literal,
            item->path,
            item->line
        );
    }

    if (shown == 0u) {
        (void)fprintf(f, "No tunable or manual-review candidates detected in this scan.\n");
    }
    (void)fprintf(f, "\n");
}

int m0_lint_write_markdown_report(const char* output_path, const M0LintCandidateSet* set) {
    if (!output_path || !set) return -1;

    FILE* f = fopen(output_path, "w");
    if (!f) return -1;

    const size_t tunable = count_classification(set, M0_LINT_GENUINELY_TUNABLE);
    const size_t structural = count_classification(set, M0_LINT_STRUCTURAL_INVARIANT);
    const size_t review = count_classification(set, M0_LINT_REVIEW_REQUIRED);

    (void)fprintf(f, "# M0 Anuttara Tunable Audit Report\n\n");
    (void)fprintf(
        f,
        "This generated audit lists candidate hardcoded relations for manual "
        "`tunable.toml` review. It does not promote values into schema law.\n\n"
    );
    (void)fprintf(f, "## Summary\n\n");
    (void)fprintf(f, "- Total candidates: %zu\n", set->count);
    (void)fprintf(f, "- Genuinely tunable candidates: %zu\n", tunable);
    (void)fprintf(f, "- Structural invariant candidates: %zu\n", structural);
    (void)fprintf(f, "- Manual review candidates: %zu\n\n", review);

    write_triage_queue(f, set);
    write_candidate_group(f, set, M0_LINT_GENUINELY_TUNABLE);
    write_candidate_group(f, set, M0_LINT_STRUCTURAL_INVARIANT);
    write_candidate_group(f, set, M0_LINT_REVIEW_REQUIRED);

    (void)fprintf(f, "## Manual Triage Guidance\n\n");
    (void)fprintf(
        f,
        "- Move only confirmed runtime-policy knobs into `tunable.toml`.\n"
        "- Keep layout sizes, enum ordinals, ring counts, masks, and coordinate "
        "cardinalities as structural invariants unless canon explicitly changes.\n"
        "- For review-required literals, inspect call-site behavior before deciding "
        "whether the value is relation law or operational policy.\n"
    );

    const int close_status = fclose(f);
    return close_status == 0 ? 0 : -1;
}

static bool has_source_extension(const char* path) {
    const char* dot = strrchr(path, '.');
    if (!dot) return false;
    return strcmp(dot, ".c") == 0 ||
           strcmp(dot, ".h") == 0 ||
           strcmp(dot, ".rs") == 0 ||
           strcmp(dot, ".toml") == 0 ||
           strcmp(dot, ".mjs") == 0 ||
           strcmp(dot, ".js") == 0 ||
           strcmp(dot, ".ts") == 0 ||
           strcmp(dot, ".tsx") == 0;
}

static int scan_directory(const char* path, M0LintCandidateSet* out) {
    DIR* dir = opendir(path);
    if (!dir) return -1;

    int status = 0;
    struct dirent* entry = NULL;
    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }

        char child[PATH_MAX];
        const int written = snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);
        if (written < 0 || (size_t)written >= sizeof(child)) {
            status = -1;
            break;
        }

        if (m0_lint_scan_path(child, out) != 0) {
            status = -1;
            break;
        }
    }

    if (closedir(dir) != 0) status = -1;
    return status;
}

static int m0_lint_scan_path(const char* path, M0LintCandidateSet* out) {
    struct stat st;
    if (stat(path, &st) != 0) return -1;

    if (S_ISDIR(st.st_mode)) return scan_directory(path, out);
    if (strstr(path, "m0_anuttara_lint.c") != NULL) return 0;
    if (!S_ISREG(st.st_mode) || !has_source_extension(path)) return 0;
    return m0_lint_scan_file(path, out);
}

#ifndef M0_ANUTTARA_LINT_NO_MAIN
int main(int argc, char** argv) {
    if (argc < 3) {
        (void)fprintf(
            stderr,
            "usage: %s <output-report.md> <source-file-or-dir> [...]\n",
            argv[0]
        );
        return 2;
    }

    M0LintCandidateSet set;
    m0_lint_candidate_set_init(&set);

    int status = 0;
    for (int i = 2; i < argc; i++) {
        if (m0_lint_scan_path(argv[i], &set) != 0) {
            (void)fprintf(stderr, "m0_anuttara_lint: failed to scan %s: %s\n", argv[i], strerror(errno));
            status = 1;
            break;
        }
    }

    if (status == 0 && m0_lint_write_markdown_report(argv[1], &set) != 0) {
        (void)fprintf(stderr, "m0_anuttara_lint: failed to write %s: %s\n", argv[1], strerror(errno));
        status = 1;
    }

    if (status == 0) {
        (void)printf(
            "m0_anuttara_lint: wrote %s with %zu candidates\n",
            argv[1],
            set.count
        );
    }

    m0_lint_candidate_set_free(&set);
    return status;
}
#endif
