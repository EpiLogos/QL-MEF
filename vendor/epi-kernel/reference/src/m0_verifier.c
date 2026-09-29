/**
 * m0_verifier.c — Verifier at position 0'/Anuttara.
 *
 * Substrate-context citation: the symbolic-coordinate-string vocabulary is
 * bound to Body/S/S5/epii-operational-capacities/
 * m5-prime-epii-on-anuttara-language-development.md as the canon-corpus
 * source named by DR-MP-1/DR-MP-3. The live Seed mirror of that document
 * carries the Anuttara language-development vocabulary; this module keeps the
 * kernel-side question namespace aligned to it. The registry it validates
 * membership against is anuttara_language.c (109 + 19 = 128 reach).
 *
 * Structural identity (Tranche 8.9 / DR-MP-1): the Verifier IS the 0' face
 * of the unified act. The report composes alongside kernel_energy_evaluate's
 * EnergyDecomposition; it never replaces it.
 */

#include "m0_verifier.h"
#include "anuttara_language.h"
#include "m0_calculus.h"
#include <stdio.h>
#include <string.h>

static float clamp01(float value) {
    if (value < 0.0f) return 0.0f;
    if (value > 1.0f) return 1.0f;
    return value;
}

static uint16_t committed_mask(const KernelState* state) {
    uint16_t mask = state->committed_virtue_mask & 0x01FFu;
    return mask == 0u ? 0x01FFu : mask;
}

static uint8_t syntax_layer_for_virtue(uint8_t virtue_index) {
    if (virtue_index <= 2u) return 3u;
    if (virtue_index <= 4u) return 5u;
    if (virtue_index <= 7u) return 7u;
    return 9u;
}

static char syntax_code_for_layer(uint8_t layer) {
    switch (layer) {
        case 3u: return 'S';
        case 5u: return 'R';
        case 7u: return 'A';
        case 9u: return 'A';
        default: return 'X';
    }
}

static uint8_t virtue_r_factor(uint8_t virtue_index) {
    if (virtue_index < M0_VERIFIER_VIRTUE_COUNT &&
        VIRTUE_LUT[virtue_index].r_factor != 0xFFu) {
        return VIRTUE_LUT[virtue_index].r_factor;
    }
    return virtue_index;
}

/* Law family for a virtue-level exception: the principle triad grounds
 * through identity chains (Law 3); the completion virtue 5R/Reality is the
 * 8+1=9 wholeness law (Law 5); the conjugates 0R..4R fail as chirality —
 * an act not witnessed in the opposite hand (Law 1). */
static uint8_t law_for_virtue(uint8_t virtue_index) {
    if (virtue_index <= 2u) return (uint8_t)M0_ANUTTARA_LAW_IDENTITY_CHAIN;
    if (virtue_index == 8u) return (uint8_t)M0_ANUTTARA_LAW_EIGHT_PLUS_ONE;
    return (uint8_t)M0_ANUTTARA_LAW_CHIRALITY;
}

static void add_typed_query(
    M0VerifierReport* out,
    uint8_t law_family,
    const char* query_kind,
    const char* symbolic
) {
    if (out->typed_query_count >= M0_VERIFIER_MAX_TYPED_QUERIES) return;
    M0TypedQuery* q = &out->typed_queries[out->typed_query_count];
    q->law_family = law_family;
    (void)snprintf(q->query_kind, M0_VERIFIER_QUERY_KIND_MAX, "%s", query_kind);
    (void)snprintf(
        q->symbolic_coordinate_string,
        M0_VERIFIER_COORDINATE_MAX,
        "%s",
        symbolic
    );
    out->typed_query_count++;
}

static void add_constraint(
    M0VerifierReport* out,
    uint8_t law_family,
    const char* query_kind,
    const char* symbolic
) {
    if (out->unsatisfied_count < M0_VERIFIER_MAX_UNSATISFIED) {
        (void)snprintf(
            out->unsatisfied_constraints[out->unsatisfied_count],
            M0_VERIFIER_COORDINATE_MAX,
            "%s",
            symbolic
        );
        out->unsatisfied_count++;
    }
    add_typed_query(out, law_family, query_kind, symbolic);
}

