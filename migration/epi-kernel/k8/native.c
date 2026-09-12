/* Complete accepted/current identity preservation and exact lifted-clock laws. */
#include "ql/m2_aperture.h"
#include "ql/coupled_clock.h"
#include <assert.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>

static void same(const char *a, const char *b) { assert((!a && !b) || (a && b && strcmp(a,b)==0)); }
static void preservation(void) {
    assert(ql_m_live_node_count()==ql_m_node_count()+27);
    assert(ql_m_live_relation_count()==ql_m_relation_count()+78);
    same(ql_m_live_base_registry_revision(),ql_m_registry_revision());
    assert(ql_m_live_accepts_base(ql_m_registry_revision()));
    assert(ql_m_live_accepts_base(ql_m_live_registry_revision()));
    assert(!ql_m_live_accepts_base(NULL) && !ql_m_live_accepts_base("foreign"));
    for (size_t i=0;i<ql_m_node_count();++i) {
        const QL_M_Node *a=ql_m_node_at(i), *b=ql_m_live_node_by_id(a->id);
        assert(b && a->id==b->id && a->parent_id==b->parent_id && a->root_id==b->root_id);
        assert(a->depth==b->depth && a->lexical_depth==b->lexical_depth && a->records_count==b->records_count);
        assert(a->root_position==b->root_position && a->parent_basis==b->parent_basis);
        assert(a->structural_status==b->structural_status && a->aggregate==b->aggregate);
        same(a->source_ref,b->source_ref); same(a->local_segment,b->local_segment);
        same(a->separator,b->separator); same(a->names_json,b->names_json); same(a->aliases_json,b->aliases_json);
        same(a->source_parent_refs_json,b->source_parent_refs_json);
        same(a->lexical_parent_source_ref,b->lexical_parent_source_ref);
        for (size_t j=0;j<a->records_count;++j) {
            const QL_M_SourceRecord *r=ql_m_node_record_at(a->id,j), *s=ql_m_live_node_record_at(b->id,j);
            assert(r && s && r->file_index==s->file_index && r->record_index==s->record_index);
            same(r->payload_sha256,s->payload_sha256); same(r->property_keys_json,s->property_keys_json);
        }
        for (size_t j=0;j<ql_m_child_count(a->id);++j) {
            const QL_M_Node *child=ql_m_child_at(a->id,j);
            assert(ql_m_live_parent(child->id)->id==a->id);
        }
    }
    for (size_t i=0;i<ql_m_relation_count();++i) {
        const QL_M_Relation *a=ql_m_relation_at(i), *b=ql_m_live_relation_by_id(a->id);
        assert(b && a->from_id==b->from_id && a->to_id==b->to_id && a->record==b->record);
        assert(a->orientation==b->orientation && a->cross_m==b->cross_m);
        same(a->relation_ref,b->relation_ref); same(a->source_kind,b->source_kind);
        same(a->from_ref,b->from_ref); same(a->to_ref,b->to_ref);
    }
    for (size_t i=0;i<ql_m_live_source_file_count();++i) {
        const QL_M_SourceOrigin *origin=ql_m_live_source_origin_at(i);
        assert(origin);
        same(origin->repository, i<ql_m_source_file_count()?ql_m_source_repository():"EpiLogos/QL-MEF");
        assert(strlen(origin->revision)==40);
    }
    assert(!ql_m_live_source_origin_at(ql_m_live_source_file_count()));
    assert(!ql_m_live_node_at(ql_m_live_node_count()));
    assert(!ql_m_live_relation_at(ql_m_live_relation_count()));
    assert(!ql_m_live_resolve("#3-5-5-0") && !ql_m_live_resolve("#"));
    assert(ql_clock_centre()->id==UINT64_C(0xf0c7dcb383d337ff));
    assert(ql_m_live_child_count(ql_clock_centre()->id)==360);
    same(ql_clock_field()->source_ref,"#3-0");
}
static void apertures(void) {
    static const unsigned division[16]={1,2,4,8,9,10,12,15,24,30,36,40,45,90,180,360};
    for (unsigned i=0;i<18;++i) {
        const QL_M2_Aperture *a=ql_m2_aperture_at(i);
        assert(a && ql_m2_aperture_by_id(a->coordinate)==a);
        assert(ql_m_live_node_by_id(a->coordinate));
        if (i<16) {
            const QL_M2_Aperture *b=ql_m2_aperture_by_id(a->reciprocal);
            assert(a->kind==QL_M2_STATIC_APERTURE && a->native_index==i);
            assert(a->division_half_degrees==division[i]*2 && a->segments==360/division[i]);
            assert(a->native_orientation_half_degrees==45*i && b->native_index==15-i);
            assert(b->reciprocal==a->coordinate && b->pair==a->pair);
            assert(ql_m_live_parent(a->coordinate)->id==a->pair);
            assert(ql_m_live_child_count(a->pair)==2);
            unsigned antipode=999;
            assert(ql_m2_void_antipode(i,&antipode) && antipode==(i+8)%16 && antipode!=b->native_index);
        } else {
            assert(a->native_index==255 && !a->pair && !a->reciprocal);
            assert(a->kind==(i==16?QL_M2_VOID_APERTURE:QL_M2_FIBONACCI_APERTURE));
            assert(a->division_half_degrees==(i==16?45:12) && a->segments==(i==16?16:60));
        }
    }
    unsigned sentinel=999;
    assert(!ql_m2_void_antipode(16,&sentinel) && sentinel==999);
    assert(!ql_m2_void_antipode(0,NULL) && !ql_m2_aperture_at(18) && !ql_m2_aperture_by_id(0));
}
static QL_CoupledClock initial(void) {
    QL_CoupledClock c={{0,719},{-1,1},{7,7,7},{1,-2},3,{0,0},0}; return c;
}
static void clocks(void) {
    QL_PhaseLift p={0,719},o={123,456};
    assert(ql_phase_add(p,1,&o)==QL_CLOCK_OK && o.turns==1 && o.half_degrees==0);
    assert(ql_phase_double_cover_half_degrees(o)==720);
    assert(ql_phase_add(o,720,&o)==QL_CLOCK_OK && o.turns==2 && ql_phase_double_cover_half_degrees(o)==0);
    assert(ql_phase_add((QL_PhaseLift){0,0},-1,&o)==QL_CLOCK_OK && o.turns==-1 && o.half_degrees==719);
    assert(ql_phase_double_cover_half_degrees(o)==1439);
    QL_PhaseLift saved=o;
    assert(ql_phase_add((QL_PhaseLift){INT64_MAX,719},1,&o)==QL_CLOCK_OVERFLOW);
    assert(o.turns==saved.turns && o.half_degrees==saved.half_degrees);
    assert(ql_phase_add((QL_PhaseLift){INT64_MIN,0},-1,&o)==QL_CLOCK_OVERFLOW);
    assert(ql_phase_add((QL_PhaseLift){0,720},1,&o)==QL_CLOCK_INVALID);
    QL_CoupledClock a=initial(),b=a,c=a;
    assert(ql_clock_validate(&a)==QL_CLOCK_OK);
    assert(ql_clock_set_axis(&a,0,0,(QL_PhaseLift){3,90},&b)==QL_CLOCK_OK);
    assert(b.inscription.turns==3 && b.lensing.turns==a.lensing.turns && b.lensing.half_degrees==1);
    assert(ql_clock_set_axis(&b,1,1,(QL_PhaseLift){-3,14},&b)==QL_CLOCK_OK);
    assert(b.inscription.turns==3 && b.lensing.turns==-3 && b.generation==2);
    c=b;
    assert(ql_clock_advance(&b,0,1,&b)==QL_CLOCK_STALE && memcmp(&b,&c,sizeof b)==0);
    assert(ql_clock_advance(&b,2,QL_CLOCK_MAX_DRIVER_STEP+1,&b)==QL_CLOCK_INVALID && memcmp(&b,&c,sizeof b)==0);
    b=initial(); c=initial();
    assert(ql_clock_advance(&b,0,713,&b)==QL_CLOCK_OK);
    for (int i=0;i<713;++i) assert(ql_clock_advance(&c,c.generation,1,&c)==QL_CLOCK_OK);
    assert(b.inscription.turns==c.inscription.turns && b.inscription.half_degrees==c.inscription.half_degrees);
    assert(b.lensing.turns==c.lensing.turns && b.lensing.half_degrees==c.lensing.half_degrees);
    assert(b.rate_remainders[0]==c.rate_remainders[0] && b.rate_remainders[1]==c.rate_remainders[1]);
    assert(ql_clock_advance(&b,b.generation,-713,&b)==QL_CLOCK_OK);
    assert(b.inscription.turns==a.inscription.turns && b.inscription.half_degrees==a.inscription.half_degrees);
    assert(b.lensing.turns==a.lensing.turns && b.lensing.half_degrees==a.lensing.half_degrees);
    assert(!b.rate_remainders[0] && !b.rate_remainders[1]);
    for (unsigned i=0;i<3;++i) assert(b.grid_origin_half_degrees[i]==7);
    assert(ql_clock_set_trajectory(&b,b.generation,9,8,2,&b)==QL_CLOCK_OK);
    assert(ql_clock_advance(&b,b.generation,720,&b)==QL_CLOCK_OK);
    assert(b.inscription.turns==5 && b.inscription.half_degrees==359);
    assert(b.lensing.turns==3 && b.lensing.half_degrees==1);
    c=b; c.generation=UINT64_MAX; b=c;
    assert(ql_clock_advance(&c,c.generation,1,&c)==QL_CLOCK_OVERFLOW && memcmp(&b,&c,sizeof b)==0);
    assert(ql_clock_validate(NULL)==QL_CLOCK_INVALID);
    uint16_t first=999,period=999;
    assert(ql_m2_grid_closure(0,1)==120 && ql_m2_grid_closure(0,2)==180 && ql_m2_grid_closure(1,2)==360);
    assert(ql_m2_grid_alignment(0,1,7,7,&first,&period)==QL_CLOCK_OK && first==7 && period==120);
    first=period=999;
    assert(ql_m2_grid_alignment(0,1,0,1,&first,&period)==QL_CLOCK_NO_ALIGNMENT && first==999 && period==999);
    assert(ql_m2_grid_alignment(0,2,6,3,&first,&period)==QL_CLOCK_OK && first%12==6 && first%45==3);
    assert(!ql_m2_grid_quantum(3) && !ql_m2_grid_closure(0,3));
    printf("{\"schema\":\"ql.k8-native-observation/v1\",\"nodes\":%zu,\"relations\":%zu,\"registry_revision\":\"%s\",\"apertures\":18,\"centre_children\":360,\"clock\":\"passed\",\"graph\":\"not-observed\"}\n", ql_m_live_node_count(),ql_m_live_relation_count(),ql_m_live_registry_revision());
}
int main(void) { preservation(); apertures(); clocks(); return 0; }
