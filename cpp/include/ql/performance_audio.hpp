#ifndef QL_PERFORMANCE_AUDIO_HPP
#define QL_PERFORMANCE_AUDIO_HPP
// Native M1 excitation, consuming prepared M2 musical targets. Preparation and
// source joining run on the control owner. This callback contains no theory,
// graph, JSON, device configuration, allocation, locks or UI-owned clock.
#include <algorithm>
#include <array>
#include <atomic>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <exception>
#include <limits>
#include <memory>
#include <ql/m_tree_live.h>
#include <ql/performance_route_programs.hpp>
#include <ql/physical_snapshot.hpp>
#include <stdexcept>
#include <string>
#include <type_traits>

namespace ql::performance {
inline constexpr const char *contract = "ql.performance-audio/v1";
inline constexpr std::size_t max_frames = 512, max_voices = 24,
                             max_touches = 96, max_tails = 16,
                             queue_capacity = 256, capture_capacity = 16;
inline constexpr double tau = 6.283185307179586476925286766559;
inline constexpr double parameter_smoothing_seconds = 0.005;
using Ref = std::array<char, 256>;
inline Ref reference(const char *value) {
  if (!value || !*value || std::strlen(value) >= 256)
    throw std::invalid_argument("bounded performance reference required");
  Ref out{};
  std::memcpy(out.data(), value, std::strlen(value));
  return out;
}
inline bool valid_ref(const Ref &ref) noexcept {
  return ref[0] && std::find(ref.begin(), ref.end(), '\0') != ref.end();
}
struct Identity {
  Ref instance{}, event{}, subject{};
  std::uint64_t m1_revision = 0, m2_generation = 0;
  bool operator==(const Identity &other) const noexcept {
    return instance == other.instance && event == other.event &&
           subject == other.subject && m1_revision == other.m1_revision &&
           m2_generation == other.m2_generation;
  }
};
struct NodalBoundary {
  std::uint8_t position = 0, face = 0, m = 0, n = 0;
};
enum class OctetScaling : std::uint8_t { NoteRelativeToReference, AbsoluteBus };
// D30 implementation policy, distinct from the source's eight antinodal roles.
// Each canonical key selects one M1 excitation program: its native root and
// eight components tuned by the ACTUAL M2 bus. No key indexes an octet slot.
struct ExcitationPolicy {
  Ref policy_ref{}, standing{};
  OctetScaling scaling = OctetScaling::NoteRelativeToReference;
  double reference_hertz = 0, root_linear = 1, octet_linear = 0;
  std::array<double, 8> weights{0.125, 0.125, 0.125, 0.125,
                                0.125, 0.125, 0.125, 0.125};
};
// Native M1/K and actual M2/B producer outputs, copied once after their
// complete joined validation. The receipt names coherence; it never implies
// permission or live graph authentication. Source face is independent of
// temporal phase.
struct Determination {
  Identity identity{};
  Ref m1_coordinate{}, m2_writer{}, registry_revision{}, source_revision{},
      relation_plan_ref{}, tuning_ref{}, native_receipt_ref{},
      body_preparation_ref{}, body_state_ref{};
  std::uint8_t m1_face = 0, m2_face = 1, tick12 = 0, basis = 0, lens12 = 0,
               context_frame = 1;
  std::uint16_t degree720 = 0;
  std::array<double, 8> audio_octet_hz{};
  std::array<NodalBoundary, 4> nodal_quartet{};
  ExcitationPolicy excitation{};
  std::uint64_t body_revision = 0;
  bool tuning_available = false;
};
struct NoteTarget {
  Identity identity{};
  Ref source_coordinate{}, tuning_ref{}, touch_ref{};
  std::uint64_t member = 0, touch = 0, ratio_numerator = 0,
                ratio_denominator = 0;
  std::uint8_t key = 0, position = 0, coordinate_face = 0, source_face = 0,
               pitch_class = 0;
  std::int8_t register_octave = 0;
  double fundamental_hz = 0, hertz = 0, phase_sin = 0, phase_cos = 1;
  bool exact_ratio = false;
};
// A scalar Newton force along P's prepared node/axis projection. P is the sole
// physical numerical owner and exposes pickup/displacement from its same q/v.
// All function pointers must refer to that resident owner for the full
// lifetime.
struct PhysicalPort {
  void *owner = nullptr;
  bool (*advance)(void *, const double *, float *, std::size_t, std::uint64_t,
                  std::uint64_t) noexcept = nullptr;
  std::uint64_t (*revision)(const void *) noexcept = nullptr;
  std::uint64_t (*cursor)(const void *) noexcept = nullptr;
  bool (*observe)(const void *, ql::PhysicalSnapshot &, std::uint64_t,
                  std::uint64_t) noexcept = nullptr;
  double max_force_newtons = 0;
  unsigned sample_rate = 0;
  Ref event{}, subject{}, preparation{}, state{};
  // Acquired/released on the control thread only. Device output requires
  // resident ownership so even a refused OS stop cannot free callback data.
  std::shared_ptr<void> custody{};
  void *routes_owner = nullptr;
  const ql::PhysicalForceRoutePortManifest *route_manifest = nullptr;
  std::shared_ptr<void> routes_custody{};
  bool (*advance_routes)(void *, const ql::PhysicalScalarForceBlock &,
                         const ql::PhysicalRouteForceBlock *, std::size_t,
                         float *, std::size_t, std::uint64_t,
                         std::uint64_t) noexcept = nullptr;
  bool (*observe_routes)(const void *,
                         ql::PhysicalForceRouteReceipt &) noexcept = nullptr;
};
enum class Kind : std::uint8_t {
  NoteOn,
  NoteOff,
  Sustain,
  Expression,
  Panic,
  Parameter,
  Determination
};
enum class Parameter : std::uint8_t {
  ForceNewtons,
  AttackSeconds,
  ReleaseSeconds,
  CutoffHertz,
  MasterLinear,
  BodyLinear,
  MonitorLinear
};
struct NativeClockMetadata {
  std::uint64_t epoch = 0, anchor_ordinal = 0, trigger_host_ticks = 0,
                admitted_host_ticks = 0;
  double mapping_uncertainty_samples = 0;
  bool input_transit_unknown = true;
};
class NativeOutputClock;
struct Operation {
  Kind kind = Kind::Panic;
  Identity identity{};
  std::uint64_t sequence = 0, sample = 0, touch = 0;
  NoteTarget note{};
  // NoteOn velocity [0,1]; expression pressure [0,1] and prepared pitch Hz;
  // sustain 0/1; parameter in the units stated by its enum. No raw MIDI pitch.
  double value = 0, pitch_hz = 0;
  Parameter parameter = Parameter::MasterLinear;
  Determination determination{};
  bool late_admitted = false;
  NativeClockMetadata native_clock{};
  // Native admission alone stamps the original requested sample. sample remains
  // the resolved queue deadline. Missing legacy provenance is never inferred.
  std::uint64_t requested_sample = 0;
  bool has_requested_sample = false;
};
struct ReleaseOperation {
  Kind kind = Kind::NoteOff;
  Identity identity{};
  std::uint64_t sequence = 0, sample = 0, touch = 0;
  bool late_admitted = false;
  NativeClockMetadata native_clock{};
  std::uint64_t requested_sample = 0;
  bool has_requested_sample = false;
};
enum class Result : std::uint8_t {
  Accepted,
  Invalid,
  Stale,
  Late,
  Order,
  Overflow,
  Unavailable,
  Exhausted
};
// Constructed only by the actual Engine admission. It is neither a
// caller-carried operation nor a callback application receipt.
class NativeQueueAdmission {
  friend class Engine;
  Result result_ = Result::Unavailable;
  Operation operation_{};
  Determination source_{};
  std::uint64_t cursor_ = 0, horizon_ = 0;

public:
  Result result() const noexcept { return result_; }
  bool queued() const noexcept { return result_ == Result::Accepted; }
  const Operation &operation() const noexcept { return operation_; }
  const Determination &source() const noexcept { return source_; }
  std::uint64_t queue_cursor() const noexcept { return cursor_; }
  std::uint64_t queue_horizon() const noexcept { return horizon_; }
};
struct Parameters {
  double force_newtons = 0.01, attack_seconds = 0.003, release_seconds = 0.05,
         cutoff_hertz = 18000, master_linear = 0.25, body_linear = 1,
         monitor_linear = 0;
};
struct VoiceReadback {
  std::uint64_t member = 0, m1_revision = 0, m2_generation = 0;
  double effective_hertz = 0, target_hertz = 0, envelope = 0;
  std::uint32_t held_touches = 0;
  bool releasing = false;
  std::array<double, 8> octet_effective_hertz{}, octet_target_hertz{};
  std::uint8_t suppressed_octet_components = 0;
};
enum class RecordingFailure : std::uint8_t {
  None,
  ApplicationQueueOverflow,
  PhysicalCommitFailure,
  ApplicationScratchOverflow
};
struct RecordingStatus {
  RecordingFailure failure = RecordingFailure::None;
  std::uint64_t dropped_applications = 0, first_failed_sequence = 0,
                first_failed_sample = 0;
};
struct Readback {
  Identity identity{};
  Determination determination{};
  ql::PhysicalSnapshot physical{};
  std::uint64_t samples_elapsed = 0, body_revision = 0, last_sequence = 0,
                last_applied_application_ordinal = 0, refused = 0, late = 0,
                overflows = 0, stolen = 0, dropped_readbacks = 0,
                dropped_captures = 0, clipping_samples = 0,
                force_limited_samples = 0;
  std::uint32_t active_voices = 0, active_touches = 0;
  double peak = 0, rms = 0, scalar_force_budget_newtons = 0;
  std::uint64_t force_zero_samples = 0, emergency_requested = 0,
                emergency_observed = 0, emergency_applied_sample = 0;
  std::uint32_t active_tails = 0;
  std::array<std::uint64_t, max_touches> held_touch_tokens{};
  Parameters source{}, effective{};
  std::array<VoiceReadback, max_voices> voices{};
  RecordingStatus recording{};
  ql::PhysicalForceRouteReceipt physical_routes{};
  bool has_route_programs = false, routes_suspended = false;
  bool sustain = false, available = false;
};
struct NativeGestureApplication {
  Identity identity{};
  NativeClockMetadata clock{};
  NoteTarget note{};
  Determination determined_source{};
  bool has_determination = false;
  Ref preparation_ref{}, state_ref{}, physical_event{}, physical_subject{},
      physical_source_coordinate{}, physical_source_revision{}, eigenbasis{};
  std::uint64_t physical_source_generation = 0;
  std::uint32_t physical_sample_rate = 0;
  bool physical_pratibimba = false;
  std::uint64_t applied_application_ordinal = 0;
  // Admission ID is independent of callback application order.
  std::uint64_t sequence = 0, admitted_sample = 0, applied_sample = 0,
                committed_cursor = 0, body_revision = 0, touch = 0;
  std::uint64_t requested_sample = 0;
  bool has_requested_sample = false;
  Kind kind = Kind::NoteOn;
  Parameter parameter = Parameter::MasterLinear;
  double value = 0, pitch_hz = 0;
  bool has_note = false, applied = false, late_admitted = false;
};

struct Capture {
  Identity identity{}, end_identity{};
  std::uint64_t start_sample = 0, body_revision = 0;
  std::uint32_t frames = 0;
  std::array<double, max_frames> force_newtons{};
  std::array<float, max_frames> pickup_linear{}, output_linear{};
  std::size_t route_count = 0;
  std::array<std::array<double, max_frames>,
             ql::physical_max_personal_force_routes>
      route_force_newtons{};
};
template <class T, std::size_t N> class Spsc {
  static_assert(N > 1 && std::is_trivially_copyable_v<T>);
  std::array<T, N> storage_{};
  alignas(64) std::atomic<std::uint64_t> write_{0};
  alignas(64) std::atomic<std::uint64_t> read_{0};

public:
  struct State {
    std::array<T, N> storage{};
    std::uint64_t write = 0, read = 0;
  };
  static bool valid_state(const State &value) noexcept {
    return value.write >= value.read && value.write - value.read <= N;
  }
  void write_checkpoint(State &out) const noexcept {
    out.storage = storage_;
    out.write = write_.load();
    out.read = read_.load();
  }
  State checkpoint() const noexcept {
    State out{};
    write_checkpoint(out);
    return out;
  }
  void restore_checkpoint(const State &value) noexcept {
    storage_ = value.storage;
    read_.store(value.read);
    write_.store(value.write);
  }
  void clear_stopped() noexcept {
    read_.store(0);
    write_.store(0);
  }
  bool push(const T &item) noexcept {
    const auto w = write_.load(std::memory_order_relaxed),
               r = read_.load(std::memory_order_acquire);
    if (w - r >= N)
      return false;
    storage_[w % N] = item;
    write_.store(w + 1, std::memory_order_release);
    return true;
  }
  const T *peek() const noexcept {
    const auto r = read_.load(std::memory_order_relaxed);
    if (r == write_.load(std::memory_order_acquire))
      return nullptr;
    return &storage_[r % N];
  }
  void pop() noexcept { read_.fetch_add(1, std::memory_order_release); }
  bool take(T &out) noexcept {
    const auto *p = peek();
    if (!p)
      return false;
    out = *p;
    pop();
    return true;
  }
  // Consume only entries published at this invocation's boundary. A producer
  // refilling freed slots cannot extend callback work past the fixed N budget.
  template <class Consumer> bool drain_snapshot(Consumer &&consume) noexcept {
    const auto r = read_.load(std::memory_order_relaxed);
    const auto w = write_.load(std::memory_order_acquire);
    const auto budget = std::min<std::uint64_t>(N, w - r);
    T item{};
    for (std::uint64_t i = 0; i < budget; ++i) {
      if (!take(item) || !consume(item))
        return false;
    }
    return true;
  }
};
static_assert(std::atomic<std::uint64_t>::is_always_lock_free &&
                  std::atomic<bool>::is_always_lock_free,
              "performance callback requires native lock-free atomics");

// One producer (the existing serial control owner), one callback consumer.
// Source decisions can be replaced at a precise sample using a prepared event;
// a whole determination cannot change physical preparation implicitly.
class Engine {
public:
  struct Voice {
    bool active = false, release = false;
    NoteTarget note{};
    std::uint64_t born = 0;
    double sine = 0, cosine = 1, envelope = 0, velocity = 0, pressure = 1,
           frequency = 0, target_frequency = 0, filter = 0;
    std::uint32_t release_remaining = 0;
    std::array<double, 8> octet_sine{}, octet_cosine{1, 1, 1, 1, 1, 1, 1, 1},
        octet_frequency{}, octet_weight{};
    double root_gain = 0, octet_gain = 0;
  };
  struct Touch {
    std::uint64_t token = 0, member = 0;
    double velocity = 0, pressure = 1;
    Ref source_touch_ref{};
    Identity source_identity{};
    NoteTarget original_note{};
  };
  struct Tail {
    Voice voice{};
    std::uint32_t left = 0;
  };
  struct ScheduledDetermination {
    Determination value{};
    std::uint64_t from_sample = 0;
  };
  struct PendingOperation {
    bool active = false;
    Operation operation{};
  };
  struct PendingRelease {
    bool active = false;
    ReleaseOperation operation{};
  };
  struct Checkpoint {
    static constexpr const char *schema = "ql.performance-checkpoint/v2";
    std::uint32_t version = 2;
    unsigned sample_rate = 0;
    Determination determination{}, producer_determination{};
    Identity producer_identity{};
    std::array<ScheduledDetermination, 8> source_schedule{};
    std::size_t source_schedule_size = 0;
    std::array<Voice, max_voices> voices{};
    std::array<Touch, max_touches> touches{};
    std::array<Tail, max_tails> tails{};
    typename Spsc<Operation, queue_capacity>::State operations{};
    std::array<PendingOperation, queue_capacity> pending_operations{};
    std::array<std::size_t, queue_capacity> operation_heap{};
    std::size_t heap_size = 0;
    typename Spsc<ReleaseOperation, 64>::State releases{};
    std::array<PendingRelease, 64> pending_releases{};
    typename Spsc<NativeGestureApplication, 256>::State applications{};
    RecordingStatus recording{};
    bool has_route_programs = false;
    NativeRouteProgramSet route_programs{};
    Parameters source{}, effective{};
    std::uint64_t cursor = 0, accepted_sequence = 0, accepted_sample = 0,
                  applied_sequence = 0, applied_application_ordinal = 0,
                  refused = 0, late = 0, stolen = 0, dropped_readbacks = 0,
                  dropped_captures = 0, clipping = 0, force_limited = 0,
                  overflow_count = 0, panic_fence = 0, force_zero_samples = 0,
                  emergency_requested = 0, emergency_observed = 0,
                  emergency_applied_sample = 0;
    bool emergency = false, capture = false, fault = false, sustain = false;
  };
  class StoppedCustody {
    friend class Engine;
    Engine *owner_ = nullptr;
    explicit StoppedCustody(Engine *owner) : owner_(owner) {}