static void add_virtue_constraint(
    M0VerifierReport* out,
    uint8_t virtue_index,
    const char* state_marker
) {
    const uint8_t layer = syntax_layer_for_virtue(virtue_index);
    char symbolic[M0_VERIFIER_COORDINATE_MAX];
    (void)snprintf(
        symbolic,
        sizeof(symbolic),
        "#R%u-0/1/%c-T%u-%s?",
        (unsigned)virtue_r_factor(virtue_index),
        syntax_code_for_layer(layer),
        (unsigned)layer,
        state_marker
    );
    add_constraint(out, law_for_virtue(virtue_index), "unwitnessed-virtue", symbolic);
}

static void add_backing_ref(M0VerifierReport* out, const char* coordinate) {
    for (uint16_t i = 0; i < out->backing_chain_count; i++) {
        if (strcmp(out->backing_chain[i].coordinate, coordinate) == 0) return;
    }
    if (out->backing_chain_count >= M0_VERIFIER_MAX_BACKING_CHAIN) return;
    M0CoordinateRef* ref = &out->backing_chain[out->backing_chain_count];
    (void)snprintf(ref->coordinate, M0_VERIFIER_COORDINATE_MAX, "%s", coordinate);
    const AnuttaraLanguageEntry* entry = anuttara_language_lookup(coordinate);
    ref->packed = entry ? entry->packed : 0xFFFFu;
    out->backing_chain_count++;
}

/* Walk one virtue's identity chain into the report-level aggregate trace.
 * Principle virtues ground unbounded; conjugates stop at the kernel default
 * conjugate depth (Class-C tunable via m0_verifier_walk_backing). */
static void walk_virtue_backing(M0VerifierReport* out, uint8_t virtue_index) {
    char anchor[24];
    (void)snprintf(anchor, sizeof(anchor), "M0-2-9-%u", (unsigned)virtue_index);
    const M0IdentityChain* chain = m0_identity_chain_find(anchor);
    if (!chain) return;
    add_backing_ref(out, chain->coordinate);
    const uint8_t limit = chain->is_principle
        ? chain->link_count
        : (chain->link_count < M0_VERIFIER_DEFAULT_CONJUGATE_DEPTH
               ? chain->link_count
               : M0_VERIFIER_DEFAULT_CONJUGATE_DEPTH);
    for (uint8_t i = 0; i < limit; i++) {
        add_backing_ref(out, chain->links[i]);
    }
    /* Conjugate chains stay grounded: the walk always lands the root even
     * when the intermediate spine is depth-capped. */
    if (!chain->is_principle && chain->link_count > 0u) {
        add_backing_ref(out, chain->links[chain->link_count - 1u]);
    }
}

/* --- Act-route trajectory (Archetype 7 face) -----------------------------
 * Copies the state's RFactorPathStep trace into the report and emits the
 * (@#) band-turn marker at the position the R-traversal flips band
 * (Beauty→Life pivot; Siva-instruction-0 seed at M0-5-(0/1)-0). When the
 * trace flips band without an explicit R_BAND_TURN step, the verifier
 * inserts the marker: base_route 6 (Shakti seed), position 0. */
#define M0_VERIFIER_ROUTE_SHAKTI_BASE 6u

static void populate_route(const KernelState* state, M0VerifierReport* out) {
    out->band_turn_index = 0xFFu;
    const uint8_t in_count = state->route_step_count <= M0_VERIFIER_MAX_ROUTE_IN
        ? state->route_step_count
        : M0_VERIFIER_MAX_ROUTE_IN;

    uint8_t n = 0u;
    for (uint8_t i = 0u; i < in_count && n < M0_VERIFIER_MAX_ROUTE_OUT; i++) {
        const RFactorPathStep* step = &state->route_steps[i];
        if (i > 0u && n > 0u) {
            const uint8_t prev_band = out->r_factor_route[n - 1u].band;
            if (prev_band == (uint8_t)R_BAND_PRAVRITTI &&
                step->band == (uint8_t)R_BAND_NIVRITTI &&
                out->band_turn_index == 0xFFu) {
                out->r_factor_route[n].r_factor = step->r_factor;
                out->r_factor_route[n].base_route = M0_VERIFIER_ROUTE_SHAKTI_BASE;
                out->r_factor_route[n].band = (uint8_t)R_BAND_TURN;
                out->r_factor_route[n].position = 0u;
                out->band_turn_index = n;
                n++;
                if (n >= M0_VERIFIER_MAX_ROUTE_OUT) break;
            }
        }
        out->r_factor_route[n] = *step;
        if (step->band == (uint8_t)R_BAND_TURN && out->band_turn_index == 0xFFu) {
            out->band_turn_index = n;
        }
        n++;
    }
    out->route_step_count = n;
}

