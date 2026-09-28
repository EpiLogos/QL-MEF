/**
 * m5.c — Epii: The Holographic Integration Layer (Implementation)
 *
 * All .rodata LUT data + API implementation for M5.
 * FR Coverage: 2.5.0 - 2.5.13
 */

#include "m5.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


/* ===================================================================
 * .RODATA: LOGOS STAGE NAMES
 * =================================================================== */

const char* const M5_LOGOS_STAGE_NAMES[6] = {
    "A-logos",   "Pro-logos", "Dia-logos",
    "Logos",     "Epi-logos", "An-a-logos"
};


/* ===================================================================
 * INTERNAL: QV search helper
 * =================================================================== */

static const M5_Quintessential_View* qv_find(const M5_Quintessential_View* arr,
                                               size_t count, uint16_t coord_id) {
    for (size_t i = 0; i < count; i++) {
        if (arr[i].coord_id == coord_id && arr[i].pithy != NULL) {
            return &arr[i];
        }
    }
    return NULL;
}

void m5_etymology_advance(M5_Etymology_FSM* fsm) {
    if (fsm->stage < ETYM_STAGE_MOBIUS_WRITEBACK) {
        fsm->stage = (M5_Etymology_Stage)(fsm->stage + 1);
    }
    fsm->write_back_ready = (fsm->stage == ETYM_STAGE_MOBIUS_WRITEBACK);
}

void m5_hold_paradox(M5_Paradox_Hold* ph, uint64_t thesis,
                     uint64_t antithesis, uint8_t current_tick) {
    ph->thesis_mask = thesis;
    ph->antithesis_mask = antithesis;
    ph->hold_since_tick = current_tick;
    ph->resolution_stage = ANALOGOS;
    ph->holding = true;
    ph->resolved = false;
}

uint64_t m5_resolve_paradox(M5_Paradox_Hold* ph) {
    if (!ph->holding) return 0;
    uint64_t synthesis = ph->thesis_mask ^ ph->antithesis_mask;
    ph->resolved = (synthesis != 0);
    ph->holding = !ph->resolved;
    return synthesis;
}

M5_Mobius_Return_Target m5_mobius_return_target(
    M4_Epii_Integration* epii,
    M4_Identity_Matrix* identity,
    M5_ContemplateSessionCloseFn contemplate_session_close,
    void* user_data) {
    M5_Mobius_Return_Target target;
    target.magic = M5_MOBIUS_RETURN_TARGET_MAGIC;
    target.size = (uint32_t)sizeof(M5_Mobius_Return_Target);
    target.epii = epii;
    target.identity = identity;
    target.contemplate_session_close = contemplate_session_close;
    target.user_data = user_data;
    return target;
}

static uint64_t m5_mix_delta_byte(uint64_t acc, uint8_t byte) {
    acc ^= (uint64_t)byte;
    acc *= 1099511628211ULL;
    return acc;
}

static uint64_t m5_mix_delta_u32(uint64_t acc, uint32_t value) {
    for (uint8_t i = 0; i < 4; i++) {
        acc = m5_mix_delta_byte(acc, (uint8_t)((value >> (i * 8)) & 0xffu));
    }
    return acc;
}

static uint64_t m5_mix_delta_u64(uint64_t acc, uint64_t value) {
    for (uint8_t i = 0; i < 8; i++) {
        acc = m5_mix_delta_byte(acc, (uint8_t)((value >> (i * 8)) & 0xffu));
    }
    return acc;
}

static uint64_t m5_mix_delta_text(uint64_t acc, const char* text) {
    if (!text) return m5_mix_delta_byte(acc, 0);
    while (*text) {
        acc = m5_mix_delta_byte(acc, (uint8_t)*text);
        text++;
    }
    return acc;
}

static uint16_t m5_scaled_quaternion_diff(double close_component,
                                           double open_component) {
    double diff = close_component - open_component;
    if (diff < 0.0) diff = -diff;
    return (uint16_t)(diff * 4096.0);
}

