/**
 * m2.h — Parashakti: The Vibrational Architecture (Subsystem #2)
 *
 * Implements: M2 (#2) = The vibrational processing layer — 72-Invariant.
 * Context frame: (0/1/2) — CF_TRIKA (The Trika)
 * Anchored to: Psychoid_2 in psychoid_numbers.c (Layer 1 .rodata)
 * Builds on:  ontology.h, psychoid_numbers.h, arena.h, m1.h
 * Feeds into: M3 (64-bit bitboard via DET), M4 (L-family link), M5 (integration)
 *
 * The 72 Invariant: every major M2 structure resolves to exactly 72 bytes.
 * The union M2_Vibrational_72_Space enforces this at compile time.
 *
 * FR Coverage: 2.2.0 – 2.2.12 (M2-parashakti-vibrational-architecture.md, Rev 2)
 *
 * ARCHITECTURE RULE: M2_Root has Holographic_Coordinate* hc
 * as its first field. Bind with HC_LINK(). Never access without GET_PTR().
 */

#ifndef M2_H
#define M2_H

#include "ontology.h"
#include "psychoid_numbers.h"
#include "arena.h"
#include "m1.h"
#include <stdbool.h>
#include <stdint.h>


/* ===================================================================
 * FR 2.2.0 (sub): #2-0 — SEED CONSTANTS
 *
 * Golden ratio ground. All M2 vibrational mathematics traces here.
 * =================================================================== */

#define M2_PHI           1.6180339887f
#define M2_PHI_SQUARED   2.6180339887f
#define M2_PHI_INVERSE   0.6180339887f
#define M2_PI            3.1415926536f
#define M2_SQRT5         2.2360679775f

/* The 72-Invariant — THE law of M2 */
#define M2_72_INVARIANT  72u
#define M2_36_BASE       36u


/* ===================================================================
 * FR 2.2.6: ELEMENT THROUGHLINE — The Five-Fold Spine
 *
 * 5 elements propagate through Tattvas, Decans, Mantras, Chakras.
 * =================================================================== */

typedef enum {
    ELEMENT_ID_AKASHA  = 0,  /* Space / Ether / Quintessence */
    ELEMENT_ID_VAYU    = 1,  /* Air                          */
    ELEMENT_ID_AGNI    = 2,  /* Fire                         */
    ELEMENT_ID_APAS    = 3,  /* Water                        */
    ELEMENT_ID_PRITHVI = 4   /* Earth                        */
} Element_Id;

#define M2_ELEMENT_COUNT 5u

typedef enum {
    CHAKRA_EARTH        = 0,  /* Ground potential (#2-5-0/1-0) */
    CHAKRA_MULADHARA    = 1,  /* Prithvi                       */
    CHAKRA_SVADHISTHANA = 2,  /* Apas                          */
    CHAKRA_MANIPURA     = 3,  /* Agni                          */
    CHAKRA_ANAHATA      = 4,  /* Vayu                          */
    CHAKRA_VISHUDDHA    = 5,  /* Akasha                        */
    CHAKRA_AJNA         = 6,  /* Beyond elements               */
    CHAKRA_SAHASRARA    = 7   /* Beyond elements               */
} Chakra_Id;

#define M2_CHAKRA_COUNT 8u

typedef enum {
    ELEM_PHASE_DESCENT = 0,  /* Manifestation downward */
    ELEM_PHASE_ASCENT  = 1,  /* Reabsorption upward   */
    ELEM_PHASE_FUSED   = 2,  /* Non-dual synthesis    */
    ELEM_PHASE_BEYOND  = 3   /* Transcendent          */
} Elemental_Phase;


/* ===================================================================
 * ELEMENTAL SIGNATURE — 8-bit compact encoding
 *
 * bits[2:0] = Element_Id (0-4)
 * bits[5:3] = Chakra_Id  (0-7)
 * bits[7:6] = phase      (0-3)
 * =================================================================== */

typedef uint8_t Elemental_Signature;

#define ELEM_SIG_PACK(elem, chakra, phase) \
    ((Elemental_Signature)( \
        (((uint8_t)(elem))  & 0x07u)       | \
        ((((uint8_t)(chakra)) & 0x07u) << 3u) | \
        ((((uint8_t)(phase))  & 0x03u) << 6u)   ))

