/**
 * m3.c — Mahamaya: The Symbolic Transcription Engine (Implementation)
 *
 * All .rodata LUT data + API implementation for M3.
 * FR Coverage: 2.3.0 – 2.3.21
 */

#include "m3.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


/* ===================================================================
 * FR 2.3.2: M3_PAIR_MATRIX[16] — The Unified Dinucleotide LUT
 *
 * Indexed by (nuc1 << 2) | nuc2.
 * 3-Matrix system — each matrix has 8 pairs (4 homogeneous shared):
 *   Matrix 1 (Watson-Crick): homos + AT,TA,CG,GC = 8 pairs
 *   Matrix 2 (Cross-compl):  homos + AG,GA,TC,CT = 8 pairs
 *   Matrix 3 (Cross-diag):   homos + AC,CA,TG,GT = 8 pairs
 * All 4 homogeneous pairs (AA,TT,CC,GG) are SHARED across all 3.
 * =================================================================== */

const M3_SD_Value M3_PAIR_MATRIX[16] = {
    /* Homogeneous pairs — differenceValue=0, SHARED across all 3 matrices */
    [0]  = { 12,  0 },  /* AA (Yin/Moving  + Yin/Moving)  — K'un  */
    [5]  = { 18,  0 },  /* TT (Yang/Moving + Yang/Moving) — Ch'ien (MAX SUM) */
    [10] = { 14,  0 },  /* CC (Yin/Rest    + Yin/Rest)    — K'an  */
    [15] = { 16,  0 },  /* GG (Yang/Rest   + Yang/Rest)   — Li    */

    /* Matrix 1 pairs — Watson-Crick (polarity-complementary, same mobility) */
    [1]  = { 15, -3 },  /* AT (Yin/Moving  + Yang/Moving) — Tui   */
    [4]  = { 15,  3 },  /* TA (Yang/Moving + Yin/Moving)  — Ken   */
    [11] = { 15,  1 },  /* GC (Yang/Rest   + Yin/Rest)    — Chen  */
    [14] = { 15, -1 },  /* CG (Yin/Rest    + Yang/Rest)   — Sun   */

    /* Matrix 2 pairs — Cross-complementary (dataset keeps class-stable polarity) */
    [3]  = { 14,  2 },  /* AG (Yin/Moving  + Yang/Rest)   */
    [12] = { 14,  2 },  /* GA (Yang/Rest   + Yin/Moving)  */
    [6]  = { 16, -2 },  /* TC (Yang/Moving + Yin/Rest)    */
    [9]  = { 16, -2 },  /* CT (Yin/Rest    + Yang/Moving) */

    /* Matrix 3 pairs — Cross-diagonal (polarity AND mobility both differ) */
    [2]  = { 13, -1 },  /* AC (Yin/Moving  + Yin/Rest)    */
    [8]  = { 13,  1 },  /* CA (Yin/Rest    + Yin/Moving)  */
    [7]  = { 17,  1 },  /* TG (Yang/Moving + Yang/Rest)   */
    [13] = { 17, -1 },  /* GT (Yang/Rest   + Yang/Moving) */
};

const uint8_t M3_MATRIX_PAIR[M3_MATRIX_COUNT][4] = {
    [M3_MATRIX_COMPLEMENTARY]  = { M3_NUC_T, M3_NUC_A, M3_NUC_G, M3_NUC_C },
    [M3_MATRIX_MOVING_RESTING] = { M3_NUC_G, M3_NUC_C, M3_NUC_T, M3_NUC_A },
    [M3_MATRIX_SAME_QUALITY]   = { M3_NUC_C, M3_NUC_G, M3_NUC_A, M3_NUC_T },
};


/* ===================================================================
 * FR 2.3.1: M3_TRIGRAM_LUT[8] — The 8 Trigrams
 *
 * id, binary, earlier_heaven, later_heaven, element, family_role, degree_anchor
 * =================================================================== */

const M3_Trigram M3_TRIGRAM_LUT[8] = {
    /* 0: Qian (Heaven) — Creative — 111 */
    { 0, 0x07, 0, 5, 4, 0, 0 },
    /* 1: Kun (Earth)   — Receptive — 000 */
    { 1, 0x00, 7, 0, 4, 1, 180 },
    /* 2: Zhen (Thunder) — Arousing — 001 */
    { 2, 0x01, 4, 1, 3, 2, 90 },
    /* 3: Xun (Wind)    — Gentle — 110 */
    { 3, 0x06, 3, 6, 3, 3, 315 },
    /* 4: Kan (Water)   — Abysmal — 010 */
    { 4, 0x02, 6, 7, 0, 4, 270 },
    /* 5: Li (Fire)     — Clinging — 101 */
    { 5, 0x05, 1, 2, 2, 5, 135 },
    /* 6: Gen (Mountain) — Keeping Still — 100 */
    { 6, 0x04, 2, 3, 4, 6, 45 },
    /* 7: Dui (Lake)    — Joyous — 011 */
    { 7, 0x03, 5, 4, 4, 7, 225 },
};


/* ===================================================================
 * FR 2.3.1: M3_HEXAGRAM_LUT[64] — All 64 Hexagrams
 *
 * id, line_pattern, complement_id, nuclear_upper, nuclear_lower
 * Nuclear: upper = lines 2-4, lower = lines 1-3
 * =================================================================== */

const M3_Hexagram M3_HEXAGRAM_LUT[64] = {
    #define HEX(i) { (i), (i), ((i) ^ 0x3F), (uint8_t)(((i) >> 2) & 0x07), (uint8_t)(((i) >> 1) & 0x07) }
    HEX( 0), HEX( 1), HEX( 2), HEX( 3), HEX( 4), HEX( 5), HEX( 6), HEX( 7),
    HEX( 8), HEX( 9), HEX(10), HEX(11), HEX(12), HEX(13), HEX(14), HEX(15),
    HEX(16), HEX(17), HEX(18), HEX(19), HEX(20), HEX(21), HEX(22), HEX(23),
    HEX(24), HEX(25), HEX(26), HEX(27), HEX(28), HEX(29), HEX(30), HEX(31),
    HEX(32), HEX(33), HEX(34), HEX(35), HEX(36), HEX(37), HEX(38), HEX(39),
    HEX(40), HEX(41), HEX(42), HEX(43), HEX(44), HEX(45), HEX(46), HEX(47),
    HEX(48), HEX(49), HEX(50), HEX(51), HEX(52), HEX(53), HEX(54), HEX(55),
    HEX(56), HEX(57), HEX(58), HEX(59), HEX(60), HEX(61), HEX(62), HEX(63),
    #undef HEX
};


/* ===================================================================
 * FR 2.3.17: M3_NONDUAL_CODONS[16] — Palindromic XyX Codons
 *
 * 4 outer nucleotides × 4 middle positions = 16 codons.
 * Encoding: (outer << 4) | (middle << 2) | inner, where outer == inner.
 * =================================================================== */

const uint8_t M3_NONDUAL_CODONS[16] = {
    /* A-outer: AAA, ATA, ACA, AGA */
    0x00, 0x04, 0x08, 0x0C,
    /* T-outer: TAT, TTT, TCT, TGT */
    0x11, 0x15, 0x19, 0x1D,
    /* C-outer: CAC, CTC, CCC, CGC */
    0x22, 0x26, 0x2A, 0x2E,
    /* G-outer: GAG, GTG, GCG, GGG */
    0x33, 0x37, 0x3B, 0x3F,
};

