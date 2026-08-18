/**
 * m0.h — Anuttara: The Multi-Level Language Runtime (Subsystem #0)
 *
 * Implements: M0 (#0) = bare-metal VM of six nested micro-algebras
 * Context frame: (00/00) — CF_VOID (Receptive Dynamism)
 * Anchored to: Psychoid_0 in psychoid_numbers.c (Layer 1 .rodata)
 * Builds on:  ontology.h (HC struct), vak.h (instruction dispatch)
 * Feeds into: M1-M5 via holographic Eckhartian correspondence
 *
 * ARCHITECTURE RULE: M0_Root has Holographic_Coordinate* hc
 * as its first field. Bind with HC_LINK(). Never access without GET_PTR().
 *
 * Public interface — all consumers need only:
 *   m0_init(arena, hc)          — allocate and HC-link the M0 root struct
 *   m0_read_cosmic_clock(deg)   — O(1) unified M1/M2/M3 clock accessor
 *   m0_compute_logos_state(tick) — branchless 12-stage pipeline state
 *   m0_teardown(root)           — release heap state (not the HC itself)
 *   m0_cli_dispatch(argc, argv, root) — CLI entry point
 */

#ifndef M0_H
#define M0_H

#include "ontology.h"
#include "psychoid_numbers.h"
#include "arena.h"
#include "m1.h"
#include "vak.h"
#include <stdbool.h>

/* ===================================================================
 * I. COMPILED FORMULATION — every M0 coordinate's concrescence trace
 *
 * The formulation is the SOURCE CODE of the ontological element.
 * The compiled fields are its executable PRATIBIMBA reflection.
 * =================================================================== */

/* Forward ref — Spanda_Discriminator defined below */
typedef union Spanda_Discriminator Spanda_Discriminator;

typedef struct {
    uint8_t  signature;        /* 7-bit operator mask + 1 Spanda bit (Spanda_Discriminator.raw) */
    uint8_t  reduction_depth;  /* number of -> steps in full formulation (2-8) */
    uint8_t  dimensionality;   /* terminal form: 0, 1, or 2 (/D suffix) */
    uint8_t  terminal_opcode;  /* final Vimarsa opcode this reduces to (0x0-0x7) */
    uint16_t core_chain;       /* 4-step reduction kernel (Vimarsa_Bytecode) */
    uint16_t _pad;             /* alignment to 8 bytes */
    const char* source;        /* full human-readable formulation (Obsidian only) */
} Compiled_Formulation;

_Static_assert(sizeof(Compiled_Formulation) == 16, "Compiled_Formulation must be 16 bytes");


/* ===================================================================
 * II. FR 2.0.0: VIMARSA ENGINE — 3-bit bytecode ISA
 * =================================================================== */

typedef enum {
    OP_VIMARSA_NULL = 0x0,
    OP_ILLUMINATE   = 0x1,   /* ?!  — Vimarsa-Prakasa       */
    OP_PROVOKE      = 0x2,   /* !?  — Prakasa-Vimarsa       */
    OP_WITHHOLD     = 0x3,   /* (-) — Negation/Withholding  */
    OP_ADD_PRESENCE = 0x4,   /* +@  — Additive Presence     */
    OP_ENCLOSE      = 0x5,   /* (@) — Enclosed Presence     */
    OP_EQUATE       = 0x6,   /* =   — Equivalence           */
    OP_DISTINGUISH  = 0x7    /* !=  — Distinction           */
} Vimarsa_Opcode;

#define VIMARSA_OP_COUNT 7

/* 4-step reduction chain packed into 16 bits:
 * (OP1 << 9) | (OP2 << 6) | (OP3 << 3) | OP4 */
typedef uint16_t Vimarsa_Bytecode;

/* Extract opcode at step N (0-3) from a Vimarsa_Bytecode */
#define VBC_STEP(bc, n) ((Vimarsa_Opcode)(((bc) >> (9 - (n)*3)) & 0x07u))

/* Pack 4 opcodes into a Vimarsa_Bytecode */
#define VBC_PACK(a, b, c, d) \
    ((Vimarsa_Bytecode)(((a)<<9)|((b)<<6)|((c)<<3)|(d)))

typedef struct {
    const char*      symbol;     /* display form — Obsidian only */
    Vimarsa_Opcode   opcode;     /* 3-bit identity */
    Vimarsa_Bytecode reduction;  /* 4-step core reduction */
    uint8_t          arity;      /* 0=nullary, 1=unary, 2=binary */
} Vimarsa_Operator;

