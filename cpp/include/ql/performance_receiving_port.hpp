#ifndef QL_PERFORMANCE_RECEIVING_PORT_HPP
#define QL_PERFORMANCE_RECEIVING_PORT_HPP
// Fixed native receiving state. Source/context authority is admitted by the
// existing serial owner; numerical compatibility is not a grant. Same A/P
// cursor, no standalone renderer or clock. No strings/allocations in callback.
#include <algorithm>
#include <cmath>
#include <cstring>
#include <memory>
#include <ql/physical_moving_receiving.hpp>
#include <type_traits>
namespace ql::performance {
using ReceivingRef = std::array<char, 2049>;
struct NativeReceivingManifest {
  unsigned version = 1, sample_rate = 0;
  ReceivingRef receiving_identity{}, event{}, subject{}, preparation{}, state{},
      source_coordinate{}, source_revision{}, eigenbasis{}, receiver{},
      context{}, source_motion{}, receiver_motion{}, policy{},
      policy_revision{}, standing{};
  // Actual first admitted pickup cursor; immutable native instance birth.
  // MovingSpatialReceiving history_start_ retains this birth, not the sliding
  // ring retention boundary. Never infer it from current cursor or labels.
  std::uint64_t source_generation = 0, body_revision = 0, origin_sample = 0,
                end_sample = 0, history_origin_sample = 0;
  bool pratibimba = false;
};
struct NativeReceivingCheckpoint {
  unsigned version = 1;
  NativeReceivingManifest manifest{};
  std::uint64_t samples_elapsed = 0, history_start_sample = 0;
  std::array<float, ql::receiving_history_samples> history_linear{};
};
struct NativeReceivingReadback {
  NativeReceivingManifest manifest{};
  std::uint64_t samples_elapsed = 0;
  ql::SpatialMotionPoint end_position{};
};
struct ReceivingPort {
  void *owner = nullptr;
  const void *body_owner = nullptr;
  const NativeReceivingManifest *manifest = nullptr;
  std::shared_ptr<void> custody{};
  bool (*preflight)(const void *, std::size_t,
                    std::uint64_t) noexcept = nullptr;
  bool (*process)(void *, const float *, float *, std::size_t,
                  std::uint64_t) noexcept = nullptr;
  std::uint64_t (*cursor)(const void *) noexcept = nullptr;
  bool (*observe)(const void *, NativeReceivingReadback &,
                  std::uint64_t) noexcept = nullptr;
  bool (*write_checkpoint)(const void *, NativeReceivingCheckpoint &,
                           std::uint64_t) noexcept = nullptr;
  bool (*validate_checkpoint)(const void *, const NativeReceivingCheckpoint &,
                              std::uint64_t) noexcept = nullptr;
  // Called only after paired P+audio preflight/restore under the same stopped
  // guard. Every failure condition was checked before first state mutation.
  void (*restore_checkpoint)(
      void *, const NativeReceivingCheckpoint &) noexcept = nullptr;
};
inline bool same_receiving_manifest(const NativeReceivingManifest &a,
                                    const NativeReceivingManifest &b) noexcept {
  return a.version == b.version && a.sample_rate == b.sample_rate &&
         a.receiving_identity == b.receiving_identity && a.event == b.event &&
         a.subject == b.subject && a.preparation == b.preparation &&
         a.state == b.state && a.source_coordinate == b.source_coordinate &&
         a.source_revision == b.source_revision &&
         a.eigenbasis == b.eigenbasis && a.receiver == b.receiver &&
         a.context == b.context && a.source_motion == b.source_motion &&
         a.receiver_motion == b.receiver_motion && a.policy == b.policy &&
         a.policy_revision == b.policy_revision && a.standing == b.standing &&
         a.source_generation == b.source_generation &&
         a.body_revision == b.body_revision &&
         a.origin_sample == b.origin_sample && a.end_sample == b.end_sample &&
         a.history_origin_sample == b.history_origin_sample &&
         a.pratibimba == b.pratibimba;
}
inline bool
valid_receiving_manifest(const NativeReceivingManifest &m) noexcept {
  if (m.version != 1 || m.sample_rate < 8000 || m.sample_rate > 192000 ||
      !m.body_revision || m.end_sample <= m.origin_sample ||
      m.history_origin_sample < m.origin_sample ||
      m.history_origin_sample >= m.end_sample ||
      m.end_sample - m.origin_sample > std::uint64_t(m.sample_rate) * 60)
    return false;
  for (const auto *ref :
       {&m.receiving_identity, &m.event, &m.subject, &m.preparation, &m.state,
        &m.source_coordinate, &m.source_revision, &m.eigenbasis, &m.receiver,
        &m.context, &m.source_motion, &m.receiver_motion, &m.policy,
        &m.policy_revision, &m.standing}) {
    if (!(*ref)[0])
      return false;
    const auto end = std::find(ref->begin(), ref->end(), '\0');
    if (end == ref->end() ||
        !std::all_of(end, ref->end(), [](char c) { return c == '\0'; }))
      return false;
    for (auto it = ref->begin(); it != end; ++it)
      if (static_cast<unsigned char>(*it) < 32 || *it == 127)
        return false;
  }
  return std::strcmp(m.standing.data(), "architecture-model") == 0 ||
         std::strcmp(m.standing.data(), "reference") == 0 ||
         std::strcmp(m.standing.data(), "tunable-model") == 0;
}
inline bool
valid_receiving_checkpoint(const NativeReceivingCheckpoint &cp,
                           const NativeReceivingManifest &m) noexcept {
  if (cp.version != 1 || !valid_receiving_manifest(m) ||
      !same_receiving_manifest(cp.manifest, m) ||
      cp.samples_elapsed < m.origin_sample ||
      cp.samples_elapsed > m.end_sample ||
      cp.history_start_sample != m.history_origin_sample ||
      cp.history_start_sample > cp.samples_elapsed)
    return false;
  for (float x : cp.history_linear)
    if (!std::isfinite(x))
      return false;
  return true;
}
static_assert(std::is_trivially_copyable_v<NativeReceivingCheckpoint>);
static_assert(std::is_trivially_copyable_v<NativeReceivingReadback>);
} // namespace ql::performance
#endif
