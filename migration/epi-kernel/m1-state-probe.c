/* Independent native observations for K5 source/coordinate/relation/state parity. */
#include "ql/m1.h"
#include "m1.h" /* frozen source-table witness, never linked into the installed library */
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#define CHECK(x) do { if(!(x)){fprintf(stderr,"M1 state check failed at %d: %s\n",__LINE__,#x);return 1;} }while(0)
static void str(const char *s) {
    if(!s){fputs("null",stdout);return;} putchar('"');
    for(const unsigned char *p=(const unsigned char*)s;*p;++p) {
        if(*p=='"'||*p=='\\'){putchar('\\');putchar(*p);}
        else if(*p<32)printf("\\u%04x",(unsigned)*p);else putchar(*p);
    } putchar('"');
}
static void id(uint64_t n){if(n)printf("\"%016" PRIx64 "\"",n);else fputs("null",stdout);}
static void ids(const QL_M_NodeId *values,size_t n){putchar('[');for(size_t i=0;i<n;++i){if(i)putchar(',');id(values[i]);}putchar(']');}
static void us(const uint32_t *values,size_t n){putchar('[');for(size_t i=0;i<n;++i){if(i)putchar(',');printf("%u",values[i]);}putchar(']');}
static void fs(const float *values,size_t n){putchar('[');for(size_t i=0;i<n;++i){if(i)putchar(',');printf("%.9g",(double)values[i]);}putchar(']');}
static void ds(const double *values,size_t n){putchar('[');for(size_t i=0;i<n;++i){if(i)putchar(',');printf("%.17g",values[i]);}putchar(']');}
int main(void){
    size_t nodes=0,edges=0;
    QL_M1_SourceCell token,old;
    CHECK(ql_m1_source_cell(0,0,0,&token));old=token;
    CHECK(!ql_m1_source_cell(6,0,0,&token)&&!memcmp(&token,&old,sizeof token));
    CHECK(!ql_m1_source_cell(256,0,0,&token)&&!ql_m1_source_cell(0,12,0,&token));
    CHECK(!ql_m1_source_cell(0,0,12,&token)&&!ql_m1_source_cell(0,0,0,NULL));
    for(uint32_t f=0;f<6;++f)for(uint32_t r=0;r<12;++r)for(uint32_t c=0;c<12;++c){
        CHECK(ql_m1_source_cell(f,r,c,&token));
        printf("{\"kind\":\"source\",\"family\":%u,\"row12\":%u,\"col12\":%u,\"raw_literal\":",f,r,c);str(token.raw_literal);
        fputs(",\"digit_root_literal\":",stdout);str(token.digit_root_literal);
        printf(",\"csv_raw_row\":%u,\"csv_dr_row\":%u,\"csv_column\":%u}\n",token.csv_raw_row,token.csv_dr_row,token.csv_column);
    }
    for(size_t i=0;i<ql_m_node_count();++i){
        const QL_M_Node *n=ql_m_node_at(i);if(n->root_position!=1u)continue;++nodes;
        CHECK(ql_m1_node(n->source_ref)==n);
        fputs("{\"kind\":\"node\",\"coordinate\":",stdout);id(n->id);fputs(",\"source_ref\":",stdout);str(n->source_ref);
        fputs(",\"parent_id\":",stdout);id(n->parent_id);fputs(",\"children\":[",stdout);
        for(size_t j=0;j<n->children_count;++j){if(j)putchar(',');id(ql_m_child_at(n->id,j)->id);}
        fputs("],\"records\":[",stdout);for(size_t j=0;j<n->records_count;++j){if(j)putchar(',');printf("%zu",ql_m_node_record_index(n->id,j));}fputs("]}\n",stdout);
        for(uint32_t phase=0;phase<2;++phase){
            QL_M1_Reflection v;CHECK(ql_m1_reflection(n->id,phase,&v));
            fputs("{\"kind\":\"reflection\",\"coordinate\":",stdout);id(v.coordinate);fputs(",\"subject\":",stdout);id(v.subject);
            fputs(",\"source_ground\":",stdout);id(v.source_ground);fputs(",\"reflection_ground\":",stdout);id(v.reflection_ground);
            printf(",\"phase\":%u,\"conjugate_phase\":%u}\n",v.phase,v.conjugate_phase);
        }
        size_t count=ql_m1_relation_count(n->id);
        CHECK(!ql_m1_relation_at(n->id,count));
        for(size_t j=0;j<count;++j){
            const QL_M_Relation *r=ql_m1_relation_at(n->id,j);CHECK(r&&ql_m_relation_by_id(r->id)==r);++edges;
            fputs("{\"kind\":\"relation\",\"subject\":",stdout);id(n->id);
            fputs(",\"value\":{\"id\":",stdout);id(r->id);fputs(",\"relation_ref\":",stdout);str(r->relation_ref);
            fputs(",\"class\":\"bimba-source\",\"source_kind\":",stdout);str(r->source_kind);
            fputs(",\"from_ref\":",stdout);str(r->from_ref);fputs(",\"to_ref\":",stdout);str(r->to_ref);
            fputs(",\"from_id\":",stdout);id(r->from_id);fputs(",\"to_id\":",stdout);id(r->to_id);
            const char *orientation[]={"directed","undirected","unspecified"};
            fputs(",\"orientation\":",stdout);str(orientation[r->orientation]);
            printf(",\"cross_m\":%s,\"record\":%u}}\n",r->cross_m?"true":"false",r->record);
        }
    }
    CHECK(nodes==43);
    CHECK(!ql_m1_relation_count(ql_m_root(2)->id)&&!ql_m1_relation_at(0,0));
    QL_M1_Reflection reflection;
    CHECK(!ql_m1_reflection(ql_m_root(2)->id,0,&reflection));
    CHECK(!ql_m1_reflection(ql_m_root(1)->id,2,&reflection));
    CHECK(!ql_m1_reflection(ql_m_root(1)->id,0,NULL));
    QL_M1_Grammar g;CHECK(ql_m1_grammar(&g)&&!ql_m1_grammar(NULL));
    fputs("{\"kind\":\"grammar\",\"coordinates\":",stdout);ids(g.coordinates,6);
    printf(",\"ratio_num\":%u,\"ratio_den\":%u,\"explicate\":%u,\"processual\":%u,\"decimal_frame\":%u,\"inversion\":",g.ratio_num,g.ratio_den,g.explicate,g.processual,g.decimal_frame);us(g.inversion,6);
    printf(",\"ring_positions\":%u,\"nesting\":",g.ring_positions);us(g.nesting,4);
    printf(",\"binary_states\":%u,\"relation_states\":%u,\"resonance_states\":%u,\"cosmic_degrees\":%u,\"retained_one_total\":%u,\"genus\":%u,\"euler_characteristic\":%u}\n",g.binary_states,g.relation_states,g.resonance_states,g.cosmic_degrees,g.retained_one_total,g.genus,g.euler_characteristic);
    for(int a=-24;a<=24;++a)for(int b=-24;b<=24;++b){
        QL_M1_Rotor v;double pa=(double)a/7.0,pb=(double)b/11.0;
        CHECK(ql_m1_rotor(pa,pb,&v));
        printf("{\"kind\":\"rotor\",\"a\":%.17g,\"b\":%.17g,\"coordinate\":",pa,pb);id(v.coordinate);fputs(",\"quaternion\":",stdout);fs(v.quaternion,4);puts("}");
    }
    for(int a=-12;a<=12;++a)for(int b=-12;b<=12;++b){
        QL_M1_Torus v;double pa=(double)a/3.0,pb=(double)b/5.0;
        CHECK(ql_m1_torus(pa,pb,&v));
        printf("{\"kind\":\"torus\",\"a\":%.17g,\"b\":%.17g,\"coordinate\":",pa,pb);id(v.coordinate);
        printf(",\"x\":%.17g,\"y\":%.17g,\"z\":%.17g}\n",v.x,v.y,v.z);
    }
    QL_M1_Rotor bad;CHECK(!ql_m1_rotor(NAN,0,&bad)&&!ql_m1_rotor(0,INFINITY,&bad)&&!ql_m1_rotor(0,0,NULL));
    const uint64_t cycles[]={0,1,2,UINT64_MAX};
    for(size_t c=0;c<4;++c)for(uint32_t t=0;t<12;++t){
        QL_M1_Carrier v;CHECK(ql_m1_carrier(cycles[c],t,&v));
        printf("{\"kind\":\"carrier\",\"cycle\":\"%" PRIu64 "\",\"tick\":%u,\"coordinates\":",cycles[c],t);ids(v.coordinates,6);
        fputs(",\"spinor\":",stdout);fs(v.spinor,4);fputs(",\"quadrature\":",stdout);ds(v.quadrature,2);fputs(",\"opposite_quadrature\":",stdout);ds(v.opposite_quadrature,2);
        printf(",\"m2_carrier_count\":%u,\"genus\":%u,\"explicate_edges\":%u,\"identification_slots\":%u}\n",v.m2_carrier_count,v.genus,v.explicate_edges,v.identification_slots);
    }
    const uint64_t deltas[]={0,1,11,12,13,24,1000,UINT64_MAX};
    for(size_t c=0;c<4;++c)for(uint32_t t=0;t<12;++t)for(size_t d=0;d<8;++d){
        QL_M1_Clock v,prev;memset(&v,0xa5,sizeof v);memcpy(&prev,&v,sizeof v);
        int ok=ql_m1_clock_advance(cycles[c],t,deltas[d],&v);
        printf("{\"kind\":\"advance\",\"cycle\":\"%" PRIu64 "\",\"tick\":%u,\"delta\":\"%" PRIu64 "\",\"ok\":%s",cycles[c],t,deltas[d],ok?"true":"false");
        if(ok)printf(",\"next_cycle\":\"%" PRIu64 "\",\"next_tick\":%u",v.cycle,v.tick12);
        else { CHECK(!memcmp(&v,&prev,sizeof v)); }
        puts("}");
    }
    CHECK(!ql_m1_clock_advance(0,12,0,&(QL_M1_Clock){0}));CHECK(!ql_m1_clock_advance(0,0,0,NULL));
    CHECK(!ql_m1_carrier(0,12,&(QL_M1_Carrier){0})&&!ql_m1_carrier(0,0,NULL));
    for(uint32_t a=0;a<64;++a)for(uint32_t b=0;b<64;++b){
        QL_M1_FiniteField v;CHECK(ql_m1_finite_field(a%6u,b%6u,a,b,&v));
        fputs("{\"kind\":\"finite\",\"coordinate\":",stdout);id(v.coordinate);
        printf(",\"left_position\":%u,\"right_position\":%u,\"direct_word\":%u,\"prime_word\":%u,\"relation_index\":%u,\"pair_index\":%u,\"direct_face_index\":%u,\"prime_face_index\":%u,\"bitwise_complement\":%u,\"binary_states\":%u,\"two_face_states\":%u,\"paired_states\":%u}\n",v.left_position,v.right_position,v.direct_word,v.prime_word,v.relation_index,v.pair_index,v.direct_face_index,v.prime_face_index,v.bitwise_complement,v.binary_states,v.two_face_states,v.paired_states);
    }
    CHECK(!ql_m1_finite_field(6,0,0,0,&(QL_M1_FiniteField){0}));
    CHECK(!ql_m1_finite_field(0,6,0,0,&(QL_M1_FiniteField){0}));
    CHECK(!ql_m1_finite_field(0,0,256,0,&(QL_M1_FiniteField){0}));
    CHECK(!ql_m1_finite_field(0,0,0,64,&(QL_M1_FiniteField){0}));
    CHECK(!ql_m1_finite_field(0,0,0,0,NULL));
    QL_M1_SourceTraits traits;CHECK(ql_m1_source_traits(&traits)&&!ql_m1_source_traits(NULL));
    for(size_t i=0;i<6;++i){CHECK(traits.branch_categories[i]==(uint32_t)M1_BRANCH_QL_CATEGORY[i]);CHECK(traits.mahamaya_ring[i]==DR_RING_MAHAMAYA[i]);CHECK(traits.parashakti_ring[i]==DR_RING_PARASHAKTI[i]);}
    CHECK(traits.unary_mask==M1_OP_UNARY&&traits.binary_mask==M1_OP_BINARY&&traits.relational_mask==M1_OP_RELATIONAL);
    fputs("{\"kind\":\"traits\",\"coordinates\":",stdout);ids(traits.coordinates,6);fputs(",\"branch_categories\":",stdout);us(traits.branch_categories,6);
    fputs(",\"mahamaya_ring\":",stdout);us(traits.mahamaya_ring,6);fputs(",\"parashakti_ring\":",stdout);us(traits.parashakti_ring,6);
    printf(",\"unary_mask\":%u,\"binary_mask\":%u,\"relational_mask\":%u}\n",traits.unary_mask,traits.binary_mask,traits.relational_mask);
    CHECK(!ferror(stdout));fprintf(stderr,"M1 state: 864 source cells; %zu coordinates; %zu incident relations; 2401 rotors; 48 carriers; 384 clock advances; 625 torus positions; 4096 independent state pairs\n",nodes,edges);return 0;
}
