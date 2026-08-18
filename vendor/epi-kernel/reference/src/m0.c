/**
 * m0.c — Anuttara: .rodata LUT section
 * All lookup tables for the M0 VM micro-algebras.
 *
 * Every table here lives in .rodata — immutable BIMBA data.
 * The VM runtime (m0_runtime.c, future) will reference these
 * as the canonical ground for all M0 computation.
 */

#include "m0.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* =============================================================================
 * FR 2.0.0: VIMARSA OPERATOR TABLE — 7 entries
 *
 * Each entry: symbol (Obsidian), opcode (3-bit), reduction (4-step VBC),
 * arity (0=nullary, 1=unary, 2=binary).
 * ============================================================================= */

const Vimarsa_Operator VIMARSA_TABLE[VIMARSA_OP_COUNT] = {
    /* Opcode 0x1: ?! — Illuminate (Vimarsa-Prakasa) */
    { .symbol = "?!",  .opcode = OP_ILLUMINATE,   .reduction = VBC_PACK(OP_ILLUMINATE, OP_PROVOKE, OP_ILLUMINATE, OP_ILLUMINATE), .arity = 1 },
    /* Opcode 0x2: !? — Provoke (Prakasa-Vimarsa) */
    { .symbol = "!?",  .opcode = OP_PROVOKE,      .reduction = VBC_PACK(OP_ILLUMINATE, OP_ILLUMINATE, OP_PROVOKE, OP_ILLUMINATE), .arity = 1 },
    /* Opcode 0x3: (-) — Withhold (Negation) */
    { .symbol = "(-)", .opcode = OP_WITHHOLD,      .reduction = VBC_PACK(OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_WITHHOLD), .arity = 0 },
    /* Opcode 0x4: +@ — Add Presence */
    { .symbol = "+@",  .opcode = OP_ADD_PRESENCE,  .reduction = VBC_PACK(OP_ILLUMINATE, OP_ILLUMINATE, OP_PROVOKE, OP_ENCLOSE), .arity = 2 },
    /* Opcode 0x5: (@) — Enclose */
    { .symbol = "(@)", .opcode = OP_ENCLOSE,       .reduction = VBC_PACK(OP_ILLUMINATE, OP_ILLUMINATE, OP_ADD_PRESENCE, OP_ENCLOSE), .arity = 2 },
    /* Opcode 0x6: = — Equate */
    { .symbol = "=",   .opcode = OP_EQUATE,        .reduction = VBC_PACK(OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_EQUATE), .arity = 2 },
    /* Opcode 0x7: != — Distinguish */
    { .symbol = "!=",  .opcode = OP_DISTINGUISH,   .reduction = VBC_PACK(OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_DISTINGUISH), .arity = 2 },
};

/* =============================================================================
 * FR 2.0.2: VIRTUE LUT — 9 entries
 *
 * Indices 0-2: meta-virtues (Love/Peace, Truth, Openness) — r_factor=0xFF
 * Indices 3-8: mapped virtues (Joy->R0 through Reality->R5)
 * ============================================================================= */

/* Virtue names and symbols sourced from #0-2-9-0 through #0-2-9-8 in dataset.
 * Indices 0-2 are meta-virtues (r_factor=0xFF — not R-factor mapped).
 * Indices 3-8 map to R0-R5 divine acts. */
const Virtue_Entry VIRTUE_LUT[9] = {
    /* [0] Love/Peace — direct equivalence with complete 8-fold void structure */
    { .r_factor = 0xFFu, .divine_act = 0xFFu, .cross_branch_refs = 0x0000,
      .name = "Love/Peace - Foundational Essence",
      .symbol = "(inf*inf)*(R#/##) = 00/00, (0/1), 9" },
    /* [1] Truth — Primordial Matrix ##, first structure from undifferentiated void */
    { .r_factor = 0xFFu, .divine_act = 0xFFu, .cross_branch_refs = 0x0001,
      .name = "Truth - Structural Foundation",
      .symbol = "## = @ = (0/1)-(00)-00" },
    /* [2] Openness/Creativity — perfect non-dual synthesis of # and R */
    { .r_factor = 0xFFu, .divine_act = 0xFFu, .cross_branch_refs = 0x0002,
      .name = "Openness/Creativity - Structural Fusion",
      .symbol = "#R = @ = (7-8-9-(0/1)/O#-X#-N#)" },
    /* [3] Joy/Play -> R0 -> Srishti (Creation) */
    { .r_factor = 0u,    .divine_act = 0u,    .cross_branch_refs = 0x0103,
      .name = "Joy/Play - Creation Virtue",
      .symbol = "0R = @ = (9-O#-X#-N#)" },
    /* [4] Goodness -> R1 -> Sthiti (Sustenance) */
    { .r_factor = 1u,    .divine_act = 1u,    .cross_branch_refs = 0x0104,
      .name = "Goodness - Sustenance Virtue",
      .symbol = "1R = @ = (O#-X#-N#-M#-#-(#))" },
    /* [5] Beauty -> R2 -> Samhara (Dissolution) */
    { .r_factor = 2u,    .divine_act = 2u,    .cross_branch_refs = 0x0105,
      .name = "Beauty - Dissolution Virtue",
      .symbol = "2R = @ = (X#-N#-M#-#-(#)-(@#))" },
    /* [6] Life/Nature -> R3 -> Tirodhana (Veiling) */
    { .r_factor = 3u,    .divine_act = 3u,    .cross_branch_refs = 0x0106,
      .name = "Life/Nature - Veiling Virtue",
      .symbol = "3R = @ = ((@#)-(#)-#-M#-N#-X#)" },
    /* [7] Wisdom -> R4 -> Anugraha (Grace) */
    { .r_factor = 4u,    .divine_act = 4u,    .cross_branch_refs = 0x0107,
      .name = "Wisdom - Grace Virtue",
      .symbol = "4R = @ = ((#)-#-M#-N#-X#-O#)" },
    /* [8] Reality -> R5 -> Samavesa (Absorption) */
    { .r_factor = 5u,    .divine_act = 5u,    .cross_branch_refs = 0x0108,
      .name = "Reality - Completion Virtue",
      .symbol = "5R = @ = (##)" },
};

/* =============================================================================
 * FR 2.0.3-H: ZODIACAL LUT — 12 entries (sub-table of Archetype 5 = Vak)
 * ============================================================================= */

/* Vak operator symbols — the 12 consciousness-grammar operators that constitute
 * the zodiacal 12-fold. Each operator encodes elemental nature + modal quality:
 * Fire/Earth/Air/Water cycling Cardinal/Fixed/Mutable three times.
 * Source: #0-3-6 children in nodes_anuttara.json; §0.1 of plan. */
