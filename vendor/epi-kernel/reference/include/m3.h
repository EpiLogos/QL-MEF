/**
 * m3.h — Mahamaya: The Symbolic Transcription Engine (Subsystem #3)
 *
 * Implements: M3 (#3) = Dynamic computational matrix & universal transcription.
 * Context frame: (0/1/2/3) — CF_QUATERNAL (Three-Plus-One)
 * Anchored to: Psychoid_3 in psychoid_numbers.c (Layer 1 .rodata)
 * Builds on:  ontology.h, psychoid_numbers.h, arena.h, m1.h, m2.h
 * Feeds into: M4 (Symbol DNA, helix state), M5 (Logos FSM)
 *
 * The 64 Invariant: the uint64_t IS the entire symbolic reality.
 * All state transformations are bitwise. No floating-point on the hot path.
 *
 * FR Coverage: 2.3.0 – 2.3.21 (M3-mahamaya-symbolic-transcription.md, Rev 2)
 *
 * ARCHITECTURE RULE: M3_Root has Holographic_Coordinate* hc
 * as its first field. Bind with HC_LINK(). Never access without GET_PTR().
 */

#ifndef M3_H
#define M3_H

#include "ontology.h"
#include "psychoid_numbers.h"
#include "arena.h"
#include "m0.h"
#include "m2.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


/* ===================================================================
 * FR 2.3.12: NUCLEOTIDE_ICHING_VALUE — Canonical I-Ching Numeric LUT
 *
 * A=6(Yin,Moving), T=9(Yang,Moving), C=8(yin,Resting), G=7(yang,Resting)
 * These values ARE the system's root arithmetic.
 * All pair sums, codon sums, and integral invariants derive from them.
 *
 * Parity law (m4_cast_iching, the generator): odd sums are yang, even
 * sums are yin; extreme sums (6=2+2+2, 9=3+3+3) move, mixed sums rest.
 * Classical grades: old yin 6, young YANG 7, young YIN 8, old yang 9 —
 * C (yin/resting, Pentacles) is the young yin 8; G (yang/resting,
 * Swords) is the young yang 7. Ratified 2026-09-07 (owner): corrected
 * from the parity-violating {6,9,7,8}; dataset tables regenerated with
 * this table in the same commit.
 * =================================================================== */

#define M3_ICHING_A_VALUE  6u
#define M3_ICHING_T_VALUE  9u
#define M3_ICHING_C_VALUE  8u
#define M3_ICHING_G_VALUE  7u

extern const uint8_t NUCLEOTIDE_ICHING_VALUE[4];

_Static_assert(
    M3_ICHING_A_VALUE + M3_ICHING_T_VALUE +
    M3_ICHING_C_VALUE + M3_ICHING_G_VALUE == 30,
    "NUCLEOTIDE_ICHING_VALUE must sum to 30 (6+9+8+7)"
);
_Static_assert(M3_ICHING_A_VALUE + M3_ICHING_T_VALUE == 15,
    "Complementary pair A+T must sum to 15");
_Static_assert(M3_ICHING_C_VALUE + M3_ICHING_G_VALUE == 15,
    "Complementary pair C+G must sum to 15");

/* O(1) accessor — single array index, no branch */
uint8_t get_iching_value(uint8_t nuc2bit);

/* Codon I-Ching sum: outer + middle + inner nucleotide values */
uint8_t get_codon_iching_sum(uint8_t codon6bit);


/* ===================================================================
 * FR 2.3.1: 2-BIT NUCLEOTIDE LOGIC — The Ontological Foundation
 *
 * Bit 0 (Polarity):  0 = Yin  (A, C)    1 = Yang (T, G)
 * Bit 1 (Mobility):  0 = Moving (A, T)  1 = Resting (C, G)
 *
 * A=0b00(0) Yin/Moving   — Old Yin (6)   — CUPS (Water)
 * T=0b01(1) Yang/Moving  — Old Yang (9)  — WANDS (Fire)
 * C=0b10(2) Yin/Resting  — Young Yin (8) — PENTACLES (Earth)
 * G=0b11(3) Yang/Resting — Young Yang (7)— SWORDS (Air)
 * =================================================================== */

#define M3_NUC_A  0x00
#define M3_NUC_T  0x01
#define M3_NUC_C  0x02
#define M3_NUC_G  0x03

/* FR 2.3.1a: Polarity and Mobility extraction */
#define GET_POLARITY(nuc)  ((nuc) & 0x01)
#define GET_MOBILITY(nuc)  (((nuc) >> 1) & 0x01)

/* FR 2.3.1b: Base-pairing = XOR 0x01 (flips Polarity, preserves Mobility) */
uint8_t get_base_pair(uint8_t nuc);

/* FR 2.3.1c: Codon encoding — 3 nucleotides × 2 bits = 6-bit address (0-63) */
uint8_t encode_codon(uint8_t n1, uint8_t n2, uint8_t n3);

/* Codon decomposition */
uint8_t codon_outer(uint8_t c);
uint8_t codon_middle(uint8_t c);
uint8_t codon_inner(uint8_t c);

/* FR 2.3.1d: DNA/RNA phase — single bit flips entire Polarity table */
uint8_t get_polarity_phased(uint8_t nuc, bool is_rna);

/* Nucleotide name for CLI output */
char nuc_to_char(uint8_t nuc);


