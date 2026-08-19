#include "kernel.h"
#include "ql/holographic.h"
#include "ql/kernel.h"

#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static unsigned failures = 0u;
#define CHECK(cond, op) do { if (!(cond)) { fprintf(stderr, "FAIL\t%s\n", (op)); failures++; } } while (0)

static int exactf(float a, float b) { return a == b; }
static int approx(float a, float b) { return fabsf(a - b) < 1e-6f; }

static QL_Quaternion qlq(Quaternion q) {
    return (QL_Quaternion){ .w = q.w, .x = q.x, .y = q.y, .z = q.z };
}

static void hook(QL_Holographic_Coordinate* self, void* ctx) {
    unsigned* n = (unsigned*)ctx;
    if (self && n) (*n)++;
}

int main(void) {
    CHECK((int)QL_FAMILY_C == (int)FAMILY_C, "family.C");
    CHECK((int)QL_FAMILY_P == (int)FAMILY_P, "family.P");
    CHECK((int)QL_FAMILY_L == (int)FAMILY_L, "family.L");
    CHECK((int)QL_FAMILY_S == (int)FAMILY_S, "family.S");
    CHECK((int)QL_FAMILY_T == (int)FAMILY_T, "family.T");
    CHECK((int)QL_FAMILY_M == (int)FAMILY_M, "family.M");
    CHECK((int)QL_FAMILY_NONE == (int)FAMILY_NONE, "family.NONE");

    CHECK(sizeof(QL_Holographic_Coordinate) == sizeof(Holographic_Coordinate), "layout.size");
    CHECK(offsetof(QL_Holographic_Coordinate, semantic_embedding) == offsetof(Holographic_Coordinate, semantic_embedding), "layout.semantic");
    CHECK(offsetof(QL_Holographic_Coordinate, c) == offsetof(Holographic_Coordinate, c), "layout.links");
    CHECK(offsetof(QL_Holographic_Coordinate, invoke_process) == offsetof(Holographic_Coordinate, invoke_process), "layout.execute");
    CHECK(offsetof(QL_Holographic_Coordinate, payload) == offsetof(Holographic_Coordinate, payload), "layout.payload");

    for (uint8_t p = 0; p < QL_POSITION_COUNT; p++) {
        const QL_Bimba* raw = ql_default_psychoid_bimba(p);
        CHECK(raw != NULL, "raw.exists");
        CHECK(raw && raw->family == QL_FAMILY_NONE, "raw.family-none");
        CHECK(raw && ql_coordinate_is_bimba(raw), "raw.bimba");
    }
    CHECK(ql_default_hash_bimba()->ql_position == QL_HASH_POSITION, "raw.hash");

    QL_Holographic_Field field;
    CHECK(ql_holographic_field_init(&field) == 0, "field.init");
    for (uint8_t f = 0; f < QL_COORDINATE_FAMILY_COUNT; f++) {
        for (uint8_t p = 0; p < QL_POSITION_COUNT; p++) {
            QL_Holographic_Coordinate* c = ql_holographic_field_get(&field, (QL_Coordinate_Family)f, p);
            CHECK(c != NULL, "field.coordinate");
            CHECK(c && c->family == f && c->ql_position == p, "field.identity");
            CHECK(c && ql_coordinate_bedrock(c) == ql_default_psychoid_bimba(p), "field.bedrock");
            CHECK(c && ql_relation_target(c->m) == ql_holographic_field_get(&field, QL_FAMILY_M, p), "field.m-link");
            CHECK(c && ql_relation_family(c->m) == QL_FAMILY_M, "field.m-link-family-tag");
            CHECK(c && ql_relation_position(c->m) == p, "field.m-link-position-tag");
            CHECK(c && ql_relation_target(c->cs) == ql_holographic_field_get(&field, (QL_Coordinate_Family)f, 5u), "field.cs");
        }
    }

    QL_Holographic_Coordinate* p4 = ql_holographic_field_get(&field, QL_FAMILY_P, 4u);
    QL_Holographic_Coordinate* p2 = ql_holographic_field_get(&field, QL_FAMILY_P, 2u);
    CHECK((ql_relation_flags(p4->c) & QL_TAG_NESTING) != 0u, "relation.nesting");
    CHECK((ql_relation_flags(p2->c) & QL_TAG_BRANCHING) != 0u, "relation.branching");
    CHECK(ql_relation_target(p4->cf) == p4, "reflective.cf-self");

    QL_Holographic_Coordinate conjugate;
    CHECK(ql_coordinate_conjugate(p2, &conjugate) == 0, "conjugate.P");
    CHECK(conjugate.family == QL_FAMILY_P && conjugate.ql_position == 3u && conjugate.inversion_state == 1u, "conjugate.P-identity");
    CHECK(ql_coordinate_topology(&conjugate) == QL_TOPO_KLEIN, "conjugate.P-topology");

    QL_Holographic_Coordinate* l1 = ql_holographic_field_get(&field, QL_FAMILY_L, 1u);
    CHECK(ql_coordinate_conjugate(l1, &conjugate) == 0, "conjugate.L");
    CHECK(conjugate.family == QL_FAMILY_L && conjugate.ql_position == 4u && conjugate.inversion_state == 1u, "conjugate.L-identity");

    QL_Holographic_Coordinate* m1 = ql_holographic_field_get(&field, QL_FAMILY_M, 1u);
    CHECK(ql_coordinate_conjugate(m1, &conjugate) == 0, "conjugate.M");
    CHECK(conjugate.family == QL_FAMILY_M && conjugate.ql_position == 4u, "conjugate.M-family-preserved");

    QL_Pratibimba manifested;
    CHECK(ql_coordinate_materialize(ql_default_psychoid_bimba(1u), &manifested) == 0, "pratibimba.materialize");
    CHECK(!ql_coordinate_is_bimba(&manifested), "pratibimba.mutable-face");
    CHECK(ql_coordinate_source(&manifested) == ql_default_psychoid_bimba(1u), "pratibimba.source-recoverable");

    QL_Holographic_Coordinate executable;
    CHECK(ql_coordinate_init(&executable, QL_FAMILY_M, 1u) == 0, "execute.init");
    executable.invoke_process = hook;
    unsigned hook_count = 0u;
    ql_coordinate_execute(&executable, &hook_count);
    CHECK(hook_count == 1u, "execute.hook");

    CHECK(exactf(ql_epogdoon_ratio(), kernel_epogdoon_ratio()), "kernel.ratio.epogdoon");
    CHECK(approx(ql_kernel_epogdoon_log(), kernel_epogdoon_log()), "kernel.ratio.epogdoon-log");
    for (uint8_t lens = 0; lens < QL_POSITION_COUNT; lens++) {
        CHECK(ql_kernel_tritone_square_for_lens(lens) == kernel_tritone_square_for_lens(lens), "kernel.tritone");
        for (uint8_t face = 0; face < QL_FACE_COUNT; face++) {
            for (uint8_t pos = 0; pos < QL_POSITION_COUNT; pos++) {
                CHECK(ql_kernel_resonance_index(lens, face, pos) == kernel_resonance_index(lens, face, pos), "kernel.resonance-index");
            }
        }
    }

    Quaternion rb = { .w = 0.0f, .x = 3.0f, .y = 0.0f, .z = 0.0f };
    Quaternion rp = { .w = 1.0f, .x = 2.0f, .y = 3.0f, .z = 4.0f };
    Kernel_Bioquaternion rstate = kernel_bioquaternion_init(rb, rp);
    QL_Kernel_Bioquaternion nstate = ql_kernel_bioquaternion_init(qlq(rb), qlq(rp));
    CHECK(approx(nstate.q_b.w, rstate.q_b.w) && approx(nstate.q_b.x, rstate.q_b.x), "kernel.bioquaternion.bimba");
    CHECK(approx(nstate.q_p.w, rstate.q_p.w) && approx(nstate.q_p.z, rstate.q_p.z), "kernel.bioquaternion.pratibimba");
    Quaternion rflip = kernel_slash_flip_bimba_prime(rstate);
    QL_Quaternion nflip = ql_kernel_slash_flip_bimba_prime(nstate);
    CHECK(approx(nflip.w, rflip.w) && approx(nflip.x, rflip.x) && approx(nflip.y, rflip.y) && approx(nflip.z, rflip.z), "kernel.slash-flip");

    Kernel_Resonance_Vector ro = {0}, rt = {0};
    QL_Kernel_Resonance_Vector no = {0}, nt = {0};
    for (uint8_t i = 0; i < QL_RESONANCE_COUNT; i++) {
        float a = (float)(i % 7u) / 7.0f;
        float b = (float)(i % 5u) / 5.0f;
        ro.values[i] = no.values[i] = a;
        rt.values[i] = nt.values[i] = b;
    }
    Kernel_Energy re = kernel_energy_evaluate(rstate, &ro, &rt, 0.25f);
    QL_Kernel_Energy ne = ql_kernel_energy_evaluate(nstate, &no, &nt, 0.25f);
    CHECK(approx(ne.bimba_pratibimba_energy, re.bimba_pratibimba_energy), "kernel.energy.bimba-pratibimba");
    CHECK(approx(ne.lens_energy, re.lens_energy), "kernel.energy.lens");
    CHECK(approx(ne.total_energy, re.total_energy), "kernel.energy.total");

    float rs[3], ns[3];
    kernel_resonance_square_emphasis(&ro, rs);
    ql_kernel_resonance_square_emphasis(&no, ns);
    for (uint8_t i = 0; i < 3u; i++) CHECK(approx(rs[i], ns[i]), "kernel.square-emphasis");

    for (unsigned raw = 0; raw <= 255u; raw++) {
        Kernel_Tick r = kernel_tick_from_epogdoon(17u, (uint8_t)raw);
        QL_Kernel_Tick n = ql_kernel_tick_from_epogdoon(17u, (uint8_t)raw);
        CHECK(n.cycle == r.cycle && n.sub_tick == r.sub_tick, "kernel.tick.identity");
        CHECK((int)n.phase == (int)r.phase, "kernel.tick.phase");
        CHECK((int)n.element == (int)r.element, "kernel.tick.element");
        CHECK(n.position6 == r.position6 && n.base_position == r.position6, "kernel.tick.position6");
        CHECK(exactf(n.harmonic_ratio, r.harmonic_ratio), "kernel.tick.ratio");
        CHECK(n.traversal_stage == ql_ring_traversal_stage((uint8_t)raw), "kernel.tick.traversal-stage-distinct");
    }

    if (failures) {
        fprintf(stderr, "R4 parity failures: %u\n", failures);
        return 1;
    }
    printf("R4 holographic/kernel parity: PASS\n");
    printf("native-kernel-api: %s\n", ql_kernel_api_version());
    return 0;
}
