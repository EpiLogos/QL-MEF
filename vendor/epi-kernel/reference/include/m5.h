/**
 * m5.h — Epii: The Holographic Integration Layer (Subsystem #5)
 *
 * Implements: M5 (#5) = holographic container, Logos FSM, quintessential view self-API.
 * Context frame: (5/0) — CF_MOBIUS (Total Synthesis / Mobius Return)
 * Anchored to: Psychoid_5 in psychoid_numbers.c (Layer 1 .rodata)
 * Builds on:  ontology.h, psychoid_numbers.h, arena.h, m0.h, m1.h, m2.h, m3.h, m4.h
 * Feeds into: M0 (Mobius write-back at tick 11)
 *
 * Sub-branch = coordinate family home (holographic containment):
 *   #5-0 = M+M' (integral identity, M0-M5 mirror)
 *   #5-1 = L+P+L'+P' (theory topology, accumulated understanding)
 *   #5-2 = S+S' (full stack objective + project-specific)
 *   #5-3 = M' UI (Electron app, WebMCP hooks)
 *   #5-4 = S4-4'/S4-5' (Anima+Aletheia agent rosters)
 *   #5-5 = T+C+T'+C' (Logos Cycle, cadence of immanence)
 *
 * ARCHITECTURE RULE: M5_Root has Holographic_Coordinate* hc
 * as its first field. Bind with HC_LINK(). Never access without GET_PTR().
 *
 * Public interface — all consumers need only:
 *   m5_init(arena, hc)                — allocate and HC-link M5 root
 *   m5_advance_logos(root)            — advance Logos FSM, return state
 *   m5_execute_mobius_return(root, target)— Sacred Violation (tick 11 only)
 *   m5_lookup(root, coord, gran)      — quintessential view self-API
 *   m5_teardown(root)                 — release heap state
 *   m5_cli_dispatch(argc, argv, root) — CLI entry point
 */

#ifndef M5_H
#define M5_H

#include "ontology.h"
#include "psychoid_numbers.h"
#include "arena.h"
#include "m0.h"
#include "m4.h"
#include <stdbool.h>
#include <stdint.h>


/* ===================================================================
 * I. LOGOS FSM — The Cadence of Immanence (#5-5)
 *
 * 6 stages, 12-tick SU(2) double-cover pipeline.
 * Reuses LogosStage + Unified_Logos_State from m0.h.
 * M5 named constants for stage semantics:
 * =================================================================== */

#define ALOGOS    0u  /* Without word: pregnant silence               */
#define PROLOGOS  1u  /* Before word: first differentiation           */
#define DIALOGOS  2u  /* Through word: relational exchange            */
#define LOGOS_STAGE 3u /* The word: rational articulation              */
#define EPILOGOS  4u  /* Upon/after word: reflexive turn              */
#define ANALOGOS  5u  /* According to ratio: proportional recognition */

/* Logos stage names — defined in m5.c */
extern const char* const M5_LOGOS_STAGE_NAMES[6];

/* M0 (Anuttara) sub-branch pithy strings — defined in qv_data.c */
extern const char* QV_PITHY_M0_BRANCHES[6];

/* Vak consciousness-grammar speech operators (12 zodiacal) — defined in qv_data.c */
extern const char* QV_PITHY_VAK_SPEECH[12];

/* # node navigation portal — orientation lines */
extern const char* QV_HASH_PITHY_LINES[4];

/* Doctrine of Vibration — philosophical ground (epi core knowing # essence) */
extern const char* QV_DOV_PITHY[6];

/* Communications seed-phrases — sympathetic technology register (epi core knowing # comms) */
extern const char* QV_COMMS_SEEDS[8];

/* Navigation map — six families at a glance (epi core knowing # map / # navigate) */
extern const char* QV_NAVIGATE_PITHY[6];


/* ===================================================================
 * II. QUINTESSENTIAL VIEW — Per-coordinate self-description
 *
 * The master self-API. Every coordinate can have a pithy summary
 * (always in C memory) plus hooks to deeper granularity layers.
 * Registers[0-5] match L0-L5 lens modes for multi-framing.
 * =================================================================== */

