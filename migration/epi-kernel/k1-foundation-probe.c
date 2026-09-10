/* Executable C half of the K1 C/Rust parity fixture. No vendored implementation
 * is linked here: Rust compares this stream with its current public APIs. */
#include "ql/kernel.h"
#include <stdio.h>
#include <stdlib.h>

#define REQUIRE(c) do { if (!(c)) { fprintf(stderr, "probe failure at line %d\n", __LINE__); exit(1); } } while (0)

static const QL_Coordinate_Family families[] = {
    QL_FAMILY_C, QL_FAMILY_P, QL_FAMILY_L, QL_FAMILY_S,
    QL_FAMILY_T, QL_FAMILY_M, QL_FAMILY_NONE
};

static void format(QL_Kernel_Address address, char out[32]) {
    REQUIRE(ql_kernel_address_format(address, out, 32) == 0);
}

int main(void) {
    QL_Holographic_Field field;
    REQUIRE(ql_holographic_field_init(&field) == 0);
    printf("contract\t%s\n", ql_kernel_contract_version());
    for (size_t i = 0; i < sizeof(families) / sizeof(families[0]); ++i)
        printf("family\t%s\t%u\n", ql_kernel_family_code(families[i]), (unsigned)families[i]);
    for (unsigned face = 0; face < QL_FACE_COUNT; ++face)
        printf("face\t%s\t%u\n", ql_kernel_face_code((QL_Coordinate_Face)face), face);
    for (unsigned r = 0; r < QL_KERNEL_RELATION_COUNT; ++r)
        printf("relation\t%s\n", ql_kernel_relation_id((QL_Kernel_Relation_Id)r));

    QL_Kernel_Address hash = ql_kernel_hash_address();
    char label[32];
    format(hash, label);
    REQUIRE(ql_kernel_address_is_bedrock(hash));
    REQUIRE(ql_holographic_field_resolve(&field, hash) == ql_default_hash_bimba());
    printf("hash\t%s\t%u\t%u\t%u\n", label, hash.family, hash.position, hash.face);
    for (unsigned r = 0; r < QL_KERNEL_RELATION_COUNT; ++r) {
        QL_Kernel_Relation_Ref ref;
        int ok = ql_kernel_relation_resolve((QL_Kernel_Relation_Id)r, hash, QL_FAMILY_NONE, &ref) == 0;
        if (ok) format(ref.target, label);
        printf("hash-edge\t%s\t%s\n", ql_kernel_relation_id((QL_Kernel_Relation_Id)r), ok ? label : "-");
    }

    for (size_t f = 0; f < sizeof(families) / sizeof(families[0]); ++f) {
        for (uint8_t p = 0; p < QL_POSITION_COUNT; ++p) {
            for (unsigned face = 0; face < QL_FACE_COUNT; ++face) {
                QL_Kernel_Address a = ql_kernel_family_address(families[f], p, (QL_Coordinate_Face)face);
                format(a, label);
                REQUIRE(ql_holographic_field_resolve(&field, a) != NULL);
                REQUIRE(ql_holographic_field_resolve(&field, a) ==
                        ql_holographic_field_resolve(&field, ql_coordinate_label_other_face(a)));
                printf("address\t%s\t%u\t%s\t%s\t%u\n", ql_kernel_family_code(families[f]), p,
                       ql_kernel_face_code((QL_Coordinate_Face)face), label,
                       ql_kernel_pitch_class(p, (QL_Coordinate_Face)face));
                for (unsigned r = 0; r < QL_KERNEL_RELATION_COUNT; ++r) {
                    size_t count = r == QL_KERNEL_REL_FAMILY_SAME_POSITION ? 7u : 1u;
                    for (size_t t = 0; t < count; ++t) {
                        QL_Coordinate_Family target = count == 1u ? QL_FAMILY_NONE : families[t];
                        QL_Kernel_Relation_Ref ref;
                        int ok = ql_kernel_relation_resolve((QL_Kernel_Relation_Id)r, a, target, &ref) == 0;
                        if (ok) format(ref.target, label);
                        printf("edge\t%s\t%s\t%u\t%s\t%s\t%s\n",
                               ql_kernel_relation_id((QL_Kernel_Relation_Id)r), ql_kernel_family_code(families[f]),
                               p, ql_kernel_face_code((QL_Coordinate_Face)face), ql_kernel_family_code(target),
                               ok ? label : "-");
                    }
                }
            }
        }
    }

    for (uint8_t lens = 0; lens < QL_POSITION_COUNT; ++lens) {
        for (unsigned face = 0; face < QL_FACE_COUNT; ++face) {
            char lens_label[32];
            format(ql_kernel_family_address(QL_FAMILY_L, lens, (QL_Coordinate_Face)face), lens_label);
            for (uint8_t local = 0; local < QL_POSITION_COUNT; ++local) {
                QL_Kernel_MEF_Address a;
                REQUIRE(ql_kernel_mef_address(lens, (QL_Coordinate_Face)face, local, &a) == 0);
                REQUIRE(ql_kernel_mef_address_format(&a, label, sizeof(label)) == 0);
                printf("mef\t%s\t%u\t%u\t%s\t%u\n", lens_label, local, a.absolute_position, label, a.pitch_class);
            }
            for (unsigned cf = 0; cf < QL_KERNEL_CONTEXT_FRAME_COUNT; ++cf) {
                QL_Kernel_Context_Frame_Address a;
                REQUIRE(ql_kernel_context_frame_address((QL_Kernel_Context_Frame_Id)cf, lens, (QL_Coordinate_Face)face, &a) == 0);
                printf("cf\tCF%u\t%s\t%u\t%u\t%s\t%s\t%s\n", cf + 1u, lens_label,
                       a.mef.local_position, a.mef.absolute_position,
                       a.unit_face == QL_KERNEL_MEF_UNIT_NAME ? "name" : "power",
                       a.grain == QL_KERNEL_MEF_GRAIN_INNER_FOUR ? "inner-four" : "outer-two", a.notation);
            }
        }
    }
    for (unsigned f = 0; f < QL_KERNEL_VAK_FAMILY_COUNT; ++f) {
        const QL_Kernel_VAK_Descriptor* d = ql_kernel_vak_descriptor((QL_Kernel_VAK_Family)f);
        REQUIRE(d != NULL);
        printf("vak\t%s\t%u\t%s\t%s\t%s\n", d->code, (unsigned)d->family,
               ql_kernel_relation_id(d->relation), d->meaning, d->m0_handler_role);
        for (uint8_t branch = 0; branch < QL_POSITION_COUNT; ++branch) {
            for (uint8_t p = 0; p < QL_POSITION_COUNT; ++p) {
                for (unsigned face = 0; face < QL_FACE_COUNT; ++face) {
                    QL_Kernel_VAK_Instruction instruction;
                    REQUIRE(ql_kernel_vak_instruction_init((QL_Kernel_VAK_Family)f, 255u, branch, p,
                            (QL_Coordinate_Face)face, &instruction) == 0);
                    REQUIRE(ql_kernel_vak_instruction_valid(&instruction));
                    printf("instruction\t%u\t%u\t%u\t%u\t%s\n", instruction.vak_family,
                           instruction.vak_index, instruction.target_branch, instruction.target_pos,
                           ql_kernel_face_code(ql_kernel_vak_instruction_face(&instruction)));
                }
            }
        }
    }
    return 0;
}
