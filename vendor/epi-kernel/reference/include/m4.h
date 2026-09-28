/**
 * m4.h — Nara: The Personal Dialogical Interface (Subsystem #4)
 *
 * Implements: M4 (#4) = personal oracle interface, Lemniscate anchor.
 * Context frame: (4.0/1-4.4/5) — CF_FRACTAL (Fractal Doubling)
 * Anchored to: Psychoid_4 in psychoid_numbers.c (Layer 1 .rodata)
 * Builds on:  ontology.h, psychoid_numbers.h, arena.h, m0.h, m2.h, m3.h
 * Feeds into: M5 (Logos FSM, Mobius write-back)
 *
 * Architecture: Vtable dispatch (not bitwise). 6-lens registry indexed.
 * All personal/mutable state = heap (PCO). Identity computed once.
 * Floats ONLY in M4_Jung_Complex (charge, autonomy).
 *
 * FR Coverage: 2.4.0 – 2.4.13 (M4-nara-personal-interface.md)
 *
 * ARCHITECTURE RULE: M4_Root has Holographic_Coordinate* hc
 * as its first field. Bind with HC_LINK(). Never access without GET_PTR().
 *
 * Public interface — all consumers need only:
 *   m4_init(arena, hc)              — allocate and HC-link M4 root
 *   m4_identity_compute(id, input)  — compute-once Symbol DNA + BLAKE3
 *   m4_snapshot_now(degree, epoch)  — create M4_Temporal_Now
 *   m4_planet_degrees_live(now)     — live kairos planet vector
 *   m4_advance_transformation(eng)  — modulo cascade cycle engine
 *   m4_teardown(root)               — release heap state
 *   m4_cli_dispatch(argc, argv, rt) — CLI entry point
 */

#ifndef M4_H
#define M4_H

#include "ontology.h"
#include "psychoid_numbers.h"
#include "arena.h"
#include "m0.h"
#include "m2.h"
#include "m3.h"
#include "m_canonical.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


/* ===================================================================
 * FR 2.4.7: ELEMENTAL THROUGHLINE CONSTANTS
 *
 * The four-element backbone linking M2 (vibration), M3 (codons), M4 (identity).
 * A=Water=Cups=Feeling, T=Fire=Wands=Intuition,
 * C=Earth=Pentacles=Sensation, G=Air=Swords=Thinking.
 * =================================================================== */

/* Canonical L2' element IDs (see m_canonical.h). The nucleotide↔element
 * identity is preserved (A=Water, T=Fire, C=Earth, G=Air) but the numeric
 * IDs are now the L2' canonical ordering, NOT the historical 0-3 sequence. */
#define M4_ELEM_WATER   ELEMENT_WATER   /* Adenine  — Cups — Feeling     (Yin)  — canonical 2 */
#define M4_ELEM_FIRE    ELEMENT_FIRE    /* Thymine  — Wands — Intuition  (Yang) — canonical 4 */
#define M4_ELEM_EARTH   ELEMENT_EARTH   /* Cytosine — Pentacles — Sensation (Yin) — canonical 1 */
#define M4_ELEM_AIR     ELEMENT_AIR     /* Guanine  — Swords — Thinking  (Yang) — canonical 3 */

/* Nucleotide-to-element consistency check — routed through the canonical
 * throughline mapping rather than raw integer equality, since nucleotide
 * IDs (A=0..G=3) no longer coincide with canonical element IDs. */
_Static_assert(m4_nuc_to_elem(M3_NUC_A) == M4_ELEM_WATER, "Elemental Throughline: A must be Water");
_Static_assert(m4_nuc_to_elem(M3_NUC_T) == M4_ELEM_FIRE,  "Elemental Throughline: T must be Fire");
_Static_assert(m4_nuc_to_elem(M3_NUC_C) == M4_ELEM_EARTH, "Elemental Throughline: C must be Earth");
_Static_assert(m4_nuc_to_elem(M3_NUC_G) == M4_ELEM_AIR,   "Elemental Throughline: G must be Air");


/* ===================================================================
 * FR 2.4.0: M4_Symbol_DNA_Profile — The Symbol DNA Blueprint
 *
 * The user's personal coordinates within M3's universal address space.
 * gene_keys_activation is a mask over M3's 64-bitboard.
 * nucleotide_balance translates Jungian type into elemental throughline.
 * =================================================================== */

typedef struct {
    uint64_t gene_keys_activation;  /* M3_Matrix_Word — M3 bitboard mask */

    struct {
        uint8_t adenine_water;      /* Cups / Feeling (Fi/Fe) */
        uint8_t thymine_fire;       /* Wands / Intuition (Ni/Ne) */
        uint8_t cytosine_earth;     /* Pentacles / Sensation (Si/Se) */
        uint8_t guanine_air;        /* Swords / Thinking (Ti/Te) */
    } nucleotide_balance;

    uint16_t sun_degree_anchor;     /* 0-719: natal sun position on 720 clock */
    uint16_t moon_degree_anchor;    /* 0-719: natal moon position */
} M4_Symbol_DNA_Profile;


/* ===================================================================
 * FR 2.4.0 EXPANDED: 6-slot open identity architecture
 * Each slot is independently optional. Hash computed from present slots only.
 * =================================================================== */

