/* Observe actual compiled graph routing; no synthetic registry or receiver. */
#include "ql/m2.h"
#include <assert.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    static const uint8_t expected[7]={7,6,5,4,3,2,1};
    unsigned i;
    QL_M2_PlanetChakraRoute unchanged={123,456,99,789},route;
    route=unchanged;
    assert(ql_m2_planet_chakra_route(10,&route)==QL_M2_INVALID);
    assert(route.planet_id==123 && route.chakra_id==456 && route.chakra_index==99 && route.relation_count==789);
    assert(ql_m2_planet_chakra_route(0,NULL)==QL_M2_INVALID);
    for(i=0;i<10;++i) {
        size_t j;
        QL_M2_Result status;
        route=unchanged;
        status=ql_m2_planet_chakra_route(i,&route);
        if(i>=7) {
            assert(status==QL_M2_UNAVAILABLE);
            assert(route.planet_id==123 && route.chakra_id==456 && route.chakra_index==99 && route.relation_count==789);
            assert(ql_m2_planet_chakra_relation(i,0)==NULL);
            printf("absent\t%u\n",i);
            continue;
        }
        assert(status==QL_M2_OK && route.chakra_index==expected[i] && route.relation_count>0);
        printf("route\t%u\t%016" PRIx64 "\t%016" PRIx64 "\t%u\t%zu\n",
            i,route.planet_id,route.chakra_id,route.chakra_index,route.relation_count);
        for(j=0;j<route.relation_count;++j) {
            const QL_M_Relation *r=ql_m2_planet_chakra_relation(i,j);
            assert(r && r->from_id==route.planet_id && r->to_id==route.chakra_id);
            assert(strcmp(r->source_kind,"PLANETARY_RESONANCE")==0);
            assert(ql_m_relation_by_id(r->id)==r);
            printf("assertion\t%u\t%zu\t%016" PRIx64 "\t%s\t%u\n",i,j,r->id,r->relation_ref,r->record);
        }
        assert(ql_m2_planet_chakra_relation(i,route.relation_count)==NULL);
    }
    assert(ql_m2_planet_chakra_relation(10,0)==NULL);
    {
        QL_M2_DecanPlanetRoute retained={123,456,99,88,7,6,5,4},decan=retained;
        assert(ql_m2_decan_planet_route(NAN,&decan)==QL_M2_NONFINITE);
        assert(decan.decan_id==123 && decan.planet_index==88 && decan.property_record==4);
        assert(ql_m2_decan_planet_route(360,&decan)==QL_M2_BOUNDARY);
        assert(ql_m2_decan_planet_route(-1,&decan)==QL_M2_BOUNDARY);
        assert(ql_m2_decan_planet_route(0,NULL)==QL_M2_INVALID);
        for(i=0;i<36;++i) {
            double longitude=(double)i*10.0+5.0;
            size_t j;
            assert(ql_m2_decan_planet_route(longitude,&decan)==QL_M2_OK);
            assert(decan.zodiac_decan_index==i && decan.candidate_count==1 && decan.relation_count>0);
            if(i==11) {
                assert(decan.source_conflict && decan.planet_index==255 && decan.planet_id==0 && decan.property_record==683);
            } else {
                assert(!decan.source_conflict && decan.planet_index<7 && decan.planet_id!=0);
            }
            if(i==2) assert(decan.planet_index==3); /* Actual Aries3 RULED_BY Venus, not retained C Jupiter. */
            printf("decan\t%u\t%016" PRIx64 "\t%u\t%u\t%zu\t%zu\n",i,decan.decan_id,decan.planet_index,decan.source_conflict,decan.candidate_count,decan.relation_count);
            for(j=0;j<decan.relation_count;++j) {
                const QL_M_Relation *r=ql_m2_decan_planet_relation(longitude,j);
                assert(r && r->from_id==decan.decan_id && strcmp(r->source_kind,"RULED_BY")==0);
                assert(ql_m_relation_by_id(r->id)==r);
                printf("decan_assertion\t%u\t%016" PRIx64 "\t%s\t%u\n",i,r->id,r->relation_ref,r->record);
            }
            assert(ql_m2_decan_planet_relation(longitude,decan.relation_count)==NULL);
        }
    }
    return 0;
}
