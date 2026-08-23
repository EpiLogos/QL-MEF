#include "ql/holographic.h"
#include "ql/kernel.h"
#include "vak.h"

#include <stdio.h>
#include <string.h>

static unsigned failures = 0u;
#define CHECK(cond, op) do { if (!(cond)) { fprintf(stderr, "FAIL\t%s\n", (op)); failures++; } } while (0)

static int same_address(QL_Kernel_Address a, QL_Kernel_Address b) {
    return a.family == b.family && a.position == b.position && a.face == b.face;
}

int main(void) {
    _Static_assert(sizeof(QL_Kernel_VAK_Instruction) == sizeof(VAK_Instruction),
                   "native and historical VAK instructions must retain 5-byte shape");

    const struct {
        QL_Kernel_VAK_Family family;
        uint8_t historical_family;
        const char* code;
        const char* relation;
        const char* meaning;
        const char* m0_role;
    } expected[] = {
        { QL_KERNEL_VAK_CPF, VAK_FAMILY_CPF, "CPF", "ql.kernel.vak.cpf/v1",
          "Category-Position-Frame", "discrimination/inversion" },
        { QL_KERNEL_VAK_CT, VAK_FAMILY_CT, "CT", "ql.kernel.vak.ct/v1",
          "Context-Time / Content Types", "QL-frame-selection" },
        { QL_KERNEL_VAK_CP, VAK_FAMILY_CP, "CP", "ql.kernel.vak.cp/v1",
          "Context-Position", "void-arithmetic-position-anchor" },
        { QL_KERNEL_VAK_CF, VAK_FAMILY_CF, "CF", "ql.kernel.context-frame/v1",
          "Context-Frame", "Context-Frame/Vimarsa-invocation" },
        { QL_KERNEL_VAK_CFP, VAK_FAMILY_CFP, "CFP", "ql.kernel.vak.cfp/v1",
          "Context-Frame-Position / Paths", "R-factor-thread" },
        { QL_KERNEL_VAK_CS, VAK_FAMILY_CS, "CS", "ql.kernel.vak.cs/v1",
          "Context-Sequence", "Logos-cycle-completion" }
    };

    CHECK(QL_KERNEL_VAK_FAMILY_COUNT == VAK_FAMILY_COUNT, "vak.family-count");
    CHECK(QL_KERNEL_REL_VAK_CF == QL_KERNEL_REL_CONTEXT_FRAME, "vak.cf-is-context-frame");

    for (size_t i = 0u; i < sizeof(expected) / sizeof(expected[0]); i++) {
        const QL_Kernel_VAK_Descriptor* descriptor = ql_kernel_vak_descriptor(expected[i].family);
        CHECK(descriptor != NULL, "vak.descriptor.present");
        if (!descriptor) continue;
        CHECK((uint8_t)expected[i].family == expected[i].historical_family,
              "vak.family-index-historical-parity");
        CHECK(strcmp(descriptor->code, expected[i].code) == 0, "vak.code");
        CHECK(strcmp(ql_kernel_relation_id(descriptor->relation), expected[i].relation) == 0,
              "vak.relation");
        CHECK(strcmp(descriptor->meaning, expected[i].meaning) == 0, "vak.meaning");
        CHECK(strcmp(descriptor->m0_handler_role, expected[i].m0_role) == 0, "vak.m0-role");
        CHECK(ql_kernel_vak_relation_id(expected[i].family) == descriptor->relation,
              "vak.relation-map");

        QL_Kernel_VAK_Instruction direct;
        QL_Kernel_VAK_Instruction prime;
        CHECK(ql_kernel_vak_instruction_init(
                expected[i].family, 7u, 2u, 4u, QL_COORD_FACE_DIRECT, &direct) == 0,
              "vak.instruction.direct.init");
        CHECK(ql_kernel_vak_instruction_init(
                expected[i].family, 7u, 2u, 4u, QL_COORD_FACE_PRIME, &prime) == 0,
              "vak.instruction.prime.init");
        CHECK(ql_kernel_vak_instruction_valid(&direct), "vak.instruction.direct.valid");
        CHECK(ql_kernel_vak_instruction_valid(&prime), "vak.instruction.prime.valid");
        CHECK(ql_kernel_vak_instruction_face(&direct) == QL_COORD_FACE_DIRECT,
              "vak.instruction.direct.face");
        CHECK(ql_kernel_vak_instruction_face(&prime) == QL_COORD_FACE_PRIME,
              "vak.instruction.prime.face");
        CHECK(direct.vak_family == expected[i].historical_family, "vak.instruction.family");
        CHECK(direct.vak_index == 7u && direct.target_branch == 2u && direct.target_pos == 4u,
              "vak.instruction.operands");
        CHECK(direct.is_inverted == 0u && prime.is_inverted == 1u,
              "vak.instruction.inversion");
    }

    QL_Kernel_VAK_Instruction invalid;
    CHECK(ql_kernel_vak_instruction_init(
            QL_KERNEL_VAK_CS, 0u, 6u, 0u, QL_COORD_FACE_DIRECT, &invalid) != 0,
          "vak.invalid-branch");
    CHECK(ql_kernel_vak_instruction_init(
            QL_KERNEL_VAK_CS, 0u, 0u, 6u, QL_COORD_FACE_DIRECT, &invalid) != 0,
          "vak.invalid-position");

    /* The old arena pointer web is implementation evidence inside the larger VAK
     * language. Preserve its materialised cf/cs parity without treating the other
     * four universal instruction families as semantically absent. */
    QL_Holographic_Field field;
    CHECK(ql_holographic_field_init(&field) == 0, "vak.field.init");

    for (uint8_t family = QL_FAMILY_C; family <= QL_FAMILY_M; family++) {
        for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
            const QL_Holographic_Coordinate* coordinate = ql_holographic_field_get_const(
                &field, (QL_Coordinate_Family)family, position);
            CHECK(coordinate != NULL, "vak.field.coordinate");
            if (!coordinate) continue;

            QL_Kernel_Address expected_cf;
            if (position == 4u) {
                expected_cf = ql_kernel_family_address(
                    (QL_Coordinate_Family)family, 4u, QL_COORD_FACE_DIRECT);
            } else if (position == 3u) {
                expected_cf = ql_kernel_family_address(
                    (QL_Coordinate_Family)family, 4u, QL_COORD_FACE_DIRECT);
            } else {
                expected_cf = ql_kernel_position_address(4u, QL_COORD_FACE_DIRECT);
            }
            QL_Kernel_Address expected_cs = ql_kernel_family_address(
                (QL_Coordinate_Family)family, 5u, QL_COORD_FACE_DIRECT);

            CHECK(ql_relation_target_const(coordinate->cf) ==
                    ql_holographic_field_resolve(&field, expected_cf),
                  "vak.pointer-web.cf-parity");
            CHECK(ql_relation_target_const(coordinate->cs) ==
                    ql_holographic_field_resolve(&field, expected_cs),
                  "vak.pointer-web.cs-parity");
        }
    }

    /* VAK families with operands are intentionally not reduced to the generic
     * unary relation resolver; their full native instruction carries index,
     * target branch, target position and inversion. */
    QL_Kernel_Address p4 = ql_kernel_family_address(
        QL_FAMILY_P, 4u, QL_COORD_FACE_DIRECT);
    const QL_Kernel_Relation_Id parameterised[] = {
        QL_KERNEL_REL_VAK_CPF,
        QL_KERNEL_REL_VAK_CT,
        QL_KERNEL_REL_VAK_CP,
        QL_KERNEL_REL_VAK_CFP,
        QL_KERNEL_REL_VAK_CS
    };
    for (size_t i = 0u; i < sizeof(parameterised) / sizeof(parameterised[0]); i++) {
        QL_Kernel_Relation_Ref ref;
        CHECK(ql_kernel_relation_resolve(parameterised[i], p4, QL_FAMILY_NONE, &ref) != 0,
              "vak.parameterised-not-unary");
    }

    QL_Kernel_Relation_Ref cf_ref;
    CHECK(ql_kernel_relation_resolve(
            QL_KERNEL_REL_CONTEXT_FRAME, p4, QL_FAMILY_NONE, &cf_ref) == 0,
          "vak.cf.shared-context-frame-relation");
    CHECK(same_address(cf_ref.source, cf_ref.target), "vak.cf.shared-context-frame-address");

    if (failures) {
        fprintf(stderr, "R4 VAK parity failures: %u\n", failures);
        return 1;
    }
    printf("R4 VAK six-family instruction language + Context-Frame identity + pointer-web parity: PASS\n");
    return 0;
}
