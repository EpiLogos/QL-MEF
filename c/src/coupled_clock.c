#include "ql/coupled_clock.h"
#include <limits.h>
#include <stddef.h>

static int add_i64(int64_t a,int64_t b,int64_t *out) {
    if ((b>0 && a>INT64_MAX-b) || (b<0 && a<INT64_MIN-b)) return 0;
    *out=a+b; return 1;
}
static int phase_valid(QL_PhaseLift p) { return p.half_degrees<720u; }
QL_ClockResult ql_phase_add(QL_PhaseLift p,int64_t delta,QL_PhaseLift *out) {
    QL_PhaseLift n=p;
    int64_t h=(int64_t)p.half_degrees+delta%720;
    if (!out || !phase_valid(p)) return QL_CLOCK_INVALID;
    if (!add_i64(p.turns,delta/720,&n.turns)) return QL_CLOCK_OVERFLOW;
    if (h<0) { h+=720; if (!add_i64(n.turns,-1,&n.turns)) return QL_CLOCK_OVERFLOW; }
    if (h>=720) { h-=720; if (!add_i64(n.turns,1,&n.turns)) return QL_CLOCK_OVERFLOW; }
    n.half_degrees=(uint16_t)h; *out=n; return QL_CLOCK_OK;
}
uint16_t ql_phase_double_cover_half_degrees(QL_PhaseLift p) {
    if (!phase_valid(p)) return UINT16_MAX;
    return (uint16_t)(((p.turns%2+2)%2)*720+p.half_degrees);
}
QL_ClockResult ql_clock_validate(const QL_CoupledClock *c) {
    if (!c || !phase_valid(c->inscription) || !phase_valid(c->lensing)
        || c->rate_denominator==0 || c->rate_denominator>QL_CLOCK_MAX_RATE) return QL_CLOCK_INVALID;
    for (unsigned i=0;i<3;++i) if (c->grid_origin_half_degrees[i]>=720) return QL_CLOCK_INVALID;
    for (unsigned i=0;i<2;++i) {
        if (c->rate_numerators[i] > (int32_t)QL_CLOCK_MAX_RATE
            || c->rate_numerators[i] < -(int32_t)QL_CLOCK_MAX_RATE
            || c->rate_remainders[i] < 0
            || c->rate_remainders[i] >= (int64_t)c->rate_denominator) return QL_CLOCK_INVALID;
    }
    return QL_CLOCK_OK;
}
static QL_ClockResult begin(const QL_CoupledClock *c,uint64_t expected,QL_CoupledClock *out) {
    if (!out || ql_clock_validate(c)!=QL_CLOCK_OK) return QL_CLOCK_INVALID;
    if (expected!=c->generation) return QL_CLOCK_STALE;
    if (c->generation==UINT64_MAX) return QL_CLOCK_OVERFLOW;
    return QL_CLOCK_OK;
}
QL_ClockResult ql_clock_set_axis(const QL_CoupledClock *c,uint64_t expected,unsigned axis,
                               QL_PhaseLift p,QL_CoupledClock *out) {
    QL_ClockResult r=begin(c,expected,out);
    if (r!=QL_CLOCK_OK) return r;
    if (axis>1 || !phase_valid(p)) return QL_CLOCK_INVALID;
    QL_CoupledClock n=*c;
    if (axis==0) n.inscription=p; else n.lensing=p;
    n.rate_remainders[axis]=0; ++n.generation; *out=n; return QL_CLOCK_OK;
}
QL_ClockResult ql_clock_set_trajectory(const QL_CoupledClock *c,uint64_t expected,
                                     int32_t a,int32_t b,uint32_t den,QL_CoupledClock *out) {
    QL_ClockResult r=begin(c,expected,out);
    if (r!=QL_CLOCK_OK) return r;
    QL_CoupledClock n=*c;
    n.rate_numerators[0]=a; n.rate_numerators[1]=b; n.rate_denominator=den;
    /* A rate change defines a new rational trajectory at the same lifted
     * phases. Fractional residue belongs to the old rate, not the new one. */
    n.rate_remainders[0]=0; n.rate_remainders[1]=0;
    if (ql_clock_validate(&n)!=QL_CLOCK_OK) return QL_CLOCK_INVALID;
    ++n.generation; *out=n; return QL_CLOCK_OK;
}
QL_ClockResult ql_clock_advance(const QL_CoupledClock *c,uint64_t expected,int64_t step,QL_CoupledClock *out) {
    QL_ClockResult r=begin(c,expected,out);
    if (r!=QL_CLOCK_OK) return r;
    if (step < -QL_CLOCK_MAX_DRIVER_STEP || step > QL_CLOCK_MAX_DRIVER_STEP) return QL_CLOCK_INVALID;
    QL_CoupledClock n=*c;
    for (unsigned i=0;i<2;++i) {
        /* The documented bounds keep this product and sum within int64. */
        int64_t scaled=step*(int64_t)n.rate_numerators[i]+n.rate_remainders[i];
        int64_t whole=scaled/(int64_t)n.rate_denominator;
        int64_t rem=scaled%(int64_t)n.rate_denominator;
        if (rem<0) { rem+=n.rate_denominator; --whole; }
        r=ql_phase_add(i==0?n.inscription:n.lensing,whole,
                       i==0?&n.inscription:&n.lensing);
        if (r!=QL_CLOCK_OK) return r;
        n.rate_remainders[i]=rem;
    }
    ++n.generation; *out=n; return QL_CLOCK_OK;
}
static uint16_t gcd(uint16_t a,uint16_t b) {
    while (b) { uint16_t r=(uint16_t)(a%b); a=b; b=r; } return a;
}
uint16_t ql_m2_grid_quantum(unsigned grid) {
    static const uint16_t quanta[3]={12,40,45}; return grid<3?quanta[grid]:0;
}
uint16_t ql_m2_grid_closure(unsigned a,unsigned b) {
    uint16_t x=ql_m2_grid_quantum(a),y=ql_m2_grid_quantum(b);
    return x&&y?(uint16_t)((x/gcd(x,y))*y):0;
}
QL_ClockResult ql_m2_grid_alignment(unsigned a,unsigned b,uint16_t oa,uint16_t ob,
                                  uint16_t *first,uint16_t *period) {
    uint16_t x=ql_m2_grid_quantum(a),y=ql_m2_grid_quantum(b),lcm=ql_m2_grid_closure(a,b);
    if (!x || !y || oa>=720 || ob>=720 || !first || !period) return QL_CLOCK_INVALID;
    for (uint16_t i=0;i<lcm;++i) {
        if (i%x==oa%x && i%y==ob%y) { *first=i; *period=lcm; return QL_CLOCK_OK; }
    }
    return QL_CLOCK_NO_ALIGNMENT;
}
const QL_M_Node *ql_clock_field(void) { return ql_m_live_resolve("#3-0"); }
const QL_M_Node *ql_clock_centre(void) { return ql_m_live_resolve("#3-5-5/0"); }
