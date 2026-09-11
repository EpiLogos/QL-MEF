#include "ql/m2.h"
#include <assert.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
int main(void) {
    size_t i,j;
    double value=91.0;
    assert(ql_m2_correspondence_at(ql_m2_correspondence_count())==NULL);
    assert(ql_m2_correspondence(72,0)==NULL);
    assert(ql_m2_correspondence(0,2)==NULL);
    assert(ql_m2_correspondence_pitch(72,0,0,0,100,&value)==QL_M2_INVALID && value==91.0);
    assert(ql_m2_correspondence_pitch(0,0,0,8,100,&value)==QL_M2_INVALID && value==91.0);
    assert(ql_m2_correspondence_pitch(0,0,2,0,100,&value)==QL_M2_INVALID && value==91.0);
    assert(ql_m2_correspondence_pitch(0,0,0,0,NAN,&value)==QL_M2_NONFINITE && value==91.0);
    assert(ql_m2_correspondence_pitch(0,0,0,0,100,NULL)==QL_M2_INVALID);
    for (i=0;i<ql_m2_correspondence_count();++i) {
        const QL_M2_Correspondence *r=ql_m2_correspondence_at(i);
        assert(ql_m2_correspondence(r->maqam_index,r->role)==r);
        printf("rule\t%u\t%u\t%016" PRIx64 "\t%016" PRIx64 "\t%016" PRIx64 "\t%016" PRIx64
               "\t%016" PRIx64 "\t%016" PRIx64 "\t%u\t%u\t%u\t%s\t%s\t%s\t%s\n",
               r->maqam_index,r->role,r->maqam_id,r->planet_id,r->chakra_id,r->tattva_id,
               r->musical_relation_id,r->planetary_relation_id,r->planet_index,r->chakra_index,r->fibre,
               r->colour_name ? r->colour_name : "-",r->element_literal,r->planetary_mode_literal,r->interval_literal);
        for(j=0;j<8;++j) printf("step\t%u\t%u\t%zu\t%u\t%u\n",r->maqam_index,r->role,j,r->spelled_supported,r->spelled_steps24[j]);
    }
    for(i=0;i<72;++i) for(j=0;j<2;++j) {
        unsigned t,d,root;
        for(t=0;t<2;++t) for(d=0;d<8;++d) for(root=0;root<3;++root) {
            double tonic=root==0?55.0:root==1?261.625565:1234.567;
            QL_M2_Result status;
            value=91.0;
            status=ql_m2_correspondence_pitch((uint8_t)i,(uint8_t)j,(uint8_t)t,(uint8_t)d,tonic,&value);
            assert(status==QL_M2_OK || status==QL_M2_UNAVAILABLE);
            if(status!=QL_M2_OK) assert(value==91.0);
            printf("pitch\t%zu\t%zu\t%u\t%u\t%u\t%u\t%.17g\n",i,j,t,d,root,(unsigned)status,value);
        }
    }
    return 0;
}