/* Archetype-7 closure law: the compiled terminal form reduces to
 * (##) and (R#) and (#R). A trajectory closes when it returns to matrix —
 * final step R5/Samavesa, structurally positionless (position 7). */
static void compile_closure(M0VerifierReport* out) {
    out->closure_marker.status = (uint8_t)M0_TRIAD_NOT_CLOSING;
    out->closure_marker.triad_bits = 0u;
    const uint8_t n = out->route_step_count;
    if (n == 0u) return;

    const RFactorPathStep* last = &out->r_factor_route[n - 1u];
    if (last->r_factor != 5u) return; /* non-closing: in-progress trajectory */

    uint8_t bits = 0u;

    /* (##) Truth — structure's lineage holds: every fretted step conforms
     * to the R-distribution matrix; R5 steps are positionless by word-size
     * law (the R_Factor_Route u16 cannot encode R5). */
    uint8_t truth = 1u;
    for (uint8_t i = 0u; i < n; i++) {
        const RFactorPathStep* step = &out->r_factor_route[i];
        if (step->band == (uint8_t)R_BAND_TURN) continue;
        if (step->r_factor == 5u) {
            if (step->position != 7u) { truth = 0u; break; }
            continue;
        }
        if (step->r_factor > 5u ||
            step->base_route >= R_FACTOR_ROUTE_COUNT) { truth = 0u; break; }
        const uint8_t expected =
            GET_R_POS(R_FACTOR_ROUTE_TABLE[step->base_route], step->r_factor);
        if (step->position != expected) { truth = 0u; break; }
    }
    if (truth) bits |= M0_TRIAD_BIT_TRUTH;

    /* (R#) Freedom — alpha of action: at least one operative act R0..R4
     * precedes the R5 return-to-matrix. */
    for (uint8_t i = 0u; i + 1u < n; i++) {
        if (out->r_factor_route[i].band != (uint8_t)R_BAND_TURN &&
            out->r_factor_route[i].r_factor <= 4u) {
            bits |= M0_TRIAD_BIT_FREEDOM;
            break;
        }
    }

    /* (#R) Openness — when both bands are traversed, the flip passed
     * through the (@#) gate; single-band closes hold openness trivially. */
    uint8_t saw_pravritti = 0u, saw_nivritti = 0u;
    for (uint8_t i = 0u; i < n; i++) {
        if (out->r_factor_route[i].band == (uint8_t)R_BAND_PRAVRITTI) saw_pravritti = 1u;
        if (out->r_factor_route[i].band == (uint8_t)R_BAND_NIVRITTI) saw_nivritti = 1u;
    }
    if (!(saw_pravritti && saw_nivritti) || out->band_turn_index != 0xFFu) {
        bits |= M0_TRIAD_BIT_OPENNESS;
    }

    out->closure_marker.triad_bits = bits;
    out->closure_marker.status =
        bits == (M0_TRIAD_BIT_TRUTH | M0_TRIAD_BIT_FREEDOM | M0_TRIAD_BIT_OPENNESS)
            ? (uint8_t)M0_TRIAD_COMPILES
            : (uint8_t)M0_TRIAD_INCOHERENT;
}

/* Kernel-static ontology validation: what the kernel can actually see of
 * the n10s validation surface. rel_type/tier law breaches are violations;
 * M0-branch endpoints that fail to resolve in the registry are counted
 * separately (Tier-4 relations live in Neo4j only; cross-branch endpoints
 * are out of kernel scope). */
static void validate_relations(M0VerifierReport* out) {
    M0OwlValidationReport* owl = &out->owl_validation;
    owl->live_n10s_deferred = 1u;
    owl->checked_relation_count = (uint16_t)M0_CORE_RELATIONS_COUNT;
    owl->violation_count = 0u;
    owl->unresolved_endpoint_count = 0u;

    for (uint16_t i = 0u; i < M0_CORE_RELATIONS_COUNT; i++) {
        const M0_Relation* rel = &M0_CORE_RELATIONS[i];
        if (rel->rel_type > (uint8_t)M0_REL_GENERATES ||
            rel->tier < 1u || rel->tier > 3u) {
            owl->violation_count++;
            continue;
        }
        const uint16_t ends[2] = { rel->source_coord, rel->target_coord };
        for (int e = 0; e < 2; e++) {
            if ((ends[e] >> 12) != 0u) continue; /* cross-branch: S2 scope */
            if (!anuttara_language_lookup_packed(ends[e])) {
                owl->unresolved_endpoint_count++;
            }
        }
    }
    owl->status = owl->violation_count == 0u
        ? (uint8_t)M0_OWL_KERNEL_STATIC_PASS
        : (uint8_t)M0_OWL_KERNEL_STATIC_VIOLATION;
}