/* #4.0-0 — Birthdate Numerological Encoding */
typedef struct {
    uint32_t numerological_key;     /* Original 4-fold encoding from date arithmetic */
    uint8_t  sixfold_difference;    /* 6-fold difference (Bimba/Pratibimba tension) */
    uint8_t  sixfold_sum;           /* 6-fold sum (synthesis) */
    uint8_t  life_path;             /* Single-digit life path number (1-9, 11, 22, 33) */
    uint8_t  _pad;
} M4_Numerological_Layer;           /* #4.0-0 — 8 bytes */

_Static_assert(sizeof(M4_Numerological_Layer) == 8,
    "M4_Numerological_Layer must be 8 bytes");

/* #4.0-1 — Astrological Natal Chart */
typedef struct {
    uint16_t sun_degree_anchor;     /* 0-719 on SU(2) ring */
    uint16_t moon_degree_anchor;    /* 0-719 */
    uint16_t asc_degree_anchor;     /* Ascendant/Rising sign (0-719) */
    uint16_t mc_degree_anchor;      /* Midheaven (0-719) */
    uint16_t planet_degrees[10];    /* All 10 planets (Planet_Id order from m2.h) */
    uint8_t  dominant_sign;         /* 0-11 zodiac sign index */
    uint8_t  dominant_element;      /* Element_Id from m2.h */
    uint8_t  dominant_modality;     /* 0=Cardinal, 1=Fixed, 2=Mutable */
    uint8_t  _pad;
} M4_Astrological_Layer;            /* #4.0-1 — 32 bytes */

_Static_assert(sizeof(M4_Astrological_Layer) == 32,
    "M4_Astrological_Layer must be 32 bytes");

/* #4.0-2 — Jungian / MBTI Typological Assessment */
typedef struct {
    struct {
        uint8_t adenine_water;      /* Cups / Feeling (Fi/Fe) — 0-255 intensity */
        uint8_t thymine_fire;       /* Wands / Intuition (Ni/Ne) */
        uint8_t cytosine_earth;     /* Pentacles / Sensation (Si/Se) */
        uint8_t guanine_air;        /* Swords / Thinking (Ti/Te) */
    } nucleotide_balance;           /* The elemental throughline encoding */
    uint8_t  mbti_raw;              /* 4-bit MBTI: bit0=E/I, bit1=S/N, bit2=T/F, bit3=J/P */
    uint8_t  dominant_function;     /* Cognitive function index (0=Ti..7=Se) */
    uint8_t  auxiliary_function;    /* Secondary cognitive function index */
    uint8_t  enneagram_type;        /* 1-9 (0 = not set) */
    uint8_t  enneagram_wing;        /* 1-9 (0 = not set) */
    uint8_t  _pad[3];
} M4_Jungian_Layer;                 /* #4.0-2 — 12 bytes */

_Static_assert(sizeof(M4_Jungian_Layer) == 12,
    "M4_Jungian_Layer must be 12 bytes");

/* #4.0-3 — Gene Keys / I-Ching Activation Profile */
typedef struct {
    uint64_t gene_keys_activation;  /* M3_Matrix_Word mask: bit N = hexagram N active */
    uint64_t shadow_mask;           /* Low-frequency shadow hexagrams active in profile */
    uint64_t gift_mask;             /* Mid-frequency gift hexagrams */
    uint64_t siddhi_mask;           /* High-frequency siddhi hexagrams */
    uint8_t  life_work_hex;         /* Primary Gene Key (Sun hexagram) — 1-64 */
    uint8_t  evolution_hex;         /* Earth hexagram (opposite of life_work) */
    uint8_t  radiance_hex;          /* Moon hexagram */
    uint8_t  purpose_hex;           /* Nodal axis hexagram */
    uint8_t  attraction_hex;        /* Venus hexagram */
    uint8_t  iq_hex;                /* Mercury hexagram */
    uint8_t  eq_hex;                /* Moon south node hexagram */
    uint8_t  sq_hex;                /* Ascendant hexagram ("spiritual quotient") */
} M4_GeneKeys_Layer;                /* #4.0-3 — 40 bytes */

_Static_assert(sizeof(M4_GeneKeys_Layer) == 40,
    "M4_GeneKeys_Layer must be 40 bytes");

/* #4.0-4 — Human Design Profile (stub — extensible) */
typedef struct {
    uint8_t  hd_type;               /* 0=Manifestor, 1=Generator, 2=ManGen, 3=Projector, 4=Reflector */
    uint8_t  hd_authority;          /* 0=Emotional..7=None */
    uint8_t  hd_profile[2];         /* Profile lines (e.g. {1,3} = "Investigator/Martyr") */
    uint8_t  hd_definition;         /* 0=None, 1=Single, 2=Split, 3=Triple, 4=Quadruple */
    uint8_t  incarnation_cross;     /* 0-63 (index into cross LUT — TBD) */
    uint16_t defined_channels;      /* Bitmask of defined channels (0-35) */
    uint32_t defined_gates[2];      /* Two uint32 bitmasks covering all 64 gates */
    uint8_t  _pad[4];
} M4_HumanDesign_Layer;             /* #4.0-4 — 20 bytes */

