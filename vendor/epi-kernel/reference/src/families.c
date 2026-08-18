/**
 * families.c — Family Instantiation
 *
 * Instantiates all 36 family coordinates (6 families × 6 positions)
 * into the arena and cross-links them.
 */

#include "ontology.h"
#include "psychoid_numbers.h"
#include "arena.h"
#include <string.h>

int families_init(Coordinate_Arena* arena, Holographic_Coordinate* mirrors[6]) {
    if (!arena || !mirrors) return -1;

    /* For each family (C,P,L,S,T,M — archetype resonance order), create 6 coordinates */
    Coordinate_Family families[] = {
        FAMILY_C, FAMILY_P, FAMILY_L, FAMILY_S, FAMILY_T, FAMILY_M
    };

    const Holographic_Coordinate* raw_psychoids[] = {
        &Psychoid_0, &Psychoid_1, &Psychoid_2,
        &Psychoid_3, &Psychoid_4, &Psychoid_5
    };

    for (int f = 0; f < 6; f++) {
        for (int pos = 0; pos < 6; pos++) {
            Holographic_Coordinate* coord = arena_alloc(arena);
            if (!coord) return -1;

            coord->ql_position = (uint8_t)pos;
            coord->family = (uint8_t)families[f];
            coord->inversion_state = 0;

            /* Encode weave_state as pos.family — e.g. C3 = 3.6, P0 = 0.1 */
            coord->weave_state = (float)pos + (float)families[f] * 0.1f;

            /* Bedrock pointer: semantic_embedding points to .rodata psychoid */
            coord->semantic_embedding = (float*)raw_psychoids[pos];

            /* Link to mirror via .c */
            coord->c = mirrors[pos];

            /* Copy invoke_process from the raw psychoid */
            coord->invoke_process = raw_psychoids[pos]->invoke_process;

            /* cf: ALL family coordinates anchor to the Lemniscate */
            coord->cf = (Holographic_Coordinate*)&Psychoid_4;

            /* Set topological modality — inner reality of P/P' families.
             * For now, inversion_state is always 0 at init (PRATIBIMBA copies;
             * P' coordinates are created separately when # is applied).
             * P-family gets the canonical topological modes; all others TOPO_TORUS
             * by default (inherit by resonance through the P crosslink). */
            if (families[f] == FAMILY_P) {
                if (pos == 4) {
                    /* #4 is the Lemniscate anchor: genus-2 nesting, cf-chain entry,
                     * C' (reflective coords: cpf/ct/cp/cf/cfp/cs) gateway.
                     * Already embedded in pointer logic — made explicit here. */
                    SET_TOPO_MODE(coord, TOPO_MODE_LEMNISCATE);
                } else {
                    /* P outward phase = Torus single-cover */
                    SET_TOPO_MODE(coord, TOPO_MODE_TORUS);
                }
            } else if (families[f] == FAMILY_C && (pos == 0 || pos == 5)) {
                /* C0 (Bimba) and C5 (Pratibimba): the Spanda seed boundary poles.
                 * These are the categorical registration of the (0/1) seed:
                 * C0 = source ground, C5 = reflection synthesis.
                 * P0/P5 are their positional counterparts. Together: the totalizing
                 * encapsulation of the Spanda oscillation across both registers. */
                SET_TOPO_MODE(coord, TOPO_MODE_ZERO_SPHERE);
            } else {
                /* All other coordinates default to Torus modality.
                 * When # (inversion) is applied to create P' coordinates,
                 * SET_TOPO_MODE(coord, TOPO_MODE_KLEIN) must be called there. */
                SET_TOPO_MODE(coord, TOPO_MODE_TORUS);
            }
        }
    }

    return 0;
}

/* Cross-link base pointers between families.
 * arena must already have mirrors at [0..5] and families at [6..41].
 * Family layout: C=[6..11], P=[12..17], L=[18..23], S=[24..29], T=[30..35], M=[36..41] */
int families_crosslink(Coordinate_Arena* arena) {
    if (!arena || arena->count < 42) return -1;

    Holographic_Coordinate* base = arena->slots;
    int c_offset = 6, p_offset = 12, l_offset = 18;
    int s_offset = 24, t_offset = 30, m_offset = 36;

    /* Link every coord (mirrors + families) to its same-position peer in each family.
     * Apply TAG_RELATION: source determines the operator flag.
     * #4 and identification edges get NESTING; all others get BRANCHING. */
    for (uint32_t i = 0; i < arena->count; i++) {
        uint8_t pos = base[i].ql_position;
        if (pos > 5) continue;
        base[i].c = TAG_RELATION(&base[i], &base[c_offset + pos]);
        base[i].p = TAG_RELATION(&base[i], &base[p_offset + pos]);
        base[i].l = TAG_RELATION(&base[i], &base[l_offset + pos]);
        base[i].s = TAG_RELATION(&base[i], &base[s_offset + pos]);
        base[i].t = TAG_RELATION(&base[i], &base[t_offset + pos]);
        base[i].m = TAG_RELATION(&base[i], &base[m_offset + pos]);
    }

    return 0;
}

/* Wire reflective coordinates (cpf, ct, cp, cf, cfp, cs) on all arena slots.
 * cf: position 4 coordinates get self-reference (Lemniscate).
 * cs: points to the position-5 coordinate in same family (system-level direction). */
int families_wire_reflective(Coordinate_Arena* arena) {
    if (!arena || arena->count < 42) return -1;

    Holographic_Coordinate* base = arena->slots;

    for (uint32_t i = 0; i < arena->count; i++) {
        uint8_t pos = base[i].ql_position;
        if (pos > 5) continue;

        /* cf: Lemniscate anchor — position 4 self-references */
        if (pos == 4) {
            base[i].cf = &base[i];
        } else if (pos == 3) {
            /* #3 reaches into #4 via cf */
            int block_start;
            if (i < 6) {
                block_start = 0;
            } else {
                block_start = (int)(((i - 6) / 6) * 6 + 6);
            }
            base[i].cf = &base[block_start + 4];
        }

        /* cs: Context-System — points to position-5 in same block */
        {
            int block_start;
            if (i < 6) {
                block_start = 0;
            } else {
                block_start = (int)(((i - 6) / 6) * 6 + 6);
            }
            base[i].cs = &base[block_start + 5];
        }
    }

    return 0;
}
