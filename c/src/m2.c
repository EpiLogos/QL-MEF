#include "ql/m2.h"
#include <math.h>
#include <string.h>
#include "m2_data.inc"
#include "m2_correspondence_data.inc"
_Static_assert(12*6==72 && 36*2==72 && 4*3*3*2==72 && 8*9==72, "native 72 readings");
_Static_assert(5+7+24==36 && 4*18==72 && 4*16==64, "native strata and fibres");
_Static_assert(99+1==100 && 50+50==100 && 8*3==24, "native symbolic cardinalities");
_Static_assert(sizeof(m2_tables)/sizeof(m2_tables[0])==QL_M2_TABLE_COUNT, "table ABI");
static const QL_M2_Ground ground = {
    {-1,-1,1}, {5,7,24}, {12,6}, {36,2}, {4,3,3,2}, {8,9},
    {4,18}, {4,16}, {99,1}, {50,50}, {8,3}
};
const QL_M2_Ground *ql_m2_ground(void) { return &ground; }
const char *ql_m2_registry_revision(void) { return M2_DATA_REGISTRY; }
const QL_M2_Table *ql_m2_table(unsigned table) {
    return table<QL_M2_TABLE_COUNT ? &m2_tables[table] : NULL;
}
const QL_M2_Record *ql_m2_record(unsigned table, size_t row) {
    const QL_M2_Table *t=ql_m2_table(table);
    return t && row<t->row_count ? &t->rows[row] : NULL;
}
size_t ql_m2_exact_record_count(QL_M_NodeId coordinate) {
    size_t i,j,count=0;
    const QL_M_Node *n=ql_m_node_by_id(coordinate), *root=ql_m_root(2);
    if (!n || !root || n->root_id!=root->id) return 0;
    for (i=0;i<QL_M2_TABLE_COUNT;i++)
        for (j=0;j<m2_tables[i].row_count;j++)
            count+=(m2_tables[i].rows[j].coordinate_id==coordinate);
    return count;
}
QL_M2_Result ql_m2_flatten(unsigned r,unsigned a,unsigned b,unsigned c,unsigned d,uint8_t *out) {
    unsigned v;
    if (!out) return QL_M2_INVALID;
    switch(r) {
        case QL_M2_MEF: if(a>=12||b>=6||c||d) return QL_M2_INVALID; v=a*6+b; break;
        case QL_M2_TATTVA: if(a>=36||b>=2||c||d) return QL_M2_INVALID; v=a*2+b; break;
        case QL_M2_DECAN: if(a>=4||b>=3||c>=3||d>=2) return QL_M2_INVALID; v=a*18+b*6+c*2+d; break;
        case QL_M2_SHEM: if(a>=8||b>=9||c||d) return QL_M2_INVALID; v=a*9+b; break;
        default: return QL_M2_INVALID;
    }
    *out=(uint8_t)v; return QL_M2_OK;
}
QL_M2_Result ql_m2_unflatten(unsigned r,unsigned i,uint8_t out[4]) {
    uint8_t v[4]={0,0,0,0};
    if(!out||i>=72) return QL_M2_INVALID;
    switch(r) {
        case QL_M2_MEF: v[0]=(uint8_t)(i/6);v[1]=(uint8_t)(i%6);break;
        case QL_M2_TATTVA:v[0]=(uint8_t)(i/2);v[1]=(uint8_t)(i%2);break;
        case QL_M2_DECAN:v[0]=(uint8_t)(i/18);v[1]=(uint8_t)(i%18/6);v[2]=(uint8_t)(i%6/2);v[3]=(uint8_t)(i%2);break;
        case QL_M2_SHEM:v[0]=(uint8_t)(i/9);v[1]=(uint8_t)(i%9);break;
        default:return QL_M2_INVALID;
    }
    memcpy(out,v,sizeof(v));return QL_M2_OK;
}
QL_M2_Result ql_m2_signature(unsigned e,unsigned c,unsigned p,uint8_t *out) {
    if(!out||e>=5||c>=8||p>=4) return QL_M2_INVALID;
    *out=(uint8_t)(e|(c<<3)|(p<<6));return QL_M2_OK;
}
QL_M2_Result ql_m2_unpack_signature(unsigned s,uint8_t out[3]) {
    uint8_t v[3];
    if(!out||s>255||(s&7)>=5) return QL_M2_INVALID;
    v[0]=(uint8_t)(s&7);v[1]=(uint8_t)((s>>3)&7);v[2]=(uint8_t)(s>>6);
    memcpy(out,v,sizeof(v));return QL_M2_OK;
}
QL_M2_Result ql_m2_tattva_step(unsigned i,unsigned p,uint8_t *out) {
    if(!out||i>=36||p>=2) return QL_M2_INVALID;
    if((p==0&&i==35)||(p==1&&i==0)) return QL_M2_BOUNDARY;
    *out=(uint8_t)(p==0?i+1:i-1);return QL_M2_OK;
}
QL_M2_Result ql_m2_decan_to_fibre(unsigned i,uint8_t *out) {
    static const unsigned order[4]={1,0,3,2};
    if(!out||i>=72) return QL_M2_INVALID;
    *out=(uint8_t)(order[i/18]*18+i%18);return QL_M2_OK;
}
QL_M2_Result ql_m2_fibre_target(unsigned i,uint8_t *out) {
    if(!out||i>=72) return QL_M2_INVALID;
    *out=(uint8_t)(i/18*16+(i%18<16?i%18:(i%18-16)*8));return QL_M2_OK;
}
QL_M2_Result ql_m2_scalar_compress(unsigned i,uint8_t *out) {
    if(!out||i>=72) return QL_M2_INVALID;
    *out=(uint8_t)(i*8/9);return QL_M2_OK;
}
QL_M2_Result ql_m2_scalar_expand(unsigned i,uint8_t *out) {
    if(!out||i>=64) return QL_M2_INVALID;
    *out=(uint8_t)(i*9/8);return QL_M2_OK;
}
QL_M2_Result ql_m2_legacy_det(const uint8_t *indices,size_t count,uint64_t *out) {
    size_t i;uint64_t mask=0;
    if(!out||(!indices&&count)) return QL_M2_INVALID;
    for(i=0;i<count;i++) {
        if(indices[i]>=72) return QL_M2_INVALID;
        mask|=m2_det_rows[indices[i]].values[0];
    }
    *out=mask;return QL_M2_OK;
}
QL_M2_Result ql_m2_asma_route(unsigned i,uint8_t *out) {
    if(!out||i>=100) return QL_M2_INVALID;
    *out=(uint8_t)((m2_routing_rows[1].values[i/64]>>(i%64))&1);
    return QL_M2_OK;
}
QL_M2_Result ql_m2_planet_preempted(unsigned index,uint8_t *out) {
    if(!out||index>=10) return QL_M2_INVALID;
    *out=(uint8_t)(index>=7);return QL_M2_OK;
}
uint8_t ql_m2_digital_root(uint64_t n) { return (uint8_t)(n?1+(n-1)%9:0); }
QL_M2_Result ql_m2_aspect(double a,double b,QL_M2_Aspect *out) {
    static const double angle[5]={0,60,90,120,180},orb[5]={10,6,8,8,10};
    QL_M2_Aspect v; unsigned i;
    if(!out) return QL_M2_INVALID;
    if(!isfinite(a)||!isfinite(b)) return QL_M2_NONFINITE;
    if(a<0||a>=360||b<0||b>=360) return QL_M2_INVALID;
    v.angle=fabs(a-b);if(v.angle>180) v.angle=360-v.angle;
    v.type=255;v.orb=999;
    for(i=0;i<5;i++) {
        double d=fabs(v.angle-angle[i]);
        if(d<=orb[i]&&d<v.orb) {v.type=(uint8_t)i;v.orb=d;}
    }
    *out=v;return QL_M2_OK;
}
QL_M2_Result ql_m2_maqam_pitch(unsigned mode,unsigned degree,double root,double *out) {
    unsigned i;uint64_t steps=0;double pitch;
    if(!out||mode>=72||degree>=8) return QL_M2_INVALID;
    if(!isfinite(root)) return QL_M2_NONFINITE;
    if(root<=0) return QL_M2_INVALID;
    for(i=0;i<degree;i++) steps+=m2_maqam_rows[mode].values[2+i];
    pitch=root*exp2((double)steps/24.0);
    if(!isfinite(pitch)) return QL_M2_NONFINITE;
    *out=pitch;return QL_M2_OK;
}
static int valid_amplitudes(const QL_M2_Amplitude *in,size_t n,int64_t limit) {
    size_t i;
    if(!in) return 0;
    for(i=0;i<n;i++) if(in[i].re < -limit || in[i].re > limit || in[i].im < -limit || in[i].im > limit) return 0;
    return 1;
}
QL_M2_Result ql_m2_modal_quadrature(const QL_M2_Amplitude in[72],QL_M2_Amplitude out[72]) {
    QL_M2_Amplitude v[72];size_t i;
    if(!out||!valid_amplitudes(in,72,QL_M2_MAX_COMPONENT)) return QL_M2_INVALID;
    for(i=0;i<72;i++) {v[i].re=-in[i].im;v[i].im=in[i].re;}
    memcpy(out,v,sizeof(v));return QL_M2_OK;
}
QL_M2_Result ql_m2_modal_transduce(const QL_M2_Amplitude in[72],QL_M2_Amplitude out[64]) {
    QL_M2_Amplitude v[64]={{0,0}};size_t i;
    if(!out||!valid_amplitudes(in,72,QL_M2_MAX_COMPONENT)) return QL_M2_INVALID;
    for(i=0;i<72;i++) {size_t j=i/18*16+(i%18<16?i%18:(i%18-16)*8);v[j].re+=in[i].re;v[j].im+=in[i].im;}
    memcpy(out,v,sizeof(v));return QL_M2_OK;
}
QL_M2_Result ql_m2_modal_power(const QL_M2_Amplitude *in,size_t count,uint64_t *out) {
    size_t i;uint64_t sum=0;
    if(!out||count>72||!valid_amplitudes(in,count,2*QL_M2_MAX_COMPONENT)) return QL_M2_INVALID;
    for(i=0;i<count;i++) sum+=(uint64_t)(in[i].re*in[i].re)+(uint64_t)(in[i].im*in[i].im);
    *out=sum;return QL_M2_OK;
}

