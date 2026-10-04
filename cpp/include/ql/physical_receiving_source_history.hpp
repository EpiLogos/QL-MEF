#ifndef QL_PHYSICAL_RECEIVING_SOURCE_HISTORY_HPP
#define QL_PHYSICAL_RECEIVING_SOURCE_HISTORY_HPP
// Dated physical emission provenance, owned by the existing receiving
// transport. Exact bounded references are stored once; IDs/equality never grant
// authority.
#include <array>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <ql/physical_receiving.hpp>
#include <string>
namespace ql {
inline constexpr std::size_t receiving_source_segments = 256;
inline constexpr std::size_t receiving_source_reference_bytes = 256 * 1024;
struct ReceivingSourceReference {
  std::uint32_t offset = 0, length = 0;
};
struct ReceivingSourceSegment {
  // source coordinate/revision, P preparation/state/eigenbasis, motion source.
  std::array<ReceivingSourceReference, 6> references{};
  std::uint64_t effective_sample = 0, trajectory_origin_sample = 0,
                body_revision = 0, source_generation = 0;
  std::array<double, 3> anchor_metres{}, velocity_metres_per_second{};
  bool pratibimba = false;
};
struct ReceivingSourceHistory {
  std::array<ReceivingSourceSegment, receiving_source_segments> segments{};
  std::array<char, receiving_source_reference_bytes> references{};
  std::uint32_t count = 0, first = 0, used = 0;
  bool explicit_history = false;
};
inline bool
valid_receiving_source_reference(const ReceivingSourceHistory &h,
                                 ReceivingSourceReference r) noexcept {
  if (!r.length || r.length > 2048 || h.used > h.references.size() ||
      r.offset >= h.used || r.length >= h.used - r.offset ||
      h.references[r.offset + r.length] != '\0')
    return false;
  for (std::uint32_t i = 0; i < r.length; ++i) {
    const auto c = static_cast<unsigned char>(h.references[r.offset + i]);
    if (c < 32 || c == 127)
      return false;
  }
  return true;
}
inline const char *
receiving_source_reference(const ReceivingSourceHistory &h,
                           ReceivingSourceReference r) noexcept {
  return valid_receiving_source_reference(h, r) ? h.references.data() + r.offset
                                                : nullptr;
}
// CONTROL only; never called while P/audio/receiving callback custody runs.
inline bool
intern_receiving_source_reference(ReceivingSourceHistory &h,
                                  const std::string &s,
                                  ReceivingSourceReference &out) noexcept {
  if (s.empty() || s.size() > 2048 || h.used > h.references.size())
    return false;
  for (unsigned char c : s)
    if (c < 32 || c == 127)
      return false;
  for (std::uint32_t at = 0; at < h.used;) {
    const auto end = static_cast<const char *>(
        std::memchr(h.references.data() + at, 0, h.used - at));
    if (!end)
      return false;
    const auto length = std::uint32_t(end - (h.references.data() + at));
    if (length == s.size() &&
        std::memcmp(h.references.data() + at, s.data(), length) == 0) {
      out = {at, length};
      return true;
    }
    at += length + 1;
  }
  if (s.size() + 1 > h.references.size() - h.used)
    return false;
  out = {h.used, std::uint32_t(s.size())};
  std::memcpy(h.references.data() + h.used, s.data(), s.size());
  h.used += std::uint32_t(s.size() + 1);
  return true;
}
inline bool valid_receiving_source_history(const ReceivingSourceHistory &h,
                                           std::uint64_t cursor,
                                           std::uint64_t birth) noexcept {
  if (!h.count || h.count > h.segments.size() || h.first >= h.count ||
      h.used > h.references.size() || birth > cursor)
    return false;
  const auto earliest =
      std::max(birth, cursor >= receiving_history_samples - 1
                          ? cursor - (receiving_history_samples - 1)
                          : birth);
  if (h.segments[0].effective_sample > earliest)
    return false;
  if (h.first && h.segments[h.first].effective_sample > earliest)
    return false;
  for (std::uint32_t i = 0; i < h.count; ++i) {
    const auto &s = h.segments[i];
    if (!s.body_revision || s.effective_sample < birth ||
        s.effective_sample > cursor ||
        s.trajectory_origin_sample > s.effective_sample ||
        (i && s.effective_sample <= h.segments[i - 1].effective_sample))
      return false;
    for (auto r : s.references)
      if (!valid_receiving_source_reference(h, r))
        return false;
    for (double x : s.anchor_metres)
      if (!std::isfinite(x) || std::abs(x) > 1e6)
        return false;
    for (double x : s.velocity_metres_per_second)
      if (!std::isfinite(x))
        return false;
  }
  return true;
}
inline bool
same_receiving_source_history(const ReceivingSourceHistory &a,
                              const ReceivingSourceHistory &b) noexcept {
  if (a.count != b.count || a.first != b.first ||
      a.explicit_history != b.explicit_history)
    return false;
  for (std::uint32_t i = 0; i < a.count; ++i) {
    const auto &x = a.segments[i], &y = b.segments[i];
    if (x.effective_sample != y.effective_sample ||
        x.trajectory_origin_sample != y.trajectory_origin_sample ||
        x.body_revision != y.body_revision ||
        x.source_generation != y.source_generation ||
        x.pratibimba != y.pratibimba || x.anchor_metres != y.anchor_metres ||
        x.velocity_metres_per_second != y.velocity_metres_per_second)
      return false;
    for (std::size_t j = 0; j < x.references.size(); ++j) {
      const auto *p = receiving_source_reference(a, x.references[j]),
                 *q = receiving_source_reference(b, y.references[j]);
      if (!p || !q || std::strcmp(p, q))
        return false;
    }
  }
  return true;
}
} // namespace ql
#endif