static bool m5_is_mobius_target(const void* maybe_target) {
    if (!maybe_target) return false;
    const M5_Mobius_Return_Target* target =
        (const M5_Mobius_Return_Target*)maybe_target;
    return target->magic == M5_MOBIUS_RETURN_TARGET_MAGIC &&
           target->size >= sizeof(M5_Mobius_Return_Target) &&
           target->epii != NULL &&
           target->identity != NULL;
}

typedef struct {
    M5_Q_BioQuaternion_Tick q_ticks[2];
    M5_Codon_Trace         codons[2];
    M5_Vak_Profile_Pair    vak_pair;
    M5_Skeleton_Event      skeleton_events[2];
} M5_ContemplationScratch;

static void m5_compose_contemplation_object(const M5_Root* root,
                                            const M5_Mobius_Return_Target* target,
                                            M5_ContemplationScratch* scratch,
                                            M5_ContemplationObject* obj) {
    static const char* session_id = "m5-close-current-session";
    static const char* codon_labels[2] = {
        "m5-close-open-resonance",
        "m5-close-return-resonance"
    };
    static const char* codon_routes[2] = {
        "m3-mahamaya/codon/open",
        "m3-mahamaya/codon/close"
    };
    static const char* syntax_prompts[M5_CONTEMPLATION_SYNTAX_SEED_COUNT] = {
        "speech-3",
        "relationship-5",
        "action-7",
        "completion-9"
    };

    memset(obj, 0, sizeof(*obj));
    memset(scratch, 0, sizeof(*scratch));

    uint64_t hash_head = 0;
    if (target && target->identity) {
        memcpy(&hash_head, target->identity->quintessence_hash, 8);
    }

    scratch->q_ticks[0].tick = 0;
    scratch->q_ticks[0].w = 1.0;
    scratch->q_ticks[0].x = (double)(uint8_t)(hash_head & 0xffu) / 255.0;
    scratch->q_ticks[0].y = (double)(uint8_t)((hash_head >> 8) & 0xffu) / 255.0;
    scratch->q_ticks[0].z = (double)(uint8_t)((hash_head >> 16) & 0xffu) / 255.0;

    scratch->q_ticks[1].tick = root ? root->logos.pipeline_tick : 0;
    scratch->q_ticks[1].w = 1.0;
    scratch->q_ticks[1].x = (double)(uint8_t)((hash_head >> 24) & 0xffu) / 255.0;
    scratch->q_ticks[1].y = (double)(uint8_t)((hash_head >> 32) & 0xffu) / 255.0;
    scratch->q_ticks[1].z = (double)(uint8_t)((hash_head >> 40) & 0xffu) / 255.0;

    scratch->codons[0].codon = root ? (uint8_t)(root->logos.pipeline_tick % 64u) : 0;
    scratch->codons[0].label = codon_labels[0];
    scratch->codons[0].m3_route = codon_routes[0];
    scratch->codons[1].codon = (uint8_t)((hash_head ^ (hash_head >> 6)) & 0x3fu);
    scratch->codons[1].label = codon_labels[1];
    scratch->codons[1].m3_route = codon_routes[1];

    scratch->vak_pair.dispatch = "Pi+Anima+Epii";
    scratch->vak_pair.profile_generation = root ?
        (uint32_t)(root->theory.session_depth + root->agents.anima_count +
                   root->agents.aletheia_count) : 0;
    scratch->vak_pair.profile_anchor = "m5://current-session/profile";
    scratch->vak_pair.acr_route = "acr://dispatch/session-close";

    scratch->skeleton_events[0].name = "contemplation-object-composed";
    scratch->skeleton_events[1].name = "mobius-return-ready";

    obj->session_id = session_id;
    obj->kairos_at_open.realtime.planet_degrees[0] =
        (uint16_t)(hash_head % 360u);
    obj->kairos_at_close.realtime.planet_degrees[0] =
        (uint16_t)((hash_head + (root ? root->logos.pipeline_tick : 0u)) % 360u);
    obj->tarot_psyche_anchor.drawn[0] = (uint8_t)(hash_head % 78u);
    obj->tarot_psyche_anchor.draw_count = 1;
    obj->q_composed_trajectory = scratch->q_ticks;
    obj->q_composed_trajectory_count = 2;
    obj->codon_trace = scratch->codons;
    obj->codon_trace_count = 2;
    obj->vak_profile_pairs = &scratch->vak_pair;
    obj->vak_profile_pair_count = 1;
    obj->m1_charge_state.pp = root ? (uint32_t)(root->logos.archetype_charge[0] & 0xffu) : 0;
    obj->m1_charge_state.nn = root ? (uint32_t)(root->logos.archetype_charge[1] & 0xffu) : 0;
    obj->m1_charge_state.np = root ? (uint32_t)(root->logos.archetype_charge[2] & 0xffu) : 0;
    obj->m1_charge_state.pn = root ? (uint32_t)(root->logos.archetype_charge[3] & 0xffu) : 0;
    obj->m1_charge_state.outer =
        (obj->m1_charge_state.pp + obj->m1_charge_state.nn +
         obj->m1_charge_state.np + obj->m1_charge_state.pn) / 4u;
    obj->m1_2_skeleton_events_fired = scratch->skeleton_events;
    obj->m1_2_skeleton_event_count = 2;

    for (uint8_t i = 0; i < M5_CONTEMPLATION_SYNTAX_SEED_COUNT; i++) {
        obj->four_syntax_compliance_seeds[i].prompt = syntax_prompts[i];
    }
}