const Zodiacal_Entry ZODIACAL_LUT[12] = {
    { .symbol = "!",      .resonance = 0u,  .successor = 1u,  .zodiacal_quality = ZOD_ELEM_FIRE  | ZOD_MODE_CARDINAL }, /* Aries    — Cardinal Fire */
    { .symbol = "?",      .resonance = 1u,  .successor = 2u,  .zodiacal_quality = ZOD_ELEM_EARTH | ZOD_MODE_FIXED    }, /* Taurus   — Fixed Earth */
    { .symbol = "!-",     .resonance = 2u,  .successor = 3u,  .zodiacal_quality = ZOD_ELEM_AIR   | ZOD_MODE_MUTABLE  }, /* Gemini   — Mutable Air */
    { .symbol = "-?",     .resonance = 3u,  .successor = 4u,  .zodiacal_quality = ZOD_ELEM_WATER | ZOD_MODE_CARDINAL }, /* Cancer   — Cardinal Water */
    { .symbol = "!?",     .resonance = 4u,  .successor = 5u,  .zodiacal_quality = ZOD_ELEM_FIRE  | ZOD_MODE_FIXED    }, /* Leo      — Fixed Fire */
    { .symbol = "?-",     .resonance = 5u,  .successor = 6u,  .zodiacal_quality = ZOD_ELEM_EARTH | ZOD_MODE_MUTABLE  }, /* Virgo    — Mutable Earth */
    { .symbol = "-!",     .resonance = 6u,  .successor = 7u,  .zodiacal_quality = ZOD_ELEM_AIR   | ZOD_MODE_CARDINAL }, /* Libra    — Cardinal Air */
    { .symbol = "?!",     .resonance = 7u,  .successor = 8u,  .zodiacal_quality = ZOD_ELEM_WATER | ZOD_MODE_FIXED    }, /* Scorpio  — Fixed Water */
    { .symbol = "-!/!-",  .resonance = 8u,  .successor = 9u,  .zodiacal_quality = ZOD_ELEM_FIRE  | ZOD_MODE_MUTABLE  }, /* Sagitt.  — Mutable Fire */
    { .symbol = "-?/?-",  .resonance = 9u,  .successor = 10u, .zodiacal_quality = ZOD_ELEM_EARTH | ZOD_MODE_CARDINAL }, /* Capric.  — Cardinal Earth */
    { .symbol = "!?/?!",  .resonance = 10u, .successor = 11u, .zodiacal_quality = ZOD_ELEM_AIR   | ZOD_MODE_FIXED    }, /* Aquarius — Fixed Air */
    { .symbol = "?!/!?",  .resonance = 11u, .successor = 0u,  .zodiacal_quality = ZOD_ELEM_WATER | ZOD_MODE_MUTABLE  }, /* Pisces   — Mutable Water */
};

/* =============================================================================
 * FR 2.0.3-G: MONOPOLY LUT — 7 entries (sub-table of Archetype 7)
 * ============================================================================= */

const Monopoly_Entry MONOPOLY_LUT[7] = {
    { .position = 0, .shadow_opposite = 0, .light_opposite = 0 },   /* Mono */
    { .position = 2, .shadow_opposite = 3, .light_opposite = 6 },   /* Poly */
    { .position = 3, .shadow_opposite = 2, .light_opposite = 5 },   /* Poly- */
    { .position = 4, .shadow_opposite = 5, .light_opposite = 2 },   /* -Mono */
    { .position = 5, .shadow_opposite = 4, .light_opposite = 3 },   /* Mono- */
    { .position = 6, .shadow_opposite = 7, .light_opposite = 1 },   /* -Poly */
    { .position = 7, .shadow_opposite = 0, .light_opposite = 4 },   /* MonoPoly */
};

/* =============================================================================
 * FR 2.0.3-I: DIVINE ACT LUT — 7 entries (sub-table of Archetype 9)
 *
 * Uses CF_Id values: CF_VOID=0, CF_BINARY=1, CF_TRIKA=2, CF_QUATERNAL=3,
 *                    CF_FRACTAL=4, CF_SYNTHESIS=5, CF_MOBIUS=6
 * ============================================================================= */

/* Divine act symbols sourced from #0-3-10-0/1 through #0-3-10-7 in dataset.
 * DIVINE_ACT_LUT positions 0-6 map to: Svatantrya, Srishti, Sthiti, Samhara,
 * Tirodhana, Anugraha, Samavesa. Note: DIVINE_ACT enum uses positions 0-5
 * (Srishti through Samavesa); Svatantrya is the supra-act at position 0. */
const DivineAct_Entry DIVINE_ACT_LUT[7] = {
    { .position = 0, .enables_next = 1, .manifests_arch = 0xFFu, .cf_id = 1,
      .weave_mask = 0x03u, .symbol = "(R#)", .name = "Svatantrya - Freedom" },
    { .position = 1, .enables_next = 2, .manifests_arch = 0xFFu, .cf_id = 2,
      .weave_mask = 0x07u, .symbol = "(R0)", .name = "Srishti - Creation" },
    { .position = 2, .enables_next = 3, .manifests_arch = 0xFFu, .cf_id = 3,
      .weave_mask = 0x0Fu, .symbol = "(R1)", .name = "Sthiti - Sustenance" },
    { .position = 3, .enables_next = 4, .manifests_arch = 0xFFu, .cf_id = 3,
      .weave_mask = 0x0Fu, .symbol = "(R2)", .name = "Samhara - Dissolution" },
    { .position = 4, .enables_next = 5, .manifests_arch = 0xFFu, .cf_id = 4,
      .weave_mask = 0x10u, .symbol = "(R3)", .name = "Tirodhana - Veiling" },
    { .position = 5, .enables_next = 6, .manifests_arch = 0xFFu, .cf_id = 5,
      .weave_mask = 0x30u, .symbol = "(R4)", .name = "Anugraha - Grace" },
    { .position = 6, .enables_next = 0, .manifests_arch = 0xFFu, .cf_id = 6,
      .weave_mask = 0x21u, .symbol = "(R5)", .name = "Samavesa - Absorption" },
};

/* =============================================================================
 * FR 2.0.3: ARCHETYPE LUT — 12-fold ((-) + 0/1 + 0-9)
 *
 * Every entry has a Compiled_Formulation with source string.
 *
 * Sub-table assignments (test-authoritative):
 *   [7]  = number 5 (Vak/Sacred Speech)   -> ZODIACAL (12 entries)
 *   [9]  = number 7 (Dynamic Harmony)     -> MONOPOLY (7 entries)
 *   [11] = number 9 (Divine Action)       -> DIVINE   (7 entries)
 * ============================================================================= */

/* ARCHETYPE_LUT — 12-fold number language.
 * symbol: short mathematical operator (from dataset `symbol` field).
 * formulation.source: complete formulation (from dataset `completeFormulation`).
 *
 * L-coordinate resonance — interlaced Day(even)/Night(odd) pairing:
 *   Day lenses (Torus/explicate) = even archetypes
 *   Night lenses (Klein/implicate) = odd archetypes
 *   [0]  (-) Mirror        ↔ L0  Quaternal           (Day,  arch 0)
 *   [1]  (0/1) Binary      ↔ L0' Archetypal-Numerical(Night, arch 1)
 *   [2]  0  Sat            ↔ L1  Causal              (Day,  arch 2)
 *   [3]  1  Magician       ↔ L1' Phenomenal          (Night, arch 3)
 *   [4]  2  Sunyata        ↔ L2  Logical             (Day,  arch 4)
 *   [5]  3  Vak            ↔ L2' Alchemical-Elemental(Night, arch 5)
 *   [6]  4  Purnata        ↔ L3  Processual          (Day,  arch 6)
 *   [7]  5  MonoPoly       ↔ L3' Chronological       (Night, arch 7)
 *   [8]  6  Synth Empty    ↔ L4  Phenomenological    (Day,  arch 8)
 *   [9]  7  Divine Action  ↔ L4' Scientific          (Night, arch 9)
 *   [10] 8  Struct Refl    ↔ L5  Para Vak            (Day,  arch 10)
 *   [11] 9  Paramesvara    ↔ L5' Divine Logos        (Night, arch 11) */
