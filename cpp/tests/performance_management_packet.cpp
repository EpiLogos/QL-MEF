// Controlled inputs exercise actual excitation -> actual P integration.
// They are not authenticated M2 production inputs. The Rust consumer fixture
// separately joins native M1/K/M2/B outputs before creating these packets.
#include "../test_support/allocation_hooks.hpp"
#include <cassert>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <new>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_live_clock.hpp>
#include <ql/performance_management.hpp>
#include <vector>
static std::atomic<bool> count_allocations{false};
static std::atomic<std::uint64_t> callback_allocations{0};
void *operator new(std::size_t size) {
  if (count_allocations.load(std::memory_order_relaxed))
    callback_allocations.fetch_add(1);
  if (void *p = ql_test_allocate(size))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t size) { return ::operator new(size); }
void operator delete(void *p) noexcept { ql_test_release(p); }
void operator delete[](void *p) noexcept { ql_test_release(p); }
void operator delete(void *p, std::size_t) noexcept { ql_test_release(p); }
void operator delete[](void *p, std::size_t) noexcept { ql_test_release(p); }
using namespace ql::performance;
static ql::PhysicalBodyInput physical_input() {
  ql::PhysicalBodyInput in{};
  in.event_ref = "controlled:performance/event";
  in.subject_ref = "controlled:performance/subject";
  in.source_coordinate = "#3-2-1-1-1";
  in.source_revision = "controlled:physical/source-revision";
  in.geometry_ref = "controlled:metric-axial-bar";
  in.geometry_revision = "1";
  in.geometry_source_ref = "controlled:analytic-bar-geometry";
  in.geometry_standing = "reference";
  in.preparation_ref = "controlled:physical/preparation";
  in.state_ref = "controlled:physical/state";
  in.source_generation = 7;
  in.body_revision = 1;
  in.sample_rate = 48000;
  in.pratibimba = true;
  in.family = ql::BodyFamily::AxialTruss;
  in.material = {"controlled:elastic-material",
                 "1",
                 "controlled:analytic-material",
                 "reference",
                 1e6,
                 2,
                 4,
                 0};
  in.nodes = {{1, "#3-2-1-1-1", {0, 0, 0}, 0, {true, true, true}},
              {2, "#3-0", {1, 0, 0}, 0, {false, true, true}}};
  in.edges = {{0, 1, 1e-4, 0}};
  in.exciter = {{1, 0, 0}, {0, 1}};
  in.pickup = in.exciter;
  in.pickup_linear_per_metre = 1000;
  in.max_force_newtons = 10;
  in.max_impulse_newton_seconds = 0.01;
  in.max_displacement_metres = 0.1;
  return in;
}
static Determination determination() {
  Determination d{};
  d.identity.instance = reference("controlled:retained-expression/instance");
  d.identity.event = reference("controlled:performance/event");
  d.identity.subject = reference("controlled:performance/subject");
  d.identity.m1_revision = 11;
  d.identity.m2_generation = 7;
  d.m1_coordinate = reference("#1-5-1");
  d.m1_face = 1;
  d.m2_writer = reference("#2-1");
  d.registry_revision = reference(ql_m_live_base_registry_revision());
  d.source_revision = reference("controlled:native-source-revision");
  d.relation_plan_ref = reference("controlled:relation-plan");
  d.tuning_ref = reference("controlled:explicit-rational-targets");
  d.native_receipt_ref =
      reference("controlled:consumer-test-no-producer-authentication");
  d.body_preparation_ref = reference("controlled:physical/preparation");
  d.body_state_ref = reference("controlled:physical/state");
  d.tick12 = 0;
  d.basis = 0;
  d.lens12 = 0;
  d.context_frame = 1;
  d.degree720 = 0;
  d.audio_octet_hz = {220, 247.5, 264, 293.3333333333333, 330, 352, 396, 412.5};
  d.nodal_quartet = {NodalBoundary{0, 0, 1, 1}, NodalBoundary{0, 1, 1, 2},
                     NodalBoundary{5, 0, 2, 1}, NodalBoundary{5, 1, 2, 2}};
  d.excitation.policy_ref = reference("controlled:D30/root-only-tone-policy");
  d.excitation.standing =
      reference("reference-controlled-not-native-source-authority");
  d.excitation.reference_hertz = 220;
  d.body_revision = 1;
  d.tuning_available = true;
  return d;
}
static NoteTarget target(const Determination &d, std::uint64_t member = 1,
                         std::uint64_t touch = 1, std::uint64_t numerator = 1,
                         std::uint64_t denominator = 1) {
  NoteTarget n{};
  n.identity = d.identity;
  n.source_coordinate = d.m1_coordinate;
  n.tuning_ref = d.tuning_ref;
  n.member = member;
  n.touch = touch;
  n.touch_ref =
      reference(("controlled:physical-touch/" + std::to_string(touch)).c_str());
  n.source_face = d.m1_face;
  n.key = std::uint8_t((member - 1) % 12);
  n.position = n.key / 2;
  n.coordinate_face = n.key % 2;
  n.pitch_class = n.key;
  n.fundamental_hz = 220;
  n.hertz = 220 * double(numerator) / double(denominator);
  n.ratio_numerator = numerator;
  n.ratio_denominator = denominator;
  n.exact_ratio = true;
  return n;
}
static Operation operation(const Determination &d, Kind kind,
                           std::uint64_t sequence, std::uint64_t sample) {
  Operation op{};
  op.identity = d.identity;
  op.kind = kind;
  op.sequence = sequence;
  op.sample = sample;
  return op;
}
static Operation note_on(const Determination &d, std::uint64_t sequence,
                         std::uint64_t sample, NoteTarget n) {
  auto op = operation(d, Kind::NoteOn, sequence, sample);
  op.note = n;
  op.value = 0.8;
  return op;
}
struct Fixture {
  std::shared_ptr<ql::PhysicalBody> body;
  std::shared_ptr<Engine> engine;
  Determination d = determination();
  explicit Fixture(Parameters p = {}) {
    body = std::make_shared<ql::PhysicalBody>(
        ql::PreparedPhysicalBody(physical_input()));
    engine = std::make_shared<Engine>(d, 48000, physical_port(body), p);
    engine->enable_capture(true);
  }
};