extern const Vimarsa_Operator VIMARSA_TABLE[VIMARSA_OP_COUNT];


/* ===================================================================
 * III. FR 2.0.0-B: CONCRESCENCE STATE MACHINE
 * =================================================================== */

typedef enum {
    TETRAL_ZERO    = 0,   /* 0: neutral ground              */
    TETRAL_PLUSMIN = 1,   /* +/-0: fused 0/1 singularity    */
    TETRAL_NEG     = 2,   /* -0: primal withholding         */
    TETRAL_POS     = 3,   /* +0: existent withholding       */
    TETRAL_VOID    = 4    /* 00: void recurrence            */
} Tetralemmic_Position;

#define CONCRESCENCE_TETRALEMMIC_STATES 5u
#define CONCRESCENCE_STEPS 12u

typedef struct {
    Tetralemmic_Position states[CONCRESCENCE_STEPS];
    uint8_t              step_count;
    uint8_t              is_descending;
} Concrescence_Trace;

_Static_assert(CONCRESCENCE_STEPS == RING_SIZE,
    "Concrescence helix must match the 12-step SU(2) ring");

/* The Quaternionic Ground — Purnata (#0-3-7)
 * q = 0 + 0i + 0j + 0k — all rotation latent, none manifest. */
static const Quaternion PURNATA_QUATERNION_SEED = {
    .w = 0.0f, .x = 0.0f, .y = 0.0f, .z = 0.0f
};


/* ===================================================================
 * IV. FR 2.0.1: SPANDA DISCRIMINATOR — 8-bit atomic state
 * =================================================================== */

union Spanda_Discriminator {
    uint8_t raw;
    struct {
        uint8_t op_masks : 7;  /* bits 0-6: one bit per Vimarsa operator */
        uint8_t spanda   : 1;  /* bit 7: 0=AND (boundless), 1=OR (limitation) */
    } bits;
};

_Static_assert(sizeof(Spanda_Discriminator) == 1, "Spanda_Discriminator must be 1 byte");

#define SPANDA_AND  0u   /* inclusive simultaneity */
#define SPANDA_OR   1u   /* exclusive choice      */


/* ===================================================================
 * V. FR 2.0.2: VOID ARITHMETIC
 * =================================================================== */

typedef uint8_t void_ops_t;

#define VOID_OP_TRANSCEND   (1u << 0)
#define VOID_OP_REFLECT     (1u << 1)
#define VOID_OP_GENERATE    (1u << 2)
#define VOID_OP_SYNTHESIZE  (1u << 3)
#define VOID_OP_DUAL(x)     ((x) << 4)

#define VOID_9  9u  /* Wholeness constant: (00+00) = 9 */

typedef struct {
    uint8_t  r_factor;
    uint8_t  divine_act;
    uint16_t cross_branch_refs;
    const char* name;           /* virtue name, e.g. "Joy/Play - Creation Virtue" */
    const char* symbol;         /* mathematical symbol, e.g. "0R = @ = (9-O#-X#-N#)" */
} Virtue_Entry;

extern const Virtue_Entry VIRTUE_LUT[9];

#define VIRTUE_TO_RFACTOR(v) ((v) >= 3u ? (uint8_t)((v) - 3u) : 0xFFu)


/* ===================================================================
 * VI. FR 2.0.3: ARCHETYPAL NUMBER LANGUAGE — 12-FOLD
 *
 * 12 elements: (-) mirror + 0/1 singularity + numbers 0-9
 * =================================================================== */

/* Sub-table forward declarations */
typedef struct Zodiacal_Entry   Zodiacal_Entry;
typedef struct Monopoly_Entry   Monopoly_Entry;
typedef struct DivineAct_Entry  DivineAct_Entry;

#define SUB_TABLE_NONE      0u
#define SUB_TABLE_ZODIACAL  1u
#define SUB_TABLE_MONOPOLY  2u
#define SUB_TABLE_DIVINE    3u
#define SUB_TABLE_VIRTUE    4u

#define ARCHETYPE_LUT_SIZE 12  /* (-) + 0/1 + 0-9 */

