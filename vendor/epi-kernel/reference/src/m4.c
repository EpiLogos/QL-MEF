/**
 * m4.c — Nara: The Personal Dialogical Interface (Implementation)
 *
 * All .rodata LUT data + API implementation for M4.
 * FR Coverage: 2.4.0 – 2.4.13
 */

#include "m4.h"
#include "blake3.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>


/* ===================================================================
 * LENS VTABLE STUBS — stub implementations for 6 lenses
 * =================================================================== */

static int m4_lens_translate_stub(uint8_t lens_id, const char* input,
                                   const char* context, char* output, size_t len) {
    (void)input; (void)context;
    if (len > 0) {
        snprintf(output, len, "[Lens %u] stub translation", lens_id);
    }
    return 0;
}

static int m4_lens_activate_stub(uint8_t lens_id, void* phenom_state) {
    (void)lens_id; (void)phenom_state;
    return 0;
}

static int m4_lens_deactivate_stub(uint8_t lens_id, void* phenom_state) {
    (void)lens_id; (void)phenom_state;
    return 0;
}

static int m4_lens_annotate_stub(uint8_t lens_id, const void* experience,
                                  char* annotation, size_t len) {
    (void)experience;
    if (len > 0) {
        snprintf(annotation, len, "[Lens %u] stub annotation", lens_id);
    }
    return 0;
}


/* ===================================================================
 * .RODATA: M4_LENS_REGISTRY[6] — 6-lens vtable
 * =================================================================== */

const M4_Lens_Vtable M4_LENS_REGISTRY[6] = {
    { "Gebser",           m4_lens_translate_stub, m4_lens_activate_stub, m4_lens_deactivate_stub, m4_lens_annotate_stub },
    { "Ontological",      m4_lens_translate_stub, m4_lens_activate_stub, m4_lens_deactivate_stub, m4_lens_annotate_stub },
    { "Epistemological",  m4_lens_translate_stub, m4_lens_activate_stub, m4_lens_deactivate_stub, m4_lens_annotate_stub },
    { "Jungian Depth",    m4_lens_translate_stub, m4_lens_activate_stub, m4_lens_deactivate_stub, m4_lens_annotate_stub },
    { "Phenomenological", m4_lens_translate_stub, m4_lens_activate_stub, m4_lens_deactivate_stub, m4_lens_annotate_stub },
    { "Trika/Kashmir",    m4_lens_translate_stub, m4_lens_activate_stub, m4_lens_deactivate_stub, m4_lens_annotate_stub },
};


/* ===================================================================
 * .RODATA: M4_PROTOCOL_LIBRARY[12][3] — 36 decan recipe cards
 *
 * Stub values: correct structure, indexed by storey/decan.
 * Real values would come from the dataset.
 * =================================================================== */