uint64_t contemplate_session_close(const M5_ContemplationObject* obj) {
    if (!obj) return 0;

    uint64_t delta = 1469598103934665603ULL;
    delta = m5_mix_delta_text(delta, obj->session_id);
    delta = m5_mix_delta_u32(delta, obj->kairos_at_open.realtime.planet_degrees[0]);
    delta = m5_mix_delta_u32(delta, obj->kairos_at_close.realtime.planet_degrees[0]);
    delta = m5_mix_delta_u32(delta, obj->tarot_psyche_anchor.drawn[0]);

    if (obj->q_composed_trajectory_count >= 2 && obj->q_composed_trajectory) {
        const M5_Q_BioQuaternion_Tick* open = &obj->q_composed_trajectory[0];
        const M5_Q_BioQuaternion_Tick* close =
            &obj->q_composed_trajectory[obj->q_composed_trajectory_count - 1];
        uint64_t quaternionic_difference =
            ((uint64_t)m5_scaled_quaternion_diff(close->w, open->w) << 48) |
            ((uint64_t)m5_scaled_quaternion_diff(close->x, open->x) << 32) |
            ((uint64_t)m5_scaled_quaternion_diff(close->y, open->y) << 16) |
            (uint64_t)m5_scaled_quaternion_diff(close->z, open->z);
        delta = m5_mix_delta_u64(delta, quaternionic_difference);
    }

    for (uint32_t i = 0; i < obj->codon_trace_count; i++) {
        delta = m5_mix_delta_byte(delta, obj->codon_trace[i].codon);
        delta = m5_mix_delta_text(delta, obj->codon_trace[i].label);
        delta = m5_mix_delta_text(delta, obj->codon_trace[i].m3_route);
    }

    for (uint32_t i = 0; i < obj->vak_profile_pair_count; i++) {
        delta = m5_mix_delta_text(delta, obj->vak_profile_pairs[i].dispatch);
        delta = m5_mix_delta_u32(delta, obj->vak_profile_pairs[i].profile_generation);
        delta = m5_mix_delta_text(delta, obj->vak_profile_pairs[i].profile_anchor);
        delta = m5_mix_delta_text(delta, obj->vak_profile_pairs[i].acr_route);
    }

    delta = m5_mix_delta_u32(delta, obj->m1_charge_state.pp);
    delta = m5_mix_delta_u32(delta, obj->m1_charge_state.nn);
    delta = m5_mix_delta_u32(delta, obj->m1_charge_state.np);
    delta = m5_mix_delta_u32(delta, obj->m1_charge_state.pn);
    delta = m5_mix_delta_u32(delta, obj->m1_charge_state.outer);

    for (uint8_t i = 0; i < M5_CONTEMPLATION_SYNTAX_SEED_COUNT; i++) {
        delta = m5_mix_delta_text(delta, obj->four_syntax_compliance_seeds[i].prompt);
    }

    return delta;
}


