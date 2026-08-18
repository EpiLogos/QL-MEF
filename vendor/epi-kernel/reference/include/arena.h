/**
 * arena.h — The Arena Allocator API
 *
 * The physical Torus in memory. Allocates 128-byte aligned slots
 * for mutable coordinate instances (Shakti/Heap).
 */

#ifndef ARENA_H
#define ARENA_H

#include "ontology.h"

/* =============================================================================
 * I. ARENA STRUCT
 * ============================================================================= */

typedef struct {
    Holographic_Coordinate* slots;   /* aligned_alloc'd buffer           */
    uint32_t capacity;               /* max number of slots              */
    uint32_t count;                  /* current occupancy                */
} Coordinate_Arena;

/* =============================================================================
 * II. ARENA API
 * ============================================================================= */

/* Initialize arena with given capacity. Returns 0 on success, -1 on failure. */
int arena_init(Coordinate_Arena* arena, uint32_t capacity);

/* Allocate the next slot. Returns pointer to slot, or NULL if full. */
Holographic_Coordinate* arena_alloc(Coordinate_Arena* arena);

/* Reset arena (mark all slots free, zero memory). */
void arena_reset(Coordinate_Arena* arena);

/* Destroy arena (free underlying memory). */
void arena_destroy(Coordinate_Arena* arena);

/* =============================================================================
 * III. TENSOR ARENA
 * ============================================================================= */

typedef struct {
    float* buffer;        /* SIMD-aligned float buffer  */
    uint32_t capacity;    /* total floats               */
    uint32_t offset;      /* next free position         */
} Tensor_Arena;

int tensor_arena_init(Tensor_Arena* arena, uint32_t capacity);
float* tensor_arena_alloc(Tensor_Arena* arena, uint32_t count);
void tensor_arena_reset(Tensor_Arena* arena);
void tensor_arena_destroy(Tensor_Arena* arena);

/* =============================================================================
 * IV. FAMILY OPERATIONS
 * ============================================================================= */

int families_init(Coordinate_Arena* arena, Holographic_Coordinate* mirrors[6]);
int families_crosslink(Coordinate_Arena* arena);
int families_wire_reflective(Coordinate_Arena* arena);


#endif /* ARENA_H */
