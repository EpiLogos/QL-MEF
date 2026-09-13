#ifndef QL_COUPLED_CLOCK_H
#define QL_COUPLED_CLOCK_H
#include "ql/m_tree_live.h"
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
#define QL_COUPLED_CLOCK_VERSION "ql.coupled-clock/v1"
#define QL_PHASE_HALF_DEGREES_PER_TURN 720u
#define QL_CLOCK_MAX_RATE 1000000u
#define QL_CLOCK_MAX_DRIVER_STEP INT64_C(1000000000)
typedef enum { QL_CLOCK_OK=0, QL_CLOCK_INVALID=1, QL_CLOCK_OVERFLOW=2,
               QL_CLOCK_STALE=3, QL_CLOCK_NO_ALIGNMENT=4 } QL_ClockResult;
/* The integer winding is not folded into a floating angle. Negative winding,
 * orientation within one turn, and the 720-degree cover remain distinct. */
typedef struct { int64_t turns; uint16_t half_degrees; } QL_PhaseLift;
typedef struct {
    QL_PhaseLift inscription, lensing;
    /* Fibonacci / elemental / void origin offsets; never snapped to zero. */
    uint16_t grid_origin_half_degrees[3];
    int32_t rate_numerators[2];
    uint32_t rate_denominator;
    int64_t rate_remainders[2];
    uint64_t generation;
} QL_CoupledClock;
QL_ClockResult ql_phase_add(QL_PhaseLift phase, int64_t half_degrees, QL_PhaseLift *out);
uint16_t ql_phase_double_cover_half_degrees(QL_PhaseLift phase);
QL_ClockResult ql_clock_validate(const QL_CoupledClock *clock);
QL_ClockResult ql_clock_set_axis(const QL_CoupledClock *clock, uint64_t expected,
                               unsigned axis, QL_PhaseLift phase, QL_CoupledClock *out);
QL_ClockResult ql_clock_set_trajectory(const QL_CoupledClock *clock, uint64_t expected,
                                     int32_t inscription_rate, int32_t lensing_rate,
                                     uint32_t denominator, QL_CoupledClock *out);
/* Driver increments are bounded integers in half-degree units before the
 * rational rate. Remainders survive partitioned calls and direction reversal.
 * All setters are transactional, including when out aliases clock. */
QL_ClockResult ql_clock_advance(const QL_CoupledClock *clock, uint64_t expected,
                              int64_t driver_half_degrees, QL_CoupledClock *out);
uint16_t ql_m2_grid_quantum(unsigned grid);
uint16_t ql_m2_grid_closure(unsigned grid_a, unsigned grid_b);
QL_ClockResult ql_m2_grid_alignment(unsigned grid_a, unsigned grid_b,
                                  uint16_t origin_a, uint16_t origin_b,
                                  uint16_t *first_half_degree, uint16_t *period_half_degrees);
const QL_M_Node *ql_clock_field(void);
const QL_M_Node *ql_clock_centre(void);
#ifdef __cplusplus
}
#endif
#endif