/* ===================================================================
 * FR 2.3.1: HEXAGRAM / TRIGRAM OPERATIONS
 *
 * All O(1) bitwise — no LUT needed.
 * =================================================================== */

/* LINE_CHANGE: flip line n (0-5) of hexagram h — 384 edges in XOR */
uint8_t m3_line_change(uint8_t hex, uint8_t line);

/* Trigram decomposition — built into 6-bit address */
uint8_t upper_trigram(uint8_t hex_id);
uint8_t lower_trigram(uint8_t hex_id);
uint8_t compose_hexagram(uint8_t upper, uint8_t lower);

/* Complementarity: all 6 lines inverted */
uint8_t m3_complement(uint8_t hex_id);

/* Integral Symmetry on full 64-bit field (all codons simultaneously, O(1)) */
uint64_t integral_symmetry_field(uint64_t m3_word);


/* ===================================================================
 * FR 2.3.2: M3_SD_Value — Sum/Difference Pair Descriptor
 *
 * The unified 16-pair dinucleotide matrix.
 * 3-Matrix system: each of the 3 matrices has 8 pairs:
 *   4 homogeneous (AA,TT,CC,GG — SHARED across all 3) + 4 unique.
 * Matrix 1 (Watson-Crick): homos + AT,TA,CG,GC
 * Matrix 2 (Cross-compl):  homos + AG,GA,TC,CT
 * Matrix 3 (Cross-diag):   homos + AC,CA,TG,GT
 * =================================================================== */

typedef struct {
    int8_t sum_value;          /* Dataset-backed sumValue */
    int8_t difference_value;   /* Dataset-backed differenceValue */
} M3_SD_Value;

extern const M3_SD_Value M3_PAIR_MATRIX[16];

typedef enum {
    M3_MATRIX_COMPLEMENTARY = 0,
    M3_MATRIX_MOVING_RESTING = 1,
    M3_MATRIX_SAME_QUALITY = 2,
    M3_MATRIX_COUNT = 3
} M3_Matrix_Type;

extern const uint8_t M3_MATRIX_PAIR[M3_MATRIX_COUNT][4];

extern const Quaternion M3_MATRIX_QUATERNION_AXIS[M3_MATRIX_COUNT];


/* ===================================================================
 * FR 2.3.3: Rotational_State — Dynamic 8-Fold Computation
 *
 * Computed from M3_PAIR_MATRIX in O(1). No pre-stored tables.
 * 0xFF = unresolved resonance target → STATUS_PROVISIONAL.
 * This codon-indexed partial operator is independent of the total 72→64
 * epogdoon compression.
 * =================================================================== */

typedef struct {
    int8_t total_sum_value;
    int8_t total_difference_value;
} Rotational_State;

#define M3_RESONANCE_GAP  0xFF

/* STATUS_PROVISIONAL: the selected resonance target is unresolved */
#define STATUS_PROVISIONAL_BIT  (1u << 4)

Rotational_State compute_rotational_state(uint8_t p1_idx, uint8_t p2_idx);

/* Safe version: handles an unresolved resonance target (0xFF sentinel) */
bool compute_rotational_state_safe(
    uint8_t p1_idx, uint8_t p2_idx,
    Rotational_State* out, uint32_t* coord_flags);

typedef struct {
    Quaternion q;
    uint8_t    active_state;
    uint8_t    codon_id;
} M3_Codon_Quaternion;

/* Ring-position shortcut: embeds the codon id as a 64-step angular position.
 * Use for torus/ring placement only; it is NOT the elemental bioquaternion. */
Quaternion m3_quat_from_codon(uint8_t codon_id);

Quaternion m3_quat_codon_state(uint8_t codon_id, uint8_t state);

uint8_t m3_quat_active_state(Quaternion env, uint8_t codon_id);


/* ===================================================================
 * FR 2.3.1: TRIGRAM AND HEXAGRAM STRUCTS
 * =================================================================== */

typedef struct {
    uint8_t  id;             /* 0-7 (3 bits) */
    uint8_t  binary;         /* 3-bit line pattern (yang=1, yin=0) */
    uint8_t  earlier_heaven; /* Position in Earlier Heaven (0-7) */
    uint8_t  later_heaven;   /* Position in Later Heaven (0-7) */
    uint8_t  element;        /* Five-Element correspondence */
    uint8_t  family_role;    /* Father/Mother/Son/Daughter */
    uint16_t degree_anchor;  /* Degree position in #3-5 wheel */
} M3_Trigram;

typedef struct {
    uint8_t id;              /* 0-63 (6 bits) = upper<<3 | lower */
    uint8_t line_pattern;    /* 6-bit pattern (lines 0-5, bottom to top) */
    uint8_t complement_id;   /* id ^ 0x3F */
    uint8_t nuclear_upper;   /* Nuclear trigram (lines 2-4) */
    uint8_t nuclear_lower;   /* Nuclear trigram (lines 1-3) */
} M3_Hexagram;

extern const M3_Trigram  M3_TRIGRAM_LUT[8];
extern const M3_Hexagram M3_HEXAGRAM_LUT[64];

