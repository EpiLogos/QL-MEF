/**
 * arena.c — The Arena Allocator
 *
 * Physical Torus in memory. 128-byte aligned slots.
 */

#include "arena.h"
#include <stdlib.h>
#include <string.h>

int arena_init(Coordinate_Arena* arena, uint32_t capacity) {
    if (!arena || capacity == 0) return -1;
    arena->slots = aligned_alloc(128, (size_t)capacity * sizeof(Holographic_Coordinate));
    if (!arena->slots) return -1;
    memset(arena->slots, 0, (size_t)capacity * sizeof(Holographic_Coordinate));
    arena->capacity = capacity;
    arena->count = 0;
    return 0;
}

Holographic_Coordinate* arena_alloc(Coordinate_Arena* arena) {
    if (!arena || arena->count >= arena->capacity) return NULL;
    Holographic_Coordinate* slot = &arena->slots[arena->count];
    arena->count++;
    return slot;
}

void arena_reset(Coordinate_Arena* arena) {
    if (!arena) return;
    memset(arena->slots, 0, (size_t)arena->capacity * sizeof(Holographic_Coordinate));
    arena->count = 0;
}

void arena_destroy(Coordinate_Arena* arena) {
    if (!arena) return;
    free(arena->slots);
    arena->slots = NULL;
    arena->capacity = 0;
    arena->count = 0;
}

int tensor_arena_init(Tensor_Arena* arena, uint32_t capacity) {
    if (!arena || capacity == 0) return -1;
    arena->buffer = aligned_alloc(64, (size_t)capacity * sizeof(float));
    if (!arena->buffer) return -1;
    memset(arena->buffer, 0, (size_t)capacity * sizeof(float));
    arena->capacity = capacity;
    arena->offset = 0;
    return 0;
}

float* tensor_arena_alloc(Tensor_Arena* arena, uint32_t count) {
    if (!arena || arena->offset + count > arena->capacity) return NULL;
    float* ptr = &arena->buffer[arena->offset];
    arena->offset += count;
    return ptr;
}

void tensor_arena_reset(Tensor_Arena* arena) {
    if (!arena) return;
    memset(arena->buffer, 0, (size_t)arena->capacity * sizeof(float));
    arena->offset = 0;
}

void tensor_arena_destroy(Tensor_Arena* arena) {
    if (!arena) return;
    free(arena->buffer);
    arena->buffer = NULL;
    arena->capacity = 0;
    arena->offset = 0;
}
