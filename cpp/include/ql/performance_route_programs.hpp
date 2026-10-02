#ifndef QL_PERFORMANCE_ROUTE_PROGRAMS_HPP
#define QL_PERFORMANCE_ROUTE_PROGRAMS_HPP
#include <ql/physical_force_route_port.hpp>
namespace ql::performance {
// Prepared by the existing native source/control owner. The original share is
// retained in the handle; peak_force_newtons ALREADY includes that share.
// A evolves these independent M1 excitation phases at its sole sample cursor.
enum class NativeRouteWaveform : unsigned { Sinusoid = 0 };
struct NativeRouteProgram {
  ql::PhysicalForceRouteProgramHandle handle{};
  ql::PhysicalForcePortRef phase_source_ref{};
  NativeRouteWaveform waveform = NativeRouteWaveform::Sinusoid;
  bool enabled = true;
  double target_gain = 1, effective_gain = 1;
  double sine = 0, cosine = 1;
};
struct NativeRouteProgramSet {
  static constexpr const char *schema = "ql.native-route-programs/v1";
  unsigned version = 1;
  bool owner_suspended = false;
  ql::PhysicalForceRoutePortManifest manifest{};
  std::size_t program_count = 0;
  // This scalar is ONLY admitted Janko/M1 note-voice force. No legacy source
  // waveform or modal-frequency remapping is produced by A.
  bool scalar_note_enabled = true;
  double scalar_note_gain = 1;
  std::array<NativeRouteProgram, ql::physical_max_personal_force_routes>
      programs{};
};
static_assert(std::is_trivially_copyable_v<NativeRouteProgramSet>);
// Engine API in the paired successor:
// bool install_route_programs(const NativeRouteProgramSet&,
//                             const StoppedCustody&, uint64_t expected_cursor)
//                             noexcept;
// Installation requires exact current manifest/immutable port/cursor; all
// programmes represented once. Snapshot/restore retains this complete set
// (including evolving sine/cosine and effective gain), paired to the same P
// q/v.
} // namespace ql::performance
#endif
