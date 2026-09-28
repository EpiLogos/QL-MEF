/**
 * m0_calculus.h — Anuttara Term-Rewriting Calculus (Tranche 01.T1.13)
 *
 * Chirality-typed rewrite engine with token alphabet, reduction rules,
 * interrogative exception system, and palindrome normal-form.
 *
 * Seven deep laws as the calculus's law-set:
 *   Law 1 — Chirality: order of marks is semantic
 *   Law 2 — Containment as ontological act: framing-by-doubling
 *   Law 3 — Identity-chains: = chains declare identity-classes
 *   Law 4 — Connective soteriology: conjunctive vs disjunctive
 *   Law 5 — 8+1=9 law: explicate-eight + implicate-one = wholeness
 *   Law 6 — Interrogative undefinedness: typed query-objects
 *   Law 7 — Derivation, address, range
 *
 * API:
 *   m0_calc_reduce(const char* formulation, M0CalcTrace* out)
 *   m0_calc_witness(const KernelState*, uint8_t* syntax_witness_out)
 */

#ifndef M0_CALCULUS_H
#define M0_CALCULUS_H

#include "m0_verifier.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ===================================================================
 * I. CHIRALITY-TYPED TOKEN ALPHABET (Law 1)
 *
 * Order IS semantic: -0 and 0- are distinct tokens.
 * The alphabet covers all primitive marks in the Anuttara syntax.
 * =================================================================== */

typedef enum {
    M0C_TOK_VOID_SINGLE   = 0,   /* 0   — neutral ground                  */
    M0C_TOK_VOID_DOUBLE   = 1,   /* 00  — void recurrence                 */
    M0C_TOK_DASH          = 2,   /* -   — the operator mark (pentavalent)  */
    M0C_TOK_FRAME_OPEN    = 3,   /* (   — containment open                */
    M0C_TOK_FRAME_CLOSE   = 4,   /* )   — containment close               */
    M0C_TOK_PLUS          = 5,   /* +   — chiral addition                 */
    M0C_TOK_MUL           = 6,   /* x   — multiplication                  */
    M0C_TOK_DIV           = 7,   /* /   — division                        */
    M0C_TOK_EQ            = 8,   /* =   — equivalence                     */
    M0C_TOK_NEQ           = 9,   /* ≠   — distinction                     */
    M0C_TOK_BANG          = 10,  /* !   — Prakasa mark                    */
    M0C_TOK_QUEST         = 11,  /* ?   — Vimarsa mark                    */
    M0C_TOK_AT            = 12,  /* @   — Presence mark                   */
    M0C_TOK_HASH          = 13,  /* #   — matrix/void mark                */
    M0C_TOK_R             = 14,  /* R   — reality/freedom mark            */
    M0C_TOK_INFINITY      = 15,  /* ∞   — unbounded                       */
    M0C_TOK_NINE          = 16,  /* 9   — wholeness constant              */
    M0C_TOK_I_SQUARED     = 17,  /* i²  — imaginary unit squared          */
    M0C_TOK_PERCENT       = 18,  /* %   — indeterminate / query-result    */
    M0C_TOK_SUPER_MUL     = 19,  /* x// — superposition-preserving mul    */
    M0C_TOK_NUMBER        = 20,  /* digit 2-8 (contextual)                */
    M0C_TOK_COMPOUND       = 21, /* multi-char compound (?!, !?, etc.)    */
    M0C_TOK_TAO_R_SHARP    = 22, /* R#  — Yin-yang 0/1 (Freedom/Svatantrya) */
    M0C_TOK_TAO_DOUBLE_SHARP=23, /* ##  — Yang-yin 1/0 (Truth/kinship ground) */
    M0C_TOK_EOF            = 24, /* end of formulation                    */
    M0C_TOK_UNKNOWN        = 25, /* unrecognized character                */
} M0CalcTokenType;

#define M0C_TOKEN_ALPHABET_SIZE 26

/* A single token in the chirality-aware stream */
typedef struct {
    M0CalcTokenType type;
    const char*     start;     /* pointer into source string */
    uint8_t         length;    /* token length in bytes      */
    uint8_t         chirality; /* 0=neutral, 1=left-heavy(-X), 2=right-heavy(X-) */
} M0CalcToken;

/* ===================================================================
 * II. REDUCTION TRACE
 *
 * Every reduction step is captured so the inspector can replay
 * the full derivation path.
 * =================================================================== */

