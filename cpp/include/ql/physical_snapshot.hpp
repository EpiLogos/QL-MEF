#ifndef QL_PHYSICAL_SNAPSHOT_HPP
#define QL_PHYSICAL_SNAPSHOT_HPP
// Called exclusively by the audio callback after the single physical advance,
// or by a stopped/callback-acknowledged owner. UI receives this value through
// A's existing bounded readback path, never reads the mutable body directly.
#include <ql/physical_body.hpp>
#include <type_traits>
namespace ql {
using PhysicalSnapshotRef = std::array<char, 2049>;
struct PhysicalSnapshot {
  unsigned version = 1;
  PhysicalSnapshotRef event_ref{}, subject_ref{}, preparation_ref{},
      state_ref{}, source_coordinate{}, source_revision{}, geometry_ref{},
      geometry_revision{}, material_ref{}, material_revision{},
      eigenbasis_identity{};
  std::uint64_t source_generation = 0, body_revision = 0, samples_elapsed = 0;
  unsigned sample_rate = 0, node_count = 0;
  bool pratibimba = false;
  double pickup_linear = 0, mechanical_energy_joules = 0;
  std::array<std::uint64_t, physical_max_nodes> node_identity{};
  // Local metric geometry; the existing host's admitted transform places it
  // in the scene. Position means rest+displacement, not shader phase/glyph.
  std::array<Vec3, physical_max_nodes> visible_positions_metres{};
};
inline bool write_physical_snapshot(const PhysicalBody &body,
                                    PhysicalSnapshot &output,
                                    std::uint64_t expected_body_revision,
                                    std::uint64_t end_sample) noexcept {
  const auto &in = body.preparation().input();
  if (expected_body_revision != body.body_revision() ||
      end_sample != body.samples_elapsed() ||
      in.nodes.size() > physical_max_nodes)
    return false;
  const auto valid = [](const std::string &source) {
    return !source.empty() && source.size() < PhysicalSnapshotRef{}.size();
  };
  for (const auto *source :
       {&in.event_ref, &in.subject_ref, &in.preparation_ref, &in.state_ref,
        &in.source_coordinate, &in.source_revision, &in.geometry_ref,
        &in.geometry_revision, &in.material.reference, &in.material.revision,
        &body.preparation().eigenbasis_identity()})
    if (!valid(*source))
      return false;
  PhysicalSnapshot candidate;
  const auto copy = [](const std::string &source, PhysicalSnapshotRef &target) {
    std::memcpy(target.data(), source.data(), source.size());
    target[source.size()] = '\0';
  };
  copy(in.event_ref, candidate.event_ref);
  copy(in.subject_ref, candidate.subject_ref);
  copy(in.preparation_ref, candidate.preparation_ref);
  copy(in.state_ref, candidate.state_ref);
  copy(in.source_coordinate, candidate.source_coordinate);
  copy(in.source_revision, candidate.source_revision);
  copy(in.geometry_ref, candidate.geometry_ref);
  copy(in.geometry_revision, candidate.geometry_revision);
  copy(in.material.reference, candidate.material_ref);
  copy(in.material.revision, candidate.material_revision);
  copy(body.preparation().eigenbasis_identity(), candidate.eigenbasis_identity);
  candidate.source_generation = in.source_generation;
  candidate.body_revision = in.body_revision;
  candidate.samples_elapsed = end_sample;
  candidate.sample_rate = in.sample_rate;
  candidate.node_count = unsigned(in.nodes.size());
  candidate.pratibimba = in.pratibimba;
  for (std::size_t n = 0; n < in.nodes.size(); ++n)
    candidate.node_identity[n] = in.nodes[n].identity;
  if (!body.write_visible_positions(candidate.visible_positions_metres.data(),
                                    in.nodes.size(), expected_body_revision,
                                    end_sample))
    return false;
  const auto observation = body.observation();
  candidate.pickup_linear = observation.pickup_linear;
  candidate.mechanical_energy_joules = observation.mechanical_energy_joules;
  if (!std::isfinite(candidate.pickup_linear) ||
      !std::isfinite(candidate.mechanical_energy_joules))
    return false;
  output = candidate;
  return true;
}
static_assert(
    std::is_trivially_copyable_v<PhysicalSnapshot>,
    "physical snapshot must cross the existing bounded queue by value");
} // namespace ql
#endif