const M4_Decan_Recipe_Card M4_PROTOCOL_LIBRARY[12][3] = {
    /* storey 0 */
    {{ 0, 0, 1, {0,0,0,0,0,0,0},   0,  0, M4_ELEM_FIRE,  1, 0 },
     { 0, 1, 1, {1,0,0,0,0,0,0},  10,  1, M4_ELEM_EARTH, 2, 0 },
     { 0, 2, 1, {2,0,0,0,0,0,0},  20,  2, M4_ELEM_AIR,   3, 0 }},
    /* storey 1 */
    {{ 1, 0, 1, {0,0,0,0,0,0,0},  30,  3, M4_ELEM_WATER, 4, 0 },
     { 1, 1, 1, {1,0,0,0,0,0,0},  40,  4, M4_ELEM_FIRE,  5, 0 },
     { 1, 2, 1, {2,0,0,0,0,0,0},  50,  5, M4_ELEM_EARTH, 6, 0 }},
    /* storey 2 */
    {{ 2, 0, 1, {0,0,0,0,0,0,0},  60,  6, M4_ELEM_AIR,   0, 0 },
     { 2, 1, 1, {1,0,0,0,0,0,0},  70,  7, M4_ELEM_WATER, 1, 0 },
     { 2, 2, 1, {2,0,0,0,0,0,0},  80,  8, M4_ELEM_FIRE,  2, 0 }},
    /* storey 3 */
    {{ 3, 0, 1, {0,0,0,0,0,0,0},  90,  9, M4_ELEM_EARTH, 3, 0 },
     { 3, 1, 1, {1,0,0,0,0,0,0}, 100, 10, M4_ELEM_AIR,   4, 0 },
     { 3, 2, 1, {2,0,0,0,0,0,0}, 110, 11, M4_ELEM_WATER, 5, 0 }},
    /* storey 4 */
    {{ 4, 0, 1, {0,0,0,0,0,0,0}, 120,  0, M4_ELEM_FIRE,  6, 0 },
     { 4, 1, 1, {1,0,0,0,0,0,0}, 130,  1, M4_ELEM_EARTH, 0, 0 },
     { 4, 2, 1, {2,0,0,0,0,0,0}, 140,  2, M4_ELEM_AIR,   1, 0 }},
    /* storey 5 */
    {{ 5, 0, 1, {0,0,0,0,0,0,0}, 150,  3, M4_ELEM_WATER, 2, 0 },
     { 5, 1, 1, {1,0,0,0,0,0,0}, 160,  4, M4_ELEM_FIRE,  3, 0 },
     { 5, 2, 1, {2,0,0,0,0,0,0}, 170,  5, M4_ELEM_EARTH, 4, 0 }},
    /* storey 6 */
    {{ 6, 0, 1, {0,0,0,0,0,0,0}, 180,  6, M4_ELEM_AIR,   5, 0 },
     { 6, 1, 1, {1,0,0,0,0,0,0}, 190,  7, M4_ELEM_WATER, 6, 0 },
     { 6, 2, 1, {2,0,0,0,0,0,0}, 200,  8, M4_ELEM_FIRE,  0, 0 }},
    /* storey 7 */
    {{ 7, 0, 1, {0,0,0,0,0,0,0}, 210,  9, M4_ELEM_EARTH, 1, 0 },
     { 7, 1, 1, {1,0,0,0,0,0,0}, 220, 10, M4_ELEM_AIR,   2, 0 },
     { 7, 2, 1, {2,0,0,0,0,0,0}, 230, 11, M4_ELEM_WATER, 3, 0 }},
    /* storey 8 */
    {{ 8, 0, 1, {0,0,0,0,0,0,0}, 240,  0, M4_ELEM_FIRE,  4, 0 },
     { 8, 1, 1, {1,0,0,0,0,0,0}, 250,  1, M4_ELEM_EARTH, 5, 0 },
     { 8, 2, 1, {2,0,0,0,0,0,0}, 260,  2, M4_ELEM_AIR,   6, 0 }},
    /* storey 9 */
    {{ 9, 0, 1, {0,0,0,0,0,0,0}, 270,  3, M4_ELEM_WATER, 0, 0 },
     { 9, 1, 1, {1,0,0,0,0,0,0}, 280,  4, M4_ELEM_FIRE,  1, 0 },
     { 9, 2, 1, {2,0,0,0,0,0,0}, 290,  5, M4_ELEM_EARTH, 2, 0 }},
    /* storey 10 */
    {{10, 0, 1, {0,0,0,0,0,0,0}, 300,  6, M4_ELEM_AIR,   3, 0 },
     {10, 1, 1, {1,0,0,0,0,0,0}, 310,  7, M4_ELEM_WATER, 4, 0 },
     {10, 2, 1, {2,0,0,0,0,0,0}, 320,  8, M4_ELEM_FIRE,  5, 0 }},
    /* storey 11 */
    {{11, 0, 1, {0,0,0,0,0,0,0}, 330,  9, M4_ELEM_EARTH, 6, 0 },
     {11, 1, 1, {1,0,0,0,0,0,0}, 340, 10, M4_ELEM_AIR,   0, 0 },
     {11, 2, 1, {2,0,0,0,0,0,0}, 350, 11, M4_ELEM_WATER, 1, 0 }},
};


/* ===================================================================
 * .RODATA: ALCHEMICAL OP STUBS
 * =================================================================== */

static int m4_op_stub(void* state, const void* reagent) {
    (void)state; (void)reagent;
    return 0;
}

const Alchemical_Op M4_ALCHEMY_OPS[7] = {
    m4_op_stub,     /* 0: Calcination */
    m4_op_stub,     /* 1: Dissolution */
    m4_op_stub,     /* 2: Separation */
    m4_op_stub,     /* 3: Conjunction */
    m4_op_stub,     /* 4: Fermentation */
    m4_op_stub,     /* 5: Distillation */
    m4_op_stub,     /* 6: Coagulation */
};