/* ===================================================================
 * API: m5_init — Allocate and HC-link M5_Root
 * =================================================================== */

M5_Root* m5_init(Coordinate_Arena* arena, Holographic_Coordinate* hc) {
    if (!arena || !hc) return NULL;
    if (hc->ql_position != 5) return NULL;  /* Must be #5 */

    M5_Root* root = calloc(1, sizeof(M5_Root));
    if (!root) return NULL;

    HC_LINK(hc, root);
    root->active_cf = cf_get(CF_MOBIUS);

    return root;
}


/* ===================================================================
 * API: m5_teardown — Release M5_Root heap state
 * =================================================================== */

void m5_teardown(M5_Root* root) {
    if (!root) return;

    free(root->agents.anima_roster);
    free(root->agents.aletheia_roster);

    if (root->hc) {
        HC_UNLINK(root->hc);
    }

    free(root);
}


/* ===================================================================
 * API: m5_advance_logos — Advance Logos FSM by one tick
 * =================================================================== */

Unified_Logos_State m5_advance_logos(M5_Root* root) {
    uint8_t tick = root->logos.pipeline_tick;
    Unified_Logos_State state = m0_compute_logos_state(tick);

    /* Advance tick (wraps at 12) */
    root->logos.pipeline_tick = (uint8_t)((tick + 1) % 12);

    return state;
}


/* ===================================================================
 * API: m5_execute_mobius_return — The Sacred Violation
 *
 * Casts away const on M0/M4 ground state at tick 11 ONLY.
 * This is philosophically mandated (Spanda) and FSM-guarded.
 * m0_ground may be the legacy uint64_t field or M5_Mobius_Return_Target.
 * =================================================================== */

int m5_execute_mobius_return(M5_Root* root, void* m0_ground) {
    if (!root || !m0_ground) return -1;

    /* Sacred Violation is ONLY authorized at tick 11 (descending ALOGOS) */
    if (root->logos.pipeline_tick != 11) return -1;

    if (m5_is_mobius_target(m0_ground)) {
        M5_Mobius_Return_Target* target = (M5_Mobius_Return_Target*)m0_ground;
        M5_ContemplationScratch scratch;
        M5_ContemplationObject obj;
        m5_compose_contemplation_object(root, target, &scratch, &obj);

        uint64_t wisdom_delta = target->contemplate_session_close ?
            target->contemplate_session_close(&obj, target->user_data) :
            contemplate_session_close(&obj);
        target->epii->wisdom_delta = wisdom_delta;
        m4_mobius_return(target->epii, target->identity);
    } else {
        uint64_t* ground = (uint64_t*)m0_ground;
        *ground ^= root->logos.archetype_charge[5];
    }

    /* Reset for next cycle */
    root->logos.pipeline_tick = 0;

    return 0;
}


/* ===================================================================
 * API: m5_lookup — Quintessential View Self-API
 * =================================================================== */

