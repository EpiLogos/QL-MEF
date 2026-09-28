/**
 * m0_calculus.c — Anuttara Term-Rewriting Calculus (Tranche 01.T1.13)
 *
 * Implements the chirality-typed rewrite engine:
 *   - Tokenizer for the chirality-aware alphabet
 *   - Pattern matching for 12+ rewrite rules
 *   - Reduction engine with trace capture
 *   - Interrogative exception system (Law 6)
 *   - Palindrome normal-form checker (Law 5 / 8+1=9)
 *   - 4-bit syntax-witness computation
 *
 * Seven deep laws as the calculus's law-set.
 */

#include "m0_calculus.h"
#include "anuttara_language.h"
#include "m0.h"
#include "m3.h"
#include <string.h>
#include <stdio.h>

/* ===================================================================
 * I. TOKENIZER — chirality-aware lexical analysis (Law 1)
 *
 * Order IS semantic: -0 and 0- are distinct tokens.
 * The dash mark is pentavalent (Law 7): its position determines
 * whether it's operator, subtraction, range, chirality-mark, or
 * strikethrough. The tokenizer types it by context.
 * =================================================================== */

/* Multi-character token lookup */
typedef struct {
    const char*    text;
    M0CalcTokenType type;
    uint8_t         len;
} TokenLiteral;

static const TokenLiteral TOKEN_LITERALS[] = {
    { "i²",   M0C_TOK_I_SQUARED,  3 },
    { "x//",  M0C_TOK_SUPER_MUL,  3 },
    { "?!",   M0C_TOK_COMPOUND,   2 },
    { "!?",   M0C_TOK_COMPOUND,   2 },
    { "?/!",  M0C_TOK_COMPOUND,   3 },
    { "?!/!?",M0C_TOK_COMPOUND,   5 },
    { "!?/?!",M0C_TOK_COMPOUND,   5 },
    { "00",   M0C_TOK_VOID_DOUBLE,2 },
    { "∞",    M0C_TOK_INFINITY,   3 },
    { "≠",    M0C_TOK_NEQ,        3 },
    { "R#",   M0C_TOK_TAO_R_SHARP,        2 },
    { "##",   M0C_TOK_TAO_DOUBLE_SHARP,   2 },
    { NULL,   0,                  0 },
};

/* Single-char token dispatch */
static M0CalcTokenType single_char_type(char c) {
    switch (c) {
        case '0': return M0C_TOK_VOID_SINGLE;
        case '-': return M0C_TOK_DASH;
        case '(': return M0C_TOK_FRAME_OPEN;
        case ')': return M0C_TOK_FRAME_CLOSE;
        case '+': return M0C_TOK_PLUS;
        case 'x': return M0C_TOK_MUL;
        case '/': return M0C_TOK_DIV;
        case '=': return M0C_TOK_EQ;
        case '!': return M0C_TOK_BANG;
        case '?': return M0C_TOK_QUEST;
        case '@': return M0C_TOK_AT;
        case '#': return M0C_TOK_HASH;
        case 'R': return M0C_TOK_R;
        case '9': return M0C_TOK_NINE;
        case '%': return M0C_TOK_PERCENT;
        case '2': case '3': case '4':
        case '5': case '6': case '7':
        case '8':
            return M0C_TOK_NUMBER;
        default:  return M0C_TOK_UNKNOWN;
    }
}

/* Determine chirality of a token in context.
 *   chirality 0 = neutral (self-symmetric)
 *   chirality 1 = left-heavy (dash on left: -X)
 *   chirality 2 = right-heavy (dash on right: X-)
 */
static uint8_t token_chirality(M0CalcTokenType type, const char* text, uint8_t len) {
    switch (type) {
        case M0C_TOK_DASH:
            /* A standalone dash is neutral by itself; chirality
             * is determined by its neighbor context in the parser. */
            return 0;
        case M0C_TOK_VOID_SINGLE:
            /* Check if adjacent to dash for -0 vs 0- */
            return 0; /* determined positionally in tokenization */
        case M0C_TOK_COMPOUND:
            if (len >= 2) {
                if (text[0] == '?' && text[1] == '!') return 1; /* ?! left-heavy */
                if (text[0] == '!' && text[1] == '?') return 2; /* !? right-heavy */
            }
            return 0;
        case M0C_TOK_TAO_R_SHARP:
            /* R# — Yin-yang 0/1: R-leading, left-heavy (Freedom/Svatantrya) */
            return 1;
        case M0C_TOK_TAO_DOUBLE_SHARP:
            /* ## — Yang-yin 1/0: self-symmetric, neutral (Truth/kinship ground) */
            return 0;
        default:
            return 0;
    }
}

/* ===================================================================
 * Tokenize a formulation string.
 * =================================================================== */

