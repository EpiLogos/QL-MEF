#include <ql/holographic.h>
#include <ql/kernel.h>
#include <ql/primitive.h>

#include <stdio.h>
#include <string.h>

int main(void) {
    if (strcmp(ql_c_api_version(), "0.1.0") != 0) return 1;
    if (strcmp(ql_kernel_api_version(), "0.1.0") != 0) return 2;

    QL_Holographic_Field field;
    if (ql_holographic_field_init(&field) != 0) return 3;
    if (!ql_holographic_field_get(&field, QL_FAMILY_M, 1u)) return 4;

    QL_Coordinate_Label p2 = ql_coordinate_label(QL_FAMILY_P, 2u, QL_COORD_FACE_BIMBA);
    QL_Coordinate_Label p2_prime = ql_coordinate_label_other_face(p2);
    if (p2_prime.position != 2u || p2_prime.face != QL_COORD_FACE_PRATIBIMBA) return 5;

    QL_Kernel_Tick tick = ql_kernel_tick_from_epogdoon(0u, 6u);
    QL_Coordinate_Label tick_label = ql_kernel_tick_position_label(&tick);
    if (tick_label.family != QL_FAMILY_P || tick_label.position != 0u ||
        tick_label.face != QL_COORD_FACE_PRATIBIMBA) return 6;

    QL_Kernel_Resonance_Map resonance;
    if (ql_kernel_resonance_map(1u, 1u, 4u, &resonance) != 0) return 7;
    if (resonance.lens.family != QL_FAMILY_L || resonance.lens.position != 1u ||
        resonance.lens.face != QL_COORD_FACE_PRATIBIMBA || resonance.inner_position != 4u) return 8;

    printf("ql-c/primitive=%s ql-c/kernel=%s revision=%s\n",
           ql_c_api_version(), ql_kernel_api_version(), ql_kernel_build_source_revision());
    return 0;
}