/* ===================================================================
 * .RODATA: MEF THRESHOLDS per lens — monotonically increasing
 * =================================================================== */

const uint8_t M4_LENS_MEF_THRESHOLD[6] = {20, 40, 60, 80, 100, 120};


/* ===================================================================
 * .RODATA: VOICE CONFIG — default 8 bytes
 * =================================================================== */

const M4_Voice_Config M4_VOICE_CONFIG = {
    .formality       = 128,
    .warmth          = 192,
    .directness      = 160,
    .depth           = 200,
    .metaphor_density = 180,
    .humor           = 80,
    .challenge       = 100,
    ._pad            = 0
};


/* ===================================================================
 * .RODATA: CONTAINER LUT[3]
 * =================================================================== */

const M4_Container_Entry M4_CONTAINER_LUT[M4_CONTAINER_COUNT] = {
    { 0, 1, {0, 0} },      /* Circle: solo */
    { 1, 2, {0, 0} },      /* Temenos: dyad */
    { 2, 6, {0, 0} },      /* Vessel: group (up to 6) */
};


/* ===================================================================
 * _Static_asserts for size claims
 * =================================================================== */

_Static_assert(sizeof(M4_Voice_Config) == 8,
    "M4_Voice_Config must be 8 bytes");
_Static_assert(sizeof(M4_Container_Entry) == 4,
    "M4_Container_Entry must be 4 bytes");


/* ===================================================================
 * SACRED RANDOM — consent-gated true random
 * =================================================================== */

#ifdef __APPLE__
#include <stdlib.h>     /* arc4random_buf on macOS */
#endif

bool m4_sacred_random(M4_Sacred_Random* rng, uint8_t* buf, size_t len) {
    if (!rng || !buf || len == 0) return false;
    if (!rng->consent_granted) return false;
#ifdef __APPLE__
    arc4random_buf(buf, len);
#else
    /* Fallback: zero-fill (not cryptographic — stub for non-macOS) */
    memset(buf, 0, len);
#endif
    return true;
}


/* ===================================================================
 * API: m4_init — Allocate and HC-link M4_Root
 * =================================================================== */

M4_Root* m4_init(Coordinate_Arena* arena, Holographic_Coordinate* hc) {
    if (!arena || !hc) return NULL;
    if (hc->ql_position != 4) return NULL;  /* Must be #4 */

    M4_Root* root = (M4_Root*)malloc(sizeof(M4_Root));
    if (!root) return NULL;

    memset(root, 0, sizeof(M4_Root));
    HC_LINK(hc, root);
    root->active_cf = cf_get(CF_FRACTAL);

    /* Wire lens registry to .rodata */
    root->pco.lenses.registry = M4_LENS_REGISTRY;

    /* Wire voice config */
    root->pco.epii.voice_config = &M4_VOICE_CONFIG;

    /* Wire alchemy ops into medicine */
    for (int i = 0; i < 7; i++) {
        root->pco.medicine.ops[i] = M4_ALCHEMY_OPS[i];
    }

    /* Default safety */
    root->pco.medicine.safety_level = 128;
    root->pco.cycle.safety_threshold = 200;

    return root;
}


/* ===================================================================
 * API: m4_teardown — Release M4_Root heap state
 * =================================================================== */

void m4_teardown(M4_Root* root) {
    if (!root) return;
    /* Free phase history if allocated */
    if (root->pco.phase_history.records) {
        free(root->pco.phase_history.records);
    }
    /* Free complexes if allocated */
    if (root->pco.lenses.complexes) {
        free(root->pco.lenses.complexes);
    }
    if (root->hc) {
        HC_UNLINK(root->hc);
    }
    free(root);
}


/* ===================================================================
 * API: m4_identity_compute — BLAKE3 quintessence + Symbol DNA
 * =================================================================== */

