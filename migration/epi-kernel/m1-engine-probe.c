/* Actual native execution versus the immutable accepted C Ananda return.
 * Output is compared field-for-field with independent Rust execution.
 */
#include "ql/m1.h"
#include "m1_ananda_projection.h"
#include <assert.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
static void clock_json(const QL_M1_Clock *c) {
    printf("{\"cycle\":\"%" PRIu64 "\",\"tick12\":%u,\"position6\":%u,\"phase\":%u,"
        "\"conjugate_tick12\":%u,\"conjugate_phase\":%u,\"degree360\":%u,"
        "\"hopf_fiber\":%u,\"degree720\":%u,\"spanda_stage\":%u}",
        c->cycle,c->tick12,c->position6,c->phase,c->conjugate_tick12,c->conjugate_phase,
        c->degree360,c->hopf_fiber,c->degree720,c->spanda_stage);
}
static void cell_json(const QL_M1_Cell *c) {
    printf("{\"coordinate\":\"%016" PRIx64 "\",\"dr_coordinate\":\"%016" PRIx64 "\","
        "\"family\":%u,\"row12\":%u,\"col12\":%u,\"scalar_valid\":%s,\"decimal10_valid\":%s,\"raw\":",
        c->coordinate,c->dr_coordinate,c->family,c->row12,c->col12,
        c->scalar_valid ? "true":"false",c->decimal10_valid ? "true":"false");
    if(c->scalar_valid) printf("%d",c->raw); else printf("null");
    printf(",\"digit_root\":");
    if(c->scalar_valid) printf("%u",c->digit_root); else printf("null");
    printf(",\"decimal10\":");
    if(c->scalar_valid && c->decimal10_valid) printf("%u",c->decimal10); else printf("null");
    printf(",\"raw_terms\":[%d,%d,%d,%d,%d],\"dr_terms\":[%u,%u,%u,%u,%u],\"decimal_terms\":",
        c->raw_terms[0],c->raw_terms[1],c->raw_terms[2],c->raw_terms[3],c->raw_terms[4],
        c->dr_terms[0],c->dr_terms[1],c->dr_terms[2],c->dr_terms[3],c->dr_terms[4]);
    if(c->decimal10_valid) printf("[%u,%u,%u,%u,%u]",c->decimal_terms[0],c->decimal_terms[1],c->decimal_terms[2],c->decimal_terms[3],c->decimal_terms[4]);
    else printf("null");
    printf(",\"clock\":"); clock_json(&c->clock); puts("}");
}
static void compare_source(const QL_M1_Cell *c) {
    M1_Ananda_Oscillatory_Address address;
    M1_Ananda_Cell_Projection old;
    assert(m1_ananda_oscillatory_address_from_clock(c->clock.cycle,(uint8_t)c->clock.tick12,&address));
    assert(m1_ananda_project_cell((Ananda_Matrix_Op)c->family,(uint8_t)c->row12,(uint8_t)c->col12,&address,&old));
    assert(c->clock.position6==address.position6 && c->clock.phase==(uint32_t)address.phase);
    assert(c->clock.degree360==address.degree360 && c->clock.hopf_fiber==address.hopf_fiber);
    assert(c->clock.degree720==address.degree720 && c->clock.conjugate_tick12==address.conjugate_tick12);
    assert(c->clock.conjugate_phase==(uint32_t)address.conjugate_phase && c->clock.spanda_stage==(uint32_t)address.spanda_stage);
    assert(c->scalar_valid==(uint32_t)old.scalar_valid);
    if(c->scalar_valid) {
        assert(c->raw==old.raw12_value && c->digit_root==old.digit_root12_value);
        assert(c->decimal10_valid==(uint32_t)old.decimal10_valid);
        if(c->decimal10_valid) assert(c->decimal10==old.decimal10_value);
    } else {
        const M1_Ananda_Quintessence_Projection *q=&old.quintessence;
        assert(c->raw_terms[0]==q->bimba_raw && c->raw_terms[1]==q->pratibimba_raw && c->raw_terms[2]==q->sum_raw);
        assert(c->raw_terms[3]==q->difference_a_raw && c->raw_terms[4]==q->difference_b_raw);
        assert(c->dr_terms[0]==q->bimba_dr && c->dr_terms[1]==q->pratibimba_dr && c->dr_terms[2]==q->sum_dr);
        assert(c->dr_terms[3]==q->difference_a_dr && c->dr_terms[4]==q->difference_b_dr);
        assert(c->decimal10_valid==(uint32_t)q->decimal10_valid);
        if(c->decimal10_valid) {
            assert(c->decimal_terms[0]==q->bimba_decimal10 && c->decimal_terms[1]==q->pratibimba_decimal10);
            assert(c->decimal_terms[2]==q->sum_decimal10 && c->decimal_terms[3]==q->difference_a_decimal10);
            assert(c->decimal_terms[4]==q->difference_b_decimal10);
        }
    }
}
int main(void) {
    uint32_t cycle,tick,f,r,c,s;
    QL_M1_Cell value, guard;
    QL_M1_Clock clock;
    QL_M1_Spanda sp;
    QL_M1_Formal form;
    QL_M1_Topology topo;
    QL_M1_Torus torus;
    assert(m1_ananda_verify_dr12_luts());
    assert(!ql_m1_node(NULL) && !ql_m1_node("#2") && !ql_m1_node("#1-4-0"));
    assert(ql_m1_node("M1-4.0")==ql_m1_node("#1-4.0"));
    memset(&guard,0xa5,sizeof(guard)); memcpy(&value,&guard,sizeof(value));
    assert(!ql_m1_cell(256,0,0,0,0,&value) && !memcmp(&value,&guard,sizeof(value)));
    assert(!ql_m1_cell(UINT32_MAX,0,0,0,0,&value));
    assert(!ql_m1_cell(0,12,0,0,0,&value) && !ql_m1_cell(0,0,12,0,0,&value));
    assert(!ql_m1_cell(0,0,0,0,12,&value) && !memcmp(&value,&guard,sizeof(value)));
    assert(!ql_m1_cell(0,0,0,0,0,NULL) && !ql_m1_clock(0,0,NULL));
    assert(ql_m1_clock(UINT64_MAX,11,&clock) && clock.hopf_fiber==1 && clock.degree720==690);
    assert(!ql_m1_clock(0,UINT32_MAX,&clock));
    for(cycle=0;cycle<2;++cycle) for(tick=0;tick<12;++tick)
        for(f=0;f<6;++f) for(r=0;r<12;++r) for(c=0;c<12;++c) {
            assert(ql_m1_cell(f,r,c,cycle,tick,&value)); compare_source(&value); cell_json(&value);
        }
    for(s=0;s<6;++s) for(c=0;c<(s==4?6u:1u);++c) {
        assert(ql_m1_spanda(s,c,&sp));
        { PRATIBIMBA hc = {0}; SPANDA_COMPILER_PASSES[s](&hc);
          assert(sp.weave_state==hc.weave_state && sp.inversion_state==hc.inversion_state); }
        printf("{\"kind\":\"spanda\",\"coordinate\":\"%016" PRIx64 "\",\"substage_coordinate\":",sp.coordinate);
        if(sp.substage_coordinate) printf("\"%016" PRIx64 "\"",sp.substage_coordinate); else printf("null");
        printf(",\"stage\":%u,\"substage\":%u,\"fold_count\":%u,\"dual_track\":%s,\"weave_state\":%.1f,\"inversion_state\":%u}\n",
            sp.stage,sp.substage,sp.fold_count,sp.dual_track?"true":"false",(double)sp.weave_state,sp.inversion_state);
        if(s==4) assert(sp.fold_count==SPANDA_CF_SUBSTAGE_LUT[c].fold_count && sp.dual_track==SPANDA_CF_SUBSTAGE_LUT[c].dual_track);
    }
    assert(!ql_m1_spanda(1,1,&sp) && !ql_m1_spanda(6,0,&sp) && !ql_m1_spanda(4,6,&sp));
    for(s=0;s<6;++s) {
        assert(ql_m1_formal(s,&form));
        assert(form.next==QL_FLOWERING[s].next && form.inverse==QL_FLOWERING[s].inverse);
        assert(form.signature==CL42_BASIS[s].signature);
        assert(form.numerator_position==QL_TRIG_TABLE[s].numerator_pos && form.denominator_position==QL_TRIG_TABLE[s].denominator_pos);
        printf("{\"kind\":\"formal\",\"coordinate\":\"%016" PRIx64 "\",\"stage\":%u,\"next\":%u,\"inverse\":%u,\"numerator_position\":%u,\"denominator_position\":%u,\"signature\":%d}\n",
            form.coordinate,s,form.next,form.inverse,form.numerator_position,form.denominator_position,form.signature);
    }
    assert(!ql_m1_formal(6,&form));
    for(tick=0;tick<12;++tick) {
        Quaternion q=quat_from_ring_pos((QL_Tick)tick);
        assert(ql_m1_topology(tick,&topo));
        assert(topo.element_count==get_topological_element_count((uint8_t)tick));
        assert(topo.legacy_ring_quaternion[0]==q.w && topo.legacy_ring_quaternion[1]==q.x);
        printf("{\"kind\":\"topology\",\"coordinate\":\"%016" PRIx64 "\",\"tick12\":%u,\"element_count\":%u,\"legacy_return_stage\":%u,\"legacy_ring_quaternion\":[%.9g,%.9g,0.0,0.0]}\n",
            topo.coordinate,tick,topo.element_count,topo.legacy_return_stage,(double)q.w,(double)q.x);
    }
    assert(!ql_m1_topology(12,&topo));
    for(s=0;s<256;++s) assert(ql_m1_valid_fold(s)==(int)is_valid_fold((uint8_t)s));
    assert(!ql_m1_valid_fold(256));
    assert(!ql_m1_torus(NAN,0,&torus) && !ql_m1_torus(0,INFINITY,&torus));
    assert(ql_m1_torus(0,0,&torus) && fabs(torus.x-25.0/9.0)<1e-12 && torus.y==0 && torus.z==0);
    fprintf(stderr,"M1 probe: 20736 source/C/Rust-comparable cells; checked Spanda, formal, topology and rejection boundaries\n");
    return 0;
}
