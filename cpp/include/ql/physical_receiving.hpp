#ifndef QL_PHYSICAL_RECEIVING_HPP
#define QL_PHYSICAL_RECEIVING_HPP
// Explicit receiving transport after the actual body pickup. This is a
// declared point-source projection, not a radiation/pressure solver or camera.
#include <ql/physical_body.hpp>
namespace ql {
inline constexpr const char *physical_receiving_contract =
    "ql.physical-receiving/v1";
inline constexpr std::size_t receiving_history_samples = 16384;
enum class ReceivingDirectivity { Omnidirectional, Cardioid };
struct SpatialReceivingInput {
  std::string receiver_ref, context_ref, source_ref, policy_ref,
      policy_revision, standing;
  std::uint64_t revision = 1;
  Vec3 source_translation_metres{}, receiver_position_metres{},
      receiver_forward{1, 0, 0};
  double speed_metres_per_second = 340, minimum_distance_metres = 1;
  ReceivingDirectivity directivity = ReceivingDirectivity::Omnidirectional;
  bool propagation_delay = true;
  unsigned transition_samples = 128;
};
class PreparedSpatialReceiving {
  friend class PreparedReceivingTransition;
  SpatialReceivingInput input_;
  std::string body_preparation_ref_, body_state_ref_, eigenbasis_identity_,
      event_ref_, subject_ref_, identity_;
  std::uint64_t body_revision_ = 0;
  unsigned sample_rate_ = 0;
  Vec3 source_position_metres_{};
  double distance_metres_ = 0, gain_ = 0, delay_samples_ = 0;

public:
  // Off-thread provider work accepts only a retained immutable preparation;
  // the live body's basis can change under callback custody.
  PreparedSpatialReceiving(const PreparedPhysicalBody &prepared,
                           SpatialReceivingInput input)
      : input_(std::move(input)) {
    const auto &in = prepared.input();
    for (const auto *x :
         {&input_.receiver_ref, &input_.context_ref, &input_.source_ref,
          &input_.policy_ref, &input_.policy_revision, &input_.standing})
      reference(*x);
    require(input_.standing == "architecture-model" ||
                input_.standing == "reference" ||
                input_.standing == "tunable-model",
            "receiving requires physical model standing");
    require(input_.revision > 0 && input_.transition_samples <= 4096,
            "receiving revision/transition budget invalid");
    const auto bounded = [](double x, double low, double high) {
      return std::isfinite(x) && x >= low && x <= high;
    };
    require(bounded(input_.speed_metres_per_second, 50, 2000) &&
                bounded(input_.minimum_distance_metres, .001, 1000) &&
                (input_.directivity == ReceivingDirectivity::Omnidirectional ||
                 input_.directivity == ReceivingDirectivity::Cardioid),
            "invalid receiving propagation policy");
    double norm = 0;
    Vec3 direction{};
    for (unsigned a = 0; a < 3; ++a) {
      require(bounded(input_.source_translation_metres[a], -1e6, 1e6) &&
                  bounded(input_.receiver_position_metres[a], -1e6, 1e6) &&
                  bounded(input_.receiver_forward[a], -1, 1),
              "invalid receiving metric geometry");
      norm += input_.receiver_forward[a] * input_.receiver_forward[a];
      source_position_metres_[a] = input_.source_translation_metres[a];
      for (std::size_t n = 0; n < in.nodes.size(); ++n)
        source_position_metres_[a] +=
            in.nodes[n].rest_metres[a] * in.pickup.node_weights[n];
      direction[a] =
          source_position_metres_[a] - input_.receiver_position_metres[a];
      distance_metres_ += direction[a] * direction[a];
    }
    require(std::abs(norm - 1) <= 1e-10,
            "receiver orientation must be normalized");
    distance_metres_ = std::sqrt(distance_metres_);
    double directional = 1;
    if (input_.directivity == ReceivingDirectivity::Cardioid &&
        distance_metres_ > 0) {
      double cosine = 0;
      for (unsigned a = 0; a < 3; ++a)
        cosine += input_.receiver_forward[a] * direction[a] / distance_metres_;
      directional = .5 * (1 + std::clamp(cosine, -1.0, 1.0));
    }
    gain_ = directional * input_.minimum_distance_metres /
            std::max(distance_metres_, input_.minimum_distance_metres);
    delay_samples_ =
        input_.propagation_delay
            ? distance_metres_ / input_.speed_metres_per_second * in.sample_rate
            : 0;
    require(std::isfinite(gain_) && gain_ >= 0 && gain_ <= 1 &&
                std::isfinite(delay_samples_) && delay_samples_ >= 0 &&
                delay_samples_ <= receiving_history_samples - 2,
            "receiving delay exceeds bounded history");
    body_preparation_ref_ = in.preparation_ref;
    body_state_ref_ = in.state_ref;
    eigenbasis_identity_ = prepared.eigenbasis_identity();
    event_ref_ = in.event_ref;
    subject_ref_ = in.subject_ref;
    body_revision_ = in.body_revision;
    sample_rate_ = in.sample_rate;
    // Compatibility fingerprint includes the actual body basis and all
    // metric/policy inputs. It conveys no authorisation or source signing.
    std::uint64_t hash = 14695981039346656037ULL;
    const auto byte = [&](unsigned char x) {
      hash ^= x;
      hash *= 1099511628211ULL;
    };
    const auto integer = [&](std::uint64_t x) {
      for (unsigned i = 0; i < 8; ++i) {
        byte(static_cast<unsigned char>(x & 255));
        x >>= 8;
      }
    };
    const auto text = [&](const std::string &x) {
      integer(x.size());
      for (unsigned char c : x)
        byte(c);
    };
    const auto real = [&](double x) {
      std::uint64_t bits;
      std::memcpy(&bits, &x, 8);
      integer(bits);
    };
    for (const auto *x :
         {&input_.receiver_ref, &input_.context_ref, &input_.source_ref,
          &input_.policy_ref, &input_.policy_revision, &input_.standing,
          &body_preparation_ref_, &body_state_ref_, &eigenbasis_identity_,
          &event_ref_, &subject_ref_})
      text(*x);
    integer(input_.revision);
    integer(body_revision_);
    integer(sample_rate_);
    integer(unsigned(input_.directivity));
    integer(input_.propagation_delay);
    integer(input_.transition_samples);
    for (const auto *vector :
         {&input_.source_translation_metres, &input_.receiver_position_metres,
          &input_.receiver_forward, &source_position_metres_})
      for (double x : *vector)
        real(x);
    for (double x :
         {input_.speed_metres_per_second, input_.minimum_distance_metres,
          distance_metres_, gain_, delay_samples_})
      real(x);
    identity_ = "ql.physical-receiving/fnv1a64-v1:";
    constexpr char digits[] = "0123456789abcdef";
    for (int i = 15; i >= 0; --i)
      identity_.push_back(digits[(hash >> (4 * i)) & 15]);
  }
  const SpatialReceivingInput &input() const noexcept { return input_; }
  const std::string &identity() const noexcept { return identity_; }
  const Vec3 &source_position_metres() const noexcept {
    return source_position_metres_;
  }
  double distance_metres() const noexcept { return distance_metres_; }
  double gain() const noexcept { return gain_; }
  double delay_samples() const noexcept { return delay_samples_; }
  unsigned sample_rate() const noexcept { return sample_rate_; }
  bool
  matches_preparation(const PreparedPhysicalBody &prepared) const noexcept {
    const auto &in = prepared.input();
    return body_preparation_ref_ == in.preparation_ref &&
           body_state_ref_ == in.state_ref &&
           eigenbasis_identity_ == prepared.eigenbasis_identity() &&
           event_ref_ == in.event_ref && subject_ref_ == in.subject_ref &&
           body_revision_ == in.body_revision && sample_rate_ == in.sample_rate;
  }
  // Callback-owned or stopped/acknowledged exclusive custody only.
  bool matches_body(const PhysicalBody &body) const noexcept {
    return matches_preparation(body.preparation());
  }
};
struct SpatialReceivingCheckpoint {
  unsigned version = 1;
  std::string contract = "ql.physical-receiving-checkpoint/v1",
              receiving_identity, body_eigenbasis_identity;
  std::uint64_t samples_elapsed = 0;
  unsigned transition_remaining = 0;
  double effective_gain = 0, effective_delay_samples = 0;
  std::string signal_unit = "linear", distance_unit = "m", speed_unit = "m/s",
              delay_unit = "sample";
  std::vector<float> history_linear;
};
class SpatialReceiving {
  friend class PreparedReceivingTransition;
  PreparedSpatialReceiving prepared_;
  std::array<float, receiving_history_samples> history_{};
  std::uint64_t elapsed_ = 0;
  double gain_ = 0, delay_ = 0;
  unsigned transition_remaining_ = 0;

public:
  // Initialize from immutable source evidence and a host-admitted native
  // cursor; constructing on the control thread never reads mutable P state.
  SpatialReceiving(const PreparedPhysicalBody &body_preparation,
                   std::uint64_t native_admitted_cursor,
                   PreparedSpatialReceiving prepared)
      : prepared_(std::move(prepared)), elapsed_(native_admitted_cursor),
        gain_(prepared_.gain()), delay_(prepared_.delay_samples()) {
    require(prepared_.matches_preparation(body_preparation),
            "disconnected physical receiving preparation");
  }
  const PreparedSpatialReceiving &preparation() const noexcept {
    return prepared_;
  }
  std::uint64_t samples_elapsed() const noexcept { return elapsed_; }
  // Callback only, directly after the same body's single advance operation.
  // The cursor asserts that causal sample span; no separate timebase ticks.
  // Failure leaves transport/output unchanged. Buffers must not overlap.
  bool process_pickup_block(const PhysicalBody &body,
                            const float *pickup_linear, float *received_linear,
                            std::size_t frames,
                            std::uint64_t start_sample) noexcept {
    if (!pickup_linear || !received_linear || frames == 0 ||
        frames > physical_max_frames || start_sample != elapsed_ ||
        elapsed_ > std::numeric_limits<std::uint64_t>::max() - frames ||
        body.samples_elapsed() != elapsed_ + frames ||
        !prepared_.matches_body(body))
      return false;
    const auto input_address = reinterpret_cast<std::uintptr_t>(pickup_linear),
               output_address =
                   reinterpret_cast<std::uintptr_t>(received_linear);
    const auto separation = input_address > output_address
                                ? input_address - output_address
                                : output_address - input_address;
    if (separation < frames * sizeof(float))
      return false;
    for (std::size_t i = 0; i < frames; ++i)
      if (!std::isfinite(pickup_linear[i]))
        return false;
    // Gains never exceed one and interpolation is convex; bounded finite
    // pickup therefore cannot overflow or fail after the first mutation.
    for (std::size_t i = 0; i < frames; ++i) {
      const auto sample = elapsed_ + i;
      history_[sample % receiving_history_samples] = pickup_linear[i];
      if (transition_remaining_) {
        gain_ += (prepared_.gain() - gain_) / transition_remaining_;
        delay_ += (prepared_.delay_samples() - delay_) / transition_remaining_;
        --transition_remaining_;
        if (!transition_remaining_) {
          gain_ = prepared_.gain();
          delay_ = prepared_.delay_samples();
        }
      }
      const auto whole = static_cast<std::uint64_t>(std::floor(delay_));
      const double fraction = delay_ - whole;
      const auto read = [&](std::uint64_t lag) {
        return lag > sample
                   ? 0.0
                   : double(
                         history_[(sample - lag) % receiving_history_samples]);
      };
      received_linear[i] = static_cast<float>(
          gain_ * ((1 - fraction) * read(whole) + fraction * read(whole + 1)));
    }
    elapsed_ += frames;
    return true;
  }
  // Stopped or callback-acknowledged exclusive custody only. Receiver acts
  // preserve propagation history and the actual body; camera has no input.
  bool replace_receiving(const PhysicalBody &body,
                         PreparedSpatialReceiving next,
                         std::uint64_t expected_revision,
                         std::uint64_t expected_sample) {
    const auto &before = prepared_.input();
    const auto &after = next.input();
    if (expected_revision != before.revision || expected_sample != elapsed_ ||
        body.samples_elapsed() != elapsed_ || !prepared_.matches_body(body) ||
        !next.matches_body(body) || after.revision <= before.revision ||
        after.receiver_ref != before.receiver_ref ||
        after.context_ref != before.context_ref)
      return false;
    prepared_ = std::move(next);
    transition_remaining_ = prepared_.input().transition_samples;
    if (!transition_remaining_) {
      gain_ = prepared_.gain();
      delay_ = prepared_.delay_samples();
    }
    return true;
  }
  SpatialReceivingCheckpoint checkpoint(const PhysicalBody &body) const {
    require(prepared_.matches_body(body) && body.samples_elapsed() == elapsed_,
            "stale physical receiving checkpoint");
    SpatialReceivingCheckpoint result;
    result.receiving_identity = prepared_.identity();
    result.body_eigenbasis_identity = body.preparation().eigenbasis_identity();
    result.samples_elapsed = elapsed_;
    result.transition_remaining = transition_remaining_;
    result.effective_gain = gain_;
    result.effective_delay_samples = delay_;
    result.history_linear.assign(history_.begin(), history_.end());
    return result;
  }
  // Restore the body first under exclusive custody, then this transport at
  // that same restored cursor. C retains this packet with A/P checkpoints.
  bool restore_checkpoint(const PhysicalBody &body,
                          const SpatialReceivingCheckpoint &saved,
                          std::uint64_t expected_receiving_sample) noexcept {
    if (expected_receiving_sample != elapsed_ || saved.version != 1 ||
        saved.contract != "ql.physical-receiving-checkpoint/v1" ||
        saved.receiving_identity != prepared_.identity() ||
        !prepared_.matches_body(body) ||
        saved.body_eigenbasis_identity !=
            body.preparation().eigenbasis_identity() ||
        saved.samples_elapsed != body.samples_elapsed() ||
        saved.history_linear.size() != receiving_history_samples ||
        saved.transition_remaining > prepared_.input().transition_samples ||
        !std::isfinite(saved.effective_gain) || saved.effective_gain < 0 ||
        saved.effective_gain > 1 ||
        !std::isfinite(saved.effective_delay_samples) ||
        saved.effective_delay_samples < 0 ||
        saved.effective_delay_samples > receiving_history_samples - 2 ||
        saved.signal_unit != "linear" || saved.distance_unit != "m" ||
        saved.speed_unit != "m/s" || saved.delay_unit != "sample" ||
        (saved.transition_remaining == 0 &&
         (saved.effective_gain != prepared_.gain() ||
          saved.effective_delay_samples != prepared_.delay_samples())))
      return false;
    for (float x : saved.history_linear)
      if (!std::isfinite(x))
        return false;
    std::copy(saved.history_linear.begin(), saved.history_linear.end(),
              history_.begin());
    elapsed_ = saved.samples_elapsed;
    transition_remaining_ = saved.transition_remaining;
    gain_ = saved.effective_gain;
    delay_ = saved.effective_delay_samples;
    return true;
  }
};
} // namespace ql
#endif