#define ELEM_SIG_GET_ELEMENT(sig)  ((Element_Id)((sig) & 0x07u))
#define ELEM_SIG_GET_CHAKRA(sig)   ((Chakra_Id)(((sig) >> 3u) & 0x07u))
#define ELEM_SIG_GET_PHASE(sig)    ((Elemental_Phase)(((sig) >> 6u) & 0x03u))


/* ===================================================================
 * FR 2.2.0: THE 72-INVARIANT LAW — Union Cell Types
 *
 * All must compile to exactly 1 byte for the union to hold.
 * =================================================================== */

typedef uint8_t MEF_Condition;
typedef uint8_t Tattva_Entry;
typedef uint8_t Decan_Face;
typedef uint8_t Shem_Name;

_Static_assert(sizeof(MEF_Condition) == 1, "MEF_Condition must be 1 byte");
_Static_assert(sizeof(Tattva_Entry)  == 1, "Tattva_Entry must be 1 byte");
_Static_assert(sizeof(Decan_Face)    == 1, "Decan_Face must be 1 byte");
_Static_assert(sizeof(Shem_Name)     == 1, "Shem_Name must be 1 byte");

/* The Canonical 72-Space Union */
typedef union {
    MEF_Condition mef_lenses[12][6];     /* 12 Lenses x 6 Positions = 72 */
    Tattva_Entry  tattvas[36][2];        /* 36 Principles x 2 Phases = 72 */
    Decan_Face    decans[4][3][3][2];    /* 4 Elem x 3 Signs x 3 Decans x 2 Faces = 72 */
    Shem_Name     shem_names[8][9];      /* 8 Choirs x 9 Names = 72 */
    uint8_t       raw_vibration[72];     /* Flat canonical access */
} M2_Vibrational_72_Space;

_Static_assert(sizeof(M2_Vibrational_72_Space) == 72,
    "M2 72-Space Violation: union must be exactly 72 bytes");


/* ===================================================================
 * FR 2.2.12: MEF TOPOLOGICAL CONSTANTS
 * =================================================================== */

#define M2_MEF_TOPOLOGICAL_GENUS          6
#define M2_MEF_EULER_CHARACTERISTIC     (-10)
#define M2_MEF_DUAL_COVERING_FACTOR       2
#define M2_MEF_BASE_LENSES                6u
#define M2_MEF_TOTAL_LENSES              12u
#define M2_MEF_POSITIONS_PER_LENS         6u
#define M2_MEF_TOTAL_CONDITIONS          72u

_Static_assert(M2_MEF_TOTAL_LENSES * M2_MEF_POSITIONS_PER_LENS == M2_MEF_TOTAL_CONDITIONS,
    "MEF topological invariant: 12 x 6 must equal 72");
_Static_assert(M2_MEF_TOPOLOGICAL_GENUS == QL_PROCESSUAL,
    "MEF topological genus must equal M1 QL_PROCESSUAL (both = 6)");


/* ===================================================================
 * FR 2.2.1: MEF DESCRIPTOR — Semantic payload table
 *
 * Canonical 12-Lens Identity Map (from QL Bimba Seed Mapping canvas):
 *   Row 0:  Quaternal          (QL Psychoid Ground)        → L0
 *   Row 1:  Causal             (Aristotle, Iccha Shakti)   → L1
 *   Row 2:  Logical            (Tetralemmic/Catuskoti)     → L2
 *   Row 3:  Processual         (Concrescence, Rhea)        → L3
 *   Row 4:  Phenomenological   (Experience, Soteriology)   → L4
 *   Row 5:  Para Vak           (Vak Ontology, Speech/Code) → L5
 *   Row 6:  Archetypal-Numerical (Jung-Pauli Psychoid)     → L0'
 *   Row 7:  Phenomenal         (Jungian Psychic Functions)  → L1'
 *   Row 8:  Alchemical-Elemental (Transcendent Function)   → L2'
 *   Row 9:  Chronological      (Hegel, Dialectic, Aion)    → L3'
 *   Row 10: Scientific         (Knowledge Work, Research)   → L4'
 *   Row 11: Divine Logos       (Divine Personage, Epi-Logos)→ L5'
 * =================================================================== */