int m0_calc_tokenize(const char* formulation, M0CalcToken* tokens, int max_tokens) {
    if (!formulation || !tokens || max_tokens <= 0) return -1;

    const char* p = formulation;
    int count = 0;

    while (*p && count < max_tokens) {
        /* Skip whitespace */
        if (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r') {
            p++;
            continue;
        }

        /* Try multi-char literals first (longest match) */
        bool matched = false;
        for (const TokenLiteral* lit = TOKEN_LITERALS; lit->text; lit++) {
            size_t lit_len = lit->len;
            if (strncmp(p, lit->text, lit_len) == 0) {
                tokens[count].type      = lit->type;
                tokens[count].start     = p;
                tokens[count].length    = (uint8_t)lit_len;
                tokens[count].chirality = token_chirality(lit->type, p, (uint8_t)lit_len);
                p += lit_len;
                matched = true;
                break;
            }
        }
        if (matched) { count++; continue; }

        /* Check for single-char x followed by // (superposition multiply) */
        if (*p == 'x' && p[1] == '/' && p[2] == '/') {
            tokens[count].type      = M0C_TOK_SUPER_MUL;
            tokens[count].start     = p;
            tokens[count].length    = 3;
            tokens[count].chirality = 0;
            p += 3;
            count++;
            continue;
        }

        /* Check for compound tokens: ?/!, ?!/!?, !?/?! */
        if (strncmp(p, "?/!", 3) == 0) {
            tokens[count].type = M0C_TOK_COMPOUND;
            tokens[count].start = p;
            tokens[count].length = 3;
            tokens[count].chirality = 1;
            p += 3; count++; continue;
        }
        if (strncmp(p, "?!/!?", 5) == 0) {
            tokens[count].type = M0C_TOK_COMPOUND;
            tokens[count].start = p;
            tokens[count].length = 5;
            tokens[count].chirality = 1;
            p += 5; count++; continue;
        }
        if (strncmp(p, "!?/?!", 5) == 0) {
            tokens[count].type = M0C_TOK_COMPOUND;
            tokens[count].start = p;
            tokens[count].length = 5;
            tokens[count].chirality = 2;
            p += 5; count++; continue;
        }

        /* Single char */
        M0CalcTokenType t = single_char_type(*p);
        tokens[count].type      = t;
        tokens[count].start     = p;
        tokens[count].length    = 1;
        tokens[count].chirality = 0;

        /* Determine chirality for dash + digit patterns */
        if (t == M0C_TOK_DASH) {
            /* Check if this dash is prefix or suffix to a digit */
            if (count > 0) {
                M0CalcTokenType prev = tokens[count-1].type;
                if (prev == M0C_TOK_VOID_SINGLE || prev == M0C_TOK_NUMBER) {
                    tokens[count].chirality = 2; /* digit-dash → right-heavy */
                }
            }
            /* Look ahead for digit after dash */
            const char* next = p + 1;
            while (*next == ' ' || *next == '\t') next++;
            if (*next == '0' || (*next >= '2' && *next <= '9')) {
                /* dash-digit pattern; chirality assigned to the dash */
                if (tokens[count].chirality == 0) {
                    tokens[count].chirality = 1; /* dash-digit → left-heavy */
                }
            }
        }

        p++;
        count++;
    }

    /* EOF token */
    if (count < max_tokens) {
        tokens[count].type      = M0C_TOK_EOF;
        tokens[count].start     = p;
        tokens[count].length    = 0;
        tokens[count].chirality = 0;
        count++;
    }

    return count;
}

/* ===================================================================
 * II. PATTERN MATCHING — string-based rewrite rule detection
 *
 * The engine walks the formulation string looking for patterns.
 * Rules are tried in precedence order: framing/hinge first, then
 * chiral addition, then polarity, then chiasm, then exceptions.
 * =================================================================== */

/* Copy a substring with length limit */
static int safe_copy(char* dst, size_t dst_size, const char* src, size_t src_len) {
    if (src_len >= dst_size) return -1;
    memcpy(dst, src, src_len);
    dst[src_len] = '\0';
    return 0;
}

/* Replace substring [pos, pos+len) with replacement in buf.
 * Returns new length, or -1 if buffer too small. */
static int replace_substring(char* buf, size_t buf_size, size_t buf_len,
                              size_t pos, size_t len, const char* repl) {
    size_t repl_len = strlen(repl);
    size_t new_len = buf_len - len + repl_len;
    if (new_len >= buf_size) return -1;

    /* Shift tail */
    memmove(buf + pos + repl_len, buf + pos + len, buf_len - pos - len + 1);
    /* Insert replacement */
    memcpy(buf + pos, repl, repl_len);
    return (int)new_len;
}

/* Find pattern in string, returns position or -1 */
static int find_pattern(const char* haystack, const char* needle, size_t start) {
    const char* found = strstr(haystack + start, needle);
    if (!found) return -1;
    return (int)(found - haystack);
}

/* ===================================================================
 * Apply a single reduction step. Returns true if a rule fired.
 * =================================================================== */