/* Port of the retained portal-core M2-1 formula; supplied M1 ratio/M3 seed. */
static unsigned m2_pitch(unsigned tick) { return tick < 6u ? tick*2u : (tick-6u)*2u+1u; }
QL_M2_Result ql_m2_vimarsha(const QL_M2_VimarshaSeed *in, QL_M2_VimarshaReading *out) {
    static const unsigned intervals[7]={0,2,4,5,7,9,11}, offsets[8]={2,4,6,8,3,5,7,9};
    QL_M2_VimarshaReading value;
    unsigned t,l,m,r,pos,ms,ns[4],mseed[4]; size_t i;
    float texture;
    if (!in || !out || in->tick12>=12 || in->lens>=12 || in->musical_mode>=7 ||
        !in->ratio_num || !in->ratio_den || in->codon>=64 || in->rotation>=8) return QL_M2_INVALID;
    t=in->tick12; l=in->lens; m=in->musical_mode; r=in->rotation;
    texture=(float)in->ratio_num/(float)in->ratio_den;
    for(i=0;i<8;++i) {
        float breath=(float)intervals[(i+m)%7u]/7.0f;
        float semitones=(float)m2_pitch(l)+(float)offsets[i]+(float)intervals[m]+(float)m2_pitch(t)/12.0f+
            breath+(i<4?0.0f:1.0f)+(t<6?0.0f:12.0f);
        float cents=(float)l*3.0f+(float)m*5.0f+(float)r;
        value.audio_octet_hz[i]=130.81279f*powf(2.0f,semitones/12.0f)*texture*powf(2.0f,cents/1200.0f);
    }
    pos=t%6u; ms=l+m+pos+r+in->codon%12u;
    mseed[0]=ms; mseed[1]=ms+5u; mseed[2]=ms+1u; mseed[3]=ms+6u;
    ns[0]=l+pos; ns[1]=m+t; ns[2]=l+m+4u; ns[3]=t+r+6u;
    for(i=0;i<4;++i) { value.nodal_quartet[i].ql_position=(uint8_t)((i%2u)*5u);
        value.nodal_quartet[i].helix=(uint8_t)(i/2u); value.nodal_quartet[i].m=(uint8_t)(1u+mseed[i]%12u);
        value.nodal_quartet[i].n=(uint8_t)(1u+ns[i]%12u); }
    *out=value; return QL_M2_OK;
}