static void check_membership(const KernelState* state, M0VerifierReport* out) {
    out->canonical_membership = 1u;
    const uint8_t count =
        state->engaged_coordinate_count <= M0_VERIFIER_MAX_ENGAGED_COORDS
            ? state->engaged_coordinate_count
            : M0_VERIFIER_MAX_ENGAGED_COORDS;
    for (uint8_t i = 0u; i < count; i++) {
        const char* coord = state->engaged_coordinates[i];
        if (coord[0] == '\0') continue;
        if (anuttara_language_is_member(coord)) continue;
        out->canonical_membership = 0u;
        char symbolic[M0_VERIFIER_COORDINATE_MAX];
        (void)snprintf(
            symbolic,
            sizeof(symbolic),
            "#M0-%.64s-violated?",
            coord
        );
        add_constraint(
            out,
            (uint8_t)M0_ANUTTARA_LAW_DERIVATION,
            "non-member-coordinate",
            symbolic
        );
    }
}

int m0_verifier_check_state(const KernelState* state, M0VerifierReport* out) {
    if (!state || !out) return -1;

    memset(out, 0, sizeof(*out));

    const uint16_t mask = committed_mask(state);

    for (uint8_t i = 0u; i < M0_VERIFIER_VIRTUE_COUNT; i++) {
        const float score = clamp01(state->virtue_evidence[i]);
        out->virtue_scores[i] = score;

        if ((mask & (uint16_t)(1u << i)) == 0u) {
            continue;
        }

        if (score >= M0_VERIFIER_VIRTUE_THRESHOLD) {
            out->virtue_witness_vector |= (uint16_t)(1u << i);
        } else {
            add_virtue_constraint(out, i, "unwitnessed");
            if (out->r_virtue_violation_count < M0_VERIFIER_VIRTUE_COUNT) {
                M0RVirtueViolation* v =
                    &out->r_virtue_violations[out->r_virtue_violation_count];
                v->virtue_index = i;
                v->r_factor = VIRTUE_LUT[i].r_factor;
                v->evidence = score;
                (void)snprintf(
                    v->coordinate,
                    M0_VERIFIER_COORDINATE_MAX,
                    "M0-2-9-%u",
                    (unsigned)i
                );
                out->r_virtue_violation_count++;
            }
        }
    }

    if (state->observed_core_relation_count != M0_CORE_RELATIONS_COUNT) {
        add_constraint(
            out,
            (uint8_t)M0_ANUTTARA_LAW_DERIVATION,
            "core-relation-skeleton",
            "#R0-0/1/R-T5-pending?"
        );
    }

    const uint16_t syntax_mask = state->syntax_layer_mask;
    if ((syntax_mask & M0_VERIFIER_SYNTAX_SPEECH) == 0u) {
        add_constraint(out, (uint8_t)M0_ANUTTARA_LAW_INTERROGATIVE,
                       "syntax-layer-speech", "#R0-0/1/S-T3-pending?");
    }
    if ((syntax_mask & M0_VERIFIER_SYNTAX_RELATIONSHIP) == 0u) {
        add_constraint(out, (uint8_t)M0_ANUTTARA_LAW_INTERROGATIVE,
                       "syntax-layer-relationship", "#R1-0/1/R-T5-pending?");
    }
    if ((syntax_mask & M0_VERIFIER_SYNTAX_ACTION) == 0u) {
        add_constraint(out, (uint8_t)M0_ANUTTARA_LAW_INTERROGATIVE,
                       "syntax-layer-action", "#R2-0/1/A-T7-pending?");
    }
    if ((syntax_mask & M0_VERIFIER_SYNTAX_COMPLETION) == 0u) {
        add_constraint(out, (uint8_t)M0_ANUTTARA_LAW_INTERROGATIVE,
                       "syntax-layer-completion", "#R5-0/1/A-T9-pending?");
    }

    out->slot_privacy_boundary_compliance =
        state->slot_privacy_boundary_compliance != 0u ? 1u : 0u;
    if (out->slot_privacy_boundary_compliance == 0u) {
        /* Law 2 — Containment as ontological act: the slot privacy boundary
         * IS a Frame; breaching it is a containment breach. */
        add_constraint(out, (uint8_t)M0_ANUTTARA_LAW_CONTAINMENT,
                       "slot-privacy-boundary",
                       "#R0-0/1/P-T0-slot-privacy-boundary?");
    }

    out->act_face = (uint8_t)(state->active_tct_position % 6u);
    out->witness_face = (uint8_t)(state->active_archetype % 6u);

    (void)m0_calc_witness(state, &out->syntax_witness_vector);
    populate_route(state, out);
    if (state->route_step_count > 0u) {
        /* The act-route face IS traced in this report (Archetype 7 bit). */
        out->syntax_witness_vector |= M0C_WITNESS_RFACTOR;
    }
    compile_closure(out);
    check_membership(state, out);
    validate_relations(out);

    /* Ontological-integrity trace: ground every emitted question and every
     * witnessed virtue back through the identity chains — the backing chain
     * is what makes the witness grounded rather than mere notation. */
    for (uint8_t i = 0u; i < M0_VERIFIER_VIRTUE_COUNT; i++) {
        if (out->virtue_witness_vector & (uint16_t)(1u << i)) {
            walk_virtue_backing(out, i);
        }
    }
    for (uint16_t i = 0u; i < out->r_virtue_violation_count; i++) {
        walk_virtue_backing(out, out->r_virtue_violations[i].virtue_index);
    }

    return 0;
}