  public:
    StoppedCustody() = default;
    StoppedCustody(const StoppedCustody &) = delete;
    StoppedCustody &operator=(const StoppedCustody &) = delete;
    StoppedCustody(StoppedCustody &&other) noexcept : owner_(other.owner_) {
      other.owner_ = nullptr;
    }
    ~StoppedCustody() {
      if (owner_)
        owner_->activity_.store(0, std::memory_order_release);
    }
    explicit operator bool() const noexcept { return owner_ != nullptr; }
  };

  // Host heap custody, prepared only under the exact stopped callback guard.
  // No public fields/JSON constructor can change a validated candidate. The
  // consumed token retains the retired port until the control owner records
  // the successful combined transaction and releases its acknowledgement.
  class PreparedCombinedRevision {
    friend class Engine;
    Engine *owner_ = nullptr;
    Determination before_{}, after_{};
    PhysicalPort candidate_port_{};
    NativeRouteProgramSet continuation_{};
    std::array<double, ql::physical_max_personal_force_routes> step_sine_{},
        step_cosine_{};
    std::uint64_t cursor_ = 0, accepted_sequence_ = 0, applied_ordinal_ = 0,
                  emergency_requested_ = 0, control_revision_ = 0;
    bool ready_ = false;

  public:
    PreparedCombinedRevision() = default;
    PreparedCombinedRevision(const PreparedCombinedRevision &) = delete;
    PreparedCombinedRevision &
    operator=(const PreparedCombinedRevision &) = delete;
    PreparedCombinedRevision(PreparedCombinedRevision &&) = delete;
    PreparedCombinedRevision &operator=(PreparedCombinedRevision &&) = delete;
    bool ready() const noexcept { return ready_; }
    const Determination &after_determination() const noexcept { return after_; }
    const NativeRouteProgramSet &after_programs() const noexcept {
      return continuation_;
    }
    std::uint64_t sample() const noexcept { return cursor_; }
  };

private:
  Determination determination_{};
  // Owned only by the serial producer; callback never touches this copy.
  Determination producer_determination_{};
  Identity producer_identity_{};
  std::array<ScheduledDetermination, 8> source_schedule_{};
  std::size_t source_schedule_size_ = 1;
  unsigned rate_;
  PhysicalPort body_{};
  std::uint64_t combined_control_revision_ = 0;
  bool has_route_programs_ = false;
  NativeRouteProgramSet route_programs_{};
  std::array<double, ql::physical_max_personal_force_routes> route_step_sine_{},
      route_step_cosine_{};
  std::array<Voice, max_voices> voices_{};
  std::array<Touch, max_touches> touches_{};
  std::array<Tail, max_tails> tails_{};
  Spsc<Operation, queue_capacity> operations_{};
  std::array<PendingOperation, queue_capacity> pending_operations_{};
  std::array<std::size_t, queue_capacity> operation_heap_{};
  std::size_t heap_size_ = 0;
  Spsc<ReleaseOperation, 64> releases_{};
  std::array<PendingRelease, 64> pending_releases_{};
  Spsc<Readback, 64> readbacks_{};
  Spsc<Capture, capture_capacity> captures_{};
  Spsc<NativeGestureApplication, 256> gesture_applications_{};
  // Fixed heap-owned scratch: receipts become public only after P/output
  // commit.
  std::array<NativeGestureApplication, queue_capacity + 64>
      block_applications_{};
  std::size_t block_application_count_ = 0;
  std::atomic<std::uint64_t> dropped_applications_{0},
      recording_failure_sequence_{0}, recording_failure_sample_{0};
  std::atomic<std::uint8_t> recording_failure_{0};
  void recording_failed(RecordingFailure reason, std::uint64_t sequence,
                        std::uint64_t sample) noexcept {
    if (!recording_failure_.load(std::memory_order_relaxed)) {
      recording_failure_sequence_.store(sequence, std::memory_order_relaxed);
      recording_failure_sample_.store(sample, std::memory_order_relaxed);
      recording_failure_.store(std::uint8_t(reason), std::memory_order_release);
    }
  }
  bool stage_application(const NativeGestureApplication &value) noexcept {
    if (block_application_count_ == block_applications_.size()) {
      recording_failed(RecordingFailure::ApplicationScratchOverflow,
                       value.sequence, value.applied_sample);
      dropped_applications_.fetch_add(1, std::memory_order_relaxed);
      return false;
    }
    block_applications_[block_application_count_++] = value;
    return true;
  }
  bool held_note(std::uint64_t token, NoteTarget &out) const noexcept {
    for (const auto &t : touches_)
      if (t.token == token) {
        for (const auto &v : voices_)
          if (v.active && v.note.member == t.member) {
            out = t.original_note.member ? t.original_note : v.note;
            out.touch = token;
            out.touch_ref = t.source_touch_ref;
            out.identity = t.source_identity;
            return true;
          }
      }
    return false;
  }