/* I-Ching state: which hexagrams are active */
typedef struct {
    uint64_t active;          /* Bitboard: bit h set = hexagram h active */
    uint8_t  current;         /* Currently focused hexagram (6-bit) */
    uint8_t  changing_lines;  /* Which lines are changing (6-bit mask) */
    uint8_t  result;          /* Hexagram after changes applied */
    bool     is_rna_phase;    /* DNA/RNA superposition flag */
} M3_IChing_State;


/* ===================================================================
 * FR 2.3.4: TAROT SUIT ENUM + MINOR ARCANA CARD
 *
 * Suit index = nucleotide 2-bit value. No translation needed.
 * =================================================================== */

typedef enum {
    SUIT_CUPS      = 0, /* A family — Yin/Moving  — Water */
    SUIT_WANDS     = 1, /* T family — Yang/Moving — Fire  */
    SUIT_PENTACLES = 2, /* C family — Yin/Resting — Earth */
    SUIT_SWORDS    = 3  /* G family — Yang/Resting — Air  */
} Tarot_Suit;

extern const char* const SUIT_NAMES[4];

typedef struct {
    uint8_t    card_id;        /* 0-55 (Minor Arcana ID) */
    Tarot_Suit suit;           /* Elemental suit (0-3) */
    uint8_t    rank;           /* 1=Ace, 2-10=Pip, 11=Page, 12=Knight, 13=Queen, 14=King */
    uint8_t    primary_codon;  /* 6-bit codon ID (0-63) */
    uint8_t    shadow_codon;   /* 6-bit codon ID, or 0xFF for single-codon cards */
} Minor_Arcana_Card;


/* ===================================================================
 * FR 2.3.5: UNIFIED COSMIC CLOCK — O(1)
 *
 * Maps any degree (0-719) across full SU(2) 720° double-cover.
 * All integer arithmetic. No floating-point.
 *
 * NOTE: Unified_Clock_State is defined in m0.h (shared across M-branches).
 * m0_read_cosmic_clock() is the canonical implementation.
 * We provide read_cosmic_clock() as a convenience alias when m0.h is included,
 * and a standalone version when it is not.
 * =================================================================== */

/* m0.h is now always included — Unified_Clock_State and m0_read_cosmic_clock() available */
Unified_Clock_State read_cosmic_clock(uint16_t d);


/* ===================================================================
 * FR 2.3.6: EPOGDOON BRIDGE — M2 ↔ M3 Harmonic Translation
 *
 * 9:8 ratio. 72 × (8/9) = 64. Integer-only. O(1).
 * The 8 missing states drive evolutionary spiral.
 * =================================================================== */

/* ASCENDING: M3 → M2 (rotational sumValue → Parashakti frequency index) */
uint8_t get_parashakti_frequency(Rotational_State m3_state,
                                                bool is_shadow_phase);

/* DESCENDING: M2 → M3 (epogdoon 9:8 compression) */
uint8_t apply_epogdoon_compression(uint8_t m2_idx_0_to_71);

/* 9:8 ROUND-TRIP LOSS DETECTION
 * True when expand(compress(address72)) does not return to address72.
 * This reports 64 non-exact round trips; it is distinct from the eight
 * compression collision pairs and the denominator block size nine. */
bool epogdoon_has_round_trip_loss(uint8_t m2_vibration_index);


/* ===================================================================
 * FR 2.3.7: SU(2)-PRESERVING POLAR OPPOSITE
 *
 * Preserves shadow layer: opposing a shadow-layer degree
 * stays in the shadow layer.
 * =================================================================== */

uint16_t polar_opposite_su2(uint16_t current_720_degree);

/* Simple wheel operations (single 360° layer) */
uint16_t flow_clockwise(uint16_t d);
uint16_t polar_opposite_simple(uint16_t d);
uint8_t  m3_quadrant(uint16_t d);


/* ===================================================================
 * FR 2.3.8: STATUS_PROVISIONAL / FR 2.3.9: RESONANCE LOOKUP
 * =================================================================== */

extern const uint8_t M3_RES_MATRIX[64];

uint8_t m3_resonance_lookup(uint8_t codon_id,
                                           uint32_t* coord_flags);


/* ===================================================================
 * FR 2.3.9: M3_Transcription_Engine — Operational Hub
 * =================================================================== */

extern const uint8_t M3_COMP_MATRIX[64];
extern const uint8_t M3_MOVE_MATRIX[64];
extern const uint8_t M3_CODON_TO_AA[64];

/* STOP-codon sentinel as it actually appears in M3_CODON_TO_AA.
 * The amino-acid index 10 marks the three STOP codons (TAA/TAG/TGA).
 * (The header comment on M3_CODON_TO_AA historically said "0xFF = STOP";
 * the live table encodes STOP as amino-acid index 10, so callers must
 * compare against this constant, not 0xFF.) */
#define M3_STOP_CODON_AA  10u

typedef enum {
    M3_TRANSCRIPT_CLASS_SHARED = 0,
    M3_TRANSCRIPT_CLASS_TRANSCRIBABLE = 1
} M3_TranscriptClass;

typedef enum {
    M3_GOVERNANCE_ROLE_NONE = 0,
    M3_GOVERNANCE_ROLE_START = 1,
    M3_GOVERNANCE_ROLE_STOP = 2
} M3_GovernanceRole;

