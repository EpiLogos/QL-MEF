#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"
OUT="$ROOT/target/m-tree"
mkdir -p "$OUT"
python3 scripts/generate-m-tree.py --check
FLAGS=(-std=c11 -O1 -Wall -Wextra -Werror -pedantic -Ic/include)
SOURCES=(migration/epi-kernel/k2-m-tree-probe.c c/src/m_tree.c)
"${CC:-cc}" "${FLAGS[@]}" "${SOURCES[@]}" -o "$OUT/probe"
"$OUT/probe" > "$OUT/native.jsonl" 2> "$OUT/native-checks.txt"
"${CLANG:-clang}" "${FLAGS[@]}" -g -fno-omit-frame-pointer -fsanitize=address,undefined \
  "${SOURCES[@]}" -o "$OUT/sanitized-probe"
ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=halt_on_error=1 \
  "$OUT/sanitized-probe" > "$OUT/sanitized.jsonl" 2> "$OUT/sanitized-checks.txt"
cmp "$OUT/native.jsonl" "$OUT/sanitized.jsonl"
python3 -m unittest discover -s scripts/tests -p test_m_tree.py -v > "$OUT/generator-tests.txt" 2>&1
make -C c all
make -C c install DESTDIR="$OUT/install" PREFIX=/ql-mef-c
cmp fixtures/kernel/m-tree-v1.json "$OUT/install/ql-mef-c/share/ql-mef-c/m-tree-v1.json"
"${CC:-cc}" -std=c11 -Wall -Wextra -Werror -pedantic -I"$OUT/install/ql-mef-c/include" \
  migration/epi-kernel/k2-m-tree-probe.c "$OUT/install/ql-mef-c/lib/libql-mef-c.a" -lm -o "$OUT/installed-probe"
"$OUT/installed-probe" > "$OUT/installed.jsonl" 2> "$OUT/installed-checks.txt"
cmp "$OUT/native.jsonl" "$OUT/installed.jsonl"
cat "$OUT/native-checks.txt" "$OUT/generator-tests.txt" "$OUT/installed-checks.txt"