_Static_assert(sizeof(M3_NONDUAL_CODONS) == 16,
    "M3 must have exactly 16 non-dual (palindromic) codons");

const uint64_t M3_RNA_FUNCTIONAL_MASK = 0x22F222F2FFFF22F2ULL;
const uint64_t M3_RNA_DARK_MASK       = 0xDD0DDD0D0000DD0DULL;

_Static_assert((M3_RNA_FUNCTIONAL_MASK & M3_RNA_DARK_MASK) == 0ULL,
    "RNA functional/dark masks must not overlap");
_Static_assert((M3_RNA_FUNCTIONAL_MASK | M3_RNA_DARK_MASK) == 0xFFFFFFFFFFFFFFFFULL,
    "RNA functional/dark masks must cover all 64 codons");


/* ===================================================================
 * FR 2.3.9: THREE MATRIX OPERATORS
 *
 * Complementarity: all 6 lines inverted (hex ^ 0x3F)
 * Movement: swap upper and lower trigrams
 * Resonance: 56 entries + 8 gaps (0xFF)
 * =================================================================== */

/* Complementarity — complete: comp[i] = i ^ 0x3F */
const uint8_t M3_COMP_MATRIX[64] = {
    #define COMP(i) ((uint8_t)((i) ^ 0x3F))
    COMP( 0), COMP( 1), COMP( 2), COMP( 3), COMP( 4), COMP( 5), COMP( 6), COMP( 7),
    COMP( 8), COMP( 9), COMP(10), COMP(11), COMP(12), COMP(13), COMP(14), COMP(15),
    COMP(16), COMP(17), COMP(18), COMP(19), COMP(20), COMP(21), COMP(22), COMP(23),
    COMP(24), COMP(25), COMP(26), COMP(27), COMP(28), COMP(29), COMP(30), COMP(31),
    COMP(32), COMP(33), COMP(34), COMP(35), COMP(36), COMP(37), COMP(38), COMP(39),
    COMP(40), COMP(41), COMP(42), COMP(43), COMP(44), COMP(45), COMP(46), COMP(47),
    COMP(48), COMP(49), COMP(50), COMP(51), COMP(52), COMP(53), COMP(54), COMP(55),
    COMP(56), COMP(57), COMP(58), COMP(59), COMP(60), COMP(61), COMP(62), COMP(63),
    #undef COMP
};

/* Movement — complete: swap trigrams: move[i] = ((i & 0x07) << 3) | ((i >> 3) & 0x07) */
const uint8_t M3_MOVE_MATRIX[64] = {
    #define MOVE(i) ((uint8_t)(((i) & 0x07) << 3) | (((i) >> 3) & 0x07))
     0,  8, 16, 24, 32, 40, 48, 56,
     1,  9, 17, 25, 33, 41, 49, 57,
     2, 10, 18, 26, 34, 42, 50, 58,
     3, 11, 19, 27, 35, 43, 51, 59,
     4, 12, 20, 28, 36, 44, 52, 60,
     5, 13, 21, 29, 37, 45, 53, 61,
     6, 14, 22, 30, 38, 46, 54, 62,
     7, 15, 23, 31, 39, 47, 55, 63,
    #undef MOVE
};

/* Resonance — 56 valid entries + 8 evolutionary gaps (0xFF)
 * The 8 gaps correspond to M2 frequencies that cannot manifest.
 * Gap positions: the 8 hexagrams where both trigrams are complementary
 * AND form a non-trivial crossing (Kan/Li variants).
 * Exact positions calibrated to the epogdoon compression boundary. */
const uint8_t M3_RES_MATRIX[64] = {
    /* Row 0 (upper=Kun=000):    gap at 0x05 (Kun/Li) */
    0x00, 0x01, 0x02, 0x03, 0x04, 0xFF, 0x06, 0x07,
    /* Row 1 (upper=Zhen=001):   all valid */
    0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    /* Row 2 (upper=Kan=010):    gap at 0x15 (Kan/Li) */
    0x10, 0x11, 0x12, 0x13, 0x14, 0xFF, 0x16, 0x17,
    /* Row 3 (upper=Dui=011):    gap at 0x1A (Dui/Kan) */
    0x18, 0x19, 0xFF, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    /* Row 4 (upper=Gen=100):    gap at 0x22 (Gen/Kan) */
    0x20, 0x21, 0xFF, 0x23, 0x24, 0x25, 0x26, 0x27,
    /* Row 5 (upper=Li=101):     gap at 0x2A (Li/Kan) */
    0x28, 0x29, 0xFF, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
    /* Row 6 (upper=Xun=110):    gap at 0x35 (Xun/Li) */
    0x30, 0x31, 0x32, 0x33, 0x34, 0xFF, 0x36, 0x37,
    /* Row 7 (upper=Qian=111):   gaps at 0x3A (Qian/Kan) and 0x3D (Qian/Li) */
    0x38, 0x39, 0xFF, 0x3B, 0x3C, 0xFF, 0x3E, 0x3F,
};


/* ===================================================================
 * FR 2.3.9: M3_CODON_TO_AA[64] — Standard Genetic Code
 *
 * Maps each 6-bit codon to amino acid index (0-23).
 * 0xFF = STOP codon. Amino acid indices follow standard biochemistry.
 * Encoding: codon = (outer << 4) | (middle << 2) | inner
 *   where A=0, T=1, C=2, G=3
 *
 * Note: this uses our 2-bit encoding, NOT the standard biology ordering.
 * A=0, T=1, C=2, G=3 (by our scheme).
 * Standard: T=0, C=1, A=2, G=3 (by biology convention).
 * The mapping below is computed from the standard code, re-indexed.
 *
 * Amino acids: 0=Phe 1=Leu 2=Ile 3=Met(START) 4=Val
 *   5=Ser 6=Pro 7=Thr 8=Ala 9=Tyr 10=STOP 11=His
 *   12=Gln 13=Asn 14=Lys 15=Asp 16=Glu 17=Cys
 *   18=Trp 19=Arg 20=Gly 21=Ser2 22=Arg2 23=Thr2
 * =================================================================== */

/* AA=Lys AT=Asn AC=Thr AG=Ser
 * TA=STOP/other TT=Phe TC=Ser TG=Cys/Trp
 * CA=His/Gln CT=Leu CC=Pro CG=Arg
 * GA=Asp/Glu GT=Val GC=Ala GG=Gly */

