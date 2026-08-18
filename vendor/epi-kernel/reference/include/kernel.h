/**
 * kernel.h — Epi-Logos bioquaternionic kernel
 *
 * The QL/MEF computational meta-layer over the M-family C substrate.
 * This file exposes the first real kernel operator primitives: harmonic
 * constants, bioquaternion state, 12-epogdoon tick phasing, 72-fold resonance
 * indexing, slash-flip conjugation, and decomposed energy evaluation.
 */

#ifndef KERNEL_H
#define KERNEL_H

#include "m1.h"
#include <stdint.h>

#define KERNEL_EPOGDOON_NUM 9u
#define KERNEL_EPOGDOON_DEN 8u
#define KERNEL_RESONANCE_DIM 72u
#define KERNEL_TRITONE_SQUARES 3u

typedef enum {
    KERNEL_PHASE_DESCENT = 0,
    KERNEL_PHASE_ASCENT  = 1
} Kernel_Phase;

typedef enum {
    KERNEL_ELEMENT_BIMBA_ENCODING        = 0,
    KERNEL_ELEMENT_PRATIBIMBA_PREHENSION = 1,
    KERNEL_ELEMENT_MOBIUS_DESCENT        = 2,
    KERNEL_ELEMENT_SLASH_FLIP            = 3,
    KERNEL_ELEMENT_PRATIBIMBA_AS_BIMBA   = 4,
    KERNEL_ELEMENT_DOUBLED_PREHENSION    = 5,
    KERNEL_ELEMENT_INVERSE_MOBIUS        = 6,
    KERNEL_ELEMENT_ENRICHED_RETURN       = 7
} Kernel_Element;

typedef struct {
    Quaternion q_b;
    Quaternion q_p;
} Kernel_Bioquaternion;

typedef struct {
    float values[KERNEL_RESONANCE_DIM];
} Kernel_Resonance_Vector;

typedef struct {
    float bimba_pratibimba_energy;
    float lens_energy;
    float r_energy;
    float total_energy;
} Kernel_Energy;

typedef struct {
    uint64_t cycle;
    uint8_t sub_tick;
    Kernel_Phase phase;
    Kernel_Element element;
    uint8_t position6;
    float harmonic_ratio;
} Kernel_Tick;

float kernel_epogdoon_ratio(void);
float kernel_epogdoon_log(void);
float kernel_ratio_ascending_fourth(void);
float kernel_ratio_descending_fourth(void);
float kernel_ratio_descending_fifth(void);
float kernel_ratio_ascending_fifth(void);

Kernel_Bioquaternion kernel_bioquaternion_init(Quaternion q_b, Quaternion q_p);
float kernel_quat_distance_sq(Quaternion a, Quaternion b);
Quaternion kernel_slash_flip_bimba_prime(Kernel_Bioquaternion state);

uint8_t kernel_resonance_index(uint8_t lens, uint8_t helix, uint8_t position);
uint8_t kernel_tritone_square_for_lens(uint8_t lens);
void kernel_resonance_square_emphasis(
    const Kernel_Resonance_Vector* vector,
    float out_square_emphasis[KERNEL_TRITONE_SQUARES]
);

Kernel_Energy kernel_energy_evaluate(
    Kernel_Bioquaternion state,
    const Kernel_Resonance_Vector* observed,
    const Kernel_Resonance_Vector* target,
    float r_energy
);

Kernel_Tick kernel_tick_from_epogdoon(uint64_t cycle, uint8_t sub_tick);

#endif /* KERNEL_H */