typedef struct {
    uint8_t  lens;           /* 0-11 (0-5 = base, 6-11 = inverted) */
    uint8_t  position;       /* 0-5 (sub-position) */
    uint8_t  is_inverted;    /* 0 = base, 1 = inverted (#-applied) */
    uint8_t  l_family_link;  /* L-coordinate index 0-5 (lens % 6) */
    uint16_t meaning_id;     /* Semantic payload index */
} MEF_Condition_Desc;

/* The master 72-byte union instance — .rodata */
extern const M2_Vibrational_72_Space M2_ARCHETYPES;

/* MEF semantic descriptor table — .rodata */
extern const MEF_Condition_Desc M2_MEF_DESC[72];

/* MEF lens name strings — .rodata */
extern const char* const M2_MEF_LENS_NAMES[12];

/* O(1) routing into the 12x6 matrix */
static inline MEF_Condition get_mef_condition(
        uint8_t lens_0_to_5,
        uint8_t pos_0_to_5,
        bool is_inverted)
{
    uint8_t actual_lens = is_inverted
        ? (uint8_t)(lens_0_to_5 + 6u)
        : lens_0_to_5;
    return M2_ARCHETYPES.mef_lenses[actual_lens][pos_0_to_5];
}

/* L-family structural linkage */
#define MEF_TO_L_FAMILY(lens_0_to_5)               (lens_0_to_5)
#define MEF_INVERTED_TO_L_FAMILY(lens_6_to_11)     ((lens_6_to_11) - 6u)

/* L_Family_State — mutable Heap payload for L-coordinate instances */
typedef struct {
    MEF_Condition active_mef_bimba;    /* 1-byte MEF condition from .rodata */
    float         lens_tension;        /* semantic density / lens strain    */
    uint32_t      semantic_vector_id;  /* Neo4j node ID for this lens      */
} L_Family_State;


/* ===================================================================
 * FR 2.2.2: TATTVA TYPES — 36 Principles x 2 Phases
 * =================================================================== */

typedef enum {
    TATTVA_SHUDDHA        = 0,  /* Pure (indices 0-4)          */
    TATTVA_SHUDDHASHUDDHA = 1,  /* Pure-Impure (indices 5-11)  */
    TATTVA_ASHUDDHA       = 2   /* Impure (indices 12-35)      */
} Tattva_Division;

#define TATTVA_PURE_START     0
#define TATTVA_PURE_END       4
#define TATTVA_MIXED_START    5
#define TATTVA_MIXED_END     11
#define TATTVA_IMPURE_START  12
#define TATTVA_IMPURE_END    35

typedef struct {
    uint8_t  index;          /* 0-35 */
    uint8_t  division;       /* Tattva_Division */
    uint8_t  element_id;     /* Element_Id (0xFF if not a Mahabhuta) */
    uint8_t  kanchuka_mask;  /* For Shuddhashuddha: which kanchukas bind */
    uint16_t meaning_id;
} Tattva_Entry_Desc;

extern const Tattva_Entry_Desc M2_TATTVA_DESC[36];


/* ===================================================================
 * FR 2.2.3: DECAN TYPES — [4][3][3][2] = 72
 * =================================================================== */

typedef struct {
    uint8_t  element;        /* Element_Id (0-4) */
    uint8_t  sign;           /* Zodiacal sign (0-2 within element) */
    uint8_t  decan;          /* Decan within sign (0-2) */
    uint8_t  face;           /* 0 = light, 1 = shadow */
    uint8_t  ruling_planet;  /* Planet index from #2-5 */
    uint8_t  _pad;
    uint16_t meaning_id;
} Decan_Face_Desc;

#define DECAN_FLAT_IDX(elem, sign, decan, face) \
    ((elem)*18u + (sign)*6u + (decan)*2u + (face))

extern const Decan_Face_Desc M2_DECAN_DESC[72];

/* Quintessence sentinel — outside the 4-element array */
extern const Decan_Face_Desc M2_QUINTESSENCE_DECAN;


/* ===================================================================
 * FR 2.2.5: PLANETARY HARMONICS — The Epogdoon Bridge
 *
 * Canonical mod-10 planet ordering — 2026-03-16
 * Source: 00-canonical-invariants.md §2
 * DO NOT reorder — all array indexing depends on this.
 * Earth is NOT in this enum — see EarthBodyState.
 * 9:8 Epogdoon = 9 non-Sun planets (Moon–Pluto) : 8 bodily sites.
 * =================================================================== */