static bool apply_reduction_step(char* term, size_t term_size, size_t* term_len,
                                  M0CalcReductionStep* step, uint8_t depth) {
    size_t len = *term_len;
    int pos = -1;
    M0CalcRule rule = M0C_RULE_NONE;
    const char* rule_name = "";
    const char* replacement = "";
    size_t match_len = 0;

    /* --- Law 3 FIRST: identity-chains are declarations, not computations.
     * The rewriting system runs MODULO the equational theory (Tranche
     * 1.14a): when the whole term is an unambiguous non-canonical member
     * of a registry =-chain class, it rewrites to the chain head before
     * any computational rule fires. The same Law-3 theory read as
     * coordinate walks is M0_IDENTITY_CHAINS (anuttara_language.c);
     * these classes are its rewrite face. Whole-term only, and the head
     * never matches as non-canonical, so this cannot loop. --- */
    {
        const int class_id = m0_identity_class_find(term);
        if (class_id >= 0) {
            const char* canonical = m0_identity_class_canonical(class_id);
            if (canonical && strcmp(canonical, term) != 0 &&
                strlen(canonical) < term_size) {
                pos = 0;
                match_len = len;
                rule = M0C_RULE_IDENTITY;
                rule_name = "Law-3 identity-chain resolution (modulo rewrite)";
                replacement = canonical;
                goto apply;
            }
        }
    }

    /* --- Rule priority order (per DR-CALC-1 O# precedence) --- */

    /* 1. Framing asymmetry — the hinge: (00+00) → 9 (framed) */
    pos = find_pattern(term, "(00+00)", 0);
    if (pos >= 0) {
        rule = M0C_RULE_VOID_ADD_FRAMED;
        rule_name = "Law-2 framing asymmetry: (00+00)→9";
        match_len = 7;
        replacement = "9";
        goto apply;
    }

    /* 2. Void addition unframed: 00+00 → 00 (the master exception) */
    /*    Must NOT be inside parentheses — check that pos is not after '(' */
    pos = find_pattern(term, "00+00", 0);
    while (pos >= 0) {
        /* Check if this 00+00 is inside parentheses */
        if (pos > 0 && term[pos-1] == '(') {
            /* Already handled by framed rule above — skip */
            pos = find_pattern(term, "00+00", (size_t)(pos + 1));
            continue;
        }
        rule = M0C_RULE_VOID_ADD_UNFRAMED;
        rule_name = "Law-2 void addition unframed: 00+00→00";
        match_len = 5;
        replacement = "00";
        goto apply;
    }

    /* 3. Chiasm annihilation: (@/-)(-/@) → 00 */
    pos = find_pattern(term, "(@/-)(-/@)", 0);
    if (pos >= 0) {
        rule = M0C_RULE_CHIASM_ANNIHILATE;
        rule_name = "Law-1 chiasm annihilation: (@/-)(-/@)→00";
        match_len = 10;
        replacement = "00";
        goto apply;
    }

    /* 4. Polarity cancellation: (+0/(+0)) → (0/(0)) */
    pos = find_pattern(term, "(+0/(+0))", 0);
    if (pos >= 0) {
        rule = M0C_RULE_POLARITY_CANCEL;
        rule_name = "Law-1 polarity cancellation: (+0/(+0))→(0/(0))";
        match_len = 9;
        replacement = "(0/(0))";
        goto apply;
    }

    /* 5. Chiral addition: -0+0- → 0 */
    pos = find_pattern(term, "-0+0-", 0);
    if (pos >= 0) {
        rule = M0C_RULE_CHIRAL_ADD_1;
        rule_name = "Law-1 chiral addition: -0+0-→0";
        match_len = 5;
        replacement = "0";
        goto apply;
    }

    /* 6. Chiral addition: 0-+-0 → 00 */
    pos = find_pattern(term, "0-+-0", 0);
    if (pos >= 0) {
        rule = M0C_RULE_CHIRAL_ADD_2;
        rule_name = "Law-1 chiral addition: 0-+-0→00";
        match_len = 5;
        replacement = "00";
        goto apply;
    }

    /* 7. Framing-by-doubling: X+X → (X) — for simple tokens */
    /*    Match pattern: a token followed by + followed by same token */
    {
        /* Scan for X+X pattern where X is a simple token */
        for (size_t i = 0; i + 2 < len; i++) {
            /* Find '+' */
            if (term[i] != '+') continue;

            /* Find left token boundary */
            size_t left_start = i;
            while (left_start > 0 && term[left_start-1] != ' ' &&
                   term[left_start-1] != '(' && term[left_start-1] != ')' &&
                   term[left_start-1] != '+' && term[left_start-1] != '/' &&
                   term[left_start-1] != 'x') {
                left_start--;
            }
            size_t left_len = i - left_start;

            /* Find right token boundary */
            size_t right_start = i + 1;
            size_t right_end = right_start;
            while (right_end < len && term[right_end] != ' ' &&
                   term[right_end] != '(' && term[right_end] != ')' &&
                   term[right_end] != '+' && term[right_end] != '/' &&
                   term[right_end] != 'x') {
                right_end++;
            }
            size_t right_len = right_end - right_start;

            /* Compare left and right tokens */
            if (left_len > 0 && left_len == right_len &&
                left_len < 4 && /* simple token, not compound */
                strncmp(term + left_start, term + right_start, left_len) == 0) {
                /* Don't fire on 00+00 (handled above) */
                if (left_len == 2 && term[left_start] == '0' && term[left_start+1] == '0') {
                    continue;
                }
                pos = (int)left_start;
                match_len = left_len + 1 + right_len;
                rule = M0C_RULE_FRAMING_DOUBLE;

                /* Build replacement: (X) */
                static char framing_repl[64];
                int rl = snprintf(framing_repl, sizeof(framing_repl),
                                  "(%.*s)", (int)left_len, term + left_start);
                if (rl > 0 && rl < (int)sizeof(framing_repl)) {
                    framing_repl[rl] = '\0';
                    replacement = framing_repl;
                } else {
                    replacement = "(X)";
                }
                rule_name = "Law-2 framing-by-doubling: X+X→(X)";
                goto apply;
            }
        }
    }

    /* 8. Superposition multiplication expansion: ((+/-0) x// (+/-0)) → 0/1 */
    pos = find_pattern(term, "((+/-0)", 0);
    if (pos >= 0) {
        int pos2 = find_pattern(term, "x//", (size_t)pos);
        if (pos2 >= 0) {
            int pos3 = find_pattern(term, "(+/-0))", (size_t)pos2);
            if (pos3 >= 0) {
                rule = M0C_RULE_SUPER_MUL_EXPAND;
                rule_name = "Law-4 superposition expansion: ((+/-0)x//(+/-0))→0/1";
                /* "(+/-0))" is 7 chars: consume through the outer close so
                 * the expansion lands "0/1", not "0/1)" (corpus law). */
                match_len = (size_t)(pos3 + 7) - (size_t)pos;
                replacement = "0/1";
                goto apply;
            }
        }
    }

    /* --- Interrogative exception system (Law 6) --- */

    /* 9. Division by zero indeterminate: 0/0 → % */
    pos = find_pattern(term, "0/0", 0);
    if (pos >= 0) {
        /* Ensure this isn't part of a longer number like 00/0 */
        bool valid = true;
        if (pos > 0 && term[pos-1] == '0') valid = false;
        if ((size_t)(pos + 3) < len && term[pos+3] == '0') valid = false;
        if (valid) {
            rule = M0C_RULE_DIV_ZERO_INDET;
            rule_name = "Law-6 indeterminate: 0/0→%";
            match_len = 3;
            replacement = "%";
            goto apply;
        }
    }

    /* 10. Application to zero: X(0) → ?!/!? */
    /*     Match pattern: token immediately followed by (0).
     *     i+2 < len (not i+3): the pattern needs indices i..i+2 valid, and
     *     the earlier off-by-one silently skipped X(0) at end-of-term. */
    {
        for (size_t i = 0; i + 2 < len; i++) {
            if (term[i] == '(' && term[i+1] == '0' && term[i+2] == ')') {
                /* Find the token before '(' */
                size_t tok_end = i;
                while (tok_end > 0 && term[tok_end-1] == ' ') tok_end--;
                size_t tok_start = tok_end;
                while (tok_start > 0 && term[tok_start-1] != ' ' &&
                       term[tok_start-1] != '(' && term[tok_start-1] != ')' &&
                       term[tok_start-1] != '+' && term[tok_start-1] != '/' &&
                       term[tok_start-1] != 'x') {
                    tok_start--;
                }
                if (tok_start < tok_end) {
                    pos = (int)tok_start;
                    match_len = (tok_end - tok_start) + 3;
                    rule = M0C_RULE_APPLY_ZERO;
                    rule_name = "Law-6 application to zero: X(0)→?!";
                    replacement = "?!";
                    goto apply;
                }
            }
        }
    }

    /* 11. Division by zero query: 1/0 → ?/! */
    pos = find_pattern(term, "1/0", 0);
    if (pos >= 0) {
        if (!(pos > 0 && term[pos-1] >= '0' && term[pos-1] <= '9')) {
            rule = M0C_RULE_DIV_ZERO_QUERY;
            rule_name = "Law-6 query: 1/0→?/!";
            match_len = 3;
            replacement = "?/!";
            goto apply;
        }
    }

    return false; /* No rule fired */

apply:
    /* Record the step */
    if (step) {
        step->rule = rule;
        snprintf(step->rule_name, M0C_MAX_RULE_NAME, "%s", rule_name);
        safe_copy(step->before, M0C_MAX_TERM_LEN, term, len);
        step->match_pos = (uint8_t)(pos & 0xFF);
        step->depth = depth;
        step->is_interrogative = (rule == M0C_RULE_DIV_ZERO_INDET ||
                                   rule == M0C_RULE_APPLY_ZERO ||
                                   rule == M0C_RULE_DIV_ZERO_QUERY);
    }

    /* Apply replacement */
    int new_len = replace_substring(term, term_size, len,
                                     (size_t)pos, match_len, replacement);
    if (new_len < 0) return false;

    *term_len = (size_t)new_len;

    if (step) {
        safe_copy(step->after, M0C_MAX_TERM_LEN, term, (size_t)new_len);
    }

    return true;
}

