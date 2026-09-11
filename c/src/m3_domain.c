#include "ql/m3_domain.h"
#include <string.h>
#include "m3_domain_data.inc"
#define COUNT(a) (sizeof(a)/sizeof((a)[0]))
const char *ql_m3_domain_revision(void) { return domain_revision; }
size_t ql_m3_source_node_count(void) { return COUNT(domain_nodes); }
const QL_M3_SourceNode *ql_m3_source_node_at(size_t i) { return i<COUNT(domain_nodes)?&domain_nodes[i]:NULL; }
size_t ql_m3_source_relation_count(void) { return COUNT(domain_relations); }
const QL_M3_SourceRelation *ql_m3_source_relation_at(size_t i) { return i<COUNT(domain_relations)?&domain_relations[i]:NULL; }
size_t ql_m3_matrix_cell_count(void) { return COUNT(domain_cells); }
const QL_M3_MatrixCell *ql_m3_matrix_cell_at(size_t i) { return i<COUNT(domain_cells)?&domain_cells[i]:NULL; }
const QL_M3_SourceRelation *ql_m3_matrix_cell_relation(size_t cell,size_t link) {
    const QL_M3_MatrixCell *c=ql_m3_matrix_cell_at(cell);
    return c && link<(size_t)c->pair_count+c->codon_count ?
        &domain_relations[domain_cell_links[c->link_offset+link]]:NULL;
}
const QL_M3_BackboneProjection *ql_m3_backbone_projection(QL_M_NodeId id) {
    for(size_t i=0;i<COUNT(domain_backbones);i++) if(domain_backbones[i].backbone==id) return &domain_backbones[i];
    return NULL;
}
QL_M3_Result ql_m3_clock_projection(uint64_t steps,QL_M3_BackboneProjection *out) {
    if(!out) return QL_M3_INVALID;
    QL_M3_Clock clock;
    if(ql_m3_clock(steps,&clock)!=QL_M3_OK) return QL_M3_INVALID;
    const QL_M3_BackboneProjection *p=ql_m3_backbone_projection(clock.backbone_node);
    if(!p) return QL_M3_GAP;
    *out=*p;return QL_M3_OK;
}
QL_M3_Result ql_m3_form(unsigned address,unsigned pose,unsigned aperture,unsigned phase,unsigned axis,QL_M3_Form *out) {
    if(!out || address>=64 || aperture>=16 || phase>=60 || axis>=3) return QL_M3_INVALID;
    QL_M3_CodonRecord codon;uint16_t ordinal;
    if(ql_m3_codon(address,&codon)!=QL_M3_OK || ql_m3_pose_ordinal(address,pose,&ordinal)!=QL_M3_OK) return QL_M3_INVALID;
    /* Rust's accepted 18-lens canon: 16 divisor lenses + Fibonacci ground +
     * Anuttara void ring. Promoted into C here, not copied from stale C prose. */
    static const int32_t divisions[16]={10,20,40,80,90,100,120,150,240,300,360,400,450,900,1800,3600};
    static const uint8_t elements[4]={2,1,0,3}; /* A/T/C/G -> Water/Fire/Earth/Air */
    QL_M3_Form f;memset(&f,0,sizeof(f));
    f.address=(uint8_t)address;f.pose=(uint8_t)pose;f.state_count=codon.state_count;
    f.aperture=(uint8_t)aperture;f.reciprocal_aperture=(uint8_t)(15-aperture);f.axis=(uint8_t)axis;
    f.pose_ordinal=ordinal;f.fibonacci_phase60=(uint16_t)phase;
    f.pair_xy=(uint8_t)(address>>2);f.pair_yz=(uint8_t)(address&15);f.hinge=codon.nucleotides[1];
    f.pair_angles_deg10[0]=(int32_t)f.pair_xy*225;f.pair_angles_deg10[1]=(int32_t)f.pair_yz*225;
    f.lens_division_deg10=divisions[aperture];f.lens_reciprocal_deg10=36000/divisions[aperture];
    f.void_ring_orientation_deg10=(int32_t)aperture*225;
    for(unsigned i=0;i<3;i++) {
        unsigned n=codon.nucleotides[i];f.angles_deg10[i]=(n&1)?-225:225;
        f.velocities_deg10[i]=(n&2)?0:225;
        f.site_elements[i]=elements[n];f.elemental_counts[elements[n]]++;
    }
    *out=f;return QL_M3_OK;
}
QL_M3_Result ql_m3_form_apply_matrix(const QL_M3_Form *form,unsigned matrix,QL_M3_Form *out) {
    if(!form || !out || matrix>=3) return QL_M3_INVALID;
    QL_M3_Form checked;
    if(ql_m3_form(form->address,form->pose,form->aperture,form->fibonacci_phase60,form->axis,&checked)!=QL_M3_OK) return QL_M3_INVALID;
    uint8_t address;QL_M3_Result result=ql_m3_apply_matrix(matrix,form->address,&address);
    if(result!=QL_M3_OK) return result;
    QL_M3_CodonRecord codon;
    if(ql_m3_codon(address,&codon)!=QL_M3_OK) return QL_M3_INVALID;
    unsigned pose=form->pose<codon.state_count?form->pose:codon.state_count-1u;
    return ql_m3_form(address,pose,form->aperture,form->fibonacci_phase60,matrix,out);
}
QL_M3_Result ql_m3_cast(const int32_t angles[3],const int32_t velocities[3],uint8_t *address) {
    if(!angles || !velocities || !address) return QL_M3_INVALID;
    uint8_t result=0;
    for(unsigned i=0;i<3;i++) result=(uint8_t)((result<<2)|(angles[i]<0?1:0)|(velocities[i]==0?2:0));
    *address=result;return QL_M3_OK;
}
