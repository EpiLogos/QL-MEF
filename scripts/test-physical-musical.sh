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

# Independent real leaves keep running after an actual failed assertion. A
# producer failure gates only its own consumers; every failure remains RED.
# Build/packaging foundations above must succeed before any native is run.
TASK_GATE_FAILURES=()
v_gate_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
TASK_GATE_OUTPUT="$TASK_OUTPUT/full-gate-outputs-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_gate_nonce"
mkdir "$TASK_GATE_OUTPUT"
run_native_gate() {
  local gate_name=$1
  shift
  local gate_status=0 stdout_status=0 stderr_status=0 stdout_pid stderr_pid
  local gate_output="$TASK_GATE_OUTPUT/$gate_name"
  # Each original command keeps its exact argv, stdout, stderr and exit code.
  # Explicit tee lifetimes finish before its receipt/producer file is consumed.
  if ! mkdir "$gate_output"; then
    TASK_GATE_FAILURES+=("$gate_name-output-directory")
    return 1
  fi
  if ! printf '%s\0' "$@" > "$gate_output/argv.nul" ||
     ! date -u +'%Y-%m-%dT%H:%M:%SZ' > "$gate_output/started-utc.txt" ||
     ! mkfifo "$gate_output/stdout.pipe" "$gate_output/stderr.pipe"; then
    TASK_GATE_FAILURES+=("$gate_name-output-prepare")
    return 1
  fi
  tee "$gate_output/stdout.log" < "$gate_output/stdout.pipe" &
  stdout_pid=$!
  tee "$gate_output/stderr.log" < "$gate_output/stderr.pipe" >&2 &
  stderr_pid=$!
  if "$@" > "$gate_output/stdout.pipe" 2> "$gate_output/stderr.pipe"; then
    gate_status=0
  else
    gate_status=$?
  fi
  if wait "$stdout_pid"; then :; else stdout_status=$?; fi
  if wait "$stderr_pid"; then :; else stderr_status=$?; fi
  if ! rm "$gate_output/stdout.pipe" "$gate_output/stderr.pipe" ||
     ! date -u +'%Y-%m-%dT%H:%M:%SZ' > "$gate_output/finished-utc.txt" ||
     ! printf '%s\n' "$gate_status" > "$gate_output/exit-code.txt" ||
     ! printf 'stdout=%s stderr=%s\n' "$stdout_status" "$stderr_status" > "$gate_output/capture-exit-code.txt"; then
    TASK_GATE_FAILURES+=("$gate_name-output-finish")
    return 1
  fi
  if (( stdout_status || stderr_status )); then
    TASK_GATE_FAILURES+=("$gate_name-output:$stdout_status:$stderr_status")
    printf 'native_gate name=%s status=output-failed stdout=%s stderr=%s\n' "$gate_name" "$stdout_status" "$stderr_status" >&2
    return 1
  fi
  if (( gate_status )); then
    TASK_GATE_FAILURES+=("$gate_name:$gate_status")
    printf 'native_gate name=%s status=failed exit=%s\n' "$gate_name" "$gate_status" >&2
    return "$gate_status"
  fi
  printf 'native_gate name=%s status=passed\n' "$gate_name" >&2
  return 0
}

# Compile the actual shared lib test binary once before concurrent invocations.
# This is an additional build foundation; every original test command still runs.
if ! run_native_gate native-lib-build cargo test -p ql-mef --locked --lib --no-run; then
  exit 1
fi

# Two independent correctness branches own separate native processes/evidence.
# No callback/device latency or 900s workload runs during this parallel window.
v_body_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
TASK_BODY_OUTPUT="$TASK_OUTPUT/current-body-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_body_nonce"
mkdir -p "$TASK_BODY_OUTPUT"