typedef struct {
    uint8_t  index;
    uint8_t  dimensionality;
    uint8_t  polarity;         /* 0=ADAM, 1=EVE, 2=NEUTRAL */
    uint8_t  complement_idx;   /* ADAM_EVE partner (0xFF if none) */
    uint8_t  weave_anchor[2];
    uint8_t  sub_table_type;
    uint8_t  sub_table_size;

    union {
        const void*            raw_ptr;
        const Zodiacal_Entry*  zodiacal_grammar;
        const Monopoly_Entry*  monopoly_dialectic;
        const DivineAct_Entry* divine_acts;
        const Virtue_Entry*    virtue_entries;
    } sub_table;

    const char* symbol;         /* short mathematical symbol: "(-)", "0/1", "2", "3"… "9" */
    Compiled_Formulation formulation;
} Archetype_Entry;

extern const Archetype_Entry ARCHETYPE_LUT[ARCHETYPE_LUT_SIZE];

/* ===================================================================
 * VI-B. MIRROR CHILDREN — Frame () and Operator - (#0-3-0/1-0, #0-3-0/1-1)
 *
 * The two pre-numerical meta-elements that compose the Mirror (-).
 * Referenced by M1 Paramasiva:
 *   #1-0 --EMBODIES_FRAME_PRINCIPLE--> #0-3-0/1-0    (The Frame '()')
 *   #1-0 --EMBODIES_OPERATOR_PRINCIPLE--> #0-3-0/1-1 (The Operator '-')
 * These are the zero-dimensional and one-dimensional foundations of Vimarsa.
 * =================================================================== */

typedef struct {
    uint8_t     position;             /* 0=Frame, 1=Operator */
    const char* symbol;               /* "()" or "-" */
    const char* name;                 /* "The Frame" or "The Operator" */
    const char* core_nature;          /* from dataset coreNature */
    const char* operational_essence;  /* from dataset operationalEssence */
} Mirror_Child_Entry;

#define MIRROR_CHILDREN_COUNT 2u
extern const Mirror_Child_Entry MIRROR_CHILDREN[MIRROR_CHILDREN_COUNT];

/* FR 2.0.3-H: Zodiacal Quality Augment */
#define ZOD_ELEM_FIRE    0x00u
#define ZOD_ELEM_EARTH   0x04u
#define ZOD_ELEM_AIR     0x08u
#define ZOD_ELEM_WATER   0x0Cu

#define ZOD_MODE_CARDINAL  0x00u
#define ZOD_MODE_FIXED     0x01u
#define ZOD_MODE_MUTABLE   0x02u

struct Zodiacal_Entry {
    const char* symbol;         /* Vak operator, e.g. "!", "!-", "?!/!?" — from #0-3-6 children */
    uint8_t resonance;
    uint8_t successor;
    uint8_t zodiacal_quality;
};

extern const Zodiacal_Entry ZODIACAL_LUT[12];

#define ZOD_GET_ELEMENT(q)   (((q) >> 2) & 0x03u)
#define ZOD_GET_MODALITY(q)  ((q) & 0x03u)

struct Monopoly_Entry {
    uint8_t position;
    uint8_t shadow_opposite;
    uint8_t light_opposite;
};

extern const Monopoly_Entry MONOPOLY_LUT[7];

/* FR 2.0.3-I: Divine Act CF and Weave Augment */
typedef uint8_t DivineAct_WeaveMask;

struct DivineAct_Entry {
    uint8_t position;
    uint8_t enables_next;
    uint8_t manifests_arch;
    uint8_t cf_id;
    DivineAct_WeaveMask weave_mask;
    const char* symbol;         /* symbolic formula, e.g. "(R0)", "(R#)" */
    const char* name;           /* act name, e.g. "Srishti - Creation" */
};

extern const DivineAct_Entry DIVINE_ACT_LUT[7];


/* ===================================================================
 * VII. FR 2.0.4: FIVE-LEVEL QL META-LOGIC STACK
 * =================================================================== */

typedef struct {
    uint8_t     frame_id;       /* 0=O#, 1=X#, 2=N#, 3=M#, 4=# */
    const char* designation;    /* Obsidian layer only */
    uint8_t     torus_next[6];  /* internal mod-6 walk */
    uint8_t     generates;      /* next frame_id in cascade (0xFF=terminal) */
    Compiled_Formulation formulation;
} QL_Frame;

extern const QL_Frame QL_STACK[5];

