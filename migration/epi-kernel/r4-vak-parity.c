#include "ql/holographic.h"
#include "ql/kernel.h"

#include <stdio.h>
#include <string.h>

static unsigned failures = 0u;
#define CHECK(cond, op) do { if (!(cond)) { fprintf(stderr, "FAIL\t%s\n", (op)); failures++; } } while (0)

static int same_address(QL_Kernel_Address a, QL_Kernel_Address b) {
    return a.family == b.family && a.position == b.position && a.face == b.face;
}

int main(void) {
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_VAK_CPF), "ql.kernel.vak.cpf/v1") == 0,
          "vak.id.cpf");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_VAK_CT), "ql.kernel.vak.ct/v1") == 0,
          "vak.id.ct");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_VAK_CP), "ql.kernel.vak.cp/v1") == 0,
          "vak.id.cp");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_VAK_CF), "ql.kernel.vak.cf/v1") == 0,
          "vak.id.cf");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_VAK_CFP), "ql.kernel.vak.cfp/v1") == 0,
          "vak.id.cfp");
    CHECK(strcmp(ql_kernel_relation_id(QL_KERNEL_REL_VAK_CS), "ql.kernel.vak.cs/v1") == 0,
          "vak.id.cs");
    CHECK(QL_KERNEL_REL_VAK_CF != QL_KERNEL_REL_CONTEXT_FRAME,
          "vak.cf-distinct-from-typed-context-frame");

    QL_Holographic_Field field;
    CHECK(ql_holographic_field_init(&field) == 0, "vak.field.init");

    for (uint8_t family = QL_FAMILY_C; family <= QL_FAMILY_M; family++) {
        for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
            QL_Kernel_Address source = ql_kernel_family_address(
                (QL_Coordinate_Family)family,
                position,
                QL_COORD_FACE_DIRECT
            );
            QL_Kernel_Relation_Ref cf;
            QL_Kernel_Relation_Ref cs;
            CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_VAK_CF, source, QL_FAMILY_NONE, &cf) == 0,
                  "vak.cf.family.resolve");
            CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_VAK_CS, source, QL_FAMILY_NONE, &cs) == 0,
                  "vak.cs.family.resolve");

            QL_Kernel_Address expected_cf;
            if (position == 4u) {
                expected_cf = source;
            } else if (position == 3u) {
                expected_cf = ql_kernel_family_address(
                    (QL_Coordinate_Family)family, 4u, QL_COORD_FACE_DIRECT);
            } else {
                expected_cf = ql_kernel_position_address(4u, QL_COORD_FACE_DIRECT);
            }
            QL_Kernel_Address expected_cs = ql_kernel_family_address(
                (QL_Coordinate_Family)family, 5u, QL_COORD_FACE_DIRECT);

            CHECK(same_address(cf.target, expected_cf), "vak.cf.family.semantic-target");
            CHECK(same_address(cs.target, expected_cs), "vak.cs.family.semantic-target");

            const QL_Holographic_Coordinate* coordinate = ql_holographic_field_get_const(
                &field, (QL_Coordinate_Family)family, position);
            CHECK(coordinate != NULL, "vak.field.coordinate");
            CHECK(ql_relation_target_const(coordinate->cf) ==
                    ql_holographic_field_resolve(&field, cf.target),
                  "vak.cf.family.pointer-parity");
            CHECK(ql_relation_target_const(coordinate->cs) ==
                    ql_holographic_field_resolve(&field, cs.target),
                  "vak.cs.family.pointer-parity");

            QL_Kernel_Address prime = ql_kernel_family_address(
                (QL_Coordinate_Family)family,
                position,
                QL_COORD_FACE_PRIME
            );
            CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_VAK_CF, prime, QL_FAMILY_NONE, &cf) != 0,
                  "vak.cf.prime-unasserted");
            CHECK(ql_kernel_relation_resolve(
                    QL_KERNEL_REL_VAK_CS, prime, QL_FAMILY_NONE, &cs) != 0,
                  "vak.cs.prime-unasserted");
        }
    }

    for (uint8_t position = 0u; position < QL_POSITION_COUNT; position++) {
        QL_Kernel_Address raw = ql_kernel_position_address(position, QL_COORD_FACE_DIRECT);
        QL_Kernel_Relation_Ref ref;
        int cf_result = ql_kernel_relation_resolve(
            QL_KERNEL_REL_VAK_CF, raw, QL_FAMILY_NONE, &ref);
        if (position == 3u || position == 4u) {
            CHECK(cf_result == 0, "vak.cf.raw-wired.resolve");
            CHECK(same_address(ref.target,
                    ql_kernel_position_address(4u, QL_COORD_FACE_DIRECT)),
                  "vak.cf.raw-wired.target");
        } else {
            CHECK(cf_result != 0, "vak.cf.raw-unasserted");
        }

        CHECK(ql_kernel_relation_resolve(
                QL_KERNEL_REL_VAK_CS, raw, QL_FAMILY_NONE, &ref) == 0,
              "vak.cs.raw.resolve");
        CHECK(same_address(ref.target,
                ql_kernel_position_address(5u, QL_COORD_FACE_DIRECT)),
              "vak.cs.raw.target");
    }

    const QL_Kernel_Relation_Id unwired[] = {
        QL_KERNEL_REL_VAK_CPF,
        QL_KERNEL_REL_VAK_CT,
        QL_KERNEL_REL_VAK_CP,
        QL_KERNEL_REL_VAK_CFP
    };
    QL_Kernel_Address p4 = ql_kernel_family_address(
        QL_FAMILY_P, 4u, QL_COORD_FACE_DIRECT);
    for (size_t i = 0u; i < sizeof(unwired) / sizeof(unwired[0]); i++) {
        QL_Kernel_Relation_Ref ref;
        CHECK(ql_kernel_relation_resolve(unwired[i], p4, QL_FAMILY_NONE, &ref) != 0,
              "vak.declared-unwired-stays-unresolved");
    }

    if (failures) {
        fprintf(stderr, "R4 VAK parity failures: %u\n", failures);
        return 1;
    }
    printf("R4 VAK reflective relation identity + historical wiring parity: PASS\n");
    return 0;
}
