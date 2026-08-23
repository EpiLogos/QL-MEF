#include <ql/holographic.h>
#include <ql/kernel.h>
#include <ql/primitive.h>

#include <stdio.h>
#include <string.h>

int main(void) {
    if (strcmp(ql_c_api_version(), "0.1.0") != 0) return 1;
    if (strcmp(ql_kernel_api_version(), "0.1.0") != 0) return 2;
    if (strcmp(ql_kernel_contract_version(), "1.1.0") != 0) return 3;

    QL_Holographic_Field field;
    if (ql_holographic_field_init(&field) != 0) return 4;

    QL_Kernel_Address hash = ql_kernel_hash_address();
    if (!ql_kernel_address_is_hash(hash) ||
        ql_holographic_field_resolve(&field, hash) != ql_default_hash_bimba()) return 5;

    QL_Kernel_Address raw2_prime = ql_kernel_position_address(2u, QL_COORD_FACE_PRIME);
    if (!ql_kernel_address_valid(raw2_prime) || raw2_prime.family != QL_FAMILY_NONE) return 6;

    QL_Kernel_Address p2 = ql_kernel_family_address(QL_FAMILY_P, 2u, QL_COORD_FACE_DIRECT);
    QL_Kernel_Relation_Ref conjugate;
    if (ql_kernel_relation_resolve(
            QL_KERNEL_REL_CROSS_SAME_POSITION,
            p2,
            QL_FAMILY_NONE,
            &conjugate) != 0) return 7;
    if (conjugate.target.position != 2u || conjugate.target.face != QL_COORD_FACE_PRIME) return 8;

    QL_Kernel_Relation_Ref mirror, complete;
    if (ql_kernel_relation_resolve(
            QL_KERNEL_REL_MIRROR_COMPLEMENT,
            p2,
            QL_FAMILY_NONE,
            &mirror) != 0) return 9;
    if (ql_kernel_relation_resolve(
            QL_KERNEL_REL_CROSS_COMPLETE,
            p2,
            QL_FAMILY_NONE,
            &complete) != 0) return 10;
    if (mirror.target.position != 3u || mirror.target.face != QL_COORD_FACE_DIRECT) return 11;
    if (complete.target.position != 3u || complete.target.face != QL_COORD_FACE_PRIME) return 12;

    QL_Kernel_Tick tick = ql_kernel_tick_from_epogdoon(0u, 6u);
    QL_Coordinate_Label tick_label = ql_kernel_tick_position_label(&tick);
    if (tick_label.family != QL_FAMILY_P || tick_label.position != 0u ||
        tick_label.face != QL_COORD_FACE_PRIME) return 13;

    QL_Kernel_MEF_Address mef;
    if (ql_kernel_mef_address(1u, QL_COORD_FACE_PRIME, 4u, &mef) != 0) return 14;
    if (mef.lens.family != QL_FAMILY_L || mef.lens.position != 1u ||
        mef.lens.face != QL_COORD_FACE_PRIME || mef.local_position != 4u ||
        mef.absolute_position != 5u) return 15;

    QL_Kernel_Context_Frame_Address cf5;
    if (ql_kernel_context_frame_address(
            QL_KERNEL_CF5,
            1u,
            QL_COORD_FACE_PRIME,
            &cf5) != 0) return 16;
    if (cf5.mef.local_position != 3u || cf5.unit_face != QL_KERNEL_MEF_UNIT_POWER ||
        cf5.grain != QL_KERNEL_MEF_GRAIN_INNER_FOUR ||
        strcmp(cf5.notation, "(4.0/1-4.4/5)") != 0) return 17;

    if (QL_KERNEL_REL_VAK_CF != QL_KERNEL_REL_CONTEXT_FRAME) return 18;
    if (ql_kernel_vak_relation_id(QL_KERNEL_VAK_CF) != QL_KERNEL_REL_CONTEXT_FRAME) return 19;
    QL_Kernel_VAK_Instruction vak;
    if (ql_kernel_vak_instruction_init(
            QL_KERNEL_VAK_CFP, 4u, 2u, 3u, QL_COORD_FACE_PRIME, &vak) != 0) return 20;
    if (!ql_kernel_vak_instruction_valid(&vak) ||
        ql_kernel_vak_instruction_face(&vak) != QL_COORD_FACE_PRIME ||
        vak.is_inverted != 1u) return 21;

    QL_Kernel_Contract_Provenance provenance = ql_kernel_contract_provenance();
    if (strcmp(provenance.historical_pointer_web_blob,
               "3eeae6f9c8cc65c5a610df1a49143b3c65bdd320") != 0) return 22;

    printf("ql-c/primitive=%s ql-c/kernel=%s contract=%s revision=%s\n",
           ql_c_api_version(), ql_kernel_api_version(), ql_kernel_contract_version(),
           ql_kernel_build_source_revision());
    return 0;
}