#define M3_CODON_ATG_AUG_VALUE  0x07u
#define M3_STOP_CODON_TAA_VALUE 0x10u
#define M3_STOP_CODON_TAG_VALUE 0x13u
#define M3_STOP_CODON_TGA_VALUE 0x1Cu

_Static_assert(M3_CODON_ATG_AUG_VALUE == 0x07u,
    "M3_CODON_ATG_AUG must match portal-core transcription.rs START_CODON");
_Static_assert(M3_STOP_CODON_TAA_VALUE == 0x10u &&
               M3_STOP_CODON_TAG_VALUE == 0x13u &&
               M3_STOP_CODON_TGA_VALUE == 0x1Cu,
    "M3_STOP_CODONS must match portal-core transcription.rs STOP_CODONS");
_Static_assert(M3_CODON_ATG_AUG_VALUE ==
               ((M3_NUC_A << 4) | (M3_NUC_T << 2) | M3_NUC_G),
    "M3_CODON_ATG_AUG must encode ATG/AUG");
_Static_assert(M3_STOP_CODON_TAA_VALUE ==
               ((M3_NUC_T << 4) | (M3_NUC_A << 2) | M3_NUC_A),
    "M3_STOP_CODONS[0] must encode TAA");
_Static_assert(M3_STOP_CODON_TAG_VALUE ==
               ((M3_NUC_T << 4) | (M3_NUC_A << 2) | M3_NUC_G),
    "M3_STOP_CODONS[1] must encode TAG");
_Static_assert(M3_STOP_CODON_TGA_VALUE ==
               ((M3_NUC_T << 4) | (M3_NUC_G << 2) | M3_NUC_A),
    "M3_STOP_CODONS[2] must encode TGA");

extern const uint8_t M3_CODON_ATG_AUG;
extern const uint8_t M3_STOP_CODONS[3];

uint8_t m3_codon_t_count(uint8_t codon6bit);

M3_TranscriptClass m3_codon_transcript_class(uint8_t codon6bit);

M3_GovernanceRole m3_codon_governance_role(uint8_t codon6bit);

uint8_t m3_codon_t_count_ffi(uint8_t codon6bit);
M3_TranscriptClass m3_codon_transcript_class_ffi(uint8_t codon6bit);
M3_GovernanceRole m3_codon_governance_role_ffi(uint8_t codon6bit);
int m3_verify_transcript_surface(void);

typedef struct {
    /* Non-dual foundation */
    uint64_t non_dual_mask;             /* 40 always-set bits */

    /* Dual field */
    uint64_t dual_field[6];             /* 336 polarity pairs (6 × 64-bit words) */

    /* Three matrix operators (.rodata pointers) */
    const uint8_t* comp_matrix;
    const uint8_t* move_matrix;
    const uint8_t* res_matrix;

    /* Transcription pipeline */
    const uint8_t* codon_to_aa;

    /* STATUS_PROVISIONAL tracking */
    uint32_t coord_flags;
} M3_Transcription_Engine;


/* ===================================================================
 * FR 2.3.11: 360-DEGREE MYTHIC SYNTHESIS WHEEL
 * =================================================================== */

typedef struct {
    uint16_t degree;            /* 0-359 (primary) */
    uint16_t shadow;            /* degree + 360 (shadow layer) */
    uint8_t  quadrant;          /* 0-3 (N/E/S/W) */
    uint8_t  sector;            /* 0-23 (15° sectors via 24-spoke division) */
    uint8_t  trigram_earlier;   /* Earlier Heaven trigram at this position */
    uint8_t  trigram_later;     /* Later Heaven trigram at this position */
    uint8_t  sign;              /* Zodiacal sign (0-11, 30° each) */
    uint8_t  decan;             /* Decan within sign (0-2, 10° each) */
} M3_DegreePosition;

typedef struct {
    uint16_t current_degree;    /* Active degree (0-719 for full double-cover) */
    uint8_t  layer;             /* 0 = primary (0-359), 1 = shadow (360-719) */
    uint64_t active_sectors;    /* Bitboard: which of 24 sectors are active */
    uint8_t  season;            /* Current quarter (0-3) */
    uint8_t  heaven_mode;       /* 0 = Earlier, 1 = Later */
} M3_Wheel_State;

/* Ring buffer — 12-deep for SU(2) double-covering (6 QL × 2) */
typedef struct {
    uint16_t buffer[12];
    uint8_t  head;
    uint8_t  count;
} M3_Ring_Buffer;

/* Full rotation requires 720° */
bool identity_returned(const M3_Wheel_State* ws);


/* ===================================================================
 * FR 2.3.13: 4-EVALUATION SYSTEM — Codon Charge Calculator
 *
 * Every codon evaluated across 4 complementary valence axes.
 * Composes additively from dinucleotide pair S/D values.
 * =================================================================== */

typedef struct {
    int8_t pp;   /* (+,+): positive resonance integral */
    int8_t mm;   /* (-,-): negative resonance integral */
    int8_t mp;   /* (-,+): ascending resonance */
    int8_t pm;   /* (+,-): descending resonance */
} M3_CodonEvaluation;

/* Evaluate codon from the dataset-native inner charge identities. */
M3_CodonEvaluation evaluate_codon(uint8_t codon6bit);