const char* m5_lookup(const M5_Root* root, uint16_t coord_id, uint8_t granularity) {
    if (!root) return NULL;

    const M5_Quintessential_View* qv = NULL;
    uint8_t fam = M5_COORD_FAMILY(coord_id);

    switch (fam) {
        case FAMILY_M:
            qv = qv_find(root->identity.m_views, 6, coord_id);
            if (!qv) qv = qv_find(root->identity.m_prime, 6, coord_id);
            break;
        case FAMILY_L:
            qv = qv_find(root->theory.l_views, 6, coord_id);
            if (!qv) qv = qv_find(root->theory.l_prime, 6, coord_id);
            break;
        case FAMILY_P:
            qv = qv_find(root->theory.p_views, 6, coord_id);
            if (!qv) qv = qv_find(root->theory.p_prime, 6, coord_id);
            break;
        case FAMILY_S:
            qv = qv_find(root->stack.s_views, 6, coord_id);
            if (!qv) qv = qv_find(root->stack.s_prime, 6, coord_id);
            break;
        case FAMILY_T:
            qv = qv_find(root->logos.t_views, 6, coord_id);
            if (!qv) qv = qv_find(root->logos.t_prime, 6, coord_id);
            break;
        case FAMILY_C:
            qv = qv_find(root->logos.c_views, 6, coord_id);
            if (!qv) qv = qv_find(root->logos.c_prime, 6, coord_id);
            break;
        default:
            return NULL;
    }

    if (!qv) return NULL;

    switch (granularity) {
        case M5_GRAN_PITHY:    return qv->pithy;
        case M5_GRAN_OBSIDIAN: return (qv->register_count > 1) ? qv->registers[1] : NULL;
        case M5_GRAN_NEO4J:    return (qv->register_count > 2) ? qv->registers[2] : NULL;
        default:               return qv->pithy;
    }
}


/* ===================================================================
 * API: m5_verify — Boot-time .rodata verification
 * =================================================================== */

bool m5_verify(void) {
    if (GET_PTR(Psychoid_5.c) != &Psychoid_0) return false;

    const Holographic_Coordinate* cf_mob = cf_get(CF_MOBIUS);
    if (!cf_mob) return false;
    if (GET_PTR(cf_mob->cf) != &Psychoid_4) return false;

    if (M5_LOGOS_STAGE_NAMES[0] == NULL) return false;
    if (M5_LOGOS_STAGE_NAMES[5] == NULL) return false;

    return true;
}


/* ===================================================================
 * API: m5_cli_dispatch — CLI entry point
 * =================================================================== */

