#include "ql/m2_aperture.h"
#include "m2_aperture_data.inc"
const QL_M2_Aperture *ql_m2_aperture_at(unsigned i) { return i<QL_M2_APERTURE_COUNT?&m2_apertures[i]:NULL; }
const QL_M2_Aperture *ql_m2_aperture_by_id(QL_M_NodeId id) {
    if (!id) return NULL;
    for (unsigned i=0;i<QL_M2_APERTURE_COUNT;++i) if (m2_apertures[i].coordinate==id) return &m2_apertures[i];
    return NULL;
}
int ql_m2_void_antipode(unsigned p,unsigned *out) {
    if (p>=16 || !out) return 0;
    *out=(p+8)%16; return 1;
}
