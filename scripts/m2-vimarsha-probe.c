#include "ql/m2.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    static const uint16_t ratios[4][2]={{1,1},{9,8},{65535,1},{1,65535}};
    unsigned t,l,m,r,k,i;
    QL_M2_VimarshaSeed s={0,0,0,0,0,1,1};
    QL_M2_VimarshaReading out,before;
    memset(&out,0xA5,sizeof(out)); before=out;
#define BAD(field,value) do { QL_M2_VimarshaSeed bad=s;bad.field=(value);assert(ql_m2_vimarsha(&bad,&out)==QL_M2_INVALID);assert(memcmp(&out,&before,sizeof(out))==0); } while(0)
    BAD(tick12,12);BAD(lens,12);BAD(musical_mode,7);BAD(codon,64);BAD(rotation,8);BAD(ratio_num,0);BAD(ratio_den,0);
    assert(ql_m2_vimarsha(NULL,&out)==QL_M2_INVALID);assert(ql_m2_vimarsha(&s,NULL)==QL_M2_INVALID);
    for(t=0;t<12;++t) for(l=0;l<12;++l) for(m=0;m<7;++m) for(r=0;r<8;++r) for(k=0;k<4;++k) {
        s.tick12=(uint8_t)t;s.lens=(uint8_t)l;s.musical_mode=(uint8_t)m;s.rotation=(uint8_t)r;
        s.codon=(uint8_t)((t*12u+l+m+r)%64u);s.ratio_num=ratios[k][0];s.ratio_den=ratios[k][1];
        assert(ql_m2_vimarsha(&s,&out)==QL_M2_OK);
        printf("vimarsha\t%u\t%u\t%u\t%u\t%u\t%u\t%u",t,l,m,s.codon,r,s.ratio_num,s.ratio_den);
        for(i=0;i<8;++i) printf("\t%.9g",(double)out.audio_octet_hz[i]);
        for(i=0;i<4;++i) { const QL_M2_NodalConstraint *n=&out.nodal_quartet[i];printf("\t%u\t%u\t%u\t%u",n->ql_position,n->helix,n->m,n->n); }
        putchar('\n');
    }
    return 0;
}