#define M0C_MAX_STEPS         128
#define M0C_MAX_TERM_LEN      512
#define M0C_MAX_RULE_NAME      48
#define M0C_MAX_DIAGNOSTIC    256

typedef enum {
    M0C_RULE_NONE             = 0,
    M0C_RULE_FRAMING_DOUBLE   = 1,   /* X+X → (X)                    */
    M0C_RULE_VOID_ADD_UNFRAMED = 2,  /* 00+00 → 00                   */
    M0C_RULE_VOID_ADD_FRAMED  = 3,   /* (00+00) → 9                  */
    M0C_RULE_CHIRAL_ADD_1     = 4,   /* -0+0- → 0                    */
    M0C_RULE_CHIRAL_ADD_2     = 5,   /* 0-+-0 → 00                   */
    M0C_RULE_POLARITY_CANCEL  = 6,   /* (+0/(+0)) → (0/(0))          */
    M0C_RULE_CHIASM_ANNIHILATE = 7,  /* (@/-)(-/@) → 00              */
    M0C_RULE_DIV_ZERO_INDET   = 8,   /* 0/0 → %                      */
    M0C_RULE_APPLY_ZERO       = 9,   /* X(0) → ?!/!?                 */
    M0C_RULE_DIV_ZERO_QUERY   = 10,  /* 1/0 → ?/!                    */
    M0C_RULE_PAREN_REDUCE     = 11,  /* reduce inside parentheses    */
    M0C_RULE_SUPER_MUL_EXPAND = 12,  /* ((+/-0) x// (+/-0)) → 0/1    */
    M0C_RULE_IDENTITY         = 13,  /* = chain resolution           */
} M0CalcRule;

typedef struct {
    M0CalcRule  rule;
    char        rule_name[M0C_MAX_RULE_NAME];
    char        before[M0C_MAX_TERM_LEN];
    char        after[M0C_MAX_TERM_LEN];
    uint8_t     depth;        /* reduction step number (1-based) */
    uint8_t     match_pos;    /* byte offset where rule matched  */
    bool        is_interrogative;  /* true if this step produced a ?-object */
} M0CalcReductionStep;

typedef struct {
    M0CalcReductionStep steps[M0C_MAX_STEPS];
    uint8_t             step_count;
    char                final_term[M0C_MAX_TERM_LEN];
    bool                is_normal_form;     /* true if no more rules apply */
    bool                is_palindrome;      /* true if normal form reads same from either pole */
    bool                has_interrogative;  /* true if any step emitted a ?-object */
    char                diagnostic[M0C_MAX_DIAGNOSTIC];
} M0CalcTrace;

/* ===================================================================
 * III. SYNTAX-WITNESS CONSTANTS (4 bits)
 *
 * One bit per odd archetype: 3/5/7/9
 *   bit 0: zodiacal utterance formed?       (Vak, archetype 3)
 *   bit 1: mono-poly state resolved?        (archetype 5)
 *   bit 2: R-factor path traced?            (archetype 7)
 *   bit 3: palindrome closure reached?      (archetype 9)
 * =================================================================== */

#define M0C_WITNESS_ZODIACAL     (1u << 0)  /* Vak utterance formed    */
#define M0C_WITNESS_MONOPOLY     (1u << 1)  /* Mono-poly resolved      */
#define M0C_WITNESS_RFACTOR      (1u << 2)  /* R-factor path traced    */
#define M0C_WITNESS_PALINDROME   (1u << 3)  /* Palindrome closure      */

#define M0C_WITNESS_ALL \
    (M0C_WITNESS_ZODIACAL | M0C_WITNESS_MONOPOLY | \
     M0C_WITNESS_RFACTOR | M0C_WITNESS_PALINDROME)

/* ===================================================================
 * IV. PUBLIC API
 * =================================================================== */

/**
 * Reduce a formulation to its normal form.
 *
 * @param formulation  Null-terminated Anuttara formulation string.
 * @param out          Output trace buffer (caller-allocated).
 * @return 0 on success, -1 on null pointer, -2 on term too long.
 */
int m0_calc_reduce(const char* formulation, M0CalcTrace* out);