run_form_replay_branch() {
  run_native_gate source-form-physical-cold env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
    QL_NATIVE_PHYSICAL_SOURCE_REPLAY_ARTIFACT="$TASK_BODY_OUTPUT/full-original-form-cold.json" \
    cargo test -p ql-mef --locked --lib continuous::performance::form::cold::tests::actual_stopped_form_material_applications_cold_replay_full_original_owner_and_detect_loss -- --ignored --exact --nocapture || :
  run_native_gate source-form-physical-cold-artifact test -s "$TASK_BODY_OUTPUT/full-original-form-cold.json" || :
  if (( ${#TASK_GATE_FAILURES[@]} )); then return 1; fi
}
run_other_replay_branch() {
  run_native_gate default-instrument-calibration env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
    QL_NATIVE_INSTRUMENT_CALIBRATION_ARTIFACT="$TASK_BODY_OUTPUT/full-original-twelve-node-calibration.json" \
    cargo test -p ql-mef --locked --lib continuous::performance::current_configuration::calibration_tests::actual_default_twelve_node_instrument_calibration_measures_native_force_body_receiving_and_pcm -- --ignored --exact --nocapture || :
  run_native_gate default-instrument-calibration-artifact test -s "$TASK_BODY_OUTPUT/full-original-twelve-node-calibration.json" || :

  v_source_reply_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
  TASK_SOURCE_REPLY_OUTPUT="$TASK_OUTPUT/source-reply-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_source_reply_nonce"
  run_native_gate source-replies env QL_NATIVE_SOURCE_REPLY_TEST="$TASK_OUTPUT/native/performance_source_reply_wire-test" \
    QL_NATIVE_SOURCE_REPLY_EVIDENCE_DIR="$TASK_SOURCE_REPLY_OUTPUT" \
    cargo test -p ql-mef --locked --lib continuous::performance::reply_tests::actual_valid_other_bodies_cannot_replace_resident_source_reply -- --ignored || :

  v_dense_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
  TASK_DENSE_OUTPUT="$TASK_OUTPUT/dense-source-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_dense_nonce"
  mkdir -p "$TASK_DENSE_OUTPUT"
  run_native_gate dense-field-source env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
    QL_NATIVE_DENSE_FIELD_SOURCE_ARTIFACT="$TASK_DENSE_OUTPUT/original-current-native-source.json" \
    cargo test -p ql-mef --locked --lib continuous::host::dense_field_source_tests::actual_dense_field_source_keeps_all_65000_original_samples_and_later_basis -- --ignored --nocapture || :

  run_native_gate source-original703-world-personal-shared cargo test -p ql-mef --locked --lib continuous::performance::form::cold::tests::genuine_original703_world_personal_shared_assets_replay_without_rewriting_current_authority -- --exact --nocapture || :
  run_native_gate source-form-original703-replay env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
    cargo test -p ql-mef --locked --lib continuous::performance::form::cold::tests::actual_native_original703_form_material_source_replays_full_history_and_fresh_admission -- --ignored --exact --nocapture || :
  run_native_gate mixed-acoustic-source-cold env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
    QL_NATIVE_ACOUSTIC_SOURCE_REPLAY_ARTIFACT="$TASK_BODY_OUTPUT/full-original-mixed-acoustic-cold.json" \
    cargo test -p ql-mef --locked --lib continuous::performance::form::cold::acoustic_history::tests::actual_m4_install_move_body_material_cold_replay_preserves_pcm_ring_and_all_source_epochs -- --ignored --exact --nocapture || :
  run_native_gate mixed-acoustic-source-cold-artifact test -s "$TASK_BODY_OUTPUT/full-original-mixed-acoustic-cold.json" || :
  run_native_gate warm-source-original-prefixes env \
    QL_NATIVE_ACOUSTIC_SOURCE_REPLAY_ARTIFACT="$TASK_BODY_OUTPUT/full-original-mixed-acoustic-cold.json" \
    cargo test -p ql-mef --locked --lib continuous::performance::contact_warm::source_tests::actual_native_acoustic_corpus_retains_exact_warm_prefixes -- --ignored --exact --nocapture || :
  if (( ${#TASK_GATE_FAILURES[@]} )); then return 1; fi
}
# Each branch retains all its failed leaves and continues its original commands.
# A failed sibling never cancels or shortens the other; both are always awaited.
run_form_replay_branch &
TASK_FORM_REPLAY_PID=$!
run_other_replay_branch &
TASK_OTHER_REPLAY_PID=$!
TASK_PARALLEL_FAILURES=()
if wait "$TASK_FORM_REPLAY_PID"; then :; else
  TASK_PARALLEL_FAILURES+=("form-replay:$?")
fi
if wait "$TASK_OTHER_REPLAY_PID"; then :; else
  TASK_PARALLEL_FAILURES+=("other-replay:$?")
fi
# Child arrays are process-local. Collect each original failed command from its
# durable status after BOTH lifetimes have ended, without inventing a new result.
for TASK_GATE_STATUS in "$TASK_GATE_OUTPUT"/*/exit-code.txt; do
  read -r TASK_GATE_EXIT < "$TASK_GATE_STATUS"
  if (( TASK_GATE_EXIT )); then
    TASK_GATE_NAME=${TASK_GATE_STATUS%/exit-code.txt}
    TASK_GATE_NAME=${TASK_GATE_NAME##*/}
    TASK_GATE_FAILURES+=("$TASK_GATE_NAME:$TASK_GATE_EXIT")
  fi
done
TASK_GATE_FAILURES+=("${TASK_PARALLEL_FAILURES[@]}")

# Discover paired suites from native source. Each receives the exact binary
# just built from the same native source/registry, never a supplied mock frame.
for source in crates/ql-mef/tests/*_native_wire.rs; do
  [[ -f "$source" ]] || continue
  suite=${source##*/}; suite=${suite%.rs}
  native=${suite/_native_wire/_wire}
  binary="$TASK_OUTPUT/native/$native-test"
  if ! run_native_gate "$suite-native-built" test -x "$binary"; then continue; fi
  if [[ "$suite" == "current_performance_receiving_native_wire" ]]; then
    v_receiving_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_RECEIVING_OUTPUT="$TASK_OUTPUT/current-receiving-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_receiving_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
      QL_CURRENT_RECEIVING_ARTIFACT_OUTPUT="$TASK_RECEIVING_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_research_native_wire" ]]; then
    v_research_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_RESEARCH_OUTPUT="$TASK_OUTPUT/research-mechanisms-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_research_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_RESEARCH_EVIDENCE_DIR="$TASK_RESEARCH_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_receiving_port_native_wire" ]]; then
    v_port_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_PORT_OUTPUT="$TASK_OUTPUT/receiving-port-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_port_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_RECEIVING_PORT_EVIDENCE_DIR="$TASK_PORT_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "receiving_restore_native_wire" ]]; then
    v_restore_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_RESTORE_OUTPUT="$TASK_OUTPUT/receiving-restore-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_restore_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_RECEIVING_RESTORE_EVIDENCE_DIR="$TASK_RESTORE_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_acoustic_native_wire" ]]; then
    v_acoustic_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_ACOUSTIC_OUTPUT="$TASK_OUTPUT/acoustic-receiving-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_acoustic_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_ACOUSTIC_RECEIVING_EVIDENCE_DIR="$TASK_ACOUSTIC_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_moving_receiving_native_wire" ]]; then
    v_motion_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_MOTION_OUTPUT="$TASK_OUTPUT/moving-receiving-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_motion_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_MOVING_RECEIVING_EVIDENCE_DIR="$TASK_MOTION_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_route_management_native_wire" ]]; then
    # Retain actual born/pending/applied Force and complete cold continuation
    # from the SAME native source, queue and Manager, never reconstructed data.
    v_prearm_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_PREARM_OUTPUT="$TASK_OUTPUT/prearm-source-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_prearm_nonce"
    mkdir -p "$TASK_PREARM_OUTPUT"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" \
      QL_NATIVE_PREARM_RECEIPT_DIR="$TASK_PREARM_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_capture_native_wire" ]]; then
    v_capture_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_CAPTURE_OUTPUT="$TASK_OUTPUT/native-capture-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_capture_nonce"
    mkdir -p "$TASK_CAPTURE_OUTPUT"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" \
      QL_NATIVE_CAPTURE_RECEIPT_DIR="$TASK_CAPTURE_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  elif [[ "$suite" == "performance_calibration_native_wire" ]]; then
    v_calibration_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
    TASK_CALIBRATION_OUTPUT="$TASK_OUTPUT/calibration-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_calibration_nonce"
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
      QL_CALIBRATION_EVIDENCE_DIR="$TASK_CALIBRATION_OUTPUT" \
      cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  else
    run_native_gate "$suite" env QL_NATIVE_WIRE_TEST="$binary" cargo test -p ql-mef --locked --test "$suite" -- --ignored || :
  fi
