#!/usr/bin/env bash
set -euo pipefail

# Synchronize or verify the frozen Epi C reference corpus.
#
# Usage:
#   scripts/sync-epi-c-reference.sh /path/to/Epi-Logos-C-Experiments
#   scripts/sync-epi-c-reference.sh --check /path/to/Epi-Logos-C-Experiments
#
# The source is read from the locked commit, not from the checkout's working tree.

MODE=sync
if [[ "${1:-}" == "--check" ]]; then
  MODE=check
  shift
fi

if [[ $# -ne 1 ]]; then
  echo "usage: $0 [--check] /path/to/Epi-Logos-C-Experiments" >&2
  exit 64
fi

EPI_REPO=$1
SOURCE_REV=daa660cbc1b8c5da83828698665a753852cb0287
SOURCE_ROOT=Body/S/S0/epi-lib
INCLUDE_TREE=f2b27d99197ee0f1cb9ed95ef52a5dd61a226e54
SRC_TREE=a60dcda1427a6ab3cfcd44565a29f988938d0881
TEST_TREE=9a6ef6505bb4e7622dba0922a07dced9bc49cd79
REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"

if ! git -C "$EPI_REPO" cat-file -e "$SOURCE_REV^{commit}" 2>/dev/null; then
  echo "locked Epi revision is not available in $EPI_REPO: $SOURCE_REV" >&2
  exit 65
fi

actual_include=$(git -C "$EPI_REPO" rev-parse "$SOURCE_REV:$SOURCE_ROOT/include")
actual_src=$(git -C "$EPI_REPO" rev-parse "$SOURCE_REV:$SOURCE_ROOT/src")
actual_test=$(git -C "$EPI_REPO" rev-parse "$SOURCE_REV:$SOURCE_ROOT/test")

[[ "$actual_include" == "$INCLUDE_TREE" ]] || { echo "include tree lock mismatch" >&2; exit 66; }
[[ "$actual_src" == "$SRC_TREE" ]] || { echo "src tree lock mismatch" >&2; exit 66; }
[[ "$actual_test" == "$TEST_TREE" ]] || { echo "test tree lock mismatch" >&2; exit 66; }

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

git -C "$EPI_REPO" archive "$SOURCE_REV" \
  "$SOURCE_ROOT/include" \
  "$SOURCE_ROOT/src" | tar -x -C "$tmp"

frozen="$tmp/$SOURCE_ROOT"

if [[ "$MODE" == "check" ]]; then
  diff -ru -- "$frozen/include" "$REFERENCE_ROOT/include"
  diff -ru -- "$frozen/src" "$REFERENCE_ROOT/src"
  echo "Epi C reference matches $SOURCE_REV"
  echo "historical test tree locked (not bulk-vendored): $TEST_TREE"
  exit 0
fi

mkdir -p "$REFERENCE_ROOT"
rm -rf "$REFERENCE_ROOT/include" "$REFERENCE_ROOT/src"
cp -R "$frozen/include" "$REFERENCE_ROOT/include"
cp -R "$frozen/src" "$REFERENCE_ROOT/src"

echo "synchronized Epi C reference from $SOURCE_REV"
echo "include tree: $INCLUDE_TREE"
echo "src tree:     $SRC_TREE"
echo "test tree:    $TEST_TREE (locked; selected source tests enter R1 deliberately)"
