#ifndef QL_PHYSICAL_MOVING_RECEIVING_HPP
#define QL_PHYSICAL_MOVING_RECEIVING_HPP
// Native point-source receiving over the SAME physical pickup. Constant metric
// velocities select retarded emission time, never oscillator/eigenmode tuning.
// The policy is explicit architecture-model work, not vendor DSP internals.
#include <ql/physical_receiving.hpp>

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
    for (const auto *ref : {&spatial_.identity(), &motion_.source_motion_ref,
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
    if (sample < motion_.origin_sample || sample > motion_.end_sample)
      return false;
    const double seconds =
        double(sample - motion_.origin_sample) / spatial_.sample_rate();
    const auto &input = spatial_.input();
    const auto &source_velocity = motion_.source_velocity_metres_per_second;
    Vec3 delta{};
    double square = 0, dot = 0, velocity_square = 0,
           receiver_velocity_square = 0;
    SpatialMotionPoint point{};
    for (unsigned axis = 0; axis < 3; ++axis) {
      const double source_now = spatial_.source_position_metres()[axis] +
                                source_velocity[axis] * seconds;
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
          spatial_.source_position_metres()[axis] +
          source_velocity[axis] * (seconds - delay_seconds);
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
struct MovingSpatialCheckpoint {
  unsigned version = 1;
  std::string contract = "ql.physical-moving-receiving-checkpoint/v1",
              receiving_identity, body_eigenbasis_identity,
              signal_unit = "linear", distance_unit = "m", speed_unit = "m/s",
              cursor_unit = "native-audio-sample";
  std::uint64_t samples_elapsed = 0, history_start_sample = 0;
  std::array<float, receiving_history_samples> history_linear{};
};
class MovingSpatialReceiving {
  PreparedMovingSpatialReceiving prepared_;
  std::array<float, receiving_history_samples> history_{};
  std::uint64_t elapsed_ = 0, history_start_ = 0;
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
  }
  const PreparedMovingSpatialReceiving &preparation() const noexcept {
    return prepared_;
  }
  std::uint64_t samples_elapsed() const noexcept { return elapsed_; }
  // Callback-owned BEFORE the sole P advance, so a scheduled trajectory
  // boundary cannot partially commit body state. No history mutates here.
  bool preflight_before_body(const PhysicalBody &body, std::size_t frames,
                             std::uint64_t start) const noexcept {
    if (!span(frames, start) || body.samples_elapsed() != start ||
        !prepared_.matches_preparation(body.preparation()))
      return false;
    SpatialMotionPoint point{};
    for (std::size_t i = 0; i < frames; ++i)
      if (!prepared_.point_at(start + i, point))
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
    std::array<SpatialMotionPoint, physical_max_frames> points{};
    for (std::size_t i = 0; i < frames; ++i)
      if (!std::isfinite(pickup[i]) ||
          !prepared_.point_at(start + i, points[i]))
        return false;
    for (std::size_t i = 0; i < frames; ++i) {
      const auto sample = start + i;
      history_[sample % receiving_history_samples] = pickup[i];
      const double delay = points[i].propagation_delay_samples;
      const auto whole = static_cast<std::uint64_t>(std::floor(delay));
      const double fraction = delay - whole;
      auto read = [&](std::uint64_t lag) {
        return lag > sample - history_start_
                   ? 0.0
                   : double(
                         history_[(sample - lag) % receiving_history_samples]);
      };
      received[i] = static_cast<float>(
          points[i].gain_linear *
          ((1 - fraction) * read(whole) + fraction * read(whole + 1)));
    }
    elapsed_ += frames;
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
  }
  bool restore_checkpoint(const PhysicalBody &body,
                          const MovingSpatialCheckpoint &saved,
                          std::uint64_t expected_cursor) noexcept {
    if (expected_cursor != elapsed_ || saved.version != 1 ||
        saved.contract != "ql.physical-moving-receiving-checkpoint/v1" ||
        saved.receiving_identity != prepared_.identity() ||
        !prepared_.matches_preparation(body.preparation()) ||
        saved.body_eigenbasis_identity !=
            body.preparation().eigenbasis_identity() ||
        saved.samples_elapsed != body.samples_elapsed() ||
        saved.history_start_sample < prepared_.motion().origin_sample ||
        saved.history_start_sample > saved.samples_elapsed ||
        saved.samples_elapsed > prepared_.motion().end_sample ||
        saved.signal_unit != "linear" || saved.distance_unit != "m" ||
        saved.speed_unit != "m/s" || saved.cursor_unit != "native-audio-sample")
      return false;
    for (float x : saved.history_linear)
      if (!std::isfinite(x))
        return false;
    history_ = saved.history_linear;
    history_start_ = saved.history_start_sample;
    elapsed_ = saved.samples_elapsed;
    return true;
  }
};
} // namespace ql
#endif
