#ifndef QL_PERFORMANCE_RESIDENT_REGISTRY_HPP
#define QL_PERFORMANCE_RESIDENT_REGISTRY_HPP
#include <ql/performance_audio.hpp>
namespace ql::performance {
enum class NativeResidentRole : std::uint8_t {
  AudioEngine,
  PhysicalBody,
  AcousticReceiving
};
struct NativeResidentRegistration {
  NativeResidentRole role = NativeResidentRole::AudioEngine;
  ql::NativeResidentToken token{};
  // Actual native construction ordinal, NOT M2/M3/body/material revision.
  // Source generations remain explicit separate independently typed fields.
  std::uint64_t generation = 0, samples_elapsed = 0;
  bool callback_output_committed = false;
};
struct NativeResidentRegistry {
  unsigned version = 1, count = 0;
  std::array<NativeResidentRegistration, 3> consumers{};
  std::uint64_t transport_epoch = 0;
  // Exact snapshot of one native queue/output boundary; no extra pulse/drain.
  Readback boundary{};
};
static_assert(std::is_trivially_copyable_v<NativeResidentRegistration>);
} // namespace ql::performance
#endif
