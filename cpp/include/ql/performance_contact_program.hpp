#ifndef QL_PERFORMANCE_CONTACT_PROGRAM_HPP
#define QL_PERFORMANCE_CONTACT_PROGRAM_HPP
// Immutable control preparation and fixed callback operands. Numerical contact
// preparation is not source/Scene/Act/consent authority. Engine owns admission,
// the only queue, slot generations and sole P output commit.
#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <ql/physical_contact.hpp>
#include <string>
#include <type_traits>

namespace ql::performance {
inline constexpr std::size_t contact_slot_capacity = 16;
inline constexpr std::size_t contact_reference_bytes = 192;
using ContactReference = std::array<char, contact_reference_bytes>;
struct NativeContactHandle {
  std::uint64_t generation = 0;
  std::uint8_t slot = 255;
  bool valid() const noexcept {
    return slot < contact_slot_capacity && generation != 0;
  }
  bool operator==(const NativeContactHandle &b) const noexcept {
    return slot == b.slot && generation == b.generation;
  }
};
enum class NativeContactStatus : std::uint8_t {
  Empty,
  Queued,
  Delivering,
  Completed,
  Refused,
  Interrupted
};
enum class NativeContactRefusal : std::uint8_t {
  None,
  CurrentSourceChanged,
  PhysicalPreparationChanged,
  EmergencyFenced,
  ForceBudgetChanged
};
// One Kind7 start/refusal application carries these bounded ORIGINAL operands.
// Delivery progression is a separate same-output-commit observation, never a
// fabricated NoteOff or a second admission/application ordinal.
struct NativeContactOperands {
  ContactReference contact_ref{}, particle_ref{}, collider_ref{}, route_ref{},
      source_ref{}, policy_ref{}, policy_revision{}, standing{},
      seed_standing{}, preparation_ref{}, state_ref{}, eigenbasis{},
      source_coordinate{}, source_revision{};
  ql::Vec3 plane_position_metres{}, normal{},
      impact_velocity_metres_per_second{};
  double height_metres = 0, initial_normal_velocity_metres_per_second = 0,
         gravity_metres_per_second_squared = 0, mass_kg = 0, restitution = 0,
         transfer_fraction = 0, minimum_impact_speed_metres_per_second = 0,
         impact_seconds = 0, impact_speed_metres_per_second = 0,
         planned_impulse_newton_seconds = 0, force_newtons = 0;
  std::uint64_t trigger_sample = 0, impact_sample = 0, body_revision = 0,
                source_generation = 0, seed = 0;
  unsigned sample_rate = 0;
  std::uint32_t duration_samples = 0;
  bool pratibimba = false;
};
struct NativeContactDelivery {
  NativeContactHandle handle{};
  NativeContactStatus status = NativeContactStatus::Empty;
  NativeContactRefusal refusal = NativeContactRefusal::None;
  std::uint64_t admission_sequence = 0, start_application_ordinal = 0,
                committed_cursor = 0, requested_impact_sample = 0,
                admitted_impact_sample = 0, actual_impact_sample = 0;
  std::uint32_t delivered_frames = 0, planned_frames = 0;
  double delivered_impulse_newton_seconds = 0,
         planned_impulse_newton_seconds = 0;
};
// Allocates only on native control. All callback-visible fields are immutable.
// The original input is retained separately from its dated numerical force.
// Native source admission still requires actual Engine/Management/current
// private Scene/Act source replay and final callback PhysicalPort validation.
class PreparedContactProgram {
  ql::GravityContactInput original_;
  ql::PhysicalBodyInput original_body_;
  ql::PreparedContactForce force_;
  NativeContactOperands operands_{};
  static ContactReference fixed(const std::string &value) {
    ql::reference(value);
    ql::require(value.size() < contact_reference_bytes,
                "contact reference exceeds native application bound");
    ContactReference out{};
    std::copy(value.begin(), value.end(), out.begin());
    return out;
  }

public:
  static constexpr const char *schema = "ql.performance-contact-program/v1";
  PreparedContactProgram(const ql::PreparedPhysicalBody &body,
                         std::uint64_t native_trigger_cursor,
                         const ql::GravityContactInput &original)
      : original_(original), original_body_(body.input()),
        force_(ql::prepare_gravity_contact(body, native_trigger_cursor,
                                           original)) {
    ql::require(
        ql::contact_matches_preparation(force_, body, native_trigger_cursor),
        "contact is detached from immutable physical preparation");
    operands_.contact_ref = fixed(original_.contact_ref);
    operands_.particle_ref = fixed(original_.particle_ref);
    operands_.collider_ref = fixed(original_.collider_ref);
    operands_.route_ref = fixed(original_.route_ref);
    operands_.source_ref = fixed(original_.source_ref);
    operands_.policy_ref = fixed(original_.policy_ref);
    operands_.policy_revision = fixed(original_.policy_revision);
    operands_.standing = fixed(original_.standing);
    operands_.seed_standing = fixed(force_.seed_standing);
    operands_.preparation_ref = fixed(force_.body_preparation_ref);
    operands_.state_ref = fixed(force_.body_state_ref);
    operands_.eigenbasis = fixed(force_.eigenbasis_identity);
    operands_.source_coordinate = fixed(force_.source_coordinate);
    operands_.source_revision = fixed(force_.source_revision);
    operands_.plane_position_metres = original_.plane_position_metres;
    operands_.normal = original_.normal;
    operands_.impact_velocity_metres_per_second =
        force_.impact_velocity_metres_per_second;
    operands_.height_metres = original_.height_metres;
    operands_.initial_normal_velocity_metres_per_second =
        original_.initial_normal_velocity_metres_per_second;
    operands_.gravity_metres_per_second_squared =
        original_.gravity_metres_per_second_squared;
    operands_.mass_kg = original_.mass_kg;
    operands_.restitution = original_.restitution;
    operands_.transfer_fraction = original_.transfer_fraction;
    operands_.minimum_impact_speed_metres_per_second =
        original_.minimum_impact_speed_metres_per_second;
    operands_.impact_seconds = force_.impact_seconds;
    operands_.impact_speed_metres_per_second =
        force_.impact_speed_metres_per_second;
    operands_.planned_impulse_newton_seconds = force_.impulse_newton_seconds;
    operands_.force_newtons = force_.force_newtons[0];
    operands_.trigger_sample = original_.start_sample;
    operands_.impact_sample = force_.start_sample;
    operands_.body_revision = force_.body_revision;
    operands_.source_generation = force_.source_generation;
    operands_.seed = force_.seed;
    operands_.sample_rate = force_.sample_rate;
    operands_.duration_samples = force_.frames;
    operands_.pratibimba = force_.pratibimba;
  }
  const ql::GravityContactInput &original() const noexcept { return original_; }
  const ql::PhysicalBodyInput &original_body() const noexcept {
    return original_body_;
  }
  const ql::PreparedContactForce &force() const noexcept { return force_; }
  const NativeContactOperands &operands() const noexcept { return operands_; }
  bool matches_preparation(const ql::PreparedPhysicalBody &body,
                           std::uint64_t admitted_cursor) const noexcept {
    return ql::contact_matches_preparation(force_, body, admitted_cursor);
  }
};
static_assert(std::is_trivially_copyable_v<NativeContactHandle>);
static_assert(std::is_trivially_copyable_v<NativeContactOperands>);
static_assert(std::is_trivially_copyable_v<NativeContactDelivery>);
} // namespace ql::performance
#endif