/* FR 2.0.4-H: Nara bridge entries */
#define NARA_POLARITY_YIN    0u
#define NARA_POLARITY_YANG   1u
#define NARA_POLARITY_BOTH   2u

typedef struct {
    uint8_t frame_position;
    uint8_t polarity;
    uint8_t dominant_val;
    uint8_t archetype_role;
} Nara_Entry;

extern const Nara_Entry NARA_MSHARP_LUT[5];


/* ===================================================================
 * VIII. FR 2.0.5: SIVA-SHAKTI OPERATOR/PROCESSOR DUALITY
 * =================================================================== */

typedef void (*Siva_Operator)(struct Holographic_Coordinate* self, void* operand);

typedef struct {
    const char*   symbol;
    Siva_Operator execute;
    uint8_t       cross_logical;
    uint8_t       cross_archetype;
} Siva_Entry;

extern const Siva_Entry SIVA_TABLE[6];


/* ===================================================================
 * VIII-B. SHAKTI TABLE — 6 entries (#0-5-5/0 children)
 *
 * The other half of the Siva-Shakti polarity. SIVA_TABLE covers Siva
 * (static operator/processor side); SHAKTI_TABLE covers Shakti
 * (dynamic psyche/duration side). @0=## through @5=R# map to QL frames.
 *
 * Dataset: nodes_anuttara.json #0-5-5/0-0 through #0-5-5/0-5
 * =================================================================== */

typedef struct {
    uint8_t     shakti_id;    /* 0-5 */
    uint8_t     ql_system;   /* QL_STACK index: 0=##/O#, 1=O#, 2=X#, 3=N#, 4=M#, 5=R# */
    const char* symbol;      /* e.g. "@0 = ##", "@5 = R#" */
    const char* name;        /* "Shakti Library", "Shakti Techne" etc */
    const char* description; /* operationalEssence from dataset */
} Shakti_Entry;

#define SHAKTI_TABLE_SIZE 6u
extern const Shakti_Entry SHAKTI_TABLE[SHAKTI_TABLE_SIZE];


/* ===================================================================
 * IX. FR 2.0.6: R-FACTOR ROUTING
 *
 * R-Factor = "Reality Factor" — a form of context sequence (CS)
 * operating at the cross-M scale. Expresses emanation chains
 * (descending from higher M-levels) and absorption chains
 * (ascending back). Complementarity law: Rx + Ry -> x+y = 5.
 * =================================================================== */

typedef uint16_t R_Factor_Route;

#define GET_R_POS(route, r_idx) (((route) >> ((r_idx) * 3)) & 0x07u)

/* Route words — 3 bits per R-factor, value 7 = absent
 * Complementarity law: Rx + R(5-x) = 5 where both present */
#define ROUTE_O_SHARP   ((R_Factor_Route)0x5FC0u)
#define ROUTE_X_SHARP   ((R_Factor_Route)0x4A09u)
#define ROUTE_N_SHARP   ((R_Factor_Route)0x3852u)
#define ROUTE_M_SHARP   ((R_Factor_Route)0x269Bu)
#define ROUTE_NARA      ((R_Factor_Route)0x14E4u)
#define ROUTE_SIVA      ((R_Factor_Route)0x032Du)
#define ROUTE_SHAKTI    ((R_Factor_Route)0x717Fu)

#define R_FACTOR_ROUTE_COUNT 7

extern const R_Factor_Route R_FACTOR_ROUTE_TABLE[R_FACTOR_ROUTE_COUNT];


/* ===================================================================
 * X. FR 2.0.9: DIVINE ACT ENUM — 6-to-6 isomorphism
 * =================================================================== */

typedef enum {
    ACT_SRISHTI   = 0,
    ACT_STHITI    = 1,
    ACT_SAMHARA   = 2,
    ACT_TIRODHANA = 3,
    ACT_ANUGRAHA  = 4,
    ACT_SAMAVESA  = 5
} Divine_Act;

_Static_assert((int)ACT_SAMAVESA == 5, "Divine_Act must span 0-5");


/* ===================================================================
 * XI. FR 2.0.8 + 2.0.10: UNIFIED CLOCK + LOGOS STATE
 * =================================================================== */