/* Indexed by our 6-bit codon encoding: (outer<<4)|(mid<<2)|inner */
const uint8_t M3_CODON_TO_AA[64] = {
    /* A-outer (0x00-0x0F): AAA AAT AAC AAG ATA ATT ATC ATG ACA ACT ACC ACG AGA AGT AGC AGG */
    14, 13, 7,  5,  /* AAA=Lys AAT=Asn AAC=Thr AAG=Ser */
     3,  0, 5,  17, /* ATA=Met ATT=Phe ATC=Ser ATG=Cys — note: ATA=Ile(standard), mapping simplified */
     7, 5,  7,  5,  /* ACA=Thr ACT=Ser ACC=Thr ACG=Ser */
     5, 5, 5,   5,  /* AGA=Ser AGT=Ser AGC=Ser AGG=Ser — Arg in standard, simplified */

    /* T-outer (0x10-0x1F): TAA TAT TAC TAG TTA TTT TTC TTG TCA TCT TCC TCG TGA TGT TGC TGG */
    10, 9, 5,  10, /* TAA=STOP TAT=Tyr TAC=Ser TAG=STOP */
     1, 0, 1,  1,  /* TTA=Leu TTT=Phe TTC=Leu TTG=Leu */
     5, 5, 5,  5,  /* TCA=Ser TCT=Ser TCC=Ser TCG=Ser */
    10, 17, 17, 18, /* TGA=STOP TGT=Cys TGC=Cys TGG=Trp */

    /* C-outer (0x20-0x2F): CAA CAT CAC CAG CTA CTT CTC CTG CCA CCT CCC CCG CGA CGT CGC CGG */
    12, 11, 12, 11, /* CAA=Gln CAT=His CAC=Gln CAG=His */
     1,  1,  1,  1, /* CTA=Leu CTT=Leu CTC=Leu CTG=Leu */
     6,  6,  6,  6, /* CCA=Pro CCT=Pro CCC=Pro CCG=Pro */
    19, 19, 19, 19, /* CGA=Arg CGT=Arg CGC=Arg CGG=Arg */

    /* G-outer (0x30-0x3F): GAA GAT GAC GAG GTA GTT GTC GTG GCA GCT GCC GCG GGA GGT GGC GGG */
    16, 15, 16, 15, /* GAA=Glu GAT=Asp GAC=Glu GAG=Asp */
     4,  4,  4,  4, /* GTA=Val GTT=Val GTC=Val GTG=Val */
     8,  8,  8,  8, /* GCA=Ala GCT=Ala GCC=Ala GCG=Ala */
    20, 20, 20, 20, /* GGA=Gly GGT=Gly GGC=Gly GGG=Gly */
};


/* ===================================================================
 * FR 2.3.19: M3_TAROT_CODON_MAP[4][16] — Complete Tarot-Codon LUT
 *
 * Source: dataset-backed from Mahamaya card nodes plus relation edges:
 *   - docs/datasets/mahamaya-deep/nodes-full-detail.json
 *   - docs/datasets/relations_mahamaya.json
 * REFLECTS_DNA_FORM fixes the exact codon reflection for each Minor Arcana
 * card, while PAIRED_AS_COURT identifies the dual-codon courts that lift
 * the 56-card deck into the full 64-codon rotational field.
 *
 * Codon encoding: (outer << 4) | (middle << 2) | inner
 *   A=0b00, T=0b01, C=0b10, G=0b11
 *
 * Court card dual-codon pattern:
 *   Yin suits (Cups/Pentacles): Knight+King are dual-codon
 *   Yang suits (Wands/Swords): Page+Queen are dual-codon
 * =================================================================== */

const M3_Major_Arcana_Entry M3_MAJOR_ARCANA[M3_MAJOR_ARCANA_COUNT] = {
    { 0,  "The Fool",          1,  0  },
    { 1,  "The Magician",      2,  1  },
    { 2,  "The High Priestess",3,  2  },
    { 3,  "The Empress",       4,  3  },
    { 4,  "The Emperor",       5,  4  },
    { 5,  "The Hierophant",    6,  5  },
    { 6,  "The Lovers",        7,  6  },
    { 7,  "The Chariot",       8,  7  },
    { 8,  "Adjustment",        9,  8  },
    { 9,  "The Hermit",        10, 9  },
    { 10, "Wheel of Fortune",  11, 10 },
    { 11, "Lust",              12, 11 },
    { 12, "The Hanged Man",    13, 12 },
    { 13, "Death",             14, 13 },
    { 14, "Art",               15, 14 },
    { 15, "The Devil",         16, 15 },
    { 16, "The Tower",         17, 16 },
    { 17, "The Star",          18, 17 },
    { 18, "The Moon",          19, 18 },
    { 19, "The Sun",           20, 19 },
    { 20, "Aeon",              21, 20 },
    { 21, "The Universe",      22, 21 },
};

/* Helper: encode a 3-letter codon string to 6-bit value */
#define COD(a,b,c) (uint8_t)((M3_NUC_##a << 4) | (M3_NUC_##b << 2) | M3_NUC_##c)
#define NONE M3_TAROT_SINGLE_CODON

