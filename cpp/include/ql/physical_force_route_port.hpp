#ifndef QL_PHYSICAL_FORCE_ROUTE_PORT_HPP
#define QL_PHYSICAL_FORCE_ROUTE_PORT_HPP
// Immutable numerical/source handles crossing the existing A/P boundary.
// No source compiler, oscillator, clock, queue or physical state lives here.
#include <ql/physical_force_routes.hpp>
#include <type_traits>
namespace ql {
using PhysicalForcePortRef = std::array<char, 256>;
inline constexpr const char *physical_force_route_port_contract =
    "ql.physical-force-route-port/v1";
struct PhysicalForceRouteProgramHandle {
  PhysicalForcePortRef driver_ref{}, target_ref{}, program_ref{},
      planet_coordinate{}, chakra_coordinate{}, projection_ref{},
      calibration_ref{}, calibration_revision{}, calibration_source_ref{},
      calibration_standing{};
  std::size_t route_index = 0;
  std::uint64_t preparation_seal = 0, program_seal = 0;
  QL_M_NodeId planet_node_id = 0, chakra_node_id = 0;
  unsigned native_planet_index = 0, centre_ordinal = 0;
  std::uint64_t share_numerator = 0, share_denominator = 0;
  double source_hertz = 0, original_denominator_share = 0,
         peak_force_newtons = 0;
};
struct PhysicalForceRoutePortManifest {
  unsigned version = 1, sample_rate = 0;
  std::size_t route_count = 0;
  std::uint64_t source_basis_seal = 0, body_revision = 0,
                admitted_cursor = 0, m1_revision = 0, m2_generation = 0,
                m3_generation = 0, m3_input_generation = 0;
  QL_M_NodeId earth_frame_node_id = 0;
  PhysicalForcePortRef event_ref{}, subject_ref{}, registry_revision{},
      source_revision{}, definition_ref{}, source_instance_ref{},
      determination_ref{}, preparation_ref{}, state_ref{}, eigenbasis_identity{},
      m1_coordinate{}, m2_writer_coordinate{}, native_basis_sha256{},
      m3_state_sha256{};
  bool m1_pratibimba = false, m2_pratibimba = false;
  unsigned tick12 = 0, degree720 = 0, temporal_phase = 0;
  // This scalar is A's source-admitted M1 NOTE excitation, independent of
  // the original N programmes. Personal receiving must remain playable.
  bool scalar_note_enabled = true;
  double scalar_note_gain = 1;
  // The older aggregate native-source scalar would duplicate N energy.
  // It has no generator or automatic enabling in the joined A callback.
  bool legacy_native_scalar_enabled = false;
  double legacy_native_scalar_gain = 0, max_force_newtons = 0;
  std::array<PhysicalForceRouteProgramHandle,
             physical_max_personal_force_routes> programs{};
};
static_assert(std::is_trivially_copyable_v<PhysicalForceRoutePortManifest>,
              "route handles must cross the bounded native queue by value");
} // namespace ql
#endif
