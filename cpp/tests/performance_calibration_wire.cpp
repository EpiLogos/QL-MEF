// Actual source-form native packet -> existing Manager/Engine/P. Numerical
// calibration is a candidate instrument configuration, not hardware audibility.
#include "../test_support/allocation_hooks.hpp"
#include <algorithm>
#include <array>
#include <atomic>
#include <cassert>
#include <cmath>
#include <cstring>
#include <iostream>
#include <map>
#include <new>
#include <ql/performance_management_wire.hpp>
#include <vector>
static std::atomic<bool> callback{false};
static std::atomic<std::uint64_t> allocations{0}, frees{0};
void *operator new(std::size_t n) {
  if (callback.load(std::memory_order_relaxed))
    ++allocations;
  if (void *p = ql_test_allocate(n))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n) { return ::operator new(n); }
void *operator new(std::size_t n, std::align_val_t a) {
  if (callback.load(std::memory_order_relaxed))
    ++allocations;
  if (void *p = ql_test_allocate_aligned(n, std::size_t(a)))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n, std::align_val_t a) {
  return ::operator new(n, a);
}
void operator delete(void *p) noexcept {
  if (p && callback.load(std::memory_order_relaxed))
    ++frees;
  ql_test_release(p);
}
void operator delete[](void *p) noexcept { ::operator delete(p); }
void operator delete(void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete(void *p, std::align_val_t) noexcept {
  ::operator delete(p);
}
void operator delete[](void *p, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
void operator delete(void *p, std::size_t, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
void operator delete[](void *p, std::size_t, std::align_val_t a) noexcept {
  ::operator delete(p, a);
}
using namespace ql;
using namespace ql::performance;
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
constexpr std::uint64_t rate = 48000, trial_samples = 2 * rate;
static Json floats(const std::vector<float> &values) {
  auto out = wire::array();
  for (float x : values)
    wire::append(out.get(), json_object_new_double(x));
  return out;
}
static Json doubles(const std::vector<double> &values) {
  auto out = wire::array();
  for (double x : values)
    wire::append(out.get(), json_object_new_double(x));
  return out;
}
static Operation operation(const PerformanceManagement &owner, Kind kind,
                           std::uint64_t sample) {
  Operation out{};
  out.kind = kind;
  out.sample = sample;
  out.sequence = owner.native().engine->accepted_sequence() + 1;
  out.identity = owner.native().determination.identity;
  return out;
}
static void enqueue(PerformanceManagement &owner, Operation op,
                    Ref input = {}) {
  const auto admission = owner.enqueue_score_input_admission(op, input);
  assert(admission.result() == Result::Accepted && admission.queue().queued());
  assert(admission.queue().operation().has_requested_sample &&
         admission.queue().operation().requested_sample == op.sample &&
         admission.queue().operation().sample == op.sample);
}
static std::unique_ptr<PerformanceManagement> manager(J *source) {
  J *producer = packet::field(source, "native_preparation");
  auto native = prepare_source_performance_packet(
      json_object_to_json_string_ext(producer, JSON_C_TO_STRING_PLAIN),
      producer, packet::field(source, "native_basis"), true, true);
  assert(native.body->preparation().input().nodes.size() == 12 &&
         native.body->preparation().input().edges.size() == 34);
  native.engine->enable_capture(true);
  return std::make_unique<PerformanceManagement>(
      std::move(native), reference("calibration:actual-retained-session"));
}
struct Observation {
  std::vector<float> pcm, pickup;
  std::vector<NativeGestureApplication> applications;
  std::vector<InputBindingRecord> journal;
  double peak = 0, pickup_peak = 0, force_peak = 0, square_sum = 0,
         max_displacement_metres = 0;
  std::uint64_t samples = 0, clipping = 0, limited = 0;
  std::uint32_t voices = 0, tails = 0;
};
static void advance(PerformanceManagement &owner, Observation &observed,
                    std::size_t frames) {
  assert(frames && frames <= 512);
  const auto start = owner.native().engine->samples_elapsed();
  std::array<float, 512> output{};
  callback.store(true, std::memory_order_relaxed);
  const bool accepted = owner.offline_advance(output.data(), frames, start);
  callback.store(false, std::memory_order_relaxed);
  if (!accepted)
    throw std::runtime_error("actual source-form calibration callback refused");
  Capture captured{};
  assert(owner.pop_audio_capture(captured));
  assert(captured.start_sample == start && captured.frames == frames &&
         !captured.has_receiving && captured.route_count == 0);
  auto pulse = owner.pulse();
  assert(pulse->has_readback &&
         pulse->reading.samples_elapsed == start + frames &&
         pulse->reading.physical.samples_elapsed == start + frames);
  assert(pulse->recording.failure == RecordingFailure::None &&
         pulse->recording.dropped_applications == 0 &&
         owner.recording_available());
  assert(pulse->reading.dropped_captures == 0 &&
         pulse->reading.dropped_readbacks == 0);
  const auto &physical = pulse->reading.physical;
  const auto &nodes = owner.native().body->preparation().input().nodes;
  assert(physical.node_count == nodes.size());
  for (std::size_t n = 0; n < nodes.size(); ++n) {
    assert(physical.node_identity[n] == nodes[n].identity);
    for (unsigned axis = 0; axis < 3; ++axis) {
      const double displacement = physical.visible_positions_metres[n][axis] -
                                  nodes[n].rest_metres[axis];
      assert(std::isfinite(displacement));
      observed.max_displacement_metres =
          std::max(observed.max_displacement_metres, std::abs(displacement));
    }
  }
  for (std::size_t i = 0; i < frames; ++i) {
    assert(std::memcmp(&output[i], &captured.output_linear[i], sizeof(float)) ==
           0);
    assert(std::isfinite(output[i]) &&
           std::isfinite(captured.pickup_linear[i]) &&
           std::isfinite(captured.force_newtons[i]));
    assert(std::abs(captured.force_newtons[i]) <= 10);
    observed.peak = std::max(observed.peak, std::abs(double(output[i])));
    observed.pickup_peak = std::max(
        observed.pickup_peak, std::abs(double(captured.pickup_linear[i])));
    observed.force_peak =
        std::max(observed.force_peak, std::abs(captured.force_newtons[i]));
    observed.square_sum += double(output[i]) * output[i];
    observed.pcm.push_back(output[i]);
    observed.pickup.push_back(captured.pickup_linear[i]);
  }
  observed.samples += frames;
  observed.clipping = pulse->reading.clipping_samples;
  observed.limited = pulse->reading.force_limited_samples;
  observed.voices = std::max(observed.voices, pulse->reading.active_voices);
  observed.tails = std::max(observed.tails, pulse->reading.active_tails);
  observed.applications.insert(observed.applications.end(),
                               pulse->applications.begin(),
                               pulse->applications.end());
  observed.journal.insert(observed.journal.end(), pulse->input_history.begin(),
                          pulse->input_history.end());
}
static void through(PerformanceManagement &owner, Observation &observed,
                    std::uint64_t end, std::size_t partition) {
  while (owner.native().engine->samples_elapsed() < end)
    advance(owner, observed,
            std::size_t(std::min<std::uint64_t>(
                partition, end - owner.native().engine->samples_elapsed())));
}
static Json observation(const Observation &o, bool include_samples) {
  assert(o.samples && o.clipping == 0 && o.max_displacement_metres < .01);
  auto out = wire::object();
  wire::u64(out.get(), "samples", o.samples);
  wire::real(out.get(), "peak", o.peak);
  wire::real(out.get(), "rms", std::sqrt(o.square_sum / o.samples));
  wire::real(out.get(), "pickup_peak", o.pickup_peak);
  wire::real(out.get(), "max_force_newtons", o.force_peak);
  wire::real(out.get(), "maximum_displacement_metres",
             o.max_displacement_metres);
  wire::u64(out.get(), "clipping_samples", o.clipping);
  wire::u64(out.get(), "force_limited_samples", o.limited);
  wire::u64(out.get(), "maximum_active_voices", o.voices);
  wire::u64(out.get(), "maximum_active_tails", o.tails);
  auto applications = wire::array(), journal = wire::array();
  for (const auto &a : o.applications)
    wire::append(applications.get(), wire::application(a).release());
  for (const auto &j : o.journal)
    wire::append(journal.get(), management_transport::history(j).release());
  wire::put(out.get(), "applications", applications.release());
  wire::put(out.get(), "original_input_history", journal.release());
  if (include_samples) {
    wire::put(out.get(), "pcm", floats(o.pcm).release());
    wire::put(out.get(), "pickup", floats(o.pickup).release());
  }
  return out;
}
static Json calibration_trial(J *source, double force, bool stress) {
  auto owner = manager(source);
  const auto immutable = owner->native().body->preparation();
  assert(immutable.input().max_force_newtons == 10 &&
         immutable.input().max_displacement_metres == .01);
  auto parameter = operation(*owner, Kind::Parameter, 0);
  parameter.parameter = Parameter::ForceNewtons;
  parameter.value = force;
  enqueue(*owner, parameter);
  const Ref input = reference("calibration:actual-source-key-touch");
  auto note = operation(*owner, Kind::NoteOn, 0);
  note.note = owner->native().notes.at(0);
  note.value = .8;
  enqueue(*owner, note, input);
  Observation trial;
  through(*owner, trial, trial_samples, 128);
  assert(trial.applications.size() == 2 &&
         trial.applications[0].kind == Kind::Parameter &&
         trial.applications[0].parameter == Parameter::ForceNewtons &&
         trial.applications[0].value == force &&
         trial.applications[1].kind == Kind::NoteOn);
  for (std::size_t i = 0; i < 2; ++i) {
    const auto &a = trial.applications[i];
    assert(a.applied && a.applied_application_ordinal == i + 1 &&
           a.sequence == i + 1 && a.has_requested_sample &&
           a.requested_sample == 0 && a.admitted_sample == 0 &&
           a.applied_sample == 0);
  }
  assert(owner->last_readback().source.force_newtons == force &&
         std::abs(owner->last_readback().effective.force_newtons - force) <
             1e-12);
  assert(trial.clipping == 0 && trial.limited == 0 && trial.peak > 0 &&
         trial.force_peak > 0);
  const auto actual_physical = owner->native().body->checkpoint();
  const auto physical_snapshot = owner->last_readback().physical;
  auto future = operation(*owner, Kind::Parameter, trial_samples + 1000);
  future.parameter = Parameter::MasterLinear;
  future.value = .2;
  enqueue(*owner, future);
  auto release = operation(*owner, Kind::NoteOff, trial_samples + 4000);
  release.touch = note.note.touch;
  enqueue(*owner, release, input);
  // Real pending operations, source, physical q/v, oscillator/envelope phase
  // and original input journal are persisted by the existing native CP codec.
  auto saved = owner->stopped_checkpoint();
  auto saved_wire = management_checkpoint_transport::checkpoint_wire(*saved);
  auto transported =
      management_checkpoint_transport::read_checkpoint_wire(saved_wire.get());
  Observation first;
  through(*owner, first, trial_samples + 1024, 128);
  TransportAcknowledgement ack{};
  assert(owner->stopped_restore(*transported, trial_samples + 1024,
                                reference("calibration:actual-restore"),
                                reference("calibration:original-checkpoint"),
                                ack));
  assert(ack.previous_cursor == trial_samples + 1024 &&
         ack.target_sample == trial_samples && ack.previous_epoch == 1 &&
         ack.epoch == 2);
  Observation second;
  through(*owner, second, trial_samples + 1024, 512);
  assert(first.pcm.size() == 1024 && second.pcm.size() == 1024 &&
         std::memcmp(first.pcm.data(), second.pcm.data(),
                     1024 * sizeof(float)) == 0);
  assert(first.applications.size() == 1 && second.applications.size() == 1 &&
         first.applications[0].kind == Kind::Parameter &&
         first.applications[0].sequence == future.sequence &&
         first.applications[0].applied_application_ordinal == 3 &&
         first.applications[0].admitted_sample == trial_samples + 1000 &&
         first.applications[0].applied_sample == trial_samples + 1000 &&
         second.applications[0].sequence == future.sequence &&
         second.applications[0].applied_application_ordinal == 3);
  // Keep the original 1024 continuation proof, then cross the genuine queued
  // release. A dropped NoteOff cannot pass exact PCM or applied history here.
  Observation release_first;
  through(*owner, release_first, trial_samples + 5024, 128);
  assert(release_first.applications.size() == 1);
  const auto &released = release_first.applications.front();
  assert(released.applied && released.kind == Kind::NoteOff &&
         released.sequence == release.sequence &&
         released.applied_application_ordinal == 4 &&
         released.has_requested_sample &&
         released.requested_sample == trial_samples + 4000 &&
         released.admitted_sample == trial_samples + 4000 &&
         released.applied_sample == trial_samples + 4000 &&
         released.touch == note.note.touch);
  const auto expected_body = owner->native().body->checkpoint();
  TransportAcknowledgement release_ack{};
  assert(owner->stopped_restore(*transported, trial_samples + 5024,
                                reference("calibration:actual-release-restore"),
                                reference("calibration:original-checkpoint"),
                                release_ack));
  assert(release_ack.previous_cursor == trial_samples + 5024 &&
         release_ack.target_sample == trial_samples &&
         release_ack.previous_epoch == 2 && release_ack.epoch == 3);
  auto restored_pending = owner->stopped_checkpoint();
  auto restored_wire =
      management_checkpoint_transport::checkpoint_wire(*restored_pending);
  auto *saved_audio =
      packet::field(packet::field(saved_wire.get(), "native_pair"), "audio");
  auto *restored_audio =
      packet::field(packet::field(restored_wire.get(), "native_pair"), "audio");
  for (const char *name :
       {"operations", "releases", "pending_operations", "pending_releases",
        "cursor", "accepted_sequence", "accepted_sample", "panic_fence"})
    assert(json_object_equal(packet::field(saved_audio, name),
                             packet::field(restored_audio, name)));
  Observation release_second;
  through(*owner, release_second, trial_samples + 5024, 512);
  assert(release_second.pcm.size() == 5024 && release_first.pcm.size() == 4000);
  assert(std::memcmp(first.pcm.data(), release_second.pcm.data(),
                     1024 * sizeof(float)) == 0);
  assert(std::memcmp(release_first.pcm.data(), release_second.pcm.data() + 1024,
                     4000 * sizeof(float)) == 0);
  assert(
      release_second.applications.size() == 2 &&
      release_second.applications[0].kind == Kind::Parameter &&
      release_second.applications[0].sequence == future.sequence &&
      release_second.applications[0].applied_application_ordinal == 3 &&
      release_second.applications[1].kind == Kind::NoteOff &&
      release_second.applications[1].sequence == release.sequence &&
      release_second.applications[1].applied_application_ordinal == 4 &&
      release_second.applications[1].requested_sample == trial_samples + 4000 &&
      release_second.applications[1].admitted_sample == trial_samples + 4000 &&
      release_second.applications[1].applied_sample == trial_samples + 4000);
  const auto actual_body = owner->native().body->checkpoint();
  assert(actual_body.samples_elapsed == expected_body.samples_elapsed &&
         actual_body.displacement_modal_metres ==
             expected_body.displacement_modal_metres &&
         actual_body.velocity_modal_metres_per_second ==
             expected_body.velocity_modal_metres_per_second);
  assert(owner->native().body->preparation().input().pickup_linear_per_metre ==
         immutable.input().pickup_linear_per_metre);
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-calibration-receipt/v1");
  wire::real(out.get(), "pickup_linear_per_metre",
             immutable.input().pickup_linear_per_metre);
  wire::real(out.get(), "applied_force_newtons", force);
  wire::put(out.get(), "trial", observation(trial, true).release());
  auto spectrum = wire::array();
  for (std::size_t i = 0; i < immutable.mode_count(); ++i)
    wire::append(spectrum.get(),
                 json_object_new_double(immutable.frequency_hz(i)));
  wire::put(out.get(), "eigenfrequencies_hz", spectrum.release());
  wire::put(out.get(), "physical_q_metres",
            doubles(actual_physical.displacement_modal_metres).release());
  wire::put(
      out.get(), "physical_v_metres_per_second",
      doubles(actual_physical.velocity_modal_metres_per_second).release());
  auto physical = management_transport::physical(physical_snapshot);
  wire::put(out.get(), "physical_positions_metres",
            json_object_get(packet::field(physical.get(), "positions_metres")));
  wire::put(out.get(), "physical", physical.release());
  wire::put(out.get(), "saved_management_checkpoint", saved_wire.release());
  wire::flag(out.get(), "continued_f32_bit_exact", true);
  wire::flag(out.get(), "queued_release_f32_bit_exact", true);
  wire::u64(out.get(), "release_continuation_samples", release_second.samples);
  wire::put(out.get(), "release_continuation",
            observation(release_second, false).release());
  wire::put(out.get(), "restored_pending_management_checkpoint",
            restored_wire.release());
  wire::flag(out.get(), "device_executed", false);
  wire::flag(out.get(), "receiving_transport_installed", false);
  wire::text(out.get(), "standing",
             "actual activated source packet/Manager/Engine/P offline; 900s "
             "Act, private Scene replay, hardware and installed UI pending");
  // A genuinely invalid native force is refused; no bounded source cap changes.
  const auto valid_force_sample = owner->native().engine->admission_horizon();
  assert(valid_force_sample >= owner->native().engine->samples_elapsed());
  auto before_invalid = owner->stopped_checkpoint();
  auto before_invalid_wire =
      management_checkpoint_transport::checkpoint_wire(*before_invalid);
  auto invalid = operation(*owner, Kind::Parameter, valid_force_sample);
  invalid.parameter = Parameter::ForceNewtons;
  invalid.value = 101;
  assert(owner->enqueue_score_input(invalid) == Result::Invalid);
  auto after_invalid = owner->stopped_checkpoint();
  auto after_invalid_wire =
      management_checkpoint_transport::checkpoint_wire(*after_invalid);
  assert(json_object_equal(before_invalid_wire.get(), after_invalid_wire.get()));
  if (stress) {
    auto many = manager(source);
    auto p = operation(*many, Kind::Parameter, 0);
    p.parameter = Parameter::ForceNewtons;
    p.value = force;
    enqueue(*many, p);
    std::map<unsigned, NoteTarget> keys;
    for (const auto &target : many->native().notes)
      keys.emplace(target.key, target);
    assert(keys.size() == 7);
    std::array<Ref, 40> inputs{};
    std::array<NoteTarget, 40> notes{};
    for (std::size_t i = 0; i < 40; ++i) {
      auto at = keys.begin();
      std::advance(at, i % keys.size());
      notes[i] = at->second;
      notes[i].touch = notes[i].member = i + 1;
      inputs[i] =
          reference(("calibration:stress/input/" + std::to_string(i)).c_str());
    }
    for (std::size_t i = 0; i < 24; ++i) {
      auto on = operation(*many, Kind::NoteOn, 0);
      on.note = notes[i];
      on.value = .8;
      enqueue(*many, on, inputs[i]);
    }
    Observation observed;
    advance(*many, observed, 512);
    for (std::size_t i = 24; i < 40; ++i) {
      auto on = operation(*many, Kind::NoteOn, 512);
      on.note = notes[i];
      on.value = .8;
      enqueue(*many, on, inputs[i]);
    }
    // A single genuine sample sees all sixteen actual stolen-voice tails.
    advance(*many, observed, 1);
    assert(observed.voices == 24 && observed.tails == 16);
    through(*many, observed, trial_samples, 128);
    assert(observed.clipping == 0 && observed.limited == 0);
    auto stress_wire = observation(observed, false);
    auto available = wire::array();
    for (const auto &entry : keys)
      wire::append(available.get(), json_object_new_uint64(entry.first));
    wire::put(stress_wire.get(), "available_native_keys", available.release());
    wire::put(out.get(), "stress", stress_wire.release());
  }
  wire::u64(out.get(), "callback_allocations", allocations.load());
  wire::u64(out.get(), "callback_frees", frees.load());
  assert(allocations.load() == 0 && frees.load() == 0);
  return out;
}
int main() {
  try {
    std::string bytes;
    char character;
    while (std::cin.get(character)) {
      if (bytes.size() == 16 * 1024 * 1024)
        throw std::invalid_argument("native calibration input exceeds bound");
      bytes.push_back(character);
    }
    auto input = ql::physical_wire::parse_native(bytes.c_str());
    wire::keys(input.get(), {"schema", "source", "force_newtons", "stress"});
    assert(packet::string(packet::field(input.get(), "schema")) ==
           "ql.native-calibration-fixture/v1");
    const double force =
        packet::number(packet::field(input.get(), "force_newtons"));
    assert(force == .01 || force == 2.);
    auto out = calibration_trial(
        packet::field(input.get(), "source"), force,
        packet::boolean(packet::field(input.get(), "stress")));
    std::cout << json_object_to_json_string_ext(out.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