/* ===================================================================
 * III. REDUCTION ENGINE — iterate until normal form
 * =================================================================== */

int m0_calc_reduce(const char* formulation, M0CalcTrace* out) {
    if (!formulation || !out) return -1;

    /* Initialize trace */
    memset(out, 0, sizeof(M0CalcTrace));

    /* Copy formulation to working buffer */
    size_t flen = strlen(formulation);
    if (flen >= M0C_MAX_TERM_LEN) return -2;

    char term[M0C_MAX_TERM_LEN];
    memcpy(term, formulation, flen + 1);
    size_t term_len = flen;

    /* Reduction loop */
    uint8_t depth = 0;
    while (depth < M0C_MAX_STEPS) {
        M0CalcReductionStep* step = &out->steps[out->step_count];

        if (!apply_reduction_step(term, sizeof(term), &term_len,
                                   step, depth + 1)) {
            /* No rule fired — we're in normal form */
            break;
        }

        out->step_count++;
        if (step->is_interrogative) {
            out->has_interrogative = true;
        }
        depth++;

        /* Safety: prevent infinite loops with a generous cap */
        if (depth >= 64) {
            snprintf(out->diagnostic, M0C_MAX_DIAGNOSTIC,
                     "Reduction halted at depth %u (cap). Term: %s",
                     depth, term);
            break;
        }
    }

    /* Copy final term */
    safe_copy(out->final_term, M0C_MAX_TERM_LEN, term, term_len);

    /* Determine state */
    out->is_normal_form = (depth < M0C_MAX_STEPS) &&
                          !strstr(out->diagnostic, "halted");

    /* Check palindrome normal-form */
    out->is_palindrome = m0_calc_is_palindrome_normal(out->final_term);

    return 0;
}