typedef enum {
    PLANET_SUN     = 0,  /* Stable root/parent — not chakra-mapped */
    PLANET_MOON    = 1,
    PLANET_MERCURY = 2,
    PLANET_VENUS   = 3,
    PLANET_MARS    = 4,
    PLANET_JUPITER = 5,
    PLANET_SATURN  = 6,
    PLANET_URANUS  = 7,  /* Transpersonal */
    PLANET_NEPTUNE = 8,  /* Transpersonal */
    PLANET_PLUTO   = 9,  /* Transpersonal */
    PLANET_COUNT   = 10
} Planet_Id;

#define M2_PLANET_COUNT 10u

/* Personal planets: Sun(0) through Saturn(6). Transpersonal: Uranus(7)+. */
#define PLANET_IS_PERSONAL(id)      ((id) <= PLANET_SATURN)
#define PLANET_IS_TRANSPERSONAL(id) ((id) >= PLANET_URANUS && (id) <= PLANET_PLUTO)

/* Uranus/Neptune/Pluto: analytically preempted — valid for hot-path use */
#define MEANING_ID_PREEMPTED  0xFFFEu

static inline bool m2_planet_is_preempted(uint8_t id) {
    return (id == PLANET_URANUS || id == PLANET_NEPTUNE || id == PLANET_PLUTO);
}

typedef struct {
    uint8_t             id;              /* Planet_Id */
    uint8_t             group_type;      /* 0=identity, 1=SU(2), 2=SU(3), 3=U(1), 4=catalytic, 5=transpersonal */
    uint8_t             prime;           /* associated prime (0 if none) */
    Elemental_Signature elem_sig;        /* compact element+chakra+phase */
    uint16_t            cousto_freq;     /* Hz (audible Cousto octave) */
    uint16_t            keplerian_vel;   /* arcsec/day x 10 */
    uint8_t             digital_root;    /* DR of Cousto frequency */
    uint8_t             ananda_row;      /* corresponding Ananda vortex row (0-9) */
    uint16_t            meaning_id;
} Planet_Operator;

extern const Planet_Operator M2_PLANET_LUT[10];

/* EarthBodyState — geocentric center/observer, clock anchor.
 * NOT in PlanetState[10]. The 9:8 epogdoon asymmetry is intentional:
 * 9 orbiting planets (Sun excluded as identity root) : 8 chakras.
 * Earth = the witnessing ground of all planetary motion.
 * Source: 00-canonical-invariants.md §2, §5 */
typedef struct {
    uint8_t  chakra_id;   /* Always CHAKRA_EARTH (= 0) */
    float    activation;  /* 1.0 = fully active as geocentric anchor */
    uint16_t reserved;
} EarthBodyState;

/* CHAKRA_EARTH = 0 is defined in Chakra_Id enum above.
 * It serves as the geocentric ground anchor for EarthBodyState. */

/* Cousto frequency #defines (canonical mod-10 order) */
#define PLANET_FREQ_SUN      126u    /* DR=9 */
#define PLANET_FREQ_MOON     210u    /* DR=3 */
#define PLANET_FREQ_MERCURY  141u    /* DR=6 */
#define PLANET_FREQ_VENUS    221u    /* DR=5 */
#define PLANET_FREQ_MARS     145u    /* DR=1 */
#define PLANET_FREQ_JUPITER  184u    /* DR=4 */
#define PLANET_FREQ_SATURN   148u    /* DR=4 */
#define PLANET_FREQ_URANUS   207u    /* DR=9 — transpersonal awakener */
#define PLANET_FREQ_NEPTUNE  211u    /* DR=4 — Lemniscate archetype */
#define PLANET_FREQ_PLUTO    140u    /* DR=5 — Mobius return */
/* Earth is geocentric observer — not in array; kept as named constant only */
#define PLANET_FREQ_EARTH    136u    /* DR=1 — the Om frequency (EarthBodyState) */