/* Canonical elemental bioquaternion path:
 *   m3_compute_charges/evaluate_codon -> m3_eval_to_quat
 *   { w=pp->Earth, x=nn/mm->Fire, y=np/mp->Water, z=pn/pm->Air }.
 * Renderers must use this path for elemental quaternion reads; do not use
 * m3_quat_from_codon except for the ring-position shortcut declared above. */
Quaternion m3_eval_to_quat(M3_CodonEvaluation eval);

M3_CodonEvaluation m3_quat_to_eval(Quaternion q);


/* ===================================================================
 * FR 2.3.14: 8-FOLD ROTATIONAL COMPOSITION LAW
 *
 * Positive valence:  Xy + Za → Xya
 * Negative valence:  Xy + Za → XZa
 * Non-dual:          when y == Z (bipolar)
 * =================================================================== */

#define M3_ROTATIONAL_TABLE_ENTRIES       8
#define M3_ROTATIONAL_POLARIZED_ENTRIES   7
#define M3_ROTATIONAL_NONDUAL_ENTRIES     1
#define M3_ROTATIONAL_NO_PAIR             0xFF
#define M3_ROTATIONAL_NO_PAIRING          0xFF

typedef enum {
    M3_ROTATIONAL_NEGATIVE = 0,
    M3_ROTATIONAL_POSITIVE = 1
} M3_Rotational_Polarity;

typedef enum {
    M3_ROTATIONAL_NON_DUAL_INITIATED = 0,
    M3_ROTATIONAL_FULL_ROTATIONAL    = 1
} M3_Rotational_State_Type;

typedef struct {
    uint8_t pair1_idx;
    uint8_t pair2_idx;
    uint8_t resulting_codon;
    uint8_t polarity;
    uint8_t rotation_slot;
    uint16_t rotation_degrees;
    int8_t rotational_value;
    bool is_non_dual;
} M3_Rotational_Generation;

typedef struct {
    uint8_t state_count;   /* dataset-backed 7 or 8 */
    uint8_t state_type;    /* non-dual-initiated or full-rotational */
    uint8_t anchor_pair_a; /* pair idx, or 0xFF if not applicable */
    uint8_t anchor_pair_b; /* pair idx, or 0xFF if not applicable */
    uint8_t paired_codon;  /* paired codon reflection, or 0xFF */
} M3_Rotational_Profile;

uint8_t m3_encode_pair(uint8_t n1, uint8_t n2);

uint8_t m3_pair_first(uint8_t pair_idx);

uint8_t m3_pair_second(uint8_t pair_idx);

uint8_t compose_rotational_state(uint8_t xy, uint8_t za, int positive);

int is_nondual_composition(uint8_t xy, uint8_t za);

extern const M3_Rotational_Profile M3_ROTATIONAL_PROFILE[64];


/* ===================================================================
 * FR 2.3.15: 360 INTEGRAL INVARIANT
 *
 * Sum of all (+,+) codon evaluations = 360.
 * Per-suit: Cups=84, Wands=96, Pentacles=92, Swords=88.
 * =================================================================== */

#define M3_INTEGRAL_INVARIANT    360U
#define M3_SUIT_A_INTEGRAL        84U   /* Cups:      Yin Moving  */
#define M3_SUIT_T_INTEGRAL        96U   /* Wands:     Yang Moving */
#define M3_SUIT_C_INTEGRAL        92U   /* Pentacles: yin Resting */
#define M3_SUIT_G_INTEGRAL        88U   /* Swords:    yang Resting */

_Static_assert(
    M3_SUIT_A_INTEGRAL + M3_SUIT_T_INTEGRAL +
    M3_SUIT_C_INTEGRAL + M3_SUIT_G_INTEGRAL == M3_INTEGRAL_INVARIANT,
    "M3 360 Integral Invariant violated: suit sum must equal 360"
);


/* ===================================================================
 * FR 2.3.16: M3_TarotCodonEntry — Canonical LUT
 *
 * 4 suits × 16 slots = 64 codons total.
 * Yin suits (Cups/Pentacles): Knight+King dual-codon
 * Yang suits (Wands/Swords): Page+Queen dual-codon
 * =================================================================== */

typedef struct {
    uint8_t suit;     /* 0=Cups, 1=Wands, 2=Pentacles, 3=Swords */
    uint8_t pip;      /* 0=Ace, 1-9=Two-Ten, 10=Princess, 11=Prince, 12=Queen, 13=King */
    uint8_t codon_a;  /* Primary codon (6-bit) */
    uint8_t codon_b;  /* Secondary codon for dual-codon courts, 0xFF = none */
} M3_TarotCodonEntry;

#define M3_TAROT_PIP_ACE        0
#define M3_TAROT_PIP_PRINCESS  10
#define M3_TAROT_PIP_PRINCE    11
#define M3_TAROT_PIP_QUEEN     12
#define M3_TAROT_PIP_KING      13
#define M3_TAROT_SINGLE_CODON  0xFF
#define M3_TAROT_ENTRIES_PER_SUIT  16
#define M3_TAROT_SUITS             4

extern const M3_TarotCodonEntry M3_TAROT_CODON_MAP[4][16];

extern const char* const TAROT_RANK_NAMES[14];

#define M3_MINOR_ARCANA_COUNT         56u
#define M3_MAJOR_ARCANA_COUNT         22u
#define M3_TRANSCENDENT_TAROT_COUNT    2u
#define M3_TAROT_QUATERNION_COUNT     80u