/* ===================================================================
 * IV. PALINDROME NORMAL-FORM CHECK (Law 5 — 8+1=9)
 *
 * A term is whole (9) when its normal form reads identically from
 * either pole. This implements the computable test for 9-ness.
 * =================================================================== */

bool m0_calc_is_palindrome_normal(const char* term) {
    if (!term) return false;

    /* Skip leading/trailing whitespace */
    const char* start = term;
    while (*start == ' ' || *start == '\t') start++;
    const char* end = start + strlen(start);
    while (end > start && (end[-1] == ' ' || end[-1] == '\t')) end--;

    size_t len = (size_t)(end - start);
    if (len == 0) return true; /* empty = void = symmetric */

    /* Tokenize the term and check symmetry */
    M0CalcToken tokens[128];
    int tok_count = m0_calc_tokenize(start, tokens, 128);
    if (tok_count <= 0) return false;

    /* Exclude EOF token */
    int n = tok_count;
    if (n > 0 && tokens[n-1].type == M0C_TOK_EOF) n--;

    if (n == 0) return true;

    /* Compare tokens from both ends */
    for (int i = 0; i < n / 2; i++) {
        int j = n - 1 - i;

        /* Must be same token type */
        if (tokens[i].type != tokens[j].type) return false;

        /* Must have same length text */
        if (tokens[i].length != tokens[j].length) return false;

        /* Must have same text content */
        if (strncmp(tokens[i].start, tokens[j].start, tokens[i].length) != 0) {
            return false;
        }

        /* Chirality must be symmetric (1 ↔ 2, 0 ↔ 0) */
        uint8_t ci = tokens[i].chirality;
        uint8_t cj = tokens[j].chirality;
        if (ci != cj) {
            /* Allow neutral ↔ neutral, left-heavy ↔ right-heavy */
            if (!((ci == 0 && cj == 0) ||
                  (ci == 1 && cj == 2) ||
                  (ci == 2 && cj == 1))) {
                return false;
            }
        }
    }

    return true;
}

/* ===================================================================
 * V. QUERY-OBJECT DETECTION (Law 6)
 *
 * Returns true if the term is a typed interrogative query-object.
 * These are the ?-objects the Verifier emits as symbolic-coordinate
 * strings — never errors, always typed questions.
 * =================================================================== */

bool m0_calc_is_query_object(const char* term) {
    if (!term) return false;

    /* Direct query tokens */
    if (strcmp(term, "?") == 0) return true;
    if (strcmp(term, "?!") == 0) return true;
    if (strcmp(term, "!?") == 0) return true;
    if (strcmp(term, "?/!") == 0) return true;
    if (strcmp(term, "%") == 0) return true;

    /* Compound query forms containing ? */
    if (strstr(term, "?!/!?") || strstr(term, "!?/?!")) return true;

    return false;
}

/* ===================================================================
 * VI. SYNTAX-WITNESS COMPUTATION (4 bits)
 *
 * Bits (LSB first):
 *   bit 0 — zodiacal utterance formed       (Vak, archetype 3)
 *   bit 1 — mono-poly state resolved        (archetype 5)
 *   bit 2 — R-factor path traced            (archetype 7)
 *   bit 3 — palindrome closure reached      (archetype 9)
 *
 * Derived from KernelState fields and virtue evidence.
 * =================================================================== */

int m0_calc_witness(const KernelState* state, uint8_t* syntax_witness_out) {
    if (!state || !syntax_witness_out) return -1;

    uint8_t witness = 0;

    /* Bit 0 — zodiacal utterance formed:
     * True when syntax layer SPEECH is committed and
     * virtue evidence for Joy/Play (index 3, archetype 3=Vak) exceeds threshold. */
    if ((state->syntax_layer_mask & M0_VERIFIER_SYNTAX_SPEECH) &&
        state->virtue_evidence[3] >= M0_VERIFIER_VIRTUE_THRESHOLD) {
        witness |= M0C_WITNESS_ZODIACAL;
    }

    /* Bit 1 — mono-poly state resolved:
     * True when syntax layer RELATIONSHIP is committed and
     * virtue evidence for Beauty (index 5, archetype 5=MonoPoly) exceeds threshold. */
    if ((state->syntax_layer_mask & M0_VERIFIER_SYNTAX_RELATIONSHIP) &&
        state->virtue_evidence[5] >= M0_VERIFIER_VIRTUE_THRESHOLD) {
        witness |= M0C_WITNESS_MONOPOLY;
    }

    /* Bit 2 — R-factor path traced:
     * True when syntax layer ACTION is committed and
     * virtue evidence for Life/Nature (index 6, archetype 7=Divine Action) exceeds threshold. */
    if ((state->syntax_layer_mask & M0_VERIFIER_SYNTAX_ACTION) &&
        state->virtue_evidence[6] >= M0_VERIFIER_VIRTUE_THRESHOLD) {
        witness |= M0C_WITNESS_RFACTOR;
    }

    /* Bit 3 — palindrome closure reached:
     * True when syntax layer COMPLETION is committed and
     * virtue evidence for Reality (index 8, archetype 9=Wholeness) exceeds threshold. */
    if ((state->syntax_layer_mask & M0_VERIFIER_SYNTAX_COMPLETION) &&
        state->virtue_evidence[8] >= M0_VERIFIER_VIRTUE_THRESHOLD) {
        witness |= M0C_WITNESS_PALINDROME;
    }

    *syntax_witness_out = witness;
    return 0;
}

