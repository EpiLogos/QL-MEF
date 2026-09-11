#include "ql/m3.h"
#include "compat/m3-reference.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>
/* Retained source has further non-constant C11 static assertions in m3.c.
 * Suppression is confined to this reference TU. Native C builds strictly. */
#define _Static_assert(...) /* reference-only */
#include "../../vendor/epi-kernel/reference/src/m3.c"
#include "../../vendor/epi-kernel/reference/src/m3_clock_lut.c"
#undef _Static_assert
int main(void) {
 unsigned checks=0;
 for(unsigned i=0;i<720;i++) {
  QL_M3_Clock native;assert(ql_m3_clock(i,&native)==QL_M3_OK);
  Unified_Clock_State reference=m0_read_cosmic_clock((uint16_t)i);
  assert(native.tick12==reference.tick12);
  assert(native.layer==reference.is_implicate_phase);
  assert(native.decan_phase==reference.m2_decan_phase);
  assert(native.uniform_hexagram_estimate==reference.m3_hexagram_id);checks+=4;
 }
 for(unsigned i=0;i<360;i++) {
  uint16_t native[26];assert(ql_m3_clock_record(i,native)==QL_M3_OK);
  const Clock_Degree_Entry *r=&CLOCK_DEGREE_LUT[i];
  uint16_t reference[]={r->degree_node_360,(uint16_t)r->exact_degree_720,r->zodiac_sign,r->zodiac_degree,r->decan_idx,r->decan_position,r->is_backbone_node,r->hexagram_id,r->hexagram_line_active,r->is_non_dual_codon,r->codon_class,r->codon_upper_pair,r->codon_lower_pair,r->tarot_card_id,r->decan_planet,r->decan_element,r->decan_chakra,r->tick12,r->strand,r->dr_ring,r->m1_ananda_value,r->m0_archetype,r->shadow_degree,r->polar_opposite,r->enneadic_chamber,r->chamber_day_night};
  assert(!memcmp(native,reference,sizeof(reference)));checks+=26;
 }
 for(unsigned i=0;i<64;i++) {
  QL_M3_CodonRecord native;assert(ql_m3_codon(i,&native)==QL_M3_OK);
  M3_CodonEvaluation e=evaluate_codon((uint8_t)i);
  assert(native.charges[0]==e.pp && native.charges[1]==e.mm && native.charges[2]==e.mp && native.charges[3]==e.pm);
  assert(native.amino_record==M3_CODON_TO_AA[i]);checks+=5;
  QL_M3_Rotation rotations[8];M3_Rotational_Generation reference[8];assert(ql_m3_rotations(i,rotations)==QL_M3_OK);assert(m3_generate_rotational_states((uint8_t)i,reference)==8);
  for(unsigned j=0;j<8;j++) {
   assert(rotations[j].pair1==reference[j].pair1_idx && rotations[j].pair2==reference[j].pair2_idx && rotations[j].codon==reference[j].resulting_codon);
   assert(rotations[j].positive==reference[j].polarity && rotations[j].slot==reference[j].rotation_slot && rotations[j].degrees==reference[j].rotation_degrees && rotations[j].value==reference[j].rotational_value && rotations[j].nondual==reference[j].is_non_dual);checks+=8;
   QL_M3_Quaternion q;assert(ql_m3_quaternion(i,j,&q)==QL_M3_OK);Quaternion rq=m3_quat_codon_state((uint8_t)i,(uint8_t)j);
   assert(fabsf(q.w-rq.w)<0.00001f && fabsf(q.x-rq.x)<0.00001f && fabsf(q.y-rq.y)<0.00001f && fabsf(q.z-rq.z)<0.00001f);checks+=4;
  }
 }
 printf("executed retained-C/native-C parity: %u field comparisons\n",checks);
 return 0;
}
