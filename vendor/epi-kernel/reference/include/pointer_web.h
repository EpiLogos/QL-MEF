/**
 * pointer_web.h -- Derived harmonic pointer web contract.
 *
 * The hot Holographic_Coordinate remains 128 bytes. This module exposes the
 * complete 36-fold relation field as a typed derived view:
 *
 *   36 = 12 family ring + 12 QL-position ring + 12 MEF-lens ring
 *
 * CF7 is exposed as a separate diatonic/context-frame overlay.
 */

#ifndef POINTER_WEB_H
#define POINTER_WEB_H

#include "arena.h"
#include "psychoid_numbers.h"
#include <stdint.h>

#define HC_WEB_RING_COUNT 3u
#define HC_WEB_RING_SIZE 12u
#define HC_WEB_HELIX_SIZE 6u
#define HC_BEDROCK_POSITION_COUNT 6u

typedef enum {
    HC_POINTER_RING_FAMILY   = 0,
    HC_POINTER_RING_POSITION = 1,
    HC_POINTER_RING_LENS     = 2
} HC_PointerRing;

typedef enum {
    HC_HELIX_BIMBA      = 0,
    HC_HELIX_PRATIBIMBA = 1,
    HC_HELIX_CROSS      = 2
} HC_Helix;

typedef enum {
    HC_REL_FAMILY_LINK       = 0,
    HC_REL_POSITION_IDENTITY = 1,
    HC_REL_INVERSION_SPANDA  = 2,
    HC_REL_EPOGDOON_TICK     = 3,
    HC_REL_MIRROR_XY5        = 4,
    HC_REL_MOBIUS_RETURN     = 5,
    HC_REL_POSITION_PROJECT  = 6,
    HC_REL_LENS_ANCHOR       = 7,
    HC_REL_CONTEXT_FRAME     = 8
} HC_RelationRole;

typedef enum {
    HC_INTERVAL_NONE          = 0,
    HC_INTERVAL_SEMITONE      = 1,
    HC_INTERVAL_WHOLE_TONE    = 2,
    HC_INTERVAL_TRITONE       = 6,
    HC_INTERVAL_TOTALITY_16_9 = 10,
    HC_INTERVAL_OCTAVE        = 12
} HC_IntervalRole;

typedef enum {
    HC_RATIO_NONE      = 0,
    HC_RATIO_UNISON    = 1,
    HC_RATIO_EPOGDOON  = 2,
    HC_RATIO_FOURTH    = 3,
    HC_RATIO_FIFTH     = 4,
    HC_RATIO_TOTALITY  = 5,
    HC_RATIO_OCTAVE    = 6
} HC_RatioRole;

typedef enum {
    HC_BEDROCK_HASH_OPERATOR     = 0,
    HC_BEDROCK_PSYCHOID_NUMBER   = 1,
    HC_BEDROCK_INVERTED_PSYCHOID = 2
} HC_BedrockRole;

typedef struct {
    const Holographic_Coordinate* target;
    uint8_t index;
    uint8_t ql_position;
    uint8_t bedrock_role;
    uint8_t relation_role;
    uint8_t interval_role;
    uint8_t ratio_role;
    uint8_t pitch_class;
} HC_BedrockRef;

typedef struct {
    HC_BedrockRef hash;
    HC_BedrockRef psychoid[HC_BEDROCK_POSITION_COUNT];
    HC_BedrockRef successor[HC_BEDROCK_POSITION_COUNT];
    HC_BedrockRef inversion[HC_BEDROCK_POSITION_COUNT];
} HC_BedrockWeb7;

typedef struct {
    Holographic_Coordinate* target;
    uint8_t ring;
    uint8_t index;
    uint8_t ql_position;
    uint8_t helix;
    uint8_t relation_role;
    uint8_t interval_role;
    uint8_t ratio_role;
    uint8_t pitch_class;
} HC_PointerRef;

typedef struct {
    HC_PointerRef family[HC_WEB_RING_SIZE];
    HC_PointerRef position[HC_WEB_RING_SIZE];
    HC_PointerRef lens[HC_WEB_RING_SIZE];
} HC_PointerWeb36;

typedef struct {
    const Holographic_Coordinate* target;
    uint8_t cf_index;
    const char* notation;
    uint8_t diatonic_degree;
    uint8_t mode_anchor;
    uint8_t ql_position;
    uint8_t helix;
    uint8_t relation_role;
    uint8_t pitch_class;
} HC_ContextFrameRef;

typedef struct {
    HC_ContextFrameRef frame[CF_COUNT];
} HC_ContextFrameWeb7;

uint8_t hc_bimba_pitch_class(uint8_t ql_position);
uint8_t hc_pratibimba_pitch_class(uint8_t ql_position);
uint8_t hc_lens_bimba_pitch_class(uint8_t lens_anchor_pc, uint8_t ql_position);
uint8_t hc_lens_pratibimba_pitch_class(uint8_t lens_anchor_pc, uint8_t ql_position);
uint8_t hc_mirror_position(uint8_t ql_position);
uint8_t hc_mirror_interval_role(uint8_t ql_position);
uint8_t hc_mirror_ratio_role(uint8_t ql_position);

int hc_bedrock_web7_fill(HC_BedrockWeb7* out);

int hc_pointer_web36_fill(
    const Coordinate_Arena* arena,
    const Holographic_Coordinate* source,
    HC_PointerWeb36* out
);

int hc_context_frame_web7_fill(HC_ContextFrameWeb7* out);

#endif /* POINTER_WEB_H */