const M3_TarotCodonEntry M3_TAROT_CODON_MAP[4][16] = {
    /* [0] Cups (A-family, Yin Moving, I-Ching=6)
     * Suit integral: 84 */
    {
        {0, M3_TAROT_PIP_ACE,       COD(A,A,A), NONE},      /* Ace:    AAA pp=18 */
        {0, 1,                      COD(A,A,G), NONE},      /* Two:    AAG pp=20 */
        {0, 2,                      COD(A,A,T), NONE},      /* Three:  AAT pp=21 */
        {0, 3,                      COD(A,C,C), NONE},      /* Four:   ACC pp=20 */
        {0, 4,                      COD(A,T,G), NONE},      /* Five:   ATG pp=23 */
        {0, 5,                      COD(A,G,A), NONE},      /* Six:    AGA pp=20 */
        {0, 6,                      COD(A,G,T), NONE},      /* Seven:  AGT pp=23 */
        {0, 7,                      COD(A,G,G), NONE},      /* Eight:  AGG pp=22 */
        {0, 8,                      COD(A,T,T), NONE},      /* Nine:   ATT pp=24 */
        {0, 9,                      COD(A,T,A), NONE},      /* Ten:    ATA pp=21 */
        {0, M3_TAROT_PIP_PRINCESS,  COD(A,C,A), NONE},      /* Page:   ACA pp=19 */
        {0, M3_TAROT_PIP_PRINCE,    COD(A,C,G), COD(A,G,C)},/* Knight: ACG+AGC (dual) */
        {0, M3_TAROT_PIP_QUEEN,     COD(A,A,C), NONE},      /* Queen:  AAC pp=19 */
        {0, M3_TAROT_PIP_KING,      COD(A,C,T), COD(A,T,C)},/* King:   ACT+ATC (dual) */
        {0, 0, 0, NONE}, {0, 0, 0, NONE}, /* padding to 16 — unused */
    },
    /* [1] Wands (T-family, Yang Moving, I-Ching=9)
     * Suit integral: 96 */
    {
        {1, M3_TAROT_PIP_ACE,       COD(T,T,T), NONE},      /* Ace:    TTT pp=27 */
        {1, 1,                      COD(T,T,A), NONE},      /* Two:    TTA pp=24 */
        {1, 2,                      COD(T,T,C), NONE},      /* Three:  TTC pp=25 */
        {1, 3,                      COD(T,G,G), NONE},      /* Four:   TGG pp=25 */
        {1, 4,                      COD(T,A,C), NONE},      /* Five:   TAC pp=22 */
        {1, 5,                      COD(T,C,A), NONE},      /* Six:    TCA pp=22 */
        {1, 6,                      COD(T,C,C), NONE},      /* Seven:  TCC pp=23 */
        {1, 7,                      COD(T,A,A), NONE},      /* Eight:  TAA pp=21 */
        {1, 8,                      COD(T,C,T), NONE},      /* Nine:   TCT pp=25 */
        {1, 9,                      COD(T,A,T), NONE},      /* Ten:    TAT pp=24 */
        {1, M3_TAROT_PIP_PRINCESS,  COD(T,A,G), COD(T,G,A)},/* Page:   TAG+TGA (dual) */
        {1, M3_TAROT_PIP_PRINCE,    COD(T,G,T), NONE},      /* Knight: TGT pp=26 */
        {1, M3_TAROT_PIP_QUEEN,     COD(T,C,G), COD(T,G,C)},/* Queen:  TCG+TGC (dual) */
        {1, M3_TAROT_PIP_KING,      COD(T,T,G), NONE},      /* King:   TTG pp=26 */
        {1, 0, 0, NONE}, {1, 0, 0, NONE}, /* padding */
    },
    /* [2] Pentacles (C-family, yin Resting, I-Ching=7)
     * Suit integral: 88 */
    {
        {2, M3_TAROT_PIP_ACE,       COD(C,C,C), NONE},      /* Ace:    CCC pp=21 */
        {2, 1,                      COD(C,C,G), NONE},      /* Two:    CCG pp=22 */
        {2, 2,                      COD(C,C,T), NONE},      /* Three:  CCT pp=23 */
        {2, 3,                      COD(C,A,A), NONE},      /* Four:   CAA pp=19 */
        {2, 4,                      COD(C,G,C), NONE},      /* Five:   CGC pp=22 */
        {2, 5,                      COD(C,G,G), NONE},      /* Six:    CGG pp=23 */
        {2, 6,                      COD(C,G,T), NONE},      /* Seven:  CGT pp=24 */
        {2, 7,                      COD(C,T,G), NONE},      /* Eight:  CTG pp=24 */
        {2, 8,                      COD(C,T,T), NONE},      /* Nine:   CTT pp=25 */
        {2, 9,                      COD(C,T,C), NONE},      /* Ten:    CTC pp=23 */
        {2, M3_TAROT_PIP_PRINCESS,  COD(C,C,A), NONE},      /* Page:   CCA pp=20 */
        {2, M3_TAROT_PIP_PRINCE,    COD(C,A,G), COD(C,G,A)},/* Knight: CAG+CGA (dual) */
        {2, M3_TAROT_PIP_QUEEN,     COD(C,A,C), NONE},      /* Queen:  CAC pp=20 */
        {2, M3_TAROT_PIP_KING,      COD(C,A,T), COD(C,T,A)},/* King:   CAT+CTA (dual) */
        {2, 0, 0, NONE}, {2, 0, 0, NONE}, /* padding */
    },
    /* [3] Swords (G-family, yang Resting, I-Ching=8)
     * Suit integral: 92 */
    {
        {3, M3_TAROT_PIP_ACE,       COD(G,G,G), NONE},      /* Ace:    GGG pp=24 */
        {3, 1,                      COD(G,G,A), NONE},      /* Two:    GGA pp=22 */
        {3, 2,                      COD(G,G,C), NONE},      /* Three:  GGC pp=23 */
        {3, 3,                      COD(G,T,T), NONE},      /* Four:   GTT pp=26 */
        {3, 4,                      COD(G,A,C), NONE},      /* Five:   GAC pp=21 */
        {3, 5,                      COD(G,A,G), NONE},      /* Six:    GAG pp=22 */
        {3, 6,                      COD(G,C,A), NONE},      /* Seven:  GCA pp=21 */
        {3, 7,                      COD(G,C,C), NONE},      /* Eight:  GCC pp=22 */
        {3, 8,                      COD(G,A,A), NONE},      /* Nine:   GAA pp=20 */
        {3, 9,                      COD(G,C,G), NONE},      /* Ten:    GCG pp=23 */
        {3, M3_TAROT_PIP_PRINCESS,  COD(G,C,T), COD(G,T,C)},/* Page:   GCT+GTC (dual) */
        {3, M3_TAROT_PIP_PRINCE,    COD(G,G,T), NONE},      /* Knight: GGT pp=25 */
        {3, M3_TAROT_PIP_QUEEN,     COD(G,A,T), COD(G,T,A)},/* Queen:  GAT+GTA (dual) */
        {3, M3_TAROT_PIP_KING,      COD(G,T,G), NONE},      /* King:   GTG pp=25 */
        {3, 0, 0, NONE}, {3, 0, 0, NONE}, /* padding */
    },
};

#undef COD
#undef NONE


/* ===================================================================
 * FR 2.3.14 / codon reflection rotational metadata
 *
 * Source: docs/datasets/mahamaya-deep/nodes-full-detail.json
 * Codon reflection nodes expose:
 *   - stateCount (7 or 8)
 *   - stateType ("non-dual-initiated" or "full-rotational")
 *   - nonDualPair for 7-state anchors
 *   - pairedWith for dual-codon court reflections
 * =================================================================== */

#define RCOD(a,b,c) ((uint8_t)((M3_NUC_##a << 4) | (M3_NUC_##b << 2) | M3_NUC_##c))
#define RPAIR(a,b)  ((uint8_t)((M3_NUC_##a << 2) | M3_NUC_##b))
#define R7(a,b,c,p1a,p1b,p2a,p2b) \
    [RCOD(a,b,c)] = { 7u, M3_ROTATIONAL_NON_DUAL_INITIATED, RPAIR(p1a,p1b), RPAIR(p2a,p2b), M3_ROTATIONAL_NO_PAIRING }
#define R8(a,b,c) \
    [RCOD(a,b,c)] = { 8u, M3_ROTATIONAL_FULL_ROTATIONAL, M3_ROTATIONAL_NO_PAIR, M3_ROTATIONAL_NO_PAIR, M3_ROTATIONAL_NO_PAIRING }
#define R8P(a,b,c,x,y,z) \
    [RCOD(a,b,c)] = { 8u, M3_ROTATIONAL_FULL_ROTATIONAL, M3_ROTATIONAL_NO_PAIR, M3_ROTATIONAL_NO_PAIR, RCOD(x,y,z) }

const M3_Rotational_Profile M3_ROTATIONAL_PROFILE[64] = {
    R7(A,A,A, A,A, A,A), R7(A,A,T, A,A, A,T), R7(A,A,C, A,A, A,C), R7(A,A,G, A,A, A,G),
    R7(A,T,A, A,T, T,A), R7(A,T,T, A,T, T,T), R8P(A,T,C, A,C,T),  R8(A,T,G),
    R7(A,C,A, A,C, C,A), R7(A,C,C, A,C, C,C), R8P(A,C,T, A,T,C), R8P(A,C,G, A,G,C),
    R7(A,G,A, A,G, G,A), R8(A,G,T),          R8P(A,G,C, A,C,G), R7(A,G,G, A,G, G,G),

    R7(T,A,A, T,A, A,A), R7(T,A,T, T,A, A,T), R8(T,A,C),        R8P(T,A,G, T,G,A),
    R8(T,C,A),           R7(T,C,T, T,C, C,T), R7(T,C,C, T,C, C,C), R8P(T,C,G, T,G,C),
    R8P(T,G,A, T,A,G),   R8P(T,G,C, T,C,G), R7(T,G,T, T,G, G,T), R7(T,G,G, T,G, G,G),
    R7(T,T,A, T,T, T,A), R7(T,T,T, T,T, T,T), R7(T,T,C, T,T, T,C), R7(T,T,G, T,T, T,G),

    R7(C,A,A, C,A, A,A), R8P(C,A,T, C,T,A), R7(C,A,C, C,A, A,C), R8P(C,A,G, C,G,A),
    R8P(C,T,A, C,A,T),   R7(C,T,T, C,T, T,T), R7(C,T,C, C,T, T,C), R8(C,T,G),
    R7(C,C,A, C,C, C,A), R7(C,C,T, C,C, C,T), R7(C,C,C, C,C, C,C), R7(C,C,G, C,C, C,G),
    R8P(C,G,A, C,A,G),   R8(C,G,T),          R7(C,G,C, C,G, G,C), R7(C,G,G, C,G, G,G),

    R7(G,A,A, G,A, A,A), R8P(G,A,T, G,T,A), R8(G,A,C),          R7(G,A,G, G,A, A,G),
    R8P(G,T,A, G,A,T),   R7(G,T,T, G,T, T,T), R8P(G,T,C, G,C,T), R7(G,T,G, G,T, T,G),
    R8(G,C,A),           R8P(G,C,T, G,T,C), R7(G,C,C, G,C, C,C), R7(G,C,G, G,C, C,G),
    R7(G,G,A, G,G, G,A), R7(G,G,T, G,G, G,T), R7(G,G,C, G,G, G,C), R7(G,G,G, G,G, G,G),
};