_Static_assert(sizeof(M4_HumanDesign_Layer) == 20,
    "M4_HumanDesign_Layer must be 20 bytes");

/* Presence bitmask — which layers are populated */
#define M4_LAYER_0_PRESENT  (1u << 0u)  /* Numerological */
#define M4_LAYER_1_PRESENT  (1u << 1u)  /* Astrological */
#define M4_LAYER_2_PRESENT  (1u << 2u)  /* Jungian */
#define M4_LAYER_3_PRESENT  (1u << 3u)  /* Gene Keys */
#define M4_LAYER_4_PRESENT  (1u << 4u)  /* Human Design */

/* ===================================================================
 * FR 2.4.1: M4_Input_Data + M4_Identity_Matrix (EXPANDED)
 *
 * Input is transient: zeroed immediately after compute.
 * Identity is compute-once, then effectively const.
 * =================================================================== */

typedef struct {
    uint32_t birth_year;
    uint8_t  birth_month;
    uint8_t  birth_day;
    uint8_t  birth_hour;
    uint8_t  birth_minute;
    float    birth_latitude;
    float    birth_longitude;
    uint8_t  mbti_raw;              /* Raw MBTI code before translation */
    uint8_t  _pad[3];
} M4_Input_Data;

/* The expanded Identity Matrix — 6 independent layer slots */
typedef struct {
    /* Presence guard — which layers are populated */
    uint8_t  layer_presence;        /* Bitmask: M4_LAYER_N_PRESENT */
    uint8_t  _pad_lp[7];           /* Align to 8-byte boundary */

    /* The six sub-system layers (each independently optional) */
    M4_Numerological_Layer  layer_0;  /* #4.0-0 — 8 bytes */
    M4_Astrological_Layer   layer_1;  /* #4.0-1 — 28 bytes */
    M4_Jungian_Layer        layer_2;  /* #4.0-2 — 12 bytes */
    M4_GeneKeys_Layer       layer_3;  /* #4.0-3 — 40 bytes */
    M4_HumanDesign_Layer    layer_4;  /* #4.0-4 — 20 bytes */

    /* The Symbol DNA Profile — synthesized view (legacy compat) */
    M4_Symbol_DNA_Profile   dna_profile;

    /* #4.0-5 — Quintessence Hash
     * BLAKE3(layer_presence || present_layer_data...) full 32-byte output.
     * Canonical: 00-canonical-invariants.md §1 — never truncated. */
    uint8_t  quintessence_hash[32];

    /* Hex preview string — DERIVED from quintessence_hash, NOT the identity.
     * 64 hex chars + null terminator. Updated whenever hash is recomputed. */
    char     quintessence_preview[65];

    /* Legacy fields (preserved for existing M4 subsystem compatibility) */
    uint32_t numerological_key;     /* #4.0-0: birthdate encoding */
    uint8_t  jung_type;             /* #4.0-2: 4-bit MBTI composite */

    /* Compute-once guard */
    bool     computed;
} M4_Identity_Matrix;

static inline bool m4_identity_ready(const M4_Identity_Matrix* id) {
    return id->computed;
}

uint8_t m4_identity_layer_count(const M4_Identity_Matrix* id);

/* Compute quintessence hash from present layers only */
void m4_identity_hash_compute(M4_Identity_Matrix* id);

/* Augment: add a new layer to existing matrix, recompute hash */
void m4_identity_augment(M4_Identity_Matrix* id,
                         uint8_t layer_index,
                         const void* new_layer_data,
                         size_t new_layer_size);


/* ===================================================================
 * FR 2.4.11: M4_Temporal_Now — The Lived Moment
 *
 * Composes M1/M2/M3 clock with planetary preemption slots.
 * Works at 0 planets (stub mode) through 10 planets (full mod-10 relay).
 * =================================================================== */

typedef enum {
    KAIROS_FRAME_NATAL = 0,
    KAIROS_FRAME_REALTIME = 1,
    KAIROS_FRAME_KAIROTIC = 2
} KairosFrameKind;

typedef struct {
    KairosFrameKind kind;                /* NATAL, REALTIME, or KAIROTIC */
    uint64_t        captured_at_ns;      /* Monotonic or wall-clock capture time */
    uint64_t        decays_at_ns;        /* 0 means no decay deadline */
    uint16_t        planet_degrees[10];  /* All 10 planets (Planet_Id order from m2.h) */
    float           pp;                  /* Oracle charge: prospective/prospective */
    float           mm;                  /* Oracle charge: mirror/mirror */
    float           mp;                  /* Oracle charge: mirror/prospective */
    float           pn;                  /* Oracle charge: prospective/null */
    uint32_t        _pad;
} KairosFrame;

_Static_assert(sizeof(KairosFrame) == 64, "KairosFrame must be one 64-byte L1 line");