void m4_identity_compute(M4_Identity_Matrix* id, M4_Input_Data* input) {
    if (!id || !input) return;
    if (id->computed) return;   /* Compute-once guard */

    /* Step 1: Derive Symbol DNA Profile from raw input */
    id->dna_profile.gene_keys_activation = 0;   /* Stub: populated by external data */
    id->dna_profile.nucleotide_balance.adenine_water  = (input->mbti_raw & 0x03) ? 180 : 60;
    id->dna_profile.nucleotide_balance.thymine_fire   = (input->mbti_raw & 0x0C) ? 180 : 60;
    id->dna_profile.nucleotide_balance.cytosine_earth = (input->mbti_raw & 0x30) ? 180 : 60;
    id->dna_profile.nucleotide_balance.guanine_air    = (input->mbti_raw & 0xC0) ? 180 : 60;
    id->dna_profile.sun_degree_anchor  = 0;     /* Stub: populated by Kerykeion */
    id->dna_profile.moon_degree_anchor = 0;

    /* Step 2: Compute numerological key from birthdate */
    id->numerological_key = (uint32_t)(input->birth_year + input->birth_month * 100u
                                       + input->birth_day);

    /* Step 3: BLAKE3 quintessence — full 32-byte archetypal synthesis hash */
    blake3_hasher hasher;
    blake3_hasher_init(&hasher);
    blake3_hasher_update(&hasher, input, sizeof(M4_Input_Data));
    blake3_hasher_finalize(&hasher, id->quintessence_hash, 32);

    /* Generate hex preview string */
    for (int _i = 0; _i < 32; _i++) {
        snprintf(id->quintessence_preview + _i * 2, 3, "%02x",
                 (unsigned)id->quintessence_hash[_i]);
    }

    /* Step 4: ARCHITECTURAL PRIVACY — destroy raw input immediately */
    memset(input, 0, sizeof(M4_Input_Data));

    /* Step 5: Mark computed */
    id->computed = true;
}


/* ===================================================================
 * API: m4_identity_hash_compute — BLAKE3 from present layers only
 *
 * Input format: layer_presence (1 byte) || each PRESENT layer's raw bytes
 * Absent layers contribute NOTHING to the hash input.
 * =================================================================== */

void m4_identity_hash_compute(M4_Identity_Matrix* id) {
    if (!id) return;

    /* Layer byte sizes and pointers — indexed by layer number */
    const void* layer_ptrs[5] = {
        &id->layer_0,  /* 0: Numerological — 8 bytes */
        &id->layer_1,  /* 1: Astrological  — 32 bytes */
        &id->layer_2,  /* 2: Jungian       — 12 bytes */
        &id->layer_3,  /* 3: Gene Keys     — 40 bytes */
        &id->layer_4,  /* 4: Human Design  — 20 bytes */
    };
    const size_t layer_sizes[5] = {
        sizeof(M4_Numerological_Layer),
        sizeof(M4_Astrological_Layer),
        sizeof(M4_Jungian_Layer),
        sizeof(M4_GeneKeys_Layer),
        sizeof(M4_HumanDesign_Layer),
    };

    blake3_hasher hasher;
    blake3_hasher_init(&hasher);

    /* First byte: presence mask */
    blake3_hasher_update(&hasher, &id->layer_presence, 1);

    /* Each present layer's raw bytes, in order 0→4 */
    for (int i = 0; i < 5; i++) {
        if (id->layer_presence & (1u << (unsigned)i)) {
            blake3_hasher_update(&hasher, layer_ptrs[i], layer_sizes[i]);
        }
    }

    blake3_hasher_finalize(&hasher, id->quintessence_hash, 32);

    /* Generate hex preview string */
    for (int _i = 0; _i < 32; _i++) {
        snprintf(id->quintessence_preview + _i * 2, 3, "%02x",
                 (unsigned)id->quintessence_hash[_i]);
    }
}


/* ===================================================================
 * API: m4_identity_augment — Add layer, recompute hash
 * =================================================================== */

void m4_identity_augment(M4_Identity_Matrix* id,
                         uint8_t layer_index,
                         const void* new_layer_data,
                         size_t new_layer_size) {
    if (!id || !new_layer_data || layer_index > 4) return;

    /* Target slot and expected size */
    void* layer_slots[5] = {
        &id->layer_0,
        &id->layer_1,
        &id->layer_2,
        &id->layer_3,
        &id->layer_4,
    };
    const size_t expected_sizes[5] = {
        sizeof(M4_Numerological_Layer),
        sizeof(M4_Astrological_Layer),
        sizeof(M4_Jungian_Layer),
        sizeof(M4_GeneKeys_Layer),
        sizeof(M4_HumanDesign_Layer),
    };

    /* Validate size */
    if (new_layer_size != expected_sizes[layer_index]) return;

    /* Copy layer data into the appropriate slot */
    memcpy(layer_slots[layer_index], new_layer_data, new_layer_size);

    /* Set presence bit */
    id->layer_presence |= (uint8_t)(1u << layer_index);

    /* Recompute quintessence hash */
    m4_identity_hash_compute(id);
}