/* Pack a coordinate ID: family(3 bits) + position(3 bits) + branch(3 bits) + flags(7 bits) */
#define M5_COORD_ID(fam, pos, branch) \
    ((uint16_t)( (((fam) & 0x7) << 13) | (((pos) & 0x7) << 10) | (((branch) & 0x7) << 7) ))

#define M5_COORD_FAMILY(id)   (((id) >> 13) & 0x7)
#define M5_COORD_POSITION(id) (((id) >> 10) & 0x7)
#define M5_COORD_BRANCH(id)   (((id) >> 7) & 0x7)

typedef struct {
    uint16_t    coord_id;           /* Packed coordinate address                */
    uint8_t     register_count;     /* How many registers populated (1-6)      */
    uint8_t     refinement_cycle;   /* Mobius cycles that have refined this     */
    const char* pithy;              /* One-liner, <=128 chars, always present   */
    const char* registers[6];       /* Multi-framing: terse -> verbose (L0-L5) */
} M5_Quintessential_View;

/* S0'/S1'/S2' compression trika — three granularity levels */
typedef struct {
    const char* s0_pithy;           /* Always present — C-level summary        */
    const char* s1_obsidian_path;   /* Path to .md file (NULL if unwritten)    */
    const char* s2_neo4j_query;     /* Cypher template (NULL if unmapped)      */
} M5_Granularity_Hook;

/* Granularity levels for m5_lookup() */
#define M5_GRAN_PITHY    0   /* S0' — terse, always available   */
#define M5_GRAN_OBSIDIAN 1   /* S1' — markdown path             */
#define M5_GRAN_NEO4J    2   /* S2' — graph query template      */


/* ===================================================================
 * III. AGENT ENTRY — Dynamic roster element for Anima/Aletheia
 * =================================================================== */

typedef struct {
    const char* name;
    uint8_t     id;
    uint8_t     capability_flags;
    uint8_t     tool_flags;
    uint8_t     active;              /* 1 = in roster, 0 = disabled */
} M5_Agent_Entry;

/* Agent capability flags */
#define M5_AGENT_CAP_NEO4J       (1u << 0)
#define M5_AGENT_CAP_VECTOR      (1u << 1)
#define M5_AGENT_CAP_SYMBOLIC    (1u << 2)
#define M5_AGENT_CAP_NARRATIVE   (1u << 3)
#define M5_AGENT_CAP_PERSONAL    (1u << 4)
#define M5_AGENT_CAP_MOBIUS      (1u << 5)

/* Agent tool flags */
#define M5_AGENT_TOOL_MCP_BIMBA  (1u << 0)
#define M5_AGENT_TOOL_MCP_NOTION (1u << 1)
#define M5_AGENT_TOOL_OBSIDIAN   (1u << 2)
#define M5_AGENT_TOOL_TENSOR     (1u << 3)
#define M5_AGENT_TOOL_CLOCK      (1u << 4)


/* ===================================================================
 * IV. SUB-BRANCH STRUCTS — One per coordinate family home
 * =================================================================== */

/* #5-0: M+M' — Integral Identity */
typedef struct {
    void*                   m_roots[6];     /* -> M0-M5 root ptrs        */
    M5_Quintessential_View  m_views[6];     /* M0-M5 self-descriptions   */
    M5_Quintessential_View  m_prime[6];     /* M0'-M5' inverted          */
} M5_Identity;

/* #5-1: L+P+L'+P' — Theory Topology */
typedef struct {
    M5_Quintessential_View  l_views[6];     /* L0-L5 lens modes          */
    M5_Quintessential_View  l_prime[6];     /* L0'-L5' inverted          */
    M5_Quintessential_View  p_views[6];     /* P0-P5 positions           */
    M5_Quintessential_View  p_prime[6];     /* P0'-P5' inverted          */
    M5_Granularity_Hook     hooks[36];      /* 6L x 6P coordinate space  */
    uint32_t                session_depth;  /* Sessions that enriched    */
    uint64_t                enrichment_charge;
} M5_Theory;

