/**
 * m_canonical.h — M2 element REGISTERS and the correspondences between them
 *
 * ─────────────────────────────────────────────────────────────────────────────
 * THERE IS NO SINGLE "ELEMENT ID". Several M2 sub-coordinates carry something
 * called an element, and they are DIFFERENT ONTOLOGIES sharing an English word —
 * not one thing in several encodings. Each register is complete and correct
 * within its owning coordinate:
 *
 *   M2-2 (36 Tattvas)  — the MAHABHUTA series, tattvas 31..35 (see m2.c:
 *                        "Mahabhutas — the 5 elements"): AKASHA=0, VAYU=1,
 *                        AGNI=2, APAS=3, PRITHVI=4. Its order is Saiva
 *                        emanation — space densifying to earth. This register
 *                        is `tattva_index - 31`; it is NOT a legacy ordering.
 *                        Also owns the chakra<->tattva (yogic) body.
 *   M2-1 (MEF) / L2'   — the ALCHEMICAL sixfold: AETHER=0, EARTH=1, WATER=2,
 *                        AIR=3, FIRE=4, SALT=5. Its order is the opus
 *                        (nigredo/solutio/sublimatio/calcinatio), and SALT is a
 *                        Tria Prima principle belonging to no other system.
 *                        L2' is ONE lens-family inside M2-1's mef_lenses[12][6].
 *   M2-3 (Decans)      — the zodiacal TRIPLICITY, four operative elements over
 *                        twelve signs, Quintessence as the #2-3-5/0 sentinel.
 *                        Also owns the decan<->body-part (Hermetic medical)
 *                        body — a DIFFERENT body ontology from M2-2's.
 *   m3.h clock         — `Clock_Degree_Entry.decan_element`, the M3 clock's own.
 *   m4.h nucleotide    — A/T/C/G reaching element via the throughline.
 *
 * So the mappings below are CORRESPONDENCES — tradition-bridging claims with
 * content — not casts. AKASHA <-> AETHER asserts something. SALT has no
 * mahabhuta counterpart not because a mapping is "lossy" but because SALT is
 * not a mahabhuta at all; the Mahabhuta series is not missing a member, it is a
 * different system. Every converter here is therefore partial by nature and
 * returns M_CANONICAL_ELEMENT_INVALID where no counterpart exists.
 *
 * ON THE WIRE. The M-stack serialises elements in the ALCHEMICAL register when
 * they cross a coordinate boundary. That is a SERIALISATION CHOICE — one shape
 * is needed on the wire and this is the one that landed — and NOT a claim that
 * M2-1's lens governs M2-2's tattvas or M2-3's triplicities. The `Canonical_*`
 * symbol names below are historical: read them as "the alchemical register /
 * the wire encoding", never as "the canonical element".
 *
 * Task: 05.T5.16 (harmonisation) — reframed by DR-L2-ELEM-2 (2026-07-25), which
 * retired the global-primacy reading. See Idea/Bimba/World/L2'.md.
 * ─────────────────────────────────────────────────────────────────────────────
 */

#ifndef M_CANONICAL_H
#define M_CANONICAL_H

#include <stdint.h>
#include <stdbool.h>

/* ===================================================================
 * THE ALCHEMICAL REGISTER (M2-1 / L2' inner-position ordering).
 * Also the wire encoding — a serialisation choice, not a primacy claim.
 * =================================================================== */

typedef enum {
    ELEMENT_AETHER = 0,  /* L2-0' — Quintessence / Prima Materia */
    ELEMENT_EARTH  = 1,  /* L2-1' — Nigredo / Fixed Principle    */
    ELEMENT_WATER  = 2,  /* L2-2' — Solutio / Dissolving         */
    ELEMENT_AIR    = 3,  /* L2-3' — Sublimatio / Volatile        */
    ELEMENT_FIRE   = 4,  /* L2-4' — Calcinatio / Transformative  */
    ELEMENT_SALT   = 5   /* L2-5' — Sal / Diamond Body           */
} Canonical_Element;

#define M_CANONICAL_ELEMENT_COUNT   6u

/* Sentinel returned when a value has no counterpart in the target ordering. */
#define M_CANONICAL_ELEMENT_INVALID 0xFFu

/* The operative quartet = bits 1-4 set (EARTH, WATER, AIR, FIRE).
 *   bit ELEMENT_EARTH(1) | ELEMENT_WATER(2) | ELEMENT_AIR(3) | ELEMENT_FIRE(4)
 *   = 0b00011110 = 0x1E. AETHER(0) and SALT(5) are NOT operative. */
#define OPERATIVE_QUARTET_MASK 0x1Eu

/* True when `elem` is one of the four operative classical elements. */
static inline bool m_canonical_is_operative(uint8_t elem) {
    return elem < 8u && ((OPERATIVE_QUARTET_MASK >> elem) & 1u) != 0u;
}

/* ===================================================================
 * NUCLEOTIDE → ALCHEMICAL REGISTER (the Elemental Throughline)
 *
 * A=Water, T=Fire, C=Earth, G=Air. Nucleotide arg uses M3_NUC_* values
 * (A=0, T=1, C=2, G=3). Implemented as a function-like macro so it yields
 * an integer constant expression — usable inside _Static_assert in m4.h.
 * =================================================================== */