/* Keplerian velocities (arcsec/day x 10, canonical mod-10 order) */
#define PLANET_KEPLERIAN_SUN       35999u
#define PLANET_KEPLERIAN_MOON      47270u  /* ~13.17 deg/day */
#define PLANET_KEPLERIAN_MERCURY   14739u
#define PLANET_KEPLERIAN_VENUS      3600u
#define PLANET_KEPLERIAN_MARS       1886u
#define PLANET_KEPLERIAN_JUPITER     299u
#define PLANET_KEPLERIAN_SATURN      120u
#define PLANET_KEPLERIAN_URANUS       42u  /* preempted */
#define PLANET_KEPLERIAN_NEPTUNE      21u  /* preempted */
#define PLANET_KEPLERIAN_PLUTO        14u  /* preempted */

/* Chakra descriptor */
typedef struct {
    uint8_t  id;            /* Chakra_Id */
    uint8_t  element_id;    /* Element_Id (0xFF for Ajna, Sahasrara) */
    uint8_t  tattva_idx;    /* Corresponding tattva index */
    uint8_t  _pad;
    uint16_t meaning_id;
} Chakra_Descriptor;

extern const Chakra_Descriptor M2_CHAKRA_LUT[8];


/* ===================================================================
 * FR 2.2.4 (sub): SHEM TYPES — 72 Names of God [8][9]
 * =================================================================== */

typedef enum {
    ANGEL_SERAPHIM       = 0,
    ANGEL_CHERUBIM       = 1,
    ANGEL_THRONES        = 2,
    ANGEL_DOMINIONS      = 3,
    ANGEL_VIRTUES        = 4,
    ANGEL_POWERS         = 5,
    ANGEL_PRINCIPALITIES = 6,
    ANGEL_ARCHANGELS     = 7
} Angelic_Choir;

typedef struct {
    uint8_t  shem_idx;       /* 0-71 flat index (row*9 + col) */
    uint8_t  choir;          /* Angelic_Choir (0-7) */
    uint8_t  position;       /* Position within choir (0-8) */
    uint8_t  element_id;     /* Element_Id (0-4) */
    uint8_t  decan_link;     /* Index into M2_DECAN_DESC (0-71) */
    uint8_t  planet_link;    /* Planet_Id (0-9) */
    uint16_t meaning_id;
} Shem_Name_Desc;

extern const Shem_Name_Desc M2_SHEM_DESC[72];

static inline uint8_t shem_flat_index(uint8_t choir, uint8_t pos_in_choir) {
    return (uint8_t)((choir * 9u) + pos_in_choir);
}

_Static_assert(7u * 9u + 8u == 71u, "8x9 Shem matrix: max flat index must be 71");


/* ===================================================================
 * FR 2.2.4 (sub): MAQAM TYPES — Musical + Spiritual
 * =================================================================== */

typedef struct {
    uint8_t num;
    uint8_t den;
} Harmonic_Ratio;

extern const Harmonic_Ratio M2_MAQAM_RATIOS[10];

typedef struct {
    uint8_t family;             /* Maqam family (0-9) */
    uint8_t mode_in_family;     /* Position within family */
    uint8_t intervals[7];       /* Interval pattern (quarter-tone units, 24-TET) */
    uint8_t planet_ruler;       /* Planet_Id */
    uint16_t meaning_id;
} Maqam_Musical_Desc;

extern const Maqam_Musical_Desc M2_MAQAM_DESC[72];

typedef struct {
    uint8_t  station;           /* 0=La Maqam, 1-7=classical */
    uint8_t  level;             /* 0=Awam, 1=Khawas, 2=Khawas al-Khawas */
    uint16_t meaning_id;
} Maqam_Spiritual_Desc;

extern const Maqam_Spiritual_Desc M2_MAQAM_SPIRITUAL[8][3];


/* ===================================================================
 * FR 2.2.9: ASMA TYPES — 99 Divine Names + Hidden
 * =================================================================== */

typedef struct {
    uint8_t  name_idx;          /* 0-99 */
    uint8_t  group;             /* 0=Jalal, 1=Kamal, 2=Jamal */
    uint8_t  index_in_group;    /* 0-32 */
    uint8_t  element_id;        /* Element_Id (0-4) */
    uint8_t  digital_root;      /* Digital root of abjad_value (1-9) */
    uint8_t  mirror_idx;        /* Index of mirror-pair name (0xFF if none) */
    uint16_t abjad_value;       /* Abjad numerical value */
    uint16_t meaning_id;
    uint8_t  _pad[2];           /* Alignment to 12 bytes */
} Asma_Name_Desc;