typedef struct {
    Unified_Clock_State clock;          /* M1/M2/M3 concentric state */
    uint16_t            degree;         /* 0-719 (SU(2) double cover) */
    uint32_t            chronos_epoch;  /* Unix seconds */

    KairosFrame natal;                  /* Birth-session frame; persists across realtime refreshes */
    KairosFrame realtime;               /* Mercurius live relay frame */
    KairosFrame kairotic;               /* Oracle consultation frame; may decay */
    uint8_t     kairotic_active;        /* Live accessor prefers kairotic only while active */
    uint16_t planet_valid;              /* 10-bit mask: which planets have data */
} M4_Temporal_Now;

#define M4_PLANET_VALID_ALL ((uint16_t)((1u << M2_PLANET_COUNT) - 1u))

/* Default kairotic decay: a captured oracle-consultation sky preempts realtime
 * for 4 hours, after which m4_planet_degrees_live_at reverts to realtime. */
#define M4_KAIROTIC_DEFAULT_TTL_NS ((uint64_t)4 * 3600ull * 1000000000ull)

extern const uint16_t M4_EMPTY_PLANET_DEGREES[M2_PLANET_COUNT];

static inline uint64_t m4_epoch_to_ns(uint32_t epoch) {
    return ((uint64_t)epoch) * 1000000000ull;
}

KairosFrame m4_kairos_frame_init(KairosFrameKind kind, uint64_t captured_at_ns);

void m4_kairos_frame_set_planets(KairosFrame* frame,
                                 const uint16_t planet_degrees[M2_PLANET_COUNT],
                                 uint16_t planet_valid);

M4_Temporal_Now m4_snapshot_now(uint16_t degree, uint32_t epoch);

void m4_temporal_now_set_planets(M4_Temporal_Now* now,
                                 const uint16_t planet_degrees[M2_PLANET_COUNT],
                                 uint16_t planet_valid);

/* Arm the kairotic tier: capture a live oracle-consultation sky, activate it
 * (it now preempts realtime in m4_planet_degrees_live), and set its decay
 * deadline to captured_at_ns + ttl_ns. ttl_ns == 0 uses M4_KAIROTIC_DEFAULT_TTL_NS
 * (4h). After the deadline, m4_planet_degrees_live_at deactivates it and the
 * live accessor reverts to realtime. This is the ONLY setter that arms the
 * kairotic frame on a live path (m4_temporal_now_set_planets writes realtime). */
void m4_temporal_now_capture_kairotic(M4_Temporal_Now* now,
                                      const uint16_t planet_degrees[M2_PLANET_COUNT],
                                      uint16_t planet_valid,
                                      uint64_t captured_at_ns,
                                      uint64_t ttl_ns);

const uint16_t* m4_planet_degrees_live(const M4_Temporal_Now* now);

const uint16_t* m4_planet_degrees_live_at(M4_Temporal_Now* now, uint64_t now_ns);

M4_Temporal_Now m4_snapshot_now_with_planets(uint16_t degree,
                                             uint32_t epoch,
                                             const uint16_t planet_degrees[M2_PLANET_COUNT],
                                             uint16_t planet_valid);


/* ===================================================================
 * FR 2.4.13: ORACLE PRIMITIVES — Sacred Random + Casting Results
 *
 * True random via arc4random_buf(). Consent-gated per invocation.
 * =================================================================== */

typedef struct {
    bool     consent_granted;       /* User MUST consent per invocation */
    uint64_t session_nonce;         /* Unique per-session */
} M4_Sacred_Random;

typedef struct {
    uint8_t  lines[6];              /* Line values (6/7/8/9), bottom to top */
    uint8_t  hexagram_id;           /* 6-bit: resulting hexagram (0-63) */
    uint8_t  changing_mask;         /* 6-bit: which lines are changing */
    uint8_t  result_hexagram;       /* Hexagram after changes applied */
    uint16_t cast_degree;           /* Kairos moment degree */
} M4_IChing_Cast;

typedef struct {
    uint8_t  cards[78];             /* Full deck (Fisher-Yates shuffled) */
    uint8_t  drawn[12];             /* Drawn cards (max spread size) */
    uint8_t  draw_count;            /* How many drawn */
    uint8_t  spread_type;           /* Spread ID */
    uint16_t cast_degree;           /* Kairos moment */
} M4_Tarot_Draw;

/* Universal emission format — bridges oracle -> medicine -> transformation */
typedef struct {
    uint16_t tag_id;                /* Universal symbol identifier */
    uint8_t  tradition;             /* 0=Tarot, 1=I-Ching, 2=custom */
    uint8_t  nucleotide;            /* A/T/C/G via Elemental Throughline */
    uint8_t  element;               /* Fire/Earth/Air/Water/Akasha */
    uint16_t degree;                /* Position on M3 wheel (0-719) */
} M4_Canonical_Tag;


/* ===================================================================
 * FR 2.4.13b: M4_Oracle_Draw — Expanded oracle draw result
 *
 * Captures full draw + canonical payload for gateway emission.
 * =================================================================== */

