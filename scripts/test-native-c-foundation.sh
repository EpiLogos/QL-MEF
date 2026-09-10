#!/usr/bin/env bash
# K1 native ABI boundaries, independent compiler/sanitizer and package identity.
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
OUT="$ROOT/target/kernel-native-c"
CC_BIN=${CC:-cc}
CLANG_BIN=${CLANG:-clang}
REVISION=${SOURCE_REVISION:-$(git -C "$ROOT" rev-parse HEAD 2>/dev/null || printf unversioned)}
mkdir -p "$OUT"
cd "$ROOT"
SOURCES=(c/src/primitive.c c/src/holographic.c c/src/kernel.c)
FLAGS=(-std=c11 -Wall -Wextra -Werror -pedantic -Ic/include)
{
  uname -sm
  "$CC_BIN" --version
  "$CLANG_BIN" --version
} > "$OUT/toolchain.txt"

"$CC_BIN" "${FLAGS[@]}" migration/epi-kernel/k1-native-edges.c "${SOURCES[@]}" -lm -o "$OUT/native-edges"
"$OUT/native-edges" | tee "$OUT/native-edges.txt"
"$CLANG_BIN" "${FLAGS[@]}" -g -O1 -fno-omit-frame-pointer -fsanitize=address,undefined \
  migration/epi-kernel/k1-native-edges.c "${SOURCES[@]}" -lm -o "$OUT/sanitized-edges"
ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=halt_on_error=1 "$OUT/sanitized-edges" | tee "$OUT/sanitized-edges.txt"
"$CLANG_BIN" "${FLAGS[@]}" -g -O1 -fno-omit-frame-pointer -fsanitize=address,undefined \
  migration/epi-kernel/k1-foundation-probe.c "${SOURCES[@]}" -lm -o "$OUT/sanitized-probe"
ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=halt_on_error=1 "$OUT/sanitized-probe" > "$OUT/clang-parity.tsv"

# Reuse one build directory across revision changes: never clean between these
# two invocations. This reproduced a stale compiled revision in recovered #76.
BUILD="$OUT/revision-build"
DIST="$OUT/revision-dist"
make -C c all CC="$CC_BIN" BUILD_DIR="$BUILD" SOURCE_REVISION=k1-before > "$OUT/revision-build.txt"
make -C c all CC="$CC_BIN" BUILD_DIR="$BUILD" SOURCE_REVISION="$REVISION" >> "$OUT/revision-build.txt"
"$CC_BIN" "${FLAGS[@]}" migration/epi-kernel/k1-native-edges.c "$BUILD/libql-mef-c.a" -lm -o "$OUT/revision-smoke"
"$OUT/revision-smoke" "$REVISION" | tee "$OUT/revision-smoke.txt"
make -C c install CC="$CC_BIN" BUILD_DIR="$BUILD" SOURCE_REVISION="$REVISION" \
  DESTDIR="$OUT/revision-install" PREFIX=/ql-mef-c >> "$OUT/revision-build.txt"
grep -Fx "$REVISION" "$OUT/revision-install/ql-mef-c/share/ql-mef-c/source-revision.txt"

# A no-op rebuild does not change the archive, and reproducible package bytes
# carry the same revision as the compiled library and installed metadata.
sha256sum "$BUILD/libql-mef-c.a" > "$OUT/archive-before.sha256"
make -C c all CC="$CC_BIN" BUILD_DIR="$BUILD" SOURCE_REVISION="$REVISION" >> "$OUT/revision-build.txt"
sha256sum -c "$OUT/archive-before.sha256"
make -C c package CC="$CC_BIN" BUILD_DIR="$BUILD" DIST_DIR="$DIST" SOURCE_REVISION="$REVISION" >> "$OUT/revision-build.txt"
PACKAGE="$DIST/ql-mef-c-0.1.0-$REVISION.tar.gz"
sha256sum "$PACKAGE" > "$OUT/package-before.sha256"
make -C c package CC="$CC_BIN" BUILD_DIR="$BUILD" DIST_DIR="$DIST" SOURCE_REVISION="$REVISION" >> "$OUT/revision-build.txt"
sha256sum -c "$OUT/package-before.sha256"
printf 'K1 edge/sanitizer + incremental revision + reproducible package: PASS\n' | tee "$OUT/native-acceptance.txt"
