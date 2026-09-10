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

static void check_address_field(QL_Holographic_Field* field) {
    const QL_Coordinate_Family families[QL_KERNEL_ADDRESS_FAMILY_COUNT] = {
        QL_FAMILY_C, QL_FAMILY_P, QL_FAMILY_L, QL_FAMILY_S,
        QL_FAMILY_T, QL_FAMILY_M, QL_FAMILY_NONE
    };

    QL_Kernel_Address hash = ql_kernel_hash_address();
    CHECK(ql_kernel_address_valid(hash), "address.hash-valid");
    CHECK(ql_kernel_address_is_hash(hash), "address.hash-identity");
    CHECK(ql_kernel_address_is_bedrock(hash), "address.hash-bedrock");
    CHECK(ql_holographic_field_resolve(field, hash) == ql_default_hash_bimba(), "address.hash-resolve");
    char formatted[32];
    CHECK(ql_kernel_address_format(hash, formatted, sizeof(formatted)) == 0 && strcmp(formatted, "#") == 0,
          "address.hash-format");

    for (uint8_t f = 0u; f < QL_KERNEL_ADDRESS_FAMILY_COUNT; f++) {
        for (uint8_t p = 0u; p < QL_POSITION_COUNT; p++) {
            for (uint8_t face = 0u; face < QL_FACE_COUNT; face++) {
                QL_Kernel_Address address = ql_kernel_family_address(
                    families[f], p, (QL_Coordinate_Face)face);
                CHECK(ql_kernel_address_valid(address), "address.full-field-valid");
                const QL_Holographic_Coordinate* substrate =
                    ql_holographic_field_resolve(field, address);
                CHECK(substrate != NULL, "address.full-field-resolve");
                if (families[f] == QL_FAMILY_NONE) {
                    CHECK(substrate == ql_default_psychoid_bimba(p), "address.raw-resolve");
                    CHECK(ql_kernel_address_is_bedrock(address), "address.raw-bedrock");
                } else {
                    CHECK(substrate == ql_holographic_field_get_const(field, families[f], p),
                          "address.family-resolve");
                }

                QL_Kernel_Address conjugate = ql_coordinate_label_other_face(address);
                CHECK(conjugate.family == address.family && conjugate.position == address.position &&
                      conjugate.face != address.face, "address.face-conjugacy");
                CHECK(ql_holographic_field_resolve(field, conjugate) == substrate,
                      "address.face-shared-substrate");
            }
        }
    }

    QL_Kernel_Address p2p = ql_kernel_family_address(QL_FAMILY_P, 2u, QL_COORD_FACE_PRIME);
    CHECK(ql_kernel_address_format(p2p, formatted, sizeof(formatted)) == 0 && strcmp(formatted, "P2'") == 0,
          "address.family-format");
    QL_Kernel_Address raw3p = ql_kernel_position_address(3u, QL_COORD_FACE_PRIME);
    CHECK(ql_kernel_address_format(raw3p, formatted, sizeof(formatted)) == 0 && strcmp(formatted, "#3'") == 0,
          "address.raw-format");
}

