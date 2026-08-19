#include <ql/holographic.h>
#include <ql/kernel.h>
#include <ql/primitive.h>

#include <stdio.h>
#include <string.h>

int main(void) {
    if (strcmp(ql_c_api_version(), "0.1.0") != 0) return 1;
    if (strcmp(ql_kernel_api_version(), "0.1.0") != 0) return 2;
    if (ql_position_invert(1u) != 4u) return 3;
    QL_Holographic_Field field;
    if (ql_holographic_field_init(&field) != 0) return 4;
    if (!ql_holographic_field_get(&field, QL_FAMILY_M, 1u)) return 5;
    printf("ql-c/primitive=%s ql-c/kernel=%s revision=%s\n",
           ql_c_api_version(), ql_kernel_api_version(), ql_kernel_build_source_revision());
    return 0;
}