typedef struct {
    uint8_t     card_id;
    const char* name;
    uint8_t     chromosome_pair;  /* 1-22 */
    uint8_t     amino_acid_index; /* 0-21 */
} M3_Major_Arcana_Entry;

extern const M3_Major_Arcana_Entry M3_MAJOR_ARCANA[M3_MAJOR_ARCANA_COUNT];

typedef struct {
    uint8_t    card_id;
    uint8_t    suit;          /* 0-3 for minor, 0xFF for major/transcendent */
    uint8_t    rank;          /* 0-13 minor, 0-21 major, 0-1 transcendent */
    uint8_t    primary_codon; /* 0xFF for major/transcendent */
    uint8_t    shadow_codon;  /* 0xFF if none */
} M3_Tarot_Quaternion_Entry;

/* Dataset-backed Tarot bridge metadata:
 * 56 Minor Arcana cards from elemental codon mappings,
 * 22 Major Arcana cards mapped to autosomal chromosome pairs,
 * 2 transcendent operators beyond the 22-fold autosomal cycle. */

/* Compile-time mask values as integer constant expressions (usable in
 * _Static_assert); the extern const symbols below carry the identical values
 * into runtime / FFI / tests. Mirrors the M3_CODON_*_VALUE idiom above. */
#define M3_RNA_FUNCTIONAL_MASK_VALUE 0x22F222F2FFFF22F2ULL
#define M3_RNA_DARK_MASK_VALUE       0xDD0DDD0D0000DD0DULL

extern const uint64_t M3_RNA_FUNCTIONAL_MASK;
extern const uint64_t M3_RNA_DARK_MASK;


/* ===================================================================
 * FR 2.3.17: NON-DUAL CODONS — Palindromic Anchors (XyX Pattern)
 *
 * 16 codons where outer == inner nucleotide.
 * Zero net directional deviation. Bipolar valence.
 * =================================================================== */

int is_nondual_codon(uint8_t codon6bit);

#define M3_NONDUAL_CODON_FLAG   0x40

extern const uint8_t M3_NONDUAL_CODONS[16];


/* ===================================================================
 * FR 2.3.17b: CODON CLASSIFICATION — 3-Tier (40 Non-Dual / 24 Dual)
 *
 * Algorithmic classification from nucleotide positions:
 *   n1==n3 && n1==n2  → PERFECT_PALINDROMIC    (4:  AAA,TTT,CCC,GGG)
 *   n1==n3 && n1!=n2  → IMPERFECT_PALINDROMIC  (12: XyX, X≠Y)
 *   n1!=n3 && (n1==n2 || n2==n3) → NON_PALINDROMIC_NONDUAL (24: repeated dinucleotide)
 *   n1!=n2 && n2!=n3 && n1!=n3   → DUAL (24: all neighbors differ)
 *
 * Non-dual = has at least one repeated adjacent pair → eigenstate
 * collapse at 0°/180° in rotational system → 7 states.
 * Dual = all distinct neighbor pairs → full 8 rotational states.
 *
 * Counts: 4 + 12 + 24 + 24 = 64 codons.
 * Rotational states: 40×7 + 24×8 = 280 + 192 = 472 total.
 * =================================================================== */

typedef enum {
    CODON_PERFECT_PALINDROMIC     = 0,  /* 4:  homogeneous triplet, 7 states */
    CODON_IMPERFECT_PALINDROMIC   = 1,  /* 12: XyX where X≠Y, 7 states */
    CODON_NON_PALINDROMIC_NONDUAL = 2,  /* 24: repeated dinucleotide, 7 states */
    CODON_DUAL                    = 3   /* 24: all neighbors differ, 8 states */
} Codon_Class;

#define CODON_NONDUAL_TOTAL_COUNT         40u
#define CODON_DUAL_COUNT                  24u
#define CODON_ROTATIONAL_STATE_TOTAL     472u

_Static_assert(CODON_NONDUAL_TOTAL_COUNT + CODON_DUAL_COUNT == 64,
    "40 non-dual + 24 dual = 64 codons");
_Static_assert(CODON_NONDUAL_TOTAL_COUNT * 7 + CODON_DUAL_COUNT * 8
    == CODON_ROTATIONAL_STATE_TOTAL,
    "40×7 + 24×8 = 472 rotational states");

typedef struct {
    Codon_Class cls;
    uint8_t     rotational_state_count;  /* 7 or 8 */
    uint8_t     paired_codon;            /* WC anticodon */
    uint8_t     amino_acid;              /* M3_CODON_TO_AA index */
} Codon_Classification;

/* Algorithmic classifier — O(1), no lookup, pure arithmetic */
Codon_Class m3_classify_codon(uint8_t codon6bit);

bool codon_is_dual(Codon_Class c);
bool codon_is_non_dual(Codon_Class c);
bool codon_is_palindromic(Codon_Class c);
bool codon_is_perfect_palindrome(Codon_Class c);

uint8_t m3_codon_rotation_count(uint8_t codon6bit);

/* Master LUT — initialized by m3_init_codon_class_lut(). Read-only after init. */
extern Codon_Classification M3_CODON_CLASS[64];
void m3_init_codon_class_lut(void);