static void check_relation_field(void) {
    const QL_Coordinate_Family families[QL_KERNEL_ADDRESS_FAMILY_COUNT] = {
        QL_FAMILY_C, QL_FAMILY_P, QL_FAMILY_L, QL_FAMILY_S,
        QL_FAMILY_T, QL_FAMILY_M, QL_FAMILY_NONE
    };
    const uint8_t pair_a[6] = {1u, 0u, 3u, 2u, 5u, 4u};
    const uint8_t pair_b[6] = {5u, 2u, 1u, 4u, 3u, 0u};
    const uint8_t pair_c[6] = {5u, 4u, 3u, 2u, 1u, 0u};

    CHECK(strcmp(ql_kernel_contract_version(), "1.1.0") == 0, "contract.version");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_CROSS_SAME_POSITION),
                 "ql.kernel.cross.same-position/v1") == 0, "relation.id.conjugate");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_CROSS_COMPLETE),
                 "ql.kernel.cross.complete/v1") == 0, "relation.id.complete");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_MIRROR_COMPLEMENT),
                 "ql.kernel.mirror.complement/v1") == 0, "relation.id.mirror");

    QL_Kernel_Contract_Provenance provenance = ql_kernel_contract_provenance();
    CHECK(strcmp(provenance.historical_reference_revision,
                 "daa660cbc1b8c5da83828698665a753852cb0287") == 0,
          "contract.provenance.reference");
    CHECK(strcmp(provenance.historical_pointer_web_blob,
                 "3eeae6f9c8cc65c5a610df1a49143b3c65bdd320") == 0,
          "contract.provenance.pointer-web");

    for (uint8_t f = 0u; f < QL_KERNEL_ADDRESS_FAMILY_COUNT; f++) {
        for (uint8_t p = 0u; p < QL_POSITION_COUNT; p++) {
            for (uint8_t face = 0u; face < QL_FACE_COUNT; face++) {
                QL_Kernel_Address source = ql_kernel_family_address(
                    families[f], p, (QL_Coordinate_Face)face);
                QL_Kernel_Relation_Ref rel;

                CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_CROSS_SAME_POSITION, source, QL_FAMILY_NONE, &rel) == 0,
                    "relation.conjugate.resolve");
                CHECK(rel.target.family == source.family && rel.target.position == p &&
                      rel.target.face != source.face, "relation.conjugate.same-index");
                CHECK(rel.interval_role == QL_KERNEL_INTERVAL_SEMITONE,
                      "relation.conjugate.semitone");

                CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_MIRROR_COMPLEMENT, source, QL_FAMILY_NONE, &rel) == 0,
                    "relation.mirror.resolve");
                CHECK(rel.target.position == (uint8_t)(5u - p) && rel.target.face == source.face,
                      "relation.mirror.same-face");

                CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_CROSS_TRANSFORM, source, QL_FAMILY_NONE, &rel) == 0,
                    "relation.transform.resolve");
                CHECK(rel.target.position == (uint8_t)((p + 1u) % 6u) && rel.target.face != source.face,
                      "relation.transform.target");

                CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_CROSS_REQUIRE, source, QL_FAMILY_NONE, &rel) == 0,
                    "relation.require.resolve");
                CHECK(rel.target.position == (uint8_t)((p + 5u) % 6u) && rel.target.face != source.face,
                      "relation.require.target");

                CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_CROSS_COMPLETE, source, QL_FAMILY_NONE, &rel) == 0,
                    "relation.complete.resolve");
                CHECK(rel.target.position == (uint8_t)(5u - p) && rel.target.face != source.face,
                      "relation.complete.cross-face");

                const QL_Kernel_Relation_Id pair_relations[3] = {
                    QL_KERNEL_REL_PAIR_A, QL_KERNEL_REL_PAIR_B, QL_KERNEL_REL_PAIR_C
                };
                const uint8_t* pair_targets[3] = {pair_a, pair_b, pair_c};
                for (uint8_t pair = 0u; pair < 3u; pair++) {
                    CHECK(ql_kernel_relation_resolve(
                        pair_relations[pair], source, QL_FAMILY_NONE, &rel) == 0,
                        "relation.pair.resolve");
                    CHECK(rel.target.position == pair_targets[pair][p] && rel.target.face == source.face,
                          "relation.pair.target");
                }

                for (uint8_t target_family = 0u; target_family < QL_KERNEL_ADDRESS_FAMILY_COUNT; target_family++) {
                    CHECK(ql_kernel_relation_resolve(
                        QL_KERNEL_REL_FAMILY_SAME_POSITION,
                        source,
                        families[target_family],
                        &rel) == 0,
                        "relation.family.resolve");
                    CHECK(rel.target.family == (uint8_t)families[target_family] &&
                          rel.target.position == p && rel.target.face == source.face,
                          "relation.family.same-position");
                }

                CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_LENS_ANCHOR, source, QL_FAMILY_NONE, &rel) == 0,
                    "relation.lens-anchor.resolve");
                CHECK(rel.target.family == QL_FAMILY_L && rel.target.position == p &&
                      rel.target.face == source.face, "relation.lens-anchor.target");

                if (p < 5u) {
                    CHECK(ql_kernel_relation_resolve(
                        QL_KERNEL_REL_POSITION_SUCCESSOR, source, QL_FAMILY_NONE, &rel) == 0,
                        "relation.successor.resolve");
                    CHECK(rel.target.position == (uint8_t)(p + 1u) && rel.target.face == source.face,
                          "relation.successor.target");
                    CHECK(rel.ratio_role == QL_KERNEL_RATIO_EPOGDOON,
                          "relation.successor.epogdoon");
                } else {
                    CHECK(ql_kernel_relation_resolve(
                        QL_KERNEL_REL_POSITION_SUCCESSOR, source, QL_FAMILY_NONE, &rel) != 0,
                        "relation.successor.stops-at-return");
                    CHECK(ql_kernel_relation_resolve(
                        QL_KERNEL_REL_MOBIUS_RETURN, source, QL_FAMILY_NONE, &rel) == 0,
                        "relation.return.resolve");
                    CHECK(rel.target.position == 0u && rel.target.face != source.face,
                          "relation.return.target");
                    CHECK(rel.interval_role == QL_KERNEL_INTERVAL_OCTAVE &&
                          rel.ratio_role == QL_KERNEL_RATIO_OCTAVE,
                          "relation.return.octave");
                }
            }

            QL_Kernel_Address prime = ql_kernel_family_address(families[f], p, QL_COORD_FACE_PRIME);
            QL_Kernel_Relation_Ref invariant;
            CHECK(ql_kernel_relation_resolve(
                QL_KERNEL_REL_CONJUGATE_INVARIANCE_A, prime, QL_FAMILY_NONE, &invariant) == 0,
                "relation.invariance.A");
            CHECK(invariant.target.position == pair_a[p] && invariant.target.face == QL_COORD_FACE_PRIME,
                  "relation.invariance.A-target");
            CHECK(ql_kernel_relation_resolve(
                QL_KERNEL_REL_CONJUGATE_INVARIANCE_B, prime, QL_FAMILY_NONE, &invariant) == 0,
                "relation.invariance.B");
            CHECK(invariant.target.position == pair_b[p], "relation.invariance.B-target");
            CHECK(ql_kernel_relation_resolve(
                QL_KERNEL_REL_CONJUGATE_INVARIANCE_C, prime, QL_FAMILY_NONE, &invariant) == 0,
                "relation.invariance.C");
            CHECK(invariant.target.position == pair_c[p], "relation.invariance.C-target");
        }
    }

    /* Pair-C and mirror reach the same same-face vertex set, while complete
     * reaches that positional complement across the other face. Operator
     * identity/provenance remains distinct. */
    QL_Kernel_Address p2 = ql_kernel_family_address(QL_FAMILY_P, 2u, QL_COORD_FACE_DIRECT);
    QL_Kernel_Relation_Ref pair_c_ref, mirror_ref, complete_ref;
    CHECK(ql_kernel_relation_resolve(QL_KERNEL_REL_PAIR_C, p2, QL_FAMILY_NONE, &pair_c_ref) == 0,
          "relation.distinction.pair-c");
    CHECK(ql_kernel_relation_resolve(QL_KERNEL_REL_MIRROR_COMPLEMENT, p2, QL_FAMILY_NONE, &mirror_ref) == 0,
          "relation.distinction.mirror");
    CHECK(ql_kernel_relation_resolve(QL_KERNEL_REL_CROSS_COMPLETE, p2, QL_FAMILY_NONE, &complete_ref) == 0,
          "relation.distinction.complete");
    CHECK(pair_c_ref.target.position == mirror_ref.target.position &&
          pair_c_ref.target.face == mirror_ref.target.face,
          "relation.distinction.same-vertices");
    CHECK(pair_c_ref.relation != mirror_ref.relation && mirror_ref.relation != complete_ref.relation,
          "relation.distinction.operator-identity");
    CHECK(complete_ref.target.position == mirror_ref.target.position &&
          complete_ref.target.face != mirror_ref.target.face,
          "relation.distinction.complete-cross-face");
}