_Static_assert(sizeof(Asma_Name_Desc) == 12, "Asma_Name_Desc must be 12 bytes");

extern const Asma_Name_Desc M2_ASMA_LUT[100];

#define ASMA_HIDDEN_INDEX 99u

#define ASMA_GET_DR(desc)      ((desc)->digital_root)
#define ASMA_GET_MIRROR(desc)  ((desc)->mirror_idx)
#define ASMA_HAS_MIRROR(desc)  ((desc)->mirror_idx != 0xFFu)


/* ===================================================================
 * FR 2.2.8: MANTRA TYPES — 100 entries (50 Bimba + 50 Pratibimba)
 * =================================================================== */

typedef struct {
    uint8_t  mantra_idx;            /* 0-99 flat index */
    uint8_t  matrika_group;         /* consonant articulation group (0-7) */
    uint8_t  element_id;            /* Element_Id (0-4) */
    uint8_t  phase;                 /* 0=Bimba/Descent, 1=Pratibimba/Ascent */
    uint16_t fundamental_frequency; /* Hz — range 144-432 Hz */
    uint16_t meaning_id;
} Mantra_Entry_Desc;

_Static_assert(sizeof(Mantra_Entry_Desc) == 8,
    "Mantra_Entry_Desc must be 8 bytes for cache alignment");

#define M2_MANTRA_FREQ_MIN  144u
#define M2_MANTRA_FREQ_MAX  432u
#define M2_MANTRA_FREQ_BASE 256u

extern const Mantra_Entry_Desc M2_MANTRA_LUT[100];


/* ===================================================================
 * FR 2.2.4 (sub): ROUTING MASKS — O(1) ASMA 36:64 Lookup
 * =================================================================== */

typedef struct {
    uint64_t low_64;   /* bits for indices 0-63  */
    uint64_t high_64;  /* bits for indices 64-99 */
} Routing_Mask_128;

_Static_assert(sizeof(Routing_Mask_128) == 16,
    "Routing_Mask_128 must be exactly 16 bytes");

extern const Routing_Mask_128 ASMA_36_INTERNAL_MASK;
extern const Routing_Mask_128 ASMA_64_PROJECTIVE_MASK;

static inline bool m2_is_projective(uint8_t asma_index) {
    if (asma_index < 64)
        return (bool)((ASMA_64_PROJECTIVE_MASK.low_64 >> asma_index) & 1ULL);
    else
        return (bool)((ASMA_64_PROJECTIVE_MASK.high_64 >> (asma_index - 64u)) & 1ULL);
}

static inline bool m2_is_internal(uint8_t asma_index) {
    if (asma_index < 64)
        return (bool)((ASMA_36_INTERNAL_MASK.low_64 >> asma_index) & 1ULL);
    else
        return (bool)((ASMA_36_INTERNAL_MASK.high_64 >> (asma_index - 64u)) & 1ULL);
}


/* ===================================================================
 * FR 2.2.6: ELEMENT THROUGHLINE TABLE
 * =================================================================== */

typedef struct {
    uint8_t tattva_idx;     /* Index in M2_TATTVA_DESC */
    uint8_t decan_element;  /* Decan element array index (0=Fire,1=Earth,2=Air,3=Water,4=Quint) */
    uint8_t mantra_group;   /* Matrika consonant group index */
    uint8_t chakra_idx;     /* Chakra_Id */
} Element_Throughline;

extern const Element_Throughline M2_ELEMENTS[5];


/* ===================================================================
 * FR 2.2.7: DISCRETE EPISTEMIC TRANSFORM (DET) — M2→M3 Bridge
 *
 * 72 uint64_t masks. Bitwise OR = wave superposition.
 * 72 x (8/9) = 64: the epogdoon compression.
 * =================================================================== */

extern const uint64_t M2_TO_M3_CYMATIC_PROJECTION[72];