/**
 * Compute the 4-bit syntax-witness vector from kernel state.
 *
 * Bits (LSB first):
 *   bit 0 — zodiacal utterance formed
 *   bit 1 — mono-poly state resolved
 *   bit 2 — R-factor path traced
 *   bit 3 — palindrome closure reached
 *
 * @param state               Kernel state to witness.
 * @param syntax_witness_out  Output byte (bits 0-3 used).
 * @return 0 on success, -1 on null pointer.
 */
int m0_calc_witness(const KernelState* state, uint8_t* syntax_witness_out);

/**
 * Check if a term is in palindrome normal-form (reads same from either pole).
 *
 * @param term  Null-terminated term string.
 * @return true if the term is a palindrome in normal form.
 */
bool m0_calc_is_palindrome_normal(const char* term);

/**
 * Test if a term string contains a typed query-object (?, ?!, !?, ?/!, etc.).
 *
 * @param term  Null-terminated term string.
 * @return true if the term is an interrogative query-object.
 */
bool m0_calc_is_query_object(const char* term);

/* --- Law 3: the equational theory (Tranche 1.14a) ------------------------
 * The M0_IDENTITY_CHAINS coordinate walks and the registry =-chain
 * equivalence classes (both anuttara_language.h) are one Law-3 theory in
 * two faces; m0_calc_reduce rewrites MODULO the classes — identity
 * resolution fires before any computational rule. */

/* --- Law 7: dash pentavalence (Tranche 1.14b, DR-CALC-2 adjacent) --------
 * The `-` mark is five things BY POSITION — positional polysemy, hence
 * parseable. `~` is NOT canon syntax (author meta-pointer, annotation only).
 */
typedef enum {
    M0C_DASH_CHIRALITY     = 0, /* void-adjacent mirror mark: -0, 0-        */
    M0C_DASH_OPERATOR      = 1, /* base-spine connector: O#-X#-N#           */
    M0C_DASH_RANGE         = 2, /* span inside a CF/dotted frame: 4.0/1-4.4/5 */
    M0C_DASH_STRIKETHROUGH = 3, /* dominance chirality in kinship: 2-/2     */
    M0C_DASH_SUBTRACTION   = 4, /* plain numeric infix: 9-8                 */
    M0C_DASH_UNKNOWN       = 5,
} M0CalcDashReading;

/**
 * Classify the dash at `pos` in `term` by its positional context (Law 7).
 * Returns M0C_DASH_UNKNOWN when term[pos] is not a dash.
 */
M0CalcDashReading m0_calc_dash_reading(const char* term, size_t pos);

/**
 * Tokenize a formulation string into an array of tokens.
 *
 * @param formulation  Null-terminated formulation string.
 * @param tokens       Output token array (caller-allocated, max 256).
 * @param max_tokens   Capacity of tokens array.
 * @return number of tokens produced, or -1 on error.
 */
int m0_calc_tokenize(const char* formulation, M0CalcToken* tokens, int max_tokens);

/**
 * Derive the I-Ching nucleotide value {6,9,8,7} from the coin method.
 *
 * Coin method: Yin (R#) = 2, Yang (##) = 3. Value = yang_count + 5.
 *   A = 3×R# + 1×##  →  6   (yin_count=3, yang_count=1)
 *   T = 0×R# + 4×##  →  9   (yin_count=0, yang_count=4)
 *   C = 1×R# + 3×##  →  8   (yin_count=1, yang_count=3)  (M3-COIN-1)
 *   G = 2×R# + 2×##  →  7   (yin_count=2, yang_count=2)  (M3-COIN-1)
 *
 * Reproduces NUCLEOTIDE_ICHING_VALUE[4] = {6, 9, 8, 7} (m3 canon, M3-COIN-1).
 *
 * @param yin_count  Number of R# (Yin) elements (0-4).
 * @param yang_count Number of ## (Yang) elements (0-4).
 * @return I-Ching value {6,7,8,9}, or -1 if counts don't sum to 4.
 */
int m0_calc_nucleotide_from_coin(int yin_count, int yang_count);

/**
 * Assert the Tao ≣ codon charge-evaluation binding.
 *
 * The kinship-grammar apex Tao (5-/5, coord-pos 5) IS the act that reads
 * the 0/1 ↔ 1/0 R#/## binary into genetic charges pp/nn/np/pn.
 * This is the seam where M0 calculus and M3 codon engine are one operation.
 *
 * @return true — the binding is an identity, always asserted.
 */
bool m0_calc_tao_is_codon_eval(void);

#ifdef __cplusplus
}
#endif

#endif /* M0_CALCULUS_H */