int m0_verifier_emit_query(const KernelState* state, M0TypedQuery* out) {
    if (!state || !out) return -1;

    M0VerifierReport report;
    const int status = m0_verifier_check_state(state, &report);
    if (status != 0) return status;

    memset(out, 0, sizeof(*out));
    if (report.typed_query_count == 0u) return 1;
    *out = report.typed_queries[0];
    return 0;
}

int m0_verifier_walk_backing(
    const M0TypedQuery* q,
    const M0CoordinateRef* anchor,
    uint8_t depth,
    M0BackingChain* out
) {
    if (!anchor || !out) return -1;

    memset(out, 0, sizeof(*out));
    const M0IdentityChain* chain = m0_identity_chain_find(anchor->coordinate);
    if (!chain) return 1; /* no chain declared for this anchor */

    /* Principle chains and Law-3 queries ground unbounded; conjugate chains
     * respect the configured depth (Class-C tunability, Track 38). */
    const uint8_t unbounded = chain->is_principle ||
        (q && q->law_family == (uint8_t)M0_ANUTTARA_LAW_IDENTITY_CHAIN);
    uint8_t limit = chain->link_count;
    if (!unbounded && depth != M0_VERIFIER_DEPTH_UNBOUNDED && depth < limit) {
        limit = depth;
    }

    for (uint8_t i = 0u; i < limit && out->link_count < M0_VERIFIER_MAX_BACKING_CHAIN; i++) {
        M0CoordinateRef* ref = &out->links[out->link_count];
        (void)snprintf(ref->coordinate, M0_VERIFIER_COORDINATE_MAX, "%s",
                       chain->links[i]);
        const AnuttaraLanguageEntry* entry =
            anuttara_language_lookup(chain->links[i]);
        ref->packed = entry ? entry->packed : 0xFFFFu;
        out->link_count++;
    }
    out->depth = limit;
    out->grounded = 0u;
    if (out->link_count > 0u) {
        const char* last = out->links[out->link_count - 1u].coordinate;
        if (strncmp(last, "M0-0", 4) == 0 || strncmp(last, "M0-1", 4) == 0) {
            out->grounded = 1u;
        }
    }
    return 0;
}

int m0_verifier_emit_question(
    const M0VerifierReport* report,
    char* out_buf,
    size_t buf_len
) {
    if (!report || !out_buf || buf_len == 0u) return -1;

    const char* question = "#R0-0/1/S-T3-pending?";
    if (report->unsatisfied_count > 0u &&
        report->unsatisfied_constraints[0][0] != '\0') {
        question = report->unsatisfied_constraints[0];
    }

    const int written = snprintf(out_buf, buf_len, "%s", question);
    if (written < 0 || (size_t)written >= buf_len) return -1;
    return 0;
}