typedef struct {
    uint8_t  system;                /* 0=tarot-rws, 1=tarot-thoth, 2=tarot-marseille,
                                       3=tarot-ql, 4=iching-coins, 5=iching-yarrow */
    uint8_t  draw_count;            /* Cards drawn or lines cast */
    uint8_t  drawn[12];             /* Card ids (tarot) or line values (iching) */
    uint8_t  reversal_mask;         /* Bitmask: which cards are reversed (tarot) */
    uint8_t  hexagram_primary;      /* I-Ching: primary hexagram (0-63), 0xFF if tarot */
    uint8_t  hexagram_relating;     /* I-Ching: relating hexagram, 0xFF if none */
    uint8_t  changing_mask;         /* I-Ching: 6-bit changing line mask */
    uint16_t cast_degree;           /* Kairos moment degree (0-719) */
    uint32_t cast_epoch;            /* Unix seconds at cast time */
    uint8_t  consent_granted;       /* 1 = user consented */
    uint8_t  hygiene_status;        /* 0=clear, 1=warning, 2=blocked */
    uint8_t  _pad[2];
    M4_Canonical_Tag canonical_tags[12]; /* Up to 12 canonical tag payloads */
    uint8_t  canonical_tag_count;   /* How many tags populated */
    uint8_t  _pad2[3];
} M4_Oracle_Draw;                   /* ~128 bytes */


/* ===================================================================
 * FR 2.4.1b: M4_Medicine_Triage — Compact elemental triage
 * =================================================================== */

typedef struct {
    uint8_t  fire;                  /* 0-255 intensity */
    uint8_t  water;
    uint8_t  earth;
    uint8_t  air;
    uint8_t  dominant_element;      /* Element_Id */
    uint8_t  deficient_element;     /* Element_Id */
    uint8_t  primary_chakra;        /* Chakra_Id (most activated) */
    uint8_t  triage_vector;         /* 0=balanced, 1=excess, 2=deficient, 3=blocked */
    uint32_t planetary_hour;        /* Current planetary hour (from kairos) */
    uint8_t  safety_mask;           /* Contraindication bitmask */
    uint8_t  _pad[3];
} M4_Medicine_Triage;               /* 16 bytes */

_Static_assert(sizeof(M4_Medicine_Triage) == 16,
    "M4_Medicine_Triage must be 16 bytes");


/* ===================================================================
 * FR 2.4.4b: M4_Transform_State — Compact transformation state
 * =================================================================== */

typedef struct {
    uint8_t  current_op;            /* Alchemical operation index (0-6) */
    uint8_t  stroke_phase;          /* 0=outer (writing), 1=inner (reflecting) */
    uint8_t  cycle_count_today;     /* Cycles completed today */
    uint8_t  container_active;      /* 0=none, 1=bohm, 2=circle, 3=diamond */
    uint32_t cycle_id;              /* Current cycle sequential id */
    uint32_t opened_at;             /* Unix seconds when cycle opened */
    uint8_t  decan_recipe_idx;      /* Active decan recipe (0-35) */
    uint8_t  arousal_level;         /* 0-255 */
    uint8_t  safety_threshold;      /* If arousal > threshold → halt */
    uint8_t  _pad;
} M4_Transform_State;               /* 16 bytes */

_Static_assert(sizeof(M4_Transform_State) == 16,
    "M4_Transform_State must be 16 bytes");


/* ===================================================================
 * FR 2.4.3: DIVINATION VTABLE + MAGIC-NUMBER TYPE SAFETY
 * =================================================================== */

#define M4_MAGIC_TAROT  0x5441524Fu     /* 'TARO' */
#define M4_MAGIC_ICHING 0x49434847u     /* 'ICHG' */

typedef struct {
    uint32_t magic;                 /* M4_MAGIC_TAROT — MUST be first */
    uint8_t  deck_id;
    uint8_t  spread_type;
    uint8_t  _pad[2];
} M4_Tarot_State;

typedef struct {
    uint32_t magic;                 /* M4_MAGIC_ICHING — MUST be first */
    uint8_t  casting_method;
    uint8_t  _pad[3];
    uint64_t hexagram_result;
} M4_IChing_State;

typedef struct {
    const char* tradition_name;
    int (*cast)(void* state, const void* query, void* result);
    int (*interpret)(const void* cast_result, const void* context,
                     char* output, size_t len);
    int (*validate)(const void* cast_result);
} Divination_Vtable;


/* ===================================================================
 * FR 2.4.1: M4_Sympathetic_Medicine — Elemental balance + safety
 * =================================================================== */

typedef struct {
    uint8_t fire;
    uint8_t earth;
    uint8_t air;
    uint8_t water;
    uint8_t quintessence;
} M4_Elemental_Balance;

typedef struct {
    uint8_t id;
    uint8_t activation;
    uint8_t blockage;
} M4_Chakra_State;

typedef int (*Alchemical_Op)(void* state, const void* reagent);

typedef struct {
    M4_Elemental_Balance elements;      /* #4.1-0 */
    M4_Chakra_State chakras[7];         /* #4.1-1 */
    void* pharmacopeia;                 /* #4.1-2 */
    Alchemical_Op ops[7];               /* #4.1-3: 7 canonical verbs */
    uint8_t  planetary_hour;            /* #4.1-4 */
    uint16_t current_degree;            /* Degree in M3 wheel (0-719) */
    uint8_t  lunar_phase;               /* Moon phase (0-27 sidereal) */
    uint8_t  safety_level;              /* #4.1-5: 0=blocked, 255=clear */
    bool     contraindicated;           /* Emergency brake */
} M4_Sympathetic_Medicine;