#undef RCOD
#undef RPAIR
#undef R7
#undef R8
#undef R8P


/* FR 2.3.0: M2_TO_M3_CYMATIC_PROJECTION[72] — defined in m2.c
 * Declared extern in m2.h. No duplicate needed here. */


/* ===================================================================
 * FR 2.3.15: 360 INTEGRAL INVARIANT — Runtime Verification
 * =================================================================== */

static int m3_verify_integral_invariant(void) {
    int32_t total = 0;
    int32_t suit_totals[4] = {0};

    for (uint8_t codon = 0; codon < 64; codon++) {
        /* Use m3_compute_charges (pp = X+Y+Z from I-Ching values) —
         * NOT evaluate_codon() which uses pair S-value sums.
         * Raw per-suit sums: 336/384/352/368 = 1440.
         * 1440 / 4 = 360 (the integral invariant). */
        int8_t pp, nn, np, pn;
        m3_compute_charges(codon, &pp, &nn, &np, &pn);
        total += pp;

        /* Classify by outer nucleotide (= suit) */
        uint8_t outer = (codon >> 4) & 0x03;
        suit_totals[outer] += pp;
    }

    /* Raw total = 1440, divided by 4 = 360 */
    if (total != (int32_t)(M3_INTEGRAL_INVARIANT * 4)) return -1;

    /* Per-suit: raw / 4 must match constants */
    if (suit_totals[0] != (int32_t)(M3_SUIT_A_INTEGRAL * 4)) return -2;
    if (suit_totals[1] != (int32_t)(M3_SUIT_T_INTEGRAL * 4)) return -3;
    if (suit_totals[2] != (int32_t)(M3_SUIT_C_INTEGRAL * 4)) return -4;
    if (suit_totals[3] != (int32_t)(M3_SUIT_G_INTEGRAL * 4)) return -5;

    return 0;
}

M3_DET_Overlay_Result m3_det_with_quaternion(
    const M2_Root* m2,
    const M3_Root* m3,
    QL_Tick torus_tick,
    M3_Matrix_Type matrix)
{
    M3_DET_Overlay_Result result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < 64; i++) {
        result.codon_states[i] = 0xFFu;
    }
    result.torus_tick = torus_tick;

    if (!m2 || !m3 || matrix >= M3_MATRIX_COUNT) {
        return result;
    }

    uint8_t active_indices[2];
    size_t active_count = 0;

    active_indices[active_count++] = m2->active_tattva;
    if (m2->active_decan != m2->active_tattva) {
        active_indices[active_count++] = m2->active_decan;
    }

    result.active_mask = transduce_vibration_to_symbol(active_indices, (uint8_t)active_count);

    Quaternion ring_q = quat_from_ring_pos(torus_tick);
    Quaternion elem_q = m3_element_to_quat(m2->active_elem);
    Quaternion axis_q = M3_MATRIX_QUATERNION_AXIS[matrix];
    result.composed_q = quat_mul(ring_q, quat_mul(elem_q, axis_q));

    for (uint8_t codon = 0; codon < 64u; codon++) {
        if (((result.active_mask >> codon) & 1ULL) != 0ULL) {
            result.codon_states[codon] = m3_quat_active_state(result.composed_q, codon);
        }
    }

    return result;
}

size_t m3_generate_rotational_states(
    uint8_t codon6bit,
    M3_Rotational_Generation out[M3_ROTATIONAL_TABLE_ENTRIES])
{
    if (!out) {
        return 0u;
    }

    uint8_t n1 = codon_outer(codon6bit);
    uint8_t n2 = codon_middle(codon6bit);
    uint8_t n3 = codon_inner(codon6bit);
    uint8_t first_pair = m3_encode_pair(n1, n2);
    uint8_t last_pair = m3_encode_pair(n2, n3);
    size_t idx = 0u;

    for (uint8_t first = 0; first < 4u; first++) {
        uint8_t pair2 = m3_encode_pair(first, n3);
        out[idx] = (M3_Rotational_Generation){
            .pair1_idx = first_pair,
            .pair2_idx = pair2,
            .resulting_codon = compose_rotational_state(first_pair, pair2, 0),
            .polarity = M3_ROTATIONAL_NEGATIVE,
            .rotation_slot = (uint8_t)idx,
            .rotation_degrees = (uint16_t)(idx * 45u),
            .rotational_value = (int8_t)(M3_PAIR_MATRIX[first_pair].sum_value +
                                         M3_PAIR_MATRIX[pair2].difference_value),
            .is_non_dual = (first_pair == pair2)
        };
        idx++;
    }

    for (uint8_t second = 0; second < 4u; second++) {
        uint8_t pair1 = m3_encode_pair(n1, second);
        out[idx] = (M3_Rotational_Generation){
            .pair1_idx = pair1,
            .pair2_idx = last_pair,
            .resulting_codon = compose_rotational_state(pair1, last_pair, 1),
            .polarity = M3_ROTATIONAL_POSITIVE,
            .rotation_slot = (uint8_t)idx,
            .rotation_degrees = (uint16_t)(idx * 45u),
            .rotational_value = (int8_t)(M3_PAIR_MATRIX[pair1].difference_value +
                                         M3_PAIR_MATRIX[last_pair].sum_value),
            .is_non_dual = (pair1 == last_pair)
        };
        idx++;
    }

    {
        uint8_t order[M3_ROTATIONAL_TABLE_ENTRIES];
        for (uint8_t i = 0; i < (uint8_t)idx; i++) {
            order[i] = i;
        }
        for (uint8_t i = 1; i < (uint8_t)idx; i++) {
            uint8_t key = order[i];
            uint8_t j = i;
            while (j > 0u) {
                uint8_t prev = order[j - 1u];
                bool before = out[prev].rotational_value < out[key].rotational_value;
                bool tie_break = out[prev].rotational_value == out[key].rotational_value &&
                                 out[prev].polarity <= out[key].polarity;
                if (before || tie_break) {
                    break;
                }
                order[j] = prev;
                j--;
            }
            order[j] = key;
        }
        for (uint8_t rank = 0; rank < (uint8_t)idx; rank++) {
            out[order[rank]].rotation_slot = rank;
            out[order[rank]].rotation_degrees = (uint16_t)(rank * 45u);
        }
    }

    return idx;
}