/* ===================================================================
 * FR 2.3.18: INNER CHARGE CLOSED-FORM DERIVATION
 *
 * pp = X+Y+Z, nn = X-Y-Z, np = X-Y+Z, pn = X+Y-Z
 * where X,Y,Z = NUCLEOTIDE_ICHING_VALUE of outer,middle,inner.
 * No lookup table needed — pure arithmetic.
 * =================================================================== */

void m3_compute_charges(
    uint8_t codon6bit,
    int8_t *pp_out, int8_t *nn_out, int8_t *np_out, int8_t *pn_out);

/* FFI-exportable non-inline wrapper for Rust / foreign callers.
   Definition in m3.c. */
void m3_compute_charges_ffi(
    uint8_t codon6bit,
    int8_t *pp_out, int8_t *nn_out, int8_t *np_out, int8_t *pn_out);

/* Compile-time verification: TTT 4X invariant (pp+nn+np+pn = 4*outer) */
_Static_assert((9+9+9) + (9-9-9) + (9-9+9) + (9+9-9) == 4*9,
    "m3_compute_charges 4X invariant must hold for TTT codon");

#define M3_EULER_PRIME_41  41u
#define M3_EULER_PRIME_43  43u

int m3_is_prime_attractor(uint8_t codon_a, uint8_t codon_b);


/* ===================================================================
 * FR 2.3.20: DNA/RNA SUPERPOSITION
 *
 * containsT = true → RNA-capable (T→U substitution possible).
 * =================================================================== */

int m3_codon_is_rna_capable(uint8_t codon6bit);

Quaternion m3_element_to_quat(Elemental_Signature sig);

Quaternion m3_tarot_rotation(uint8_t card_id);

uint8_t m3_tarot_translate(
    uint8_t card_id,
    uint8_t source_pos,
    int codon_to_hexagram);


/* ===================================================================
 * FR 2.3.17: NON-DUAL GUARD — Evaluation with flag
 * =================================================================== */

M3_CodonEvaluation m3_evaluate_with_nondual_guard(
    uint8_t codon6bit, uint8_t *flags_out);

typedef struct {
    uint64_t   active_mask;
    uint8_t    codon_states[64];
    Quaternion composed_q;
    QL_Tick    torus_tick;
} M3_DET_Overlay_Result;


/* ===================================================================
 * M3 ROOT STRUCT — HC-anchored module state
 * =================================================================== */

typedef struct {
    Holographic_Coordinate*       hc;         /* FIRST FIELD — HC_LINK'd to Psychoid_3 mirror */
    const Holographic_Coordinate* active_cf;  /* CF_TABLE[CF_QUATERNAL] */
    M3_IChing_State               iching;     /* Active hexagram state */
    M3_Transcription_Engine       engine;     /* Operational hub */
    M3_Wheel_State                wheel;      /* 360° wheel state */
    M3_Ring_Buffer                ring;       /* 12-deep SU(2) history */
} M3_Root;


/* ===================================================================
 * PUBLIC API
 * =================================================================== */

/* Allocate and HC-link M3_Root; hc must be Psychoid_3's mutable mirror */
M3_Root* m3_init(Coordinate_Arena* arena, Holographic_Coordinate* hc);

/* Release M3_Root heap state (not the HC itself) */
void     m3_teardown(M3_Root* root);

/* CLI dispatch entry point: argv[0] = "m3" */
int      m3_cli_dispatch(int argc, char** argv, M3_Root* root);

M3_DET_Overlay_Result m3_det_with_quaternion(
    const M2_Root* m2,
    const M3_Root* m3,
    QL_Tick torus_tick,
    M3_Matrix_Type matrix);

size_t m3_generate_rotational_states(
    uint8_t codon6bit,
    M3_Rotational_Generation out[M3_ROTATIONAL_TABLE_ENTRIES]);

const M3_Rotational_Profile* m3_get_rotational_profile(uint8_t codon6bit);

/* Transcribe a 6-bit codon to its Major Arcana card index.
 *
 * Composition: codon -> M3_CODON_TO_AA[codon] (amino-acid index) ->
 * reverse-lookup against M3_MAJOR_ARCANA[].amino_acid_index. Used by the
 * M5 Mobius return to transcribe each codon in a session's M3 trace.
 *
 * Returns the card index (0-21) on a match, or 0xFF for STOP codons and
 * any amino-acid index with no Major Arcana assignment. */
uint8_t  m3_major_arcana_from_codon(uint8_t codon);

/* Boot-time verification of .rodata integrity */
bool     m3_verify(void);


/* ─────────────────────────────────────────────────────────────────────────
 * CLOCK_DEGREE_LUT — 360-entry lookup table mapping degree nodes to
 * all four M-layer data (M0-M3). Built by tools/build_clock_degree_lut.py.
 *
 * Topology law (compile-time verified):
 *   384 = 360 dynamic degrees + 24 palindromic backbone nodes = 64 × 6 lines
 *
 * Fields that require Neo4j dataset query to be accurate:
 *   hexagram_id, hexagram_line_active, codon_upper_pair, codon_lower_pair,
 *   tarot_card_id, is_non_dual_codon (precise), m1_ananda_value, m0_archetype
 *
 * Fields computable from first principles (populated by build_clock_degree_lut.py):
 *   degree_node_360, zodiac_sign, zodiac_degree, decan_idx, decan_position,
 *   is_backbone_node, decan_planet, decan_element, decan_chakra, tick12,
 *   strand, shadow_degree, polar_opposite, exact_degree_720,
 *   enneadic_chamber, dr_ring
 * ─────────────────────────────────────────────────────────────────────── */

