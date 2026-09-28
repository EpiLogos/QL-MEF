/* The Spanda field (T2.11) executed natively in C, one JSON line per
 * observation, for field-for-field comparison with ql_mef::spanda_field.
 */
#include "m1.h"
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
int main(void) {
    Spanda_HKB_Params p = spanda_hkb_params_default();
    printf("{\"kind\":\"params\",\"delta_omega\":%.17g,\"a\":%.17g,\"b\":%.17g,\"base_freq_hz\":%.17g}\n",
           p.delta_omega, p.a, p.b, p.base_freq_hz);
    for (int i = -48; i <= 48; ++i) {
        double phi = (double)i * M_PI / 24.0;
        printf("{\"kind\":\"hkb\",\"phi\":%.17g,\"drift\":%.17g,\"potential\":%.17g,\"curvature\":%.17g,"
               "\"settle\":%.17g,\"tick12\":%u}\n",
               phi, spanda_hkb_drift(phi, &p), spanda_hkb_potential(phi, &p),
               spanda_hkb_curvature(phi, &p), spanda_hkb_settle(phi, 0.01, 2000u, &p),
               spanda_tick12_readout(phi));
    }
    for (int i = 0; i < 24; ++i) {
        double x = (double)i * M_PI / 12.0, t = (double)i * 0.37;
        for (int s = 0; s < 2; ++s)
            printf("{\"kind\":\"wave\",\"x\":%.17g,\"t\":%.17g,\"swapped\":%s,\"pole0\":%.17g,\"pole1\":%.17g,"
                   "\"superposition\":%.17g,\"envelope\":%.17g}\n",
                   x, t, s ? "true" : "false", spanda_pole_wave(x, t, 0u, s), spanda_pole_wave(x, t, 1u, s),
                   spanda_superposition(x, t, s), spanda_standing_envelope(x, s));
    }
    for (uint8_t n = 0; n < 12; ++n)
        printf("{\"kind\":\"half_turn\",\"n\":%u,\"value\":%u}\n", n, spanda_half_turn_index(n));
    /* Rotations about the i axis across the full 720°, both SU(2) signs, and
     * off-axis states, over two full lens cycles. */
    for (int k = 0; k < 48; ++k) {
        float h = (float)((double)k * M_PI / 24.0);
        float offs[3] = {0.0f, 0.3f, -0.7f};
        for (int o = 0; o < 3; ++o) {
            Quaternion q = {cosf(h), sinf(h), offs[o], offs[o] * 0.5f};
            for (uint64_t cycle = 0; cycle < 12; ++cycle)
                printf("{\"kind\":\"codon\",\"q\":[%.9g,%.9g,%.9g,%.9g],\"cycle\":%" PRIu64 ",\"codon\":%u}\n",
                       (double)q.w, (double)q.x, (double)q.y, (double)q.z, cycle, spanda_codon_advance(q, cycle));
        }
    }
    return 0;
}