  Parameters source_{}, effective_{};
  std::uint64_t cursor_ = 0, accepted_sequence_ = 0, accepted_sample_ = 0,
                last_admission_sample_ = 0, applied_sequence_ = 0,
                applied_application_ordinal_ = 0, refused_ = 0, late_ = 0,
                stolen_ = 0, dropped_readbacks_ = 0, dropped_captures_ = 0,
                clipping_ = 0, force_limited_ = 0;
  std::atomic<std::uint64_t> published_cursor_{0}, overflow_count_{0},
      published_sequence_{0}, panic_fence_{0}, published_horizon_{0};
  std::atomic<bool> emergency_{false}, capture_{false}, fault_{false};
  // Lock-free render/custody exclusion. Device start/stop marks actual callback
  // custody; a requested stop is insufficient until the OS has acknowledged it.
  std::atomic<std::uint32_t> activity_{0};
  std::atomic<bool> device_running_{false};
  std::atomic<std::uint64_t> emergency_requested_{0};
  std::uint64_t emergency_observed_ = 0, emergency_applied_sample_ = 0,
                force_zero_samples_ = 0;
  bool sustain_ = false;
  static bool
  same_route_handle(const ql::PhysicalForceRouteProgramHandle &a,
                    const ql::PhysicalForceRouteProgramHandle &b) noexcept {
    return a.driver_ref == b.driver_ref && a.target_ref == b.target_ref &&
           a.program_ref == b.program_ref &&
           a.planet_coordinate == b.planet_coordinate &&
           a.chakra_coordinate == b.chakra_coordinate &&
           a.projection_ref == b.projection_ref &&
           a.calibration_ref == b.calibration_ref &&
           a.calibration_revision == b.calibration_revision &&
           a.calibration_source_ref == b.calibration_source_ref &&
           a.calibration_standing == b.calibration_standing &&
           a.route_index == b.route_index &&
           a.preparation_seal == b.preparation_seal &&
           a.program_seal == b.program_seal &&
           a.planet_node_id == b.planet_node_id &&
           a.chakra_node_id == b.chakra_node_id &&
           a.native_planet_index == b.native_planet_index &&
           a.centre_ordinal == b.centre_ordinal &&
           a.share_numerator == b.share_numerator &&
           a.share_denominator == b.share_denominator &&
           a.source_hertz == b.source_hertz &&
           a.original_denominator_share == b.original_denominator_share &&
           a.peak_force_newtons == b.peak_force_newtons;
  }
  // Reserve every source's independently calibrated peak at gain one. The
  // original N denominator/shares and waveforms are never renormalized to
  // make room for a note. A's scalar note limiter owns only the remainder.
  static double
  route_scalar_budget(const ql::PhysicalForceRoutePortManifest &m) noexcept {
    if (m.route_count > ql::physical_max_personal_force_routes ||
        !scalar(m.max_force_newtons, 1e-12, 1e9))
      return 0;
    double reserved = 0;
    for (std::size_t i = 0; i < m.route_count; ++i) {
      if (!scalar(m.programs[i].peak_force_newtons, 0, m.max_force_newtons))
        return 0;
      reserved += m.programs[i].peak_force_newtons;
    }
    // P adds up to ten absolute terms in double precision. Keep a small
    // explicit numerical margin instead of relying on cancellation.
    const double margin =
        m.max_force_newtons * (64 * std::numeric_limits<double>::epsilon());
    const double remaining = m.max_force_newtons - reserved - margin;
    return std::isfinite(remaining) && remaining > 0 ? remaining : 0;
  }
  bool valid_route_programs(const NativeRouteProgramSet &set,
                            const PhysicalPort &port,
                            const Determination &determination) const noexcept {
    if (!port.route_manifest || !port.routes_owner || !port.routes_custody ||
        !port.advance_routes || !port.observe_routes || set.version != 1 ||
        set.program_count > ql::physical_max_personal_force_routes ||
        set.program_count != port.route_manifest->route_count ||
        !set.scalar_note_enabled || set.scalar_note_gain != 1 ||
        route_scalar_budget(set.manifest) <= 0)
      return false;
    const auto &a = set.manifest, &b = *port.route_manifest;
    if (a.version != b.version || a.sample_rate != rate_ ||
        a.sample_rate != b.sample_rate || a.route_count != b.route_count ||
        a.source_basis_seal != b.source_basis_seal ||
        a.body_revision != b.body_revision ||
        a.admitted_cursor != b.admitted_cursor ||
        a.m1_revision != b.m1_revision || a.m2_generation != b.m2_generation ||
        a.m3_generation != b.m3_generation ||
        a.m3_input_generation != b.m3_input_generation ||
        a.earth_frame_node_id != b.earth_frame_node_id ||
        a.event_ref != b.event_ref || a.subject_ref != b.subject_ref ||
        a.registry_revision != b.registry_revision ||
        a.source_revision != b.source_revision ||
        a.definition_ref != b.definition_ref ||
        a.source_instance_ref != b.source_instance_ref ||
        a.determination_ref != b.determination_ref ||
        a.preparation_ref != b.preparation_ref || a.state_ref != b.state_ref ||
        a.eigenbasis_identity != b.eigenbasis_identity ||
        a.m1_coordinate != b.m1_coordinate ||
        a.m2_writer_coordinate != b.m2_writer_coordinate ||
        a.native_basis_sha256 != b.native_basis_sha256 ||
        a.m3_state_sha256 != b.m3_state_sha256 ||
        a.m1_pratibimba != b.m1_pratibimba ||
        a.m2_pratibimba != b.m2_pratibimba || a.tick12 != b.tick12 ||
        a.degree720 != b.degree720 || a.temporal_phase != b.temporal_phase ||
        a.scalar_note_enabled != b.scalar_note_enabled ||
        a.scalar_note_gain != b.scalar_note_gain ||
        a.legacy_native_scalar_enabled || a.legacy_native_scalar_gain != 0 ||
        a.max_force_newtons != b.max_force_newtons ||
        a.max_force_newtons != port.max_force_newtons ||
        a.event_ref != determination.identity.event ||
        a.subject_ref != determination.identity.subject ||
        a.m1_revision != determination.identity.m1_revision ||
        a.m2_generation != determination.identity.m2_generation ||
        a.body_revision != determination.body_revision ||
        a.m1_coordinate != determination.m1_coordinate ||
        a.m1_pratibimba != (determination.m1_face == 1) ||
        a.m2_writer_coordinate != determination.m2_writer ||
        a.m2_pratibimba != (determination.m2_face == 1) ||
        a.tick12 != determination.tick12 ||
        a.degree720 != determination.degree720 ||
        a.temporal_phase != determination.tick12 / 6 ||
        a.determination_ref != determination.native_receipt_ref ||
        !ql_m_live_accepts_base(determination.registry_revision.data()) ||
        std::strcmp(a.registry_revision.data(),
                    ql_m_live_registry_revision()) != 0 ||
        std::strcmp(a.source_revision.data(), ql_m_live_source_revision()) !=
            0 ||
        a.preparation_ref != port.preparation || a.state_ref != port.state)
      return false;
    for (std::size_t i = 0; i < set.program_count; ++i) {
      const auto &p = set.programs[i];
      if (!same_route_handle(a.programs[i], b.programs[i]) ||
          !same_route_handle(p.handle, b.programs[i]) ||
          p.handle.route_index != i || !valid_ref(p.phase_source_ref) ||
          p.phase_source_ref != a.m1_coordinate ||
          p.waveform != NativeRouteWaveform::Sinusoid ||
          !scalar(p.target_gain, 0, 1) || !scalar(p.effective_gain, 0, 1) ||
          !scalar(p.sine, -1, 1) || !scalar(p.cosine, -1, 1) ||
          std::abs(p.sine * p.sine + p.cosine * p.cosine - 1) > 1e-10 ||
          !scalar(p.handle.source_hertz, 1, .45 * rate_) ||
          !scalar(p.handle.peak_force_newtons, 0, port.max_force_newtons))
        return false;
    }
    return true;
  }
  bool valid_route_programs(const NativeRouteProgramSet &set) const noexcept {
    return valid_route_programs(set, body_, determination_);
  }
  void prepare_route_steps() noexcept {
    for (std::size_t i = 0; i < route_programs_.program_count; ++i) {
      const double step =
          tau * route_programs_.programs[i].handle.source_hertz / rate_;
      route_step_sine_[i] = std::sin(step);
      route_step_cosine_[i] = std::cos(step);
    }
  }
  static bool scalar(double x, double lo, double hi) noexcept {
    return std::isfinite(x) && x >= lo && x <= hi;
  }
  static bool same_lineage(const Identity &a, const Identity &b) noexcept {
    return a.instance == b.instance && a.event == b.event &&
           a.subject == b.subject;
  }
  static bool valid_requested_timing(bool present, std::uint64_t requested,
                                     std::uint64_t admitted,
                                     bool late) noexcept {
    return present ? requested <= admitted && (requested == admitted || late)
                   : requested == 0;
  }
  static bool valid_native_clock(const NativeClockMetadata &c) noexcept {
    if (!c.epoch)
      return !c.anchor_ordinal && !c.trigger_host_ticks &&
             !c.admitted_host_ticks && c.mapping_uncertainty_samples == 0;
    return c.anchor_ordinal && c.trigger_host_ticks &&
           c.admitted_host_ticks >= c.trigger_host_ticks &&
           scalar(c.mapping_uncertainty_samples, 1, 1440);
  }
  bool valid_determination(const Determination &d) const noexcept {
    if (!valid_ref(d.identity.instance) || !valid_ref(d.identity.event) ||
        !valid_ref(d.identity.subject) || !valid_ref(d.m1_coordinate) ||
        !valid_ref(d.registry_revision) || !valid_ref(d.source_revision) ||
        !valid_ref(d.relation_plan_ref) || !valid_ref(d.tuning_ref) ||
        !valid_ref(d.native_receipt_ref) ||
        !valid_ref(d.body_preparation_ref) || !valid_ref(d.body_state_ref) ||
        !valid_ref(d.m2_writer) || d.m1_face > 1 || d.m2_face != 1 ||
        std::strcmp(d.m2_writer.data(), "#2-1") != 0 || d.tick12 >= 12 ||
        d.lens12 >= 12 || d.basis > 1 || d.context_frame < 1 ||
        d.context_frame > 7 || d.degree720 >= 720 || !d.tuning_available)
      return false;
    for (double hz : d.audio_octet_hz)
      if (!scalar(hz, 0.001, rate_ * 0.45))
        return false;
    const auto &p = d.excitation;
    if (!valid_ref(p.policy_ref) || !valid_ref(p.standing) ||
        unsigned(p.scaling) > 1 ||
        !scalar(p.reference_hertz, 0.001, rate_ * 0.45) ||
        !scalar(p.root_linear, 0, 1) || !scalar(p.octet_linear, 0, 1) ||
        p.root_linear + p.octet_linear > 1)
      return false;
    double weights = 0;
    for (double w : p.weights) {
      if (!scalar(w, 0, 1))
        return false;
      weights += w;
    }
    if (std::abs(weights - 1) > 1e-12)
      return false;
    for (const auto &n : d.nodal_quartet)
      if ((n.position != 0 && n.position != 5) || n.face > 1 || n.m < 1 ||
          n.m > 12 || n.n < 1 || n.n > 12)
        return false;
    const auto *source = ql_m_live_resolve(d.m1_coordinate.data());
    if (!source || source->root_position != 1 ||
        !ql_m_live_accepts_base(d.registry_revision.data()) ||
        d.identity.m2_generation > 9007199254740991ULL)
      return false;
    return true;
  }
  const Determination &source_at(std::uint64_t sample) const noexcept {
    const Determination *selected = &source_schedule_[0].value;
    for (std::size_t i = 1; i < source_schedule_size_; ++i)
      if (source_schedule_[i].from_sample <= sample)
        selected = &source_schedule_[i].value;
    return *selected;
  }
  void retire_source_schedule(std::uint64_t cursor) noexcept {
    std::size_t current = 0;
    for (std::size_t i = 1; i < source_schedule_size_; ++i)
      if (source_schedule_[i].from_sample <= cursor)
        current = i;
    if (current) {
      for (std::size_t i = current; i < source_schedule_size_; ++i)
        source_schedule_[i - current] = source_schedule_[i];
      source_schedule_size_ -= current;
    }
  }
  bool valid_note(const NoteTarget &n,
                  const Determination &source) const noexcept {
    if (!(n.identity == source.identity) ||
        n.source_coordinate != source.m1_coordinate ||
        n.tuning_ref != source.tuning_ref || n.source_face != source.m1_face ||
        !valid_ref(n.touch_ref) || !n.member || !n.touch || n.key >= 12 ||
        n.position != n.key / 2 || n.coordinate_face != n.key % 2 ||
        n.pitch_class >= 12 || !scalar(n.hertz, 0.001, rate_ * 0.45) ||
        !scalar(n.fundamental_hz, 0.001, rate_ * 0.45) ||
        !scalar(n.phase_sin, -1, 1) || !scalar(n.phase_cos, -1, 1) ||
        std::abs(n.phase_sin * n.phase_sin + n.phase_cos * n.phase_cos - 1) >
            1e-10)
      return false;
    if (n.exact_ratio) {
      if (!n.ratio_numerator || !n.ratio_denominator)
        return false;
      const double target = n.fundamental_hz * (double(n.ratio_numerator) /
                                                double(n.ratio_denominator));
      if (!std::isfinite(target) ||
          std::abs(target - n.hertz) > 1e-11 * n.hertz)
        return false;
    } else if (n.ratio_numerator || n.ratio_denominator)
      return false;
    return true;
  }
  static bool valid_parameter(Parameter p, double x, unsigned rate) noexcept {
    switch (p) {
    case Parameter::ForceNewtons:
      return scalar(x, 0, 100);
    case Parameter::AttackSeconds:
      return scalar(x, 0.0001, 10);
    case Parameter::ReleaseSeconds:
      return scalar(x, 0.001, 30);
    case Parameter::CutoffHertz:
      return scalar(x, 1, rate * 0.45);
    case Parameter::MasterLinear:
    case Parameter::BodyLinear:
    case Parameter::MonitorLinear:
      return scalar(x, 0, 1);
    }
    return false;
  }
  static double &field(Parameters &p, Parameter parameter) noexcept {
    switch (parameter) {
    case Parameter::ForceNewtons:
      return p.force_newtons;
    case Parameter::AttackSeconds:
      return p.attack_seconds;
    case Parameter::ReleaseSeconds:
      return p.release_seconds;
    case Parameter::CutoffHertz:
      return p.cutoff_hertz;
    case Parameter::MasterLinear:
      return p.master_linear;
    case Parameter::BodyLinear:
      return p.body_linear;
    case Parameter::MonitorLinear:
      return p.monitor_linear;
    }
    return p.master_linear;
  }
  void clear() noexcept {
    voices_.fill({});
    touches_.fill({});
    tails_.fill({});
    sustain_ = false;
  }
  void fence_attacks(std::uint64_t sequence) noexcept {
    auto previous = panic_fence_.load(std::memory_order_relaxed);
    while (previous < sequence &&
           !panic_fence_.compare_exchange_weak(previous, sequence,
                                               std::memory_order_release,
                                               std::memory_order_relaxed)) {
    }
  }
  bool operation_before(std::size_t a, std::size_t b) const noexcept {
    const auto &x = pending_operations_[a].operation;
    const auto &y = pending_operations_[b].operation;
    return x.sample < y.sample ||
           (x.sample == y.sample && x.sequence < y.sequence);
  }
  bool prepare_operation(const Operation &operation) noexcept {
    if (heap_size_ == queue_capacity)
      return false;
    std::size_t index = 0;
    while (pending_operations_[index].active)
      ++index;
    pending_operations_[index] = PendingOperation{true, operation};
    auto child = heap_size_++;
    operation_heap_[child] = index;
    while (child) {
      const auto parent = (child - 1) / 2;
      if (!operation_before(operation_heap_[child], operation_heap_[parent]))
        break;
      std::swap(operation_heap_[child], operation_heap_[parent]);
      child = parent;
    }
    return true;
  }
  void consume_operation() noexcept {
    pending_operations_[operation_heap_[0]].active = false;
    --heap_size_;
    if (!heap_size_)
      return;
    operation_heap_[0] = operation_heap_[heap_size_];
    std::size_t parent = 0;
    for (;;) {
      auto child = parent * 2 + 1;
      if (child >= heap_size_)
        break;
      if (child + 1 < heap_size_ &&
          operation_before(operation_heap_[child + 1], operation_heap_[child]))
        ++child;
      if (!operation_before(operation_heap_[child], operation_heap_[parent]))
        break;
      std::swap(operation_heap_[child], operation_heap_[parent]);
      parent = child;
    }
  }
  void release(Voice &voice) noexcept {
    if (voice.release)
      return;
    voice.release = true;
    voice.release_remaining =
        std::uint32_t(std::ceil(source_.release_seconds * rate_));
  }
  void refresh(Voice &voice) noexcept {
    bool held = false;
    double velocity = 0, pressure = 0;
    for (const auto &t : touches_)
      if (t.token && t.member == voice.note.member) {
        held = true;
        velocity = std::max(velocity, t.velocity);
        pressure = std::max(pressure, t.pressure);
      }
    if (held) {
      voice.velocity = velocity;
      voice.pressure = pressure;
    } else if (!sustain_)
      release(voice);
  }
  void retire(Voice &voice) noexcept {
    auto slot = std::min_element(
        tails_.begin(), tails_.end(),
        [](const Tail &a, const Tail &b) { return a.left < b.left; });
    // A fixed 64-sample steal fade is declared click-control, not body decay.
    *slot = Tail{voice, 64};
    for (auto &t : touches_)
      if (t.member == voice.note.member)
        t = {};
    voice = {};
    ++stolen_;
  }
  bool apply(const Operation &op) noexcept {
    // Maximum acknowledged sequence; release admission can overtake a
    // future automation sequence without moving that automation's sample.
    applied_sequence_ = std::max(applied_sequence_, op.sequence);
    switch (op.kind) {
    case Kind::Panic:
      fence_attacks(op.sequence);
      touches_.fill({});
      sustain_ = false;
      for (auto &v : voices_)
        if (v.active)
          release(v);
      break;
    case Kind::Parameter:
      field(source_, op.parameter) = op.value;
      break;
    case Kind::Determination:
      determination_ = op.determination;
      break;
    case Kind::Sustain:
      sustain_ = op.value == 1;
      if (!sustain_)
        for (auto &v : voices_)
          if (v.active)
            refresh(v);
      break;
    case Kind::NoteOff:
      for (auto &t : touches_)
        if (t.token == op.touch) {
          const auto member = t.member;
          t = {};
          for (auto &v : voices_)
            if (v.active && v.note.member == member)
              refresh(v);
          break;
        }
      break;
    case Kind::Expression: {
      bool matched = false;
      for (auto &t : touches_)
        if (t.token == op.touch) {
          matched = true;
          t.pressure = op.value;
          for (auto &v : voices_)
            if (v.active && v.note.member == t.member) {
              refresh(v);
              if (op.pitch_hz > 0) {
                v.target_frequency = op.pitch_hz;
                v.note = op.note;
              }
            }
        }
      if (!matched) {
        ++refused_;
        return false;
      }
      break;
    }
    case Kind::NoteOn: {
      if (op.sequence <= panic_fence_.load(std::memory_order_relaxed))
        return false;
      for (const auto &t : touches_)
        if (t.token == op.note.touch) {
          ++refused_;
          return false;
        }
      auto touch = std::find_if(touches_.begin(), touches_.end(),
                                [](const Touch &t) { return !t.token; });
      if (touch == touches_.end()) {
        ++refused_;
        return false;
      }
      auto voice =
          std::find_if(voices_.begin(), voices_.end(), [&](const Voice &v) {
            return v.active && !v.release && v.note.member == op.note.member;
          });
      if (voice == voices_.end()) {
        voice = std::find_if(voices_.begin(), voices_.end(),
                             [](const Voice &v) { return !v.active; });
        if (voice == voices_.end()) {
          voice = std::min_element(voices_.begin(), voices_.end(),
                                   [](const Voice &a, const Voice &b) {
                                     if (a.release != b.release)
                                       return a.release;
                                     if (a.release && a.envelope != b.envelope)
                                       return a.envelope < b.envelope;
                                     return a.born < b.born;
                                   });
          retire(*voice);
        }
        *voice = Voice{};
        voice->active = true;
        voice->note = op.note;
        voice->born = op.sample;
        voice->sine = op.note.phase_sin;
        voice->cosine = op.note.phase_cos;
        voice->frequency = voice->target_frequency = op.note.hertz;
        for (std::size_t j = 0; j < 8; ++j) {
          voice->octet_sine[j] = op.note.phase_sin;
          voice->octet_cosine[j] = op.note.phase_cos;
          voice->octet_frequency[j] =
              std::clamp(octet_target(*voice, j), 0.001, rate_ * 0.45);
          voice->octet_weight[j] = determination_.excitation.weights[j];
        }
        voice->root_gain = determination_.excitation.root_linear;
        voice->octet_gain = determination_.excitation.octet_linear;
      } else if (voice->note.key != op.note.key ||
                 voice->note.register_octave != op.note.register_octave ||
                 std::abs(voice->target_frequency - op.note.hertz) > 1e-10) {
        ++refused_;
        return false;
      }
      *touch = Touch{op.note.touch,     op.note.member,   op.value, 1,
                     op.note.touch_ref, op.note.identity, op.note};
      refresh(*voice);
      break;
    }
    }
    return true;
  }
  bool apply_release(const ReleaseOperation &op) noexcept {
    if (!same_lineage(op.identity, determination_.identity)) {
      ++refused_;
      return false;
    }
    applied_sequence_ = std::max(applied_sequence_, op.sequence);
    if (op.kind == Kind::Panic) {
      fence_attacks(op.sequence);
      touches_.fill({});
      sustain_ = false;
      for (auto &v : voices_)
        if (v.active)
          release(v);
    } else if (op.kind == Kind::Sustain) {
      sustain_ = false;
      for (auto &v : voices_)
        if (v.active)
          refresh(v);
    } else {
      for (auto &t : touches_)
        if (t.token == op.touch) {
          const auto member = t.member;
          t = {};
          for (auto &v : voices_)
            if (v.active && v.note.member == member)
              refresh(v);
          break;
        }
    }
    return true;
  }
  double octet_target(const Voice &v, std::size_t j) const noexcept {
    const auto &policy = determination_.excitation;
    return determination_.audio_octet_hz[j] *
           (policy.scaling == OctetScaling::AbsoluteBus
                ? 1.0
                : v.target_frequency / policy.reference_hertz);
  }
  double sample_voice(Voice &v, double smoothing,
                      double filter_coefficient) noexcept {
    if (!v.active)
      return 0;
    v.frequency += (v.target_frequency - v.frequency) * smoothing;
    const double angle = tau * v.frequency / rate_, c = std::cos(angle),
                 s = std::sin(angle);
    const double next_sine = v.sine * c + v.cosine * s;
    v.cosine = v.cosine * c - v.sine * s;
    v.sine = next_sine;
    // Renormalize this source quadrature every sample; no phase reset on edit.
    const double norm = std::hypot(v.sine, v.cosine);
    v.sine /= norm;
    v.cosine /= norm;
    if (v.release) {
      if (!v.release_remaining) {
        v = {};
        return 0;
      }
      v.envelope *=
          double(v.release_remaining - 1) / double(v.release_remaining);
      --v.release_remaining;
    } else
      v.envelope =
          std::min(1.0, v.envelope + 1.0 / (effective_.attack_seconds * rate_));
    v.root_gain +=
        (determination_.excitation.root_linear - v.root_gain) * smoothing;
    v.octet_gain +=
        (determination_.excitation.octet_linear - v.octet_gain) * smoothing;
    double program = v.root_gain * v.sine;
    for (std::size_t j = 0; j < 8; ++j) {
      const double target = octet_target(v, j);
      // Explicit anti-alias policy: suppress components outside the
      // admitted band, retaining their phase at the bounded frequency.
      const bool audible = target >= 0.001 && target <= rate_ * 0.45;
      v.octet_frequency[j] +=
          (std::clamp(target, 0.001, rate_ * 0.45) - v.octet_frequency[j]) *
          smoothing;
      const double a = tau * v.octet_frequency[j] / rate_, oc = std::cos(a),
                   os = std::sin(a);
      const double ns = v.octet_sine[j] * oc + v.octet_cosine[j] * os;
      v.octet_cosine[j] = v.octet_cosine[j] * oc - v.octet_sine[j] * os;
      v.octet_sine[j] = ns;
      const double on = std::hypot(v.octet_sine[j], v.octet_cosine[j]);
      v.octet_sine[j] /= on;
      v.octet_cosine[j] /= on;
      v.octet_weight[j] +=
          (determination_.excitation.weights[j] - v.octet_weight[j]) *
          smoothing;
      if (audible)
        program += v.octet_gain * v.octet_weight[j] * v.octet_sine[j];
    }
    const double raw = program * v.envelope * v.velocity * v.pressure;
    v.filter += (raw - v.filter) * filter_coefficient;
    return v.filter;
  }

public:
  Engine(Determination determination, unsigned sample_rate, PhysicalPort body,
         Parameters parameters = {})
      : determination_(determination), producer_determination_(determination),
        producer_identity_(determination.identity), rate_(sample_rate),
        body_(body), source_(parameters), effective_(parameters) {
    if (rate_ < 8000 || rate_ > 192000 ||
        !valid_determination(determination_) || !body_.owner ||
        !body_.advance || !body_.revision || !body_.cursor || !body_.observe ||
        body_.revision(body_.owner) != determination_.body_revision ||
        body_.event != determination_.identity.event ||
        body_.subject != determination_.identity.subject ||
        body_.preparation != determination_.body_preparation_ref ||
        body_.state != determination_.body_state_ref ||
        body_.sample_rate != rate_ ||
        !scalar(body_.max_force_newtons, 1e-12, 1e9))
      throw std::invalid_argument("disconnected prepared native audio/body");
    for (unsigned id = 0; id <= unsigned(Parameter::MonitorLinear); ++id)
      if (!valid_parameter(Parameter(id), field(source_, Parameter(id)), rate_))
        throw std::invalid_argument("invalid performance parameter");
    cursor_ = body_.cursor(body_.owner);
    accepted_sample_ = cursor_;
    published_cursor_.store(cursor_);
    published_horizon_.store(cursor_);
    source_schedule_[0] = ScheduledDetermination{determination_, cursor_};
  }
  // Native control descriptor; immutable port preparation determines the
  // scalar note's available Newton scale, rather than a UI-computed range.
  double force_parameter_maximum() const noexcept {
    return body_.route_manifest
               ? std::min(100.0, route_scalar_budget(*body_.route_manifest))
               : 100.0;
  }
  friend class NativeOutputClock;

private:
  Result enqueue_native_admitted(Operation op) noexcept {
    op.requested_sample = op.sample;
    op.has_requested_sample = true;
    op.sample = std::max(op.sample, admission_horizon());
    return enqueue_impl(op, true);
  }

public:
  // Single control owner only. Failed admission consumes no source sequence.
  // A full queue additionally requests safe all-notes-off out of band, so a
  // lost NoteOff can never leave a permanently sounding excitation.
  NativeQueueAdmission enqueue_with_receipt(const Operation &op) noexcept {
    NativeQueueAdmission receipt{};
    if (op.has_requested_sample || op.requested_sample ||
        op.native_clock.epoch || op.native_clock.anchor_ordinal ||
        op.native_clock.trigger_host_ticks ||
        op.native_clock.admitted_host_ticks ||
        op.native_clock.mapping_uncertainty_samples) {
      receipt.result_ = Result::Invalid;
      return receipt;
    }
    receipt.result_ = enqueue_impl(op, false, &receipt);
    return receipt;
  }
  Result enqueue(const Operation &op) noexcept {
    return enqueue_with_receipt(op).result();
  }

private:
  Result enqueue_impl(const Operation &op, bool native_gesture,
                      NativeQueueAdmission *receipt = nullptr) noexcept {
    if (activity_.load(std::memory_order_acquire) == 2)
      return Result::Unavailable;
    if (fault_.load(std::memory_order_acquire))
      return Result::Unavailable;
    const auto cursor = published_cursor_.load(std::memory_order_acquire);
    retire_source_schedule(cursor);
    if (accepted_sequence_ == std::numeric_limits<std::uint64_t>::max())
      return Result::Exhausted;
    if (unsigned(op.kind) > unsigned(Kind::Determination) ||
        !valid_native_clock(op.native_clock) ||
        (native_gesture && !op.native_clock.epoch))
      return Result::Invalid;
    if (op.sequence != accepted_sequence_ + 1)
      return Result::Order;
    const bool late = op.sample < cursor;
    const auto horizon = published_horizon_.load(std::memory_order_acquire);
    const bool critical = op.kind == Kind::NoteOff || op.kind == Kind::Panic ||
                          (op.kind == Kind::Sustain && op.value == 0);
    const auto &source = source_at(std::max(op.sample, cursor));
    if (critical) {
      if (!same_lineage(op.identity, producer_identity_) ||
          op.identity.m1_revision > producer_identity_.m1_revision ||
          op.identity.m2_generation > producer_identity_.m2_generation)
        return Result::Stale;
    } else if (!(op.identity == source.identity))
      return Result::Stale;
    if ((late || op.sample < horizon) && !critical && !native_gesture)
      return Result::Late;
    const auto limit = cursor > std::numeric_limits<std::uint64_t>::max() -
                                    std::uint64_t(rate_) * 2
                           ? std::numeric_limits<std::uint64_t>::max()
                           : cursor + std::uint64_t(rate_) * 2;
    if (op.sample > limit)
      return Result::Late;
    Operation admitted = op;
    admitted.requested_sample =
        native_gesture ? op.requested_sample : op.sample;
    admitted.has_requested_sample = true;
    admitted.late_admitted =
        late || admitted.requested_sample < admitted.sample;
    if (late)
      admitted.sample = cursor;
    admitted.late_admitted = admitted.requested_sample < admitted.sample;
    const bool separate_release = critical;
    if ((op.kind == Kind::NoteOn &&
         (!valid_note(op.note, source) || !scalar(op.value, 0, 1))) ||
        (op.kind == Kind::NoteOff && !op.touch) ||
        (op.kind == Kind::Expression &&
         (!op.touch || !scalar(op.value, 0, 1) ||
          !scalar(op.pitch_hz, 0, rate_ * 0.45) ||
          (op.pitch_hz > 0 &&
           (!valid_note(op.note, source) || op.note.touch != op.touch ||
            op.note.hertz != op.pitch_hz)))) ||
        (op.kind == Kind::Sustain && op.value != 0 && op.value != 1) ||
        (op.kind == Kind::Parameter &&
         !valid_parameter(op.parameter, op.value, rate_)))
      return Result::Invalid;
    if (op.kind == Kind::Parameter && op.parameter == Parameter::ForceNewtons &&
        has_route_programs_ &&
        op.value > route_scalar_budget(route_programs_.manifest))
      return Result::Invalid;
    // A source turn while N9 is installed requires the paired native
    // after-basis/routes transaction. Never carry an old N programme witness
    // into a new source simply because the numerical body pointer is equal.
    if (op.kind == Kind::Determination && has_route_programs_)
      return Result::Unavailable;
    if (op.kind == Kind::Determination &&
        (!valid_determination(op.determination) ||
         !same_lineage(op.determination.identity, producer_identity_) ||
         op.determination.identity.m2_generation <=
             producer_identity_.m2_generation ||
         op.determination.identity.m1_revision <
             producer_identity_.m1_revision ||
         op.determination.body_revision !=
             producer_determination_.body_revision ||
         op.determination.body_state_ref !=
             producer_determination_.body_state_ref ||
         op.determination.body_preparation_ref !=
             producer_determination_.body_preparation_ref))
      return Result::Invalid;
    if (op.kind == Kind::Determination) {
      if (op.sample < source_schedule_[source_schedule_size_ - 1].from_sample)
        return Result::Order;
      if (source_schedule_size_ == source_schedule_.size())
        return Result::Exhausted;
    }
    const bool queued =
        separate_release
            ? releases_.push(ReleaseOperation{
                  op.kind, op.identity, op.sequence, admitted.sample, op.touch,
                  admitted.late_admitted, admitted.native_clock,
                  admitted.requested_sample, true})
            : operations_.push(admitted);
    if (!queued) {
      overflow_count_.fetch_add(1);
      emergency_.store(true, std::memory_order_release);
      return Result::Overflow;
    }
    if (receipt) {
      receipt->operation_ = admitted;
      receipt->source_ = source;
      receipt->cursor_ = cursor;
      receipt->horizon_ = horizon;
    }
    accepted_sequence_ = op.sequence;
    last_admission_sample_ = admitted.sample;
    if (!separate_release)
      accepted_sample_ = admitted.sample;
    published_sequence_.store(accepted_sequence_, std::memory_order_release);
    if (op.kind == Kind::Determination) {
      producer_identity_ = op.determination.identity;
      producer_determination_ = op.determination;
      source_schedule_[source_schedule_size_++] =
          ScheduledDetermination{op.determination, op.sample};
    }
    return Result::Accepted;
  }

public:
  std::uint64_t admission_horizon() const noexcept {
    return std::max(published_cursor_.load(std::memory_order_acquire),
                    published_horizon_.load(std::memory_order_acquire));
  }
  std::uint64_t last_admitted_sample() const noexcept {
    return last_admission_sample_;
  }
  bool install_routes_port(PhysicalPort port, const NativeRouteProgramSet &set,
                           const StoppedCustody &guard,
                           std::uint64_t expected_cursor) noexcept {
    if (guard.owner_ != this || activity_.load() != 2 ||
        device_running_.load() || cursor_ != expected_cursor ||
        port.owner != body_.owner ||
        port.custody.get() != body_.custody.get() ||
        port.advance != body_.advance || port.observe != body_.observe ||
        port.cursor != body_.cursor || port.revision != body_.revision ||
        port.event != body_.event || port.subject != body_.subject ||
        port.preparation != body_.preparation || port.state != body_.state ||
        port.sample_rate != rate_ ||
        port.max_force_newtons != body_.max_force_newtons ||
        combined_control_revision_ ==
            std::numeric_limits<std::uint64_t>::max() ||
        set.manifest.admitted_cursor != expected_cursor)
      return false;
    // Stopped exclusive control custody: no physical, musical or queue state
    // mutates during candidate qualification. Retired numerical-route handles
    // remain retained by previous until this acknowledged control call returns.
    if (!valid_route_programs(set, port, determination_))
      return false;
    body_ = std::move(port);
    route_programs_ = set;
    has_route_programs_ = true;
    prepare_route_steps();
    ++combined_control_revision_;
    return true;
  }
  bool install_route_programs(const NativeRouteProgramSet &set,
                              const StoppedCustody &guard,
                              std::uint64_t expected_cursor) noexcept {
    if (guard.owner_ != this || activity_.load() != 2 ||
        device_running_.load() || cursor_ != expected_cursor ||
        set.manifest.admitted_cursor != expected_cursor ||
        combined_control_revision_ ==
            std::numeric_limits<std::uint64_t>::max() ||
        !valid_route_programs(set))
      return false;
    route_programs_ = set;
    has_route_programs_ = true;
    prepare_route_steps();
    ++combined_control_revision_;
    return true;
  }
  bool device_callbacks_running() const noexcept {
    return device_running_.load(std::memory_order_acquire);
  }
  double parameter_smoothing_time_constant_samples() const noexcept {
    return parameter_smoothing_seconds * rate_;
  }
  double parameter_smoothing_coefficient() const noexcept {
    return -std::expm1(-1.0 / parameter_smoothing_time_constant_samples());
  }