/* ===================================================================
 * API: m4_cast_iching — I-Ching casting
 * =================================================================== */

int m4_cast_iching(M4_Sacred_Random* rng, uint16_t cast_degree,
                   M4_IChing_Cast* out) {
    if (!rng || !out) return -1;
    memset(out, 0, sizeof(M4_IChing_Cast));
    out->cast_degree = cast_degree;

    uint8_t rand_buf[6];
    if (!m4_sacred_random(rng, rand_buf, 6)) return -EPERM;

    uint8_t hex_bits = 0;
    for (int i = 0; i < 6; i++) {
        /* 3 coins from random byte: bits 0-2, heads=3 tails=2 */
        uint8_t sum = 0;
        for (int c = 0; c < 3; c++)
            sum += ((rand_buf[i] >> c) & 1) ? 3 : 2;
        out->lines[i] = sum;   /* 6, 7, 8, or 9 */
        /* Yang line (7 or 9) = bit set; Yin (6 or 8) = bit clear */
        if (sum & 1) hex_bits |= (uint8_t)(1u << i);
        /* Changing lines: 6 (old yin) or 9 (old yang) */
        if (sum == 6 || sum == 9) out->changing_mask |= (uint8_t)(1u << i);
    }
    out->hexagram_id = hex_bits & 0x3F;
    out->result_hexagram = (uint8_t)((hex_bits ^ out->changing_mask) & 0x3F);
    return 0;
}


/* ===================================================================
 * API: m4_draw_tarot — Tarot draw with Fisher-Yates shuffle
 * =================================================================== */

int m4_draw_tarot(M4_Sacred_Random* rng, uint8_t count, uint16_t cast_degree,
                  M4_Tarot_Draw* out) {
    if (!rng || !out || count == 0 || count > 12) return -1;
    memset(out, 0, sizeof(M4_Tarot_Draw));
    out->cast_degree = cast_degree;
    out->draw_count = count;

    /* Initialize deck 0-77 */
    for (int i = 0; i < 78; i++) out->cards[i] = (uint8_t)i;

    /* Fisher-Yates shuffle (consent-gated random) */
    uint8_t rand_buf[78];
    if (!m4_sacred_random(rng, rand_buf, 78)) return -EPERM;

    for (int i = 77; i > 0; i--) {
        uint8_t j = rand_buf[i] % (uint8_t)(i + 1);
        uint8_t tmp = out->cards[i];
        out->cards[i] = out->cards[j];
        out->cards[j] = tmp;
    }

    /* Draw top N */
    for (int i = 0; i < count; i++) out->drawn[i] = out->cards[i];
    return 0;
}


/* ===================================================================
 * API: m4_verify — Boot-time .rodata verification
 * =================================================================== */

bool m4_verify(void) {
    /* Voice config size */
    if (sizeof(M4_Voice_Config) != 8) return false;

    /* Container LUT has 3 entries */
    if (M4_CONTAINER_LUT[0].container_type != 0) return false;
    if (M4_CONTAINER_LUT[1].container_type != 1) return false;
    if (M4_CONTAINER_LUT[2].container_type != 2) return false;

    /* MEF thresholds monotonically increasing */
    for (int i = 1; i < 6; i++) {
        if (M4_LENS_MEF_THRESHOLD[i] <= M4_LENS_MEF_THRESHOLD[i - 1])
            return false;
    }

    /* Protocol library: 12x3 spot check — storey indices match */
    for (int s = 0; s < 12; s++) {
        for (int d = 0; d < 3; d++) {
            if (M4_PROTOCOL_LIBRARY[s][d].storey != (uint8_t)s) return false;
            if (M4_PROTOCOL_LIBRARY[s][d].decan != (uint8_t)d) return false;
        }
    }

    /* Elemental Throughline: nucleotide == element index */
    if (M3_NUC_A != M4_ELEM_WATER) return false;
    if (M3_NUC_T != M4_ELEM_FIRE)  return false;
    if (M3_NUC_C != M4_ELEM_EARTH) return false;
    if (M3_NUC_G != M4_ELEM_AIR)   return false;

    /* Lens registry populated */
    for (int i = 0; i < 6; i++) {
        if (M4_LENS_REGISTRY[i].translate == NULL) return false;
        if (M4_LENS_REGISTRY[i].lens_name == NULL) return false;
    }

    /* Alchemy ops populated */
    for (int i = 0; i < 7; i++) {
        if (M4_ALCHEMY_OPS[i] == NULL) return false;
    }

    return true;
}


