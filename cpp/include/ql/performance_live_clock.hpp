#ifndef QL_PERFORMANCE_LIVE_CLOCK_HPP
#define QL_PERFORMANCE_LIVE_CLOCK_HPP
#include <ql/performance_audio.hpp>

namespace ql::performance {
// Qualified only by the serial native owner. DOM/performance.now timestamps
// must never be passed as native host ticks. BridgeReceipt explicitly leaves
// the earlier physical-input transit unmeasured.
enum class NativeInputOrigin : std::uint8_t { BridgeReceipt, NativeEvent };
struct NativeInputStamp {
  NativeInputOrigin origin = NativeInputOrigin::BridgeReceipt;
  std::uint64_t host_ticks = 0, uncertainty_ticks = 0;
};
struct NativeClockAdmission {
  Result result = Result::Unavailable;
  std::uint64_t epoch = 0, anchor_ordinal = 0, trigger_host_ticks = 0,
                admitted_host_ticks = 0, requested_sample = 0,
                accepted_sample = 0, sequence = 0;
  double mapping_uncertainty_samples = 0;
  bool input_transit_unknown = true;
};
/** Callback timestamps establish the sole output mapping. The control owner
 * reads at most three atomic snapshots: no waits/locks/allocations and no
 * private advancement of the native sample cursor. All fields use lock-free
 * atomics to avoid the data race of a plain struct behind a seqlock. */
class NativeOutputClock {
  static_assert(std::atomic<std::uint64_t>::is_always_lock_free,
                "native output clock requires lock-free u64");
  struct Anchor {
    std::uint64_t epoch = 0, ordinal = 0, host = 0, start = 0, frames = 0;
  };
  std::atomic<std::uint64_t> stamp_{0}, epoch_{0}, ordinal_{0}, host_{0},
      start_{0}, frames_{0};
  unsigned rate_ = 0;
  double seconds_per_tick_ = 0;
  std::uint64_t mapping_numerator_ = 0, mapping_denominator_ = 0;
  std::uint64_t active_epoch_ = 0, next_ordinal_ = 0;
  bool anchor(Anchor &out) const noexcept {
    for (unsigned trial = 0; trial < 3; ++trial) {
      const auto before = stamp_.load(std::memory_order_acquire);
      if (before & 1u)
        continue;
      out = {epoch_.load(std::memory_order_relaxed),
             ordinal_.load(std::memory_order_relaxed),
             host_.load(std::memory_order_relaxed),
             start_.load(std::memory_order_relaxed),
             frames_.load(std::memory_order_relaxed)};
      if (stamp_.load(std::memory_order_acquire) == before)
        return out.epoch && out.host && out.ordinal && out.frames;
    }
    return false;
  }

public:
  // Actual stopped/acknowledged device custody only; epoch advances on a new
  // device start/recovery. Numerator/denominator come from mach_timebase_info.
  bool configure(unsigned rate, std::uint64_t epoch, std::uint32_t numerator,
                 std::uint32_t denominator) noexcept {
    if (rate != 48000 || !epoch || epoch <= active_epoch_ || !numerator ||
        !denominator)
      return false;
    rate_ = rate;
    seconds_per_tick_ = double(numerator) / double(denominator) * 1e-9;
    mapping_numerator_ = std::uint64_t(numerator) * rate;
    mapping_denominator_ = std::uint64_t(denominator) * 1000000000u;
    active_epoch_ = epoch;
    next_ordinal_ = 0;
    invalidate();
    return true;
  }
  // Sole callback writer, or serial control after actual OS stop.
  void invalidate() noexcept {
    stamp_.fetch_add(1, std::memory_order_acq_rel);
    epoch_.store(0, std::memory_order_relaxed);
    host_.store(0, std::memory_order_relaxed);
    stamp_.fetch_add(1, std::memory_order_release);
  }
  bool publish_callback(std::uint64_t host_ticks, std::uint64_t start_sample,
                        std::size_t frames, bool continuous) noexcept {
    if (!continuous || !active_epoch_ || !host_ticks || !frames ||
        frames > max_frames ||
        start_sample > std::numeric_limits<std::uint64_t>::max() - frames ||
        next_ordinal_ == std::numeric_limits<std::uint64_t>::max()) {
      invalidate();
      return false;
    }
    stamp_.fetch_add(1, std::memory_order_acq_rel);
    epoch_.store(active_epoch_, std::memory_order_relaxed);
    ordinal_.store(++next_ordinal_, std::memory_order_relaxed);
    host_.store(host_ticks, std::memory_order_relaxed);
    start_.store(start_sample, std::memory_order_relaxed);
    frames_.store(frames, std::memory_order_relaxed);
    stamp_.fetch_add(1, std::memory_order_release);
    return true;
  }
  NativeClockAdmission enqueue(Engine &engine, Operation operation,
                               NativeInputStamp trigger,
                               std::uint64_t native_now_ticks) const noexcept {
    NativeClockAdmission receipt{};
    Anchor a{};
    if (!anchor(a) || !engine.device_callbacks_running() ||
        engine.sample_rate() != rate_ || !trigger.host_ticks ||
        trigger.host_ticks > native_now_ticks || !native_now_ticks)
      return receipt;
    const double anchor_age =
        native_now_ticks >= a.host
            ? double(native_now_ticks - a.host) * seconds_per_tick_
            : -double(a.host - native_now_ticks) * seconds_per_tick_;
    const double input_age =
        double(native_now_ticks - trigger.host_ticks) * seconds_per_tick_;
    const double uncertainty =
        double(trigger.uncertainty_ticks) * seconds_per_tick_ * rate_ + 1.0;
    // A future AUHAL output timestamp is legal within one bounded slice plus
    // its reported pipeline. Stale/missing anchors never invent a clock.
    if (!std::isfinite(anchor_age) || std::abs(anchor_age) > 0.05 ||
        !std::isfinite(input_age) || input_age > 0.25 ||
        !std::isfinite(uncertainty) || uncertainty > rate_ * 0.03)
      return receipt;
    if (operation.kind != Kind::NoteOn && operation.kind != Kind::NoteOff &&
        operation.kind != Kind::Expression && operation.kind != Kind::Sustain &&
        operation.kind != Kind::Panic && operation.kind != Kind::Parameter)
      return receipt;
    const bool forward = trigger.host_ticks >= a.host;
    const auto ticks =
        forward ? trigger.host_ticks - a.host : a.host - trigger.host_ticks;
    if (ticks > std::numeric_limits<std::uint64_t>::max() / mapping_numerator_)
      return receipt;
    const auto product = ticks * mapping_numerator_;
    const auto whole = product / mapping_denominator_;
    const auto remainder = product % mapping_denominator_;
    if (whole > std::uint64_t(rate_) * 3 / 10)
      return receipt;
    // Exact ceil of the signed rational offset. Neither a large u64 cursor
    // nor an exact sample boundary is converted to floating point.
    const auto offset =
        forward ? std::int64_t(whole + (remainder != 0)) : -std::int64_t(whole);
    if (offset > 0 && a.start > std::numeric_limits<std::uint64_t>::max() -
                                    std::uint64_t(offset))
      return receipt;
    receipt.epoch = a.epoch;
    receipt.anchor_ordinal = a.ordinal;
    receipt.trigger_host_ticks = trigger.host_ticks;
    receipt.admitted_host_ticks = native_now_ticks;
    receipt.requested_sample = offset < 0
                                   ? (a.start >= std::uint64_t(-offset)
                                          ? a.start - std::uint64_t(-offset)
                                          : 0)
                                   : a.start + std::uint64_t(offset);
    receipt.mapping_uncertainty_samples = uncertainty;
    receipt.input_transit_unknown =
        trigger.origin == NativeInputOrigin::BridgeReceipt;
    operation.sample =
        std::max(receipt.requested_sample, engine.admission_horizon());
    operation.native_clock = {
        a.epoch,          a.ordinal,   trigger.host_ticks,
        native_now_ticks, uncertainty, receipt.input_transit_unknown};
    receipt.result = engine.enqueue_native_admitted(operation);
    receipt.accepted_sample =
        receipt.result == Result::Accepted ? engine.last_admitted_sample() : 0;
    receipt.sequence =
        receipt.result == Result::Accepted ? operation.sequence : 0;
    return receipt;
  }
};
} // namespace ql::performance
#endif