static inline bool m4_medicine_safe(const M4_Sympathetic_Medicine* med) {
    return !med->contraindicated && med->safety_level > 0;
}


/* ===================================================================
 * FR 2.4.4: M4_Decan_Recipe_Card + M4_Cycle_Engine
 *
 * 12x3 = 36 recipe cards in .rodata. Modulo cascade advance.
 * =================================================================== */

typedef struct {
    uint8_t  storey;                /* 0-11 (mod-12) */
    uint8_t  decan;                 /* 0-2 (trinitarian sub-position) */
    uint8_t  op_count;
    uint8_t  ops[7];                /* Indices into 7 canonical verbs */
    uint16_t optimal_degree;        /* M3 wheel position (0-719) */
    uint8_t  planetary_hour;
    uint8_t  element_focus;
    uint8_t  chakra_focus;
    uint8_t  _pad;
} M4_Decan_Recipe_Card;

typedef struct {
    uint8_t current_storey;         /* 0-11 */
    uint8_t current_decan;          /* 0-2 */
    uint8_t current_stroke;         /* 0-23 */
    const M4_Decan_Recipe_Card* active_recipe;
    uint8_t arousal_level;          /* 0-255 */
    uint8_t safety_threshold;       /* If arousal > threshold -> halt */
    bool    poison_cure_toggle;     /* Paracelsian dose toggle */
} M4_Cycle_Engine;

/* FR 2.4.4: Modulo cascade — no nested if/else */
void m4_advance_transformation(M4_Cycle_Engine* engine);

static inline bool m4_transformation_safe(const M4_Cycle_Engine* engine) {
    return engine->arousal_level <= engine->safety_threshold;
}


/* ===================================================================
 * FR 2.4.9: SAFETY GOVERNOR
 * =================================================================== */

typedef enum {
    STALL_NONE          = 0,
    STALL_AROUSAL       = 1,        /* Arousal exceeds threshold */
    STALL_CONTRAINDICATED = 2,      /* Medical safety brake */
    STALL_CONSENT       = 3,        /* Oracle consent not granted */
    STALL_MEF_LOW       = 4         /* MEF below lens threshold */
} M4_Stall_Type;

typedef struct {
    M4_Stall_Type type;
    uint8_t       severity;         /* 0-255 */
    uint8_t       hysteresis;       /* Band width before re-clear */
    uint8_t       _pad;
} M4_Safety_Governor;

M4_Safety_Governor m4_safety_check(
    const M4_Cycle_Engine* engine,
    const M4_Sympathetic_Medicine* med,
    const M4_Sacred_Random* rng);


/* ===================================================================
 * FR 2.4.8: CONTAINER LUT — Dialogical inquiry containers
 * =================================================================== */

typedef struct {
    uint8_t container_type;         /* 0=circle, 1=temenos, 2=vessel */
    uint8_t capacity;               /* Max participants */
    uint8_t _pad[2];
} M4_Container_Entry;

_Static_assert(sizeof(M4_Container_Entry) == 4,
    "M4_Container_Entry must be 4 bytes");

#define M4_CONTAINER_COUNT 3


/* ===================================================================
 * FR 2.4.5: M4_Jung_Complex — THE ONLY FLOATS IN M4
 *
 * Floats permitted ONLY in charge and autonomy.
 * Never for array indices. Threshold checks and Pi Agent reporting only.
 * =================================================================== */

typedef struct {
    uint8_t archetype_id;
    float   charge;                 /* Emotional intensity */
    float   autonomy;               /* Autonomous operation (0.0-1.0) */
    bool    conscious;              /* Made conscious? */
} M4_Jung_Complex;

/* Jungian type: 4 functions + 2 attitudes in 6 bits */
typedef uint8_t M4_Jung_Type;
#define JUNG_INTROVERT  0x00
#define JUNG_EXTRAVERT  0x20
#define JUNG_THINKING   0x02        /* -> Guanine/Air */
#define JUNG_FEELING    0x04        /* -> Adenine/Water */
#define JUNG_SENSATION  0x08        /* -> Cytosine/Earth */
#define JUNG_INTUITION  0x10        /* -> Thymine/Fire */

/* Alchemical stage state machine */
typedef enum {
    ALCH_PRIMA_MATERIA = 0,
    ALCH_NIGREDO       = 1,
    ALCH_ALBEDO        = 2,
    ALCH_CITRINITAS    = 3,
    ALCH_RUBEDO        = 4,
    ALCH_TRANSCENDENT  = 5
} M4_Alchemical_Stage;

bool m4_alchemy_can_advance(M4_Alchemical_Stage current,
                            M4_Alchemical_Stage target);


/* ===================================================================
 * FR 2.4.4: M4_Lens_Vtable + M4_Personal_Pratibimba + M4_Context_Lenses
 * =================================================================== */

typedef struct {
    const char* lens_name;
    int (*translate)(uint8_t lens_id, const char* input,
                     const char* context, char* output, size_t len);
    int (*activate)(uint8_t lens_id, void* phenom_state);
    int (*deactivate)(uint8_t lens_id, void* phenom_state);
    int (*annotate)(uint8_t lens_id, const void* experience,
                    char* annotation, size_t len);
} M4_Lens_Vtable;

