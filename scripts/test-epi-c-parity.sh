#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
REFERENCE_ROOT="$REPO_ROOT/vendor/epi-kernel/reference"
OUT_DIR="$REPO_ROOT/target/epi-c-parity"
CC_BIN=${CC:-cc}

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

"$CC_BIN" \
  -std=c11 -Wall -Wextra -Werror \
  -I"$REPO_ROOT/c/include" \
  -I"$REFERENCE_ROOT/include" \
  "$REPO_ROOT/migration/epi-kernel/parity-first-tranche.c" \
  "$REPO_ROOT/c/src/primitive.c" \
  "$REFERENCE_ROOT/src/kernel.c" \
  -lm \
  -o "$OUT_DIR/first-tranche-parity"

"$OUT_DIR/first-tranche-parity" | tee "$OUT_DIR/first-tranche-parity.txt"

python3 - "$OUT_DIR/parity-results.json" <<'PY'
import json
from datetime import datetime, timezone
from pathlib import Path
import sys

out = Path(sys.argv[1])
reference_revision = "daa660cbc1b8c5da83828698665a753852cb0287"
result = {
    "schema": "ql-mef.epi-c-parity/v1",
    "semantic_authority": "QL-MEF generalized primitive candidate; Epi semantic/domain composition remains Epi-owned",
    "reference": {
        "repository": "EpiLogos/Epi-Logos-C-Experiments",
        "revision": reference_revision,
        "root": "Body/S/S0/epi-lib",
        "files": [
            "include/m1.h",
            "include/m2.h",
            "include/m3.h",
            "include/kernel.h",
            "src/kernel.c",
            "test/m1/test_m1.c",
            "test/engine/test_kernel.c"
        ]
    },
    "native": {
        "repository": "EpiLogos/QL-MEF",
        "api": "ql-c/primitive",
        "version": "0.1.0",
        "header": "c/include/ql/primitive.h",
        "source": "c/src/primitive.c"
    },
    "comparison_rule": {
        "integer_and_enum": "exact",
        "harmonic_float_ratios": "exact IEEE-754 equality for identical rational expressions in this tranche",
        "structural_only": "invariant-based where the frozen source supplies cardinality/law but no callable generic operation"
    },
    "operations": [
        {"operation":"position.invert","coverage":"6 positions + invalid boundary","comparison":"exact + involution","result":"pass"},
        {"operation":"ring.wrap/half/base-position/traversal-stage","coverage":"all 256 uint8 inputs","comparison":"exact against M1 macros/helpers","result":"pass"},
        {"operation":"relation.index","coverage":"6x6 = 36","comparison":"bijection/range invariant; no frozen callable generic relation-index operation","result":"pass","readiness":"structural-native-not-reference-replacement"},
        {"operation":"state6.complement","coverage":"all 64 states","comparison":"exact against m3_complement + involution","result":"pass"},
        {"operation":"state6.line-change","coverage":"64x6 = 384","comparison":"exact against m3_line_change + involution","result":"pass"},
        {"operation":"resonance.index","coverage":"6x2x6 = 72 + invalid boundaries","comparison":"exact against kernel_resonance_index","result":"pass"},
        {"operation":"tritone-square","coverage":"6 lenses + invalid boundary","comparison":"exact against kernel_tritone_square_for_lens","result":"pass"},
        {"operation":"harmonic-ratios","coverage":"5 promoted ratios","comparison":"exact float","result":"pass"},
        {"operation":"epogdoon.tick","coverage":"all 256 uint8 sub_tick inputs at fixed cycle 17","comparison":"cycle/sub-tick/base-position/harmonic exact against kernel_tick_from_epogdoon","result":"pass"}
    ],
    "observed_discrepancies": [
        {
            "id":"ring-stage-vs-kernel-position",
            "reference_fact":"M1 ql_get_stage maps inverted-half ticks 6..11 to 5..0; kernel_tick_from_epogdoon position6 maps them to 0..5 via tick % 6",
            "native_disposition":"preserve as separate traversal_stage and base_position fields; do not reconcile in migration",
            "status":"requires explicit formal/semantic decision before any unification"
        },
        {
            "id":"ring-direction-labels",
            "reference_fact":"M1 describes ticks 0..5 as ascending and 6..11 as descending; kernel phase labels the first half DESCENT and second half ASCENT",
            "native_disposition":"use neutral direct/inverted half identity; keep historical kernel phase only in parity mapping",
            "status":"requires explicit formal/semantic decision before naming becomes normative"
        }
    ],
    "readiness": "first-native-tranche-parity-pass; no consumer flip implied",
    "observation_time_utc": datetime.now(timezone.utc).isoformat()
}
out.write_text(json.dumps(result, indent=2) + "\n")
PY

cat "$OUT_DIR/parity-results.json"