static void check_mef_and_context_frames(void) {
    static const uint8_t cf_local[7] = {0u, 1u, 2u, 2u, 3u, 4u, 5u};
    static const QL_Kernel_MEF_Unit_Face cf_unit[7] = {
        QL_KERNEL_MEF_UNIT_NAME, QL_KERNEL_MEF_UNIT_NAME, QL_KERNEL_MEF_UNIT_NAME,
        QL_KERNEL_MEF_UNIT_POWER, QL_KERNEL_MEF_UNIT_POWER, QL_KERNEL_MEF_UNIT_POWER,
        QL_KERNEL_MEF_UNIT_POWER
    };
    static const QL_Kernel_MEF_Grain cf_grain[7] = {
        QL_KERNEL_MEF_GRAIN_OUTER_TWO, QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR, QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_INNER_FOUR, QL_KERNEL_MEF_GRAIN_INNER_FOUR,
        QL_KERNEL_MEF_GRAIN_OUTER_TWO
    };
    static const char* const cf_notation[7] = {
        "(00/00)", "(0/1)", "(0/1/2)", "(0/1/2/3)",
        "(4.0/1-4.4/5)", "(4.5/0)", "(5/0)"
    };

    for (uint8_t lens = 0u; lens < QL_POSITION_COUNT; lens++) {
        for (uint8_t face = 0u; face < QL_FACE_COUNT; face++) {
            for (uint8_t local = 0u; local < QL_POSITION_COUNT; local++) {
                QL_Kernel_MEF_Address address;
                CHECK(ql_kernel_mef_address(lens, (QL_Coordinate_Face)face, local, &address) == 0,
                      "mef.address.resolve");
                CHECK(address.lens.family == QL_FAMILY_L && address.lens.position == lens &&
                      address.lens.face == face, "mef.address.lens");
                CHECK(address.local_position == local &&
                      address.absolute_position == (uint8_t)((lens + local) % 6u),
                      "mef.address.rotation");
                CHECK(address.resonance_index == kernel_resonance_index(lens, face, local),
                      "mef.address.resonance-parity");
                CHECK(address.tritone_square == kernel_tritone_square_for_lens(lens),
                      "mef.address.square-parity");
                char ref[64];
                CHECK(ql_kernel_mef_address_format(&address, ref, sizeof(ref)) == 0,
                      "mef.address.ref-format");
            }

            for (uint8_t cf = 0u; cf < QL_KERNEL_CONTEXT_FRAME_COUNT; cf++) {
                QL_Kernel_Context_Frame_Address address;
                CHECK(ql_kernel_context_frame_address(
                    (QL_Kernel_Context_Frame_Id)cf,
                    lens,
                    (QL_Coordinate_Face)face,
                    &address) == 0,
                    "cf.address.resolve");
                CHECK(address.frame == (QL_Kernel_Context_Frame_Id)cf,
                      "cf.address.id");
                CHECK(address.mef.local_position == cf_local[cf], "cf.address.local-position");
                CHECK(address.unit_face == cf_unit[cf], "cf.address.unit-face");
                CHECK(address.grain == cf_grain[cf], "cf.address.grain");
                CHECK(strcmp(address.notation, cf_notation[cf]) == 0, "cf.address.notation");
            }
        }
    }
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

    check_address_field(&field);
    check_relation_field();
    check_mef_and_context_frames();

    QL_Coordinate_Label p2_label = ql_coordinate_label(QL_FAMILY_P, 2u, QL_COORD_FACE_DIRECT);
    QL_Coordinate_Label p2_prime = ql_coordinate_label_other_face(p2_label);
    CHECK(ql_coordinate_label_valid(p2_label), "label.P-valid");
    CHECK(p2_prime.family == QL_FAMILY_P && p2_prime.position == 2u &&
          p2_prime.face == QL_COORD_FACE_PRIME, "label.P-prime-same-position");

    CHECK(ql_coordinate_set_face(p2, QL_COORD_FACE_PRIME) == 0, "face.P-set");
    CHECK(p2->ql_position == 2u && ql_coordinate_face(p2) == QL_COORD_FACE_PRIME,
          "face.P-preserves-position");
    CHECK(ql_coordinate_topology(p2) == QL_TOPO_KLEIN, "face.P-prime-klein");
    CHECK(ql_coordinate_set_face(p2, QL_COORD_FACE_DIRECT) == 0, "face.P-reset");
    CHECK(p2->ql_position == 2u && ql_coordinate_face(p2) == QL_COORD_FACE_DIRECT,
          "face.P-reset-same-position");

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
                QL_Kernel_Resonance_Map mapped;
                CHECK(ql_kernel_resonance_map(lens, face, pos, &mapped) == 0, "kernel.resonance-map");
                CHECK(mapped.lens.family == QL_FAMILY_L && mapped.lens.position == lens,
                      "kernel.resonance-map-lens");
                CHECK(mapped.lens.face == face, "kernel.resonance-map-face");
                CHECK(mapped.inner_position == pos, "kernel.resonance-map-inner-position");
                CHECK(mapped.resonance_index == kernel_resonance_index(lens, face, pos),
                      "kernel.resonance-map-index");
                CHECK(mapped.tritone_square == kernel_tritone_square_for_lens(lens),
                      "kernel.resonance-map-square");
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
        CHECK(n.position6 == r.position6, "kernel.tick.position6");
        CHECK(exactf(n.harmonic_ratio, r.harmonic_ratio), "kernel.tick.ratio");

        QL_Coordinate_Label tick_label = ql_kernel_tick_position_label(&n);
        CHECK(tick_label.family == QL_FAMILY_P, "kernel.tick-map-family-P");
        CHECK(tick_label.position == r.position6, "kernel.tick-map-same-position");
        CHECK(tick_label.face == (n.sub_tick < QL_POSITION_COUNT
              ? QL_COORD_FACE_DIRECT : QL_COORD_FACE_PRIME),
              "kernel.tick-map-face");
    }

    if (failures) {
        fprintf(stderr, "R4 parity failures: %u\n", failures);
        return 1;
    }
    printf("R4 native holographic kernel address/relation/MEF/CF + mature parity: PASS\n");
    printf("native-kernel-api: %s contract: %s\n", ql_kernel_api_version(), ql_kernel_contract_version());
    return 0;
}
