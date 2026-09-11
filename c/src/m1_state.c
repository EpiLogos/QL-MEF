/* K4/K5 coordinate-backed state and full Ananda source access. */
#include "ql/m1.h"
#include "ql/kernel.h"
#include <math.h>
#include <stdio.h>
#include "m1_source_data.inc"
static QL_M_NodeId seat(const char *ref) {
    const QL_M_Node *n = ql_m1_node(ref);
    return n ? n->id : QL_M_NO_ID;
}
int ql_m1_source_cell(uint32_t f, uint32_t r, uint32_t c, QL_M1_SourceCell *out) {
    if (!out || f>=6u || r>=12u || c>=12u) return 0;
    *out = source_cells[f*144u+r*12u+c];
    return 1;
}
int ql_m1_reflection(QL_M_NodeId subject, uint32_t phase, QL_M1_Reflection *out) {
    QL_M1_Reflection v;
    const QL_M_Node *n = ql_m_node_by_id(subject);
    if (!out || phase>1u || !n || n->root_position!=1u) return 0;
    v.coordinate=seat(phase ? "#1-1" : "#1-0");
    v.subject=subject; v.source_ground=seat("#1-0"); v.reflection_ground=seat("#1-1");
    v.phase=phase; v.conjugate_phase=1u-phase;
    if (!v.coordinate || !v.source_ground || !v.reflection_ground) return 0;
    *out=v; return 1;
}
int ql_m1_grammar(QL_M1_Grammar *out) {
    QL_M1_Grammar v={0}; uint32_t i; char ref[32];
    if (!out) return 0;
    for (i=0;i<6u;++i) {
        (void)snprintf(ref,sizeof(ref),"#1-4.%u",(unsigned)i);
        v.coordinates[i]=seat(ref); if (!v.coordinates[i]) return 0;
        v.inversion[i]=ql_position_invert((uint8_t)i);
    }
    v.explicate=4u; v.processual=QL_POSITION_COUNT;
    v.ratio_num=v.explicate*v.explicate; v.ratio_den=3u*3u;
    v.decimal_frame=v.explicate+v.processual; v.ring_positions=QL_TICK_COUNT;
    for (i=0;i<4u;++i) v.nesting[i]=v.processual+i+1u;
    v.binary_states=UINT32_C(1)<<v.processual;
    v.relation_states=v.processual*v.processual;
    v.resonance_states=v.relation_states*QL_FACE_COUNT;
    v.cosmic_degrees=v.processual*v.decimal_frame*v.processual;
    v.retained_one_total=1u+v.binary_states+v.resonance_states;
    v.genus=1u; v.euler_characteristic=2u-2u*v.genus;
    *out=v; return 1;
}
int ql_m1_rotor(double a, double b, QL_M1_Rotor *out) {
    QL_M1_Rotor v; QL_Quaternion qa,qb,q;
    if (!out || !isfinite(a) || !isfinite(b)) return 0;
    v.coordinate=seat("#1-5-0"); if (!v.coordinate) return 0;
    qa=(QL_Quaternion){(float)cos(a),(float)sin(a),0,0};
    qb=(QL_Quaternion){(float)cos(b),0,(float)sin(b),0};
    q=ql_quat_multiply(qa,qb);
    v.quaternion[0]=q.w;v.quaternion[1]=q.x;v.quaternion[2]=q.y;v.quaternion[3]=q.z;
    *out=v; return 1;
}
int ql_m1_carrier(uint64_t cycle,uint32_t tick,QL_M1_Carrier *out) {
    QL_M1_Carrier v={0}; QL_M1_Rotor r; uint32_t i; char ref[32]; double angle;
    if (!out || !ql_m1_clock(cycle,tick,&v.clock)) return 0;
    for (i=0;i<6u;++i) {
        (void)snprintf(ref,sizeof(ref),"#1-5-%u",(unsigned)i);
        v.coordinates[i]=seat(ref); if (!v.coordinates[i]) return 0;
    }
    angle=(double)v.clock.degree720*(acos(-1.0)/180.0);
    if (!ql_m1_rotor(angle/2.0,0.0,&r)) return 0;
    for (i=0;i<4u;++i) v.spinor[i]=r.quaternion[i];
    v.quadrature[0]=cos(angle);v.quadrature[1]=sin(angle);
    v.opposite_quadrature[0]=-v.quadrature[0];v.opposite_quadrature[1]=-v.quadrature[1];
    v.m2_carrier_count=QL_RESONANCE_COUNT;v.genus=1u;
    v.explicate_edges=4u*v.genus;v.identification_slots=2u*v.genus;
    *out=v;return 1;
}
int ql_m1_clock_advance(uint64_t cycle,uint32_t tick,uint64_t ticks,QL_M1_Clock *out) {
    uint64_t cycles,rem,carry;
    if (!out || tick>=12u) return 0;
    cycles=ticks/12u;rem=ticks%12u+tick;carry=rem/12u;
    if (UINT64_MAX-cycle<cycles || UINT64_MAX-(cycle+cycles)<carry) return 0;
    return ql_m1_clock(cycle+cycles+carry,(uint32_t)(rem%12u),out);
}
size_t ql_m1_relation_count(QL_M_NodeId coordinate) {
    size_t i,count=0;const QL_M_Node *n=ql_m_node_by_id(coordinate);
    if (!n || n->root_position!=1u) return 0;
    for(i=0;i<ql_m_relation_count();++i) {
        const QL_M_Relation *r=ql_m_relation_at(i);
        if(r->from_id==coordinate || r->to_id==coordinate) ++count;
    }
    return count;
}
const QL_M_Relation *ql_m1_relation_at(QL_M_NodeId coordinate,size_t index) {
    size_t i;const QL_M_Node *n=ql_m_node_by_id(coordinate);
    if (!n || n->root_position!=1u) return NULL;
    for(i=0;i<ql_m_relation_count();++i) {
        const QL_M_Relation *r=ql_m_relation_at(i);
        if(r->from_id==coordinate || r->to_id==coordinate) {
            if(index==0u) return r;
            --index;
        }
    }
    return NULL;
}