done

# OFF-default diagnostic custody exercised only with this real compiled worker.
v_malloc_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
TASK_MALLOC_OUTPUT="$TASK_OUTPUT/worker-malloc-custody-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_malloc_nonce"
run_native_gate worker-malloc-custody env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
  QL_WORKER_MALLOC_CUSTODY_TEST_OUTPUT="$TASK_MALLOC_OUTPUT" \
  cargo test -p ql-mef --locked --lib continuous::worker_malloc_custody_tests::actual_worker_retains_only_selected_bytes_and_native_stderr_across_close -- --ignored || :

run_native_gate native-field-clock env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
  cargo test -p ql-mef --locked --lib continuous::receipt::tests::actual_native_field_clock_constructor_continues_across_owned_operations -- --ignored || :

run_native_gate native-contact-generic-ingress env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
  cargo test -p ql-mef --locked --lib continuous::host::scene_contact_ingress_tests::actual_prepared_worker_denies_generic_contact_without_changing_full_checkpoint -- --ignored || :

run_native_gate initial-acoustic-candidate env QL_NATIVE_FIELD_WORKER="$TASK_OUTPUT/native/ql-field-worker" \
  cargo test -p ql-mef --locked --lib continuous::host::acoustic_initial_tests::actual_native_acoustic_initial_candidate_is_pure_and_detects_stale_boundary -- --ignored || :

