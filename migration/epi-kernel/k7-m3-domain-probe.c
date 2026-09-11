#include "ql/m3_domain.h"
#include <assert.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
static void form_row(const char *kind,unsigned index,unsigned extra,const QL_M3_Form *f) {
 printf("[\"%s\",%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d",kind,index,extra,
 f->address,f->pose,f->state_count,f->aperture,f->reciprocal_aperture,f->axis,f->pose_ordinal,f->fibonacci_phase60,
 f->pair_xy,f->pair_yz,f->hinge,f->angles_deg10[0],f->angles_deg10[1],f->angles_deg10[2],
 f->velocities_deg10[0],f->velocities_deg10[1],f->velocities_deg10[2],f->pair_angles_deg10[0],f->pair_angles_deg10[1],
 f->lens_division_deg10,f->lens_reciprocal_deg10,f->void_ring_orientation_deg10);
 for(unsigned i=0;i<3;i++)printf(",%u",f->site_elements[i]);
 for(unsigned i=0;i<4;i++)printf(",%u",f->elemental_counts[i]);
 puts("]");
}
int main(void) {
 printf("[\"revision\",\"%s\"]\n",ql_m3_domain_revision());
 for(size_t i=0;i<ql_m3_source_node_count();i++) {
  const QL_M3_SourceNode *n=ql_m3_source_node_at(i);assert(n);
  printf("[\"source-node\",%zu,\"%016" PRIx64 "\",%u,\"%s\"]\n",i,n->id,n->source_record,n->role);
 }
 for(size_t i=0;i<ql_m3_source_relation_count();i++) {
  const QL_M3_SourceRelation *r=ql_m3_source_relation_at(i);assert(r);
  printf("[\"source-edge\",%zu,\"%016" PRIx64 "\",\"%016" PRIx64 "\",\"%016" PRIx64 "\",%u,\"%s\",%s]\n",i,r->id,r->from,r->to,r->source_record,r->kind,r->properties_json);
 }
 for(size_t i=0;i<ql_m3_matrix_cell_count();i++) {
  const QL_M3_MatrixCell *c=ql_m3_matrix_cell_at(i);assert(c);
  printf("[\"cell\",%zu,\"%016" PRIx64 "\",\"%016" PRIx64 "\",\"%016" PRIx64 "\",%u,%u",i,c->id,c->hexagram,c->resolves,c->family,c->address);
  for(size_t j=0;j<(size_t)c->pair_count+c->codon_count;j++) {
   const QL_M3_SourceRelation *r=ql_m3_matrix_cell_relation(i,j);assert(r);printf(",\"%016" PRIx64 "\"",r->id);
  }
  assert(!ql_m3_matrix_cell_relation(i,(size_t)c->pair_count+c->codon_count));puts("]");
 }
 for(unsigned s=0;s<=1440;s++) {
  QL_M3_BackboneProjection p;assert(ql_m3_clock_projection(s,&p)==QL_M3_OK);
  printf("[\"projection\",%u,\"%016" PRIx64 "\",\"%016" PRIx64 "\",\"%016" PRIx64 "\",%u,%u,%u]\n",s,p.backbone,p.codon,p.hexagram,p.codon_address,p.hexagram_address,p.source_record);
 }
 for(unsigned codon=0;codon<64;codon++)for(unsigned slot=0;slot<8;slot++) {
  QL_M3_Quaternion q;uint8_t active;assert(ql_m3_quaternion(codon,slot,&q)==QL_M3_OK);
  assert(ql_m3_active_state(q,codon,&active)==QL_M3_OK);
  printf("[\"active\",%u,%u,%u]\n",codon,slot,active);
 }
 unsigned row=0;
 for(unsigned c=0;c<64;c++) {
  QL_M3_CodonRecord codon;assert(ql_m3_codon(c,&codon)==QL_M3_OK);
  for(unsigned p=0;p<codon.state_count;p++)for(unsigned a=0;a<16;a++) {
   QL_M3_Form f;assert(ql_m3_form(c,p,a,row%60,row%3,&f)==QL_M3_OK);form_row("form",row,0,&f);
   uint8_t cast=255;assert(ql_m3_cast(f.angles_deg10,f.velocities_deg10,&cast)==QL_M3_OK && cast==c);
   for(unsigned m=0;m<3;m++) {
    QL_M3_Form to;memset(&to,0xa5,sizeof(to));QL_M3_Form prior=to;
    QL_M3_Result result=ql_m3_form_apply_matrix(&f,m,&to);
    if(result==QL_M3_GAP){assert(!memcmp(&to,&prior,sizeof(to)));printf("[\"gap\",%u,%u]\n",row,m);}
    else{assert(result==QL_M3_OK);form_row("applied",row,m,&to);}
   }
   row++;
  }
 }
 assert(row==472u*16u);
 QL_M3_Form f;memset(&f,0xa5,sizeof(f));QL_M3_Form old=f;
 assert(ql_m3_form(64,0,0,0,0,&f)==QL_M3_INVALID && !memcmp(&old,&f,sizeof(f)));
 assert(ql_m3_form(0,7,0,0,0,&f)==QL_M3_INVALID);
 assert(ql_m3_form(0,0,16,0,0,&f)==QL_M3_INVALID);
 assert(ql_m3_form(0,0,0,60,0,&f)==QL_M3_INVALID);
 assert(ql_m3_form(0,0,0,0,3,&f)==QL_M3_INVALID);
 assert(ql_m3_form(0,0,0,0,0,NULL)==QL_M3_INVALID);
 assert(!ql_m3_source_node_at(996));assert(!ql_m3_source_relation_at(4891));assert(!ql_m3_matrix_cell_at(184));
 assert(!ql_m3_backbone_projection(0));assert(ql_m3_clock_projection(0,NULL)==QL_M3_INVALID);
 return 0;
}