/* Structural law: 360 dynamic degrees + 24 palindromic backbone = 64×6 LINE_CHANGES */
_Static_assert(360 + 24 == 64 * 6,
    "Clock topology: 360 degree nodes + 24 backbone = 384 = 64 hexagrams x 6 lines");
_Static_assert(64 * 6 - 24 == 360,
    "Clock topology: 64 hexagrams x 6 lines - 24 backbone = 360 degree nodes");
_Static_assert(60 * 6 == 360,
    "Fibonacci ground period × 6°/step = full clock");

typedef struct {
    uint16_t degree;
    uint8_t  backbone_index;
    uint8_t  hour_of_day;
    uint8_t  zodiac_sign;
    uint8_t  is_cusp;
    uint8_t  amino_acid_idx;
    uint8_t  is_palindromic;
    uint8_t  _pad[4];
} Clock_Backbone_Node;

_Static_assert(sizeof(Clock_Backbone_Node) == 12,
    "Clock_Backbone_Node must be exactly 12 bytes");
_Static_assert(24 % 12 == 0, "24 backbone nodes must evenly contain 12 zodiac nodes");
_Static_assert(12 % 4 == 0, "12 zodiac nodes must evenly contain 4 cardinal nodes");
_Static_assert(360 % 24 == 0, "360 degree nodes must evenly span 24 backbone intervals");

#ifdef M3_BUILDING_SOURCE
extern Clock_Backbone_Node CLOCK_BACKBONE[24];
#else
extern const Clock_Backbone_Node CLOCK_BACKBONE[24];
#endif
void m3_build_backbone(void);

/* Fibonacci ground digit cycle (Track 35 §1.2, consumed by Tranche 5.24):
 * the Pisano period of Fibonacci mod 10 is exactly 60 — one full clock of
 * 6° steps (60 × 6 == 360, asserted above). pisano_digit_lut[n] = F(n) % 10;
 * the cycle closes: F(60) ≡ 0, F(61) ≡ 1 (mod 10). */
extern const uint8_t pisano_digit_lut[60];

typedef struct {
    /* Identity */
    uint16_t degree_node_360;       /* 0-359  LUT index                         */
    float    exact_degree_720;      /* degree * 2.0  in [0.0, 720.0)            */

    /* Ring membership */
    uint8_t  zodiac_sign;           /* 0-11   Aries=0 … Pisces=11               */
    uint8_t  zodiac_degree;         /* 0-29   degree within the zodiac sign      */
    uint8_t  decan_idx;             /* 0-35   floor(degree/10)                   */
    uint8_t  decan_position;        /* 0-9    degree within the decan            */
    uint8_t  is_backbone_node;      /* 1 if this is a palindromic anchor (d%15==0) */

    /* M3 data — symbolic transcription layer (Neo4j-sourced when available) */
    uint8_t  hexagram_id;           /* 0-63   I-Ching hexagram                   */
    uint8_t  hexagram_line_active;  /* 0-5    which of the 6 lines at this degree */
    uint8_t  is_non_dual_codon;     /* 1 if palindromic/non-dual codon           */
    uint8_t  codon_class;           /* Codon_Class: 0=perfect,1=imperfect,2=nonpal_nd,3=dual */
    uint8_t  codon_upper_pair;      /* 0-3    upper nucleotide (A/T/C/G)         */
    uint8_t  codon_lower_pair;      /* 0-3    lower nucleotide                   */
    uint8_t  tarot_card_id;         /* 0-55   Minor Arcana (0=unavailable)       */

    /* M2 data — vibrational/elemental layer */
    uint8_t  decan_planet;          /* 0-9    canonical mod-10 decan ruler       */
    uint8_t  decan_element;         /* 0-4    FIRE/EARTH/AIR/WATER/AKASHA        */
    uint8_t  decan_chakra;          /* 0-7    chakra from PLANET_CHAKRA[planet]  */

    /* M1 data — mathematical/harmonic layer */
    uint8_t  tick12;                /* 0-11   canonical spanda stage             */
    uint8_t  strand;                /* 0=Strand-A/explicate  1=Strand-B/implicate */
    uint8_t  dr_ring;               /* 0=Mahamaya{1,2,4,8,7,5} 1=Parashakti{3,6,9} */
    uint8_t  m1_ananda_value;       /* ananda harmonic (0=unavailable)           */

    /* M0 data — archetypal number language */
    uint8_t  m0_archetype;          /* 0-11   Archetype_LUT index (0=unavailable) */

    /* SU(2) double-cover shadow identity */
    uint16_t shadow_degree;         /* degree + 360 in the 720° double-cover     */
    uint16_t polar_opposite;        /* (degree + 180) % 360                      */

    /* Enneadic chamber */
    uint8_t  enneadic_chamber;      /* 0-8    which 40° chamber (degree/40)      */
    uint8_t  chamber_day_night;     /* 0=day (first 20°)  1=night (second 20°)  */
} Clock_Degree_Entry;

extern const Clock_Degree_Entry CLOCK_DEGREE_LUT[360];


#endif /* M3_H */
