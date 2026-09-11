/* Native M2 acceptance observer. Assertion failure is a test failure, never a receipt. */
#include "ql/m2.h"
#include <assert.h>
#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    size_t t,i,j,k,coordinates=0,relations=0,records=0;
    const QL_M_Node *root=ql_m_root(2);
    uint8_t out=201,axes[4],expanded;
    uint64_t word=123,power,power_q;
    QL_M2_Aspect aspect;
    QL_M2_Amplitude input[72],q[72],again[72],form[64],qform[64];
    const QL_M2_Ground *g=ql_m2_ground();
    assert(root && strcmp(ql_m2_registry_revision(),ql_m_registry_revision())==0);
    assert(g->phi_polynomial[0]==-1 && g->phi_polynomial[1]==-1 && g->phi_polynomial[2]==1);
    assert(g->tattva_divisions[0]==5 && g->tattva_divisions[1]==7 && g->tattva_divisions[2]==24);
    assert(g->material[0]*g->material[1]==72 && g->form[0]*g->form[1]==64);
    assert(ql_m2_table(QL_M2_TABLE_COUNT)==NULL && ql_m2_record(999,0)==NULL);
    printf("registry\t%s\n",ql_m2_registry_revision());
    printf("ground");
    for(j=0;j<3;j++) printf("\t%d",(int)g->phi_polynomial[j]);
#define G(field) for(j=0;j<sizeof(g->field)/sizeof(g->field[0]);j++) printf("\t%u",(unsigned)g->field[j])
    G(tattva_divisions);G(mef);G(tattva);G(decan);G(shem);G(material);G(form);G(asma);G(mantra);G(station);
