#!/usr/bin/env bash
# Focused K5 checks; reuses existing native CI rather than adding a new job matrix.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/m1-engine
cc=${CC:-clang}
flags=(-std=c11 -O1 -g -Wall -Wextra -Werror -pedantic -ffunction-sections -fdata-sections)
includes=(-Ic/include -Ivendor/epi-kernel/reference/include -Imigration/epi-kernel/m1-return)
sources=(migration/epi-kernel/m1-engine-probe.c migration/epi-kernel/m1-return/m1_ananda_projection.c vendor/epi-kernel/reference/src/m1.c vendor/epi-kernel/reference/src/psychoid_numbers.c c/src/m_tree.c c/src/m1.c)
"$cc" "${flags[@]}" -fsanitize=address,undefined -fno-omit-frame-pointer "${includes[@]}" "${sources[@]}" -Wl,--gc-sections -lm -o target/m1-engine/sanitized
ASAN_OPTIONS=detect_leaks=1:halt_on_error=1 UBSAN_OPTIONS=halt_on_error=1 target/m1-engine/sanitized > target/m1-engine/sanitized.jsonl
python3 scripts/check-m1-source.py --observations target/m1-engine/sanitized.jsonl
make -C c install DESTDIR="$PWD/target/m1-engine/installed" PREFIX=/ql-mef-c
cat > target/m1-engine/consumer.cpp <<'CPP'
#include <ql/m1.h>
#include <cassert>
#include <cstdint>
int main() {
    QL_M1_Cell value{};
    assert(ql_m1_cell(5,3,4,UINT64_MAX,11,&value));
    assert(!value.scalar_valid && value.raw_terms[0]==12 && value.raw_terms[2]==25);
    assert(value.clock.degree720==690 && ql_m_node_by_id(value.coordinate));
    return 0;
}
CPP
"${CXX:-c++}" -std=c++17 -Wall -Wextra -Werror -pedantic -Itarget/m1-engine/installed/ql-mef-c/include target/m1-engine/consumer.cpp target/m1-engine/installed/ql-mef-c/lib/libql-mef-c.a -lm -o target/m1-engine/consumer
target/m1-engine/consumer
printf '%s\n' 'M1 native sanitizers and installed C++ consumer passed (not C++ instrument parity).'
