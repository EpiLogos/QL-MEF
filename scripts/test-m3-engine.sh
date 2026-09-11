#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
out="$root/target/m3-native"
mkdir -p "$out"
python3 scripts/generate-m3.py --out "$out/m3_data.inc"
for compiler in cc clang; do
  command -v "$compiler" >/dev/null
  "$compiler" -std=c11 -O1 -Wall -Wextra -Werror -pedantic -Ic/include -I"$out" \
    migration/epi-kernel/k7-m3-probe.c c/src/m3.c c/src/m_tree.c -lm -o "$out/$compiler-probe"
  "$out/$compiler-probe" > "$out/$compiler.jsonl"
  "$compiler" -std=c11 -O1 -ffunction-sections -fdata-sections -Wl,--gc-sections \
    -Ic/include -I"$out" -Ivendor/epi-kernel/reference/include \
    migration/epi-kernel/k7-m3-oracle.c c/src/m3.c c/src/m_tree.c -lm -o "$out/$compiler-oracle"
  "$out/$compiler-oracle" | tee "$out/$compiler-oracle.txt"
done
cmp "$out/cc.jsonl" "$out/clang.jsonl"
clang -std=c11 -O1 -g -Wall -Wextra -Werror -pedantic -Ic/include -I"$out" \
  -fsanitize=address,undefined -fno-omit-frame-pointer \
  migration/epi-kernel/k7-m3-probe.c c/src/m3.c c/src/m_tree.c -lm -o "$out/sanitized"
ASAN_OPTIONS=detect_leaks=1 "$out/sanitized" > "$out/sanitized.jsonl"
cmp "$out/cc.jsonl" "$out/sanitized.jsonl"
make -C c BUILD_DIR="$out/build" install DESTDIR="$out/install" PREFIX=/ql-mef-c
cat > "$out/client.cpp" <<'CPP'
#include <ql/m3.h>
#include <cassert>
#include <cstring>
int main() {
  QL_M3_Clock c{};
  assert(ql_m3_clock(720, &c) == QL_M3_OK);
  assert(c.completed_double_covers == 1 && c.degree720 == 0);
  auto *node = ql_m_node_by_id(c.degree_node);
  assert(node && std::strcmp(node->source_ref, "#3-5-5/0-0/360") == 0);
  QL_M3_CodonRecord codon{};
  assert(ql_m3_codon(0, &codon) == QL_M3_OK);
  assert(codon.charges[0] == 18);
  assert(ql_m3_clock_reconciled_symbolic_fields() == 0);
  assert(ql_m3_codon(64, &codon) == QL_M3_INVALID);
}
CPP
c++ -std=c++17 -Wall -Wextra -Werror -pedantic -I"$out/install/ql-mef-c/include" \
  "$out/client.cpp" "$out/install/ql-mef-c/lib/libql-mef-c.a" -lm -o "$out/client"
(cd /tmp && "$out/client")
printf 'native M3: retained oracle, cc/clang, sanitizers, installed C++ consumer passed\n'
