// Real Rust retained N9/body producer -> SAME native Manager/P callback ->
// exact original observation wire and checkpoint continuation. This numerical
// test grants no private Scene/Act authority and claims no attached hardware.
#include "../test_support/allocation_hooks.hpp"
#include "../test_support/native_fixture_carrier.hpp"
#include <cassert>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iostream>
#include <new>
#include <ql/performance_capture_wire.hpp>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_physical_receiving.hpp>

static bool callback_probe = false;
static std::size_t allocations = 0, releases = 0;
void *operator new(std::size_t bytes) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate(bytes))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t bytes) { return ::operator new(bytes); }
void operator delete(void *p) noexcept {
  if (p && callback_probe)
    ++releases;
  ql_test_release(p);
}
void operator delete[](void *p) noexcept { ::operator delete(p); }
void operator delete(void *p, std::size_t) noexcept { ::operator delete(p); }
void operator delete[](void *p, std::size_t) noexcept { ::operator delete(p); }
void *operator new(std::size_t bytes, std::align_val_t a) {
  if (callback_probe)
    ++allocations;
  if (void *p = ql_test_allocate_aligned(bytes, std::size_t(a)))
    return p;
  throw std::bad_alloc();
}
void *operator new[](std::size_t bytes, std::align_val_t a) {
  return ::operator new(bytes, a);
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
using namespace ql::performance;
namespace wire = checkpoint_transport;
namespace capture_wire = native_capture_transport;
using J = json_object;
using Json = wire::Json;
using ql::require;

struct Rig {
  std::unique_ptr<PerformanceManagement> owner;
  std::shared_ptr<PhysicalRoutesPortBinding> routes;
  std::shared_ptr<MovingReceivingPortBinding> receiving;
  explicit Rig(J *fixture) {
    auto *p = packet::field(fixture, "performance_preparation");
    auto native = prepare_performance_packet(
        json_object_to_json_string_ext(p, JSON_C_TO_STRING_PLAIN),
        packet::field(p, "native_basis"), true, true);
    const auto immutable = native.body->preparation();
    auto *refs = packet::field(fixture, "program_refs");
    std::vector<std::string> programme_refs;
    for (std::size_t i = 0; i < json_object_array_length(refs); ++i)
      programme_refs.push_back(
          packet::string(json_object_array_get_idx(refs, i)));
    auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
        read_native_receiving_admission(
            packet::field(fixture, "native_admission"),
            packet::field(fixture, "current_native_admission"), native,
            packet::field(p, "native_basis"), immutable, programme_refs, 0));
    routes = std::make_shared<PhysicalRoutesPortBinding>(
        native.body, admitted, immutable, native.determination,
        packet::field(p, "native_basis"), true, 0);
    NativeRouteProgramSet programmes{};
    programmes.manifest = routes->manifest();
    programmes.program_count = programmes.manifest.route_count;
    programmes.scalar_note_enabled = programmes.manifest.scalar_note_enabled;
    programmes.scalar_note_gain = programmes.manifest.scalar_note_gain;
    for (std::size_t i = 0; i < programmes.program_count; ++i) {
      auto &programme = programmes.programs[i];
      programme.handle = programmes.manifest.programs[i];
      programme.phase_source_ref = native.determination.m1_coordinate;
      programme.sine = native.notes.front().phase_sin;
      programme.cosine = native.notes.front().phase_cos;
    }
    owner = std::make_unique<PerformanceManagement>(
        native, reference("native:capture/actual-retained-manager"));
    require(owner->admit_distinct_receiving(
                     physical_routes_port(physical_port(native.body), routes),
                     programmes, 0)
                    .result == Result::Accepted,
            "actual native source N9/P admission refused");
    // Declared numerical observing policy on this actual native body. No
    // source/receiver token or C lease is invented by this component test.
    ql::SpatialReceivingInput spatial{};
    spatial.receiver_ref = "reference:capture/numerical-receiver";
    spatial.context_ref = "reference:capture/numerical-context";
    spatial.source_ref = "reference:capture/actual-physical-pickup";
    spatial.policy_ref = "reference:capture/spherical-delay";
    spatial.policy_revision = "1";
    spatial.standing = "reference";
    spatial.revision = 1;
    spatial.receiver_position_metres = {0.5, 0.25, 1};
    spatial.receiver_forward = {0, 0, -1};
    spatial.speed_metres_per_second = 343;
    spatial.minimum_distance_metres = .1;
    spatial.propagation_delay = true;
    ql::SpatialMotionInput motion{};
    motion.source_motion_ref = "reference:capture/retained-source-motion";
    motion.receiver_motion_ref = "reference:capture/retained-receiver-motion";
    motion.policy_ref = spatial.policy_ref;
    motion.policy_revision = "1";
    motion.standing = "reference";
    motion.origin_sample = 0;
    motion.end_sample = 48000;
    auto prepared =
        ql::PreparedMovingSpatialReceiving(immutable, spatial, motion);
    receiving = std::make_shared<MovingReceivingPortBinding>(
        native.body, immutable, 0, std::move(prepared));
    auto guard = owner->native().engine->acquire_stopped_custody();
    require(bool(guard) && owner->native().engine->install_receiving_port(
                               receiving->port(receiving), guard, 0),
            "actual same native P receiving admission refused");
    owner->refresh_stopped_reading(guard);
  }
};
static Operation op(PerformanceManagement &owner, Kind kind,
                    std::uint64_t sequence, std::uint64_t date) {
  Operation out{};
  out.kind = kind;
  out.sequence = sequence;
  out.sample = date;
  out.identity = owner.native().engine->current_source().identity;
  return out;
}
static Json take(Rig &rig, const ManagementPulse &pulse) {
  return capture_wire::batch(*rig.owner, pulse);
}
static std::unique_ptr<ManagementPulse> render(Rig &rig,
                                               std::array<float, 128> &pcm) {
  const auto cursor = rig.owner->native().engine->samples_elapsed();
  callback_probe = true;
  const bool committed =
      rig.owner->offline_advance(pcm.data(), pcm.size(), cursor);
  callback_probe = false;
  require(committed && !allocations && !releases,
          "original capture callback allocated/released or refused P");
  return rig.owner->pulse();
}
static J *one_block(J *batch) {
  auto *blocks = packet::field(batch, "audio_blocks");
  require(json_object_array_length(blocks) == 1,
          "actual single callback observation missing/duplicated");
  require(json_object_array_length(packet::field(batch, "device_blocks")) == 0,
          "numerical callback falsely claims AUHAL block");
  return json_object_array_get_idx(blocks, 0);
}
static std::uint32_t sample_bits(float value) {
  std::uint32_t bits{};
  static_assert(sizeof(bits) == sizeof(value));
  std::memcpy(&bits, &value, sizeof(bits));
  return bits;
}
static bool pcm_bits_equal(const std::array<float, 128> &left,
                           const std::array<float, 128> &right) {
  for (std::size_t i = 0; i < left.size(); ++i)
    if (sample_bits(left[i]) != sample_bits(right[i]))
      return false;
  return true;
}
static bool wire_samples_bits_equal(J *left, J *right) {
  if (json_object_get_type(left) != json_type_array ||
      json_object_get_type(right) != json_type_array ||
      json_object_array_length(left) != json_object_array_length(right))
    return false;
  for (std::size_t i = 0; i < json_object_array_length(left); ++i)
    if (sample_bits(static_cast<float>(
            json_object_get_double(json_object_array_get_idx(left, i)))) !=
        sample_bits(static_cast<float>(
            json_object_get_double(json_object_array_get_idx(right, i)))))
      return false;
  return true;
}
static void compare_block_sample_bits(J *left, J *right) {
  for (const char *name :
       {"force_newtons", "note_force_newtons", "contact_force_newtons",
        "body_gain_linear", "monitor_gain_linear", "force_scale_newtons",
        "pickup_linear", "received_linear", "output_linear"})
    require(wire_samples_bits_equal(packet::field(left, name),
                                    packet::field(right, name)),
            "reopened original native float sample bits changed");
  auto *l = packet::field(left, "route_force_newtons");
  auto *r = packet::field(right, "route_force_newtons");
  require(json_object_array_length(l) == json_object_array_length(r),
          "reopened native route force suffix changed");
  for (std::size_t i = 0; i < json_object_array_length(l); ++i)
    require(wire_samples_bits_equal(json_object_array_get_idx(l, i),
                                    json_object_array_get_idx(r, i)),
            "reopened original native route float sample bits changed");
}
static void compare_array(J *block, const char *name,
                          const std::array<float, 128> &pcm) {
  auto *samples = packet::field(block, name);
  require(json_object_array_length(samples) == pcm.size(),
          "actual capture frame suffix lost/expanded");
  for (std::size_t i = 0; i < pcm.size(); ++i)
    require(sample_bits(static_cast<float>(json_object_get_double(
                json_object_array_get_idx(samples, i)))) == sample_bits(pcm[i]),
            "observed PCM float bits were rescaled/reconstructed");
}
static Json run(J *fixture) {
  Rig rig(fixture);
  auto born = rig.owner->stopped_checkpoint();
  require(!born->native_pair.audio.capture &&
              born->native_pair.audio.cursor == 0 &&
              born->native_pair.audio.accepted_sequence == 0,
          "capture enrollment changed original native birth");
  auto initial = rig.owner->pulse();
  auto empty = take(rig, *initial);
  require(
      json_object_array_length(packet::field(empty.get(), "audio_blocks")) ==
              0 &&
          !packet::boolean(packet::field(packet::field(empty.get(), "counters"),
                                         "has_callback_readback")),
      "birth readback fabricated callback sound");
  const bool started = capture_wire::start_device(*rig.owner);
  require(!started && !rig.owner->native().engine->capture_enabled(),
          "unprepared actual device start fabricated success or changed "
          "capture flag");
  const auto force = rig.owner->enqueue_stopped_parameter_admission(
      Parameter::ForceNewtons, 2,
      rig.owner->native().engine->current_source().identity, 0);
  require(force.queue().queued() && force.queue().operation().sequence == 1 &&
              force.queue().operation().has_requested_sample &&
              force.queue().operation().requested_sample == 0,
          "actual queued Force2 timing changed");
  auto pending = rig.owner->stopped_checkpoint();
  require(pending->native_pair.audio.accepted_sequence == 1 &&
              pending->native_pair.audio.applied_application_ordinal == 0,
          "queued Force2 falsely became performed");
  rig.owner->native().engine->enable_capture(true);
  auto attack = op(*rig.owner, Kind::NoteOn, 2, 0);
  attack.note = rig.owner->native().notes.at(0);
  attack.value = .8;
  const auto input = reference("native:capture/original-janko-input");
  require(rig.owner->enqueue_score_input(attack, input) == Result::Accepted,
          "actual native original touch refused");
  auto parameter = op(*rig.owner, Kind::Parameter, 3, 768);
  parameter.parameter = Parameter::MasterLinear;
  parameter.value = .6;
  auto release = op(*rig.owner, Kind::NoteOff, 4, 1152);
  release.touch = attack.note.touch;
  require(rig.owner->enqueue_score_input(parameter, {}) == Result::Accepted &&
              rig.owner->enqueue_score_input(release, input) ==
                  Result::Accepted,
          "actual future control/release refused");
  auto batches = wire::array();
  std::array<float, 128> pcm{};
  bool physical_sound = false, spatial_sound = false, route_force = false;
  std::uint64_t native_negative_zero_samples = 0, native_zero_refusals = 0;
  for (unsigned i = 0; i < 4; ++i) {
    auto pulse = render(rig, pcm);
    auto batch = take(rig, *pulse);
    auto *block = one_block(batch.get());
    require(
        wire::decimal(packet::field(block, "start_sample")) == i * 128 &&
            packet::integer(packet::field(block, "sample_rate")) ==
                rig.owner->native().engine->sample_rate() &&
            wire::decimal(packet::field(block, "body_revision")) ==
                pulse->reading.body_revision &&
            json_object_equal(packet::field(block, "identity"),
                              wire::identity(pulse->reading.identity).get()) &&
            wire::decimal(packet::field(block, "end_application_ordinal")) ==
                pulse->reading.last_applied_application_ordinal &&
            pulse->reading.physical.samples_elapsed == (i + 1) * 128,
        "capture lost actual sample/body/application/source scope");
    compare_array(block, "output_linear", pcm);
    // Exercise signed-zero loss only where the actual callback produced zero.
    // The original native block is restored and checked again before retention.
    auto *output = packet::field(block, "output_linear");
    for (std::size_t sample = 0; sample < pcm.size(); ++sample) {
      native_negative_zero_samples += sample_bits(pcm[sample]) == 0x80000000U;
      if (pcm[sample] != 0)
        continue;
      auto original =
          wire::own(json_object_get(json_object_array_get_idx(output, sample)));
      require(json_object_array_put_idx(
                  output, sample,
                  json_object_new_double(-double(pcm[sample]))) == 0,
              "actual native zero mutation failed");
      bool refused = false;
      try {
        compare_array(block, "output_linear", pcm);
      } catch (const std::exception &) {
        refused = true;
      }
      require(json_object_array_put_idx(output, sample, original.release()) ==
                  0,
              "actual native zero restoration failed");
      require(refused, "native signed-zero corruption was accepted");
      ++native_zero_refusals;
      compare_array(block, "output_linear", pcm);
    }
    auto *pickup = packet::field(block, "pickup_linear"),
         *received = packet::field(block, "received_linear"),
         *route = packet::field(block, "route_force_newtons");
    require(json_object_array_length(route) == 9 &&
                packet::boolean(packet::field(block, "has_receiving")),
            "actual source N9/receiving capture missing");
    require(
        json_object_equal(
            packet::field(block, "receiving_manifest"),
            wire::receiving_manifest(pulse->reading.receiving.manifest).get()),
        "original M4 context/source trajectory replaced by later reading");
    for (unsigned sample = 0; sample < 128; ++sample) {
      physical_sound |= json_object_get_double(
                            json_object_array_get_idx(pickup, sample)) != 0;
      spatial_sound |= json_object_get_double(
                           json_object_array_get_idx(received, sample)) != 0;
      for (unsigned r = 0; r < 9; ++r)
        route_force |= json_object_get_double(json_object_array_get_idx(
                           json_object_array_get_idx(route, r), sample)) != 0;
    }
    if (i == 0)
      require(pulse->applications.size() == 2 &&
                  pulse->applications.front().kind == Kind::Parameter &&
                  pulse->applications.front().applied &&
                  pulse->applications.front().value == 2 &&
                  pulse->applications.front().applied_sample == 0,
              "captured Force2 was not the actual performed control");
    wire::append(batches.get(), batch.release());
    auto repeated = take(rig, *pulse);
    require(json_object_array_length(
                packet::field(repeated.get(), "audio_blocks")) == 0,
            "same original capture block replayed twice");
  }
  require(
      physical_sound && spatial_sound && route_force,
      "capture returned disconnected physical/receiving/native-force samples");
  auto saved = rig.owner->stopped_checkpoint();
  auto saved_wire = management_checkpoint_transport::checkpoint_wire(*saved);
  require(saved->native_pair.audio.cursor == 512 &&
              saved->native_pair.audio.capture &&
              saved->native_pair.audio.heap_size == 2 &&
              saved->native_pair.audio.has_receiving &&
              saved->native_pair.audio.receiving.history_linear.size() == 16384,
          "native saved take lost original voices/queue/receiver ring");
  auto decoded =
      management_checkpoint_transport::read_checkpoint_wire(saved_wire.get());
  Rig reopened(fixture);
  TransportAcknowledgement ack{};
  require(reopened.owner->stopped_restore(
              *decoded, 0, reference("native:capture/actual-reopen"),
              reference("native:capture/cut512"), ack) &&
              ack.target_sample == 512 && ack.epoch == 2 &&
              reopened.owner->native().engine->capture_enabled(),
          "fresh same-source native checkpoint continuation refused");
  std::array<float, 128> resumed{};
  auto continued = wire::array();
  for (unsigned i = 0; i < 12; ++i) {
    auto a = render(rig, pcm), b = render(reopened, resumed);
    auto left = take(rig, *a), right = take(reopened, *b);
    auto *l = one_block(left.get()), *r = one_block(right.get());
    require(pcm_bits_equal(pcm, resumed) && json_object_equal(l, r),
            "reopened original native force/pickup/received/PCM/sequence "
            "capture changed");
    compare_array(l, "output_linear", pcm);
    compare_array(r, "output_linear", resumed);
    compare_block_sample_bits(l, r);
    wire::append(continued.get(), right.release());
  }
  auto ended = rig.owner->stopped_checkpoint(),
       repeated = reopened.owner->stopped_checkpoint();
  auto left_physical =
           ql::physical_wire::checkpoint_wire(ended->native_pair.physical),
       right_physical =
           ql::physical_wire::checkpoint_wire(repeated->native_pair.physical);
  require(json_object_equal(left_physical.get(), right_physical.get()),
          "reopened sole P q/v differs");
  // A newer callback can finish after the copied Management pulse. Export
  // only blocks through that ORIGINAL committed cursor; leave the newer one
  // for its own pulse without rendering or minting a second readback.
  Rig fenced(fixture);
  fenced.owner->native().engine->enable_capture(true);
  auto older = render(fenced, pcm);
  require(fenced.owner->offline_advance(resumed.data(), 128, 128),
          "actual later original callback refused");
  auto old_batch = take(fenced, *older);
  require(wire::decimal(
              packet::field(one_block(old_batch.get()), "start_sample")) == 0 &&
              fenced.owner->native().engine->samples_elapsed() == 256,
          "capture export crossed original pulse or advanced native clock");
  auto newer = fenced.owner->pulse();
  auto new_batch = take(fenced, *newer);
  require(wire::decimal(
              packet::field(one_block(new_batch.get()), "start_sample")) == 128,
          "original next callback was lost/recreated at previous pulse");
  // Real finite original ring overflow. The readback sees each preceding
  // overflow; the CURRENT failed push appears in the following callback.
  Rig crowded(fixture);
  crowded.owner->native().engine->enable_capture(true);
  for (unsigned i = 0; i < capture_capacity + 2; ++i) {
    callback_probe = true;
    const bool ok = crowded.owner->offline_advance(pcm.data(), 128, i * 128);
    callback_probe = false;
    require(ok && !allocations && !releases,
            "actual overflow callback refused/allocated");
  }
  auto overflow_pulse = crowded.owner->pulse();
  auto overflow = take(crowded, *overflow_pulse);
  require(
      json_object_array_length(packet::field(overflow.get(), "audio_blocks")) ==
              capture_capacity &&
          wire::decimal(packet::field(packet::field(overflow.get(), "counters"),
                                      "dropped_audio_blocks_at_readback")) == 1,
      "original native capture overflow was hidden or capacity raised");
  auto following = render(crowded, pcm);
  auto gap = take(crowded, *following);
  auto *late = one_block(gap.get());
  require(
      wire::decimal(packet::field(late, "start_sample")) ==
              (capture_capacity + 2) * 128 &&
          wire::decimal(packet::field(packet::field(gap.get(), "counters"),
                                      "dropped_audio_blocks_at_readback")) == 2,
      "actual missing capture interval was fabricated/renumbered");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.actual-native-capture-component/v1");
  wire::text(out.get(), "standing",
             "actual same-owner callback/PCM/reopen; no closed C lease or "
             "attached device claim");
  wire::put(out.get(), "source_packet",
            json_object_get(packet::field(fixture, "performance_preparation")));
  wire::put(out.get(), "born_checkpoint",
            management_checkpoint_transport::checkpoint_wire(*born).release());
  wire::put(
      out.get(), "pending_force_checkpoint",
      management_checkpoint_transport::checkpoint_wire(*pending).release());
  wire::put(out.get(), "batches", batches.release());
  wire::put(out.get(), "saved_checkpoint", saved_wire.release());
  wire::put(out.get(), "continued_batches", continued.release());
  wire::put(out.get(), "overflow_batch", overflow.release());
  wire::put(out.get(), "gap_batch", gap.release());
  wire::u64(out.get(), "callback_allocations", allocations);
  wire::u64(out.get(), "callback_releases", releases);
  wire::u64(out.get(), "native_negative_zero_samples",
            native_negative_zero_samples);
  wire::u64(out.get(), "native_zero_refusals", native_zero_refusals);
  return out;
}
int main() {
  try {
    std::string input;
    char chunk[8192];
    while (std::cin.read(chunk, sizeof(chunk)) || std::cin.gcount()) {
      input.append(chunk, std::size_t(std::cin.gcount()));
      require(input.size() <= 8 * 1024 * 1024,
              "actual native input exceeds bound");
    }
    auto carrier = ql_test_native_carrier::parse(input);
    auto fixture = ql_test_native_carrier::decode(carrier.get());
    auto result = run(fixture.get());
    std::cout << json_object_to_json_string_ext(result.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
