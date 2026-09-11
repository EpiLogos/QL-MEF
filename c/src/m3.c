#include "ql/m3.h"
#include <math.h>
#include <string.h>
#include "m3_data.inc"

const char *ql_m3_registry_revision(void) { return native_registry_revision; }
const char *ql_m3_source_digest(void) { return native_source_digest; }
static const QL_M_NodeId *group(QL_M3_NodeKind kind, size_t *count) {
#define G(k) case k: *count=sizeof(native_group_##k)/sizeof(native_group_##k[0]); return native_group_##k
 switch(kind) { G(0);G(1);G(2);G(3);G(4);G(5);G(6);G(7);G(8);G(9);G(10);G(11); default:*count=0;return NULL; }
#undef G
}
size_t ql_m3_group_count(QL_M3_NodeKind kind) { size_t count; (void)group(kind,&count); return count; }
const QL_M_Node *ql_m3_node(QL_M3_NodeKind kind, size_t ordinal) {
 size_t count; const QL_M_NodeId *ids=group(kind,&count);
 return ordinal<count ? ql_m_node_by_id(ids[ordinal]) : NULL;
}
size_t ql_m3_coordinate_count(void) { const QL_M_Node *root=ql_m_root(3); return root ? root->subtree_count : 0; }
const QL_M_Node *ql_m3_coordinate_at(size_t ordinal) {
 for(size_t i=0;i<ql_m_node_count();i++) {
  const QL_M_Node *node=ql_m_node_at(i);
  if(node->root_position==3) { if(ordinal==0) return node; ordinal--; }
 }
 return NULL;
}
QL_M3_Result ql_m3_codon(unsigned c, QL_M3_CodonRecord *out) {
 if(c>=64 || !out) return QL_M3_INVALID;
 QL_M3_CodonRecord r; memset(&r,0,sizeof(r)); r.address=(uint8_t)c;
 unsigned a=c>>4,b=(c>>2)&3,d=c&3;
 r.nucleotides[0]=(uint8_t)a;r.nucleotides[1]=(uint8_t)b;r.nucleotides[2]=(uint8_t)d;
 for(unsigned i=0;i<3;i++) r.values[i]=native_values[r.nucleotides[i]];
 int x=r.values[0],y=r.values[1],z=r.values[2];
 r.charges[0]=(int8_t)(x+y+z);r.charges[1]=(int8_t)(x-y-z);
 r.charges[2]=(int8_t)(x-y+z);r.charges[3]=(int8_t)(x+y-z);
 r.class_id=(uint8_t)(a==d ? (a==b?0:1) : (a==b || b==d?2:3));
 r.state_count=native_profiles[c][0];r.complement=native_hexagrams[c][2];
 r.nuclear=(uint8_t)((native_hexagrams[c][3]<<3)|native_hexagrams[c][4]);
 r.anticodon=(uint8_t)(((d^1)<<4)|((b^1)<<2)|(a^1));
 r.amino_record=native_aa[c];r.is_stop=(uint8_t)(r.amino_record==10);
 r.rna_changes=(uint8_t)(a==1 || b==1 || d==1);r.coordinate=native_group_3[c];
 *out=r;return QL_M3_OK;
}
QL_M3_Result ql_m3_pair(unsigned pair,int8_t out[2]) {
 if(pair>=16 || !out) return QL_M3_INVALID;
 memcpy(out,native_pairs[pair],sizeof(native_pairs[pair]));return QL_M3_OK;
}
QL_M3_Result ql_m3_trigram(unsigned id,uint16_t out[7]) {
 if(id>=8 || !out) return QL_M3_INVALID;
 memcpy(out,native_trigrams[id],sizeof(native_trigrams[id]));return QL_M3_OK;
}
QL_M3_Result ql_m3_hexagram(unsigned id,uint8_t out[5]) {
 if(id>=64 || !out) return QL_M3_INVALID;
 memcpy(out,native_hexagrams[id],sizeof(native_hexagrams[id]));return QL_M3_OK;
}
QL_M3_Result ql_m3_line_change(unsigned codon,unsigned line,uint8_t *out) {
 if(codon>=64 || line>=6 || !out) return QL_M3_INVALID;
 *out=(uint8_t)(codon^(1u<<line));return QL_M3_OK;
}
QL_M3_Result ql_m3_matrix_partner(unsigned matrix,unsigned nucleotide,uint8_t *out) {
 if(matrix>=3 || nucleotide>=4 || !out) return QL_M3_INVALID;
 *out=native_matrix_pairs[matrix][nucleotide];return QL_M3_OK;
}
QL_M3_Result ql_m3_apply_matrix(unsigned matrix,unsigned codon,uint8_t *out) {
 if(matrix>=3 || codon>=64 || !out) return QL_M3_INVALID;
 const uint8_t *table=matrix==0?native_comp:matrix==1?native_move:native_res;
 if(table[codon]==255) return QL_M3_GAP; /* no fake target or partial mutation */
 *out=table[codon];return QL_M3_OK;
}
QL_M3_Result ql_m3_rotational_profile(unsigned codon,uint8_t out[5]) {
 if(codon>=64 || !out) return QL_M3_INVALID;
 memcpy(out,native_profiles[codon],5);return QL_M3_OK;
}
QL_M3_Result ql_m3_rotations(unsigned codon,QL_M3_Rotation out[8]) {
 if(codon>=64 || !out) return QL_M3_INVALID;
 unsigned a=codon>>4,b=(codon>>2)&3,c=codon&3;
 QL_M3_Rotation rows[8]; unsigned order[8];
 for(unsigned i=0;i<8;i++) {
  unsigned positive=i/4,p=i%4;
  unsigned xy=positive?(a<<2)|p:(a<<2)|b;
  unsigned za=positive?(b<<2)|c:(p<<2)|c;
  rows[i]=(QL_M3_Rotation){(uint8_t)xy,(uint8_t)za,
   (uint8_t)((a<<4)|(p<<2)|c),(uint8_t)positive,0,
   (uint8_t)(xy==za),0,(int8_t)(native_pairs[xy][positive?1:0]+native_pairs[za][positive?0:1])};
  order[i]=i;
 }
 /* Stable ranking matches the retained generator; ordinal order stays distinct
  * from the curated 7/8-state profile. Eight candidates are not eight legal poses. */
 for(unsigned i=1;i<8;i++) {
  unsigned key=order[i],j=i;
  while(j>0) {
   unsigned prev=order[j-1];
   if(rows[prev].value<rows[key].value || (rows[prev].value==rows[key].value && rows[prev].positive<=rows[key].positive)) break;
   order[j]=prev;j--;
  }
  order[j]=key;
 }
 for(unsigned i=0;i<8;i++){rows[order[i]].slot=(uint8_t)i;rows[order[i]].degrees=(uint16_t)(i*45);}
 memcpy(out,rows,sizeof(rows));return QL_M3_OK;
}
QL_M3_Result ql_m3_pose_ordinal(unsigned codon,unsigned slot,uint16_t *out) {
 if(codon>=64 || !out || slot>=native_profiles[codon][0]) return QL_M3_INVALID;
 unsigned ordinal=slot;for(unsigned i=0;i<codon;i++) ordinal+=native_profiles[i][0];
 *out=(uint16_t)ordinal;return QL_M3_OK;
}
QL_M3_Result ql_m3_minor(unsigned card,uint8_t out[4]) {
 if(card>=56 || !out) return QL_M3_INVALID;
 memcpy(out,native_minor[card/14][card%14],4);return QL_M3_OK;
}
const QL_M3_Major *ql_m3_major(unsigned card) { return card<22?&native_major[card]:NULL; }
static QL_M3_Quaternion multiply(QL_M3_Quaternion a,QL_M3_Quaternion b) {
 return (QL_M3_Quaternion){a.w*b.w-a.x*b.x-a.y*b.y-a.z*b.z,
 a.w*b.x+a.x*b.w+a.y*b.z-a.z*b.y,a.w*b.y-a.x*b.z+a.y*b.w+a.z*b.x,
 a.w*b.z+a.x*b.y-a.y*b.x+a.z*b.w};
}
QL_M3_Result ql_m3_quaternion(unsigned codon,unsigned state,QL_M3_Quaternion *out) {
 if(codon>=64 || state>=8 || !out) return QL_M3_INVALID;
 QL_M3_CodonRecord c;if(ql_m3_codon(codon,&c)!=QL_M3_OK) return QL_M3_INVALID;
 QL_M3_Quaternion q={(float)c.charges[0],(float)((int)c.values[0]-(int)c.values[2]),0.0f,(float)(c.charges[0]%6)};
 if(state) {float angle=(float)state*0.7853981633974483f; q=multiply((QL_M3_Quaternion){cosf(angle*0.5f),sinf(angle*0.5f),0,0},q);}
 *out=q;return QL_M3_OK;
}
QL_M3_Result ql_m3_active_state(QL_M3_Quaternion env,unsigned codon,uint8_t *out) {
 if(codon>=64 || !out || !isfinite(env.w) || !isfinite(env.x) || !isfinite(env.y) || !isfinite(env.z)) return QL_M3_INVALID;
 QL_M3_Quaternion q;if(ql_m3_quaternion(codon,0,&q)!=QL_M3_OK) return QL_M3_INVALID;
 q=multiply(env,q);
 if(!isfinite(q.w) || !isfinite(q.x) || (q.w==0 && q.x==0)) return QL_M3_INVALID;
 float angle=atan2f(q.x,q.w);if(angle<0) angle+=6.2831853071795865f;
 *out=(uint8_t)((uint8_t)(angle/0.7853981633974483f)&7u);return QL_M3_OK;
}
QL_M3_Result ql_m3_transcribe(unsigned codon,unsigned rna,char out[4]) {
 if(codon>=64 || rna>1 || !out) return QL_M3_INVALID;
 const char *alphabet=rna?"AUCG":"ATCG";
 out[0]=alphabet[codon>>4];out[1]=alphabet[(codon>>2)&3];out[2]=alphabet[codon&3];out[3]='\0';return QL_M3_OK;
}
QL_M3_Result ql_m3_transduce(const uint8_t *indices,size_t count,uint64_t *out) {
 if(!out || (count && !indices)) return QL_M3_INVALID;
 uint64_t result=0;
 for(size_t i=0;i<count;i++) {unsigned idx=indices[i];if(idx>=72) return QL_M3_INVALID;
  unsigned target=idx<64?idx:(idx-64)*8;result|=UINT64_C(1)<<target;}
 *out=result;return QL_M3_OK;
}
QL_M3_Result ql_m3_epogdoon(unsigned index,uint8_t *compressed,uint8_t *roundtrip_gap) {
 if(index>=72 || !compressed || !roundtrip_gap || compressed==roundtrip_gap) return QL_M3_INVALID;
 unsigned c=index*8/9;*compressed=(uint8_t)c;*roundtrip_gap=(uint8_t)(c*9/8!=index);return QL_M3_OK;
}
QL_M3_Result ql_m3_clock(uint64_t steps,QL_M3_Clock *out) {
 if(!out) return QL_M3_INVALID;
 QL_M3_Clock r;memset(&r,0,sizeof(r));r.steps=steps;r.completed_double_covers=steps/720;
 r.degree720=(uint16_t)(steps%720);r.degree360=(uint16_t)(steps%360);r.layer=(uint8_t)(r.degree720/360);
 r.polar720=(uint16_t)(r.layer*360+(r.degree360+180)%360);
 r.tick12=(uint8_t)(r.degree360/30);r.decan_phase=(uint8_t)(r.degree360/10+r.layer*36);
 r.uniform_hexagram_estimate=(uint8_t)(r.degree360*64/360);
 r.degree_node=native_group_9[r.degree360];r.backbone_node=native_anchors[r.degree360];
 r.clockwise_node=native_group_9[(r.degree360+1)%360];r.polar_node=native_group_9[(r.degree360+180)%360];
 *out=r;return QL_M3_OK;
}
QL_M3_Result ql_m3_clock_advance(uint64_t steps,uint64_t delta,QL_M3_Clock *out) {
 if(delta>UINT64_MAX-steps) return QL_M3_INVALID;
 return ql_m3_clock(steps+delta,out);
}
QL_M3_Result ql_m3_clock_record(unsigned degree,uint16_t out[QL_M3_CLOCK_FIELDS]) {
 if(degree>=360 || !out) return QL_M3_INVALID;
 memcpy(out,native_clock[degree],sizeof(native_clock[degree]));return QL_M3_OK;
}
uint32_t ql_m3_clock_reconciled_symbolic_fields(void) { return 0; }