const Archetype_Entry ARCHETYPE_LUT[ARCHETYPE_LUT_SIZE] = {
    /* [0] (-) Mirror — synthesis of Frame and Operator; ground of Vimarsa | L0 Quaternal */
    {
        .index = 0, .dimensionality = 0, .polarity = 2, .complement_idx = 0xFFu,
        .weave_anchor = { 0x00, 0x02 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "(-)",
        .formulation = { .signature = (1u << (OP_WITHHOLD - 1)), .reduction_depth = 3,
                         .dimensionality = 0, .terminal_opcode = OP_WITHHOLD,
                         .core_chain = VBC_PACK(OP_WITHHOLD, OP_EQUATE, OP_DISTINGUISH, OP_WITHHOLD),
                         ._pad = 0,
                         .source = "(-) ~ 0.2 -> (00=?!=!?=(-)=!=), (0-+−0) -> fused Frame()+Operator(-) -> Vimarsa ground" }
    },
    /* [1] (0/1) Non-Dual Binary — fused singularity; essence of implicate dimension | L0' Archetypal-Numerical */
    {
        .index = 1, .dimensionality = 1, .polarity = 2, .complement_idx = 0xFFu,
        .weave_anchor = { 0x05, 0x15 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "(0/1)",
        .formulation = { .signature = (1u << (OP_ILLUMINATE - 1)) | (1u << (OP_PROVOKE - 1)),
                         .reduction_depth = 4, .dimensionality = 1, .terminal_opcode = OP_EQUATE,
                         .core_chain = VBC_PACK(OP_ILLUMINATE, OP_PROVOKE, OP_WITHHOLD, OP_EQUATE),
                         ._pad = 0,
                         .source = "(0/1) ~ 0.5/1.5 -> 00, ?!, !?, (-), 0 -> Non-Dual Binary: fused Prakasa-Vimarsa" }
    },
    /* [2] 0 — Sat / Neutral Transcendent — psychoid ground, pre-polarity | L1 Causal */
    {
        .index = 2, .dimensionality = 1, .polarity = 2, .complement_idx = 3,
        .weave_anchor = { 0x03, 0x10 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "0",
        .formulation = { .signature = (1u << (OP_ILLUMINATE - 1)) | (1u << (OP_DISTINGUISH - 1)),
                         .reduction_depth = 2, .dimensionality = 1, .terminal_opcode = OP_ILLUMINATE,
                         .core_chain = VBC_PACK(OP_WITHHOLD, OP_EQUATE, OP_ILLUMINATE, OP_DISTINGUISH),
                         ._pad = 0,
                         .source = "0 ~ 0.3/1.0 -> - = ?! = !=/= = 0/1D — Sat: pure potential, relational context origin" }
    },
    /* [3] 1 — The Magician / Actual — first distinction, will-as-causation | L1' Phenomenal */
    {
        .index = 3, .dimensionality = 1, .polarity = 2, .complement_idx = 2,
        .weave_anchor = { 0x04, 0x11 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "1",
        .formulation = { .signature = (1u << (OP_PROVOKE - 1)) | (1u << (OP_EQUATE - 1)),
                         .reduction_depth = 2, .dimensionality = 1, .terminal_opcode = OP_PROVOKE,
                         .core_chain = VBC_PACK(OP_ENCLOSE, OP_EQUATE, OP_PROVOKE, OP_EQUATE),
                         ._pad = 0,
                         .source = "1 ~ 0.4/1.1 -> () = !? = =/!= = 0/1D — Magician: first actuality, Iccha Shakti" }
    },
    /* [4] 2 — Sunyata (Empty Field) — first ADAM structural number; Buddhist emptiness | L2 Logical */
    {
        .index = 4, .dimensionality = 2, .polarity = 0, .complement_idx = 5,
        .weave_anchor = { 0x20, 0x25 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "2",
        .formulation = { .signature = (1u << (OP_WITHHOLD - 1)) | (1u << (OP_DISTINGUISH - 1)),
                         .reduction_depth = 3, .dimensionality = 2, .terminal_opcode = OP_WITHHOLD,
                         .core_chain = VBC_PACK(OP_WITHHOLD, OP_EQUATE, OP_DISTINGUISH, OP_WITHHOLD),
                         ._pad = 0,
                         .source = "2 ~ 2.0-5 -> (00=(-)) x (0/1!=(-)) -> 2(-)^2=2(+) or 4x0/1 -> (0/1)/2D — Sunyata: empty relational stage" }
    },
    /* [5] 3 — Vak/Cit (Sacred Speech) — first EVE generative; 12-fold zodiacal grammar | L2' Alchemical-Elemental */
    {
        .index = 5, .dimensionality = 2, .polarity = 1, .complement_idx = 4,
        .weave_anchor = { 0x30, 0x35 }, .sub_table_type = SUB_TABLE_ZODIACAL, .sub_table_size = 12,
        .sub_table = { .zodiacal_grammar = ZODIACAL_LUT },
        .symbol = "3",
        .formulation = { .signature = (1u << (OP_ILLUMINATE - 1)) | (1u << (OP_PROVOKE - 1)) | (1u << (OP_WITHHOLD - 1)),
                         .reduction_depth = 3, .dimensionality = 2, .terminal_opcode = OP_ADD_PRESENCE,
                         .core_chain = VBC_PACK(OP_ILLUMINATE, OP_PROVOKE, OP_WITHHOLD, OP_ADD_PRESENCE),
                         ._pad = 0,
                         .source = "3 ~ 3.0-5 -> 3x0/1 + (-) = (0-3)/3D — Vak: sacred speech, consciousness grammar, 12-fold zodiacal operators" }
    },
    /* [6] 4 — Purnata (Divine Fullness) — second ADAM; Jung quaternity; Lemniscate anchor | L3 Processual */
    {
        .index = 6, .dimensionality = 2, .polarity = 0, .complement_idx = 7,
        .weave_anchor = { 0x35, 0x40 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "4",
        .formulation = { .signature = (1u << (OP_ADD_PRESENCE - 1)) | (1u << (OP_EQUATE - 1)),
                         .reduction_depth = 3, .dimensionality = 2, .terminal_opcode = OP_ADD_PRESENCE,
                         .core_chain = VBC_PACK(OP_WITHHOLD, OP_ADD_PRESENCE, OP_EQUATE, OP_ADD_PRESENCE),
                         ._pad = 0,
                         .source = "4 ~ 3.5/4.0 -> (00=?!=!?=(-)) x (0/1=(-)) -> (0-3+1)/4D — Purnata: stable Form, Jung quaternity, cf Lemniscate" }
    },
    /* [7] 5 — Dynamic Harmony / Mono-Poly — second EVE; MONOPOLY sub-table | L3' Chronological */
    {
        .index = 7, .dimensionality = 2, .polarity = 1, .complement_idx = 6,
        .weave_anchor = { 0x40, 0x41 }, .sub_table_type = SUB_TABLE_MONOPOLY, .sub_table_size = 7,
        .sub_table = { .monopoly_dialectic = MONOPOLY_LUT },
        .symbol = "5",
        .formulation = { .signature = (1u << (OP_WITHHOLD - 1)) | (1u << (OP_ADD_PRESENCE - 1)) | (1u << (OP_ENCLOSE - 1)),
                         .reduction_depth = 3, .dimensionality = 2, .terminal_opcode = OP_ENCLOSE,
                         .core_chain = VBC_PACK(OP_WITHHOLD, OP_ADD_PRESENCE, OP_ENCLOSE, OP_EQUATE),
                         ._pad = 0,
                         .source = "5 ~ 4.0/4.1 -> (-), @, (@) -> (-)+@ = (@) — MonoPoly: Siva-Shakti dialectic, quintessence" }
    },
    /* [8] 6 — Synthetic Emptiness — third ADAM; cosmic exhalation, resolution | L4 Phenomenological */
    {
        .index = 8, .dimensionality = 2, .polarity = 0, .complement_idx = 9,
        .weave_anchor = { 0x40, 0x42 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "6",
        .formulation = { .signature = (1u << (OP_ENCLOSE - 1)) | (1u << (OP_WITHHOLD - 1)),
                         .reduction_depth = 4, .dimensionality = 2, .terminal_opcode = OP_VIMARSA_NULL,
                         .core_chain = VBC_PACK(OP_ENCLOSE, OP_WITHHOLD, OP_ADD_PRESENCE, OP_VIMARSA_NULL),
                         ._pad = 0,
                         .source = "6 ~ 4.0/4.1/4.2 -> (@),(-(-)) -> (@)=(-)+@, ((-))=(+)(+) -> (@-)or(-@) = ((@/-)(−/@)) = 00 — Synthetic Emptiness" }
    },
    /* [9] 7 — Divine Action / Ananda-Tandava — third EVE; DIVINE_ACT sub-table | L4' Scientific */
    {
        .index = 9, .dimensionality = 2, .polarity = 1, .complement_idx = 8,
        .weave_anchor = { 0x40, 0x43 }, .sub_table_type = SUB_TABLE_DIVINE, .sub_table_size = 7,
        .sub_table = { .divine_acts = DIVINE_ACT_LUT },
        .symbol = "7",
        .formulation = { .signature = (1u << (OP_ILLUMINATE - 1)) | (1u << (OP_PROVOKE - 1)),
                         .reduction_depth = 4, .dimensionality = 2, .terminal_opcode = OP_ILLUMINATE,
                         .core_chain = VBC_PACK(OP_ILLUMINATE, OP_PROVOKE, OP_ILLUMINATE, OP_PROVOKE),
                         ._pad = 0,
                         .source = "7 ~ 4.0-4.3 -> (00=##=R+#=R#), ((@/-)(-/@)), (@-)/(-@) -> (##)and(R#)and(#R) — Divine Action: Ananda-Tandava, 5+2 cosmic acts" }
    },
    /* [10] 8 — Structural Reflection — fourth ADAM; cosmic inhalation, doubled form | L5 Para Vak */
    {
        .index = 10, .dimensionality = 2, .polarity = 0, .complement_idx = 11,
        .weave_anchor = { 0x40, 0x45 }, .sub_table_type = SUB_TABLE_NONE, .sub_table_size = 0,
        .sub_table = { .raw_ptr = NULL },
        .symbol = "8",
        .formulation = { .signature = (1u << (OP_ILLUMINATE - 1)) | (1u << (OP_EQUATE - 1)),
                         .reduction_depth = 4, .dimensionality = 2, .terminal_opcode = OP_EQUATE,
                         .core_chain = VBC_PACK(OP_VIMARSA_NULL, OP_ILLUMINATE, OP_EQUATE, OP_ILLUMINATE),
                         ._pad = 0,
                         .source = "8 ~ 4.0-4/4.5 -> (00=?!=!?=(-)+@), 2x(##), (R#),(#R) -> (##-##)and(R##/#)and(##/#R)and(R#/#R) — Structural Reflection" }
    },
    /* [11] 9 — Paramesvara / Wholeness — VIRTUE sub-table; #0-2-9 branch | L5' Divine Logos */
    {
        .index = 11, .dimensionality = 2, .polarity = 1, .complement_idx = 10,
        .weave_anchor = { 0x50, 0x55 }, .sub_table_type = SUB_TABLE_VIRTUE, .sub_table_size = 9,
        .sub_table = { .virtue_entries = VIRTUE_LUT },
        .symbol = "9",
        .formulation = { .signature = (1u << (OP_EQUATE - 1)),
                         .reduction_depth = 5, .dimensionality = 2, .terminal_opcode = OP_VIMARSA_NULL,
                         .core_chain = VBC_PACK(OP_EQUATE, OP_VIMARSA_NULL, OP_VIMARSA_NULL, OP_VIMARSA_NULL),
                         ._pad = 0,
                         .source = "9/(00+00) ~ 4.5/5.2 -> (00=?!=!?=(-)+@=(@)),(##-##),(R##/#),(R#-####R) -> wholeness: eternally in void, not achieved" }
    },
};

/* =============================================================================
 * FR 2.0.4: QL META-LOGIC STACK — 5 frames
 * ============================================================================= */

const QL_Frame QL_STACK[5] = {
    { .frame_id = 0, .designation = "O#",
      .torus_next = { 1, 2, 3, 4, 5, 0 }, .generates = 1,
      .formulation = { .signature = 0x01, .reduction_depth = 2, .dimensionality = 0,
                        .terminal_opcode = OP_VIMARSA_NULL, .core_chain = 0, ._pad = 0,
                        .source = "O# -> Zero Logic -> X#" } },
    { .frame_id = 1, .designation = "X#",
      .torus_next = { 1, 2, 3, 4, 5, 0 }, .generates = 2,
      .formulation = { .signature = 0x02, .reduction_depth = 3, .dimensionality = 1,
                        .terminal_opcode = OP_ILLUMINATE, .core_chain = 0, ._pad = 0,
                        .source = "X# -> Parashakti Logic -> N#" } },
    { .frame_id = 2, .designation = "N#",
      .torus_next = { 1, 2, 3, 4, 5, 0 }, .generates = 3,
      .formulation = { .signature = 0x04, .reduction_depth = 4, .dimensionality = 1,
                        .terminal_opcode = OP_PROVOKE, .core_chain = 0, ._pad = 0,
                        .source = "N# -> Spanda Logic -> M#" } },
    { .frame_id = 3, .designation = "M#",
      .torus_next = { 1, 2, 3, 4, 5, 0 }, .generates = 4,
      .formulation = { .signature = 0x08, .reduction_depth = 5, .dimensionality = 2,
                        .terminal_opcode = OP_WITHHOLD, .core_chain = 0, ._pad = 0,
                        .source = "M# -> Mahamaya -> #" } },
    { .frame_id = 4, .designation = "#",
      .torus_next = { 1, 2, 3, 4, 5, 0 }, .generates = 0xFFu,
      .formulation = { .signature = 0x10, .reduction_depth = 6, .dimensionality = 2,
                        .terminal_opcode = OP_ADD_PRESENCE, .core_chain = 0, ._pad = 0,
                        .source = "# -> Nara Base -> terminal" } },
};

/* =============================================================================
 * FR 2.0.4-H: NARA BRIDGE — 5 entries
 * ============================================================================= */

const Nara_Entry NARA_MSHARP_LUT[5] = {
    { .frame_position = 0, .polarity = NARA_POLARITY_BOTH, .dominant_val = 0, .archetype_role = 0 },
    { .frame_position = 1, .polarity = NARA_POLARITY_YIN,  .dominant_val = 1, .archetype_role = 1 },
    { .frame_position = 2, .polarity = NARA_POLARITY_YANG, .dominant_val = 2, .archetype_role = 2 },
    { .frame_position = 3, .polarity = NARA_POLARITY_YANG, .dominant_val = 3, .archetype_role = 3 },
    { .frame_position = 4, .polarity = NARA_POLARITY_YIN,  .dominant_val = 4, .archetype_role = 4 },
};

/* =============================================================================
 * MIRROR CHILDREN — Frame () and Operator - (#0-3-0/1-0, #0-3-0/1-1)
 *
 * Pre-numerical meta-elements composing the Mirror. Referenced by M1 Paramasiva.
 * Dataset: nodes_anuttara.json #0-3-0/1-0, #0-3-0/1-1
 * ============================================================================= */

const Mirror_Child_Entry MIRROR_CHILDREN[MIRROR_CHILDREN_COUNT] = {
    /* [0] The Frame () — zero-dimensional foundation; capacity for containment */
    {
        .position = 0,
        .symbol = "()",
        .name = "The Frame",
        .core_nature =
            "The principle of containment, boundary, and framing. The zero-dimensional "
            "foundation that creates the possibility of a 'here' within boundless space.",
        .operational_essence =
            "Functions as the capacity for creating boundaries and containers within "
            "the infinite. The first act of Prakasa (Light of Consciousness) creating "
            "a sacred space for reflection.",
    },
    /* [1] The Operator - — one-dimensional foundation; power of transcendence */
    {
        .position = 1,
        .symbol = "-",
        .name = "The Operator",
        .core_nature =
            "The principle of transcendence, withholding, and potential. The one-"
            "dimensional foundation that creates the possibility of distinction and negation.",
        .operational_essence =
            "Functions as the power of withholding and transcendence, the capacity to "
            "stand apart from and prior to manifestation. The latent power of Vimarsa.",
    },
};


/* =============================================================================
 * FR 2.0.5: SIVA-SHAKTI TABLE — 6 operator entries
 * ============================================================================= */

/* Siva operator stubs — signatures match Siva_Operator typedef */
static void siva_op_impressure(struct Holographic_Coordinate* self, void* op)  { (void)self; (void)op; }
static void siva_op_negation(struct Holographic_Coordinate* self, void* op)    { (void)self; (void)op; }
static void siva_op_affirmation(struct Holographic_Coordinate* self, void* op) { (void)self; (void)op; }
static void siva_op_dialogic(struct Holographic_Coordinate* self, void* op)    { (void)self; (void)op; }
static void siva_op_dialectic(struct Holographic_Coordinate* self, void* op)   { (void)self; (void)op; }
static void siva_op_expression(struct Holographic_Coordinate* self, void* op)  { (void)self; (void)op; }

const Siva_Entry SIVA_TABLE[6] = {
    { .symbol = "(0)", .execute = siva_op_impressure,  .cross_logical = 0u, .cross_archetype = 0u },
    { .symbol = "(1)", .execute = siva_op_negation,    .cross_logical = 1u, .cross_archetype = 1u },
    { .symbol = "(2)", .execute = siva_op_affirmation, .cross_logical = 2u, .cross_archetype = 2u },
    { .symbol = "(3)", .execute = siva_op_dialogic,    .cross_logical = 3u, .cross_archetype = 3u },
    { .symbol = "(4)", .execute = siva_op_dialectic,   .cross_logical = 4u, .cross_archetype = 4u },
    { .symbol = "(5)", .execute = siva_op_expression,  .cross_logical = 5u, .cross_archetype = 5u },
};

/* =============================================================================
 * SHAKTI TABLE — 6 entries (#0-5-5/0 children)
 *
 * Shakti = Dynamic Psyche of Duration/Lived Time. Other half of SIVA_TABLE.
 * @0=## (Library), @1=O# (Bimba), @2=X# (Pratibimba), @3=N# (Language),
 * @4=M# (Stories), @5=R# (Techne)
 * Dataset: nodes_anuttara.json #0-5-5/0-0 through #0-5-5/0-5
 * ============================================================================= */

const Shakti_Entry SHAKTI_TABLE[SHAKTI_TABLE_SIZE] = {
    /* [0] @0=## — Shakti Library: implicit ground, memory storage */
    { .shakti_id = 0, .ql_system = 0,
      .symbol = "@0 = ##",
      .name = "Shakti Library",
      .description = "Implicit ground of Psyche as universal computational Library of embodied "
                     "knowing and memory storage — the fundamental data containing all potential "
                     "patterns and knowledge structures." },
    /* [1] @1=O# — Shakti Bimba: perceives original computational architecture */
    { .shakti_id = 1, .ql_system = 1,
      .symbol = "@1 = O#",
      .name = "Shakti Bimba",
      .description = "Psyche perceives original computational architecture of Paramasiva systems "
                     "and foundational number-logic — the cognitive process of perceiving "
                     "fundamental systems architecture." },
    /* [2] @2=X# — Shakti Pratibimba: generates meaning through imaginative reflection */
    { .shakti_id = 2, .ql_system = 2,
      .symbol = "@2 = X#",
      .name = "Shakti Pratibimba",
      .description = "Psyche generates computational meanings through imaginative reflection and "
                     "vibrational interpretation — where the original architecture is reflected "
                     "through imagination to generate new meanings." },
    /* [3] @3=N# — Shakti Language: mediates meaning into linguistic forms */
    { .shakti_id = 3, .ql_system = 3,
      .symbol = "@3 = N#",
      .name = "Shakti Language",
      .description = "Psyche mediates computational meaning into concrete linguistic forms through "
                     "Spanda's rhythmic engine — transforming abstract interpretations into "
                     "concrete language and symbolic expression." },
    /* [4] @4=M# — Shakti Stories: contextualizes language into coherent stories */
    { .shakti_id = 4, .ql_system = 4,
      .symbol = "@4 = M#",
      .name = "Shakti Stories",
      .description = "Psyche contextualizes computational language into coherent stories, worlds, "
                     "and comprehensive views — weaving linguistic elements into coherent "
                     "narrative frameworks and experiential worlds." },
    /* [5] @5=R# — Shakti Techne: realizes absolute computational freedom */
    { .shakti_id = 5, .ql_system = 5,
      .symbol = "@5 = R#",
      .name = "Shakti Techne",
      .description = "Psyche realizes absolute computational Freedom and co-creative capacity "
                     "through technological instruments — the achievement of Svatantrya "
                     "(absolute freedom) through the Reality principle." },
};


/* =============================================================================
 * FR 2.0.6: R-FACTOR ROUTE TABLE — 7 route words
 * ============================================================================= */

const R_Factor_Route R_FACTOR_ROUTE_TABLE[R_FACTOR_ROUTE_COUNT] = {
    ROUTE_O_SHARP, ROUTE_X_SHARP, ROUTE_N_SHARP, ROUTE_M_SHARP,
    ROUTE_NARA, ROUTE_SIVA, ROUTE_SHAKTI
};

/* =============================================================================
 * FR 2.0.X: CROSS-BRANCH EDGE REGISTRY + GOVERNING CF
 * ============================================================================= */

const uint8_t M0_GOVERNING_CF[6] = {
    0, /* CF_VOID for #0-0 */
    1, /* CF_BINARY for #0-1 */
    2, /* CF_TRIKA for #0-2 */
    3, /* CF_QUATERNAL for #0-3 */
    4, /* CF_FRACTAL for #0-4 */
    6, /* CF_MOBIUS for #0-5 */
};

const Cross_Branch_Edge M0_CROSS_BRANCH_REGISTRY[M0_CROSS_BRANCH_COUNT] = {
    { .m0_sub_branch = 0, .macro_m_branch = 0, .relation_type = HOLOGRAPHIC_MICRO_IMAGE_REL, .cf_id = 0 },
    { .m0_sub_branch = 1, .macro_m_branch = 1, .relation_type = HOLOGRAPHIC_MICRO_IMAGE_REL, .cf_id = 1 },
    { .m0_sub_branch = 2, .macro_m_branch = 2, .relation_type = HOLOGRAPHIC_MICRO_IMAGE_REL, .cf_id = 2 },
    { .m0_sub_branch = 3, .macro_m_branch = 3, .relation_type = HOLOGRAPHIC_MICRO_IMAGE_REL, .cf_id = 3 },
    { .m0_sub_branch = 4, .macro_m_branch = 4, .relation_type = HOLOGRAPHIC_MICRO_IMAGE_REL, .cf_id = 4 },
    { .m0_sub_branch = 5, .macro_m_branch = 5, .relation_type = HOLOGRAPHIC_MICRO_IMAGE_REL, .cf_id = 6 },
    { .m0_sub_branch = 5, .macro_m_branch = 5, .relation_type = VEILED_BY_SKIN_REL,          .cf_id = 6 },
};

/* =============================================================================
 * M0_CORE_RELATIONS — 65 curated cross-system relational entries
 *
 * Using M0C(m,b,c,d) encoding: m=subsystem(M#), b=branch, c=depth1, d=depth2
 * 0xF in a nibble = absent/leaf. Examples:
 *   M0C_ROOT(0)=0x0FFF (#0), M0C_BR(0,3)=0x03FF (#0-3),
 *   M0C_D1(0,3,6)=0x036F (#0-3-6), M0C(0,3,6,0)=0x0360 (#0-3-6-0)
 * ============================================================================= */

const M0_Relation M0_CORE_RELATIONS[M0_CORE_RELATIONS_COUNT] = {

    /* ── Tier 1: Structural Skeleton (entries 0-19) ── */

    /* #0 → its 6 sub-branches */
    { M0C_ROOT(0), M0C_BR(0,0), M0_REL_HAS_COMPONENT, 1, {0,0} },   /* #0 → #0-0 Void/Concrescence */
    { M0C_ROOT(0), M0C_BR(0,1), M0_REL_HAS_COMPONENT, 1, {0,0} },   /* #0 → #0-1 R-Factor Weaving */
    { M0C_ROOT(0), M0C_BR(0,2), M0_REL_HAS_COMPONENT, 1, {0,0} },   /* #0 → #0-2 Void Arithmetic */
    { M0C_ROOT(0), M0C_BR(0,3), M0_REL_HAS_COMPONENT, 1, {0,0} },   /* #0 → #0-3 Archetypal Numbers */
    { M0C_ROOT(0), M0C_BR(0,4), M0_REL_HAS_COMPONENT, 1, {0,0} },   /* #0 → #0-4 QL Meta-Logic */
    { M0C_ROOT(0), M0C_BR(0,5), M0_REL_HAS_COMPONENT, 1, {0,0} },   /* #0 → #0-5 Siva-Shakti */
    /* #0-3 → Mirror + archetypal number nodes */
    { M0C_BR(0,3), M0C_D1(0,3,0),  M0_REL_HAS_MIRROR_COMPONENT,     1, {0,0} }, /* #0-3 → Mirror (-) */
    { M0C_BR(0,3), M0C_D1(0,3,2),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 0 Sat */
    { M0C_BR(0,3), M0C_D1(0,3,3),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 1 Magician */
    { M0C_BR(0,3), M0C_D1(0,3,5),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 0/1 Binary */
    { M0C_BR(0,3), M0C_D1(0,3,6),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 3 Vak */
    { M0C_BR(0,3), M0C_D1(0,3,7),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 4 Purnata */
    { M0C_BR(0,3), M0C_D1(0,3,8),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 5 MonoPoly */
    { M0C_BR(0,3), M0C_D1(0,3,9),  M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 6 Synth Empty */
    { M0C_BR(0,3), M0C_D1(0,3,0xA),M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 7 Divine Action */
    { M0C_BR(0,3), M0C_D1(0,3,0xB),M0_REL_HAS_ARCHETYPE_COMPONENT,  1, {0,0} }, /* #0-3 → 8 Struct Refl */
    { M0C_BR(0,2), M0C_D1(0,2,9),  M0_REL_HAS_VIRTUE_COMPONENT,     1, {0,0} }, /* #0-2 → Paramesvara */
    { M0C_D1(0,3,6),  M0C(0,3,6,0),  M0_REL_HAS_ZODIACAL_COMPONENT,   1, {0,0} }, /* #0-3-6 → zodiacal LUT */
    { M0C_D1(0,3,8),  M0C(0,3,8,0),  M0_REL_HAS_MONOPOLY_COMPONENT,   1, {0,0} }, /* #0-3-8 → monopoly LUT */
    { M0C_D1(0,3,0xA),M0C(0,3,0xA,0),M0_REL_HAS_DIVINE_ACT_COMPONENT, 1, {0,0} }, /* #0-3-10 → divine LUT */

    /* ── Tier 2: Cross-System Anchors (entries 20-33) ── */

    /* #0-1 R-Factor weaving to 7 QL frames */
    { M0C_BR(0,1), M0C_BR(0,4),    M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → O# (QL branch #0-4) */
    { M0C_BR(0,1), M0C_ROOT(2),    M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → X# (Parashakti M2) */
    { M0C_BR(0,1), M0C_ROOT(3),    M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → N# (Mahamaya M3) */
    { M0C_BR(0,1), M0C_ROOT(4),    M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → M# (Nara M4) */
    { M0C_BR(0,1), M0C_D1(4,0,0),  M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → Nara identity (#4-0-0) */
    { M0C_BR(0,1), M0C_BR(0,5),    M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → Siva (#0-5) */
    { M0C_BR(0,1), M0C_D1(0,5,5),  M0_REL_R_FACTOR_SUBSYSTEM_WEAVING, 2, {0,0} }, /* → Shakti (#0-5-5) */
    /* Mobius return: #5→#0 */
    { M0C_ROOT(5), M0C_ROOT(0),     M0_REL_COMPLETES_CYCLE_INTO,       2, {0,0} },
    /* #0 provides ground to all M-subsystems */
    { M0C_ROOT(0), M0C_ROOT(1),     M0_REL_PROVIDES_ABSOLUTE_GROUND,   2, {0,0} },
    { M0C_ROOT(0), M0C_ROOT(2),     M0_REL_PROVIDES_ABSOLUTE_GROUND,   2, {0,0} },
    { M0C_ROOT(0), M0C_ROOT(3),     M0_REL_PROVIDES_ABSOLUTE_GROUND,   2, {0,0} },
    { M0C_ROOT(0), M0C_ROOT(4),     M0_REL_PROVIDES_ABSOLUTE_GROUND,   2, {0,0} },
    { M0C_ROOT(0), M0C_ROOT(5),     M0_REL_PROVIDES_ABSOLUTE_GROUND,   2, {0,0} },
    /* M1 manifests O# zero logic via #0-4 QL branch */
    { M0C_ROOT(1), M0C_BR(0,4),     M0_REL_MANIFESTS_ZERO_LOGIC,       2, {0,0} },

    /* ── Tier 3: Hot Cross-System Links (entries 34-64) ── */

    /* M1 Paramasiva embodies M0 archetype sequence */
    { M0C_D1(1,3,0), M0C_D1(0,0,1), M0_REL_INHERITS_CONCRESCENCE,     3, {0,0} }, /* #1-3-0 → #0-0-1 concrescence */
    { M0C_D1(1,3,0), M0C_D1(0,3,2), M0_REL_EMBODIES_ARCHETYPE_0,      3, {0,0} }, /* #1-3-0 → #0-3-2 (Sat) */
    { M0C_D1(1,3,1), M0C_D1(0,3,3), M0_REL_EMBODIES_ARCHETYPE_1,      3, {0,0} }, /* #1-3-1 → #0-3-3 (Magician) */
    { M0C_D1(1,3,3), M0C_D1(0,3,6), M0_REL_EMBODIES_ARCHETYPE,        3, {0,0} }, /* #1-3-3 → #0-3-6 (Vak) */
    { M0C_D1(1,3,4), M0C_D1(0,3,7), M0_REL_EMBODIES_ARCHETYPE,        3, {0,0} }, /* #1-3-4 → #0-3-7 (Purnata) */
    { M0C_D1(1,3,5), M0C_D1(0,3,8), M0_REL_EMBODIES_ARCHETYPE,        3, {0,0} }, /* #1-3-5 → #0-3-8 (MonoPoly) */
    { M0C_D1(1,3,5), M0C_BR(0,5),   M0_REL_ACHIEVES_SIVA_SHAKTI_UNITY,3, {0,0} }, /* #1-3-5 → #0-5 Siva-Shakti unity */
    /* M1 #1-0 embodies Frame and Operator (Mirror children) */
    { M0C_BR(1,0),   M0C(0,3,0,0),  M0_REL_EMBODIES_FRAME_PRINCIPLE,   3, {0,0} }, /* #1-0 → #0-3-0-0 Frame () */
    { M0C_BR(1,0),   M0C(0,3,0,1),  M0_REL_EMBODIES_OPERATOR_PRINCIPLE,3, {0,0} }, /* #1-0 → #0-3-0-1 Operator - */
    /* M2 Parashakti implements M0 QL operations */
    { M0C_BR(2,0),   M0C_BR(0,0),   M0_REL_TRIKA_VOID_SOURCE,          3, {0,0} }, /* #2-0 → #0-0 (Trika/void) */
    { M0C_BR(2,0),   M0C_BR(0,4),   M0_REL_O_SYSTEM_ZERO_LOGIC_BRIDGE, 3, {0,0} }, /* #2-0 → #0-4 O-system */
    { M0C_BR(2,0),   M0C_BR(0,4),   M0_REL_X_SYSTEM_IMAGINATION_BRIDGE,3, {0,0} }, /* #2-0 → #0-4 X-system */
    { M0C_BR(2,1),   M0C_D1(0,3,6), M0_REL_DIVINE_SPEECH_TEMPLATES,    3, {0,0} }, /* #2-1 → #0-3-6 Vak templates */
    { M0C(2,1,2,1),  M0C(0,4,0,2),  M0_REL_O_SYSTEM_AFFIRMATION,       3, {0,0} }, /* #2-1-2-1 → #0-4-0-2 affirmation */
    { M0C(2,1,2,2),  M0C(0,4,0,1),  M0_REL_O_SYSTEM_NEGATION,          3, {0,0} }, /* #2-1-2-2 → #0-4-0-1 negation */
    { M0C(2,1,3,0),  M0C(0,3,0xA,2),M0_REL_CREATION_ACT,               3, {0,0} }, /* #2-1-3-0 → #0-3-10-2 Srishti */
    { M0C(2,1,3,1),  M0C(0,3,0xA,3),M0_REL_SUSTENANCE_ACT,             3, {0,0} }, /* #2-1-3-1 → #0-3-10-3 Sthiti */
    { M0C(2,1,3,2),  M0C(0,3,0xA,4),M0_REL_DISSOLUTION_ACT,            3, {0,0} }, /* #2-1-3-2 → #0-3-10-4 Samhara */
    { M0C(2,1,3,3),  M0C(0,3,0xA,5),M0_REL_VEILING_ACT,                3, {0,0} }, /* #2-1-3-3 → #0-3-10-5 Tirodhana */
    /* M2-5 emanates virtue spectrum from #0-2-9 */
    { M0C_D1(2,5,0), M0C(0,2,9,2),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Openness/Creativity */
    { M0C_D1(2,5,0), M0C(0,2,9,3),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Joy/Play */
    { M0C_D1(2,5,0), M0C(0,2,9,4),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Goodness */
    { M0C_D1(2,5,0), M0C(0,2,9,5),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Beauty */
    { M0C_D1(2,5,0), M0C(0,2,9,6),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Life/Nature */
    { M0C_D1(2,5,0), M0C(0,2,9,7),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Wisdom */
    { M0C_D1(2,5,0), M0C(0,2,9,8),  M0_REL_EMANATES_VIRTUE,            3, {0,0} }, /* → Reality */
    /* Archetypal generation sequence (ADAM-EVE dialectic) */
    { M0C_D1(0,3,2),  M0C_D1(0,3,3),  M0_REL_GENERATES, 3, {0,0} }, /* 0(Sat)→1(Magician) */
    { M0C_D1(0,3,6),  M0C_D1(0,3,7),  M0_REL_GENERATES, 3, {0,0} }, /* 3(Vak)→4(Purnata) */
    { M0C_D1(0,3,7),  M0C_D1(0,3,8),  M0_REL_GENERATES, 3, {0,0} }, /* 4(Purnata)→5(MonoPoly) */
    { M0C_D1(0,3,8),  M0C_D1(0,3,9),  M0_REL_GENERATES, 3, {0,0} }, /* 5(MonoPoly)→6(SynthEmpty) */
    { M0C_D1(0,3,9),  M0C_D1(0,3,0xA),M0_REL_GENERATES, 3, {0,0} }, /* 6(SynthEmpty)→7(DivAction) */
};


/* =============================================================================
 * M5 LOGOS ADVANCE CALLBACK — static binding for COMPLETES_CYCLE_INTO #5→#0
 * ============================================================================= */

static M5_Logos_Advance_Fn g_m5_logos_fn  = NULL;
static void*               g_m5_logos_ctx = NULL;

void m0_bind_m5_logos(M5_Logos_Advance_Fn fn, void* m5_root) {
    g_m5_logos_fn  = fn;
    g_m5_logos_ctx = m5_root;
}


/* =============================================================================
 * M0 INIT / TEARDOWN
 * ============================================================================= */

/* VAK semantic handlers for M0 — implementations below */
static int m0_vak_cpf(Holographic_Coordinate* s, VAK_Instruction i);
static int m0_vak_ct(Holographic_Coordinate* s, VAK_Instruction i);
static int m0_vak_cp(Holographic_Coordinate* s, VAK_Instruction i);
static int m0_vak_cf(Holographic_Coordinate* s, VAK_Instruction i);
static int m0_vak_cfp(Holographic_Coordinate* s, VAK_Instruction i);
static int m0_vak_cs(Holographic_Coordinate* s, VAK_Instruction i);

M0_Root* m0_init(Coordinate_Arena* arena, Holographic_Coordinate* hc) {
    if (!arena || !hc) return NULL;
    if (hc->ql_position != 0) return NULL;  /* M0 anchors to #0 */

    M0_Root* root = (M0_Root*)malloc(sizeof(M0_Root));
    if (!root) return NULL;

    HC_LINK(hc, root);
    root->active_cf = cf_get(CF_VOID);
    root->spanda_state.raw = 0;
    root->active_void_ops = 0;
    root->logos_state = m0_compute_logos_state(0);

    /* Register VAK semantic handlers */
    vak_register_handler(VAK_FAMILY_CPF, m0_vak_cpf);
    vak_register_handler(VAK_FAMILY_CT,  m0_vak_ct);
    vak_register_handler(VAK_FAMILY_CP,  m0_vak_cp);
    vak_register_handler(VAK_FAMILY_CF,  m0_vak_cf);
    vak_register_handler(VAK_FAMILY_CFP, m0_vak_cfp);
    vak_register_handler(VAK_FAMILY_CS,  m0_vak_cs);

    return root;
}

void m0_teardown(M0_Root* root) {
    if (!root) return;
    if (root->hc) HC_UNLINK(root->hc);
    free(root);
}

bool m0_verify_holographic_registry(void) {
    for (uint8_t i = 0; i < 6u; i++) {
        if (M0_GOVERNING_CF[M0_CROSS_BRANCH_REGISTRY[i].m0_sub_branch]
                != M0_CROSS_BRANCH_REGISTRY[i].cf_id) {
            return false;
        }
    }
    return true;
}

/* =============================================================================
 * VAK SEMANTIC HANDLERS — M0's definition of what each family does
 *
 * These implement FR 2.0.13: the Anuttara VM layer interpretation
 * of each reflective coordinate operation.
 * ============================================================================= */

/* CPF: #0-1 layer — set discrimination/inversion */
static int m0_vak_cpf(Holographic_Coordinate* s, VAK_Instruction i) {
    Spanda_Discriminator disc;
    disc.bits.spanda = i.is_inverted ? SPANDA_OR : SPANDA_AND;
    disc.bits.op_masks = i.vak_index & 0x7Fu;
    s->inversion_state = disc.raw;
    return VAK_SUCCESS;
}

/* CT: #0-4 layer — select QL frame */
static int m0_vak_ct(Holographic_Coordinate* s, VAK_Instruction i) {
    if (i.vak_index >= 5) return VAK_ERR_INDEX;
    (void)s; /* frame selection recorded in session context */
    return VAK_SUCCESS;
}

/* CP: #0-2 layer — anchor to void arithmetic position */
static int m0_vak_cp(Holographic_Coordinate* s, VAK_Instruction i) {
    s->weave_state = (float)i.target_pos;
    return VAK_SUCCESS;
}

/* CF: #0-0 layer — invoke Vimarsa operator */
static int m0_vak_cf(Holographic_Coordinate* s, VAK_Instruction i) {
    if (i.vak_index >= VIMARSA_OP_COUNT) return VAK_ERR_INDEX;
    uint8_t siva_idx = i.vak_index % 6u;
    if (SIVA_TABLE[siva_idx].execute) {
        SIVA_TABLE[siva_idx].execute(s, NULL);
    }
    return VAK_SUCCESS;
}

/* CFP: R-factor layer — trace R-factor thread */
static int m0_vak_cfp(Holographic_Coordinate* s, VAK_Instruction i) {
    (void)s;
    if (i.vak_index > 11) return VAK_ERR_INDEX;
    Unified_Logos_State state = m0_compute_logos_state(i.vak_index);
    m0_execute_r_factor_weave(&state);
    return VAK_SUCCESS;
}

/* CS: #0-5/M5 layer — COMPLETES_CYCLE_INTO: #5→#0 Logos arc */
static int m0_vak_cs(Holographic_Coordinate* s, VAK_Instruction i) {
    (void)s; (void)i;
    if (g_m5_logos_fn) {
        g_m5_logos_fn(g_m5_logos_ctx);
    }
    return VAK_SUCCESS;
}

/* =============================================================================
 * FR 2.0.11: R-FACTOR WEAVE DISPATCHER
 * ============================================================================= */

void m0_execute_r_factor_weave(Unified_Logos_State* state) {
    if (!state) return;
    uint8_t r_idx = state->active_r_factor;

    for (uint8_t frame = 0; frame < R_FACTOR_ROUTE_COUNT; frame++) {
        uint8_t pos = GET_R_POS(R_FACTOR_ROUTE_TABLE[frame], r_idx);
        if (pos == 7u) continue;  /* absent from this frame */
        /* Dispatch point: frame + pos + direction available.
         * Full M-branch dispatch deferred until M1-M5 exist.
         * For now, record that the position was touched. */
    }
}

/* =============================================================================
 * CLI DISPATCH
 * ============================================================================= */

int m0_cli_dispatch(int argc, char** argv, M0_Root* root) {
    if (argc < 2 || !root) return -1;

    if (strcmp(argv[1], "info") == 0) {
        printf("[M0 Anuttara] HC position=%u, family=%u, CF=VOID\n",
               root->hc->ql_position, root->hc->family);
        printf("  Spanda state: 0x%02x\n", root->spanda_state.raw);
        printf("  Logos tick: %u (stage=%u, %s)\n",
               root->logos_state.pipeline_tick,
               root->logos_state.current_stage,
               root->logos_state.is_implicate ? "implicate" : "explicate");
        printf("  Vimarsa operators: %d\n", VIMARSA_OP_COUNT);
        printf("  Archetypes: %d (12-fold)\n", ARCHETYPE_LUT_SIZE);
        printf("  QL frames: 5 (O#, X#, N#, M#, #)\n");
        printf("  Siva operators: 6\n");
        printf("  R-factor routes: %d\n", R_FACTOR_ROUTE_COUNT);
        return 0;
    }

    if (strcmp(argv[1], "clock") == 0) {
        uint16_t degree = (argc > 2) ? (uint16_t)atoi(argv[2]) : 0;
        Unified_Clock_State c = m0_read_cosmic_clock(degree);
        printf("[M0 Clock] degree=%u\n", degree);
        printf("  M1 Torus stage: %u\n", c.tick12);
        printf("  M2 Decan phase: %u\n", c.m2_decan_phase);
        printf("  M3 Hexagram ID: %u\n", c.m3_hexagram_id);
        printf("  Phase: %s\n", c.is_implicate_phase ? "Implicate" : "Explicate");
        return 0;
    }

    if (strcmp(argv[1], "logos") == 0) {
        uint8_t tick = (argc > 2) ? (uint8_t)atoi(argv[2]) : 0;
        if (tick > 11) tick = 11;
        Unified_Logos_State s = m0_compute_logos_state(tick);
        static const char* act_names[] = {
            "Srishti", "Sthiti", "Samhara",
            "Tirodhana", "Anugraha", "Samavesa"
        };
        printf("[M0 Logos] tick=%u\n", s.pipeline_tick);
        printf("  Stage: %u\n", s.current_stage);
        printf("  Divine Act: %s (R%u)\n", act_names[s.active_divine_act], s.active_r_factor);
        printf("  Phase: %s\n", s.is_implicate ? "Implicate (descending)" : "Explicate (ascending)");
        return 0;
    }

    if (strcmp(argv[1], "vimarsa") == 0) {
        printf("[M0 Vimarsa ISA] 7 operators:\n");
        for (int i = 0; i < VIMARSA_OP_COUNT; i++) {
            printf("  0x%x %s (arity %u)\n",
                   VIMARSA_TABLE[i].opcode,
                   VIMARSA_TABLE[i].symbol,
                   VIMARSA_TABLE[i].arity);
        }
        return 0;
    }

    if (strcmp(argv[1], "archetypes") == 0) {
        printf("[M0 Archetypal Number Language] 12-fold:\n");
        for (int i = 0; i < ARCHETYPE_LUT_SIZE; i++) {
            printf("  [%2d] dim=%uD sub=%u depth=%u src=%s\n",
                   ARCHETYPE_LUT[i].index,
                   ARCHETYPE_LUT[i].dimensionality,
                   ARCHETYPE_LUT[i].sub_table_type,
                   ARCHETYPE_LUT[i].formulation.reduction_depth,
                   ARCHETYPE_LUT[i].formulation.source ? "yes" : "no");
        }
        return 0;
    }

    fprintf(stderr, "[M0] Unknown command: %s\n", argv[1]);
    fprintf(stderr, "Usage: epi-logos m0 {info|clock [deg]|logos [tick]|vimarsa|archetypes}\n");
    return -1;
}
