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
  if [[ "$suite" == "current_performance_receiving_native_wire" ]]; then
    v_receiving_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_RECEIVING_OUTPUT="$TASK_OUTPUT/current-receiving-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_receiving_nonce"
    QL_NATIVE_WIRE_TEST="$binary" QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
      QL_CURRENT_RECEIVING_ARTIFACT_OUTPUT="$TASK_RECEIVING_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored
  elif [[ "$suite" == "performance_research_native_wire" ]]; then
    v_research_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_RESEARCH_OUTPUT="$TASK_OUTPUT/research-mechanisms-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_research_nonce"
    QL_NATIVE_WIRE_TEST="$binary" QL_RESEARCH_EVIDENCE_DIR="$TASK_RESEARCH_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored
  elif [[ "$suite" == "receiving_restore_native_wire" ]]; then
    v_restore_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_RESTORE_OUTPUT="$TASK_OUTPUT/receiving-restore-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_restore_nonce"
    QL_NATIVE_WIRE_TEST="$binary" QL_RECEIVING_RESTORE_EVIDENCE_DIR="$TASK_RESTORE_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored
  else
    QL_NATIVE_WIRE_TEST="$binary" cargo test -p ql-mef --locked --test "$suite" -- --ignored
  fi
done

# Exact original SourceForm, source-key and resident reply production owners.
TASK_SOURCE_OUTPUT="$TASK_OUTPUT/source-performance"
mkdir -p "$TASK_SOURCE_OUTPUT"
cargo run --quiet -p ql-mef --locked --example retained-source-performance-fixture > "$TASK_SOURCE_OUTPUT/native-source.json"
"$TASK_OUTPUT/native/performance_source_packet-test" "$TASK_SOURCE_OUTPUT/native-source.json"
QL_NATIVE_SOURCE_REPLY_TEST="$TASK_OUTPUT/native/performance_source_reply_wire-test" cargo test -p ql-mef --locked --lib continuous::performance::reply_tests::actual_valid_other_bodies_cannot_replace_resident_source_reply -- --ignored

# Keep the real Rust producer and the C++ consumer in one executed passage.
# These are finite component fixtures; they are not installed host authority.
TASK_PROCEDURAL_OUTPUT="$TASK_OUTPUT/procedural-stage"
mkdir -p "$TASK_PROCEDURAL_OUTPUT"
TA_ONTA_FIXTURE_OUTPUT="$TASK_PROCEDURAL_OUTPUT/native-producer.json" cargo test -p ql-mef --locked --test procedural_manifestation --test procedural_composition --test procedural_stage_independent
TASK_PACKET_OUTPUT="$TASK_OUTPUT/performance-packets"
mkdir -p "$TASK_PACKET_OUTPUT"
QL_PERFORMANCE_PACKET_OUTPUT="$TASK_PACKET_OUTPUT" cargo test -p ql-mef --locked --test performance_audio actual_vimarsha_determinant_changes_octet_with_fixed_keys_metric_body_and_policy -- --exact
"$TASK_OUTPUT/native/native_performance_packet-test" "$TASK_PACKET_OUTPUT"
"$TASK_OUTPUT/native/independent_clock_admission_packet-test" "$TASK_PACKET_OUTPUT"
"$TASK_OUTPUT/native/performance_management_packet-test" "$TASK_PACKET_OUTPUT"
"$TASK_OUTPUT/native/performance_application_order_packet-test" "$TASK_PACKET_OUTPUT"
v_order_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
TASK_MANAGED_ORDER_OUTPUT="$TASK_OUTPUT/managed-application-order-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_order_nonce"
"$TASK_OUTPUT/native/performance_managed_application_order_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_MANAGED_ORDER_OUTPUT"
"$TASK_OUTPUT/native/performance_requested_timing_packet-test" "$TASK_PACKET_OUTPUT"
QL_NATIVE_RECEIVING_ADMISSION_TEST="$TASK_OUTPUT/native/performance_receiving_admission-test" cargo test -p ql-mef --locked --test performance_receiving_admission -- --ignored

# Actual post-command SourceForm material and its genuine mechanical body:
# the same q/v emits both copied visible positions and native PCM.
TASK_MATERIAL_OUTPUT="$TASK_OUTPUT/material-fold"
mkdir -p "$TASK_MATERIAL_OUTPUT"
QL_MATERIAL_FOLD_FIXTURE="$TASK_MATERIAL_OUTPUT/native-fold.json" cargo test -p ql-mef --locked --test m3_material_fold
"$TASK_OUTPUT/native/material_fold_body-test" "$TASK_MATERIAL_OUTPUT/native-fold.json" "$TASK_MATERIAL_OUTPUT/physical-observed.json" "$TASK_MATERIAL_OUTPUT/captured-native.wav"
# Preserve the actual original pre-material source and applied input journal
# for the existing C Scene/Act gate. Its optional offline fingerprint belongs
# to that native owner and is supplied by the ordinary C fixture gate.
TASK_MANAGEMENT_OUTPUT="$TASK_OUTPUT/management-artifacts"
mkdir -p "$TASK_MANAGEMENT_OUTPUT"
"$TASK_OUTPUT/native/performance_management_artifacts_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_MANAGEMENT_OUTPUT"

# Exact original context, real queue cancellation/loss and complete 45k/24voice native workload.
cargo run --quiet -p ql-mef --locked --example retained-performance-context-fixture > "$TASK_OUTPUT/retained-performance-context-fixture.json"
"$TASK_OUTPUT/native/performance_score_reservation_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_OUTPUT/native-score-reservations"
"$TASK_OUTPUT/native/performance_retained_workload_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_OUTPUT/native-retained-workload"

# Retain the same native M3 score/return and real A/P continuation for the
# existing Expression/Act tests. Full paused voice/body state is generated by
# the actual C++ owner and independently replayed before it is emitted.
cargo run --quiet -p ql-mef --locked --example retained-performance-fixture > "$TASK_OUTPUT/retained-performance-fixture.json"
"$TASK_OUTPUT/native/retained_performance_checkpoint_wire-test" "$TASK_OUTPUT/retained-performance-fixture.json" > "$TASK_OUTPUT/retained-performance-checkpoint-fixture.json"

# Independent V/#293 trials use the same compiled native producer. Copy the
# unchanged verifier into its bounded evidence directory before invocation.
cargo build -p ql-mef --locked --example m2_engine
TASK_INDEPENDENT="$TASK_OUTPUT/independent"
mkdir -p "$TASK_INDEPENDENT"
cp scripts/test-m2-producer-negatives.py "$TASK_INDEPENDENT/driver.py"
v_candidate_revision=$(git rev-parse HEAD)
v_source_revision=$(python3 -c 'import json; print(json.load(open("fixtures/kernel/m2-correspondences-v1.json"))["source_revision"])')
v_trial_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
python3 "$TASK_INDEPENDENT/driver.py" \
  --producer target/debug/examples/m2_engine \
  --fixture fixtures/kernel/m2-condition-request-v1.json \
  --output "$TASK_INDEPENDENT/run-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_trial_nonce" \
  --candidate-revision "$v_candidate_revision" \
  --source-revision "$v_source_revision"