#define m4_nuc_to_elem(nuc) \
    ((nuc) == 0u ? (uint8_t)ELEMENT_WATER : \
     (nuc) == 1u ? (uint8_t)ELEMENT_FIRE  : \
     (nuc) == 2u ? (uint8_t)ELEMENT_EARTH : \
     (nuc) == 3u ? (uint8_t)ELEMENT_AIR   : \
                   (uint8_t)M_CANONICAL_ELEMENT_INVALID)

/* ===================================================================
 * m4.h NUCLEOTIDE-ORDER REGISTER  ⇄  ALCHEMICAL
 *   legacy: WATER=0, FIRE=1, EARTH=2, AIR=3
 * =================================================================== */

static inline uint8_t m_canonical_from_m4_h_legacy(uint8_t legacy) {
    switch (legacy) {
        case 0u: return (uint8_t)ELEMENT_WATER;  /* legacy WATER */
        case 1u: return (uint8_t)ELEMENT_FIRE;   /* legacy FIRE  */
        case 2u: return (uint8_t)ELEMENT_EARTH;  /* legacy EARTH */
        case 3u: return (uint8_t)ELEMENT_AIR;    /* legacy AIR   */
        default: return (uint8_t)M_CANONICAL_ELEMENT_INVALID;
    }
}

static inline uint8_t m_canonical_to_m4_h_legacy(uint8_t canonical) {
    switch (canonical) {
        case ELEMENT_WATER: return 0u;
        case ELEMENT_FIRE:  return 1u;
        case ELEMENT_EARTH: return 2u;
        case ELEMENT_AIR:   return 3u;
        default:            return (uint8_t)M_CANONICAL_ELEMENT_INVALID; /* AETHER/SALT */
    }
}

/* ===================================================================
 * M2-2 MAHABHUTA REGISTER  ⇄  ALCHEMICAL (a correspondence, not a cast)
 *   mahabhuta (== m2.h Element_Id, tattvas 31..35):
 *     AKASHA=0, VAYU/AIR=1, AGNI/FIRE=2, APAS/WATER=3, PRITHVI/EARTH=4
 *   (also the ordering the kairos python adapter emits)
 *   The `medicine_rs_legacy` names are historical — this is M2-2's own series.
 * =================================================================== */

static inline uint8_t m_canonical_from_medicine_rs_legacy(uint8_t legacy) {
    switch (legacy) {
        case 0u: return (uint8_t)ELEMENT_AETHER; /* AKASHA */
        case 1u: return (uint8_t)ELEMENT_AIR;    /* VAYU   */
        case 2u: return (uint8_t)ELEMENT_FIRE;   /* AGNI   */
        case 3u: return (uint8_t)ELEMENT_WATER;  /* APAS   */
        case 4u: return (uint8_t)ELEMENT_EARTH;  /* PRITHVI */
        default: return (uint8_t)M_CANONICAL_ELEMENT_INVALID;
    }
}

static inline uint8_t m_canonical_to_medicine_rs_legacy(uint8_t canonical) {
    switch (canonical) {
        case ELEMENT_AETHER: return 0u; /* AKASHA */
        case ELEMENT_AIR:    return 1u; /* VAYU   */
        case ELEMENT_FIRE:   return 2u; /* AGNI   */
        case ELEMENT_WATER:  return 3u; /* APAS   */
        case ELEMENT_EARTH:  return 4u; /* PRITHVI */
        default:             return (uint8_t)M_CANONICAL_ELEMENT_INVALID; /* SALT */
    }
}

/* ===================================================================
 * M2-3 BIMBA BRANCH ORDERING  ⇄  ALCHEMICAL
 *   branch: 0=Aether, 1=Fire, 2=Earth, 3=Air, 4=Water, 5=Salt
 *   (the #2-3-{1..4} triplicity coordinate convention — full bijection)
 *
 * NOTE: this maps the *element semantics* of the branch convention; the
 * #2-3-N coordinate strings themselves are graph addresses and are never
 * rewritten by element migration.
 * =================================================================== */

static inline uint8_t m_canonical_from_m2_3_branch(uint8_t branch) {
    switch (branch) {
        case 0u: return (uint8_t)ELEMENT_AETHER;
        case 1u: return (uint8_t)ELEMENT_FIRE;
        case 2u: return (uint8_t)ELEMENT_EARTH;
        case 3u: return (uint8_t)ELEMENT_AIR;
        case 4u: return (uint8_t)ELEMENT_WATER;
        case 5u: return (uint8_t)ELEMENT_SALT;
        default: return (uint8_t)M_CANONICAL_ELEMENT_INVALID;
    }
}

static inline uint8_t m_canonical_to_m2_3_branch(uint8_t canonical) {
    switch (canonical) {
        case ELEMENT_AETHER: return 0u;
        case ELEMENT_FIRE:   return 1u;
        case ELEMENT_EARTH:  return 2u;
        case ELEMENT_AIR:    return 3u;
        case ELEMENT_WATER:  return 4u;
        case ELEMENT_SALT:   return 5u;
        default:             return (uint8_t)M_CANONICAL_ELEMENT_INVALID;
    }
}

#endif /* M_CANONICAL_H */
