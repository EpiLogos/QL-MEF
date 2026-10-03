#ifndef QL_NATIVE_RESIDENT_LIFETIME_HPP
#define QL_NATIVE_RESIDENT_LIFETIME_HPP
#include <array>
#include <atomic>
#include <cstdint>
#include <limits>
#include <random>
#include <stdexcept>
#include <type_traits>
namespace ql {
class PhysicalBody;
namespace performance {
class Engine;
class MovingReceivingPortBinding;
class PerformanceManagement;
} // namespace performance
// Identity only. A token is NOT a source, consent, document, clock or IPC
// grant. JSON/CP never supplies one. Native constructors mint distinct
// process-scoped lifetimes off callback; the qualified private worker channel
// binds origin.
struct NativeResidentToken {
  std::array<std::uint64_t, 2> process_nonce{};
  std::uint64_t ordinal = 0;
  bool valid() const noexcept {
    return ordinal && (process_nonce[0] || process_nonce[1]);
  }
  bool operator==(const NativeResidentToken &other) const noexcept {
    return process_nonce == other.process_nonce && ordinal == other.ordinal;
  }
  bool operator!=(const NativeResidentToken &other) const noexcept {
    return !(*this == other);
  }
};
class NativeResidentLifetime {
  friend class PhysicalBody;
  friend class performance::Engine;
  friend class performance::MovingReceivingPortBinding;
  friend class performance::PerformanceManagement;
  NativeResidentToken token_{};
  static NativeResidentToken mint() {
    // Once per actual native process, on its control-construction path. No
    // wall/audio clock and no filesystem/second persistence store.
    static const std::array<std::uint64_t, 2> nonce = [] {
      std::random_device entropy;
      std::array<std::uint64_t, 2> out{};
      for (auto &word : out)
        word = (std::uint64_t(std::uint32_t(entropy())) << 32) |
               std::uint64_t(std::uint32_t(entropy()));
      if (!out[0] && !out[1])
        throw std::runtime_error("native resident entropy unavailable");
      return out;
    }();
    static std::atomic<std::uint64_t> next{1};
    auto value = next.load(std::memory_order_relaxed);
    for (;;) {
      if (value == std::numeric_limits<std::uint64_t>::max())
        throw std::overflow_error("native resident ordinal exhausted");
      if (next.compare_exchange_weak(value, value + 1,
                                     std::memory_order_relaxed))
        return {nonce, value};
    }
  }
  NativeResidentLifetime() : token_(mint()) {}

public:
  NativeResidentLifetime(const NativeResidentLifetime &) = delete;
  NativeResidentLifetime &operator=(const NativeResidentLifetime &) = delete;
  NativeResidentLifetime(NativeResidentLifetime &&other) noexcept
      : token_(other.token_) {
    other.token_ = {};
  }
  NativeResidentLifetime &operator=(NativeResidentLifetime &&) = delete;
  const NativeResidentToken &token() const noexcept { return token_; }
};
static_assert(std::is_trivially_copyable_v<NativeResidentToken>);
} // namespace ql
#endif
