// Actual retained sourceForm/M1 note -> sole native A/P -> installed receiving
// -> final Engine output and full stopped checkpoint. No test waveform/clock.
#include "../test_support/allocation_hooks.hpp"
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <memory>
#include <new>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_physical_receiving.hpp>
#include <ql/performance_receiving_wire.hpp>
#include <ql/performance_source_packet.hpp>
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
  r.context_ref = "research:moving/world";
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
  std::vector<float> pickup, received, pcm;
  std::unique_ptr<PhysicalBodyCheckpoint> physical;
  PhysicalSnapshot snapshot{};
  NativeReceivingReadback receiving{};
};
struct Session {
  std::unique_ptr<PerformanceManagement> owner;
  std::shared_ptr<MovingReceivingPortBinding> receiving_owner;
  std::vector<NativeGestureApplication> applications;
  std::vector<InputBindingRecord> journal;
  explicit Session(J *fixture, double source_speed, double receiver_speed,
                   bool install = true) {
    auto p = prepare(fixture);
    assert(p.engine->sample_rate() == 48000);
    assert(p.body->preparation().input().nodes.size() == 12 &&
           p.body->preparation().input().edges.size() == 34 &&
           p.notes.size() == 7);
    assert(p.determination.excitation.root_linear == 1 &&
           p.determination.excitation.octet_linear == 0 &&
           p.body->preparation().input().material.damping_alpha_per_second ==
               40);
    owner = std::make_unique<PerformanceManagement>(
        p, reference("research:port/native-manager"));
    const auto immutable = p.body->preparation();
    auto *context = packet::field(packet::field(fixture, "basis"), "context");
    assert(packet::string(packet::field(context, "kind")) == "world" &&
           !packet::boolean(packet::field(context, "private")) &&
           packet::string(packet::field(packet::field(context, "context"),
                                        "ref")) == "research:moving/world" &&
           packet::string(packet::field(packet::field(context, "receiver"),
                                        "ref")) == "research:moving/receiver");
    receiving_owner = std::make_shared<MovingReceivingPortBinding>(
        p.body, immutable, 0,
        receiving(immutable, source_speed, receiver_speed));
    if (install) {
      auto guard = p.engine->acquire_stopped_custody();
      assert(guard && p.engine->install_receiving_port(
                          receiving_owner->port(receiving_owner), guard, 0));
    }
    const auto admitted = owner->enqueue_score_input_admission(
        note(p), reference("research:port/original-input"));
    assert(admitted.result() == Result::Accepted && admitted.queue().queued());
    assert(admitted.queue().operation().sample == 0 &&
           admitted.queue().operation().requested_sample == 0);
    p.engine->enable_capture(true);
  }
  Output advance(std::uint64_t end, std::size_t block) {
    Output out;
    const auto begin = owner->native().engine->samples_elapsed();
    out.pickup.reserve(end - begin);
    out.received.reserve(end - begin);
    out.pcm.reserve(end - begin);
    std::array<float, 512> pcm{};
    while (owner->native().engine->samples_elapsed() < end) {
      const auto start = owner->native().engine->samples_elapsed();
      const auto frames = std::min<std::size_t>(block, end - start);
      Capture capture{};
      callback_probe = true;
      assert(owner->offline_advance(pcm.data(), frames, start));
      callback_probe = false;
      assert(owner->pop_audio_capture(capture));
      assert(capture.start_sample == start && capture.frames == frames);
      assert(capture.has_receiving ==
             owner->native().engine->has_receiving_port());
      auto pulse = owner->pulse();
      assert(pulse->has_readback &&
             pulse->reading.samples_elapsed == start + frames &&
             pulse->reading.physical.samples_elapsed == start + frames &&
             pulse->recording.failure == RecordingFailure::None);
      assert(pulse->reading.has_receiving == capture.has_receiving);
      if (capture.has_receiving) {
        assert(pulse->reading.receiving.samples_elapsed == start + frames);
        assert(same_receiving_manifest(pulse->reading.receiving.manifest,
                                       receiving_owner->manifest()));
        out.receiving = pulse->reading.receiving;
      }
      out.snapshot = pulse->reading.physical;
      applications.insert(applications.end(), pulse->applications.begin(),
                          pulse->applications.end());
      journal.insert(journal.end(), pulse->input_history.begin(),
                     pulse->input_history.end());
      for (std::size_t i = 0; i < frames; ++i) {
        assert(capture.output_linear[i] == pcm[i]);
        assert(pcm[i] ==
               float(std::clamp(.25 * double(capture.received_linear[i]), -.98,
                                .98)));
        if (!capture.has_receiving)
          assert(capture.received_linear[i] == capture.pickup_linear[i]);
      }
      out.pickup.insert(out.pickup.end(), capture.pickup_linear.begin(),
                        capture.pickup_linear.begin() + frames);
      out.received.insert(out.received.end(), capture.received_linear.begin(),
                          capture.received_linear.begin() + frames);
      out.pcm.insert(out.pcm.end(), pcm.begin(), pcm.begin() + frames);
    }
    auto guard = owner->native().engine->acquire_stopped_custody();
    assert(guard);
    out.physical = std::make_unique<PhysicalBodyCheckpoint>(
        owner->native().body->checkpoint());
    assert(out.physical->samples_elapsed == end &&
           out.snapshot.samples_elapsed == end);
    return out;
  }
};
static void exact_body(const PhysicalBodyCheckpoint &a,
                       const PhysicalBodyCheckpoint &b) {
  auto x = ql::physical_wire::checkpoint_wire(a),
       y = ql::physical_wire::checkpoint_wire(b);
  assert(json_object_equal(x.get(), y.get()));
}
static Json samples(const std::vector<float> &signal) {
  auto out = array();
  for (float x : signal)
    append(out.get(), json_object_new_double(x));
  return out;
}
static double frequency(const std::vector<float> &signal, std::size_t first) {
  std::vector<double> crossings;
  for (std::size_t i = std::max<std::size_t>(1, first); i < signal.size(); ++i)
    if (signal[i - 1] <= 0 && signal[i] > 0)
      crossings.push_back(double(i - 1) -
                          signal[i - 1] / (double(signal[i]) - signal[i - 1]));
  assert(crossings.size() > 100);
  return double(crossings.size() - 1) * 48000 /
         (crossings.back() - crossings.front());
}
static void original_input(const Session &s) {
  assert(s.applications.size() == 1 && s.journal.size() == 2);
  const auto &a = s.applications[0];
  assert(a.kind == Kind::NoteOn && a.applied && a.has_note && a.sequence == 1 &&
         a.applied_application_ordinal == 1 && a.has_requested_sample &&
         a.requested_sample == 0 && a.admitted_sample == 0 &&
         a.applied_sample == 0 &&
         a.physical_source_generation ==
             s.owner->native().body->preparation().input().source_generation);
  const auto &admitted = s.journal[0], &applied = s.journal[1];
  assert(admitted.change == InputBindingChange::PressAdmitted &&
         applied.change == InputBindingChange::Applied &&
         admitted.native_sequence == 1 && applied.native_sequence == 1 &&
         admitted.input_ref == reference("research:port/original-input") &&
         applied.input_ref == admitted.input_ref);
  auto n = checkpoint_transport::note(a.note),
       n0 = checkpoint_transport::note(admitted.target),
       n1 = checkpoint_transport::note(applied.target);
  assert(json_object_equal(n.get(), n0.get()) &&
         json_object_equal(n.get(), n1.get()));
}
static void corruption(Session &s) {
  auto saved = s.owner->stopped_checkpoint();
  const auto original =
      management_checkpoint_transport::checkpoint_wire(*saved);
  {
    const auto &immutable = s.owner->native().body->preparation();
    auto original_preparation = receiving(immutable, 0, 0);
    auto motion = original_preparation.motion();
    const auto duration = motion.end_sample - motion.origin_sample;
    motion.origin_sample = saved->native_pair.audio.cursor;
    motion.end_sample = motion.origin_sample + duration;
    PreparedMovingSpatialReceiving valid_current_receiver(
        immutable, original_preparation.spatial().input(), motion);
    auto new_owner = std::make_shared<MovingReceivingPortBinding>(
        s.owner->native().body, immutable, saved->native_pair.audio.cursor,
        std::move(valid_current_receiver));
    auto guard = s.owner->native().engine->acquire_stopped_custody();
    assert(guard && !s.owner->native().engine->install_receiving_port(
                        new_owner->port(new_owner), guard,
                        saved->native_pair.audio.cursor));
  }
  for (unsigned kind = 0; kind < 8; ++kind) {
    auto changed = std::make_unique<ManagementCheckpoint>(*saved);
    auto &cp = changed->native_pair.audio;
    if (kind == 0)
      cp.receiving.history_linear[1] = std::numeric_limits<float>::quiet_NaN();
    if (kind == 1)
      cp.receiving.manifest.context[0] =
          cp.receiving.manifest.context[0] == 'x' ? 'y' : 'x';
    if (kind == 2)
      cp.receiving.manifest.source_generation++;
    if (kind == 3)
      cp.receiving.manifest.eigenbasis[0] = 'x';
    if (kind == 4)
      cp.receiving.samples_elapsed++;
    if (kind == 5)
      cp.has_receiving = cp.receiving_encoding_present = false;
    if (kind == 6)
      cp.receiving.manifest.receiver[0] = 'x';
    if (kind == 7) {
      assert(cp.receiving.samples_elapsed > cp.receiving.history_start_sample);
      // Within the old inclusive bounds but detached from actual native birth:
      // this would erase every available delay tail if accepted.
      cp.receiving.history_start_sample = cp.receiving.samples_elapsed;
    }
    TransportAcknowledgement ack{};
    assert(!s.owner->stopped_restore(*changed, saved->native_pair.audio.cursor,
                                     reference("research:port/corrupt-restore"),
                                     reference("research:port/corrupt"), ack));
    auto unchanged = s.owner->stopped_checkpoint();
    auto after = management_checkpoint_transport::checkpoint_wire(*unchanged);
    assert(json_object_equal(original.get(), after.get()));
  }
  auto raw = checkpoint_transport::checkpoint_wire(saved->native_pair);
  J *audio = packet::field(raw.get(), "audio");
  json_object_object_del(audio, "receiving");
  bool refused = false;
  try {
    (void)checkpoint_transport::read_checkpoint_wire(raw.get());
  } catch (const std::invalid_argument &) {
    refused = true;
  }
  assert(refused);
}
static std::vector<KeyboardCell> source_catalog(J *fixture) {
  auto *rows = packet::field(fixture, "native_catalog");
  packet::array(rows, 36);
  std::vector<KeyboardCell> out;
  for (std::size_t i = 0; i < 36; ++i)
    out.push_back(
        management_transport::read_key(json_object_array_get_idx(rows, i)));
  return out;
}
static std::shared_ptr<MovingReceivingPortBinding>
replacement_port(Session &s, const NativeReceivingCheckpoint &saved,
                 unsigned changed_identity = 0) {
  const auto &immutable = s.owner->native().body->preparation();
  const auto before = receiving(immutable, 0, 0);
  auto spatial = before.spatial().input();
  spatial.receiver_position_metres = {21, 0, 0};
  if (changed_identity == 1)
    spatial.context_ref = "research:port/other-world";
  if (changed_identity == 2)
    spatial.receiver_ref = "research:port/other-receiver";
  auto motion = before.motion();
  motion.origin_sample = saved.samples_elapsed;
  motion.receiver_motion_ref = "research:port/receiver-motion-2";
  motion.policy_revision = "2";
  PreparedMovingSpatialReceiving after(immutable, spatial, motion);
  return std::make_shared<MovingReceivingPortBinding>(
      s.owner->native().body, immutable, saved.samples_elapsed,
      std::move(after), saved);
}
static Json replacement(J *fixture) {
  auto s = std::make_unique<Session>(fixture, 0, 0);
  const auto catalog = source_catalog(fixture);
  s->owner->admit_catalog(catalog);
  s->advance(4096, 128);
  original_input(*s);
  Operation automation{};
  automation.kind = Kind::Parameter;
  automation.identity = s->owner->native().determination.identity;
  automation.sequence = 2;
  automation.sample = 9000;
  automation.parameter = Parameter::MasterLinear;
  automation.value = .61;
  assert(s->owner->enqueue_score_input(automation) == Result::Accepted);
  Operation release{};
  release.kind = Kind::NoteOff;
  release.identity = automation.identity;
  release.sequence = 3;
  release.sample = 10000;
  release.touch = s->owner->native().notes.at(0).touch;
  assert(s->owner->enqueue_score_input(
             release, reference("research:port/original-input")) ==
         Result::Accepted);
  auto before = s->owner->stopped_checkpoint();
  assert(before->native_pair.audio.cursor == 4096 &&
         before->native_pair.audio.accepted_sequence == 3 &&
         before->native_pair.audio.applied_application_ordinal == 1 &&
         std::count_if(before->native_pair.audio.voices.begin(),
                       before->native_pair.audio.voices.end(),
                       [](const auto &v) { return v.active && !v.release; }) ==
             1);
  auto candidate = replacement_port(*s, before->native_pair.audio.receiving);
  // A changed finite sample is structurally valid but cannot replace the
  // actual native ring. It must refuse BEFORE any owner state mutates.
  auto corrupt = std::make_unique<NativeReceivingCheckpoint>(
      before->native_pair.audio.receiving);
  corrupt->history_linear[17] += .000001f;
  auto altered = replacement_port(*s, *corrupt);
  {
    auto guard = s->owner->native().engine->acquire_stopped_custody();
    auto token =
        std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
    assert(guard && !s->owner->prepare_stopped_receiving_replacement(
                        altered->port(altered), guard, 4096, *token));
  }
  // Real native validated same-cursor restore after preflight can change a
  // ring while all manifest/cursor/source/queue/catalogue fields stay exact.
  // Candidate and resident changes must BOTH invalidate the private token.
  for (bool mutate_resident : {false, true}) {
    auto saved_ring = std::make_unique<NativeReceivingCheckpoint>();
    auto changed_ring = std::make_unique<NativeReceivingCheckpoint>();
    auto prior_refusal = std::make_unique<Engine::Checkpoint>();
    auto after_refusal = std::make_unique<Engine::Checkpoint>();
    {
      auto guard = s->owner->native().engine->acquire_stopped_custody();
      auto token = std::make_unique<
          PerformanceManagement::PreparedReceivingReplacement>();
      auto candidate_port = candidate->port(candidate);
      assert(guard && s->owner->prepare_stopped_receiving_replacement(
                          candidate_port, guard, 4096, *token));
      auto port = mutate_resident ? s->receiving_owner->port(s->receiving_owner)
                                  : candidate_port;
      assert(port.write_checkpoint(port.owner, *saved_ring, 4096));
      *changed_ring = *saved_ring;
      changed_ring->history_linear[41] += .000001f;
      assert(changed_ring->history_linear[41] !=
                 saved_ring->history_linear[41] &&
             port.validate_checkpoint(port.owner, *changed_ring, 4096));
      port.restore_checkpoint(port.owner, *changed_ring);
      s->owner->native().engine->write_checkpoint(*prior_refusal, guard);
      const auto old_allocations = allocations, old_releases = releases;
      callback_probe =
          true; // numerical current() itself must not allocate/free
      const auto current =
          s->owner->receiving_replacement_current(*token, guard);
      callback_probe = false;
      assert(!current && allocations == old_allocations &&
             releases == old_releases);
      s->owner->native().engine->write_checkpoint(*after_refusal, guard);
      auto prior = audio_wire(*prior_refusal),
           after = audio_wire(*after_refusal);
      assert(json_object_equal(prior.get(), after.get()));
      // Restore only the actual deliberately changed port through its real
      // validated native API. No queue/cursor/source/body or journal is
      // retagged.
      assert(port.validate_checkpoint(port.owner, *saved_ring, 4096));
      port.restore_checkpoint(port.owner, *saved_ring);
    }
    auto restored = s->owner->stopped_checkpoint();
    auto original_wire =
             management_checkpoint_transport::checkpoint_wire(*before),
         restored_wire =
             management_checkpoint_transport::checkpoint_wire(*restored);
    assert(json_object_equal(original_wire.get(), restored_wire.get()));
  }
  // Two valid numerical contexts cannot use a geometry-only token to switch
  // the current receiver or privacy occasion. The combined current receiving
  // source/N9/context transaction is a different native owner operation.
  for (unsigned changed_identity : {1u, 2u}) {
    auto wrong = replacement_port(*s, before->native_pair.audio.receiving,
                                  changed_identity);
    auto guard = s->owner->native().engine->acquire_stopped_custody();
    auto token =
        std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
    assert(guard && !s->owner->prepare_stopped_receiving_replacement(
                        wrong->port(wrong), guard, 4096, *token));
  }
  auto unchanged = s->owner->stopped_checkpoint();
  auto bw = management_checkpoint_transport::checkpoint_wire(*before),
       uw = management_checkpoint_transport::checkpoint_wire(*unchanged);
  assert(json_object_equal(bw.get(), uw.get()));
  // A lawful same-source catalog write invalidates its private candidate.
  // Native note values/coordinates/ratios remain unchanged in this case.
  {
    auto guard = s->owner->native().engine->acquire_stopped_custody();
    auto stale =
        std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
    assert(guard && s->owner->prepare_stopped_receiving_replacement(
                        candidate->port(candidate), guard, 4096, *stale));
    auto renamed = catalog;
    renamed.front().label += " current";
    s->owner->admit_catalog(std::move(renamed));
    assert(!s->owner->receiving_replacement_current(*stale, guard));
  }
  {
    auto guard = s->owner->native().engine->acquire_stopped_custody();
    auto ready =
        std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
    assert(guard && s->owner->prepare_stopped_receiving_replacement(
                        candidate->port(candidate), guard, 4096, *ready));
    assert(s->owner->receiving_replacement_current(*ready, guard));
    s->owner->commit_stopped_receiving_replacement(*ready, guard);
    assert(!ready->ready());
    s->owner->refresh_stopped_reading(guard);
  }
  auto saved = s->owner->stopped_checkpoint();
  const auto &old_rx = before->native_pair.audio.receiving;
  const auto &new_rx = saved->native_pair.audio.receiving;
  assert(new_rx.samples_elapsed == 4096 && new_rx.history_start_sample == 0 &&
         new_rx.manifest.origin_sample == 4096 &&
         new_rx.manifest.history_origin_sample == 0 &&
         std::memcmp(old_rx.history_linear.data(), new_rx.history_linear.data(),
                     sizeof(old_rx.history_linear)) == 0);
  auto expected_manifest = old_rx.manifest;
  expected_manifest.receiving_identity = new_rx.manifest.receiving_identity;
  expected_manifest.receiver_motion = new_rx.manifest.receiver_motion;
  expected_manifest.policy_revision = new_rx.manifest.policy_revision;
  expected_manifest.origin_sample = 4096;
  assert(same_receiving_manifest(expected_manifest, new_rx.manifest));
  exact_body(before->native_pair.physical, saved->native_pair.physical);
  auto original = management_checkpoint_transport::checkpoint_wire(*before),
       after = management_checkpoint_transport::checkpoint_wire(*saved);
  // Only the explicitly checked receiving subsystem may differ. All voices,
  // phases, touches, pending queues/timing/IDs, source and input journal remain
  // byte-equivalent through the original native checkpoint transport.
  json_object_object_del(
      packet::field(packet::field(original.get(), "native_pair"), "audio"),
      "receiving");
  json_object_object_del(
      packet::field(packet::field(after.get(), "native_pair"), "audio"),
      "receiving");
  assert(json_object_equal(original.get(), after.get()));
  // The real native commit above installed candidate. Retain that exact owner
  // for subsequent same-pulse manifest checks after all checkpoint proofs.
  assert(same_receiving_manifest(candidate->manifest(), new_rx.manifest));
  s->receiving_owner = candidate;
  const auto first = s->advance(13000, 128);
  assert(std::any_of(first.received.begin(), first.received.begin() + 512,
                     [](float value) { return value != 0.f; }));
  auto stationary = std::make_unique<Session>(fixture, 0, 0);
  stationary->owner->admit_catalog(catalog);
  stationary->advance(4096, 128);
  assert(stationary->owner->enqueue_score_input(automation) ==
         Result::Accepted);
  assert(stationary->owner->enqueue_score_input(
             release, reference("research:port/original-input")) ==
         Result::Accepted);
  const auto unmoved = stationary->advance(13000, 128);
  assert(first.pickup == unmoved.pickup && first.received != unmoved.received &&
         first.pcm != unmoved.pcm);
  exact_body(*first.physical, *unmoved.physical);
  const auto first_journal = s->journal;
  const auto final = s->owner->stopped_checkpoint();
  assert(s->applications.size() == 3 && s->applications[1].sequence == 2 &&
         s->applications[1].applied_application_ordinal == 2 &&
         s->applications[1].requested_sample == 9000 &&
         s->applications[1].admitted_sample == 9000 &&
         s->applications[1].applied_sample == 9000 &&
         s->applications[2].sequence == 3 &&
         s->applications[2].applied_application_ordinal == 3 &&
         s->applications[2].requested_sample == 10000 &&
         s->applications[2].admitted_sample == 10000 &&
         s->applications[2].applied_sample == 10000 &&
         s->applications[2].touch == release.touch);
  assert(std::count_if(s->journal.begin(), s->journal.end(), [](const auto &h) {
           return h.change == InputBindingChange::ReleaseAdmitted &&
                  h.native_sequence == 3 &&
                  h.input_ref == reference("research:port/original-input");
         }) == 1);
  TransportAcknowledgement ack{};
  assert(s->owner->stopped_restore(
      *saved, 13000, reference("research:port/replacement-restore"),
      reference("research:port/replacement-saved-4096"), ack));
  assert(ack.previous_cursor == 13000 && ack.target_sample == 4096 &&
         ack.epoch == ack.previous_epoch + 1);
  const auto repeat = s->advance(13000, 512);
  assert(first.pickup == repeat.pickup && first.received == repeat.received &&
         first.pcm == repeat.pcm);
  exact_body(*first.physical, *repeat.physical);
  const auto replay_final = s->owner->stopped_checkpoint();
  auto f = receiving_checkpoint(final->native_pair.audio.receiving),
       rf = receiving_checkpoint(replay_final->native_pair.audio.receiving);
  assert(json_object_equal(f.get(), rf.get()));
  // A queue writer after releasing the original guard also invalidates a
  // prepared token. A new guard at the same cursor is not the same custody.
  auto queued = std::make_unique<Session>(fixture, 0, 0);
  queued->owner->admit_catalog(catalog);
  queued->advance(4096, 128);
  auto qcp = queued->owner->stopped_checkpoint();
  auto qp = replacement_port(*queued, qcp->native_pair.audio.receiving);
  auto stale =
      std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
  {
    auto guard = queued->owner->native().engine->acquire_stopped_custody();
    assert(guard && queued->owner->prepare_stopped_receiving_replacement(
                        qp->port(qp), guard, 4096, *stale));
  }
  assert(queued->owner->enqueue_score_input(automation) == Result::Accepted);
  {
    auto guard = queued->owner->native().engine->acquire_stopped_custody();
    assert(guard &&
           !queued->owner->receiving_replacement_current(*stale, guard));
  }
  auto output = object();
  text(output.get(), "schema", "ql.native-receiving-replacement-component/v1");
  text(output.get(), "standing",
       "stopped numerical same-owner replacement; private live/retained source "
       "reader is an independent required gate");
  put(output.get(), "before_receiving", receiving_checkpoint(old_rx).release());
  put(output.get(), "after_receiving", receiving_checkpoint(new_rx).release());
  put(output.get(), "final_receiving", f.release());
  put(output.get(), "physical_before",
      ql::physical_wire::checkpoint_wire(before->native_pair.physical)
          .release());
  put(output.get(), "physical_after",
      ql::physical_wire::checkpoint_wire(saved->native_pair.physical)
          .release());
  auto apps = array(), journal = array();
  for (std::size_t i = 0; i < 3; ++i)
    append(apps.get(), application(s->applications[i]).release());
  for (const auto &h : first_journal)
    if (h.native_sequence <= 3)
      append(journal.get(), management_transport::history(h).release());
  put(output.get(), "applications", apps.release());
  put(output.get(), "input_history", journal.release());
  put(output.get(), "continued_pcm", samples(first.pcm).release());
  put(output.get(), "stationary_pcm", samples(unmoved.pcm).release());
  put(output.get(), "continued_pickup", samples(first.pickup).release());
  put(output.get(), "stationary_pickup", samples(unmoved.pickup).release());
  u64(output.get(), "replacement_sample", 4096);
  u64(output.get(), "end_cursor", 13000);
  u64(output.get(), "sizeof_engine_candidate",
      sizeof(Engine::PreparedReceivingReplacement));
  u64(output.get(), "sizeof_management_candidate",
      sizeof(PerformanceManagement::PreparedReceivingReplacement));
  return output;
}
static Json measure(J *fixture) {
  auto receipt = object();
  text(receipt.get(), "schema", "ql.native-receiving-port-receipt/v1");
  const std::array<std::array<double, 2>, 3> speeds{{{0, 0}, {5, 0}, {0, -3}}};
  const std::array<const char *, 3> names{
      {"stationary", "moving_emitter", "moving_receiver"}};
  auto runs = array();
  std::unique_ptr<PhysicalBodyCheckpoint> common_body;
  std::vector<float> common_pickup;
  std::vector<float> common_output;
  NoteTarget selected{};
  for (unsigned route = 0; route < speeds.size(); ++route) {
    auto session =
        std::make_unique<Session>(fixture, speeds[route][0], speeds[route][1]);
    selected = session->owner->native().notes.at(0);
    auto first = session->advance(64000, 128);
    original_input(*session);
    auto saved = session->owner->stopped_checkpoint();
    assert(saved->native_pair.audio.receiving.history_start_sample == 0 &&
           saved->native_pair.audio.receiving.manifest.history_origin_sample ==
               0);
    assert(saved->native_pair.audio.has_receiving &&
           saved->native_pair.audio.receiving_encoding_present &&
           saved->native_pair.audio.receiving.samples_elapsed == 64000 &&
           saved->native_pair.physical.samples_elapsed == 64000);
    auto exact_saved_wire =
        management_checkpoint_transport::checkpoint_wire(*saved);
    auto decoded = management_checkpoint_transport::read_checkpoint_wire(
        exact_saved_wire.get());
    corruption(*session);
    auto continued = session->advance(65024, 128);
    TransportAcknowledgement ack{};
    assert(session->owner->stopped_restore(
        *decoded, 65024, reference("research:port/exact-restore"),
        reference("research:port/saved-64000"), ack));
    assert(ack.previous_cursor == 65024 && ack.target_sample == 64000 &&
           ack.epoch == ack.previous_epoch + 1);
    // Management publishes a copied receiving reading at the same restored
    // cursor. It may not reset the receiving history or hide the transport.
    auto restored = session->owner->pulse();
    assert(restored->has_readback && restored->reading.has_receiving &&
           restored->reading.samples_elapsed == 64000 &&
           restored->reading.receiving.samples_elapsed == 64000);
    auto restored_observation =
        receiving_transport::readback(restored->reading.receiving);
    assert(checkpoint_transport::decimal(packet::field(
               restored_observation.get(), "samples_elapsed")) == 64000);
    auto after_restore = session->owner->stopped_checkpoint();
    auto before_rx = receiving_checkpoint(saved->native_pair.audio.receiving),
         after_rx =
             receiving_checkpoint(after_restore->native_pair.audio.receiving);
    assert(json_object_equal(before_rx.get(), after_rx.get()));
    auto replayed = session->advance(65024, 512);
    assert(continued.pickup == replayed.pickup &&
           continued.received == replayed.received &&
           continued.pcm == replayed.pcm);
    exact_body(*continued.physical, *replayed.physical);
    auto remaining = session->advance(96000, 128);
    auto join = [&](std::vector<float> Output::*field) {
      auto out = first.*field;
      out.insert(out.end(), (continued.*field).begin(),
                 (continued.*field).end());
      out.insert(out.end(), (remaining.*field).begin(),
                 (remaining.*field).end());
      return out;
    };
    const auto pickup = join(&Output::pickup),
               received = join(&Output::received), pcm = join(&Output::pcm);
    auto partition =
        std::make_unique<Session>(fixture, speeds[route][0], speeds[route][1]);
    auto entire = partition->advance(96000, 512);
    assert(entire.pickup == pickup && entire.received == received &&
           entire.pcm == pcm);
    exact_body(*remaining.physical, *entire.physical);
    if (route == 0) {
      common_body =
          std::make_unique<PhysicalBodyCheckpoint>(*remaining.physical);
      common_pickup = pickup;
      common_output = pcm;
    } else {
      exact_body(*common_body, *remaining.physical);
      assert(common_pickup == pickup && common_output != pcm);
    }
    const double expected = route == 0   ? selected.hertz
                            : route == 1 ? selected.hertz * 340 / 335
                                         : selected.hertz * 343 / 340;
    const auto actual = frequency(pcm, 48000);
    assert(std::abs(1200 * std::log2(actual / expected)) <= 1);
    auto run = object();
    text(run.get(), "name", names[route]);
    real(run.get(), "measured_hertz", actual);
    real(run.get(), "expected_hertz", expected);
    put(run.get(), "received", samples(received).release());
    put(run.get(), "output", samples(pcm).release());
    put(run.get(), "checkpoint_receiving", after_rx.release());
    put(run.get(), "physical",
        management_transport::physical(remaining.snapshot).release());
    put(run.get(), "receiving",
        receiving_transport::readback(remaining.receiving).release());
    auto apps = array(), journal = array();
    for (const auto &a : session->applications)
      append(apps.get(), application(a).release());
    for (const auto &h : session->journal)
      append(journal.get(), management_transport::history(h).release());
    put(run.get(), "applications", apps.release());
    put(run.get(), "input_history", journal.release());
    append(runs.get(), run.release());
    // The explicitly finite admitted trajectory cannot wrap/restart, advance
    // a detached audio cursor or produce output beyond its native end.
    float beyond = 1;
    assert(!session->owner->offline_advance(&beyond, 1, 96000) && beyond == 0);
    assert(session->owner->native().engine->samples_elapsed() == 96000 &&
           session->owner->native().body->samples_elapsed() == 96000);
    exact_body(*remaining.physical,
               session->owner->native().body->checkpoint());
  }
  // Existing no-port native path stays exact pickup -> body gain. It cannot
  // acquire a transported output merely by a label or legacy CP interpretation.
  auto legacy = std::make_unique<Session>(fixture, 0, 0, false);
  auto raw = legacy->advance(96000, 512);
  assert(raw.pickup == common_pickup && raw.pcm != common_output);
  auto legacy_saved = legacy->owner->stopped_checkpoint();
  auto legacy_wire = audio_wire(legacy_saved->native_pair.audio);
  J *unknown = nullptr;
  assert(!json_object_object_get_ex(legacy_wire.get(), "has_receiving",
                                    &unknown) &&
         !json_object_object_get_ex(legacy_wire.get(), "receiving", &unknown));
  auto decoded = std::make_unique<Engine::Checkpoint>();
  read_audio(legacy_wire.get(), *decoded);
  assert(!decoded->has_receiving && !decoded->receiving_encoding_present);
  auto roundtrip = audio_wire(*decoded);
  assert(json_object_equal(legacy_wire.get(), roundtrip.get()));
  // Genuine two valid immutable numerical preparations: a receiver bound to
  // another body/source generation cannot enter the existing same-P owner.
  auto wrong_input = legacy->owner->native().body->preparation().input();
  wrong_input.source_generation++;
  wrong_input.body_revision++;
  PreparedPhysicalBody wrong(wrong_input);
  auto wrong_motion = receiving(wrong, 0, 0).motion();
  wrong_motion.origin_sample = 96000;
  wrong_motion.end_sample = 192000;
  PreparedMovingSpatialReceiving wrong_receiving(
      wrong, receiving(wrong, 0, 0).spatial().input(), wrong_motion);
  auto wrong_binding = std::make_shared<MovingReceivingPortBinding>(
      legacy->owner->native().body, wrong, 96000, std::move(wrong_receiving));
  {
    auto guard = legacy->owner->native().engine->acquire_stopped_custody();
    assert(guard && !legacy->owner->native().engine->install_receiving_port(
                        wrong_binding->port(wrong_binding), guard, 96000));
  }
  assert(!legacy->owner->native().engine->has_receiving_port());
  assert(allocations == 0 && releases == 0);
  put(receipt.get(), "replacement", replacement(fixture).release());
  put(receipt.get(), "native_note",
      checkpoint_transport::note(selected).release());
  put(receipt.get(), "pickup", samples(common_pickup).release());
  put(receipt.get(), "runs", runs.release());
  put(receipt.get(), "physical_checkpoint",
      ql::physical_wire::checkpoint_wire(*common_body).release());
  u64(receipt.get(), "end_cursor", 96000);
  u64(receipt.get(), "exact_restore_start", 64000);
  u64(receipt.get(), "exact_restore_frames", 1024);
  u64(receipt.get(), "callback_allocations", allocations);
  u64(receipt.get(), "callback_releases", releases);
  u64(receipt.get(), "sizeof_engine", sizeof(Engine));
  u64(receipt.get(), "sizeof_audio_checkpoint", sizeof(Engine::Checkpoint));
  u64(receipt.get(), "sizeof_paired_checkpoint", sizeof(PairedCheckpoint));
  u64(receipt.get(), "sizeof_management_checkpoint",
      sizeof(ManagementCheckpoint));
  u64(receipt.get(), "sizeof_readback", sizeof(Readback));
  u64(receipt.get(), "sizeof_capture", sizeof(Capture));
  text(receipt.get(), "object_custody",
       "Engine and all audio/paired/management checkpoints are heap-owned; "
       "ordinary native callback/control stack without raised limits");
  text(receipt.get(), "open",
       "Current private Act/context host installation and prepared live "
       "after-body/receiver transaction remain separate; native component is "
       "not device/UI acceptance");
  return receipt;
}
int main() {
  try {
    const std::string input((std::istreambuf_iterator<char>(std::cin)), {});
    require(input.size() < 16 * 1024 * 1024,
            "native receiving-port fixture bound exceeded");
    Json fixture = own(json_tokener_parse(input.c_str()));
    require(bool(fixture) &&
                packet::string(packet::field(fixture.get(), "schema")) ==
                    "ql.native-moving-receiving-fixture/v1",
            "native receiving-port source fixture refused");
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