/* ===================================================================
 * CLI: Helper Print Functions
 * =================================================================== */

static void m4_print_info(const M4_Root* root) {
    printf("M4 (Nara) — The Personal Dialogical Interface\n");
    printf("  Context Frame: CF_FRACTAL (4.0/1-4.4/5)\n");
    printf("  Lenses:        6 (vtable dispatch, indexed)\n");
    printf("  Protocol:      12x3 = 36 recipe cards (.rodata)\n");
    printf("  Alchemy Ops:   7 canonical verbs\n");
    printf("  Containers:    %d dialogical containers\n", M4_CONTAINER_COUNT);
    printf("  Voice Config:  %zu bytes (.rodata)\n", sizeof(M4_Voice_Config));
    printf("  Identity:      %s\n",
           root->pco.identity.computed ? "computed" : "not computed");
    printf("  Cycle:         storey=%u decan=%u stroke=%u\n",
           root->pco.cycle.current_storey,
           root->pco.cycle.current_decan,
           root->pco.cycle.current_stroke);
    printf("  Safety:        arousal=%u threshold=%u\n",
           root->pco.cycle.arousal_level,
           root->pco.cycle.safety_threshold);
    printf("  Alch Stage:    %u\n", root->pco.lenses.alch_stage);
}

static void m4_print_identity(const M4_Root* root) {
    const M4_Identity_Matrix* id = &root->pco.identity;
    if (!id->computed) {
        printf("M4 Identity: not computed.\n");
        printf("  Use m4_identity_compute() to derive Symbol DNA from input data.\n");
        return;
    }
    printf("M4 Identity (computed):\n");
    printf("  Quintessence Hash: %s\n", id->quintessence_preview);
    printf("  Numerological Key: %u\n", id->numerological_key);
    printf("  Gene Keys Activation: 0x%016llX\n", (unsigned long long)id->dna_profile.gene_keys_activation);
    printf("  Nucleotide Balance: A=%u T=%u C=%u G=%u\n",
           id->dna_profile.nucleotide_balance.adenine_water,
           id->dna_profile.nucleotide_balance.thymine_fire,
           id->dna_profile.nucleotide_balance.cytosine_earth,
           id->dna_profile.nucleotide_balance.guanine_air);
    printf("  Sun Degree: %u  Moon Degree: %u\n",
           id->dna_profile.sun_degree_anchor,
           id->dna_profile.moon_degree_anchor);
}

static void m4_print_now(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr, "Usage: m4 now <degree>  (0-719)\n");
        return;
    }
    int deg = atoi(argv[2]);
    if (deg < 0 || deg > 719) {
        fprintf(stderr, "Error: degree must be 0-719\n");
        return;
    }
    M4_Temporal_Now now = m4_snapshot_now((uint16_t)deg, 0);
    printf("M4 Temporal Now at %d:\n", deg);
    printf("  M1 Torus Stage:  %u (of 12)\n", now.clock.tick12);
    printf("  M2 Decan Phase:  %u (of 72)\n", now.clock.m2_decan_phase);
    printf("  M3 Hexagram:     %u (of 64)\n", now.clock.m3_hexagram_id);
    printf("  Layer:           %s\n", now.clock.is_implicate_phase ? "Shadow (implicate)" : "Primary (explicate)");
    printf("  Planets:         %u of 7 valid\n", now.planet_valid);
}

static void m4_print_pratibimba(const M4_Root* root) {
    const M4_Personal_Pratibimba* p = &root->pco.lenses.pratibimba;
    printf("M4 Personal Pratibimba (#4.4.4.4):\n");
    printf("  Identity Manifested: %s\n", p->identity_manifested ? "yes" : "no");
    printf("  Telemetry Active:    %s\n", p->telemetry_active ? "yes" : "no");
    printf("  Pattern Count:       %u\n", p->pattern_count);
    printf("  Synth Signature:     0x%016llX\n", (unsigned long long)p->synthesized_signature);
    printf("  Active Complexes:    6 slots\n");
    for (int i = 0; i < 6; i++) {
        printf("    [%d] arch=%u charge=%.2f autonomy=%.2f conscious=%s\n",
               i, p->active_complexes[i].archetype_id,
               (double)p->active_complexes[i].charge,
               (double)p->active_complexes[i].autonomy,
               p->active_complexes[i].conscious ? "yes" : "no");
    }
}

