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
 * Casts away const on M0 ground state at tick 11 ONLY.
 * This is philosophically mandated (Spanda) and FSM-guarded.
 * m0_ground points to the uint64_t field to XOR-enrich.
 * =================================================================== */

int m5_execute_mobius_return(M5_Root* root, void* m0_ground) {
    if (!root || !m0_ground) return -1;

    /* Sacred Violation is ONLY authorized at tick 11 (descending ALOGOS) */
    if (root->logos.pipeline_tick != 11) return -1;

    uint64_t* ground = (uint64_t*)m0_ground;
    *ground ^= root->logos.archetype_charge[5];

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
