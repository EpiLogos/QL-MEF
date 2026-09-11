#include "ql/m3.h"
#include <assert.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
static void bytes(const char *kind,unsigned index,const uint8_t *row,size_t count) {
 printf("[\"%s\",%u",kind,index);for(size_t i=0;i<count;i++) printf(",%u",row[i]);puts("]");
}
static void boundary_checks(void) {
 QL_M3_CodonRecord c;memset(&c,0xa5,sizeof(c));QL_M3_CodonRecord old=c;
 assert(ql_m3_codon(64,&c)==QL_M3_INVALID && !memcmp(&c,&old,sizeof(c)));
 assert(ql_m3_codon(0,NULL)==QL_M3_INVALID);
 uint8_t value=42;assert(ql_m3_line_change(0,6,&value)==QL_M3_INVALID && value==42);
 assert(ql_m3_apply_matrix(2,5,&value)==QL_M3_GAP && value==42);
 assert(ql_m3_apply_matrix(3,0,&value)==QL_M3_INVALID && value==42);
 uint8_t minor[4]={0};assert(ql_m3_minor(56,minor)==QL_M3_INVALID);
 uint64_t word=123;uint8_t bad[]={0,72};
 assert(ql_m3_transduce(bad,2,&word)==QL_M3_INVALID && word==123);
 assert(ql_m3_transduce(NULL,1,&word)==QL_M3_INVALID && word==123);
 assert(ql_m3_transduce(NULL,0,&word)==QL_M3_OK && word==0);
 QL_M3_Clock clock;memset(&clock,0xa5,sizeof(clock));QL_M3_Clock prior=clock;
 assert(ql_m3_clock_advance(UINT64_MAX,1,&clock)==QL_M3_INVALID && !memcmp(&clock,&prior,sizeof(clock)));
 assert(ql_m3_clock_advance(719,1,&clock)==QL_M3_OK && clock.completed_double_covers==1 && clock.degree720==0);
 assert(ql_m3_clock(360,&clock)==QL_M3_OK && clock.layer==1 && clock.completed_double_covers==0 && clock.tick12==0);
 assert(ql_m3_clock_reconciled_symbolic_fields()==0);
 assert(ql_m3_active_state((QL_M3_Quaternion){NAN,0,0,0},0,&value)==QL_M3_INVALID);
 assert(ql_m3_active_state((QL_M3_Quaternion){0,0,0,0},0,&value)==QL_M3_INVALID);
 assert(!ql_m3_node(QL_M3_DEGREE,360));assert(!ql_m3_node((QL_M3_NodeKind)255,0));
 assert(ql_m3_coordinate_count()==996);assert(!ql_m3_coordinate_at(996));
}
int main(void) {
 boundary_checks();
 for(unsigned kind=0;kind<12;kind++) for(size_t i=0;i<ql_m3_group_count((QL_M3_NodeKind)kind);i++) {
  const QL_M_Node *node=ql_m3_node((QL_M3_NodeKind)kind,i);assert(node);
  printf("[\"node\",%u,%zu,\"%016" PRIx64 "\",\"%s\"]\n",kind,i,node->id,node->source_ref);
 }
 for(unsigned i=0;i<16;i++){int8_t pair[2];assert(ql_m3_pair(i,pair)==QL_M3_OK);printf("[\"pair\",%u,%d,%d]\n",i,pair[0],pair[1]);}
 for(unsigned i=0;i<8;i++){uint16_t t[7];assert(ql_m3_trigram(i,t)==QL_M3_OK);printf("[\"trigram\",%u",i);for(unsigned j=0;j<7;j++)printf(",%u",t[j]);puts("]");}
 for(unsigned i=0;i<64;i++) {
  QL_M3_CodonRecord c;assert(ql_m3_codon(i,&c)==QL_M3_OK);
  printf("[\"codon\",%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%d,%d,%d,%d]\n",i,c.values[0],c.values[1],c.values[2],c.class_id,c.state_count,c.complement,c.nuclear,c.anticodon,c.amino_record,c.rna_changes,c.charges[0],c.charges[1],c.charges[2],c.charges[3]);
  for(unsigned l=0;l<6;l++){uint8_t to;assert(ql_m3_line_change(i,l,&to)==QL_M3_OK);printf("[\"line\",%u,%u,%u]\n",i,l,to);}
  for(unsigned m=0;m<3;m++){uint8_t to=255;QL_M3_Result result=ql_m3_apply_matrix(m,i,&to);printf("[\"matrix\",%u,%u,%u,%u]\n",i,m,(unsigned)result,to);}
  uint8_t profile[5];assert(ql_m3_rotational_profile(i,profile)==QL_M3_OK);bytes("profile",i,profile,5);
  for(unsigned slot=0;slot<c.state_count;slot++){uint16_t ordinal;assert(ql_m3_pose_ordinal(i,slot,&ordinal)==QL_M3_OK);printf("[\"pose\",%u,%u,%u]\n",i,slot,ordinal);}
  uint16_t bad=123;assert(ql_m3_pose_ordinal(i,c.state_count,&bad)==QL_M3_INVALID && bad==123);
  QL_M3_Rotation rot[8];assert(ql_m3_rotations(i,rot)==QL_M3_OK);
  for(unsigned r=0;r<8;r++)printf("[\"rotation\",%u,%u,%u,%u,%u,%u,%u,%u,%u,%d]\n",i,r,rot[r].pair1,rot[r].pair2,rot[r].codon,rot[r].positive,rot[r].slot,rot[r].degrees,rot[r].nondual,rot[r].value);
  for(unsigned r=0;r<8;r++){QL_M3_Quaternion q;assert(ql_m3_quaternion(i,r,&q)==QL_M3_OK);printf("[\"quaternion\",%u,%u,%.9g,%.9g,%.9g,%.9g]\n",i,r,(double)q.w,(double)q.x,(double)q.y,(double)q.z);}
  for(unsigned phase=0;phase<2;phase++){char seq[4];assert(ql_m3_transcribe(i,phase,seq)==QL_M3_OK);printf("[\"transcription\",%u,%u,\"%s\"]\n",i,phase,seq);}
 }
 for(unsigned m=0;m<3;m++)for(unsigned n=0;n<4;n++){uint8_t p;assert(ql_m3_matrix_partner(m,n,&p)==QL_M3_OK);printf("[\"partner\",%u,%u,%u]\n",m,n,p);}
 for(unsigned i=0;i<56;i++){uint8_t t[4];assert(ql_m3_minor(i,t)==QL_M3_OK);bytes("minor",i,t,4);}
 for(unsigned i=0;i<22;i++){const QL_M3_Major *m=ql_m3_major(i);assert(m);printf("[\"major\",%u,\"%s\",%u,%u]\n",i,m->name,m->chromosome_pair,m->amino_acid_index);}
 for(unsigned i=0;i<72;i++){uint8_t idx=(uint8_t)i,c,g;uint64_t word;assert(ql_m3_transduce(&idx,1,&word)==QL_M3_OK);assert(ql_m3_epogdoon(i,&c,&g)==QL_M3_OK);printf("[\"transduce\",%u,\"%016" PRIx64 "\",%u,%u]\n",i,word,c,g);}
 for(unsigned i=0;i<360;i++){uint16_t row[26];assert(ql_m3_clock_record(i,row)==QL_M3_OK);printf("[\"record\",%u",i);for(unsigned j=0;j<26;j++)printf(",%u",row[j]);puts("]");}
 for(unsigned i=0;i<1441;i++){QL_M3_Clock c;assert(ql_m3_clock(i,&c)==QL_M3_OK);printf("[\"clock\",%u,%u,%u,%u,%u,%u,%u,%u,%" PRIu64 ",\"%016" PRIx64 "\",\"%016" PRIx64 "\",\"%016" PRIx64 "\",\"%016" PRIx64 "\"]\n",i,c.degree720,c.degree360,c.layer,c.tick12,c.decan_phase,c.polar720,c.uniform_hexagram_estimate,c.completed_double_covers,c.degree_node,c.backbone_node,c.clockwise_node,c.polar_node);}
 return 0;
}