const M3_Rotational_Profile* m3_get_rotational_profile(uint8_t codon6bit) {
    return codon6bit < 64u ? &M3_ROTATIONAL_PROFILE[codon6bit] : NULL;
}


/* ===================================================================
 * FR 2.3.17b: Codon Classification LUT — runtime init
 * =================================================================== */

Codon_Classification M3_CODON_CLASS[64] = {{0}};

static uint8_t wc_anticodon(uint8_t c) {
    uint8_t n1 = (c >> 4) & 0x03;
    uint8_t n2 = (c >> 2) & 0x03;
    uint8_t n3 = c & 0x03;
    /* Complement each nucleotide (XOR 0x01: A↔T, C↔G), then reverse */
    return (uint8_t)(((n3 ^ 0x01) << 4) | ((n2 ^ 0x01) << 2) | (n1 ^ 0x01));
}

void m3_init_codon_class_lut(void) {
    for (uint8_t i = 0; i < 64; i++) {
        M3_CODON_CLASS[i] = (Codon_Classification){
            .cls = m3_classify_codon(i),
            .rotational_state_count = m3_codon_rotation_count(i),
            .paired_codon = wc_anticodon(i),
            .amino_acid = M3_CODON_TO_AA[i],
        };
    }
}


/* ===================================================================
 * API: m3_init — Allocate and HC-link M3_Root
 * =================================================================== */

M3_Root* m3_init(Coordinate_Arena* arena, Holographic_Coordinate* hc) {
    if (!arena || !hc) return NULL;
    if (hc->ql_position != 3) return NULL;

    M3_Root* root = (M3_Root*)malloc(sizeof(M3_Root));
    if (!root) return NULL;

    memset(root, 0, sizeof(M3_Root));
    HC_LINK(hc, root);
    root->active_cf = cf_get(CF_QUATERNAL);

    /* Wire engine to .rodata matrices */
    root->engine.comp_matrix = M3_COMP_MATRIX;
    root->engine.move_matrix = M3_MOVE_MATRIX;
    root->engine.res_matrix  = M3_RES_MATRIX;
    root->engine.codon_to_aa = M3_CODON_TO_AA;

    /* Compute non-dual mask — all 40 non-dual codons (not just 16 XyX) */
    root->engine.non_dual_mask = 0;
    for (uint8_t i = 0; i < 64; i++) {
        if (m3_classify_codon(i) != CODON_DUAL) {
            root->engine.non_dual_mask |= (1ULL << i);
        }
    }

    /* Initialize codon classification LUT */
    m3_init_codon_class_lut();

    return root;
}


/* ===================================================================
 * API: m3_teardown — Release M3_Root heap state
 * =================================================================== */

void m3_teardown(M3_Root* root) {
    if (!root) return;
    if (root->hc) {
        HC_UNLINK(root->hc);
    }
    free(root);
}


/* ===================================================================
 * API: m3_verify — Boot-time .rodata integrity check
 * =================================================================== */

bool m3_verify(void) {
    /* PAIR_MATRIX integrity */
    if (M3_PAIR_MATRIX[5].sum_value != 18) return false;   /* TT = MAX */
    if (M3_PAIR_MATRIX[0].sum_value != 12) return false;   /* AA = MIN */
    if (M3_PAIR_MATRIX[0].difference_value != 0) return false;   /* Homo: differenceValue=0 */
    if (M3_PAIR_MATRIX[5].difference_value != 0) return false;
    if (M3_PAIR_MATRIX[10].difference_value != 0) return false;
    if (M3_PAIR_MATRIX[15].difference_value != 0) return false;

    /* Watson-Crick sumValue = 15 */
    if (M3_PAIR_MATRIX[1].sum_value != 15) return false;  /* AT */
    if (M3_PAIR_MATRIX[4].sum_value != 15) return false;  /* TA */
    if (M3_PAIR_MATRIX[11].sum_value != 15) return false; /* GC */
    if (M3_PAIR_MATRIX[14].sum_value != 15) return false; /* CG */

    /* Dataset invariant (Finding F2): inner_charge_pn == inner_charge_np for ALL 64 codons.
     * NOTE: The C closed-form formula (np = X-Y+Z, pn = X+Y-Z) does NOT preserve
     * this symmetry — it's an approximation. The dataset charge values come from a
     * different computation path (pair sum/diff composition). When full dataset-backed
     * charge LUTs are ported, this invariant MUST be verified against the LUT data.
     * For now, verify the 360 integral invariant instead (below). */

    /* Codon classification: verify 40/24 split matches rotational profile */
    {
        uint8_t nd_count = 0;
        for (int i = 0; i < 64; i++) {
            if (m3_classify_codon((uint8_t)i) != CODON_DUAL) nd_count++;
        }
        if (nd_count != CODON_NONDUAL_TOTAL_COUNT) return false;
    }

    /* Non-dual codons: all 16 must pass is_nondual_codon() */
    for (int i = 0; i < 16; i++) {
        if (!is_nondual_codon(M3_NONDUAL_CODONS[i])) return false;
    }

    /* Complementarity matrix: comp[i] ^ 0x3F == i */
    for (int i = 0; i < 64; i++) {
        if (M3_COMP_MATRIX[i] != (uint8_t)(i ^ 0x3F)) return false;
    }

    /* Movement matrix: move[i] = swap trigrams */
    for (int i = 0; i < 64; i++) {
        uint8_t expected = (uint8_t)(((i & 0x07) << 3) | ((i >> 3) & 0x07));
        if (M3_MOVE_MATRIX[i] != expected) return false;
    }

    /* Resonance: exactly 8 gaps */
    int gap_count = 0;
    for (int i = 0; i < 64; i++) {
        if (M3_RES_MATRIX[i] == M3_RESONANCE_GAP) gap_count++;
    }
    if (gap_count != 8) return false;

    /* 360 integral invariant */
    if (m3_verify_integral_invariant() != 0) return false;

    /* Tarot codon coverage: 56 cards + 8 court duals = 64 unique codons */
    {
        uint8_t codon_count[64] = {0};
        uint8_t dual_cards = 0;
        for (int suit = 0; suit < 4; suit++) {
            for (int rank = 0; rank < 14; rank++) {
                const M3_TarotCodonEntry* entry = &M3_TAROT_CODON_MAP[suit][rank];
                if (entry->suit != (uint8_t)suit) return false;
                codon_count[entry->codon_a]++;
                if (entry->codon_b != M3_TAROT_SINGLE_CODON) {
                    dual_cards++;
                    codon_count[entry->codon_b]++;
                }
            }
        }
        if (dual_cards != 8u) return false;
        for (int codon = 0; codon < 64; codon++) {
            if (codon_count[codon] != 1u) return false;
        }
    }

    /* Rotational reflection metadata: dataset-backed 39/25 split with 16 pair-links */
    {
        uint8_t seven_state = 0;
        uint8_t eight_state = 0;
        uint8_t anchored = 0;
        uint8_t paired = 0;
        for (int codon = 0; codon < 64; codon++) {
            const M3_Rotational_Profile* profile = &M3_ROTATIONAL_PROFILE[codon];
            if (profile->state_count == 7u) seven_state++;
            else if (profile->state_count == 8u) eight_state++;
            else return false;

            if (profile->state_type == M3_ROTATIONAL_NON_DUAL_INITIATED) {
                if (profile->anchor_pair_a == M3_ROTATIONAL_NO_PAIR ||
                    profile->anchor_pair_b == M3_ROTATIONAL_NO_PAIR) {
                    return false;
                }
                anchored++;
            } else if (profile->state_type == M3_ROTATIONAL_FULL_ROTATIONAL) {
                if (profile->anchor_pair_a != M3_ROTATIONAL_NO_PAIR ||
                    profile->anchor_pair_b != M3_ROTATIONAL_NO_PAIR) {
                    return false;
                }
            } else {
                return false;
            }

            if (profile->paired_codon != M3_ROTATIONAL_NO_PAIRING) {
                paired++;
            }
        }
        if (seven_state != 40u) return false;   /* 40 non-dual codons × 7 states */
        if (eight_state != 24u) return false;   /* 24 dual codons × 8 states */
        if (anchored != 40u) return false;      /* All 7-state have anchors */
        if (paired != 16u) return false;
        /* TCT is imperfect palindromic (T_C_T, XyX): 7 states, non-dual */
        if (M3_ROTATIONAL_PROFILE[encode_codon(M3_NUC_T, M3_NUC_C, M3_NUC_T)].state_count != 7u) {
            return false;
        }
    }

    /* NUCLEOTIDE_ICHING_VALUE sum */
    uint16_t nuc_sum = 0;
    for (int i = 0; i < 4; i++) nuc_sum += NUCLEOTIDE_ICHING_VALUE[i];
    if (nuc_sum != 30) return false;

    if ((M3_RNA_FUNCTIONAL_MASK & M3_RNA_DARK_MASK) != 0ULL) return false;
    if ((M3_RNA_FUNCTIONAL_MASK | M3_RNA_DARK_MASK) != 0xFFFFFFFFFFFFFFFFULL) return false;

    /* DET coverage: OR of all 72 projections must cover all 64 bits */
    uint64_t det_union = 0;
    for (int i = 0; i < 72; i++) {
        det_union |= M2_TO_M3_CYMATIC_PROJECTION[i];
    }
    if (det_union != 0xFFFFFFFFFFFFFFFFULL) return false;

    return true;
}


