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
#include <limits>
#include <memory>
#include <ql/m_tree_live.h>
#include <ql/physical_snapshot.hpp>
#include <stdexcept>
#include <type_traits>

namespace ql::performance {
inline constexpr const char *contract = "ql.performance-audio/v1";
inline constexpr std::size_t max_frames = 512, max_voices = 24,
                             max_touches = 96, max_tails = 16,
                             queue_capacity = 256, capture_capacity = 16;
inline constexpr double tau = 6.283185307179586476925286766559;
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
};
struct ReleaseOperation {
  Kind kind = Kind::NoteOff;
  Identity identity{};
  std::uint64_t sequence = 0, sample = 0, touch = 0;
  bool late_admitted = false;
  NativeClockMetadata native_clock{};
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
struct Readback {
  Identity identity{};
  Determination determination{};
  ql::PhysicalSnapshot physical{};
  std::uint64_t samples_elapsed = 0, body_revision = 0, last_sequence = 0,
                refused = 0, late = 0, overflows = 0, stolen = 0,
                dropped_readbacks = 0, dropped_captures = 0,
                clipping_samples = 0, force_limited_samples = 0;
  std::uint32_t active_voices = 0, active_touches = 0;
  double peak = 0, rms = 0;
  Parameters source{}, effective{};
  std::array<VoiceReadback, max_voices> voices{};
  bool sustain = false, available = false;
};
struct NativeGestureApplication {
  Identity identity{};
  NativeClockMetadata clock{};
  std::uint64_t sequence = 0, admitted_sample = 0, applied_sample = 0,
                touch = 0;
  Kind kind = Kind::NoteOn;
  bool applied = false;
};
struct Capture {
  Identity identity{}, end_identity{};
  std::uint64_t start_sample = 0, body_revision = 0;
  std::uint32_t frames = 0;
  std::array<double, max_frames> force_newtons{};
  std::array<float, max_frames> pickup_linear{}, output_linear{};
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
    static constexpr const char *schema = "ql.performance-checkpoint/v1";
    std::uint32_t version = 1;
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
    Parameters source{}, effective{};
    std::uint64_t cursor = 0, accepted_sequence = 0, accepted_sample = 0,
                  applied_sequence = 0, refused = 0, late = 0, stolen = 0,
                  dropped_readbacks = 0, dropped_captures = 0, clipping = 0,
                  force_limited = 0, overflow_count = 0, panic_fence = 0;
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

private:
  Determination determination_{};
  // Owned only by the serial producer; callback never touches this copy.
  Determination producer_determination_{};
  Identity producer_identity_{};
  std::array<ScheduledDetermination, 8> source_schedule_{};
  std::size_t source_schedule_size_ = 1;
  unsigned rate_;
  PhysicalPort body_{};
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
  Parameters source_{}, effective_{};
  std::uint64_t cursor_ = 0, accepted_sequence_ = 0, accepted_sample_ = 0,
                last_admission_sample_ = 0, applied_sequence_ = 0, refused_ = 0,
                late_ = 0, stolen_ = 0, dropped_readbacks_ = 0,
                dropped_captures_ = 0, clipping_ = 0, force_limited_ = 0;
  std::atomic<std::uint64_t> published_cursor_{0}, overflow_count_{0},
      published_sequence_{0}, panic_fence_{0}, published_horizon_{0};
  std::atomic<bool> emergency_{false}, capture_{false}, fault_{false};
  // Lock-free render/custody exclusion. Device start/stop marks actual callback
  // custody; a requested stop is insufficient until the OS has acknowledged it.
  std::atomic<std::uint32_t> activity_{0};
  std::atomic<bool> device_running_{false};
  bool sustain_ = false;
  static bool scalar(double x, double lo, double hi) noexcept {
    return std::isfinite(x) && x >= lo && x <= hi;
  }
  static bool same_lineage(const Identity &a, const Identity &b) noexcept {
    return a.instance == b.instance && a.event == b.event &&
           a.subject == b.subject;
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
  void apply(const Operation &op) noexcept {
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
    case Kind::Expression:
      for (auto &t : touches_)
        if (t.token == op.touch) {
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
      break;
    case Kind::NoteOn: {
      if (op.sequence <= panic_fence_.load(std::memory_order_relaxed))
        return;
      for (const auto &t : touches_)
        if (t.token == op.note.touch) {
          ++refused_;
          return;
        }
      auto touch = std::find_if(touches_.begin(), touches_.end(),
                                [](const Touch &t) { return !t.token; });
      if (touch == touches_.end()) {
        ++refused_;
        return;
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
        return;
      }
      *touch = Touch{op.note.touch,     op.note.member,  op.value, 1,
                     op.note.touch_ref, op.note.identity};
      refresh(*voice);
      break;
    }
    }
  }
  void apply_release(const ReleaseOperation &op) noexcept {
    if (!same_lineage(op.identity, determination_.identity)) {
      ++refused_;
      return;
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
  friend class NativeOutputClock;

private:
  Result enqueue_native_admitted(Operation op) noexcept {
    op.sample = std::max(op.sample, admission_horizon());
    return enqueue_impl(op, true);
  }

public:
  // Single control owner only. Failed admission consumes no source sequence.
  // A full queue additionally requests safe all-notes-off out of band, so a
  // lost NoteOff can never leave a permanently sounding excitation.
  Result enqueue(const Operation &op) noexcept {
    if (op.native_clock.epoch || op.native_clock.anchor_ordinal ||
        op.native_clock.trigger_host_ticks ||
        op.native_clock.admitted_host_ticks ||
        op.native_clock.mapping_uncertainty_samples)
      return Result::Invalid;
    return enqueue_impl(op, false);
  }

private:
  Result enqueue_impl(const Operation &op, bool native_gesture) noexcept {
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
    admitted.late_admitted = late;
    if (late)
      admitted.sample = cursor;
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
            ? releases_.push(ReleaseOperation{op.kind, op.identity, op.sequence,
                                              admitted.sample, op.touch, late,
                                              admitted.native_clock})
            : operations_.push(admitted);
    if (!queued) {
      overflow_count_.fetch_add(1);
      emergency_.store(true, std::memory_order_release);
      return Result::Overflow;
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
  bool device_callbacks_running() const noexcept {
    return device_running_.load(std::memory_order_acquire);
  }
  // Out-of-band panic cancels previously queued attacks while preserving
  // prepared source changes and physical tails. A subsequent human attack
  // gets a higher sequence and can play normally.
  void request_panic() noexcept {
    fence_attacks(published_sequence_.load(std::memory_order_acquire));
    emergency_.store(true, std::memory_order_release);
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
  bool validate_note_target(const NoteTarget &note) const noexcept {
    return valid_note(
        note, source_at(published_cursor_.load(std::memory_order_acquire)));
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
  // Destination is host heap custody. Never put several MiB of fixed queue
  // state on a callback or ordinary native control thread's stack.
  void write_checkpoint(Checkpoint &cp, const StoppedCustody &guard) const {
    if (guard.owner_ != this || activity_.load() != 2 ||
        body_.cursor(body_.owner) != cursor_ ||
        body_.revision(body_.owner) != determination_.body_revision)
      throw std::logic_error(
          "exclusive paired audio/body checkpoint custody required");
    cp.version = 1;
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
    cp.source = source_;
    cp.effective = effective_;
    cp.cursor = cursor_;
    cp.accepted_sequence = accepted_sequence_;
    cp.accepted_sample = accepted_sample_;
    cp.applied_sequence = applied_sequence_;
    cp.refused = refused_;
    cp.late = late_;
    cp.stolen = stolen_;
    cp.dropped_readbacks = dropped_readbacks_;
    cp.dropped_captures = dropped_captures_;
    cp.clipping = clipping_;
    cp.force_limited = force_limited_;
    cp.overflow_count = overflow_count_.load();
    cp.panic_fence = panic_fence_.load();
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
        cp.version != 1 || cp.sample_rate != rate_ ||
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
        cp.applied_sequence > cp.accepted_sequence ||
        cp.panic_fence > cp.accepted_sequence ||
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
            !scalar(t.pressure, 0, 1))
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
    return true;
  }
  bool restore_checkpoint(const Checkpoint &cp, const StoppedCustody &guard,
                          std::uint64_t expected_cursor) noexcept {
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
    cursor_ = cp.cursor;
    accepted_sequence_ = cp.accepted_sequence;
    accepted_sample_ = cp.accepted_sample;
    applied_sequence_ = cp.applied_sequence;
    refused_ = cp.refused;
    late_ = cp.late;
    stolen_ = cp.stolen;
    dropped_readbacks_ = cp.dropped_readbacks;
    dropped_captures_ = cp.dropped_captures;
    clipping_ = cp.clipping;
    force_limited_ = cp.force_limited;
    overflow_count_.store(cp.overflow_count);
    panic_fence_.store(cp.panic_fence);
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
    gesture_applications_.clear_stopped();
    return true;
  }
  bool available() const noexcept {
    return !fault_.load(std::memory_order_acquire);
  }
  bool pop_readback(Readback &out) noexcept { return readbacks_.take(out); }
  bool pop_capture(Capture &out) noexcept { return captures_.take(out); }
  bool pop_gesture_application(NativeGestureApplication &out) noexcept {
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
      std::atomic<std::uint32_t> &activity;
      ~RenderCustody() { activity.store(0, std::memory_order_release); }
    } rendering{activity_};
    if (fault_.load(std::memory_order_relaxed) || start_sample != cursor_ ||
        cursor_ > std::numeric_limits<std::uint64_t>::max() - frames)
      return false;
    if (body_.revision(body_.owner) != determination_.body_revision ||
        body_.cursor(body_.owner) != cursor_) {
      fault_.store(true, std::memory_order_release);
      return false;
    }
    published_horizon_.store(cursor_ + frames, std::memory_order_release);
    Capture capture{};
    capture.identity = determination_.identity;
    capture.start_sample = cursor_;
    capture.body_revision = determination_.body_revision;
    capture.frames = std::uint32_t(frames);
    std::array<double, max_frames> body_gain{}, monitor_gain{}, force_scale{};
    const double smoothing = -std::expm1(-1.0 / (0.005 * rate_));
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
          apply_release(due->operation);
          if (due->operation.native_clock.epoch &&
              !gesture_applications_.push(NativeGestureApplication{
                  due->operation.identity, due->operation.native_clock,
                  due->operation.sequence, due->operation.sample, cursor_ + i,
                  due->operation.touch, due->operation.kind, true}))
            ++dropped_readbacks_;
          due->active = false;
        } else {
          if (op->late_admitted || op->sample < cursor_ + i)
            ++late_;
          const bool apply_now =
              op->identity == determination_.identity &&
              (op->sample == cursor_ + i || op->native_clock.epoch ||
               op->kind == Kind::Determination || op->kind == Kind::Parameter);
          if (apply_now)
            apply(*op);
          else
            ++refused_;
          if (op->native_clock.epoch &&
              !gesture_applications_.push(NativeGestureApplication{
                  op->identity, op->native_clock, op->sequence, op->sample,
                  cursor_ + i,
                  op->kind == Kind::NoteOn ? op->note.touch : op->touch,
                  op->kind, apply_now}))
            ++dropped_readbacks_;
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
      if (std::abs(force) > body_.max_force_newtons)
        ++force_limited_;
      force =
          std::clamp(force, -body_.max_force_newtons, body_.max_force_newtons);
      capture.force_newtons[i] = force;
      body_gain[i] = effective_.master_linear * effective_.body_linear;
      monitor_gain[i] = effective_.master_linear * effective_.monitor_linear;
      force_scale[i] = effective_.force_newtons;
    }
    capture.end_identity = determination_.identity;
    if (!body_.advance(body_.owner, capture.force_newtons.data(),
                       capture.pickup_linear.data(), frames,
                       determination_.body_revision, cursor_)) {
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
    Readback receipt{};
    receipt.identity = determination_.identity;
    receipt.determination = determination_;
    receipt.physical = physical;
    receipt.samples_elapsed = cursor_;
    receipt.body_revision = determination_.body_revision;
    receipt.last_sequence = applied_sequence_;
    receipt.refused = refused_;
    receipt.late = late_;
    receipt.overflows = overflow_count_.load();
    receipt.stolen = stolen_;
    receipt.dropped_readbacks = dropped_readbacks_;
    receipt.dropped_captures = dropped_captures_;
    receipt.clipping_samples = clipping_;
    receipt.force_limited_samples = force_limited_;
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
    for (const auto &t : touches_)
      receipt.active_touches += bool(t.token);
    receipt.peak = peak;
    receipt.rms = std::sqrt(power / frames);
    receipt.source = source_;
    receipt.effective = effective_;
    receipt.sustain = sustain_;
    receipt.available = available();
    if (!readbacks_.push(receipt))
      ++dropped_readbacks_;
    if (capture_.load(std::memory_order_relaxed) && !captures_.push(capture))
      ++dropped_captures_;
    return true;
  }
};
} // namespace ql::performance
#endif
