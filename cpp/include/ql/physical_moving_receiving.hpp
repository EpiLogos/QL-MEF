#ifndef QL_PHYSICAL_MOVING_RECEIVING_HPP
#define QL_PHYSICAL_MOVING_RECEIVING_HPP
// Native point-source receiving over the SAME physical pickup. Constant metric
// velocities select retarded emission time, never oscillator/eigenmode tuning.
// The policy is explicit architecture-model work, not vendor DSP internals.
#include <memory>
#include <ql/physical_receiving.hpp>
#include <ql/physical_receiving_source_history.hpp>

namespace ql::performance {
class MovingReceivingPortBinding;
}
namespace ql {
struct SpatialMotionInput {
  std::string source_motion_ref, receiver_motion_ref, policy_ref,
      policy_revision, standing;
  Vec3 source_velocity_metres_per_second{},
      receiver_velocity_metres_per_second{};
  std::uint64_t origin_sample = 0, end_sample = 0;
};
struct SpatialMotionPoint {
  Vec3 source_emission_position_metres{}, receiver_position_metres{};
  double distance_metres = 0, propagation_delay_samples = 0, gain_linear = 0,
         emission_samples_per_received_sample = 1;
};
class PreparedMovingSpatialReceiving {
  PreparedSpatialReceiving spatial_;
  SpatialMotionInput motion_;
  std::string identity_;

public:
  PreparedMovingSpatialReceiving(const PreparedPhysicalBody &body,
                                 SpatialReceivingInput spatial,
                                 SpatialMotionInput motion)
      : spatial_(body, std::move(spatial)), motion_(std::move(motion)) {
    for (const auto *ref :
         {&motion_.source_motion_ref, &motion_.receiver_motion_ref,
          &motion_.policy_ref, &motion_.policy_revision, &motion_.standing})
      reference(*ref);
    require(motion_.standing == "architecture-model" ||
                motion_.standing == "reference" ||
                motion_.standing == "tunable-model",
            "native spatial motion requires declared model standing");
    require(motion_.end_sample > motion_.origin_sample &&
                motion_.end_sample - motion_.origin_sample <=
                    std::uint64_t(spatial_.sample_rate()) * 60,
            "native spatial motion exceeds finite sixty-second span");
    const double c = spatial_.input().speed_metres_per_second;
    for (const auto *velocity :
         {&motion_.source_velocity_metres_per_second,
          &motion_.receiver_velocity_metres_per_second}) {
      double square = 0;
      for (double x : *velocity) {
        require(std::isfinite(x), "nonfinite native spatial velocity");
        square += x * x;
      }
      // Native P modes and A forcing are admitted below .45*rate. This
      // conservative subsonic policy bounds worst frequency multiplier to
      // 1.025/.975 < .5/.45. Linear fractional-delay reconstruction remains
      // the explicitly declared model; no ideal radiation/bandlimit claim.
      require(std::isfinite(square) && square <= c * c * .025 * .025,
              "native receiving velocity exceeds admitted audio-band policy");
    }
    SpatialMotionPoint point{};
    require(point_at(motion_.origin_sample, point) &&
                point_at(motion_.end_sample, point),
            "native trajectory exceeds metric/delay bounds");
    // Compatibility only. Full original source/receiving/context owner
    // authority and typed current basis remain independent of this seal.
    std::uint64_t hash = 14695981039346656037ULL;
    auto byte = [&](unsigned char x) { hash = (hash ^ x) * 1099511628211ULL; };
    auto integer = [&](std::uint64_t x) {
      for (unsigned i = 0; i < 8; ++i) {
        byte(static_cast<unsigned char>(x));
        x >>= 8;
      }
    };
    auto text = [&](const std::string &s) {
      integer(s.size());
      for (unsigned char x : s)
        byte(x);
    };
    auto real = [&](double x) {
      std::uint64_t bits;
      std::memcpy(&bits, &x, sizeof(bits));
      integer(bits);
    };
    for (const std::string *ref : std::array<const std::string *, 6>{
             &spatial_.identity(), &motion_.source_motion_ref,
             &motion_.receiver_motion_ref, &motion_.policy_ref,
             &motion_.policy_revision, &motion_.standing})
      text(*ref);
    integer(motion_.origin_sample);
    integer(motion_.end_sample);
    for (const auto *v : {&motion_.source_velocity_metres_per_second,
                          &motion_.receiver_velocity_metres_per_second})
      for (double x : *v)
        real(x);
    identity_ = "ql.physical-moving-receiving/fnv1a64-v1:";
    constexpr char digits[] = "0123456789abcdef";
    for (int i = 15; i >= 0; --i)
      identity_.push_back(digits[(hash >> (4 * i)) & 15]);
  }
  const PreparedSpatialReceiving &spatial() const noexcept { return spatial_; }
  const SpatialMotionInput &motion() const noexcept { return motion_; }
  const std::string &identity() const noexcept { return identity_; }
  bool matches_preparation(const PreparedPhysicalBody &body) const noexcept {
    return spatial_.matches_preparation(body);
  }
  // Native sample difference is bounded before conversion; no large absolute
  // integer cursor is rounded into a floating-point clock.
  bool point_at(std::uint64_t sample, SpatialMotionPoint &out) const noexcept {
    return point_at_source(sample, spatial_.source_position_metres(),
                           motion_.origin_sample,
                           motion_.source_velocity_metres_per_second, out);
  }
  // A historical source anchor takes effect at its actual EMISSION date.
  // The receiver position still belongs to the current received-sample act.
  bool point_at_source(std::uint64_t sample, const Vec3 &source_anchor,
                       std::uint64_t source_origin, const Vec3 &source_velocity,
                       SpatialMotionPoint &out) const noexcept {
    if (sample < source_origin || sample < motion_.origin_sample ||
        sample > motion_.end_sample)
      return false;
    const double seconds =
        double(sample - motion_.origin_sample) / spatial_.sample_rate();
    const auto &input = spatial_.input();
    const double source_seconds =
        double(sample - source_origin) / spatial_.sample_rate();
    Vec3 delta{};
    double square = 0, dot = 0, velocity_square = 0,
           receiver_velocity_square = 0;
    SpatialMotionPoint point{};
    for (unsigned axis = 0; axis < 3; ++axis) {
      const double source_now =
          source_anchor[axis] + source_velocity[axis] * source_seconds;
      point.receiver_position_metres[axis] =
          input.receiver_position_metres[axis] +
          motion_.receiver_velocity_metres_per_second[axis] * seconds;
      if (!std::isfinite(source_now) ||
          !std::isfinite(point.receiver_position_metres[axis]) ||
          std::abs(source_now) > 1e6 ||
          std::abs(point.receiver_position_metres[axis]) > 1e6)
        return false;
      delta[axis] = source_now - point.receiver_position_metres[axis];
      square += delta[axis] * delta[axis];
      dot += delta[axis] * source_velocity[axis];
      velocity_square += source_velocity[axis] * source_velocity[axis];
      receiver_velocity_square +=
          motion_.receiver_velocity_metres_per_second[axis] *
          motion_.receiver_velocity_metres_per_second[axis];
    }
    const double c = input.speed_metres_per_second;
    if (velocity_square > c * c * .025 * .025)
      return false;
    // c*tau = |source(t_receive)-v_source*tau-receiver(t_receive)|.
    // Stable positive root of (c²-|v_source|²)tau²+2dot*tau-|r|²=0.
    const double discriminant = dot * dot + (c * c - velocity_square) * square;
    if (!std::isfinite(discriminant) || discriminant < 0)
      return false;
    const double denominator = std::sqrt(discriminant) + dot;
    const double delay_seconds =
        !input.propagation_delay
            ? 0
            : (velocity_square == 0 ? std::sqrt(square) / c
                                    : (square == 0 ? 0 : square / denominator));
    if (!std::isfinite(delay_seconds) || delay_seconds < 0)
      return false;
    point.distance_metres = !input.propagation_delay || velocity_square == 0
                                ? std::sqrt(square)
                                : c * delay_seconds;
    point.propagation_delay_samples =
        input.propagation_delay ? delay_seconds * spatial_.sample_rate() : 0;
    double directional = 1, source_radial = 0, receiver_radial = 0;
    Vec3 ray{};
    for (unsigned axis = 0; axis < 3; ++axis) {
      point.source_emission_position_metres[axis] =
          source_anchor[axis] +
          source_velocity[axis] * (source_seconds - delay_seconds);
      if (!std::isfinite(point.source_emission_position_metres[axis]) ||
          std::abs(point.source_emission_position_metres[axis]) > 1e6)
        return false;
      ray[axis] = point.source_emission_position_metres[axis] -
                  point.receiver_position_metres[axis];
      if (point.distance_metres > 0) {
        const double n = ray[axis] / point.distance_metres;
        source_radial += n * source_velocity[axis];
        receiver_radial +=
            n * motion_.receiver_velocity_metres_per_second[axis];
      }
    }
    if (input.directivity == ReceivingDirectivity::Cardioid &&
        point.distance_metres > 0) {
      double cosine = 0;
      for (unsigned axis = 0; axis < 3; ++axis)
        cosine +=
            input.receiver_forward[axis] * ray[axis] / point.distance_metres;
      directional = .5 * (1 + std::clamp(cosine, -1.0, 1.0));
    }
    point.gain_linear =
        directional * input.minimum_distance_metres /
        std::max(point.distance_metres, input.minimum_distance_metres);
    // Unit ray projection and derivative are bounded to their mathematical
    // ranges, preventing roundoff from inventing a super-policy velocity.
    source_radial = std::clamp(source_radial, -std::sqrt(velocity_square),
                               std::sqrt(velocity_square));
    receiver_radial =
        std::clamp(receiver_radial, -std::sqrt(receiver_velocity_square),
                   std::sqrt(receiver_velocity_square));
    point.emission_samples_per_received_sample =
        input.propagation_delay
            ? std::clamp((c + receiver_radial) / (c + source_radial),
                         .975 / 1.025, 1.025 / .975)
            : 1;
    if (!std::isfinite(point.gain_linear) || point.gain_linear < 0 ||
        point.gain_linear > 1 ||
        !std::isfinite(point.propagation_delay_samples) ||
        point.propagation_delay_samples < 0 ||
        point.propagation_delay_samples > receiving_history_samples - 2 ||
        !std::isfinite(point.emission_samples_per_received_sample) ||
        point.emission_samples_per_received_sample < .975 / 1.025 ||
        point.emission_samples_per_received_sample > 1.025 / .975)
      return false;
    out = point;
    return true;
  }
};
inline bool current_receiving_source_matches(
    const ReceivingSourceHistory &history, const PreparedPhysicalBody &body,
    const PreparedMovingSpatialReceiving &receiving) noexcept {
  if (!history.count || history.count > history.segments.size() ||
      !receiving.matches_preparation(body))
    return false;
  const auto &last = history.segments[history.count - 1];
  const auto &in = body.input();
  if (last.body_revision != in.body_revision ||
      last.source_generation != in.source_generation ||
      last.pratibimba != in.pratibimba ||
      last.trajectory_origin_sample != receiving.motion().origin_sample ||
      last.anchor_metres != receiving.spatial().source_position_metres() ||
      last.velocity_metres_per_second !=
          receiving.motion().source_velocity_metres_per_second)
    return false;
  const std::array<const std::string *, 6> refs{
      &in.source_coordinate,       &in.source_revision,
      &in.preparation_ref,         &in.state_ref,
      &body.eigenbasis_identity(), &receiving.motion().source_motion_ref};
  for (std::size_t i = 0; i < refs.size(); ++i) {
    const auto *original =
        receiving_source_reference(history, last.references[i]);
    if (!original || std::strcmp(original, refs[i]->c_str()))
      return false;
  }
  return true;
}
// Fractional-delay interpolation is windowed by the actual dated emitting
// segment at EACH endpoint. This preserves first/last partial samples and
// cannot take a future body's pickup into an old body's emission history.
inline bool receiving_source_has_endpoint(std::uint64_t sample,
                                          std::uint64_t birth,
                                          std::uint64_t begin,
                                          std::uint64_t end,
                                          double delay) noexcept {
  const auto whole = static_cast<std::uint64_t>(std::floor(delay));
  const double fraction = delay - whole;
  const auto contains = [&](std::uint64_t lag) noexcept {
    if (sample < birth || lag > sample - birth)
      return false;
    const auto at = sample - lag;
    return at >= begin && at < end;
  };
  return ((1 - fraction) > 0 && contains(whole)) ||
         (fraction > 0 && contains(whole + 1));
}
// CONTROL only. Prune metadata only after its last possible retarded arrival.
// Exact original references remain complete; no label/arena entry is authority.
inline bool append_receiving_source(
    ReceivingSourceHistory &history, const PreparedPhysicalBody &body,
    const PreparedMovingSpatialReceiving &receiving,
    std::uint64_t effective_sample, std::uint64_t history_birth) {
  if (!receiving.matches_preparation(body) ||
      effective_sample < history_birth ||
      effective_sample < receiving.motion().origin_sample ||
      (history.count && !valid_receiving_source_history(
                            history, effective_sample, history_birth)))
    return false;
  auto next = std::make_unique<ReceivingSourceHistory>();
  const auto earliest = std::max(
      history_birth, effective_sample >= receiving_history_samples - 1
                         ? effective_sample - (receiving_history_samples - 1)
                         : history_birth);
  std::uint32_t first = history.first;
  while (first + 1 < history.count &&
         history.segments[first + 1].effective_sample <= earliest)
    ++first;
  for (std::uint32_t i = first; i < history.count; ++i) {
    if (history.segments[i].effective_sample == effective_sample)
      break;
    if (history.segments[i].effective_sample > effective_sample ||
        next->count == next->segments.size())
      return false;
    auto segment = history.segments[i];
    for (std::size_t j = 0; j < segment.references.size(); ++j) {
      const auto *original =
          receiving_source_reference(history, segment.references[j]);
      if (!original || !intern_receiving_source_reference(
                           *next, original, segment.references[j]))
        return false;
    }
    next->segments[next->count++] = segment;
  }
  if (next->count == next->segments.size())
    return false;
  const auto &in = body.input();
  auto &segment = next->segments[next->count++];
  segment.effective_sample = effective_sample;
  segment.trajectory_origin_sample = receiving.motion().origin_sample;
  segment.body_revision = in.body_revision;
  segment.source_generation = in.source_generation;
  segment.pratibimba = in.pratibimba;
  segment.anchor_metres = receiving.spatial().source_position_metres();
  segment.velocity_metres_per_second =
      receiving.motion().source_velocity_metres_per_second;
  const std::array<const std::string *, 6> refs{
      &in.source_coordinate,       &in.source_revision,
      &in.preparation_ref,         &in.state_ref,
      &body.eigenbasis_identity(), &receiving.motion().source_motion_ref};
  for (std::size_t j = 0; j < refs.size(); ++j)
    if (!intern_receiving_source_reference(*next, *refs[j],
                                           segment.references[j]))
      return false;
  next->explicit_history = history.explicit_history || history.count != 0;
  if (!valid_receiving_source_history(*next, effective_sample, history_birth))
    return false;
  history = *next;
  return true;
}
struct MovingSpatialCheckpoint {
  unsigned version = 1;
  std::string contract = "ql.physical-moving-receiving-checkpoint/v1",
              receiving_identity, body_eigenbasis_identity,
              signal_unit = "linear", distance_unit = "m", speed_unit = "m/s",
              cursor_unit = "native-audio-sample";
  std::uint64_t samples_elapsed = 0, history_start_sample = 0;
  std::array<float, receiving_history_samples> history_linear{};
  ReceivingSourceHistory source_history{};
};
class MovingSpatialReceiving {
  friend class ql::performance::MovingReceivingPortBinding;
  PreparedMovingSpatialReceiving prepared_;
  std::array<float, receiving_history_samples> history_{};
  ReceivingSourceHistory source_history_{};
  std::uint64_t elapsed_ = 0, history_start_ = 0;
  // Conservative callback-owned bound; recomputed from the full actual ring
  // only on CONTROL restore. Callback work is constant in ring capacity.
  double history_peak_ = 0;
  void restore_history_peak() noexcept {
    history_peak_ = 0;
    for (float x : history_)
      history_peak_ = std::max(history_peak_, std::abs(double(x)));
  }
  bool span(std::size_t frames, std::uint64_t start) const noexcept {
    return frames && frames <= physical_max_frames && start == elapsed_ &&
           elapsed_ <= std::numeric_limits<std::uint64_t>::max() - frames &&
           start >= prepared_.motion().origin_sample &&
           start + frames <= prepared_.motion().end_sample;
  }

public:
  MovingSpatialReceiving(const PreparedPhysicalBody &body,
                         std::uint64_t native_admitted_cursor,
                         PreparedMovingSpatialReceiving prepared)
      : prepared_(std::move(prepared)), elapsed_(native_admitted_cursor),
        history_start_(native_admitted_cursor) {
    require(prepared_.matches_preparation(body) &&
                native_admitted_cursor >= prepared_.motion().origin_sample &&
                native_admitted_cursor <= prepared_.motion().end_sample,
            "moving receiving lost immutable body/native admitted cursor");
    require(append_receiving_source(source_history_, body, prepared_,
                                    native_admitted_cursor, history_start_),
            "moving receiving lost original complete source history");
  }
  const PreparedMovingSpatialReceiving &preparation() const noexcept {
    return prepared_;
  }
  std::uint64_t samples_elapsed() const noexcept { return elapsed_; }
  const ReceivingSourceHistory &source_history() const noexcept {
    return source_history_;
  }
  bool historical_point(std::uint64_t sample, SpatialMotionPoint &out,
                        unsigned &contributions,
                        std::uint32_t *selected = nullptr) const noexcept {
    contributions = 0;
    if (selected)
      *selected = source_history_.count - 1;
    if (!source_history_.explicit_history) {
      contributions = 1;
      return prepared_.point_at(sample, out);
    }
    SpatialMotionPoint latest{};
    for (std::uint32_t j = source_history_.first; j < source_history_.count;
         ++j) {
      const auto &s = source_history_.segments[j];
      const auto end = j + 1 < source_history_.count
                           ? source_history_.segments[j + 1].effective_sample
                           : std::numeric_limits<std::uint64_t>::max();
      if (sample >= receiving_history_samples - 1 &&
          end <= sample - (receiving_history_samples - 1))
        continue;
      SpatialMotionPoint point{};
      if (!prepared_.point_at_source(sample, s.anchor_metres,
                                     s.trajectory_origin_sample,
                                     s.velocity_metres_per_second, point))
        return false;
      if (!receiving_source_has_endpoint(sample, history_start_,
                                         s.effective_sample, end,
                                         point.propagation_delay_samples))
        continue;
      latest = point;
      ++contributions;
      if (selected)
        *selected = j;
    }
    if (!contributions && !prepared_.point_at(sample, latest))
      return false;
    out = latest;
    return true;
  }
  bool bounded_received_output(const PhysicalBody &body) const noexcept {
    double peak = body.preparation().input().max_displacement_metres *
                  std::abs(body.preparation().input().pickup_linear_per_metre);
    double weights = 0;
    for (double x : body.preparation().input().pickup.node_weights)
      weights += std::abs(x);
    peak *= weights;
    peak = std::max(peak, history_peak_);
    return std::isfinite(peak) &&
           peak * source_history_.count < std::numeric_limits<float>::max();
  }
  // Callback-owned BEFORE the sole P advance, so a scheduled trajectory
  // boundary cannot partially commit body state. No history mutates here.
  bool preflight_before_body(const PhysicalBody &body, std::size_t frames,
                             std::uint64_t start) const noexcept {
    if (!span(frames, start) || body.samples_elapsed() != start ||
        !prepared_.matches_preparation(body.preparation()) ||
        !bounded_received_output(body))
      return false;
    SpatialMotionPoint point{};
    unsigned contributions = 0;
    for (std::size_t i = 0; i < frames; ++i)
      if (!historical_point(start + i, point, contributions))
        return false;
    return true;
  }
  // Immediately AFTER actual P advances. All input/span/source/point checks
  // precede history/output mutation; no allocation, clock or q/v integration.
  bool process_pickup_block(const PhysicalBody &body, const float *pickup,
                            float *received, std::size_t frames,
                            std::uint64_t start) noexcept {
    if (!pickup || !received || !span(frames, start) ||
        body.samples_elapsed() != start + frames ||
        !prepared_.matches_preparation(body.preparation()))
      return false;
    const auto a = reinterpret_cast<std::uintptr_t>(pickup),
               b = reinterpret_cast<std::uintptr_t>(received);
    if ((a > b ? a - b : b - a) < frames * sizeof(float))
      return false;
    if (!bounded_received_output(body))
      return false;
    SpatialMotionPoint checked{};
    unsigned contributions = 0;
    for (std::size_t i = 0; i < frames; ++i)
      if (!std::isfinite(pickup[i]) ||
          !historical_point(start + i, checked, contributions))
        return false;
    // All fail-able source/output qualifications completed before mutation.
    for (std::size_t i = 0; i < frames; ++i) {
      const auto sample = start + i;
      history_[sample % receiving_history_samples] = pickup[i];
      history_peak_ = std::max(history_peak_, std::abs(double(pickup[i])));
      if (!source_history_.explicit_history) {
        SpatialMotionPoint point{};
        (void)prepared_.point_at(sample, point);
        const auto whole = static_cast<std::uint64_t>(
            std::floor(point.propagation_delay_samples));
        const double fraction = point.propagation_delay_samples - whole;
        auto read = [&](std::uint64_t lag) {
          return lag > sample - history_start_
                     ? 0.0
                     : double(history_[(sample - lag) %
                                       receiving_history_samples]);
        };
        received[i] = static_cast<float>(
            point.gain_linear *
            ((1 - fraction) * read(whole) + fraction * read(whole + 1)));
        continue;
      }
      double sum = 0;
      for (std::uint32_t j = source_history_.first; j < source_history_.count;
           ++j) {
        const auto &s = source_history_.segments[j];
        const auto end = j + 1 < source_history_.count
                             ? source_history_.segments[j + 1].effective_sample
                             : std::numeric_limits<std::uint64_t>::max();
        if (sample >= receiving_history_samples - 1 &&
            end <= sample - (receiving_history_samples - 1))
          continue;
        SpatialMotionPoint point{};
        (void)prepared_.point_at_source(sample, s.anchor_metres,
                                        s.trajectory_origin_sample,
                                        s.velocity_metres_per_second, point);
        const double delay = point.propagation_delay_samples;
        if (!receiving_source_has_endpoint(sample, history_start_,
                                           s.effective_sample, end, delay))
          continue;
        const auto whole = static_cast<std::uint64_t>(std::floor(delay));
        const double fraction = delay - whole;
        auto read = [&](std::uint64_t lag) {
          if (lag > sample - history_start_)
            return 0.0;
          const auto at = sample - lag;
          return at < s.effective_sample || at >= end
                     ? 0.0
                     : double(history_[at % receiving_history_samples]);
        };
        sum += point.gain_linear *
               ((1 - fraction) * read(whole) + fraction * read(whole + 1));
      }
      received[i] = static_cast<float>(sum);
    }
    elapsed_ += frames;
    const auto earliest = elapsed_ >= receiving_history_samples - 1
                              ? elapsed_ - (receiving_history_samples - 1)
                              : history_start_;
    while (
        source_history_.first + 1 < source_history_.count &&
        source_history_.segments[source_history_.first + 1].effective_sample <=
            earliest)
      ++source_history_.first;
    return true;
  }
  // Actual stopped/acknowledged sole-body custody, not a control live-body
  // read.
  void write_checkpoint(const PhysicalBody &body,
                        MovingSpatialCheckpoint &out) const {
    require(body.samples_elapsed() == elapsed_ &&
                prepared_.matches_preparation(body.preparation()),
            "moving receiving checkpoint source/cursor differs");
    out = MovingSpatialCheckpoint{};
    out.receiving_identity = prepared_.identity();
    out.body_eigenbasis_identity = body.preparation().eigenbasis_identity();
    out.samples_elapsed = elapsed_;
    out.history_start_sample = history_start_;
    out.history_linear = history_;
    out.source_history = source_history_;
    if (source_history_.explicit_history) {
      out.version = 2;
      out.contract = "ql.physical-moving-receiving-checkpoint/v2";
    }
  }
  bool restore_checkpoint(const PhysicalBody &body,
                          const MovingSpatialCheckpoint &saved,
                          std::uint64_t expected_cursor) noexcept {
    if (expected_cursor != elapsed_ ||
        (saved.version != 1 && saved.version != 2) ||
        saved.contract !=
            (saved.version == 2
                 ? "ql.physical-moving-receiving-checkpoint/v2"
                 : "ql.physical-moving-receiving-checkpoint/v1") ||
        (saved.version == 2 &&
         (!saved.source_history.explicit_history ||
          !valid_receiving_source_history(saved.source_history,
                                          saved.samples_elapsed,
                                          saved.history_start_sample) ||
          !current_receiving_source_matches(saved.source_history,
                                            body.preparation(), prepared_))) ||
        saved.receiving_identity != prepared_.identity() ||
        !prepared_.matches_preparation(body.preparation()) ||
        saved.body_eigenbasis_identity !=
            body.preparation().eigenbasis_identity() ||
        saved.samples_elapsed != body.samples_elapsed() ||
        saved.history_start_sample > saved.samples_elapsed ||
        saved.samples_elapsed > prepared_.motion().end_sample ||
        saved.signal_unit != "linear" || saved.distance_unit != "m" ||
        saved.speed_unit != "m/s" || saved.cursor_unit != "native-audio-sample")
      return false;
    for (float x : saved.history_linear)
      if (!std::isfinite(x))
        return false;
    history_ = saved.history_linear;
    restore_history_peak();
    if (saved.version == 2)
      source_history_ = saved.source_history;
    history_start_ = saved.history_start_sample;
    elapsed_ = saved.samples_elapsed;
    return true;
  }
};
} // namespace ql
#endif