int m5_cli_dispatch(int argc, char** argv, M5_Root* root) {
    if (!root) return -1;

    if (argc < 2) {
        printf("M5 (Epii) — Holographic Integration Layer\n");
        printf("  info              — HC anchoring + status\n");
        printf("  lookup <fam> <pos>— Quintessential view\n");
        printf("  logos tick        — Current FSM state\n");
        printf("  logos advance     — Advance one tick\n");
        printf("  agents list       — Agent rosters\n");
        printf("  stack             — S + S' status\n");
        printf("  theory            — L+P topology\n");
        return 0;
    }

    if (strcmp(argv[1], "info") == 0) {
        printf("[M5] Epii — Holographic Integration Layer\n");
        printf("  HC position: %u, family: %u\n",
               root->hc->ql_position, root->hc->family);
        printf("  CF: MOBIUS (5/0)\n");
        printf("  Logos tick: %u/11\n", root->logos.pipeline_tick);
        printf("  Theory sessions: %u\n", root->theory.session_depth);
        printf("  Anima agents: %u, Aletheia agents: %u\n",
               root->agents.anima_count, root->agents.aletheia_count);
        return 0;
    }

    if (strcmp(argv[1], "logos") == 0) {
        if (argc < 3) {
            printf("[M5] logos: specify 'tick' or 'advance'\n");
            return -1;
        }
        if (strcmp(argv[2], "tick") == 0) {
            Unified_Logos_State s = m0_compute_logos_state(root->logos.pipeline_tick);
            printf("[M5] Logos tick %u: %s (%s)\n",
                   s.pipeline_tick,
                   M5_LOGOS_STAGE_NAMES[s.current_stage],
                   s.is_implicate ? "descending" : "ascending");
            return 0;
        }
        if (strcmp(argv[2], "advance") == 0) {
            Unified_Logos_State s = m5_advance_logos(root);
            printf("[M5] Advanced from tick %u: %s (%s) -> tick %u\n",
                   s.pipeline_tick,
                   M5_LOGOS_STAGE_NAMES[s.current_stage],
                   s.is_implicate ? "descending" : "ascending",
                   root->logos.pipeline_tick);
            return 0;
        }
        printf("[M5] Unknown logos command: %s\n", argv[2]);
        return -1;
    }

    if (strcmp(argv[1], "agents") == 0) {
        if (argc >= 3 && strcmp(argv[2], "list") == 0) {
            printf("[M5] Anima roster (%u agents):\n", root->agents.anima_count);
            for (uint8_t i = 0; i < root->agents.anima_count; i++) {
                printf("  [%u] %s (cap=0x%02x, tools=0x%02x, %s)\n",
                       root->agents.anima_roster[i].id,
                       root->agents.anima_roster[i].name,
                       root->agents.anima_roster[i].capability_flags,
                       root->agents.anima_roster[i].tool_flags,
                       root->agents.anima_roster[i].active ? "active" : "disabled");
            }
            printf("[M5] Aletheia roster (%u agents):\n", root->agents.aletheia_count);
            for (uint8_t i = 0; i < root->agents.aletheia_count; i++) {
                printf("  [%u] %s (cap=0x%02x, tools=0x%02x, %s)\n",
                       root->agents.aletheia_roster[i].id,
                       root->agents.aletheia_roster[i].name,
                       root->agents.aletheia_roster[i].capability_flags,
                       root->agents.aletheia_roster[i].tool_flags,
                       root->agents.aletheia_roster[i].active ? "active" : "disabled");
            }
            return 0;
        }
        printf("[M5] agents: specify 'list'\n");
        return -1;
    }

    if (strcmp(argv[1], "stack") == 0) {
        printf("[M5] Stack (S + S'):\n");
        for (int i = 0; i < 6; i++) {
            printf("  S%d: %s\n", i,
                   root->stack.s_views[i].pithy ? root->stack.s_views[i].pithy : "(empty)");
            printf("  S%d': %s\n", i,
                   root->stack.s_prime[i].pithy ? root->stack.s_prime[i].pithy : "(empty)");
        }
        return 0;
    }

    if (strcmp(argv[1], "theory") == 0) {
        printf("[M5] Theory Topology (L+P+L'+P'):\n");
        printf("  Session depth: %u\n", root->theory.session_depth);
        for (int i = 0; i < 6; i++) {
            printf("  L%d: %s\n", i,
                   root->theory.l_views[i].pithy ? root->theory.l_views[i].pithy : "(empty)");
        }
        for (int i = 0; i < 6; i++) {
            printf("  P%d: %s\n", i,
                   root->theory.p_views[i].pithy ? root->theory.p_views[i].pithy : "(empty)");
        }
        return 0;
    }

    if (strcmp(argv[1], "lookup") == 0) {
        if (argc < 4) {
            printf("[M5] lookup: specify <family_num> <position>\n");
            return -1;
        }
        uint8_t fam = (uint8_t)atoi(argv[2]);
        uint8_t pos = (uint8_t)atoi(argv[3]);
        uint16_t cid = M5_COORD_ID(fam, pos, 0);
        const char* view = m5_lookup(root, cid, M5_GRAN_PITHY);
        if (view) {
            printf("[M5] %s\n", view);
        } else {
            printf("[M5] No quintessential view for family=%u pos=%u\n", fam, pos);
        }
        return 0;
    }

    printf("[M5] Unknown command: %s\n", argv[1]);
    return -1;
}