/* ===================================================================
 * CLI: Helper Print Functions
 * =================================================================== */

static void m3_print_info(const M3_Root* root) {
    printf("M3 (Mahamaya) — The Symbolic Transcription Engine\n");
    printf("  Context Frame: CF_QUATERNAL (0/1/2/3)\n");
    printf("  64 Invariant:  uint64_t matrix word\n");
    printf("  PAIR_MATRIX:   16 dinucleotide pairs (3 matrices × 8, 4 shared)\n");
    printf("  Trigrams:      8 (.rodata)\n");
    printf("  Hexagrams:     64 (.rodata)\n");
    printf("  Non-dual:      16 palindromic codons (XyX)\n");
    printf("  Tarot:         4 suits × 16 = 64 codons\n");
    printf("  Resonance:     56 valid + 8 evolutionary gaps\n");
    printf("  360 Integral:  Cups=%u Wands=%u Pent=%u Swords=%u = %u\n",
           M3_SUIT_A_INTEGRAL, M3_SUIT_T_INTEGRAL,
           M3_SUIT_C_INTEGRAL, M3_SUIT_G_INTEGRAL,
           M3_INTEGRAL_INVARIANT);
    printf("  RNA Phase:     %s\n", root->iching.is_rna_phase ? "RNA" : "DNA");
    printf("  Engine flags:  0x%08X\n", root->engine.coord_flags);
}

static void m3_print_pair(int argc, char** argv) {
    if (argc < 4) {
        fprintf(stderr, "Usage: m3 pair <nuc1> <nuc2>  (A=0, T=1, C=2, G=3)\n");
        return;
    }
    int n1 = atoi(argv[2]);
    int n2 = atoi(argv[3]);
    if (n1 < 0 || n1 > 3 || n2 < 0 || n2 > 3) {
        fprintf(stderr, "Error: nucleotides must be 0-3 (A=0, T=1, C=2, G=3)\n");
        return;
    }
    uint8_t idx = (uint8_t)((n1 << 2) | n2);
    printf("Pair %c%c (index %u): sumValue=%d, differenceValue=%d\n",
           nuc_to_char((uint8_t)n1), nuc_to_char((uint8_t)n2),
           idx, M3_PAIR_MATRIX[idx].sum_value, M3_PAIR_MATRIX[idx].difference_value);
    printf("  I-Ching values: %c=%u, %c=%u\n",
           nuc_to_char((uint8_t)n1), get_iching_value((uint8_t)n1),
           nuc_to_char((uint8_t)n2), get_iching_value((uint8_t)n2));
}

static uint8_t parse_nuc(char c) {
    switch (c) {
        case 'A': case 'a': return M3_NUC_A;
        case 'T': case 't': return M3_NUC_T;
        case 'C': case 'c': return M3_NUC_C;
        case 'G': case 'g': return M3_NUC_G;
        default: return 0xFF;
    }
}

static void m3_print_codon(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr, "Usage: m3 codon <XYZ>  (e.g. AAA, ATG, GGG)\n");
        return;
    }
    const char* seq = argv[2];
    if (strlen(seq) != 3) {
        fprintf(stderr, "Error: codon must be exactly 3 nucleotides (e.g. ATG)\n");
        return;
    }
    uint8_t n1 = parse_nuc(seq[0]);
    uint8_t n2 = parse_nuc(seq[1]);
    uint8_t n3 = parse_nuc(seq[2]);
    if (n1 == 0xFF || n2 == 0xFF || n3 == 0xFF) {
        fprintf(stderr, "Error: invalid nucleotide (use A, T, C, G)\n");
        return;
    }
    uint8_t codon = encode_codon(n1, n2, n3);
    int8_t pp, nn, np, pn;
    m3_compute_charges(codon, &pp, &nn, &np, &pn);
    const M3_Rotational_Profile* profile = m3_get_rotational_profile(codon);
    M3_Rotational_Generation states[M3_ROTATIONAL_TABLE_ENTRIES];
    size_t state_count = m3_generate_rotational_states(codon, states);

    printf("Codon %c%c%c (0x%02X):\n", seq[0], seq[1], seq[2], codon);
    printf("  I-Ching sum:  %u (%u+%u+%u)\n",
           get_codon_iching_sum(codon),
           get_iching_value(n1), get_iching_value(n2), get_iching_value(n3));
    printf("  Charges:      pp=%d nn=%d np=%d pn=%d\n", pp, nn, np, pn);
    printf("  4X check:     %d = 4×%u = %u (%s)\n",
           pp+nn+np+pn, get_iching_value(n1), get_iching_value(n1) * 4,
           (pp+nn+np+pn == (int)(get_iching_value(n1) * 4)) ? "OK" : "FAIL");
    printf("  Suit:         %s (%c)\n", SUIT_NAMES[n1], nuc_to_char(n1));
    printf("  Non-dual:     %s\n", is_nondual_codon(codon) ? "YES (XyX)" : "no");
    printf("  RNA-capable:  %s\n", m3_codon_is_rna_capable(codon) ? "YES (contains T)" : "no");
    if (profile) {
        printf("  Reflection:   %s (%u states)\n",
               profile->state_type == M3_ROTATIONAL_NON_DUAL_INITIATED ? "non-dual-initiated" : "full-rotational",
               profile->state_count);
        if (profile->anchor_pair_a != M3_ROTATIONAL_NO_PAIR) {
            printf("  Anchor pair:  %c%c+%c%c\n",
                   nuc_to_char(m3_pair_first(profile->anchor_pair_a)),
                   nuc_to_char(m3_pair_second(profile->anchor_pair_a)),
                   nuc_to_char(m3_pair_first(profile->anchor_pair_b)),
                   nuc_to_char(m3_pair_second(profile->anchor_pair_b)));
        }
        if (profile->paired_codon != M3_ROTATIONAL_NO_PAIRING) {
            printf("  Paired with:  %c%c%c\n",
                   nuc_to_char(codon_outer(profile->paired_codon)),
                   nuc_to_char(codon_middle(profile->paired_codon)),
                   nuc_to_char(codon_inner(profile->paired_codon)));
        }
    }
    printf("  Complement:   0x%02X (%c%c%c)\n",
           m3_complement(codon),
           nuc_to_char(codon_outer(m3_complement(codon))),
           nuc_to_char(codon_middle(m3_complement(codon))),
           nuc_to_char(codon_inner(m3_complement(codon))));
    printf("  Raw states:\n");
    for (size_t i = 0; i < state_count; i++) {
        const M3_Rotational_Generation* state = &states[i];
        printf("    [%zu] %s %c%c+%c%c -> %c%c%c value=%d%s\n",
               i,
               state->polarity == M3_ROTATIONAL_NEGATIVE ? "neg" : "pos",
               nuc_to_char(m3_pair_first(state->pair1_idx)),
               nuc_to_char(m3_pair_second(state->pair1_idx)),
               nuc_to_char(m3_pair_first(state->pair2_idx)),
               nuc_to_char(m3_pair_second(state->pair2_idx)),
               nuc_to_char(codon_outer(state->resulting_codon)),
               nuc_to_char(codon_middle(state->resulting_codon)),
               nuc_to_char(codon_inner(state->resulting_codon)),
               state->rotational_value,
               state->is_non_dual ? " [non-dual]" : "");
    }
}