/* ===================================================================
 * VII. TAO-ELEMENT OPERATIONS (Tranche 01.T1.17)
 *
 * R# (Yin-yang 0/1) and ## (Yang-yin 1/0) are the two tao elements —
 * the binary read both ways. The coin method derives nucleotide I-Ching
 * values from them, and the Tao↔codon binding asserts the seam where
 * M0 calculus and M3 codon engine are one operation.
 * =================================================================== */

int m0_calc_nucleotide_from_coin(int yin_count, int yang_count) {
    /* Coin method: Yin (R#) = 2, Yang (##) = 3.
     * Value = yang_count + 5 over a 4-slot frame.
     * Produces NUCLEOTIDE_ICHING_VALUE[4] = {6, 9, 8, 7} (M3-COIN-1) */
    if (yin_count < 0 || yang_count < 0) return -1;
    if (yin_count + yang_count != 4) return -1;
    return yang_count + 5;
}

bool m0_calc_tao_is_codon_eval(void) {
    /* The kinship-grammar apex Tao (5-/5, coord-pos 5) IS the act
     * that reads the 0/1 ↔ 1/0 R#/## binary into genetic charges
     * pp/nn/np/pn — verified structurally, not asserted:
     *   (1) the apex exists: Tao at kinship position 5, dominant
     *       synthesis on the verbatim 5-/5 chiral coordinate;
     *   (2) the R#/## coin construction (4-slot frame, yang+5)
     *       regenerates NUCLEOTIDE_ICHING_VALUE {6,9,8,7} (M3-COIN-1);
     *   (3) for every codon, m3_compute_charges equals the X#
     *       sign-algebra over those coin-constructed values. */
    static const int coin_counts[4][2] = {
        { 3, 1 }, /* A — Old Yin     3xR# + 1x## -> 6 */
        { 0, 4 }, /* T — Old Yang    0xR# + 4x## -> 9 */
        { 1, 3 }, /* C — Young Yin   1xR# + 3x## -> 8 (M3-COIN-1) */
        { 2, 2 }, /* G — Young Yang  2xR# + 2x## -> 7 (M3-COIN-1) */
    };

    const Nara_Entry* tao = &NARA_MSHARP_LUT[5];
    if (tao->frame_position != 5u ||
        tao->dominance_mode != (uint8_t)NARA_DOM_DOMINANT ||
        tao->coordinate == NULL ||
        strcmp(tao->coordinate, "5-/5") != 0) {
        return false;
    }

    for (int n = 0; n < 4; n += 1) {
        if (m0_calc_nucleotide_from_coin(coin_counts[n][0], coin_counts[n][1])
            != (int)NUCLEOTIDE_ICHING_VALUE[n]) {
            return false;
        }
    }

    for (int codon = 0; codon < 64; codon += 1) {
        int8_t pp, nn, np, pn;
        int x = m0_calc_nucleotide_from_coin(
            coin_counts[(codon >> 4) & 0x03][0],
            coin_counts[(codon >> 4) & 0x03][1]);
        int y = m0_calc_nucleotide_from_coin(
            coin_counts[(codon >> 2) & 0x03][0],
            coin_counts[(codon >> 2) & 0x03][1]);
        int z = m0_calc_nucleotide_from_coin(
            coin_counts[codon & 0x03][0],
            coin_counts[codon & 0x03][1]);
        m3_compute_charges((uint8_t)codon, &pp, &nn, &np, &pn);
        if (pp != (int8_t)(x + y + z) || nn != (int8_t)(x - y - z) ||
            np != (int8_t)(x - y + z) || pn != (int8_t)(x + y - z)) {
            return false;
        }
    }

    return true;
}

/* ===================================================================
 * VIII. SELF-TEST: corpus-supplied unit tests as C assertions
 *
 * These validate the reduction engine against the canonical corpus:
 *   - Framing asymmetry: 00+00→00 (unframed) vs (00+00)→9 (framed)
 *   - Chiral addition: -0+0-→0, 0-+-0→00
 *   - Interrogative exceptions: 0/0→%, X(0)→?!, 1/0→?/!
 *   - Polarity cancellation and chiasm annihilation
 * =================================================================== */

#ifdef M0_CALCULUS_SELF_TEST

#include <assert.h>
#include <stdio.h>

static void test_framing_asymmetry(void) {
    M0CalcTrace trace;

    /* Unframed: 00+00 → 00 */
    memset(&trace, 0, sizeof(trace));
    int rc = m0_calc_reduce("00+00", &trace);
    assert(rc == 0);
    assert(trace.step_count >= 1);
    assert(strcmp(trace.final_term, "00") == 0);
    printf("  PASS: 00+00 → 00 (unframed)\n");

    /* Framed: (00+00) → 9 */
    memset(&trace, 0, sizeof(trace));
    rc = m0_calc_reduce("(00+00)", &trace);
    assert(rc == 0);
    assert(trace.step_count >= 1);
    assert(strcmp(trace.final_term, "9") == 0);
    printf("  PASS: (00+00) → 9 (framed)\n");
}

