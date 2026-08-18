/**
 * vak.h — The VAK Instruction Set (Reflective Coordinate Operations)
 *
 * VAK instructions operationalize the 6 reflective coordinates:
 *   cpf, ct, cp, cf, cfp, cs
 *
 * These are UNIVERSAL — any M-branch or subsystem can issue and receive
 * VAK instructions. M0 (Anuttara) defines the semantic handlers for what
 * each family does when executed; the dispatch infrastructure lives here.
 *
 * Architecture:
 *   VAK_Instruction is a 5-byte struct parsed from coordinate strings.
 *   execute_vak_instruction() dispatches by integer family index (0-5).
 *   Each family maps 1:1 to a reflective coordinate pointer field.
 *   No string comparisons on the execution path.
 *
 * VAK Family → Reflective Coordinate Mapping:
 *   0 CPF → cpf (Category-Position-Frame)
 *   1 CT  → ct  (Context-Time / Content Types)
 *   2 CP  → cp  (Context-Position)
 *   3 CF  → cf  (Context-Frame)
 *   4 CFP → cfp (Context-Frame-Position / Paths)
 *   5 CS  → cs  (Context-Sequence)
 */

#ifndef VAK_H
#define VAK_H

#include "ontology.h"

/* ===================================================================
 * I. VAK FAMILY CONSTANTS — index into reflective coordinate space
 * =================================================================== */

#define VAK_FAMILY_CPF   0u   /* cpf — Category-Position-Frame       */
#define VAK_FAMILY_CT    1u   /* ct  — Context-Time / Content Types   */
#define VAK_FAMILY_CP    2u   /* cp  — Context-Position               */
#define VAK_FAMILY_CF    3u   /* cf  — Context-Frame                  */
#define VAK_FAMILY_CFP   4u   /* cfp — Context-Frame-Position / Paths */
#define VAK_FAMILY_CS    5u   /* cs  — Context-Sequence               */
#define VAK_FAMILY_COUNT 6u

/* Human-readable names — Obsidian/CLI layer ONLY */
static const char* const VAK_FAMILY_NAMES[VAK_FAMILY_COUNT] = {
    "CPF", "CT", "CP", "CF", "CFP", "CS"
};

/* ===================================================================
 * II. VAK INSTRUCTION — 5-byte parsed instruction
 * =================================================================== */

typedef struct {
    uint8_t  vak_family;     /* 0-5: which reflective coordinate      */
    uint8_t  vak_index;      /* coordinate's own index within family   */
    uint8_t  target_branch;  /* which M-branch (0-5) to target        */
    uint8_t  target_pos;     /* position within that branch            */
    uint8_t  is_inverted;    /* 1 if prime (') suffix present          */
} VAK_Instruction;

_Static_assert(sizeof(VAK_Instruction) == 5, "VAK_Instruction must be 5 bytes");

/* ===================================================================
 * III. RETURN CODES
 * =================================================================== */

#define VAK_SUCCESS      0
#define VAK_ERR_FAMILY  (-1)
#define VAK_ERR_INDEX   (-2)
#define VAK_ERR_SESSION (-3)

/* ===================================================================
 * IV. VAK SEMANTIC HANDLER — function pointer type
 *
 * M-branch modules register handlers for each VAK family.
 * The default handler returns VAK_ERR_FAMILY (unimplemented).
 * =================================================================== */

typedef int (*VAK_Handler)(Holographic_Coordinate* session,
                           VAK_Instruction instr);

/* ===================================================================
 * V. VAK DISPATCH — engine-level dispatcher
 * =================================================================== */

/**
 * execute_vak_instruction — dispatch a VAK instruction on a session HC.
 *
 * Looks up the handler for instr.vak_family and calls it.
 * Returns VAK_SUCCESS or VAK_ERR_* code.
 *
 * Session must not be NULL (returns VAK_ERR_SESSION).
 * Family must be 0-5 (returns VAK_ERR_FAMILY).
 */
int execute_vak_instruction(Holographic_Coordinate* session,
                            VAK_Instruction instr);

/**
 * vak_register_handler — register a semantic handler for a VAK family.
 *
 * Called by M-branch init functions to wire their semantics into VAK.
 * E.g., M0 registers handlers for what CPF/CT/CP/CF/CFP/CS mean
 * within the Anuttara VM layer.
 */
void vak_register_handler(uint8_t family, VAK_Handler handler);

#endif /* VAK_H */