# Exact original SourceForm, source-key and resident reply production owners.
TASK_SOURCE_OUTPUT="$TASK_OUTPUT/source-performance"
mkdir -p "$TASK_SOURCE_OUTPUT"
if run_native_gate source-performance-producer cargo run --quiet -p ql-mef --locked --example retained-source-performance-fixture > "$TASK_SOURCE_OUTPUT/native-source.json"; then
  run_native_gate source-performance-packet "$TASK_OUTPUT/native/performance_source_packet-test" "$TASK_SOURCE_OUTPUT/native-source.json" || :
fi
# Keep the real Rust producer and the C++ consumer in one executed passage.
# These are finite component fixtures; they are not installed host authority.
TASK_PROCEDURAL_OUTPUT="$TASK_OUTPUT/procedural-stage"
mkdir -p "$TASK_PROCEDURAL_OUTPUT"
run_native_gate procedural-stage env TA_ONTA_FIXTURE_OUTPUT="$TASK_PROCEDURAL_OUTPUT/native-producer.json" cargo test -p ql-mef --locked --test procedural_manifestation --test procedural_composition --test procedural_stage_independent || :
# Export the existing real native-control producer's radius/strength
# takeover and release for its independent Expression consumer tests.
run_native_gate procedural-controls env \
  QL_PROCEDURAL_CONTROL_CONSUMER_ARTIFACT="$TASK_PROCEDURAL_OUTPUT/native-radius-control.json" \
  QL_PROCEDURAL_CONTROL_STRENGTH_ARTIFACT="$TASK_PROCEDURAL_OUTPUT/native-strength-control.json" \
  cargo test -p ql-mef --locked --test procedural_control || :
TASK_PACKET_OUTPUT="$TASK_OUTPUT/performance-packets"
mkdir -p "$TASK_PACKET_OUTPUT"
TASK_PACKETS_READY=false
if run_native_gate native-performance-producer env QL_PERFORMANCE_PACKET_OUTPUT="$TASK_PACKET_OUTPUT" cargo test -p ql-mef --locked --test performance_audio actual_vimarsha_determinant_changes_octet_with_fixed_keys_metric_body_and_policy -- --exact; then
TASK_PACKETS_READY=true
run_native_gate native_performance_packet "$TASK_OUTPUT/native/native_performance_packet-test" "$TASK_PACKET_OUTPUT" || :
run_native_gate performance_contact_slots_packet "$TASK_OUTPUT/native/performance_contact_slots_packet-test" "$TASK_PACKET_OUTPUT" || :
run_native_gate performance_scene_contact_geometry_packet "$TASK_OUTPUT/native/performance_scene_contact_geometry_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_PACKET_OUTPUT/actual-native-scene-contact-geometry.json" || :
run_native_gate independent_clock_admission_packet "$TASK_OUTPUT/native/independent_clock_admission_packet-test" "$TASK_PACKET_OUTPUT" || :
run_native_gate performance_management_packet "$TASK_OUTPUT/native/performance_management_packet-test" "$TASK_PACKET_OUTPUT" || :
run_native_gate performance_application_order_packet "$TASK_OUTPUT/native/performance_application_order_packet-test" "$TASK_PACKET_OUTPUT" || :
v_order_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
TASK_MANAGED_ORDER_OUTPUT="$TASK_OUTPUT/managed-application-order-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_order_nonce"
run_native_gate performance_managed_application_order_packet "$TASK_OUTPUT/native/performance_managed_application_order_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_MANAGED_ORDER_OUTPUT" || :
run_native_gate performance_requested_timing_packet "$TASK_OUTPUT/native/performance_requested_timing_packet-test" "$TASK_PACKET_OUTPUT" || :
run_native_gate receiving-admission env QL_NATIVE_RECEIVING_ADMISSION_TEST="$TASK_OUTPUT/native/performance_receiving_admission-test" cargo test -p ql-mef --locked --test performance_receiving_admission -- --ignored || :
fi

