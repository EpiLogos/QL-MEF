#include "kernel.h"

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

static void fail(const char* message) {
    fprintf(stderr, "epi C reference smoke failure: %s\n", message);
    exit(EXIT_FAILURE);
}

static void expect_close(float actual, float expected, float tolerance, const char* message) {
    if (fabsf(actual - expected) > tolerance) {
        fprintf(stderr, "%s: expected %.9f, got %.9f\n", message, expected, actual);
        exit(EXIT_FAILURE);
    }
}

int main(void) {
    static const float expected_ratios[12] = {
        1.0f,
        4.0f / 3.0f,
        3.0f / 4.0f,
        9.0f / 8.0f,
        1.0f,
        2.0f / 3.0f,
        2.0f / 3.0f,
        3.0f / 4.0f,
        9.0f / 8.0f,
        3.0f / 2.0f,
        3.0f / 2.0f,
        9.0f / 8.0f
    };

    expect_close(kernel_epogdoon_ratio(), 9.0f / 8.0f, 1e-6f, "epogdoon ratio");
    expect_close(kernel_ratio_ascending_fourth(), 4.0f / 3.0f, 1e-6f, "ascending fourth");
    expect_close(kernel_ratio_descending_fourth(), 3.0f / 4.0f, 1e-6f, "descending fourth");
    expect_close(kernel_ratio_descending_fifth(), 2.0f / 3.0f, 1e-6f, "descending fifth");
    expect_close(kernel_ratio_ascending_fifth(), 3.0f / 2.0f, 1e-6f, "ascending fifth");

    for (uint8_t tick = 0; tick < 12; tick++) {
        Kernel_Tick state = kernel_tick_from_epogdoon(17u, tick);
        if (state.cycle != 17u) fail("tick cycle was not preserved");
        if (state.sub_tick != tick) fail("sub-tick identity changed");
        if (state.position6 != (uint8_t)(tick % 6u)) fail("position6 projection changed");
        if (state.phase != (tick < 6u ? KERNEL_PHASE_DESCENT : KERNEL_PHASE_ASCENT)) {
            fail("tick phase changed");
        }
        expect_close(state.harmonic_ratio, expected_ratios[tick], 1e-6f, "tick harmonic ratio");
    }

    Kernel_Tick wrapped = kernel_tick_from_epogdoon(3u, 25u);
    if (wrapped.sub_tick != 1u || wrapped.position6 != 1u) fail("12-fold tick wrapping changed");

    uint8_t seen[72] = {0};
    for (uint8_t lens = 0; lens < 6; lens++) {
        for (uint8_t helix = 0; helix < 2; helix++) {
            for (uint8_t position = 0; position < 6; position++) {
                uint8_t idx = kernel_resonance_index(lens, helix, position);
                if (idx >= 72u) fail("resonance index escaped 72-space");
                if (seen[idx]) fail("resonance index collision");
                seen[idx] = 1u;
            }
        }
    }
    for (uint8_t idx = 0; idx < 72; idx++) {
        if (!seen[idx]) fail("resonance 72-space is not exhaustive");
    }
    if (kernel_resonance_index(6u, 0u, 0u) != 0xFFu) fail("invalid lens must reject");
    if (kernel_resonance_index(0u, 2u, 0u) != 0xFFu) fail("invalid helix must reject");
    if (kernel_resonance_index(0u, 0u, 6u) != 0xFFu) fail("invalid position must reject");

    static const uint8_t expected_squares[6] = {0u, 1u, 2u, 2u, 1u, 0u};
    for (uint8_t lens = 0; lens < 6; lens++) {
        if (kernel_tritone_square_for_lens(lens) != expected_squares[lens]) {
            fail("tritone square grouping changed");
        }
    }

    puts("epi C reference smoke: ok");
    return EXIT_SUCCESS;
}
