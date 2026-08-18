#ifndef QL_MEF_EPI_M3_REFERENCE_COMPAT_H
#define QL_MEF_EPI_M3_REFERENCE_COMPAT_H

/*
 * Reference-only compatibility scaffolding.
 *
 * The frozen Epi `m3.h` at daa660cbc1b8c5da83828698665a753852cb0287
 * contains two `_Static_assert` expressions that index a file-scope
 * `static const` array. Those expressions are not integer constant
 * expressions under strict C11, so the header cannot be included by a strict
 * C11 parity translation unit unchanged.
 *
 * We deliberately do not repair the frozen oracle. Its normal prerequisites
 * are included first, then `_Static_assert` is suppressed only while the
 * historical m3 header is parsed so the actual historical inline operations
 * (`m3_complement`, `m3_line_change`) can execute in the parity harness.
 * The full-corpus characterization separately records m3.c/m3_clock_lut.c as
 * strict-C11 build failures, so this wrapper must never be used by production
 * QL-MEF C code.
 */
#include "m0.h"
#include "m2.h"

#define _Static_assert(...) /* historical reference compatibility only */
#include "m3.h"
#undef _Static_assert

#endif /* QL_MEF_EPI_M3_REFERENCE_COMPAT_H */