typedef struct {
    uint8_t  tick12;          /* Canonical discrete clock position 0–11.
                               * IS torus_stage IS spanda_stage — one field, one name.
                               * Source: 00-canonical-invariants.md §3 */
    uint8_t  m2_decan_phase;
    uint8_t  m3_hexagram_id;
    bool     is_implicate_phase;
} Unified_Clock_State;

static inline Unified_Clock_State m0_read_cosmic_clock(uint16_t degree_0_to_719) {
    Unified_Clock_State s;
    s.is_implicate_phase = hopf_fiber(degree_0_to_719);
    uint16_t base = hopf_project(degree_0_to_719);
    s.tick12 = hopf_tick12(degree_0_to_719);
    uint8_t base_decan = (uint8_t)(base / 10u);  /* 10° per decan — decan-specific */
    s.m2_decan_phase = s.is_implicate_phase ? (uint8_t)(base_decan + 36u) : base_decan;
    s.m3_hexagram_id = (uint8_t)((base * 64u) / FULL_CYCLE_DEG);
    return s;
}

typedef uint8_t LogosStage;

typedef struct {
    uint8_t    pipeline_tick;
    LogosStage current_stage;
    Divine_Act active_divine_act;
    bool       is_implicate;
    uint8_t    active_r_factor;
} Unified_Logos_State;

static inline Unified_Logos_State m0_compute_logos_state(uint8_t tick_0_to_11) {
    Unified_Logos_State s;
    s.pipeline_tick = tick_0_to_11;
    s.is_implicate  = (tick_0_to_11 >= 6u);
    s.current_stage = (LogosStage)(s.is_implicate
                          ? (11u - tick_0_to_11)
                          : tick_0_to_11);
    s.active_divine_act = (Divine_Act)s.current_stage;
    s.active_r_factor   = (uint8_t)s.current_stage;
    return s;
}


/* ===================================================================
 * XII. FR 2.0.X: CROSS-BRANCH EDGE REGISTRY
 * =================================================================== */

typedef enum {
    QL_CAT_FOUNDATIONAL_TRANSCENDENT = 0,
    QL_CAT_IMPLICATE                 = 1,
    QL_CAT_IMPLICATE_EXPLICATE       = 2,
} M0_QL_Category;

typedef struct {
    uint8_t m0_sub_branch;
    uint8_t macro_m_branch;
    uint8_t relation_type;
    uint8_t cf_id;
} Cross_Branch_Edge;

#define HOLOGRAPHIC_MICRO_IMAGE_REL 0u
#define VEILED_BY_SKIN_REL          1u
#define GENERATES_REL               2u

#define M0_CROSS_BRANCH_COUNT 7u

extern const Cross_Branch_Edge M0_CROSS_BRANCH_REGISTRY[M0_CROSS_BRANCH_COUNT];
extern const uint8_t M0_GOVERNING_CF[6];


/* ===================================================================
 * XIII. M0_CORE_RELATIONS — Curated cross-system relational skeleton
 *
 * Coordinate encoding (4 nibbles): M0C(m,b,c,d) = ((m)<<12)|((b)<<8)|((c)<<4)|(d)
 * where each nibble is 0-15, 0xF = absent/leaf-level.
 * Examples:
 *   #0 root  = M0C_ROOT(0)    = 0x0FFF
 *   #0-3     = M0C_BR(0,3)    = 0x03FF
 *   #0-3-6   = M0C_D1(0,3,6)  = 0x036F
 *   #0-3-6-0 = M0C(0,3,6,0)   = 0x0360
 *   #1-3-5   = M0C_D1(1,3,5)  = 0x135F
 *   #0-3-10  = M0C_D1(0,3,0xA)= 0x03AF
 *
 * 65 entries: Tier1=20 structural skeleton, Tier2=14 anchors, Tier3=31 hot-links
 * Tier4 (remaining ~900 Parashakti relations) lives in Neo4j only.
 * =================================================================== */

#define M0C(m,b,c,d)  ((uint16_t)(((uint16_t)(m)<<12)|((uint16_t)(b)<<8)|((uint16_t)(c)<<4)|(uint16_t)(d)))
#define M0C_ROOT(m)   M0C((m),0xF,0xF,0xF)
#define M0C_BR(m,b)   M0C((m),(b),0xF,0xF)
#define M0C_D1(m,b,c) M0C((m),(b),(c),0xF)