/* #5-2: S+S' — Full Stack */
typedef struct {
    M5_Quintessential_View  s_views[6];     /* S0-S5 objective           */
    M5_Quintessential_View  s_prime[6];     /* S0'-S5' project-specific  */
} M5_Stack;

/* #5-3: M' UI — Electron/WebMCP */
typedef struct {
    M5_Quintessential_View  m_prime_ui[6];  /* M0'-M5' UI face           */
    void (*webmcp_hook)(uint8_t branch, const void* msg, void* response);
} M5_UI;

/* #5-4: S4-4'/S4-5' — Anima + Aletheia Agent Rosters */
typedef struct {
    M5_Agent_Entry* anima_roster;           /* Dynamic: VAK agents       */
    uint8_t         anima_count;
    uint8_t         anima_capacity;
    M5_Agent_Entry* aletheia_roster;        /* Dynamic: knowledge agents */
    uint8_t         aletheia_count;
    uint8_t         aletheia_capacity;
} M5_Agents;

/* #5-5: T+C+T'+C' — Logos Cycle / Cadence of Immanence */
typedef struct {
    uint8_t                 pipeline_tick;       /* 0-11: THE ENTIRE FSM  */
    M5_Quintessential_View  t_views[6];          /* T0-T5 thought types   */
    M5_Quintessential_View  t_prime[6];          /* T0'-T5'               */
    M5_Quintessential_View  c_views[6];          /* C0-C5 categories      */
    M5_Quintessential_View  c_prime[6];          /* C0'-C5'               */
    uint64_t                archetype_charge[6]; /* Per #0-#5             */
    uint64_t                inverse_charge[6];   /* Per #0'-#5'           */
    uint64_t                frame_charge[6];     /* Per context frame     */
    void (*mobius_write_back)(void* m0_ground, uint64_t charge);
} M5_Logos;


/* ===================================================================
 * V. ETYMOLOGY ARCHAEOLOGY FSM (within ANALOGOS stage)
 * =================================================================== */

typedef enum {
    ETYM_STAGE_PIE_ROOT        = 0,
    ETYM_STAGE_COGNATE_MAP     = 1,
    ETYM_STAGE_SEMANTIC_DRIFT  = 2,
    ETYM_STAGE_PSYCHOID_CHARGE = 3,
    ETYM_STAGE_PROS_HEN        = 4,
    ETYM_STAGE_MOBIUS_WRITEBACK = 5
} M5_Etymology_Stage;

typedef struct {
    M5_Etymology_Stage stage;
    uint8_t            position;
    uint64_t           semantic_charge;
    bool               write_back_ready;
} M5_Etymology_FSM;

void m5_etymology_advance(M5_Etymology_FSM* fsm);


/* ===================================================================
 * VI. PARADOX-HOLDING + SCENT-FOLLOWING
 * =================================================================== */

typedef struct {
    uint64_t scent_vector;
    uint8_t  confidence;
    uint8_t  following_since;
    uint8_t  m_branch_of_origin;
    uint8_t  _pad;
} M5_Scent_State;

typedef struct {
    uint64_t thesis_mask;
    uint64_t antithesis_mask;
    uint8_t  hold_since_tick;
    uint8_t  resolution_stage;
    bool     holding;
    bool     resolved;
} M5_Paradox_Hold;

void m5_hold_paradox(M5_Paradox_Hold* ph, uint64_t thesis,
                     uint64_t antithesis, uint8_t current_tick);

uint64_t m5_resolve_paradox(M5_Paradox_Hold* ph);


/* ===================================================================
 * VII. CONTEMPLATION OBJECT — Session-Close Hologram
 * =================================================================== */

#define M5_CONTEMPLATION_PLANET_COUNT 10u
#define M5_CONTEMPLATION_SYNTAX_SEED_COUNT 4u

