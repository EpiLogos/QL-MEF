/**
 * anuttara_language.c — Anuttara Coordinate-Language Registry bodies.
 *
 * The 109 M0-alphabet rows are GENERATED from the canonical dataset
 * (Idea/Bimba/Map/datasets/anuttara-deep/anuttara-language-map.md) via
 * .codex/scripts/gen-anuttara-language-registry.mjs into
 * anuttara_language_registry.inc. The 19-entry QL closure (7 psychoid bases
 * + 12 coordinate types per M0-ARCHITECTURE §11) is hand-authored here:
 * it is M1-closure law, not a dataset row. 109 + 19 = 128, the M0/M1
 * typed-calculus reach. The cardinality is carried by the generated count,
 * not pinned in doctrine — the alphabet may mature.
 */

#include "anuttara_language.h"
#include <string.h>

#include "anuttara_language_registry.inc"

const AnuttaraLanguageEntry ANUTTARA_LANGUAGE_REGISTRY[] = {
    ANUTTARA_LANGUAGE_M0_ALPHABET_ROWS

    /* --- QL closure: 7 psychoid bases (R-distribution spine, M1 register).
     * Order mirrors R_FACTOR_ROUTE_TABLE: O#, X#, N#, M#, #, Siva, Shakti.
     * The glyphs "(#)" and "(@#)" in the 1R..4R spine signatures resolve
     * here through symbol identity (Siva pole, Shakti seed). */
    { .coordinate = "O#", .symbol = "O#",
      .name = "Paramasiva Base (psychoid, anchors M0-(4.0/1))",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },
    { .coordinate = "X#", .symbol = "X#",
      .name = "Parashakti Base (psychoid, anchors M0-(4.0/1/2))",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },
    { .coordinate = "N#", .symbol = "N#",
      .name = "Spanda Base (psychoid, anchors M0-(4.0/1/2/3))",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },
    { .coordinate = "M#", .symbol = "M#",
      .name = "Mahamaya Base (psychoid, anchors M0-4.4.0-(4.4/5))",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },
    { .coordinate = "#", .symbol = "#",
      .name = "Nara Base (psychoid, anchors M0-(4.5/0))",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },
    { .coordinate = "Siva", .symbol = "(#)",
      .name = "Siva pole (psychoid, anchors M0-5-(0/1))",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },
    { .coordinate = "Shakti", .symbol = "(@#)",
      .name = "Shakti seed (psychoid, anchors M0-5-(5/0); the (@#) turn)",
      .kind = ANUTTARA_ENTRY_PSYCHOID_BASE, .packed = 0xFFFFu },

    /* --- QL closure: 12 coordinate types (six families + inversions).
     * QL theory is implicit across all twelve; P/P' and L/L' are especially
     * load-bearing (functional-position substrate; MEF lens-system). */
    { .coordinate = "P", .symbol = "P",
      .name = "Position family (functional semantics)",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "S", .symbol = "S",
      .name = "Stack family (technology layers)",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "T", .symbol = "T",
      .name = "Thought family (artifacts/cognition)",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "M", .symbol = "M",
      .name = "Subsystem family (consciousness domains)",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "L", .symbol = "L",
      .name = "Lens family (epistemic modes)",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "C", .symbol = "C",
      .name = "Category family (ontological foundation)",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "P'", .symbol = "P'",
      .name = "Position family, inverted phase",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "S'", .symbol = "S'",
      .name = "Stack family, inverted phase",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "T'", .symbol = "T'",
      .name = "Thought family, inverted phase",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "M'", .symbol = "M'",
      .name = "Subsystem family, inverted phase",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "L'", .symbol = "L'",
      .name = "Lens family, inverted phase",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
    { .coordinate = "C'", .symbol = "C'",
      .name = "Category family, inverted phase",
      .kind = ANUTTARA_ENTRY_COORDINATE_TYPE, .packed = 0xFFFFu },
};

const size_t ANUTTARA_LANGUAGE_REGISTRY_COUNT =
    sizeof(ANUTTARA_LANGUAGE_REGISTRY) / sizeof(ANUTTARA_LANGUAGE_REGISTRY[0]);

_Static_assert(
    sizeof(ANUTTARA_LANGUAGE_REGISTRY) / sizeof(ANUTTARA_LANGUAGE_REGISTRY[0])
        == ANUTTARA_LANGUAGE_M0_ALPHABET_COUNT + ANUTTARA_LANGUAGE_QL_CLOSURE_COUNT,
    "registry = M0 alphabet + 19 QL closure (M0-ARCHITECTURE section 11)");

const AnuttaraLanguageEntry* anuttara_language_lookup(const char* coordinate) {
    if (!coordinate) return NULL;
    for (size_t i = 0; i < ANUTTARA_LANGUAGE_REGISTRY_COUNT; i++) {
        if (strcmp(ANUTTARA_LANGUAGE_REGISTRY[i].coordinate, coordinate) == 0) {
            return &ANUTTARA_LANGUAGE_REGISTRY[i];
        }
    }
    return NULL;
}

const AnuttaraLanguageEntry* anuttara_language_lookup_packed(uint16_t packed) {
    if (packed == 0xFFFFu) return NULL;
    for (size_t i = 0; i < ANUTTARA_LANGUAGE_REGISTRY_COUNT; i++) {
        if (ANUTTARA_LANGUAGE_REGISTRY[i].packed == packed) {
            return &ANUTTARA_LANGUAGE_REGISTRY[i];
        }
    }
    return NULL;
}

bool anuttara_language_is_member(const char* coordinate_or_symbol) {
    if (!coordinate_or_symbol || coordinate_or_symbol[0] == '\0') return false;
    if (anuttara_language_lookup(coordinate_or_symbol)) return true;
    for (size_t i = 0; i < ANUTTARA_LANGUAGE_REGISTRY_COUNT; i++) {
        const char* symbol = ANUTTARA_LANGUAGE_REGISTRY[i].symbol;
        if (symbol && strcmp(symbol, coordinate_or_symbol) == 0) {
            return true;
        }
    }
    return false;
}

/* ===================================================================
 * M0_IDENTITY_CHAINS — Law-3 equational theory, first landing.
 *
 * Principle-triad + meta chains (Tranche 1.10 brief, verbatim lineage):
 *   M0-2-9-0 Love/Peace "(∞x∞)x(R#/##)" chains to M0-1 Brimming Void
 *     "(0/1)/00x00 = (∞x∞)x(R#/##)" and M0-0-0 Ultimate Mystery.
 *   M0-2-9-1 Truth "## = @ = (0/1)-(00)-00" chains to the ## Primordial
 *     Matrix and the M0-0 derivations (structure's lineage to void).
 *   M0-2-9-2 Openness "#R = @ = (7-8-9-(0/1)/O#-X#-N#)" chains via the
 *     7-8-9 spine and the Non-Dual Binary (0/1).
 * Conjugate chains follow each virtue's own base-spine signature in
 * VIRTUE_LUT (m0.c) through M0-3/M0-4 to the M0-0/M0-1 roots — longer
 * chains but no less real.
 * =================================================================== */

const M0IdentityChain M0_IDENTITY_CHAINS[] = {
    { .coordinate = "M0-2-9-0", /* R#/## Love/Peace (meta-virtue) */
      .links = { "M0-1-(0/1)", "M0-1", "M0-0-0", "M0-0" },
      .link_count = 4, .is_principle = 1 },
    { .coordinate = "M0-2-9-1", /* ## Truth */
      .links = { "M0-(4.5/0)-0", "M0-1-(0/1)", "M0-0-1", "M0-0" },
      .link_count = 4, .is_principle = 1 },
    { .coordinate = "M0-2-9-2", /* #R Openness (7-8-9 spine) */
      .links = { "M0-2-9", "M0-3-4", "O#", "X#", "N#", "M0-1-(0/1)", "M0-0" },
      .link_count = 7, .is_principle = 1 },
    { .coordinate = "M0-2-9-3", /* 0R Joy "(9-O#-X#-N#)" */
      .links = { "M0-2-9", "O#", "X#", "N#", "M0-1-(0/1)", "M0-0" },
      .link_count = 6, .is_principle = 0 },
    { .coordinate = "M0-2-9-4", /* 1R Goodness "(O#-X#-N#-M#-#-(#))" */
      .links = { "O#", "X#", "N#", "M#", "#", "Siva", "M0-1-(0/1)", "M0-0" },
      .link_count = 8, .is_principle = 0 },
    { .coordinate = "M0-2-9-5", /* 2R Beauty "(X#-N#-M#-#-(#)-(@#))" */
      .links = { "X#", "N#", "M#", "#", "Siva", "Shakti", "M0-1-(0/1)", "M0-0" },
      .link_count = 8, .is_principle = 0 },
    { .coordinate = "M0-2-9-6", /* 3R Life "((@#)-(#)-#-M#-N#-X#)" */
      .links = { "Shakti", "Siva", "#", "M#", "N#", "X#", "M0-1-(0/1)", "M0-0" },
      .link_count = 8, .is_principle = 0 },
    { .coordinate = "M0-2-9-7", /* 4R Wisdom "((#)-#-M#-N#-X#-O#)" */
      .links = { "Siva", "#", "M#", "N#", "X#", "O#", "M0-1-(0/1)", "M0-0" },
      .link_count = 8, .is_principle = 0 },
    { .coordinate = "M0-2-9-8", /* 5R Reality "(##)" — return-to-matrix */
      .links = { "M0-(4.5/0)-0", "M0-0" },
      .link_count = 2, .is_principle = 0 },
};

const size_t M0_IDENTITY_CHAINS_COUNT =
    sizeof(M0_IDENTITY_CHAINS) / sizeof(M0_IDENTITY_CHAINS[0]);

const M0IdentityChain* m0_identity_chain_find(const char* coordinate) {
    if (!coordinate) return NULL;
    for (size_t i = 0; i < M0_IDENTITY_CHAINS_COUNT; i++) {
        if (strcmp(M0_IDENTITY_CHAINS[i].coordinate, coordinate) == 0) {
            return &M0_IDENTITY_CHAINS[i];
        }
    }
    return NULL;
}

/* ===================================================================
 * Law-3 =-chain equivalence classes (Tranche 1.14a).
 *
 * Built lazily from ANUTTARA_LANGUAGE_REGISTRY c_1_symbol chains: every
 * symbol containing " = " contributes one class whose members are the
 * chain segments. Members appearing in more than one class are AMBIGUOUS
 * and never rewrite (the Presence mark `@` rides most virtue chains).
 * The M0_IDENTITY_CHAINS coordinate walks (above) are the same Law-3
 * theory read as backing-walks; these classes are its rewrite face.
 * =================================================================== */

#define M0_IDC_MAX_CLASSES 160
#define M0_IDC_MAX_MEMBERS 8
#define M0_IDC_POOL_SIZE 16384

typedef struct {
    const char* members[M0_IDC_MAX_MEMBERS];
    uint8_t member_count;
} M0IdentityClass;

static M0IdentityClass idc_classes[M0_IDC_MAX_CLASSES];
static size_t idc_class_count = 0;
static char idc_pool[M0_IDC_POOL_SIZE];
static size_t idc_pool_used = 0;
static int idc_built = 0;

static const char* idc_intern(const char* start, size_t len) {
    /* Trim surrounding whitespace. */
    while (len > 0 && (start[0] == ' ' || start[0] == '\t')) { start++; len--; }
    while (len > 0 && (start[len - 1] == ' ' || start[len - 1] == '\t')) len--;
    if (len == 0 || idc_pool_used + len + 1 > M0_IDC_POOL_SIZE) return NULL;
    char* dst = &idc_pool[idc_pool_used];
    memcpy(dst, start, len);
    dst[len] = '\0';
    idc_pool_used += len + 1;
    return dst;
}

static void idc_build(void) {
    if (idc_built) return;
    idc_built = 1;
    for (size_t i = 0; i < ANUTTARA_LANGUAGE_REGISTRY_COUNT; i++) {
        const char* symbol = ANUTTARA_LANGUAGE_REGISTRY[i].symbol;
        if (!symbol || !strstr(symbol, " = ")) continue;
        if (idc_class_count >= M0_IDC_MAX_CLASSES) break;
        M0IdentityClass* cls = &idc_classes[idc_class_count];
        cls->member_count = 0;
        const char* cursor = symbol;
        while (cursor && cls->member_count < M0_IDC_MAX_MEMBERS) {
            const char* next = strstr(cursor, " = ");
            const size_t seg_len = next ? (size_t)(next - cursor) : strlen(cursor);
            const char* member = idc_intern(cursor, seg_len);
            if (member && member[0] != '\0') {
                cls->members[cls->member_count++] = member;
            }
            cursor = next ? next + 3 : NULL;
        }
        if (cls->member_count >= 2) idc_class_count++;
    }
}

static int idc_member_occurrences(const char* term) {
    int hits = 0;
    for (size_t c = 0; c < idc_class_count; c++) {
        for (uint8_t m = 0; m < idc_classes[c].member_count; m++) {
            if (strcmp(idc_classes[c].members[m], term) == 0) {
                hits++;
                break; /* count each class once */
            }
        }
    }
    return hits;
}

int m0_identity_class_find(const char* term) {
    if (!term || term[0] == '\0') return -1;
    idc_build();
    /* A term that is itself some registry entry's COMPLETE symbol is
     * canonical in its own right (`0/1` is the Non-Dual Binary, not a
     * rewritable alias) — chains only rewrite their proper segments. */
    for (size_t i = 0; i < ANUTTARA_LANGUAGE_REGISTRY_COUNT; i++) {
        const char* symbol = ANUTTARA_LANGUAGE_REGISTRY[i].symbol;
        if (symbol && strcmp(symbol, term) == 0) return -1;
    }
    if (idc_member_occurrences(term) != 1) return -1; /* unknown or ambiguous */
    for (size_t c = 0; c < idc_class_count; c++) {
        for (uint8_t m = 0; m < idc_classes[c].member_count; m++) {
            if (strcmp(idc_classes[c].members[m], term) == 0) return (int)c;
        }
    }
    return -1;
}

const char* m0_identity_class_canonical(int class_id) {
    idc_build();
    if (class_id < 0 || (size_t)class_id >= idc_class_count) return NULL;
    return idc_classes[class_id].members[0];
}

size_t m0_identity_class_count(void) {
    idc_build();
    return idc_class_count;
}