# Actual post-command SourceForm material and its genuine mechanical body:
# the same q/v emits both copied visible positions and native PCM.
TASK_MATERIAL_OUTPUT="$TASK_OUTPUT/material-fold"
mkdir -p "$TASK_MATERIAL_OUTPUT"
if run_native_gate material-fold-producer env QL_MATERIAL_FOLD_FIXTURE="$TASK_MATERIAL_OUTPUT/native-fold.json" QL_SOURCE_FORM_RECEIVING_FIXTURE="$TASK_MATERIAL_OUTPUT/native-source-form-receiving.json" cargo test -p ql-mef --locked --test m3_material_fold; then
  run_native_gate material-fold-body "$TASK_OUTPUT/native/material_fold_body-test" "$TASK_MATERIAL_OUTPUT/native-fold.json" "$TASK_MATERIAL_OUTPUT/physical-observed.json" "$TASK_MATERIAL_OUTPUT/captured-native.wav" || :
  run_native_gate source-form-receiving-artifact test -s "$TASK_MATERIAL_OUTPUT/native-source-form-receiving.json" || :
fi
# Preserve the actual original pre-material source and applied input journal
# for the existing C Scene/Act gate. Its optional offline fingerprint belongs
# to that native owner and is supplied by the ordinary C fixture gate.
TASK_MANAGEMENT_OUTPUT="$TASK_OUTPUT/management-artifacts"
mkdir -p "$TASK_MANAGEMENT_OUTPUT"
if [[ "$TASK_PACKETS_READY" == true ]]; then
  run_native_gate management-artifacts "$TASK_OUTPUT/native/performance_management_artifacts_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_MANAGEMENT_OUTPUT" || :
fi

# Exact original context, real queue cancellation/loss and complete 45k/24voice native workload.
run_native_gate retained-context-producer cargo run --quiet -p ql-mef --locked --example retained-performance-context-fixture > "$TASK_OUTPUT/retained-performance-context-fixture.json" || :
if [[ "$TASK_PACKETS_READY" == true ]]; then
  run_native_gate performance_score_reservation_packet "$TASK_OUTPUT/native/performance_score_reservation_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_OUTPUT/native-score-reservations" || :
fi
if [[ "$TASK_PACKETS_READY" == true ]]; then
  run_native_gate performance_retained_workload_packet "$TASK_OUTPUT/native/performance_retained_workload_packet-test" "$TASK_PACKET_OUTPUT" "$TASK_OUTPUT/native-retained-workload" || :
fi

# Retain the same native M3 score/return and real A/P continuation for the
# existing Expression/Act tests. Full paused voice/body state is generated by
# the actual C++ owner and independently replayed before it is emitted.
if run_native_gate retained-performance-producer cargo run --quiet -p ql-mef --locked --example retained-performance-fixture > "$TASK_OUTPUT/retained-performance-fixture.json"; then
  run_native_gate retained-performance-checkpoint "$TASK_OUTPUT/native/retained_performance_checkpoint_wire-test" "$TASK_OUTPUT/retained-performance-fixture.json" > "$TASK_OUTPUT/retained-performance-checkpoint-fixture.json" || :
fi

# Independent V/#293 trials use the same compiled native producer. Copy the
# unchanged verifier into its bounded evidence directory before invocation.
if run_native_gate independent-native-m2-producer cargo build -p ql-mef --locked --example m2_engine; then
TASK_INDEPENDENT="$TASK_OUTPUT/independent"
mkdir -p "$TASK_INDEPENDENT"
cp scripts/test-m2-producer-negatives.py "$TASK_INDEPENDENT/driver.py"
v_candidate_revision=$(git rev-parse HEAD)
v_source_revision=$(python3 -c 'import json; print(json.load(open("fixtures/kernel/m2-correspondences-v1.json"))["source_revision"])')
v_trial_nonce=$(python3 -c 'import uuid; print(uuid.uuid4().hex)')
run_native_gate independent-native-m2-trials python3 "$TASK_INDEPENDENT/driver.py" \
  --producer target/debug/examples/m2_engine \
  --fixture fixtures/kernel/m2-condition-request-v1.json \
  --output "$TASK_INDEPENDENT/run-${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-1}-$v_trial_nonce" \
  --candidate-revision "$v_candidate_revision" \
  --source-revision "$v_source_revision" || :
fi

if (( ${#TASK_GATE_FAILURES[@]} )); then
  printf 'Native physical-musical floor failed; all independent available leaves attempted:\n' >&2
  printf '  %s\n' "${TASK_GATE_FAILURES[@]}" >&2
  exit 1
fi