typedef struct {
    uint32_t tick;
    double   w;
    double   x;
    double   y;
    double   z;
} M5_Q_BioQuaternion_Tick;

typedef struct {
    uint8_t     codon;
    const char* label;
    const char* m3_route;
} M5_Codon_Trace;

typedef struct {
    const char* dispatch;
    uint32_t    profile_generation;
    const char* profile_anchor;
    const char* acr_route;
} M5_Vak_Profile_Pair;

typedef struct {
    uint32_t pp;
    uint32_t nn;
    uint32_t np;
    uint32_t pn;
    uint32_t outer;
} M5_ArchNineChargeState;

typedef struct {
    const char* name;
} M5_Skeleton_Event;

typedef struct {
    const char* prompt;
} M5_Syntax_Compliance_Seed;

typedef struct {
    const char*                    session_id;
    M4_Temporal_Now                kairos_at_open;
    M4_Temporal_Now                kairos_at_close;
    M4_Tarot_Draw                  tarot_psyche_anchor;
    const M5_Q_BioQuaternion_Tick* q_composed_trajectory;
    uint32_t                       q_composed_trajectory_count;
    const M5_Codon_Trace*          codon_trace;
    uint32_t                       codon_trace_count;
    const M5_Vak_Profile_Pair*     vak_profile_pairs;
    uint32_t                       vak_profile_pair_count;
    M5_ArchNineChargeState         m1_charge_state;
    const M5_Skeleton_Event*       m1_2_skeleton_events_fired;
    uint32_t                       m1_2_skeleton_event_count;
    M5_Syntax_Compliance_Seed      four_syntax_compliance_seeds[M5_CONTEMPLATION_SYNTAX_SEED_COUNT];
} M5_ContemplationObject;

typedef uint64_t (*M5_ContemplateSessionCloseFn)(const M5_ContemplationObject* object,
                                                  void* user_data);

#define M5_MOBIUS_RETURN_TARGET_MAGIC 0x4D354D52u /* "M5MR" */

typedef struct {
    uint32_t                         magic;
    uint32_t                         size;
    M4_Epii_Integration*             epii;
    M4_Identity_Matrix*              identity;
    M5_ContemplateSessionCloseFn     contemplate_session_close;
    void*                            user_data;
} M5_Mobius_Return_Target;

M5_Mobius_Return_Target m5_mobius_return_target(
    M4_Epii_Integration* epii,
    M4_Identity_Matrix* identity,
    M5_ContemplateSessionCloseFn contemplate_session_close,
    void* user_data);


/* ===================================================================
 * VIII. M5_ROOT — The Holographic Container
 * =================================================================== */

typedef struct {
    Holographic_Coordinate* hc;              /* FIRST FIELD — Psychoid_5  */
    const Holographic_Coordinate* active_cf; /* -> CF_TABLE[CF_MOBIUS]    */

    M5_Identity   identity;  /* #5-0: M+M'                               */
    M5_Theory     theory;    /* #5-1: L+P+L'+P'                           */
    M5_Stack      stack;     /* #5-2: S+S'                                */
    M5_UI         ui;        /* #5-3: M' UI                               */
    M5_Agents     agents;    /* #5-4: S4-4'/S4-5'                         */
    M5_Logos      logos;     /* #5-5: T+C+T'+C'                           */

    M5_Etymology_FSM etymology;
    M5_Paradox_Hold  paradox;
    M5_Scent_State   scent;
} M5_Root;


/* ===================================================================
 * IX. PUBLIC API
 * =================================================================== */

M5_Root* m5_init(Coordinate_Arena* arena, Holographic_Coordinate* hc);

Unified_Logos_State m5_advance_logos(M5_Root* root);

int m5_execute_mobius_return(M5_Root* root, void* m0_ground);

const char* m5_lookup(const M5_Root* root, uint16_t coord_id, uint8_t granularity);

void m5_teardown(M5_Root* root);

int m5_cli_dispatch(int argc, char** argv, M5_Root* root);

bool m5_verify(void);


#endif /* M5_H */