typedef struct {
    const M4_Symbol_DNA_Profile* user_dna;
    M4_Sympathetic_Medicine* current_body_state;
    MEF_Condition* active_epistemic_lens;   /* -> M2 .rodata */
    M4_Jung_Complex active_complexes[6];
    bool    identity_manifested;
    bool    telemetry_active;
    uint32_t pattern_count;
    uint64_t synthesized_signature;         /* quintessence in action */
} M4_Personal_Pratibimba;

typedef struct {
    const M4_Lens_Vtable* registry;         /* -> M4_LENS_REGISTRY[6] */
    uint8_t active_lens;                    /* Currently selected (0-5) */
    uint8_t lens_stack[6];                  /* Lens priority stack */
    M4_Jung_Complex* complexes;             /* Dynamic array */
    uint8_t complex_count;
    M4_Alchemical_Stage alch_stage;
    M4_Jung_Type jung_type;
    M4_Personal_Pratibimba pratibimba;
    uint8_t phenom_layer;                   /* 0-5 */
    uint8_t mobius_count;
} M4_Context_Lenses;


/* ===================================================================
 * FR 2.4.10: M4_Voice_Config + Transparency + Logos Cycle
 * =================================================================== */

typedef struct {
    uint8_t formality;              /* 0=casual, 255=formal */
    uint8_t warmth;                 /* 0=clinical, 255=warm */
    uint8_t directness;             /* 0=indirect, 255=direct */
    uint8_t depth;                  /* 0=surface, 255=deep */
    uint8_t metaphor_density;       /* 0=literal, 255=symbolic */
    uint8_t humor;                  /* 0=none, 255=playful */
    uint8_t challenge;              /* 0=supportive, 255=challenging */
    uint8_t _pad;
} M4_Voice_Config;

_Static_assert(sizeof(M4_Voice_Config) == 8, "M4_Voice_Config must be 8 bytes");

typedef struct {
    uint32_t timestamp;
    uint8_t  storey;
    uint8_t  decan;
    uint8_t  stroke;
    uint8_t  operation;
    uint8_t  outcome;
    uint8_t  _pad[3];
} M4_Transparency_Record;

typedef struct {
    uint8_t position;               /* 0-5 (current QL position) */
    uint8_t cycle_count;            /* Complete cycles this session */
    bool    complete;               /* Has this episode finished? */
} M4_Logos_Cycle_State;


/* ===================================================================
 * FR 2.4.5: M4_Epii_Integration + Mobius return
 * =================================================================== */

typedef struct {
    void* curriculum;                       /* #4.5-0 */
    const M4_Voice_Config* voice_config;    /* #4.5-1: .rodata */
    void* transparency_log;                 /* #4.5-2 */
    void* integration_state;                /* #4.5-3 */
    uint8_t readiness_level;                /* #4.5-4 */
    M4_Logos_Cycle_State logos;             /* #4.5-5 */
    uint64_t wisdom_delta;
    bool     return_ready;
} M4_Epii_Integration;

void m4_mobius_return(M4_Epii_Integration* epii,
                      M4_Identity_Matrix* identity);


/* ===================================================================
 * M4_Symbolic_Protein — Session-as-transcription body
 * =================================================================== */

#define M4_SYMBOLIC_PROTEIN_MAX_STEPS 256u
#define M4_SYMBOLIC_PROTEIN_DEFAULT_CAPACITY 256u
#define M4_TRANSCRIPTION_TAIL_MARKER_CODON 0xFEu

#define M4_TRANSCRIPTION_STEP_START  (1u << 0u)
#define M4_TRANSCRIPTION_STEP_STOP   (1u << 1u)
#define M4_TRANSCRIPTION_STEP_TAIL   (1u << 2u)

typedef enum {
    M4_STOP_CODON_POLICY_KAIROS_DERIVED = 0,
    M4_STOP_CODON_POLICY_ROUND_ROBIN = 1,
    M4_STOP_CODON_POLICY_FIXED_TAA = 2,
    M4_STOP_CODON_POLICY_FIXED_TAG = 3,
    M4_STOP_CODON_POLICY_FIXED_TGA = 4
} M4_Stop_Codon_Policy;

typedef struct {
    uint16_t degree;
    uint8_t  hexagram;
    uint8_t  codon;
    uint8_t  amino_acid;
    uint8_t  transcript_class;      /* M3_TranscriptClass */
    uint8_t  governance_role;       /* M3_GovernanceRole */
    uint8_t  flags;                 /* M4_TRANSCRIPTION_STEP_* */
} M4_TranscriptionStep;

typedef struct {
    char     session_id[64];
    uint8_t  start_codon;
    uint8_t  stop_codon;
    uint8_t  sealed;
    uint8_t  truncated;
    uint64_t kairos_open;
    uint64_t kairos_close;
    uint8_t  identity_hash[32];
    uint32_t step_count;
    uint32_t capacity;
    uint8_t  has_mythos_archetype_reading;
    char     mythos_archetype_reading[256];
    M4_TranscriptionStep steps[M4_SYMBOLIC_PROTEIN_MAX_STEPS];
} M4_Symbolic_Protein;


