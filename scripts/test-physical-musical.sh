#!/usr/bin/env bash
# Real physical/audio component and paired producer floor. This does not claim
# an installed app, device latency, fifteen-minute workload or human acceptance.
set -euo pipefail
TASK_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TASK_OUTPUT="$TASK_ROOT/target/physical-musical"
mkdir -p "$TASK_OUTPUT"
cd "$TASK_ROOT"
make -C c all
make -C cpp test BUILD_DIR="$TASK_OUTPUT/native" -j1

# Discover paired suites from native source. Each receives the exact binary
# just built from the same native source/registry, never a supplied mock frame.
for source in crates/ql-mef/tests/*_native_wire.rs; do
  [[ -f "$source" ]] || continue
  suite=${source##*/}; suite=${suite%.rs}
  native=${suite/_native_wire/_wire}
  binary="$TASK_OUTPUT/native/$native-test"
  [[ -x "$binary" ]] || { printf 'Missing paired native producer binary: %s\n' "$binary" >&2; exit 1; }
  QL_NATIVE_WIRE_TEST="$binary" cargo test -p ql-mef --locked --test "$suite" -- --ignored
done

# Keep the real Rust producer and the C++ consumer in one executed passage.
# These are finite component fixtures; they are not installed host authority.
TASK_PACKET_OUTPUT="$TASK_OUTPUT/performance-packets"
mkdir -p "$TASK_PACKET_OUTPUT"
QL_PERFORMANCE_PACKET_OUTPUT="$TASK_PACKET_OUTPUT" cargo test -p ql-mef --locked --test performance_audio actual_vimarsha_determinant_changes_octet_with_fixed_keys_metric_body_and_policy -- --exact
"$TASK_OUTPUT/native/native_performance_packet-test" "$TASK_PACKET_OUTPUT"
