// Actual retained sourceForm/M1 note -> one native A/P pickup -> explicitly
// declared retarded-time receiving. No test oscillator, copied body or clock.
#include "../test_support/allocation_hooks.hpp"
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <memory>
#include <new>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_source_packet.hpp>
#include <ql/physical_moving_receiving.hpp>
#include <vector>
using namespace ql;
using namespace ql::performance;
using namespace ql::performance::checkpoint_transport;
using J = json_object;
using Json = ql::physical_wire::Json;
static bool callback_probe = false;
static std::size_t allocations = 0, releases = 0;
void *operator new(std::size_t n) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate(n))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n) { return ::operator new(n); }
void operator delete(void *p) noexcept {
  if (p && callback_probe)
    ++releases;
  ql_test_release(p);
}
void operator delete[](void *p) noexcept { ::operator delete(p); }
void operator delete(void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::size_t) noexcept { ::operator delete(p); }
void *operator new(std::size_t n, std::align_val_t a) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate_aligned(n, static_cast<std::size_t>(a)))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t n, std::align_val_t a) {
  return ::operator new(n, a);
}
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
static NativePerformance prepare(J *fixture) {
  auto value = packet::field(fixture, "native_preparation");
  Parameters p;
  p.force_newtons = .01;
  p.monitor_linear = 0;
  return prepare_source_performance_packet(
      json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN), value,
      packet::field(fixture, "native_basis"), true, true, p);
}
static Operation note(const NativePerformance &p) {
  Operation op{};
  op.kind = Kind::NoteOn;
  op.identity = p.determination.identity;
  op.sequence = 1;
  op.sample = 0;
  op.note = p.notes.at(0);
  op.touch = op.note.touch;
  op.value = .7;
  return op;
}
static PreparedMovingSpatialReceiving receiving(const PreparedPhysicalBody &p,
                                                double source_speed,
                                                double receiver_speed) {
  SpatialReceivingInput r;
  r.receiver_ref = "research:moving/receiver";
  r.context_ref = "research:neutral-world";
  r.source_ref = "research:actual-source-form-pickup";
  r.policy_ref = "research:point-source-retarded-linear-delay";
  r.policy_revision = "1";
  r.standing = "architecture-model";
  // Put the independently prepared pickup rest point at metric origin.
  for (std::size_t n = 0; n < p.input().nodes.size(); ++n)
    for (unsigned a = 0; a < 3; ++a)
      r.source_translation_metres[a] -=
          p.input().nodes[n].rest_metres[a] * p.input().pickup.node_weights[n];
  r.receiver_position_metres = {20, 0, 0};
  r.receiver_forward = {-1, 0, 0};
  r.transition_samples = 0;
  SpatialMotionInput m;
  m.source_motion_ref = "research:moving/source-constant-metric-velocity";
  m.receiver_motion_ref = "research:moving/receiver-constant-metric-velocity";
  m.policy_ref = "research:explicit-subsonic-retarded-time";
  m.policy_revision = "1";
  m.standing = "architecture-model";
  m.source_velocity_metres_per_second = {source_speed, 0, 0};
  m.receiver_velocity_metres_per_second = {receiver_speed, 0, 0};
  m.origin_sample = 0;
  m.end_sample = 96000;
  return PreparedMovingSpatialReceiving(p, r, m);
}
struct Output {
  std::vector<float> pickup, stationary, source, receiver;
  std::unique_ptr<PhysicalBodyCheckpoint> physical;
  PhysicalSnapshot snapshot{};
};
struct Session {
  std::unique_ptr<PerformanceManagement> owner;
  std::array<std::unique_ptr<MovingSpatialReceiving>, 3> transports;
  explicit Session(J *fixture) {
    auto p = prepare(fixture);
    assert(p.body->preparation().input().nodes.size() == 12 &&
           p.body->preparation().input().edges.size() == 34 &&
           p.notes.size() == 7);
    assert(p.determination.excitation.root_linear == 1 &&
           p.determination.excitation.octet_linear == 0 &&
           p.body->preparation().input().material.damping_alpha_per_second ==
               40);
    owner = std::make_unique<PerformanceManagement>(
        p, reference("research:moving/native-manager"));
    const auto immutable = p.body->preparation();
    const std::array<std::array<double, 2>, 3> speeds{
        {{0, 0}, {5, 0}, {0, -3}}};
    for (unsigned i = 0; i < 3; ++i)
      transports[i] = std::make_unique<MovingSpatialReceiving>(
          immutable, 0, receiving(immutable, speeds[i][0], speeds[i][1]));
    assert(owner->enqueue_score_input(
               note(p), reference("research:moving/original-input")) ==
           Result::Accepted);
    p.engine->enable_capture(true);
  }
  Output advance(std::uint64_t end, std::size_t block) {
    Output result;
    const auto begin = owner->native().engine->samples_elapsed();
    result.pickup.reserve(end - begin);
    result.stationary.reserve(end - begin);
    result.source.reserve(end - begin);
    result.receiver.reserve(end - begin);
    std::array<float, 512> pcm{}, stationary{}, source{}, receiver{};
    while (owner->native().engine->samples_elapsed() < end) {
      const auto start = owner->native().engine->samples_elapsed();
      const auto frames = std::min<std::size_t>(block, end - start);
      auto &body = *owner->native().body;
      Capture capture{};
      callback_probe = true;
      for (const auto &transport : transports)
        assert(transport->preflight_before_body(body, frames, start));
      assert(owner->offline_advance(pcm.data(), frames, start));
      assert(owner->pop_audio_capture(capture));
      assert(capture.start_sample == start && capture.frames == frames);
      callback_probe = false;
      // Snapshot allocation is under actual stopped offline-owner custody,
      // outside the measured callback path. Receiving itself may neither
      // advance nor modify the modal q/v state it observes.
      auto observing_custody =
          owner->native().engine->acquire_stopped_custody();
      assert(observing_custody);
      auto before = std::make_unique<PhysicalBodyCheckpoint>(body.checkpoint());
      callback_probe = true;
      assert(transports[0]->process_pickup_block(
          body, capture.pickup_linear.data(), stationary.data(), frames,
          start));
      assert(transports[1]->process_pickup_block(
          body, capture.pickup_linear.data(), source.data(), frames, start));
      assert(transports[2]->process_pickup_block(
          body, capture.pickup_linear.data(), receiver.data(), frames, start));
      callback_probe = false;
      auto after = ql::physical_wire::checkpoint_wire(body.checkpoint());
      auto before_wire = ql::physical_wire::checkpoint_wire(*before);
      assert(json_object_equal(before_wire.get(), after.get()));
      auto pulse = owner->pulse();
      assert(pulse->has_readback &&
             pulse->reading.samples_elapsed == start + frames &&
             pulse->reading.physical.samples_elapsed == start + frames &&
             pulse->recording.failure == RecordingFailure::None);
      result.snapshot = pulse->reading.physical;
      for (const auto &transport : transports)
        assert(transport->samples_elapsed() == start + frames);
      result.pickup.insert(result.pickup.end(), capture.pickup_linear.begin(),
                           capture.pickup_linear.begin() + frames);
      result.stationary.insert(result.stationary.end(), stationary.begin(),
                               stationary.begin() + frames);
      result.source.insert(result.source.end(), source.begin(),
                           source.begin() + frames);
      result.receiver.insert(result.receiver.end(), receiver.begin(),
                             receiver.begin() + frames);
    }
    auto guard = owner->native().engine->acquire_stopped_custody();
    assert(guard);
    result.physical = std::make_unique<PhysicalBodyCheckpoint>(
        owner->native().body->checkpoint());
    assert(result.physical->samples_elapsed == end &&
           result.snapshot.samples_elapsed == end);
    return result;
  }
};
static double frequency(const std::vector<float> &signal, std::size_t first) {
  // Interpolated actual positive crossings after one second of the explicitly
  // damped body's settling. No generated reference waveform enters the path.
  std::vector<double> crossings;
  for (std::size_t i = std::max<std::size_t>(1, first); i < signal.size(); ++i)
    if (signal[i - 1] <= 0 && signal[i] > 0) {
      const double span = double(signal[i]) - signal[i - 1];
      crossings.push_back(double(i - 1) - signal[i - 1] / span);
    }
  assert(crossings.size() > 100);
  return double(crossings.size() - 1) * 48000 /
         (crossings.back() - crossings.front());
}
static Json samples(const std::vector<float> &signal) {
  auto out = array();
  for (float x : signal)
    append(out.get(), json_object_new_double(x));
  return out;
}
static void exact_body(const PhysicalBodyCheckpoint &a,
                       const PhysicalBodyCheckpoint &b) {
  auto x = ql::physical_wire::checkpoint_wire(a),
       y = ql::physical_wire::checkpoint_wire(b);
  assert(json_object_equal(x.get(), y.get()));
}
template <class F> static void refuses(F f) {
  bool refused = false;
  try {
    f();
  } catch (const std::invalid_argument &) {
    refused = true;
  }
  assert(refused);
}
static void exact_receiving(const MovingSpatialCheckpoint &a,
                            const MovingSpatialCheckpoint &b) {
  assert(a.version == b.version && a.contract == b.contract &&
         a.receiving_identity == b.receiving_identity &&
         a.body_eigenbasis_identity == b.body_eigenbasis_identity &&
         a.signal_unit == b.signal_unit && a.distance_unit == b.distance_unit &&
         a.speed_unit == b.speed_unit && a.cursor_unit == b.cursor_unit &&
         a.samples_elapsed == b.samples_elapsed &&
         a.history_start_sample == b.history_start_sample &&
         a.history_linear == b.history_linear);
}
static Json measure(J *fixture) {
  auto session = std::make_unique<Session>(fixture);
  const auto immutable = session->owner->native().body->preparation();
  const auto basis = immutable.eigenbasis_identity();
  const auto selected = session->owner->native().notes.at(0);
  auto first = session->advance(64000, 128);
  auto saved = session->owner->stopped_checkpoint();
  std::array<std::unique_ptr<MovingSpatialCheckpoint>, 3> receiving_saved;
  for (unsigned i = 0; i < 3; ++i) {
    receiving_saved[i] = std::make_unique<MovingSpatialCheckpoint>();
    session->transports[i]->write_checkpoint(*session->owner->native().body,
                                             *receiving_saved[i]);
  }
  auto continued = session->advance(65024, 128);
  for (unsigned i = 0; i < 3; ++i) {
    auto original = std::make_unique<MovingSpatialCheckpoint>();
    session->transports[i]->write_checkpoint(*session->owner->native().body,
                                             *original);
    for (unsigned failure = 0; failure < 3; ++failure) {
      auto corrupt = std::make_unique<MovingSpatialCheckpoint>(*original);
      if (failure == 0)
        corrupt->body_eigenbasis_identity += ":wrong";
      if (failure == 1)
        corrupt->history_linear[0] = std::numeric_limits<float>::quiet_NaN();
      if (failure == 2)
        corrupt->receiving_identity += ":wrong-context";
      assert(!session->transports[i]->restore_checkpoint(
          *session->owner->native().body, *corrupt, 65024));
      auto after = std::make_unique<MovingSpatialCheckpoint>();
      session->transports[i]->write_checkpoint(*session->owner->native().body,
                                               *after);
      exact_receiving(*original, *after);
    }
  }
  TransportAcknowledgement ack{};
  assert(session->owner->stopped_restore(
      *saved, 65024, reference("research:moving/exact-restore"),
      reference("research:moving/saved-64000"), ack));
  assert(ack.previous_cursor == 65024 && ack.target_sample == 64000 &&
         ack.epoch == ack.previous_epoch + 1);
  auto &body = *session->owner->native().body;
  auto before_receiving_restore =
      std::make_unique<PhysicalBodyCheckpoint>(body.checkpoint());
  for (unsigned i = 0; i < 3; ++i) {
    assert(session->transports[i]->samples_elapsed() == 65024);
    assert(session->transports[i]->restore_checkpoint(body, *receiving_saved[i],
                                                      65024));
  }
  exact_body(*before_receiving_restore, body.checkpoint());
  auto replayed = session->advance(65024, 512);
  assert(continued.pickup == replayed.pickup &&
         continued.stationary == replayed.stationary &&
         continued.source == replayed.source &&
         continued.receiver == replayed.receiver);
  exact_body(*continued.physical, *replayed.physical);
  auto remaining = session->advance(96000, 128);
  auto join = [&](std::vector<float> Output::*field) {
    auto out = first.*field;
    out.insert(out.end(), (continued.*field).begin(), (continued.*field).end());
    out.insert(out.end(), (remaining.*field).begin(), (remaining.*field).end());
    return out;
  };
  const auto pickup = join(&Output::pickup),
             stationary = join(&Output::stationary),
             source = join(&Output::source), receiver = join(&Output::receiver);
  const double f0 = frequency(stationary, 48000), fs = frequency(source, 48000),
               fr = frequency(receiver, 48000), input_hz = selected.hertz;
  const std::array<double, 3> expected{
      {input_hz, input_hz * 340 / 335, input_hz * 343 / 340}};
  const std::array<double, 3> measured{{f0, fs, fr}};
  for (unsigned i = 0; i < 3; ++i)
    assert(std::abs(1200 * std::log2(measured[i] / expected[i])) <= 1);
  assert(source != stationary && receiver != stationary && fs > f0 && fr > f0);
  auto partition = std::make_unique<Session>(fixture);
  const auto entire = partition->advance(96000, 512);
  assert(entire.pickup == pickup && entire.stationary == stationary &&
         entire.source == source && entire.receiver == receiver);
  exact_body(*remaining.physical, *entire.physical);
  assert(body.preparation().eigenbasis_identity() == basis);
  for (unsigned i = 0; i < 3; ++i) {
    SpatialMotionPoint at_zero{}, at_one{};
    assert(session->transports[i]->preparation().point_at(0, at_zero) &&
           session->transports[i]->preparation().point_at(48000, at_one));
    assert(!session->transports[i]->preflight_before_body(body, 1, 96000));
    assert(session->transports[i]->samples_elapsed() == 96000);
    if (i == 0)
      assert(at_zero.propagation_delay_samples == 20.0 / 340 * 48000);
    if (i == 1) {
      assert(std::abs(at_zero.propagation_delay_samples - 20.0 / 335 * 48000) <
             1e-10);
      assert(std::abs(at_one.emission_samples_per_received_sample -
                      340.0 / 335) < 1e-12);
    }
    if (i == 2)
      assert(std::abs(at_one.emission_samples_per_received_sample -
                      343.0 / 340) < 1e-12);
  }
  auto invalid_motion = receiving(immutable, 0, 0).motion();
  invalid_motion.source_velocity_metres_per_second = {9, 0, 0};
  refuses([&] {
    PreparedMovingSpatialReceiving(immutable,
                                   receiving(immutable, 0, 0).spatial().input(),
                                   invalid_motion);
  });
  auto changed_input = immutable.input();
  changed_input.body_revision++;
  PreparedPhysicalBody changed(changed_input);
  assert(!session->transports[0]->preparation().matches_preparation(changed));
  assert(allocations == 0 && releases == 0);
  auto out = object();
  text(out.get(), "schema", "ql.native-moving-receiving-receipt/v1");
  text(out.get(), "standing",
       "declared retarded-time point-source architecture model over actual "
       "sourceForm A/P pickup; vendor DSP and device/UI not executed");
  put(out.get(), "native_note", checkpoint_transport::note(selected).release());
  put(out.get(), "physical",
      management_transport::physical(remaining.snapshot).release());
  put(out.get(), "physical_checkpoint",
      ql::physical_wire::checkpoint_wire(*remaining.physical).release());
  real(out.get(), "stationary_hertz", f0);
  real(out.get(), "moving_emitter_hertz", fs);
  real(out.get(), "moving_receiver_hertz", fr);
  real(out.get(), "expected_emitter_hertz", expected[1]);
  real(out.get(), "expected_receiver_hertz", expected[2]);
  u64(out.get(), "end_cursor", 96000);
  u64(out.get(), "exact_restore_start", 64000);
  u64(out.get(), "exact_restore_frames", 1024);
  u64(out.get(), "callback_allocations", allocations);
  u64(out.get(), "callback_releases", releases);
  put(out.get(), "pickup", samples(pickup).release());
  put(out.get(), "stationary_received", samples(stationary).release());
  put(out.get(), "moving_emitter_received", samples(source).release());
  put(out.get(), "moving_receiver_received", samples(receiver).release());
  text(out.get(), "open",
       "Engine final-output receiving port, scheduled trajectory changes and "
       "same-Act live controls remain separate production joins; no ideal "
       "radiation/reconstruction claim");
  return out;
}
int main() {
  try {
    const std::string input((std::istreambuf_iterator<char>(std::cin)), {});
    require(input.size() < 16 * 1024 * 1024,
            "native moving fixture input bound exceeded");
    Json fixture = own(json_tokener_parse(input.c_str()));
    require(bool(fixture), "native moving source fixture parse refused");
    require(packet::string(packet::field(fixture.get(), "schema")) ==
                "ql.native-moving-receiving-fixture/v1",
            "native moving source fixture schema refused");
    auto result = measure(fixture.get());
    std::cout << json_object_to_json_string_ext(result.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &e) {
    callback_probe = false;
    std::cerr << e.what() << '\n';
    return 1;
  }
}
