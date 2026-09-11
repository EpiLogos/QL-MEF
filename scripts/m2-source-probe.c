/* Executed retained-source oracle. Never link this into the native kernel. */
#include "m2.h"
#include <inttypes.h>
#include <stdio.h>
#define START(t, i) printf("%s\t%u", t, (unsigned)(i))
#define V(x) printf("\t%" PRIu64, (uint64_t)(x))
#define END() putchar('\n')
int main(void) {
    unsigned i, j;
    for (i=0;i<72;i++) { START("carrier",i); V(M2_ARCHETYPES.raw_vibration[i]); END(); }
    for (i=0;i<72;i++) { const MEF_Condition_Desc *r=&M2_MEF_DESC[i]; START("mef",i); V(r->lens); V(r->position); V(r->is_inverted); V(r->l_family_link); V(r->meaning_id); END(); }
    for (i=0;i<36;i++) { const Tattva_Entry_Desc *r=&M2_TATTVA_DESC[i]; START("tattva",i); V(r->index); V(r->division); V(r->element_id); V(r->kanchuka_mask); V(r->meaning_id); END(); }
    for (i=0;i<73;i++) { const Decan_Face_Desc *r=i==72?&M2_QUINTESSENCE_DECAN:&M2_DECAN_DESC[i]; START("decan",i); V(r->element); V(r->sign); V(r->decan); V(r->face); V(r->ruling_planet); V(r->meaning_id); END(); }
    for (i=0;i<10;i++) { const Planet_Operator *r=&M2_PLANET_LUT[i]; START("planet",i); V(r->id); V(r->group_type); V(r->prime); V(r->elem_sig); V(r->cousto_freq); V(r->keplerian_vel); V(r->digital_root); V(r->ananda_row); V(r->meaning_id); END(); }
    for (i=0;i<8;i++) { const Chakra_Descriptor *r=&M2_CHAKRA_LUT[i]; START("chakra",i); V(r->id); V(r->element_id); V(r->tattva_idx); V(r->meaning_id); END(); }
    for (i=0;i<72;i++) { const Shem_Name_Desc *r=&M2_SHEM_DESC[i]; START("shem",i); V(r->shem_idx); V(r->choir); V(r->position); V(r->element_id); V(r->decan_link); V(r->planet_link); V(r->meaning_id); END(); }
    for (i=0;i<10;i++) { START("ratio",i); V(M2_MAQAM_RATIOS[i].num); V(M2_MAQAM_RATIOS[i].den); END(); }
    for (i=0;i<72;i++) { const Maqam_Musical_Desc *r=&M2_MAQAM_DESC[i]; START("maqam",i); V(r->family); V(r->mode_in_family); for(j=0;j<7;j++) V(r->intervals[j]); V(r->planet_ruler); V(r->meaning_id); END(); }
    for (i=0;i<24;i++) { const Maqam_Spiritual_Desc *r=&M2_MAQAM_SPIRITUAL[i/3][i%3]; START("station",i); V(r->station); V(r->level); V(r->meaning_id); END(); }
    for (i=0;i<100;i++) { const Asma_Name_Desc *r=&M2_ASMA_LUT[i]; START("asma",i); V(r->name_idx); V(r->group); V(r->index_in_group); V(r->element_id); V(r->digital_root); V(r->mirror_idx); V(r->abjad_value); V(r->meaning_id); END(); }
    for (i=0;i<100;i++) { const Mantra_Entry_Desc *r=&M2_MANTRA_LUT[i]; START("mantra",i); V(r->mantra_idx); V(r->matrika_group); V(r->element_id); V(r->phase); V(r->fundamental_frequency); V(r->meaning_id); END(); }
    for(i=0;i<5;i++) { const Element_Throughline *r=&M2_ELEMENTS[i]; START("element",i); V(r->tattva_idx); V(r->decan_element); V(r->mantra_group); V(r->chakra_idx); END(); }
    for(i=0;i<72;i++) { START("det",i); V(M2_TO_M3_CYMATIC_PROJECTION[i]); END(); }
    for(i=0;i<36;i++) { START("resonance",i); V(M2_CAUSAL_RESONANCE_MASKS[i]); END(); }
    START("routing",0); V(ASMA_36_INTERNAL_MASK.low_64); V(ASMA_36_INTERNAL_MASK.high_64); END();
    START("routing",1); V(ASMA_64_PROJECTIVE_MASK.low_64); V(ASMA_64_PROJECTIVE_MASK.high_64); END();
    return ferror(stdout)?1:0;
}