size_t ql_m2_correspondence_count(void) {
    return sizeof(m2_correspondences)/sizeof(m2_correspondences[0]);
}
const QL_M2_Correspondence *ql_m2_correspondence_at(size_t index) {
    return index<ql_m2_correspondence_count() ? &m2_correspondences[index] : NULL;
}
const QL_M2_Correspondence *ql_m2_correspondence(uint8_t index,uint8_t role) {
    size_t i;
    if (index>=72 || role>=2) return NULL;
    for (i=0;i<ql_m2_correspondence_count();++i)
        if (m2_correspondences[i].maqam_index==index && m2_correspondences[i].role==role)
            return &m2_correspondences[i];
    return NULL;
}
QL_M2_Result ql_m2_correspondence_pitch(uint8_t index,uint8_t role,
    uint8_t tuning,uint8_t degree,double tonic,double *out) {
    const QL_M2_Correspondence *r;
    double value;
    if (!out || index>=72 || role>=2 || tuning>=2 || degree>=8 || tonic<=0.0)
        return QL_M2_INVALID;
    if (!isfinite(tonic)) return QL_M2_NONFINITE;
    if (tuning==0) return ql_m2_maqam_pitch(index,degree,tonic,out);
    r=ql_m2_correspondence(index,role);
    if (!r || !r->spelled_supported) return QL_M2_UNAVAILABLE;
    value=tonic*pow(2.0,(double)r->spelled_steps24[degree]/24.0);
    if (!isfinite(value)) return QL_M2_NONFINITE;
    *out=value;
    return QL_M2_OK;
}