static void test_chiral_addition(void) {
    M0CalcTrace trace;

    /* -0+0- → 0 */
    memset(&trace, 0, sizeof(trace));
    int rc = m0_calc_reduce("-0+0-", &trace);
    assert(rc == 0);
    assert(trace.step_count >= 1);
    assert(strcmp(trace.final_term, "0") == 0);
    printf("  PASS: -0+0- → 0\n");

    /* 0-+-0 → 00 */
    memset(&trace, 0, sizeof(trace));
    rc = m0_calc_reduce("0-+-0", &trace);
    assert(rc == 0);
    assert(trace.step_count >= 1);
    assert(strcmp(trace.final_term, "00") == 0);
    printf("  PASS: 0-+-0 → 00\n");
}

static void test_interrogative_exceptions(void) {
    M0CalcTrace trace;

    /* 0/0 → % */
    memset(&trace, 0, sizeof(trace));
    int rc = m0_calc_reduce("0/0", &trace);
    assert(rc == 0);
    assert(trace.has_interrogative);
    assert(m0_calc_is_query_object(trace.final_term));
    printf("  PASS: 0/0 → %%\n");

    /* 1/0 → ?/! */
    memset(&trace, 0, sizeof(trace));
    rc = m0_calc_reduce("1/0", &trace);
    assert(rc == 0);
    assert(trace.has_interrogative);
    assert(m0_calc_is_query_object(trace.final_term));
    printf("  PASS: 1/0 → ?/!\n");
}

static void test_polarity_cancellation(void) {
    M0CalcTrace trace;

    memset(&trace, 0, sizeof(trace));
    int rc = m0_calc_reduce("(+0/(+0))", &trace);
    assert(rc == 0);
    assert(trace.step_count >= 1);
    /* Result should be (0/(0)) after cancellation */
    printf("  PASS: (+0/(+0)) reduced (polarity cancellation)\n");
}

static void test_chiasm_annihilation(void) {
    M0CalcTrace trace;

    memset(&trace, 0, sizeof(trace));
    int rc = m0_calc_reduce("(@/-)(-/@)", &trace);
    assert(rc == 0);
    assert(trace.step_count >= 1);
    assert(strcmp(trace.final_term, "00") == 0);
    printf("  PASS: (@/-)(-/@) → 00\n");
}

static void test_palindrome_normal(void) {
    /* Symmetric terms */
    assert(m0_calc_is_palindrome_normal("00"));
    assert(m0_calc_is_palindrome_normal("0"));
    assert(m0_calc_is_palindrome_normal("9"));
    printf("  PASS: palindrome normal-form detection\n");
}

static void test_witness(void) {
    KernelState ks;
    memset(&ks, 0, sizeof(ks));
    ks.syntax_layer_mask = M0_VERIFIER_SYNTAX_ALL;
    for (int i = 0; i < 9; i++) {
        ks.virtue_evidence[i] = 0.6f;
    }

    uint8_t witness = 0;
    int rc = m0_calc_witness(&ks, &witness);
    assert(rc == 0);
    assert(witness == M0C_WITNESS_ALL);
    printf("  PASS: full syntax-witness (all 4 bits)\n");
}

static void test_tokenizer(void) {
    M0CalcToken tokens[64];
    int n = m0_calc_tokenize("-0+0-", tokens, 64);
    assert(n >= 4);
    assert(tokens[0].type == M0C_TOK_DASH);
    assert(tokens[1].type == M0C_TOK_VOID_SINGLE);
    assert(tokens[2].type == M0C_TOK_PLUS);
    printf("  PASS: tokenizer chirality preservation\n");

    n = m0_calc_tokenize("00+00", tokens, 64);
    assert(n >= 3);
    assert(tokens[0].type == M0C_TOK_VOID_DOUBLE);
    assert(tokens[2].type == M0C_TOK_VOID_DOUBLE);
    printf("  PASS: tokenizer double-void recognition\n");

    n = m0_calc_tokenize("(00+00)", tokens, 64);
    assert(n >= 5);
    assert(tokens[0].type == M0C_TOK_FRAME_OPEN);
    assert(tokens[4].type == M0C_TOK_FRAME_CLOSE);
    printf("  PASS: tokenizer frame recognition\n");
}

static void test_nucleotide_from_coin(void) {
    /* A = 3×R# + 1×## → 6 (yang_count=1, 1+5=6) */
    assert(m0_calc_nucleotide_from_coin(3, 1) == 6);
    /* T = 0×R# + 4×## → 9 (yang_count=4, 4+5=9) */
    assert(m0_calc_nucleotide_from_coin(0, 4) == 9);
    /* C = 2×R# + 2×## → 7 (yang_count=2, 2+5=7) */
    assert(m0_calc_nucleotide_from_coin(2, 2) == 7);
    /* G = 1×R# + 3×## → 8 (yang_count=3, 3+5=8) */
    assert(m0_calc_nucleotide_from_coin(1, 3) == 8);

    /* Bad inputs */
    assert(m0_calc_nucleotide_from_coin(-1, 5) == -1);
    assert(m0_calc_nucleotide_from_coin(2, 3) == -1);  /* sums to 5 */
    assert(m0_calc_nucleotide_from_coin(0, 0) == -1);

    printf("  PASS: nucleotide I-Ching derivation {6,9,8,7}\n");
}