static int m4_cli_cast(int argc, char** argv, M4_Root* root) {
    if (argc < 3) {
        fprintf(stderr, "Usage: m4 cast iching|tarot [count]\n");
        return 1;
    }
    /* Grant consent for CLI usage */
    root->pco.rng.consent_granted = true;

    if (strcmp(argv[2], "iching") == 0) {
        M4_IChing_Cast cast;
        int rc = m4_cast_iching(&root->pco.rng, root->pco.last_now.degree, &cast);
        if (rc != 0) {
            fprintf(stderr, "I-Ching cast failed: %d\n", rc);
            return rc;
        }
        printf("I-Ching Cast at degree %u:\n", cast.cast_degree);
        printf("  Lines (bottom to top): ");
        for (int i = 0; i < 6; i++) printf("%u ", cast.lines[i]);
        printf("\n");
        printf("  Hexagram:     %u\n", cast.hexagram_id);
        printf("  Changing:     0x%02X\n", cast.changing_mask);
        printf("  Result hex:   %u\n", cast.result_hexagram);
        return 0;
    }
    if (strcmp(argv[2], "tarot") == 0) {
        uint8_t count = 3;
        if (argc >= 4) count = (uint8_t)atoi(argv[3]);
        if (count == 0 || count > 12) count = 3;

        M4_Tarot_Draw draw;
        int rc = m4_draw_tarot(&root->pco.rng, count, root->pco.last_now.degree, &draw);
        if (rc != 0) {
            fprintf(stderr, "Tarot draw failed: %d\n", rc);
            return rc;
        }
        printf("Tarot Draw (%u cards) at degree %u:\n", draw.draw_count, draw.cast_degree);
        for (int i = 0; i < draw.draw_count; i++) {
            printf("  Card %d: %u\n", i + 1, draw.drawn[i]);
        }
        return 0;
    }
    fprintf(stderr, "Unknown cast type: %s (use iching or tarot)\n", argv[2]);
    return 1;
}


/* ===================================================================
 * API: m4_cli_dispatch — CLI entry point
 * =================================================================== */

int m4_cli_dispatch(int argc, char** argv, M4_Root* root) {
    if (argc < 2) {
        m4_print_info(root);
        return 0;
    }

    const char* cmd = argv[1];
    if (strcmp(cmd, "info") == 0)       { m4_print_info(root);               return 0; }
    if (strcmp(cmd, "identity") == 0)   { m4_print_identity(root);           return 0; }
    if (strcmp(cmd, "now") == 0)        { m4_print_now(argc, argv);          return 0; }
    if (strcmp(cmd, "cast") == 0)       { return m4_cli_cast(argc, argv, root); }
    if (strcmp(cmd, "pratibimba") == 0) { m4_print_pratibimba(root);         return 0; }
    if (strcmp(cmd, "advance") == 0)    {
        m4_advance_transformation(&root->pco.cycle);
        printf("Advanced: storey=%u decan=%u stroke=%u\n",
               root->pco.cycle.current_storey,
               root->pco.cycle.current_decan,
               root->pco.cycle.current_stroke);
        return 0;
    }
    if (strcmp(cmd, "lens") == 0)       {
        if (argc < 3) {
            printf("Active lens: %u (%s)\n",
                   root->pco.lenses.active_lens,
                   M4_LENS_REGISTRY[root->pco.lenses.active_lens].lens_name);
            return 0;
        }
        int idx = atoi(argv[2]);
        if (idx < 0 || idx > 5) {
            fprintf(stderr, "Lens index must be 0-5\n");
            return 1;
        }
        root->pco.lenses.active_lens = (uint8_t)idx;
        printf("Activated lens %d: %s\n", idx, M4_LENS_REGISTRY[idx].lens_name);
        return 0;
    }

    fprintf(stderr, "m4: unknown subcommand '%s'\n", cmd);
    fprintf(stderr, "Usage: m4 [info|identity|now|cast|advance|lens|pratibimba]\n");
    return 1;
}