int ql_m1_finite_field(uint32_t left,uint32_t right,uint32_t direct_word,uint32_t prime_word,QL_M1_FiniteField *out) {
    QL_M1_FiniteField v={0};
    if(!out || left>=QL_POSITION_COUNT || right>=QL_POSITION_COUNT || direct_word>=QL_STATE6_COUNT || prime_word>=QL_STATE6_COUNT) return 0;
    v.coordinate=seat("#1-4");if(!v.coordinate)return 0;
    v.left_position=left;v.right_position=right;v.direct_word=direct_word;v.prime_word=prime_word;
    v.relation_index=ql_relation_index((uint8_t)left,(uint8_t)right);
    v.pair_index=direct_word*QL_STATE6_COUNT+prime_word;
    v.direct_face_index=direct_word;v.prime_face_index=QL_STATE6_COUNT+prime_word;
    v.bitwise_complement=ql_state6_complement((uint8_t)direct_word);
    v.binary_states=QL_STATE6_COUNT;v.two_face_states=QL_STATE6_COUNT*QL_FACE_COUNT;
    v.paired_states=QL_STATE6_COUNT*QL_STATE6_COUNT;*out=v;return 1;
}

/* Source tables are distinct from live state; M2/M3 retain their native engines. */
int ql_m1_source_traits(QL_M1_SourceTraits *out) {
    static const uint32_t category[6]={0,1,2,5,4,6};
    static const uint32_t doubling[6]={1,2,4,8,7,5};
    static const uint32_t tripling[6]={3,6,9,3,6,9};
    QL_M1_SourceTraits v={0};char ref[16];
    if(!out)return 0;
    for(uint32_t i=0;i<6u;++i){
        (void)snprintf(ref,sizeof(ref),"#1-%u",(unsigned)i);v.coordinates[i]=seat(ref);if(!v.coordinates[i])return 0;
        v.branch_categories[i]=category[i];v.mahamaya_ring[i]=doubling[i];v.parashakti_ring[i]=tripling[i];
    }
    v.unary_mask=1;v.binary_mask=2;v.relational_mask=4;*out=v;return 1;
}
