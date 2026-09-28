#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"
OUT_DIR="$REPO_ROOT/target/epi-c-reference"
CC_BIN=${CC:-cc}

mkdir -p "$OUT_DIR"

# shellcheck source=scripts/epi-c-reference-lib.sh
. "$REPO_ROOT/scripts/epi-c-reference-lib.sh"

"$CC_BIN" \
  -std=c11 \
  -Wall \
  -Wextra \
  "${EPI_C_REFERENCE_FLAGS[@]}" \
  "$REPO_ROOT/migration/epi-kernel/reference-smoke.c" \
  "${EPI_C_REFERENCE_SOURCES[@]}" \
  -lm \
  -o "$OUT_DIR/kernel-smoke"

"$OUT_DIR/kernel-smoke"