static bool render_actual(Engine &engine, float *output, std::size_t frames) {
  count_allocations.store(true);
  const bool result = engine.render(output, frames, engine.samples_elapsed());
  count_allocations.store(false);
  return result;
}
static void committed_receipts_and_no_false_play() {
  Fixture f;
  auto attack = note_on(f.d, 1, 48, target(f.d));
  assert(f.engine->enqueue(attack) == Result::Accepted);
  NativeGestureApplication a{};
  assert(!f.engine->pop_gesture_application(a));
  std::array<float, 128> output{};
  assert(render_actual(*f.engine, output.data(), 128));
  assert(f.engine->pop_gesture_application(a) && a.applied && a.has_note &&
         a.kind == Kind::NoteOn);
  assert(a.note.touch_ref == attack.note.touch_ref &&
         a.note.ratio_numerator == 1 && a.note.ratio_denominator == 1);
  assert(a.sequence == 1 && a.admitted_sample == 48 && a.applied_sample == 48 &&
         a.committed_cursor == 128);
  assert(a.body_revision == 1 &&
         a.preparation_ref == f.d.body_preparation_ref &&
         a.state_ref == f.d.body_state_ref);
  auto wire = checkpoint_transport::application(a);
  auto round = checkpoint_transport::read_application(wire.get());
  assert(round.note.touch_ref == a.note.touch_ref &&
         round.applied_sample == 48 && round.committed_cursor == 128);
  // Real P displacement-bound refusal: A has processed the note internally,
  // but there is no committed P state or output, hence no played receipt.
  auto in = physical_input();
  in.max_displacement_metres = 1e-12;
  auto body = std::make_shared<ql::PhysicalBody>(ql::PreparedPhysicalBody(in));
  auto engine =
      std::make_shared<Engine>(f.d, 48000, physical_port(body), Parameters{});
  assert(engine->enqueue(note_on(f.d, 1, 0, target(f.d))) == Result::Accepted);
  const auto q = body->checkpoint();
  output.fill(1);
  assert(!render_actual(*engine, output.data(), 128));
  assert(body->samples_elapsed() == 0 &&
         body->checkpoint().displacement_modal_metres ==
             q.displacement_modal_metres);
  assert(!engine->pop_gesture_application(a) && !engine->available());
  assert(engine->recording_status().failure ==
         RecordingFailure::PhysicalCommitFailure);
  assert(engine->recording_status().first_failed_sequence == 1);
  for (float x : output)
    assert(x == 0);
  // A reused touch/panic-fenced note must not be called played.
  Fixture duplicate;
  assert(duplicate.engine->enqueue(note_on(
             duplicate.d, 1, 0, target(duplicate.d))) == Result::Accepted);
  assert(duplicate.engine->enqueue(note_on(
             duplicate.d, 2, 0, target(duplicate.d))) == Result::Accepted);
  assert(render_actual(*duplicate.engine, output.data(), 128));
  assert(duplicate.engine->pop_gesture_application(a) && a.applied);
  assert(duplicate.engine->pop_gesture_application(a) && !a.applied &&
         a.sequence == 2);
}
static void loss_is_recording_failure_and_queue_checkpoint_is_exact() {
  Fixture f;
  for (std::uint64_t i = 1; i <= 256; ++i) {
    auto op = operation(f.d, Kind::Parameter, i, 0);
    op.parameter = Parameter::MasterLinear;
    op.value = 0.2;
    assert(f.engine->enqueue(op) == Result::Accepted);
  }
  std::array<float, 128> output{};
  assert(render_actual(*f.engine, output.data(), 128));
  for (std::uint64_t i = 257; i <= 258; ++i) {
    auto op = operation(f.d, Kind::Parameter, i, 128);
    op.parameter = Parameter::MasterLinear;
    op.value = 0.3;
    assert(f.engine->enqueue(op) == Result::Accepted);
  }
  assert(render_actual(*f.engine, output.data(), 128));
  auto status = f.engine->recording_status();
  assert(status.failure == RecordingFailure::ApplicationQueueOverflow);
  assert(status.dropped_applications == 2 &&
         status.first_failed_sequence == 257 &&
         status.first_failed_sample == 128);
  assert(
      f.engine
          ->available()); // Continue safe audio; recording explicitly failed.
  auto guard = f.engine->acquire_stopped_custody();
  assert(guard);
  auto saved = checkpoint_heap(*f.engine, *f.body, guard);
  assert(saved->audio.applications.write - saved->audio.applications.read ==
         256);
  auto wire = checkpoint_transport::checkpoint_wire(*saved);
  auto decoded = checkpoint_transport::read_checkpoint_wire(wire.get());
  assert(decoded->audio.recording.dropped_applications == 2);
  assert(restore_checkpoint(*f.engine, *f.body, *decoded, guard, 256));
  std::uint64_t count = 0;
  NativeGestureApplication receipt{};
  while (f.engine->pop_gesture_application(receipt)) {
    ++count;
    assert(receipt.sequence == count && receipt.committed_cursor == 128 &&
           receipt.applied);
  }
  assert(count == 256 && f.engine->recording_status().failure ==
                             RecordingFailure::ApplicationQueueOverflow);
  decoded->audio.recording.failure = RecordingFailure::None;
  const auto before = f.body->checkpoint();
  assert(!restore_checkpoint(*f.engine, *f.body, *decoded, guard, 256));
  assert(f.body->checkpoint().displacement_modal_metres ==
             before.displacement_modal_metres &&
         f.body->samples_elapsed() == 256);
}
static void physical_release_proof_requires_actual_force_zero() {
  Parameters p{};
  p.master_linear = 0; // Real muted output still has force.
  Fixture f(p);
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d))) ==
         Result::Accepted);
  std::array<float, 512> output{};
  assert(render_actual(*f.engine, output.data(), output.size()));
  Readback reading{};
  assert(f.engine->pop_readback(reading));
  for (float x : output)
    assert(x == 0);
  assert(reading.active_touches == 1 && reading.active_voices == 1);
  assert(!release_zero_proven(reading, 1));
  const auto release_request = f.engine->request_panic();
  assert(release_request == 1);
  // Request/enqueue alone supplies no physical release proof.
  assert(!release_zero_proven(reading, release_request));
  bool proven = false;
  for (unsigned i = 0; i < 16; ++i) {
    assert(render_actual(*f.engine, output.data(), output.size()));
    assert(f.engine->pop_readback(reading));
    Capture capture{};
    while (f.engine->pop_capture(capture)) {
    }
    if (release_zero_proven(reading, release_request)) {
      proven = true;
      break;
    }
  }
  assert(proven && reading.emergency_observed == release_request &&
         reading.force_zero_samples >= 512 && !reading.active_touches &&
         !reading.active_voices && !reading.active_tails);
  // P ringdown belongs to the sole P owner; excitation-zero is not q/v-zero.
  assert(f.body->observation().mechanical_energy_joules > 0);
  const auto guard = f.engine->acquire_stopped_custody();
  auto saved = checkpoint_heap(*f.engine, *f.body, guard);
  auto wire = checkpoint_transport::checkpoint_wire(*saved);
  auto decoded = checkpoint_transport::read_checkpoint_wire(wire.get());
  assert(decoded->audio.force_zero_samples == reading.force_zero_samples &&
         decoded->audio.emergency_observed == release_request);
}
static void full_native_determination_receipt_and_original_touch() {
  Fixture f;
  assert(f.engine->enqueue(note_on(f.d, 1, 0, target(f.d))) ==
         Result::Accepted);
  std::array<float, 128> output{};
  assert(render_actual(*f.engine, output.data(), output.size()));
  auto after = f.d;
  ++after.identity.m2_generation;
  after.native_receipt_ref =
      reference("controlled:actual-source-update/receipt");
  after.audio_octet_hz[1] = 256;
  auto update = operation(f.d, Kind::Determination, 2, 128);
  update.determination = after;
  assert(f.engine->enqueue(update) == Result::Accepted);
  assert(render_actual(*f.engine, output.data(), output.size()));
  NativeGestureApplication a{};
  assert(f.engine->pop_gesture_application(a) && a.kind == Kind::NoteOn);
  assert(f.engine->pop_gesture_application(a) &&
         a.kind == Kind::Determination && a.applied);
  assert(a.has_determination && a.identity == f.d.identity &&
         a.determined_source.identity == after.identity &&
         a.determined_source.audio_octet_hz[1] == 256);
  auto wire = checkpoint_transport::application(a);
  auto decoded = checkpoint_transport::read_application(wire.get());
  assert(decoded.determined_source.native_receipt_ref ==
         after.native_receipt_ref);
  auto release = operation(f.d, Kind::NoteOff, 3, 256);
  release.touch = 1;
  assert(f.engine->enqueue(release) == Result::Accepted);
  assert(render_actual(*f.engine, output.data(), output.size()));
  assert(f.engine->pop_gesture_application(a) && a.has_note && a.applied);
  assert(a.note.identity == f.d.identity &&
         a.note.touch_ref == target(f.d).touch_ref);
}
static std::string read_file(const std::string &path) {
  std::ifstream stream(path, std::ios::binary);
  if (!stream)
    throw std::invalid_argument("missing native producer fixture");
  stream.seekg(0, std::ios::end);
  const auto size = stream.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("fixture bound");
  std::string out(std::size_t(size), '\0');
  stream.seekg(0);
  if (!stream.read(out.data(), size))
    throw std::invalid_argument("incomplete fixture");
  return out;
}
static ql::physical_wire::Json parse_file(const std::string &path) {
  auto text = read_file(path);
  auto *tok = json_tokener_new_ex(64);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(tok, text.data(), int(text.size())));
  const auto error = json_tokener_get_error(tok);
  const auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  if (error != json_tokener_success || !out ||
      text.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("strict actual fixture required");
  return out;
}
static void actual_native_producer_application(const std::string &directory) {
  auto basis = parse_file(directory + "/baseline.basis.json");
  auto native = prepare_performance_packet(
      read_file(directory + "/baseline.packet.json"), basis.get(), true, true);
  assert(native.notes.size() == 12);
  auto op = note_on(native.determination, 1, 37, native.notes[4]);
  native.engine->enable_capture(true);
  assert(native.engine->enqueue(op) == Result::Accepted);
  std::array<float, 128> output{};
  assert(render_actual(*native.engine, output.data(), 128));
  NativeInputBindings bindings;
  const auto input_ref = reference("controlled:original-UI-pointer/input-42");
  assert(input_ref != native.notes[4].touch_ref);
  assert(bindings.bind(input_ref, native.notes[4], 1));
  Capture c{};
  assert(native.engine->pop_capture(c));
  assert(std::any_of(c.force_newtons.begin(), c.force_newtons.begin() + 128,
                     [](double x) { return x != 0; }));
  assert(std::any_of(c.pickup_linear.begin(), c.pickup_linear.begin() + 128,
                     [](float x) { return x != 0; }));
  assert(native.body->samples_elapsed() == 128);
  // Genuine P material transition while the original application remains
  // unread. Both numerical owners preflight, q/v projects into the new basis,
  // and the source event/state/cursor persists. This is an explicit test-owned
  // material policy over the actual producer body, not M2 source authority.
  auto before_preparation = native.body->preparation();
  auto after_input = before_preparation.input();
  after_input.body_revision += 1;
  after_input.source_generation += 1;
  after_input.preparation_ref += "/material-test-successor";
  after_input.material.revision += "/material-test-successor";
  after_input.material.young_modulus_pa *= 1.5;
  ql::PreparedPhysicalBody after_preparation(after_input);
  ql::PreparedPhysicalTransition transition(
      before_preparation, after_preparation, 42, 128,
      ql::PhysicalLiveUpdateKind::Material,
      ql::PhysicalFormTransition::ProjectCorrespondingNodes,
      "reference:real-native-packet-material-regression");
  auto after_determination = native.determination;
  after_determination.body_revision = after_input.body_revision;
  after_determination.body_preparation_ref =
      reference(after_input.preparation_ref.c_str());
  {
    auto guard = native.engine->acquire_stopped_custody();
    assert(guard);
    ql::PhysicalLiveTransitionReceipt receipt{};
    assert(apply_stopped_physical_transition(*native.engine, native.body,
                                             transition, after_determination,
                                             guard, receipt));
    assert(receipt.samples_elapsed == 128 &&
           receipt.before_revision != receipt.after_revision);
  }
  // Queue an exact old-touch release, then save BEFORE delivery. Original
  // input_ref, member, native key target and pending release survive reopen.
  auto release = operation(native.determination, Kind::NoteOff, 2, 256);
  release.touch = native.notes[4].touch;
  assert(native.engine->enqueue(release) == Result::Accepted);
  assert(bindings.release_admitted(input_ref, 2));
  auto saved = std::make_unique<ManagementCheckpoint>();
  {
    auto guard = native.engine->acquire_stopped_custody();
    assert(guard);
    native.engine->write_checkpoint(saved->native_pair.audio, guard);
    saved->native_pair.physical = native.body->checkpoint();
  }
  bindings.write_checkpoint(saved->bindings);
  saved->session = reference("controlled:retained-native-performance/session");
  saved->transport_epoch = 1;
  assert(saved->native_pair.audio.applications.write -
             saved->native_pair.audio.applications.read ==
         1);
  const auto &historical = saved->native_pair.audio.applications.storage[0];
  assert(historical.body_revision !=
         saved->native_pair.audio.determination.body_revision);
  assert(historical.preparation_ref !=
         saved->native_pair.audio.determination.body_preparation_ref);
  assert(historical.eigenbasis !=
         reference(after_preparation.eigenbasis_identity().c_str()));
  assert(historical.physical_source_generation + 1 ==
         after_input.source_generation);
  assert(historical.identity == native.determination.identity &&
         historical.applied);
  // Those comparisons discriminate the original frozen implementation's
  // current-body equality fault. Historical receipt stays ORIGINAL, not
  // relabelled.
  // The historical receipt must retain an actual valid original note, not
  // merely a matching label/identity. A corrupt old Hz/ratio refuses without
  // changing the resident body or the complete frozen source test.
  {
    auto guard = native.engine->acquire_stopped_custody();
    assert(guard);
    auto &old = saved->native_pair.audio.applications.storage[0].note;
    const auto original_hertz = old.hertz;
    old.hertz += 1;
    assert(!native.engine->validate_checkpoint(saved->native_pair.audio, guard, 128));
    assert(native.body->samples_elapsed() == 128);
    old.hertz = original_hertz;
    assert(native.engine->validate_checkpoint(saved->native_pair.audio, guard, 128));
  }
  auto management_wire =
      management_checkpoint_transport::checkpoint_wire(*saved);
  auto decoded = management_checkpoint_transport::read_checkpoint_wire(
      management_wire.get());
  assert(decoded->bindings.inputs[0].input_ref == input_ref &&
         decoded->bindings.inputs[0].target.touch_ref ==
             native.notes[4].touch_ref &&
         decoded->bindings.inputs[0].target.member == native.notes[4].member &&
         decoded->bindings.inputs[0].release_pending &&
         decoded->bindings.inputs[0].release_sequence == 2);
  auto reopened_body = std::make_shared<ql::PhysicalBody>(after_preparation);
  auto reopened_engine = std::make_shared<Engine>(after_determination, 48000,
                                                  physical_port(reopened_body));
  {
    auto guard = reopened_engine->acquire_stopped_custody();
    assert(guard);
    assert(restore_checkpoint(*reopened_engine, *reopened_body,
                              decoded->native_pair, guard, 0));
  }
  NativeInputBindings reopened_bindings;
  reopened_bindings.restore_validated(decoded->bindings);
  NativeGestureApplication a{};
  assert(reopened_engine->pop_gesture_application(a) && a.applied &&
         a.has_note && a.applied_sample == 37 && a.committed_cursor == 128);
  assert(a.body_revision == native.determination.body_revision &&
         a.preparation_ref == native.determination.body_preparation_ref &&
         a.physical_source_generation + 1 == after_input.source_generation);
  assert(reopened_bindings.find(input_ref) &&
         !reopened_bindings.find(native.notes[4].touch_ref));
  assert(reopened_bindings.application(a) &&
         reopened_bindings.find(input_ref)->press_applied);
  assert(render_actual(*native.engine, output.data(), 128));
  std::array<float, 128> reopened_output{};
  assert(render_actual(*reopened_engine, reopened_output.data(), 128));
  assert(output == reopened_output);
  assert(render_actual(*native.engine, output.data(), 128));
  assert(render_actual(*reopened_engine, reopened_output.data(), 128));
  assert(output == reopened_output);
  assert(reopened_engine->pop_gesture_application(a) &&
         a.kind == Kind::NoteOff && a.applied);
  assert(reopened_bindings.application(a) &&
         !reopened_bindings.find(input_ref));
  InputBindingRecord entry{};
  assert(reopened_bindings.pop_history(entry) && entry.input_ref == input_ref &&
         entry.change == InputBindingChange::PressAdmitted &&
         entry.target.touch_ref == native.notes[4].touch_ref);
  assert(reopened_bindings.pop_history(entry) && entry.input_ref == input_ref &&
         entry.change == InputBindingChange::ReleaseAdmitted);
  assert(reopened_bindings.pop_history(entry) && entry.input_ref == input_ref &&
         entry.change == InputBindingChange::Applied &&
         entry.operation == Kind::NoteOn);
  assert(reopened_bindings.pop_history(entry) && entry.input_ref == input_ref &&
         entry.change == InputBindingChange::Applied &&
         entry.operation == Kind::NoteOff);
  assert(!reopened_bindings.pop_history(entry));
  // Native export uses the same actual A/P callback, source, cursor and PCM.
  NativeOfflineRenderScope export_scope{};
  export_scope.session = saved->session;
  export_scope.scene = reference("reference:actual-packet-export/scene");
  export_scope.performance_revision =
      reference("reference:export-performance-revision");
  export_scope.basis_seal =
      reference("reference:export-basis-seal-control-test");
  export_scope.event_prefix_seal =
      reference("reference:export-prefix-seal-control-test");
  export_scope.checkpoint_ref = reference("reference:actual-packet-checkpoint");
  export_scope.expected_source = after_determination.identity;
  export_scope.expected_body_revision = after_input.body_revision;
  export_scope.expected_cursor = 384;
  export_scope.expected_accepted_sequence = 2;
  std::array<float, 129> chunk_pcm{}, continuation_pcm{};
  auto chunk =
      render_native_offline_chunk(*native.engine, native.body, export_scope,
                                  chunk_pcm.data(), chunk_pcm.size());
  assert(chunk->result == Result::Accepted && chunk->capture_complete &&
         chunk->state_committed);
  assert(chunk->start_sample == 384 && chunk->committed_cursor == 513 &&
         chunk->reading.physical.samples_elapsed == 513);
  assert(render_actual(*reopened_engine, continuation_pcm.data(),
                       continuation_pcm.size()));
  assert(chunk_pcm == continuation_pcm);
  for (std::size_t i = 0; i < chunk_pcm.size(); ++i)
    assert(chunk_pcm[i] == chunk->capture.output_linear[i]);
  export_scope.expected_cursor = 513;
  assert(
      native.engine->begin_device_callbacks()); // Actual Engine custody flag,
                                                // not an OS device claim.
  auto running_refusal =
      render_native_offline_chunk(*native.engine, native.body, export_scope,
                                  chunk_pcm.data(), chunk_pcm.size());
  assert(running_refusal->result == Result::Unavailable &&
         !running_refusal->state_committed);
  native.engine->end_device_callbacks_after_stop();
  ++export_scope.expected_source.m2_generation;
  auto stale_refusal =
      render_native_offline_chunk(*native.engine, native.body, export_scope,
                                  chunk_pcm.data(), chunk_pcm.size());
  assert(stale_refusal->result == Result::Stale &&
         native.engine->samples_elapsed() == 513);
  decoded->bindings.inputs[0].target.hertz += 1;
  assert(!NativeInputBindings::validate(decoded->bindings,
                                        decoded->native_pair.audio));
  // Wrong original source/event manifest cannot be promoted to current
  // standing.
  decoded->native_pair.audio.applications.storage[0].physical_event =
      reference("wrong:original-event");
  {
    auto guard = native.engine->acquire_stopped_custody();
    assert(guard);
    assert(!native.engine->validate_checkpoint(decoded->native_pair.audio,
                                               guard, 513));
  }
  auto wire = checkpoint_transport::application(a);
  std::cout << json_object_to_json_string_ext(wire.get(),
                                              JSON_C_TO_STRING_PLAIN)
            << '\n';
}
int main(int argc, char **argv) {
  if (argc != 2) {
    std::cerr << "usage: performance_management_packet-test "
                 "ACTUAL_NATIVE_PRODUCER_FIXTURE_DIRECTORY\n";
    return 2;
  }
  try {
    committed_receipts_and_no_false_play();
    loss_is_recording_failure_and_queue_checkpoint_is_exact();
    physical_release_proof_requires_actual_force_zero();
    full_native_determination_receipt_and_original_touch();
    actual_native_producer_application(argv[1]);
    assert(callback_allocations.load() == 0);
    std::cout << "{\"schema\":\"ql.performance-component-resource/"
                 "v1\",\"sizeof_engine\":"
              << sizeof(Engine) << ",\"sizeof_operation\":" << sizeof(Operation)
              << ",\"sizeof_readback\":" << sizeof(Readback)
              << ",\"sizeof_checkpoint\":" << sizeof(Engine::Checkpoint)
              << ",\"sizeof_paired_checkpoint\":" << sizeof(PairedCheckpoint)
              << ",\"sizeof_application\":" << sizeof(NativeGestureApplication)
              << ",\"sizeof_management_pulse\":" << sizeof(ManagementPulse)
              << ",\"sizeof_management_checkpoint\":"
              << sizeof(ManagementCheckpoint)
              << ",\"sizeof_input_bindings\":" << sizeof(NativeInputBindings)
              << ",\"callback_allocations\":" << callback_allocations.load()
              << "}\n";
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