/* ===================================================================
 * M4_Session_Frame — Session-context inheritance at open
 *
 * Opening a session inherits three things together: the lived moment
 * (kairos), who is present (identity), and the cards drawn at that
 * moment (tarot_psyche_anchor). The draw is the contextual envelope
 * around the START marker, not a parallel lifecycle mechanism. Seeded
 * from kairos so the same moment recalls the same draw; the randomness
 * across moments is the necessary openness.
 * =================================================================== */

typedef struct {
    uint64_t        kairos;              /* Inherited moment (RNG seed source) */
    M4_Identity_Matrix* identity;       /* Inherited identity (caller-owned) */
    M4_Tarot_Draw   tarot_psyche_anchor;/* Cards drawn at open — context, not event */
    M4_Symbolic_Protein protein_storage;/* Owned bounded session protein */
    M4_Symbolic_Protein* protein;       /* Protected handle to protein_storage */
    M4_Stop_Codon_Policy stop_codon_policy;
    bool            opened;             /* True once m4_session_open succeeds */
} M4_Session_Frame;


/* ===================================================================
 * PCO: M4_PersonalContextOverlay — The complete heap struct
 * =================================================================== */

typedef struct {
    M4_Identity_Matrix identity;
    M4_Sympathetic_Medicine medicine;
    M4_Cycle_Engine cycle;
    M4_Context_Lenses lenses;
    M4_Epii_Integration epii;

    struct {
        M4_Transparency_Record* records;
        uint32_t count;
        uint32_t capacity;
    } phase_history;

    M4_Sacred_Random rng;
    M4_Temporal_Now last_now;
} M4_PersonalContextOverlay;


/* ===================================================================
 * M4 ROOT STRUCT — HC-anchored module state
 * =================================================================== */

typedef struct {
    Holographic_Coordinate*       hc;           /* FIRST FIELD — HC_LINK'd */
    const Holographic_Coordinate* active_cf;    /* CF_TABLE[CF_FRACTAL] */
    M4_PersonalContextOverlay     pco;          /* All mutable personal state */
} M4_Root;


/* ===================================================================
 * .RODATA EXTERN PROMISES — verification-visible LUT data
 * =================================================================== */

extern const M4_Lens_Vtable      M4_LENS_REGISTRY[6];
extern const M4_Decan_Recipe_Card M4_PROTOCOL_LIBRARY[12][3];
extern const Alchemical_Op       M4_ALCHEMY_OPS[7];
extern const uint8_t             M4_LENS_MEF_THRESHOLD[6];
extern const M4_Voice_Config     M4_VOICE_CONFIG;
extern const M4_Container_Entry  M4_CONTAINER_LUT[M4_CONTAINER_COUNT];


/* ===================================================================
 * PUBLIC API
 * =================================================================== */

/* Allocate and HC-link M4_Root; hc must be Psychoid_4's mutable mirror */
M4_Root* m4_init(Coordinate_Arena* arena, Holographic_Coordinate* hc);

/* Compute-once identity: derives Symbol DNA, BLAKE3 quintessence.
 * Zeroes input immediately after compute. */
void m4_identity_compute(M4_Identity_Matrix* id, M4_Input_Data* input);

/* Release M4_Root heap state (not the HC itself) */
void m4_teardown(M4_Root* root);

/* Boot-time .rodata verification */
bool m4_verify(void);

/* CLI dispatch entry point: argv[0] = "m4" */
int m4_cli_dispatch(int argc, char** argv, M4_Root* root);

/* Oracle casting functions */
int m4_cast_iching(M4_Sacred_Random* rng, uint16_t cast_degree,
                   M4_IChing_Cast* out);
int m4_draw_tarot(M4_Sacred_Random* rng, uint8_t count, uint16_t cast_degree,
                  M4_Tarot_Draw* out);

/* Open a session: inherit kairos + identity, draw the conditioning cards.
 * Seeds the RNG deterministically from kairos (mixed with identity) so the
 * same moment recalls the same draw. Returns 0 on success, <0 on error.
 * The drawn cards land in out->tarot_psyche_anchor as session context. */
int m4_session_open(M4_Identity_Matrix* identity, uint64_t kairos,
                    M4_Session_Frame* out);

/* Append a transcription step to a session protein. If the bounded capacity is
 * exhausted, the final slot is replaced with a tail marker and no overflow
 * occurs. Returns 0 on success, <0 on error. */
int m4_symbolic_protein_append_step(M4_Symbolic_Protein* protein,
                                    uint16_t degree,
                                    uint8_t hexagram,
                                    uint8_t codon,
                                    M3_GovernanceRole role);

/* Close a session protein by emitting a STOP marker selected by the frame's
 * stop_codon_policy (default: kairos-derived). The sealed protein is copied to
 * out as an opaque handle payload for protected write-through. */
int m4_session_close(M4_Session_Frame* frame, M4_Symbolic_Protein* out);

/* Consent-gated true random */
bool m4_sacred_random(M4_Sacred_Random* rng, uint8_t* buf, size_t len);


#endif /* M4_H */