static void test_tao_codon_binding(void) {
    /* The Tao ≡ codon charge-evaluation binding is an identity */
    assert(m0_calc_tao_is_codon_eval() == true);
    printf("  PASS: Tao↔codon evaluation binding asserted\n");
}

static void test_tao_element_tokenizer(void) {
    M0CalcToken tokens[64];
    int n;

    /* R# as compound token */
    n = m0_calc_tokenize("R#", tokens, 64);
    assert(n >= 2);
    assert(tokens[0].type == M0C_TOK_TAO_R_SHARP);
    assert(tokens[0].length == 2);
    printf("  PASS: tokenizer R# tao-element recognition\n");

    /* ## as compound token */
    n = m0_calc_tokenize("##", tokens, 64);
    assert(n >= 2);
    assert(tokens[0].type == M0C_TOK_TAO_DOUBLE_SHARP);
    assert(tokens[0].length == 2);
    printf("  PASS: tokenizer ## tao-element recognition\n");

    /* R# and ## together */
    n = m0_calc_tokenize("R##", tokens, 64);
    assert(n >= 3);
    assert(tokens[0].type == M0C_TOK_TAO_R_SHARP);
    assert(tokens[0].length == 2);
    assert(tokens[1].type == M0C_TOK_HASH); /* trailing # after R## */
    printf("  PASS: tokenizer R# + trailing #\n");

    /* ##R# */
    n = m0_calc_tokenize("##R#", tokens, 64);
    assert(n >= 3);
    assert(tokens[0].type == M0C_TOK_TAO_DOUBLE_SHARP);
    assert(tokens[0].length == 2);
    assert(tokens[1].type == M0C_TOK_TAO_R_SHARP);
    printf("  PASS: tokenizer ## + R# sequence\n");
}

int main(void) {
    printf("m0_calculus self-tests (Tranche 01.T1.13 + 01.T1.17):\n");
    test_framing_asymmetry();
    test_chiral_addition();
    test_interrogative_exceptions();
    test_polarity_cancellation();
    test_chiasm_annihilation();
    test_palindrome_normal();
    test_witness();
    test_tokenizer();
    test_nucleotide_from_coin();
    test_tao_codon_binding();
    test_tao_element_tokenizer();
    printf("\nAll tests passed.\n");
    return 0;
}

#endif /* M0_CALCULUS_SELF_TEST */

/* ===================================================================
 * Law 7: dash pentavalence (Tranche 1.14b).
 *
 * Positional polysemy, hence parseable — the reading is determined by
 * what flanks the mark, checked in specificity order:
 *   strikethrough: digit-dash against a kinship slash (2-/2, 1/1-)
 *   chirality:     void-adjacent mirror mark (-0, 0-)
 *   operator:      base-spine connector between #-glyphs (O#-X#)
 *   range:         span between dotted coordinates inside a frame
 *   subtraction:   plain numeric infix (9-8)
 * =================================================================== */

static bool idc_is_digit(char c) { return c >= '0' && c <= '9'; }

M0CalcDashReading m0_calc_dash_reading(const char* term, size_t pos) {
    if (!term) return M0C_DASH_UNKNOWN;
    const size_t len = strlen(term);
    if (pos >= len || term[pos] != '-') return M0C_DASH_UNKNOWN;

    const char before = pos > 0 ? term[pos - 1] : '\0';
    const char after = pos + 1 < len ? term[pos + 1] : '\0';

    /* Strikethrough: the dominance dash rides a kinship ratio — digit
     * before the dash with a slash directly after (2-/2), or a trailing
     * dash on a ratio's denominator digit (1/1-). */
    if (idc_is_digit(before) && after == '/') return M0C_DASH_STRIKETHROUGH;
    if (idc_is_digit(before) && after == '\0' && memchr(term, '/', pos)) {
        return M0C_DASH_STRIKETHROUGH;
    }

    /* Chirality: the mirror mark hugs the void. */
    if ((after == '0' && !idc_is_digit(before)) ||
        (before == '0' && !idc_is_digit(after) && after != '/')) {
        return M0C_DASH_CHIRALITY;
    }

    /* Operator: connective between #-bearing base glyphs. */
    if (before == '#' || after == 'O' || after == 'X' || after == 'N' ||
        after == 'M' || after == '#') {
        return M0C_DASH_OPERATOR;
    }

    /* Range: a span between dotted coordinate fragments (CF frames). */
    if (idc_is_digit(before) && idc_is_digit(after)) {
        bool dotted_before = false, dotted_after = false;
        for (size_t i = pos; i-- > 0 && term[i] != '(' && term[i] != ' ';) {
            if (term[i] == '.') { dotted_before = true; break; }
        }
        for (size_t i = pos + 1; i < len && term[i] != ')' && term[i] != ' '; i++) {
            if (term[i] == '.') { dotted_after = true; break; }
        }
        if (dotted_before || dotted_after) return M0C_DASH_RANGE;
        return M0C_DASH_SUBTRACTION;
    }

    return M0C_DASH_UNKNOWN;
}