typedef enum {
    M0_REL_HAS_COMPONENT               = 0,
    M0_REL_HAS_ARCHETYPE_COMPONENT     = 1,
    M0_REL_HAS_VIRTUE_COMPONENT        = 2,
    M0_REL_HAS_ZODIACAL_COMPONENT      = 3,
    M0_REL_HAS_MONOPOLY_COMPONENT      = 4,
    M0_REL_HAS_DIVINE_ACT_COMPONENT    = 5,
    M0_REL_HAS_MIRROR_COMPONENT        = 6,
    M0_REL_PROVIDES_ABSOLUTE_GROUND    = 7,
    M0_REL_COMPLETES_CYCLE_INTO        = 8,
    M0_REL_MANIFESTS_ZERO_LOGIC        = 9,
    M0_REL_R_FACTOR_SUBSYSTEM_WEAVING  = 10,
    M0_REL_INHERITS_CONCRESCENCE       = 11,
    M0_REL_EMBODIES_ARCHETYPE_0        = 12,
    M0_REL_EMBODIES_ARCHETYPE_1        = 13,
    M0_REL_EMBODIES_ARCHETYPE          = 14,
    M0_REL_ACHIEVES_SIVA_SHAKTI_UNITY  = 15,
    M0_REL_EMBODIES_FRAME_PRINCIPLE    = 16,
    M0_REL_EMBODIES_OPERATOR_PRINCIPLE = 17,
    M0_REL_TRIKA_VOID_SOURCE           = 18,
    M0_REL_O_SYSTEM_ZERO_LOGIC_BRIDGE  = 19,
    M0_REL_X_SYSTEM_IMAGINATION_BRIDGE = 20,
    M0_REL_DIVINE_SPEECH_TEMPLATES     = 21,
    M0_REL_O_SYSTEM_AFFIRMATION        = 22,
    M0_REL_O_SYSTEM_NEGATION           = 23,
    M0_REL_CREATION_ACT                = 24,
    M0_REL_SUSTENANCE_ACT              = 25,
    M0_REL_DISSOLUTION_ACT             = 26,
    M0_REL_VEILING_ACT                 = 27,
    M0_REL_EMANATES_VIRTUE             = 28,
    M0_REL_GENERATES                   = 29,
} M0_Rel_Type;

typedef struct {
    uint16_t source_coord;  /* M0C(...) packed coordinate */
    uint16_t target_coord;  /* M0C(...) packed coordinate */
    uint8_t  rel_type;      /* M0_Rel_Type (fits in uint8_t: max 29) */
    uint8_t  tier;          /* 1=structural skeleton, 2=anchor, 3=hot-link */
    uint8_t  _pad[2];       /* alignment to 8 bytes */
} M0_Relation;

#define M0_CORE_RELATIONS_COUNT 65u
extern const M0_Relation M0_CORE_RELATIONS[M0_CORE_RELATIONS_COUNT];


/* ===================================================================
 * XIV. M5 LOGOS ADVANCE CALLBACK — #5→#0 COMPLETES_CYCLE_INTO arc
 *
 * Wire from m5_init: m0_bind_m5_logos(fn_wrapper, m5_root_ptr)
 * This avoids a circular header dependency (m0.h ← m5.h would be circular).
 * =================================================================== */

typedef Unified_Logos_State (*M5_Logos_Advance_Fn)(void* m5_root);
void m0_bind_m5_logos(M5_Logos_Advance_Fn fn, void* m5_root);


/* ===================================================================
 * XV. M0 ROOT STRUCT — HC-anchored module state
 * =================================================================== */

typedef struct {
    Holographic_Coordinate* hc;           /* FIRST FIELD — always. HC_LINK'd. */
    const Holographic_Coordinate* active_cf;  /* CF_TABLE[CF_VOID] */
    Spanda_Discriminator spanda_state;
    void_ops_t active_void_ops;
    Unified_Logos_State logos_state;
} M0_Root;


/* ===================================================================
 * XIV. PUBLIC API (<=6 functions)
 * =================================================================== */

M0_Root* m0_init(Coordinate_Arena* arena, Holographic_Coordinate* hc);
void     m0_teardown(M0_Root* root);
int      m0_cli_dispatch(int argc, char** argv, M0_Root* root);

/* R-factor weave dispatcher — dispatches across 7 frames */
void m0_execute_r_factor_weave(Unified_Logos_State* state);

/* Verify holographic registry at boot */
bool m0_verify_holographic_registry(void);


#endif /* M0_H */