  // Out-of-band panic cancels previously queued attacks while preserving
  // prepared source changes and physical tails. A subsequent human attack
  // gets a higher sequence and can play normally.
  std::uint64_t request_panic() noexcept {
    fence_attacks(published_sequence_.load(std::memory_order_acquire));
    auto previous = emergency_requested_.load(std::memory_order_relaxed);
    if (previous == std::numeric_limits<std::uint64_t>::max())
      return 0; // Explicit owner refusal: no wrapped release acknowledgement.
    emergency_requested_.store(previous + 1, std::memory_order_release);
    emergency_.store(true, std::memory_order_release);
    return previous + 1;
  }
  void enable_capture(bool enabled) noexcept {
    capture_.store(enabled, std::memory_order_release);
  }
  std::uint64_t samples_elapsed() const noexcept {
    return published_cursor_.load(std::memory_order_acquire);
  }
  unsigned sample_rate() const noexcept { return rate_; }
  // Serial control owner only: the admitted source for the NEXT native
  // sample, including a determination already due at a block boundary.
  const Determination &current_source() const noexcept {
    return source_at(published_cursor_.load(std::memory_order_acquire));
  }
  const Determination &source_for_native_admission() const noexcept {
    return source_at(admission_horizon());
  }
  // Numerical preflight of separately source-admitted AFTER catalog entries.
  // Does not change currentness, source authority, body or a playing target.
  bool validate_candidate_note_target(
      const NoteTarget &note, const Determination &candidate) const noexcept {
    return valid_determination(candidate) && valid_note(note, candidate);
  }
  bool validate_note_target(const NoteTarget &note) const noexcept {
    return valid_note(
        note, source_at(published_cursor_.load(std::memory_order_acquire)));
  }
  bool validate_note_target_for_native_admission(
      const NoteTarget &note) const noexcept {
    return valid_note(note, source_for_native_admission());
  }
  bool has_physical_custody() const noexcept { return bool(body_.custody); }
  bool owns_physical_owner(const void *owner) const noexcept {
    return body_.owner == owner;
  }
  bool begin_device_callbacks() noexcept {
    if (activity_.load(std::memory_order_acquire) == 2)
      return false;
    bool expected = false;
    if (!device_running_.compare_exchange_strong(expected, true,
                                                 std::memory_order_acq_rel))
      return false;
    if (activity_.load(std::memory_order_acquire) == 2) {
      device_running_.store(false, std::memory_order_release);
      return false;
    }
    return true;
  }
  void end_device_callbacks_after_stop() noexcept {
    device_running_.store(false, std::memory_order_release);
  }
  StoppedCustody acquire_stopped_custody() noexcept {
    if (device_running_.load(std::memory_order_acquire))
      return {};
    std::uint32_t expected = 0;
    if (!activity_.compare_exchange_strong(expected, 2,
                                           std::memory_order_acq_rel))
      return {};
    if (device_running_.load(std::memory_order_acquire)) {
      activity_.store(0, std::memory_order_release);
      return {};
    }
    return StoppedCustody(this);
  }
  // Native stopped/acknowledged owner can preflight an actual P transaction.
  // Pending future source generations require a separately prepared combined
  // transaction; this finite seam refuses rather than deleting their schedule.
  bool preflight_stopped_physical_revision(
      const Determination &after, const PhysicalPort &port,
      const StoppedCustody &guard) const noexcept {
    return guard.owner_ == this && activity_.load() == 2 &&
           !device_running_.load() && source_schedule_size_ == 1 &&
           valid_determination(after) &&
           same_lineage(after.identity, determination_.identity) &&
           after.identity.m1_revision >= determination_.identity.m1_revision &&
           after.identity.m2_generation >=
               determination_.identity.m2_generation &&
           port.owner == body_.owner && port.custody == body_.custody &&
           port.advance == body_.advance && port.observe == body_.observe &&
           port.revision == body_.revision && port.cursor == body_.cursor &&
           port.sample_rate == rate_ && port.event == after.identity.event &&
           port.subject == after.identity.subject &&
           valid_ref(port.preparation) && port.state == body_.state &&
           after.body_state_ref == port.state &&
           after.body_preparation_ref == port.preparation &&
           after.body_revision > determination_.body_revision &&
           scalar(port.max_force_newtons, 1e-12, 1e9);
  }
  // Candidate-only, BEFORE the sole P projection changes its body. Fresh
  // full native source/occasion/catalog custody is supplied by the existing
  // serial owner and P typed port constructor; numerical equality is not an
  // authority grant. All programmes retain exact independent native IDs.
  bool preflight_stopped_combined_revision(
      const Determination &after, const PhysicalPort &port,
      const NativeRouteProgramSet &after_seed, const StoppedCustody &guard,
      std::uint64_t expected_cursor, PreparedCombinedRevision &out) noexcept {
    if (out.ready_ || out.owner_ || !has_route_programs_ ||
        cursor_ != expected_cursor ||
        combined_control_revision_ ==
            std::numeric_limits<std::uint64_t>::max() ||
        !preflight_stopped_physical_revision(after, port, guard) ||
        body_.revision(body_.owner) != determination_.body_revision ||
        body_.cursor(body_.owner) != cursor_ ||
        after_seed.manifest.admitted_cursor != expected_cursor ||
        !valid_route_programs(after_seed, port, after) ||
        route_programs_.program_count != after_seed.program_count)
      return false;
    // Match by the original native driver/node identity, never array slot or
    // a centre average. Geometry/projection/calibration/programme handles may
    // lawfully change after the current native source is recompiled.
    NativeRouteProgramSet continuation = after_seed;
    std::array<bool, ql::physical_max_personal_force_routes> used{};
    for (std::size_t i = 0; i < continuation.program_count; ++i) {
      auto &next = continuation.programs[i];
      const NativeRouteProgram *prior = nullptr;
      for (std::size_t j = 0; j < route_programs_.program_count; ++j) {
        const auto &old = route_programs_.programs[j];
        if (old.handle.driver_ref == next.handle.driver_ref &&
            old.handle.planet_node_id == next.handle.planet_node_id &&
            old.handle.native_planet_index == next.handle.native_planet_index &&
            old.handle.chakra_node_id == next.handle.chakra_node_id &&
            old.handle.centre_ordinal == next.handle.centre_ordinal) {
          if (prior || used[j])
            return false;
          prior = &old;
          used[j] = true;
        }
      }
      if (!prior || prior->waveform != next.waveform)
        return false;
      next.enabled = prior->enabled;
      next.target_gain = prior->target_gain;
      next.effective_gain = prior->effective_gain;
      next.sine = prior->sine;
      next.cosine = prior->cosine;
    }
    continuation.owner_suspended = route_programs_.owner_suspended;
    if (!valid_route_programs(continuation, port, after))
      return false;
    out.before_ = determination_;
    out.after_ = after;
    out.candidate_port_ = port;
    out.continuation_ = continuation;
    for (std::size_t i = 0; i < continuation.program_count; ++i) {
      const double angle =
          tau * continuation.programs[i].handle.source_hertz / rate_;
      out.step_sine_[i] = std::sin(angle);
      out.step_cosine_[i] = std::cos(angle);
    }
    out.cursor_ = cursor_;
    out.accepted_sequence_ = accepted_sequence_;
    out.applied_ordinal_ = applied_application_ordinal_;
    out.emergency_requested_ = emergency_requested_.load();
    out.control_revision_ = combined_control_revision_;
    out.owner_ = this;
    out.ready_ = true;
    return true;
  }
  // The combined native owner calls this immediately BEFORE P preflight/apply
  // in the same uninterrupted guard. It never interprets an AFTER body label
  // as actual state. Future source schedules remain explicitly refused.
  bool combined_revision_current(const PreparedCombinedRevision &candidate,
                                 const StoppedCustody &guard) const noexcept {
    return candidate.ready_ && candidate.owner_ == this &&
           guard.owner_ == this && activity_.load() == 2 &&
           !device_running_.load() && cursor_ == candidate.cursor_ &&
           accepted_sequence_ == candidate.accepted_sequence_ &&
           applied_application_ordinal_ == candidate.applied_ordinal_ &&
           emergency_requested_.load() == candidate.emergency_requested_ &&
           combined_control_revision_ == candidate.control_revision_ &&
           determination_.identity == candidate.before_.identity &&
           determination_.body_revision == candidate.before_.body_revision &&
           determination_.body_preparation_ref ==
               candidate.before_.body_preparation_ref &&
           source_schedule_size_ == 1 &&
           body_.owner == candidate.candidate_port_.owner &&
           body_.state == candidate.candidate_port_.state;
  }
  // Nofail half of the native owner's atomic P+Engine transaction. P and any
  // receiver preflights MUST all succeed before its actual q/v projection.
  // This call follows that successful projection under the SAME guard and
  // serial control custody. Misuse is a native programming fault, never a
  // partial-success receipt. No callback allocation, synthesis or clock here.
  void commit_stopped_combined_revision(PreparedCombinedRevision &candidate,
                                        const StoppedCustody &guard) noexcept {
    if (!combined_revision_current(candidate, guard) ||
        candidate.candidate_port_.revision(candidate.candidate_port_.owner) !=
            candidate.after_.body_revision ||
        candidate.candidate_port_.cursor(candidate.candidate_port_.owner) !=
            cursor_)
      std::terminate();
    std::swap(body_,
              candidate.candidate_port_); // token retains retired route custody
    route_programs_ = candidate.continuation_;
    route_step_sine_ = candidate.step_sine_;
    route_step_cosine_ = candidate.step_cosine_;
    determination_ = producer_determination_ = candidate.after_;
    producer_identity_ = candidate.after_.identity;
    source_schedule_[0] = ScheduledDetermination{candidate.after_, cursor_};
    ++combined_control_revision_;
    candidate.ready_ = false;
    // Voice/touch/phase/tail/sustain/queued operation/release/panic fences and
    // original application/journal identities stay intact. A future old-basis
    // attack is retained and gets its actual explicit applied/refused receipt;
    // source identity is never rewritten to make it current.
  }
  bool commit_stopped_physical_revision(const Determination &after,
                                        const PhysicalPort &port,
                                        const StoppedCustody &guard) noexcept {
    if (has_route_programs_ || body_.route_manifest)
      return false; // Requires the prepared combined body/receiver/routes seam.
    if (!preflight_stopped_physical_revision(after, port, guard) ||
        port.revision(port.owner) != after.body_revision ||
        port.cursor(port.owner) != cursor_)
      return false;
    body_ = port;
    determination_ = producer_determination_ = after;
    producer_identity_ = after.identity;
    source_schedule_[0] = ScheduledDetermination{after, cursor_};
    // Existing oscillator phase, touch lifetimes, queues and P q/v are
    // retained.
    return true;
  }
  // Destination is host heap custody. Never put several MiB of fixed queue
  // state on a callback or ordinary native control thread's stack.
  void write_checkpoint(Checkpoint &cp, const StoppedCustody &guard) const {
    if (guard.owner_ != this || activity_.load() != 2 ||
        body_.cursor(body_.owner) != cursor_ ||
        body_.revision(body_.owner) != determination_.body_revision)
      throw std::logic_error(
          "exclusive paired audio/body checkpoint custody required");
    cp.version = 2;
    cp.sample_rate = rate_;
    cp.determination = determination_;
    cp.producer_determination = producer_determination_;
    cp.producer_identity = producer_identity_;
    cp.source_schedule = source_schedule_;
    cp.source_schedule_size = source_schedule_size_;
    cp.voices = voices_;
    cp.touches = touches_;
    cp.tails = tails_;
    operations_.write_checkpoint(cp.operations);
    cp.pending_operations = pending_operations_;
    cp.operation_heap = operation_heap_;
    cp.heap_size = heap_size_;
    releases_.write_checkpoint(cp.releases);
    cp.pending_releases = pending_releases_;
    gesture_applications_.write_checkpoint(cp.applications);
    cp.recording = recording_status();
    cp.has_route_programs = has_route_programs_;
    cp.route_programs = route_programs_;
    cp.source = source_;
    cp.effective = effective_;
    cp.cursor = cursor_;
    cp.accepted_sequence = accepted_sequence_;
    cp.accepted_sample = accepted_sample_;
    cp.applied_sequence = applied_sequence_;
    cp.applied_application_ordinal = applied_application_ordinal_;
    cp.refused = refused_;
    cp.late = late_;
    cp.stolen = stolen_;
    cp.dropped_readbacks = dropped_readbacks_;
    cp.dropped_captures = dropped_captures_;
    cp.clipping = clipping_;
    cp.force_limited = force_limited_;
    cp.overflow_count = overflow_count_.load();
    cp.panic_fence = panic_fence_.load();
    cp.force_zero_samples = force_zero_samples_;
    cp.emergency_requested =
        emergency_requested_.load(std::memory_order_acquire);
    cp.emergency_observed = emergency_observed_;
    cp.emergency_applied_sample = emergency_applied_sample_;
    cp.emergency = emergency_.load();
    cp.capture = capture_.load();
    cp.fault = fault_.load();
    cp.sustain = sustain_;
  }
  // Compatibility API only. The retained native management owner uses the
  // destination writer / paired heap factory, avoiding a by-value temporary.
  Checkpoint checkpoint(const StoppedCustody &guard) const {
    Checkpoint cp{};
    write_checkpoint(cp, guard);
    return cp;
  }
  bool validate_checkpoint(const Checkpoint &cp, const StoppedCustody &guard,
                           std::uint64_t expected_cursor) const noexcept {
    if (guard.owner_ != this || activity_.load() != 2 ||
        device_running_.load() || cursor_ != expected_cursor ||
        cp.version != 2 || cp.sample_rate != rate_ ||
        cp.has_route_programs != bool(body_.route_manifest) ||
        (cp.has_route_programs && !valid_route_programs(cp.route_programs)) ||
        !valid_determination(cp.determination) ||
        !valid_determination(cp.producer_determination) ||
        !same_lineage(cp.determination.identity, determination_.identity) ||
        !same_lineage(cp.producer_identity, cp.determination.identity) ||
        !(cp.producer_identity == cp.producer_determination.identity) ||
        cp.determination.body_revision != body_.revision(body_.owner) ||
        cp.determination.body_preparation_ref != body_.preparation ||
        cp.determination.body_state_ref != body_.state ||
        cp.producer_determination.body_revision !=
            cp.determination.body_revision ||
        cp.producer_determination.body_preparation_ref != body_.preparation ||
        cp.producer_determination.body_state_ref != body_.state ||
        !cp.source_schedule_size ||
        cp.source_schedule_size > cp.source_schedule.size() ||
        cp.heap_size > queue_capacity ||
        !Spsc<Operation, queue_capacity>::valid_state(cp.operations) ||
        !Spsc<ReleaseOperation, 64>::valid_state(cp.releases) ||
        !Spsc<NativeGestureApplication, 256>::valid_state(cp.applications) ||
        unsigned(cp.recording.failure) >
            unsigned(RecordingFailure::ApplicationScratchOverflow) ||
        (cp.recording.failure == RecordingFailure::None &&
         (cp.recording.dropped_applications ||
          cp.recording.first_failed_sequence ||
          cp.recording.first_failed_sample)) ||
        cp.recording.first_failed_sequence > cp.accepted_sequence ||
        cp.applied_sequence > cp.accepted_sequence ||
        cp.applied_application_ordinal > cp.accepted_sequence ||
        cp.applications.write > cp.applied_application_ordinal ||
        (cp.recording.failure == RecordingFailure::None &&
         cp.applications.write != cp.applied_application_ordinal) ||
        cp.panic_fence > cp.accepted_sequence ||
        cp.force_zero_samples > cp.cursor ||
        cp.emergency_observed > cp.emergency_requested ||
        cp.emergency_applied_sample > cp.cursor ||
        (!cp.emergency_observed && cp.emergency_applied_sample) ||
        cp.determination.identity.m2_generation >
            cp.producer_identity.m2_generation ||
        cp.determination.identity.m1_revision >
            cp.producer_identity.m1_revision)
      return false;
    for (unsigned p = 0; p <= unsigned(Parameter::MonitorLinear); ++p) {
      Parameters a = cp.source, b = cp.effective;
      if (!valid_parameter(Parameter(p), field(a, Parameter(p)), rate_) ||
          !valid_parameter(Parameter(p), field(b, Parameter(p)), rate_))
        return false;
    }
    for (std::size_t i = 0; i < cp.source_schedule_size; ++i) {
      const auto &s = cp.source_schedule[i];
      if (!valid_determination(s.value) ||
          !same_lineage(s.value.identity, cp.determination.identity) ||
          s.value.body_revision != cp.determination.body_revision ||
          s.value.body_preparation_ref != body_.preparation ||
          s.value.body_state_ref != body_.state ||
          (i && (s.from_sample < cp.source_schedule[i - 1].from_sample ||
                 s.value.identity.m2_generation <=
                     cp.source_schedule[i - 1].value.identity.m2_generation ||
                 s.value.identity.m1_revision <
                     cp.source_schedule[i - 1].value.identity.m1_revision)))
        return false;
    }
    if (!(cp.source_schedule[cp.source_schedule_size - 1].value.identity ==
          cp.producer_identity))
      return false;
    auto valid_origin = [&](const Identity &id) {
      return same_lineage(id, cp.determination.identity) &&
             id.m1_revision <= cp.producer_identity.m1_revision &&
             id.m2_generation <= cp.producer_identity.m2_generation;
    };
    auto valid_saved_note = [&](const NoteTarget &note) {
      if (!valid_origin(note.identity))
        return false;
      Determination origin = cp.determination;
      origin.identity = note.identity;
      origin.m1_coordinate = note.source_coordinate;
      origin.m1_face = note.source_face;
      origin.tuning_ref = note.tuning_ref;
      return valid_determination(origin) && valid_note(note, origin);
    };
    auto valid_voice = [&](const Voice &v) {
      if (!v.active)
        return true;
      if (!valid_saved_note(v.note) || !scalar(v.sine, -1, 1) ||
          !scalar(v.cosine, -1, 1) ||
          std::abs(v.sine * v.sine + v.cosine * v.cosine - 1) > 1e-10 ||
          !scalar(v.envelope, 0, 1) || !scalar(v.velocity, 0, 1) ||
          !scalar(v.pressure, 0, 1) ||
          !scalar(v.frequency, 0.001, rate_ * 0.45) ||
          !scalar(v.target_frequency, 0.001, rate_ * 0.45) ||
          !scalar(v.filter, -1, 1) ||
          v.release_remaining > std::uint64_t(rate_) * 30 ||
          !scalar(v.root_gain, 0, 1) || !scalar(v.octet_gain, 0, 1) ||
          v.root_gain + v.octet_gain > 1 + 1e-12 || v.born > cp.cursor)
        return false;
      double weight = 0;
      for (std::size_t j = 0; j < 8; ++j) {
        if (!scalar(v.octet_frequency[j], 0.001, rate_ * 0.45) ||
            !scalar(v.octet_weight[j], 0, 1) ||
            !scalar(v.octet_sine[j], -1, 1) ||
            !scalar(v.octet_cosine[j], -1, 1) ||
            std::abs(v.octet_sine[j] * v.octet_sine[j] +
                     v.octet_cosine[j] * v.octet_cosine[j] - 1) > 1e-10)
          return false;
        weight += v.octet_weight[j];
      }
      return std::abs(weight - 1) <= 1e-10;
    };
    for (const auto &v : cp.voices)
      if (!valid_voice(v))
        return false;
    for (const auto &t : cp.tails)
      if (t.left > 64 || !valid_voice(t.voice))
        return false;
    for (std::size_t i = 0; i < max_touches; ++i)
      if (cp.touches[i].token) {
        const auto &t = cp.touches[i];
        if (!t.member || !valid_ref(t.source_touch_ref) ||
            !valid_origin(t.source_identity) || !scalar(t.velocity, 0, 1) ||
            !scalar(t.pressure, 0, 1) ||
            (t.original_note.member &&
             (!valid_saved_note(t.original_note) ||
              t.original_note.member != t.member ||
              t.original_note.touch != t.token ||
              t.original_note.touch_ref != t.source_touch_ref ||
              !(t.original_note.identity == t.source_identity))))
          return false;
        bool found = false;
        for (const auto &v : cp.voices)
          if (v.active && !v.release && v.note.member == t.member)
            found = true;
        if (!found)
          return false;
        for (std::size_t j = 0; j < i; ++j)
          if (cp.touches[j].token == t.token)
            return false;
      }
    std::array<bool, queue_capacity> seen_heap{};
    std::size_t active_pending = 0;
    for (const auto &p : cp.pending_operations)
      active_pending += p.active;
    if (active_pending != cp.heap_size)
      return false;
    for (std::size_t i = 0; i < cp.heap_size; ++i) {
      const auto slot = cp.operation_heap[i];
      if (slot >= queue_capacity || seen_heap[slot] ||
          !cp.pending_operations[slot].active)
        return false;
      seen_heap[slot] = true;
      if (i) {
        const auto &child = cp.pending_operations[slot].operation;
        const auto &parent =
            cp.pending_operations[cp.operation_heap[(i - 1) / 2]].operation;
        if (child.sample < parent.sample ||
            (child.sample == parent.sample && child.sequence < parent.sequence))
          return false;
      }
    }
    std::array<std::uint64_t, 640> ordinals{};
    std::size_t ordinals_size = 0;
    auto ordinal = [&](std::uint64_t sequence) {
      if (!sequence || sequence > cp.accepted_sequence ||
          ordinals_size == ordinals.size())
        return false;
      for (std::size_t i = 0; i < ordinals_size; ++i)
        if (ordinals[i] == sequence)
          return false;
      ordinals[ordinals_size++] = sequence;
      return true;
    };
    auto valid_operation = [&](const Operation &op) {
      if (unsigned(op.kind) > unsigned(Kind::Determination) ||
          !ordinal(op.sequence) || !valid_origin(op.identity) ||
          !valid_native_clock(op.native_clock) ||
          !valid_requested_timing(op.has_requested_sample, op.requested_sample,
                                  op.sample, op.late_admitted) ||
          (op.native_clock.epoch && op.kind != Kind::NoteOn &&
           op.kind != Kind::NoteOff && op.kind != Kind::Expression &&
           op.kind != Kind::Sustain && op.kind != Kind::Panic))
        return false;
      switch (op.kind) {
      case Kind::NoteOn:
        return valid_saved_note(op.note) && op.note.identity == op.identity &&
               scalar(op.value, 0, 1);
      case Kind::NoteOff:
        return op.touch != 0;
      case Kind::Expression:
        return op.touch && scalar(op.value, 0, 1) &&
               scalar(op.pitch_hz, 0, rate_ * 0.45) &&
               (op.pitch_hz == 0 ||
                (valid_saved_note(op.note) && op.note.identity == op.identity &&
                 op.note.hertz == op.pitch_hz && op.note.touch == op.touch));
      case Kind::Sustain:
        return op.value == 0 || op.value == 1;
      case Kind::Parameter:
        return valid_parameter(op.parameter, op.value, rate_);
      case Kind::Determination:
        return valid_determination(op.determination) &&
               valid_origin(op.determination.identity) &&
               op.determination.body_revision ==
                   cp.determination.body_revision &&
               op.determination.body_preparation_ref == body_.preparation &&
               op.determination.body_state_ref == body_.state;
      case Kind::Panic:
        return true;
      }
      return false;
    };
    auto valid_release = [&](const ReleaseOperation &op) {
      return ordinal(op.sequence) && valid_origin(op.identity) &&
             valid_native_clock(op.native_clock) &&
             valid_requested_timing(op.has_requested_sample,
                                    op.requested_sample, op.sample,
                                    op.late_admitted) &&
             (op.kind == Kind::Panic || op.kind == Kind::Sustain ||
              (op.kind == Kind::NoteOff && op.touch));
    };
    for (auto i = cp.operations.read; i < cp.operations.write; ++i)
      if (!valid_operation(cp.operations.storage[i % queue_capacity]))
        return false;
    for (const auto &p : cp.pending_operations)
      if (p.active && !valid_operation(p.operation))
        return false;
    for (auto i = cp.releases.read; i < cp.releases.write; ++i)
      if (!valid_release(cp.releases.storage[i % 64]))
        return false;
    for (const auto &p : cp.pending_releases)
      if (p.active && !valid_release(p.operation))
        return false;
    if (ordinals_size > cp.accepted_sequence - cp.applied_application_ordinal)
      return false;
    std::uint64_t previous_application_ordinal = 0;
    for (auto i = cp.applications.read; i < cp.applications.write; ++i) {
      const auto &a = cp.applications.storage[i % 256];
      if (!a.applied_application_ordinal ||
          a.applied_application_ordinal > cp.applied_application_ordinal ||
          (previous_application_ordinal &&
           a.applied_application_ordinal <= previous_application_ordinal) ||
          (cp.recording.failure == RecordingFailure::None &&
           a.applied_application_ordinal != i + 1) ||
          !valid_origin(a.identity) || !a.sequence ||
          a.sequence > cp.accepted_sequence ||
          a.applied_sample < a.admitted_sample ||
          !valid_requested_timing(a.has_requested_sample, a.requested_sample,
                                  a.admitted_sample, a.late_admitted) ||
          a.committed_cursor > cp.cursor ||
          a.committed_cursor <= a.applied_sample || !a.body_revision ||
          a.body_revision > cp.determination.body_revision ||
          !valid_ref(a.preparation_ref) ||
          a.state_ref != cp.determination.body_state_ref ||
          a.physical_event != a.identity.event ||
          a.physical_subject != a.identity.subject ||
          !valid_ref(a.physical_source_coordinate) ||
          !valid_ref(a.physical_source_revision) || !valid_ref(a.eigenbasis) ||
          !ql_m_live_resolve(a.physical_source_coordinate.data()) ||
          ql_m_live_resolve(a.physical_source_coordinate.data())
                  ->root_position != 3 ||
          a.physical_sample_rate != rate_ || !a.physical_pratibimba ||
          !valid_native_clock(a.clock) ||
          unsigned(a.kind) > unsigned(Kind::Determination) ||
          unsigned(a.parameter) > unsigned(Parameter::MonitorLinear) ||
          !std::isfinite(a.value) || !std::isfinite(a.pitch_hz) ||
          a.has_determination != (a.kind == Kind::Determination) ||
          (a.has_determination &&
           (!valid_determination(a.determined_source) ||
            !valid_origin(a.determined_source.identity) ||
            a.determined_source.identity.m2_generation <=
                a.identity.m2_generation ||
            a.determined_source.body_revision != a.body_revision ||
            a.determined_source.body_preparation_ref != a.preparation_ref ||
            a.determined_source.body_state_ref != a.state_ref)) ||
          (a.has_note &&
           (!valid_saved_note(a.note) || a.note.touch != a.touch ||
            !same_lineage(a.note.identity, a.identity) ||
            (a.kind == Kind::NoteOn && !(a.note.identity == a.identity)))))
        return false;
      previous_application_ordinal = a.applied_application_ordinal;
    }
    return true;
  }
  bool restore_checkpoint(const Checkpoint &cp, const StoppedCustody &guard,
                          std::uint64_t expected_cursor) noexcept {
    if (combined_control_revision_ == std::numeric_limits<std::uint64_t>::max())
      return false;
    if (!validate_checkpoint(cp, guard, expected_cursor) ||
        body_.cursor(body_.owner) != cp.cursor)
      return false;
    determination_ = cp.determination;
    producer_determination_ = cp.producer_determination;
    producer_identity_ = cp.producer_identity;
    source_schedule_ = cp.source_schedule;
    source_schedule_size_ = cp.source_schedule_size;
    voices_ = cp.voices;
    touches_ = cp.touches;
    tails_ = cp.tails;
    operations_.restore_checkpoint(cp.operations);
    pending_operations_ = cp.pending_operations;
    operation_heap_ = cp.operation_heap;
    heap_size_ = cp.heap_size;
    releases_.restore_checkpoint(cp.releases);
    pending_releases_ = cp.pending_releases;
    source_ = cp.source;
    effective_ = cp.effective;
    has_route_programs_ = cp.has_route_programs;
    route_programs_ = cp.route_programs;
    prepare_route_steps();
    ++combined_control_revision_;
    cursor_ = cp.cursor;
    accepted_sequence_ = cp.accepted_sequence;
    accepted_sample_ = cp.accepted_sample;
    applied_sequence_ = cp.applied_sequence;
    applied_application_ordinal_ = cp.applied_application_ordinal;
    refused_ = cp.refused;
    late_ = cp.late;
    stolen_ = cp.stolen;
    dropped_readbacks_ = cp.dropped_readbacks;
    dropped_captures_ = cp.dropped_captures;
    clipping_ = cp.clipping;
    force_limited_ = cp.force_limited;
    overflow_count_.store(cp.overflow_count);
    panic_fence_.store(cp.panic_fence);
    force_zero_samples_ = cp.force_zero_samples;
    emergency_requested_.store(cp.emergency_requested,
                               std::memory_order_release);
    emergency_observed_ = cp.emergency_observed;
    emergency_applied_sample_ = cp.emergency_applied_sample;
    emergency_.store(cp.emergency);
    capture_.store(cp.capture);
    fault_.store(cp.fault);
    sustain_ = cp.sustain;
    published_cursor_.store(cursor_, std::memory_order_release);
    published_sequence_.store(accepted_sequence_, std::memory_order_release);
    published_horizon_.store(cursor_, std::memory_order_release);
    // Observer delivery is deliberately fresh; saved future meter/capture
    // messages are not replayed as if they described the restored state.
    readbacks_.clear_stopped();
    captures_.clear_stopped();
    gesture_applications_.restore_checkpoint(cp.applications);
    dropped_applications_.store(cp.recording.dropped_applications,
                                std::memory_order_relaxed);
    recording_failure_sequence_.store(cp.recording.first_failed_sequence,
                                      std::memory_order_relaxed);
    recording_failure_sample_.store(cp.recording.first_failed_sample,
                                    std::memory_order_relaxed);
    recording_failure_.store(std::uint8_t(cp.recording.failure),
                             std::memory_order_release);
    block_application_count_ = 0;
    return true;
  }
  bool available() const noexcept {
    return !fault_.load(std::memory_order_acquire);
  }
  std::uint64_t accepted_sequence() const noexcept {
    return published_sequence_.load(std::memory_order_acquire);
  }
  RecordingStatus recording_status() const noexcept {
    RecordingStatus out{};
    out.failure =
        RecordingFailure(recording_failure_.load(std::memory_order_acquire));
    out.dropped_applications =
        dropped_applications_.load(std::memory_order_relaxed);
    if (out.failure != RecordingFailure::None) {
      out.first_failed_sequence =
          recording_failure_sequence_.load(std::memory_order_relaxed);
      out.first_failed_sample =
          recording_failure_sample_.load(std::memory_order_relaxed);
    }
    return out;
  }
  bool pop_readback(Readback &out) noexcept { return readbacks_.take(out); }
  bool pop_capture(Capture &out) noexcept { return captures_.take(out); }
  bool pop_gesture_application(NativeGestureApplication &out) noexcept {
    return gesture_applications_.take(out);
  }
  bool pop_gesture_application_up_to(NativeGestureApplication &out,
                                     std::uint64_t committed_ordinal) noexcept {
    const auto *next = gesture_applications_.peek();
    if (!next || next->applied_application_ordinal > committed_ordinal)
      return false;
    return gesture_applications_.take(out);
  }
  // Callback only; input cursor must be the actual body's sample cursor.
  // A detached/refused body emits silence, latches unavailable, and needs an
  // explicit stopped-owner recovery. It never advances a parallel clock.
  bool render(float *output, std::size_t frames,
              std::uint64_t start_sample) noexcept {
    if (!output || frames == 0 || frames > max_frames)
      return false;
    std::fill_n(output, frames, 0);
    std::uint32_t idle = 0;
    if (!activity_.compare_exchange_strong(idle, 1, std::memory_order_acq_rel))
      return false;
    struct RenderCustody {
      Engine &engine;
      ~RenderCustody() {
        if (engine.block_application_count_)
          engine.recording_failed(RecordingFailure::PhysicalCommitFailure,
                                  engine.block_applications_[0].sequence,
                                  engine.cursor_);
        engine.activity_.store(0, std::memory_order_release);
      }
    } rendering{*this};
    block_application_count_ = 0;
    if (fault_.load(std::memory_order_relaxed) || start_sample != cursor_ ||
        cursor_ > std::numeric_limits<std::uint64_t>::max() - frames)
      return false;
    if (body_.revision(body_.owner) != determination_.body_revision ||
        body_.cursor(body_.owner) != cursor_) {
      fault_.store(true, std::memory_order_release);
      return false;
    }
    published_horizon_.store(cursor_ + frames, std::memory_order_release);
    block_application_count_ = 0;
    Capture capture{};
    capture.identity = determination_.identity;
    capture.start_sample = cursor_;
    capture.body_revision = determination_.body_revision;
    capture.frames = std::uint32_t(frames);
    std::array<double, max_frames> body_gain{}, monitor_gain{}, force_scale{};
    const double smoothing = parameter_smoothing_coefficient();
    const bool releases_admitted = releases_.drain_snapshot(
        [&](const ReleaseOperation &arriving) noexcept {
          auto slot =
              std::find_if(pending_releases_.begin(), pending_releases_.end(),
                           [](const PendingRelease &p) { return !p.active; });
          if (slot == pending_releases_.end())
            return false;
          *slot = PendingRelease{true, arriving};
          return true;
        });
    const bool operations_admitted =
        operations_.drain_snapshot([&](const Operation &arriving) noexcept {
          return prepare_operation(arriving);
        });
    if (!releases_admitted || !operations_admitted) {
      overflow_count_.fetch_add(1);
      clear();
      fault_.store(true, std::memory_order_release);
      return false;
    }
    for (std::size_t i = 0; i < frames; ++i) {
      if (emergency_.exchange(false, std::memory_order_acq_rel)) {
        if (overflow_count_.load(std::memory_order_relaxed)) {
          // Queued source generations may be ahead of the callback.
          // Do not pretend that discarding them leaves a coherent
          // playing owner. Explicit stopped-owner recovery is needed.
          clear();
          fault_.store(true, std::memory_order_release);
          return false;
        }
        if (has_route_programs_)
          route_programs_.owner_suspended = true;
        emergency_observed_ =
            emergency_requested_.load(std::memory_order_acquire);
        emergency_applied_sample_ = cursor_ + i;
        touches_.fill({});
        sustain_ = false;
        for (auto &v : voices_)
          if (v.active)
            release(v);
      }
      // Bounded sample/sequence order, independent of arrival order.
      // Live playing and releases can overtake future clip/automation.
      for (;;) {
        PendingRelease *due = nullptr;
        for (auto &pending : pending_releases_)
          if (pending.active && pending.operation.sample <= cursor_ + i &&
              (!due || pending.operation.sample < due->operation.sample ||
               (pending.operation.sample == due->operation.sample &&
                pending.operation.sequence < due->operation.sequence)))
            due = &pending;
        const Operation *op =
            heap_size_ ? &pending_operations_[operation_heap_[0]].operation
                       : nullptr;
        if (op && op->sample > cursor_ + i)
          op = nullptr;
        if (!due && !op)
          break;
        if (due && (!op || due->operation.sample < op->sample ||
                    (due->operation.sample == op->sample &&
                     due->operation.sequence < op->sequence))) {
          if (due->operation.late_admitted ||
              due->operation.sample < cursor_ + i)
            ++late_;
          NativeGestureApplication application{};
          application.identity = due->operation.identity;
          application.clock = due->operation.native_clock;
          application.sequence = due->operation.sequence;
          application.admitted_sample = due->operation.sample;
          application.requested_sample = due->operation.requested_sample;
          application.has_requested_sample =
              due->operation.has_requested_sample;
          application.applied_sample = cursor_ + i;
          application.touch = due->operation.touch;
          application.kind = due->operation.kind;
          application.late_admitted = due->operation.late_admitted;
          application.has_note = held_note(application.touch, application.note);
          application.applied = apply_release(due->operation);
          if (!stage_application(application)) {
            clear();
            fault_.store(true, std::memory_order_release);
            return false;
          }
          due->active = false;
        } else {
          if (op->late_admitted || op->sample < cursor_ + i)
            ++late_;
          const bool apply_now =
              op->identity == determination_.identity &&
              (op->sample == cursor_ + i || op->native_clock.epoch ||
               op->kind == Kind::Determination || op->kind == Kind::Parameter);
          NativeGestureApplication application{};
          application.identity = op->identity;
          application.clock = op->native_clock;
          application.sequence = op->sequence;
          application.admitted_sample = op->sample;
          application.requested_sample = op->requested_sample;
          application.has_requested_sample = op->has_requested_sample;
          application.applied_sample = cursor_ + i;
          application.touch =
              op->kind == Kind::NoteOn ? op->note.touch : op->touch;
          application.kind = op->kind;
          application.parameter = op->parameter;
          if (op->kind == Kind::Determination) {
            application.determined_source = op->determination;
            application.has_determination = true;
          }
          application.value = op->value;
          application.pitch_hz = op->pitch_hz;
          application.late_admitted = op->late_admitted;
          if (op->kind == Kind::NoteOn ||
              (op->kind == Kind::Expression && op->pitch_hz > 0)) {
            application.note = op->note;
            application.has_note = true;
          } else
            application.has_note =
                held_note(application.touch, application.note);
          if (apply_now)
            application.applied = apply(*op);
          else
            ++refused_;
          if (!stage_application(application)) {
            clear();
            fault_.store(true, std::memory_order_release);
            return false;
          }
          consume_operation();
        }
      }
      for (unsigned id = 0; id <= unsigned(Parameter::MonitorLinear); ++id) {
        auto &x = field(effective_, Parameter(id));
        x += (field(source_, Parameter(id)) - x) * smoothing;
      }
      const double filter = -std::expm1(-tau * effective_.cutoff_hertz / rate_);
      double force = 0;
      for (auto &v : voices_)
        force += sample_voice(v, smoothing, filter);
      for (auto &tail : tails_)
        if (tail.left) {
          force += sample_voice(tail.voice, smoothing, filter) *
                   (double(tail.left) / 64);
          --tail.left;
        }
      // Fixed headroom bound, independent of current polyphony. Newton
      // scale is declared material policy and visible in readback.
      force *= effective_.force_newtons / double(max_voices + max_tails);
      const double scalar_budget =
          has_route_programs_ ? route_scalar_budget(route_programs_.manifest)
                              : body_.max_force_newtons;
      if (std::abs(force) > scalar_budget)
        ++force_limited_;
      force = std::clamp(force, -scalar_budget, scalar_budget);
      capture.force_newtons[i] = force;
      if (has_route_programs_) {
        capture.route_count = route_programs_.program_count;
        for (std::size_t route = 0; route < capture.route_count; ++route) {
          auto &p = route_programs_.programs[route];
          p.effective_gain += (p.target_gain - p.effective_gain) * smoothing;
          capture.route_force_newtons[route][i] =
              p.enabled && !route_programs_.owner_suspended
                  ? p.handle.peak_force_newtons *
                        std::clamp(p.sine, -1.0, 1.0) * p.effective_gain
                  : 0;
          const double sine = p.sine * route_step_cosine_[route] +
                              p.cosine * route_step_sine_[route];
          const double cosine = p.cosine * route_step_cosine_[route] -
                                p.sine * route_step_sine_[route];
          // Recursive quadrature can overshoot an exact unit peak by roundoff.
          // Its mathematical range is closed [-1,1]; keep that same range in
          // both force samples and stored checkpoint components. This neither
          // changes the calibrated Newton bound nor rephases the programme.
          p.sine = std::clamp(sine, -1.0, 1.0);
          p.cosine = std::clamp(cosine, -1.0, 1.0);
          if ((cursor_ + i + 1) % 1024 == 0) {
            const double norm = std::hypot(p.sine, p.cosine);
            p.sine = std::clamp(p.sine / norm, -1.0, 1.0);
            p.cosine = std::clamp(p.cosine / norm, -1.0, 1.0);
          }
        }
      }
      body_gain[i] = effective_.master_linear * effective_.body_linear;
      monitor_gain[i] = effective_.master_linear * effective_.monitor_linear;
      force_scale[i] = effective_.force_newtons;
    }
    capture.end_identity = determination_.identity;
    bool physical_committed = false;
    if (has_route_programs_) {
      std::array<ql::PhysicalRouteForceBlock,
                 ql::physical_max_personal_force_routes>
          blocks{};
      for (std::size_t i = 0; i < capture.route_count; ++i)
        blocks[i] = {i, route_programs_.programs[i].handle.preparation_seal,
                     capture.route_force_newtons[i].data(), 1,
                     route_programs_.programs[i].enabled &&
                         !route_programs_.owner_suspended};
      const ql::PhysicalScalarForceBlock scalar{
          capture.force_newtons.data(), route_programs_.scalar_note_gain,
          route_programs_.scalar_note_enabled};
      physical_committed = body_.advance_routes(
          body_.routes_owner, scalar, blocks.data(), capture.route_count,
          capture.pickup_linear.data(), frames, determination_.body_revision,
          cursor_);
    } else if (!body_.route_manifest) {
      physical_committed =
          body_.advance(body_.owner, capture.force_newtons.data(),
                        capture.pickup_linear.data(), frames,
                        determination_.body_revision, cursor_);
    }
    if (!physical_committed) {
      if (block_application_count_)
        recording_failed(RecordingFailure::PhysicalCommitFailure,
                         block_applications_[0].sequence, cursor_);
      clear();
      fault_.store(true, std::memory_order_release);
      return false;
    }
    cursor_ += frames;
    published_cursor_.store(cursor_, std::memory_order_release);
    ql::PhysicalSnapshot physical{};
    if (!body_.observe(body_.owner, physical, determination_.body_revision,
                       cursor_) ||
        physical.samples_elapsed != cursor_ ||
        physical.body_revision != determination_.body_revision ||
        physical.node_count > 32 || physical.sample_rate != rate_ ||
        std::strncmp(physical.event_ref.data(),
                     determination_.identity.event.data(),
                     physical.event_ref.size()) != 0 ||
        std::strncmp(physical.subject_ref.data(),
                     determination_.identity.subject.data(),
                     physical.subject_ref.size()) != 0 ||
        std::strncmp(physical.preparation_ref.data(), body_.preparation.data(),
                     physical.preparation_ref.size()) != 0 ||
        std::strncmp(physical.state_ref.data(), body_.state.data(),
                     physical.state_ref.size()) != 0) {
      clear();
      fault_.store(true, std::memory_order_release);
      return false;
    }
    ql::PhysicalForceRouteReceipt physical_routes{};
    if (has_route_programs_ &&
        (!body_.observe_routes(body_.routes_owner, physical_routes) ||
         physical_routes.start_sample != capture.start_sample ||
         physical_routes.end_sample != cursor_ ||
         physical_routes.source_basis_seal !=
             route_programs_.manifest.source_basis_seal ||
         physical_routes.route_count != route_programs_.program_count)) {
      clear();
      fault_.store(true, std::memory_order_release);
      return false;
    }
    double peak = 0, power = 0;
    for (std::size_t i = 0; i < frames; ++i) {
      const double monitor =
          force_scale[i] > 0 ? capture.force_newtons[i] / force_scale[i] : 0;
      const double raw =
          body_gain[i] * capture.pickup_linear[i] + monitor_gain[i] * monitor;
      if (!std::isfinite(raw)) {
        fault_.store(true, std::memory_order_release);
        clear();
        std::fill_n(output, frames, 0);
        return false;
      }
      if (std::abs(raw) > 0.98)
        ++clipping_;
      output[i] = float(std::clamp(raw, -0.98, 0.98));
      capture.output_linear[i] = output[i];
      peak = std::max(peak, std::abs(double(output[i])));
      power += double(output[i]) * output[i];
    }
    // This proof describes the actual committed force submitted to P. It is
    // neither silence at the output gain nor a zero displacement requirement:
    // the sole physical body may keep ringing after excitation is released.
    for (std::size_t i = 0; i < frames; ++i) {
      bool zero = capture.force_newtons[i] == 0;
      for (std::size_t route = 0; route < capture.route_count; ++route)
        zero = zero && capture.route_force_newtons[route][i] == 0;
      force_zero_samples_ =
          zero ? std::min(cursor_, force_zero_samples_ + std::uint64_t(1)) : 0;
    }
    for (std::size_t j = 0; j < block_application_count_; ++j) {
      auto &a = block_applications_[j];
      a.committed_cursor = cursor_;
      a.body_revision = determination_.body_revision;
      a.preparation_ref = body_.preparation;
      a.state_ref = body_.state;
      // Original callback-stamped manifest is retained through later
      // body/source transitions. Current preparation is never substituted on
      // restore.
      auto fixed_ref = [](const auto &source, Ref &target) noexcept {
        const auto length = std::char_traits<char>::length(source.data());
        if (length >= target.size())
          return false;
        std::copy_n(source.data(), length + 1, target.data());
        return valid_ref(target);
      };
      if (!fixed_ref(physical.event_ref, a.physical_event) ||
          !fixed_ref(physical.subject_ref, a.physical_subject) ||
          !fixed_ref(physical.source_coordinate,
                     a.physical_source_coordinate) ||
          !fixed_ref(physical.source_revision, a.physical_source_revision) ||
          !fixed_ref(physical.eigenbasis_identity, a.eigenbasis)) {
        recording_failed(RecordingFailure::PhysicalCommitFailure, a.sequence,
                         a.applied_sample);
        clear();
        fault_.store(true, std::memory_order_release);
        block_application_count_ = 0;
        std::fill_n(output, frames, 0);
        return false;
      }
      a.physical_source_generation = physical.source_generation;
      a.physical_sample_rate = physical.sample_rate;
      a.physical_pratibimba = physical.pratibimba;
      // P and final output have committed. Even an explicitly refused operation
      // now has a committed history fact. Failed publication consumes its
      // ordinal and records loss, so C can detect both interior and trailing
      // missing facts.
      a.applied_application_ordinal = ++applied_application_ordinal_;
      if (!gesture_applications_.push(a)) {
        recording_failed(RecordingFailure::ApplicationQueueOverflow, a.sequence,
                         a.applied_sample);
        dropped_applications_.fetch_add(1, std::memory_order_relaxed);
      }
    }
    block_application_count_ = 0;
    Readback receipt{};
    receipt.identity = determination_.identity;
    receipt.determination = determination_;
    receipt.physical = physical;
    receipt.scalar_force_budget_newtons =
        has_route_programs_ ? route_scalar_budget(route_programs_.manifest)
                            : body_.max_force_newtons;
    receipt.has_route_programs = has_route_programs_;
    receipt.routes_suspended =
        has_route_programs_ && route_programs_.owner_suspended;
    receipt.physical_routes = physical_routes;
    receipt.samples_elapsed = cursor_;
    receipt.body_revision = determination_.body_revision;
    receipt.last_sequence = applied_sequence_;
    receipt.last_applied_application_ordinal = applied_application_ordinal_;
    receipt.refused = refused_;
    receipt.late = late_;
    receipt.overflows = overflow_count_.load();
    receipt.stolen = stolen_;
    receipt.dropped_readbacks = dropped_readbacks_;
    receipt.dropped_captures = dropped_captures_;
    receipt.clipping_samples = clipping_;
    receipt.force_limited_samples = force_limited_;
    receipt.force_zero_samples = force_zero_samples_;
    receipt.emergency_requested =
        emergency_requested_.load(std::memory_order_acquire);
    receipt.emergency_observed = emergency_observed_;
    receipt.emergency_applied_sample = emergency_applied_sample_;
    for (const auto &tail : tails_)
      receipt.active_tails += tail.left != 0;
    for (const auto &v : voices_)
      if (v.active) {
        auto &observed = receipt.voices[receipt.active_voices++];
        observed.member = v.note.member;
        observed.m1_revision = v.note.identity.m1_revision;
        observed.m2_generation = v.note.identity.m2_generation;
        observed.effective_hertz = v.frequency;
        observed.target_hertz = v.target_frequency;
        observed.envelope = v.envelope;
        observed.releasing = v.release;
        for (std::size_t j = 0; j < 8; ++j) {
          observed.octet_effective_hertz[j] = v.octet_frequency[j];
          observed.octet_target_hertz[j] = octet_target(v, j);
          observed.suppressed_octet_components +=
              octet_target(v, j) < 0.001 || octet_target(v, j) > rate_ * 0.45;
        }
        for (const auto &t : touches_)
          observed.held_touches += t.token && t.member == v.note.member;
      }
    for (std::size_t i = 0; i < touches_.size(); ++i) {
      receipt.active_touches += bool(touches_[i].token);
      receipt.held_touch_tokens[i] = touches_[i].token;
    }
    receipt.peak = peak;
    receipt.rms = std::sqrt(power / frames);
    receipt.source = source_;
    receipt.effective = effective_;
    receipt.sustain = sustain_;
    receipt.available = available();
    receipt.recording = recording_status();
    if (!readbacks_.push(receipt))
      ++dropped_readbacks_;
    if (capture_.load(std::memory_order_relaxed) && !captures_.push(capture))
      ++dropped_captures_;
    return true;
  }
};
} // namespace ql::performance
#endif