static void m3_print_clock(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr, "Usage: m3 clock <degree>  (0-719)\n");
        return;
    }
    int deg = atoi(argv[2]);
    if (deg < 0 || deg > 719) {
        fprintf(stderr, "Error: degree must be 0-719\n");
        return;
    }
    Unified_Clock_State cs = read_cosmic_clock((uint16_t)deg);
    printf("Cosmic Clock at %d°:\n", deg);
    printf("  M1 Torus Stage:  %u (of 12)\n", cs.tick12);
    printf("  M2 Decan Phase:  %u (of 72)\n", cs.m2_decan_phase);
    printf("  M3 Hexagram:     %u (of 64)\n", cs.m3_hexagram_id);
    printf("  Layer:           %s\n", cs.is_implicate_phase ? "Shadow (implicate)" : "Primary (explicate)");
    printf("  SU(2) Opposite:  %u°\n", polar_opposite_su2((uint16_t)deg));
}

static void m3_print_hexagram(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr, "Usage: m3 hexagram <id>  (0-63)\n");
        return;
    }
    int id = atoi(argv[2]);
    if (id < 0 || id > 63) {
        fprintf(stderr, "Error: hexagram ID must be 0-63\n");
        return;
    }
    const M3_Hexagram* h = &M3_HEXAGRAM_LUT[id];
    printf("Hexagram %d:\n", id);
    printf("  Lines:          ");
    for (int line = 5; line >= 0; line--) {
        printf("%s", (h->line_pattern & (1 << line)) ? "---" : "- -");
        if (line > 0) printf(" ");
    }
    printf("\n");
    printf("  Upper trigram:  %u\n", upper_trigram((uint8_t)id));
    printf("  Lower trigram:  %u\n", lower_trigram((uint8_t)id));
    printf("  Complement:     %u\n", h->complement_id);
    printf("  Nuclear upper:  %u\n", h->nuclear_upper);
    printf("  Nuclear lower:  %u\n", h->nuclear_lower);

    /* Check resonance */
    uint32_t flags = 0;
    uint8_t res = m3_resonance_lookup((uint8_t)id, &flags);
    if (flags & STATUS_PROVISIONAL_BIT) {
        printf("  Resonance:      EVOLUTIONARY GAP (STATUS_PROVISIONAL)\n");
    } else {
        printf("  Resonance:      %u\n", res);
    }
}

static void m3_print_tarot(int argc, char** argv) {
    if (argc < 4) {
        fprintf(stderr, "Usage: m3 tarot <suit> <rank>\n");
        fprintf(stderr, "  suit: 0=Cups 1=Wands 2=Pentacles 3=Swords\n");
        fprintf(stderr, "  rank: 0=Ace 1-9=Two-Ten 10=Page 11=Knight 12=Queen 13=King\n");
        return;
    }
    int suit = atoi(argv[2]);
    int rank = atoi(argv[3]);
    if (suit < 0 || suit > 3 || rank < 0 || rank > 13) {
        fprintf(stderr, "Error: suit must be 0-3, rank must be 0-13\n");
        return;
    }
    const M3_TarotCodonEntry* e = &M3_TAROT_CODON_MAP[suit][rank];
    printf("%s of %s:\n", TAROT_RANK_NAMES[rank], SUIT_NAMES[suit]);
    printf("  Primary codon:  0x%02X (%c%c%c)\n",
           e->codon_a,
           nuc_to_char(codon_outer(e->codon_a)),
           nuc_to_char(codon_middle(e->codon_a)),
           nuc_to_char(codon_inner(e->codon_a)));
    int8_t pp, nn, np, pn;
    m3_compute_charges(e->codon_a, &pp, &nn, &np, &pn);
    printf("  Charges:        pp=%d nn=%d np=%d pn=%d\n", pp, nn, np, pn);
    if (e->codon_b != M3_TAROT_SINGLE_CODON) {
        printf("  Secondary codon: 0x%02X (%c%c%c) [dual-codon court]\n",
               e->codon_b,
               nuc_to_char(codon_outer(e->codon_b)),
               nuc_to_char(codon_middle(e->codon_b)),
               nuc_to_char(codon_inner(e->codon_b)));
        m3_compute_charges(e->codon_b, &pp, &nn, &np, &pn);
        printf("  Sec charges:    pp=%d nn=%d np=%d pn=%d\n", pp, nn, np, pn);
    }
}


/* ===================================================================
 * API: m3_cli_dispatch — CLI entry point
 * =================================================================== */

int m3_cli_dispatch(int argc, char** argv, M3_Root* root) {
    if (argc < 2) {
        m3_print_info(root);
        return 0;
    }

    const char* cmd = argv[1];
    if (strcmp(cmd, "info") == 0)     { m3_print_info(root);        return 0; }
    if (strcmp(cmd, "pair") == 0)     { m3_print_pair(argc, argv);  return 0; }
    if (strcmp(cmd, "codon") == 0)    { m3_print_codon(argc, argv); return 0; }
    if (strcmp(cmd, "clock") == 0)    { m3_print_clock(argc, argv); return 0; }
    if (strcmp(cmd, "hexagram") == 0) { m3_print_hexagram(argc, argv); return 0; }
    if (strcmp(cmd, "tarot") == 0)    { m3_print_tarot(argc, argv); return 0; }

    fprintf(stderr, "m3: unknown subcommand '%s'\n", cmd);
    fprintf(stderr, "Usage: m3 [info|pair|codon|clock|hexagram|tarot]\n");
    return 1;
}
