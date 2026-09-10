/* Invalid inputs must not silently acquire a valid canonical identity. */
#include "ql/kernel.h"
#include <stdio.h>
#include <string.h>

static unsigned checks, failures;
#define CHECK(c) do { ++checks; if (!(c)) { fprintf(stderr, "FAIL line %d: %s\n", __LINE__, #c); ++failures; } } while (0)

int main(int argc, char** argv) {
    const int invalid_families[] = {-1, 6, 8, 255, 256, 257, 263, 512};
    const int invalid_faces[] = {-1, 2, 255, 256, 257, 512};
    QL_Holographic_Field field;
    CHECK(ql_holographic_field_init(&field) == 0);
    QL_Holographic_Coordinate coordinate;
    CHECK(ql_coordinate_init(&coordinate, QL_FAMILY_P, 4) == 0);
    for (size_t i = 0; i < sizeof(invalid_families)/sizeof(invalid_families[0]); ++i) {
        QL_Coordinate_Family family = (QL_Coordinate_Family)invalid_families[i];
        CHECK(!ql_kernel_address_valid(ql_kernel_family_address(family, 0, QL_COORD_FACE_DIRECT)));
        CHECK(ql_coordinate_init(&coordinate, family, 0) != 0);
        CHECK(ql_holographic_field_get(&field, family, 0) == NULL);
        QL_Kernel_Relation_Ref ref;
        CHECK(ql_kernel_relation_resolve(QL_KERNEL_REL_FAMILY_SAME_POSITION,
              ql_kernel_position_address(0, QL_COORD_FACE_DIRECT), family, &ref) != 0);
    }
    for (size_t i = 0; i < sizeof(invalid_faces)/sizeof(invalid_faces[0]); ++i) {
        QL_Coordinate_Face face = (QL_Coordinate_Face)invalid_faces[i];
        CHECK(!ql_kernel_address_valid(ql_kernel_family_address(QL_FAMILY_C, 0, face)));
        CHECK(!ql_kernel_address_valid(ql_kernel_position_address(0, face)));
        uint8_t before = coordinate.inversion_state;
        CHECK(ql_coordinate_set_face(&coordinate, face) != 0);
        CHECK(coordinate.inversion_state == before);
        CHECK(ql_kernel_pitch_class(0, face) == QL_INVALID_U8);
        QL_Kernel_VAK_Instruction instruction;
        CHECK(ql_kernel_vak_instruction_init(QL_KERNEL_VAK_CF, 0, 0, 0, face, &instruction) != 0);
        QL_Kernel_MEF_Address mef;
        CHECK(ql_kernel_mef_address(0, face, 0, &mef) != 0);
    }
    QL_Kernel_Address invalid_hash = ql_kernel_hash_address();
    invalid_hash.face = QL_COORD_FACE_PRIME;
    CHECK(!ql_kernel_address_valid(invalid_hash));
    CHECK(ql_holographic_field_resolve(&field, invalid_hash) == NULL);
    CHECK(!ql_kernel_address_valid(ql_kernel_position_address(6, QL_COORD_FACE_DIRECT)));
    CHECK(!ql_kernel_address_valid(ql_kernel_position_address(255, QL_COORD_FACE_DIRECT)));
    char buffer[8] = "guard";
    CHECK(ql_kernel_address_format(invalid_hash, buffer, sizeof(buffer)) != 0);
    CHECK(strcmp(buffer, "guard") == 0);
    CHECK(ql_kernel_address_format(ql_kernel_hash_address(), NULL, 8) != 0);
    CHECK(ql_kernel_address_format(ql_kernel_hash_address(), buffer, 0) != 0);
    CHECK(ql_kernel_address_format(ql_kernel_hash_address(), buffer, 1) != 0);
    CHECK(buffer[0] == '\0');
    CHECK(ql_holographic_field_init(NULL) != 0);
    CHECK(ql_coordinate_materialize(NULL, &coordinate) != 0);
    CHECK(ql_kernel_relation_id((QL_Kernel_Relation_Id)-1) == NULL);
    CHECK(ql_kernel_relation_id((QL_Kernel_Relation_Id)QL_KERNEL_RELATION_COUNT) == NULL);
    QL_Kernel_Relation_Ref ref;
    memset(&ref, 0xA5, sizeof(ref));
    unsigned char saved[sizeof(ref)];
    memcpy(saved, &ref, sizeof(ref));
    CHECK(ql_kernel_relation_resolve(QL_KERNEL_REL_POSITION_SUCCESSOR,
          ql_kernel_position_address(5, QL_COORD_FACE_DIRECT), QL_FAMILY_NONE, &ref) != 0);
    CHECK(memcmp(saved, &ref, sizeof(ref)) == 0);
    CHECK(ql_kernel_relation_resolve(QL_KERNEL_REL_MOBIUS_RETURN,
          ql_kernel_position_address(0, QL_COORD_FACE_DIRECT), QL_FAMILY_NONE, &ref) != 0);
    CHECK(memcmp(saved, &ref, sizeof(ref)) == 0);
    CHECK(ql_kernel_vak_instruction_valid(NULL) == 0);
    QL_Kernel_Context_Frame_Address cf;
    CHECK(ql_kernel_context_frame_address((QL_Kernel_Context_Frame_Id)7, 0, QL_COORD_FACE_DIRECT, &cf) != 0);
    CHECK(ql_kernel_context_frame_address((QL_Kernel_Context_Frame_Id)-1, 0, QL_COORD_FACE_DIRECT, &cf) != 0);
    if (argc == 2) CHECK(strcmp(ql_kernel_build_source_revision(), argv[1]) == 0);
    printf("K1 native edge checks: %u; failures: %u; revision: %s\n", checks, failures, ql_kernel_build_source_revision());
    return failures ? 1 : 0;
}