/* Wave superposition: OR active M2 states into M3 bitboard */
static inline uint64_t transduce_vibration_to_symbol(
        const uint8_t m2_active_indices[],
        uint8_t count)
{
    uint64_t m3_bitboard = 0;
    for (uint8_t i = 0; i < count; i++) {
        m3_bitboard |= M2_TO_M3_CYMATIC_PROJECTION[m2_active_indices[i]];
    }
    return m3_bitboard;
}

/* Epogdoon integer transforms — 9:8 ratio */
static inline uint8_t m2_epogdoon_compress(uint8_t val_72) {
    return (uint8_t)((val_72 * 8u) / 9u);
}

static inline uint8_t m3_epogdoon_expand(uint8_t val_64) {
    return (uint8_t)((val_64 * 9u) / 8u);
}


/* ===================================================================
 * FR 2.2.5: CAUSAL RESONANCE MASKS — O(1) Cross-Weave
 *
 * 36 uint64_t bitmasks: one per base MEF condition.
 * Bit N set in mask K means condition K resonates with condition N.
 * =================================================================== */

extern const uint64_t M2_CAUSAL_RESONANCE_MASKS[36];


/* ===================================================================
 * M2 ROOT STRUCT — HC-anchored module state
 * =================================================================== */

typedef struct {
    Holographic_Coordinate*       hc;          /* FIRST FIELD — HC_LINK'd to Psychoid_2 mirror */
    const Holographic_Coordinate* active_cf;   /* CF_TABLE[CF_TRIKA] */
    Elemental_Signature           active_elem; /* Current active element */
    uint8_t                       active_decan;  /* Current decan index */
    uint8_t                       active_tattva; /* Current tattva index */
    uint8_t                       _pad;
} M2_Root;


/* ===================================================================
 * PUBLIC API
 * =================================================================== */

/* Allocate and HC-link M2_Root; hc must be Psychoid_2's mutable mirror */
M2_Root* m2_init(Coordinate_Arena* arena, Holographic_Coordinate* hc);

/* Release M2_Root heap state (not the HC itself) */
void     m2_teardown(M2_Root* root);

/* CLI dispatch entry point: argv[0] = "m2" */
int      m2_cli_dispatch(int argc, char** argv, M2_Root* root);

/* Boot-time verification of .rodata integrity */
bool     m2_verify(void);


/* ===================================================================
 * ASPECT COMPUTATION — Angular relationships between planet degrees
 *
 * 5 major aspects: conjunction(0°), sextile(60°), square(90°),
 * trine(120°), opposition(180°). Each with standard orb tolerance.
 * Degree inputs are float [0, 360). Wrap-around handled.
 * =================================================================== */

#define ASPECT_NONE         0xFF
#define ASPECT_CONJUNCTION  0
#define ASPECT_SEXTILE      1
#define ASPECT_SQUARE       2
#define ASPECT_TRINE        3
#define ASPECT_OPPOSITION   4
#define ASPECT_COUNT        5

static const uint16_t MAJOR_ASPECT_ANGLE[5] = { 0, 60, 90, 120, 180 };
static const uint8_t  MAJOR_ASPECT_ORB[5]   = { 10, 6, 8, 8, 10 };

typedef struct {
    uint8_t aspect_type;  /* ASPECT_CONJUNCTION..ASPECT_OPPOSITION, or ASPECT_NONE */
    float   angle;        /* Actual angular separation [0, 180] */
    float   orb;          /* Deviation from exact aspect angle */
} Aspect_Result;

/* Compute aspect between two zodiacal degrees (0-360).
 * Returns the tightest matching major aspect, or ASPECT_NONE. */
static inline Aspect_Result m2_aspect_between(float deg_a, float deg_b) {
    float diff = deg_a - deg_b;
    if (diff < 0.0f) diff = -diff;
    if (diff > 180.0f) diff = 360.0f - diff;

    Aspect_Result best = { .aspect_type = ASPECT_NONE, .angle = diff, .orb = 999.0f };

    for (uint8_t i = 0; i < ASPECT_COUNT; i++) {
        float dev = diff - (float)MAJOR_ASPECT_ANGLE[i];
        if (dev < 0.0f) dev = -dev;
        if (dev <= (float)MAJOR_ASPECT_ORB[i] && dev < best.orb) {
            best.aspect_type = i;
            best.orb = dev;
        }
    }

    return best;
}


#endif /* M2_H */