#undef G
    putchar('\n');
    for(t=0;t<QL_M2_TABLE_COUNT;t++) {
        const QL_M2_Table *table=ql_m2_table((unsigned)t);
        const QL_M_Node *scope=ql_m_resolve(table->scope);
        assert(scope && scope->root_id==root->id && table->column_count<=11);
        assert(ql_m2_record((unsigned)t,table->row_count)==NULL);
        for(i=0;i<table->row_count;i++) {
            const QL_M2_Record *r=ql_m2_record((unsigned)t,i);
            printf("record\t%s\t%zu",table->name,i);
            for(j=0;j<table->column_count;j++) printf("\t%" PRIu64,r->values[j]);
            putchar('\n');
            printf("binding\t%s\t%zu\t%016" PRIx64 "\n",table->name,i,r->coordinate_id);
            if(r->coordinate_id) {
                const QL_M_Node *n=ql_m_node_by_id(r->coordinate_id);
                assert(n && n->root_id==root->id);
                while(n && n->id!=scope->id) n=ql_m_parent(n->id);
                assert(n && n->id==scope->id);
            }
            records++;
        }
    }
    assert(records==764);
    assert(ql_m2_exact_record_count(0)==0 && ql_m2_exact_record_count(ql_m_root(1)->id)==0);
    for(i=0;i<ql_m_node_count();i++) {
        const QL_M_Node *n=ql_m_node_at(i);
        if(n->root_id!=root->id) continue;
        printf("coordinate\t%s\t%016" PRIx64 "\t%016" PRIx64 "\t%zu\n",n->source_ref,n->id,n->parent_id,ql_m2_exact_record_count(n->id));
        coordinates++;
    }
    for(i=0;i<ql_m_relation_count();i++) {
        const QL_M_Relation *r=ql_m_relation_at(i);
        const QL_M_Node *from=ql_m_node_by_id(r->from_id), *to=ql_m_node_by_id(r->to_id);
        if((!from||from->root_id!=root->id)&&(!to||to->root_id!=root->id)) continue;
        printf("relation\t%016" PRIx64 "\t%s\t%016" PRIx64 "\t%016" PRIx64 "\n",r->id,r->source_kind,r->from_id,r->to_id);
        relations++;
    }
    for(t=0;t<4;t++) for(i=0;i<72;i++) {
        assert(ql_m2_unflatten((unsigned)t,(unsigned)i,axes)==QL_M2_OK);
        assert(ql_m2_flatten((unsigned)t,axes[0],axes[1],axes[2],axes[3],&out)==QL_M2_OK && out==i);
        printf("axes\t%zu\t%zu\t%u\t%u\t%u\t%u\n",t,i,axes[0],axes[1],axes[2],axes[3]);
    }
    out=201;
    assert(ql_m2_flatten(99,0,0,0,0,&out)==QL_M2_INVALID && out==201);
    assert(ql_m2_flatten(QL_M2_MEF,12,0,0,0,&out)==QL_M2_INVALID && out==201);
    assert(ql_m2_flatten(QL_M2_MEF,0,0,1,0,&out)==QL_M2_INVALID);
    assert(ql_m2_unflatten(QL_M2_TATTVA,72,axes)==QL_M2_INVALID);
    assert(ql_m2_unflatten(QL_M2_SHEM,0,NULL)==QL_M2_INVALID);
    for(i=0;i<5;i++) for(j=0;j<8;j++) for(k=0;k<4;k++) {
        uint8_t unpacked[3];
        assert(ql_m2_signature((unsigned)i,(unsigned)j,(unsigned)k,&out)==QL_M2_OK);
        assert(ql_m2_unpack_signature(out,unpacked)==QL_M2_OK);
        assert(unpacked[0]==i && unpacked[1]==j && unpacked[2]==k);
        printf("signature\t%zu\t%zu\t%zu\t%u\n",i,j,k,out);
    }
    assert(ql_m2_signature(5,0,0,&out)==QL_M2_INVALID);
    assert(ql_m2_unpack_signature(7,axes)==QL_M2_INVALID);
    assert(ql_m2_unpack_signature(256,axes)==QL_M2_INVALID);
    for(i=0;i<36;i++) for(j=0;j<2;j++) {
        QL_M2_Result result;
        out=201;result=ql_m2_tattva_step((unsigned)i,(unsigned)j,&out);
        printf("tattva-step\t%zu\t%zu\t%u\t%u\n",i,j,(unsigned)result,out);
    }
    assert(ql_m2_tattva_step(36,0,&out)==QL_M2_INVALID);
    for(i=0;i<72;i++) {
        uint8_t fibre,target,scalar,idx=(uint8_t)i;
        assert(ql_m2_decan_to_fibre((unsigned)i,&fibre)==QL_M2_OK);
        assert(ql_m2_fibre_target((unsigned)i,&target)==QL_M2_OK);
        assert(ql_m2_scalar_compress((unsigned)i,&scalar)==QL_M2_OK);
        assert(ql_m2_legacy_det(&idx,1,&word)==QL_M2_OK);
        assert(target/16==i/18);
        printf("transforms\t%zu\t%u\t%u\t%u\t%" PRIu64 "\n",i,fibre,target,scalar,word);
    }
    for(i=0;i<64;i++) {
        assert(ql_m2_scalar_expand((unsigned)i,&expanded)==QL_M2_OK);
        printf("expand\t%zu\t%u\n",i,expanded);
    }
    assert(ql_m2_scalar_expand(64,&out)==QL_M2_INVALID);
    assert(ql_m2_fibre_target(72,&out)==QL_M2_INVALID);
    assert(ql_m2_decan_to_fibre(72,&out)==QL_M2_INVALID);
    assert(ql_m2_scalar_compress(72,&out)==QL_M2_INVALID);
    word=123;out=255;
    assert(ql_m2_legacy_det(&out,1,&word)==QL_M2_INVALID && word==123);
    assert(ql_m2_legacy_det(NULL,1,&word)==QL_M2_INVALID && word==123);
    assert(ql_m2_legacy_det(NULL,0,&word)==QL_M2_OK && word==0);
    for(i=0;i<10;i++) {
        assert(ql_m2_planet_preempted((unsigned)i,&out)==QL_M2_OK && out==(i>=7));
        printf("preempted\t%zu\t%u\n",i,out);
    }
    out=201;assert(ql_m2_planet_preempted(10,&out)==QL_M2_INVALID && out==201);
    j=0;
    for(i=0;i<100;i++) {
        const QL_M2_Record *r=ql_m2_record(QL_M2_ASMA_TABLE,i);
        assert(ql_m2_asma_route((unsigned)i,&out)==QL_M2_OK);
        j+=out;
        printf("asma-route\t%zu\t%u\t%u\n",i,out,ql_m2_digital_root(r->values[6]));
    }
    assert(j==64 && ql_m2_asma_route(100,&out)==QL_M2_INVALID);
    assert(ql_m2_asma_route(UINT_MAX,&out)==QL_M2_INVALID);
    assert(ql_m2_digital_root(0)==0 && ql_m2_digital_root(UINT64_MAX)==6);
    for(i=0;i<72;i++) for(j=0;j<8;j++) {
        double hz;
        assert(ql_m2_maqam_pitch((unsigned)i,(unsigned)j,220,&hz)==QL_M2_OK);
        printf("pitch\t%zu\t%zu\t%.17g\n",i,j,hz);
    }
    {double hz=123;
        assert(ql_m2_maqam_pitch(72,0,220,&hz)==QL_M2_INVALID && hz==123);
        assert(ql_m2_maqam_pitch(0,8,220,&hz)==QL_M2_INVALID);
        assert(ql_m2_maqam_pitch(0,1,-1,&hz)==QL_M2_INVALID);
        assert(ql_m2_maqam_pitch(0,7,DBL_MAX,&hz)==QL_M2_NONFINITE);
    }
    for(i=0;i<360;i++) for(j=0;j<360;j++) {
        assert(ql_m2_aspect((double)i,(double)j,&aspect)==QL_M2_OK);
        printf("aspect\t%zu\t%zu\t%u\t%.17g\t%.17g\n",i,j,aspect.type,aspect.angle,aspect.orb);
    }
    assert(ql_m2_aspect(NAN,0,&aspect)==QL_M2_NONFINITE);
    assert(ql_m2_aspect(INFINITY,0,&aspect)==QL_M2_NONFINITE);
    assert(ql_m2_aspect(360,0,&aspect)==QL_M2_INVALID);
    assert(ql_m2_aspect(-1,0,&aspect)==QL_M2_INVALID);
    for(i=0;i<72;i++) {input[i].re=(int64_t)(i%7)-3;input[i].im=(int64_t)(i%5)-2;}
    assert(ql_m2_modal_quadrature(input,q)==QL_M2_OK);
    assert(ql_m2_modal_power(input,72,&power)==QL_M2_OK);
    assert(ql_m2_modal_power(q,72,&power_q)==QL_M2_OK && power==power_q);
    assert(ql_m2_modal_transduce(input,form)==QL_M2_OK);
    assert(ql_m2_modal_transduce(q,qform)==QL_M2_OK);
    memcpy(again,input,sizeof(input));
    for(i=0;i<4;i++) assert(ql_m2_modal_quadrature(again,again)==QL_M2_OK);
    assert(memcmp(input,again,sizeof(input))==0);
    for(i=0;i<72;i++) printf("quadrature\t%zu\t%" PRId64 "\t%" PRId64 "\n",i,q[i].re,q[i].im);
    for(i=0;i<64;i++) {
        assert(qform[i].re==-form[i].im && qform[i].im==form[i].re);
        printf("form\t%zu\t%" PRId64 "\t%" PRId64 "\n",i,form[i].re,form[i].im);
    }
    printf("power\t%" PRIu64 "\n",power);
    for(i=0;i<72;i++) {
        QL_M2_Amplitude impulse[72]={{0,0}};
        impulse[i].re=1;assert(ql_m2_modal_transduce(impulse,form)==QL_M2_OK);
        for(j=0;j<64;j++) assert(form[j].re==(int64_t)(j==i/18*16+(i%18<16?i%18:(i%18-16)*8)) && form[j].im==0);
    }
    input[0].re=INT64_MIN;again[0].re=777;
    assert(ql_m2_modal_quadrature(input,again)==QL_M2_INVALID && again[0].re==777);
    assert(ql_m2_modal_transduce(input,form)==QL_M2_INVALID);
    assert(ql_m2_modal_power(input,72,&power)==QL_M2_INVALID);
    assert(ql_m2_modal_power(input,73,&power)==QL_M2_INVALID);
    assert(ql_m2_modal_quadrature(NULL,q)==QL_M2_INVALID);
    fprintf(stderr,"M2 native: %zu records, %zu coordinates, %zu relations; exhaustive finite laws and bounded modal checks passed\n",records,coordinates,relations);
    return ferror(stdout)?1:0;
}
