#ifndef QL_PHYSICAL_CONTACT_HPP
#define QL_PHYSICAL_CONTACT_HPP
// Control-thread contact preparation. The analytic particle has one physical
// degree of freedom normal to a declared rigid plane. It produces a dated force
// for A's existing queue, never an additional body integrator or audio clock.
#include <ql/physical_body.hpp>

namespace ql {
inline constexpr const char *physical_contact_contract =
    "ql.physical-contact/v1";
struct GravityContactInput {
  std::string contact_ref, particle_ref, collider_ref, route_ref, source_ref,
      policy_ref, policy_revision, standing;
  Vec3 plane_position_metres{}, normal{0, 1, 0};
  double height_metres = 0, initial_normal_velocity_metres_per_second = 0;
  double gravity_metres_per_second_squared = 9.81, mass_kg = 0;
  double restitution = 0, transfer_fraction = 1,
         minimum_impact_speed_metres_per_second = 0;
  std::uint64_t start_sample = 0;
  std::uint32_t duration_samples = 1;
};
struct PreparedContactForce {
  std::string contract = physical_contact_contract;
  std::string contact_ref, particle_ref, collider_ref, route_ref, dedup_ref,
      source_ref, policy_ref, policy_revision, standing;
  std::string event_ref, subject_ref, body_preparation_ref, body_state_ref,
      source_coordinate, source_revision, eigenbasis_identity;
  std::uint64_t source_generation = 0, body_revision = 0, start_sample = 0;
  unsigned sample_rate = 0;
  bool pratibimba = false;
  Vec3 position_metres{}, normal{}, impact_velocity_metres_per_second{};
  double impact_seconds = 0, impact_speed_metres_per_second = 0,
         impulse_newton_seconds = 0;
  std::uint32_t frames = 0;
  std::array<double, physical_max_frames> force_newtons{};
  // No stochastic input exists in this analytic mechanism. A retained seed
  // remains explicit instead of implying that an arbitrary seed drove it.
  std::string seed_standing = "deterministic-no-randomness";
  std::uint64_t seed = 0;
};
// Control owner supplies a retained immutable preparation and a cursor admitted
// from its native readback/timeline. Never inspect the live mutable body here.
// This packet is a proposal; A still checks the current body at delivery.
inline PreparedContactForce
prepare_gravity_contact(const PreparedPhysicalBody &prepared,
                        std::uint64_t native_admitted_cursor,
                        const GravityContactInput &source) {
  const auto &in = prepared.input();
  for (const auto *ref :
       {&source.contact_ref, &source.particle_ref, &source.collider_ref,
        &source.route_ref, &source.source_ref, &source.policy_ref,
        &source.policy_revision, &source.standing})
    reference(*ref);
  require(source.standing == "architecture-model" ||
              source.standing == "reference" ||
              source.standing == "tunable-model",
          "contact physical magnitudes need declared model standing");
  const auto bounded = [](double x, double low, double high) {
    return std::isfinite(x) && x >= low && x <= high;
  };
  require(bounded(source.height_metres, 0, 100) &&
              bounded(source.initial_normal_velocity_metres_per_second, -1e4,
                      1e4) &&
              bounded(source.gravity_metres_per_second_squared, 1e-6, 1e4) &&
              bounded(source.mass_kg, 1e-12, 1e3) &&
              bounded(source.restitution, 0, 1) &&
              bounded(source.transfer_fraction, 0, 1) &&
              bounded(source.minimum_impact_speed_metres_per_second, 0, 1e4),
          "invalid gravity contact physical quantities");
  require(source.duration_samples > 0 &&
              source.duration_samples <= physical_max_frames &&
              source.start_sample == native_admitted_cursor,
          "contact duration or native source cursor differs");
  double norm = 0;
  for (unsigned a = 0; a < 3; ++a) {
    require(bounded(source.plane_position_metres[a], -1e6, 1e6) &&
                bounded(source.normal[a], -1, 1),
            "invalid contact geometry");
    norm += source.normal[a] * source.normal[a];
  }
  require(std::abs(norm - 1) <= 1e-10, "contact normal must be a unit vector");
  const long double h = source.height_metres,
                    v = source.initial_normal_velocity_metres_per_second,
                    g = source.gravity_metres_per_second_squared;
  const long double speed = std::sqrt(v * v + 2 * g * h);
  // Stable quadratic root avoids subtracting nearly equal values for a
  // particle already travelling rapidly toward the plane.
  const long double seconds =
      v < 0 ? (h == 0 ? 0 : 2 * h / (speed - v)) : (v + speed) / g;
  require(std::isfinite(seconds) && seconds <= 86400,
          "contact time exceeds preparation budget");
  const long double offset = std::ceil(seconds * in.sample_rate);
  require(source.start_sample <= std::numeric_limits<std::uint64_t>::max() -
                                     source.duration_samples &&
              offset <= static_cast<long double>(
                            std::numeric_limits<std::uint64_t>::max() -
                            source.start_sample - source.duration_samples),
          "contact native sample range exhausted");
  double projection = 0;
  for (unsigned a = 0; a < 3; ++a)
    projection -= source.normal[a] * in.exciter.axis[a];
  const double impulse = speed < source.minimum_impact_speed_metres_per_second
                             ? 0
                             : source.transfer_fraction *
                                   (1 + source.restitution) * source.mass_kg *
                                   static_cast<double>(speed) * projection;
  const double force = impulse * in.sample_rate / source.duration_samples;
  require(std::isfinite(impulse) &&
              std::abs(impulse) <= in.max_impulse_newton_seconds &&
              std::isfinite(force) && std::abs(force) <= in.max_force_newtons,
          "contact transfer exceeds prepared physical body limits");
  PreparedContactForce result;
  result.contact_ref = source.contact_ref;
  result.particle_ref = source.particle_ref;
  result.collider_ref = source.collider_ref;
  result.route_ref = source.route_ref;
  result.dedup_ref = source.contact_ref;
  result.source_ref = source.source_ref;
  result.policy_ref = source.policy_ref;
  result.policy_revision = source.policy_revision;
  result.standing = source.standing;
  result.event_ref = in.event_ref;
  result.subject_ref = in.subject_ref;
  result.body_preparation_ref = in.preparation_ref;
  result.body_state_ref = in.state_ref;
  result.source_coordinate = in.source_coordinate;
  result.source_revision = in.source_revision;
  result.source_generation = in.source_generation;
  result.eigenbasis_identity = prepared.eigenbasis_identity();
  result.body_revision = in.body_revision;
  result.sample_rate = in.sample_rate;
  result.pratibimba = in.pratibimba;
  result.position_metres = source.plane_position_metres;
  result.normal = source.normal;
  for (unsigned a = 0; a < 3; ++a)
    result.impact_velocity_metres_per_second[a] =
        -static_cast<double>(speed) * source.normal[a];
  result.impact_seconds = static_cast<double>(seconds);
  result.impact_speed_metres_per_second = static_cast<double>(speed);
  result.start_sample =
      source.start_sample + static_cast<std::uint64_t>(offset);
  result.frames = source.duration_samples;
  result.impulse_newton_seconds = impulse;
  std::fill_n(result.force_newtons.begin(), result.frames, force);
  return result;
}
// Pair this admission with the actual current M1/M2 determination on A's
// control owner. It does not reconstruct tuning or grant source authority.
inline bool
contact_matches_preparation(const PreparedContactForce &contact,
                            const PreparedPhysicalBody &prepared,
                            std::uint64_t native_admitted_cursor) noexcept {
  const auto &in = prepared.input();
  const auto valid_reference = [](const std::string &x) {
    if (x.empty() || x.size() > 2048)
      return false;
    for (unsigned char c : x)
      if (c < 32 || c == 127)
        return false;
    return true;
  };
  for (const auto *x :
       {&contact.contact_ref, &contact.particle_ref, &contact.collider_ref,
        &contact.route_ref, &contact.source_ref, &contact.policy_ref,
        &contact.policy_revision, &contact.standing})
    if (!valid_reference(*x))
      return false;
  if (contact.contract != physical_contact_contract ||
      contact.event_ref != in.event_ref ||
      contact.subject_ref != in.subject_ref ||
      contact.body_preparation_ref != in.preparation_ref ||
      contact.body_state_ref != in.state_ref ||
      contact.source_coordinate != in.source_coordinate ||
      contact.source_revision != in.source_revision ||
      contact.source_generation != in.source_generation ||
      contact.pratibimba != in.pratibimba ||
      contact.body_revision != in.body_revision ||
      contact.sample_rate != in.sample_rate ||
      contact.eigenbasis_identity != prepared.eigenbasis_identity() ||
      contact.start_sample < native_admitted_cursor || contact.frames == 0 ||
      contact.frames > physical_max_frames ||
      contact.start_sample >
          std::numeric_limits<std::uint64_t>::max() - contact.frames ||
      contact.dedup_ref != contact.contact_ref ||
      (contact.standing != "architecture-model" &&
       contact.standing != "reference" &&
       contact.standing != "tunable-model") ||
      contact.seed_standing != "deterministic-no-randomness" ||
      contact.seed != 0)
    return false;
  double norm = 0, speed = 0;
  for (unsigned a = 0; a < 3; ++a) {
    if (!std::isfinite(contact.position_metres[a]) ||
        std::abs(contact.position_metres[a]) > 1e6 ||
        !std::isfinite(contact.normal[a]) || std::abs(contact.normal[a]) > 1 ||
        !std::isfinite(contact.impact_velocity_metres_per_second[a]))
      return false;
    norm += contact.normal[a] * contact.normal[a];
    speed += contact.impact_velocity_metres_per_second[a] *
             contact.impact_velocity_metres_per_second[a];
    if (std::abs(contact.impact_velocity_metres_per_second[a] +
                 contact.impact_speed_metres_per_second * contact.normal[a]) >
        1e-10)
      return false;
  }
  if (!std::isfinite(contact.impact_seconds) || contact.impact_seconds < 0 ||
      contact.impact_seconds > 86400 ||
      !std::isfinite(contact.impact_speed_metres_per_second) ||
      contact.impact_speed_metres_per_second < 0 ||
      std::abs(norm - 1) > 1e-10 || !std::isfinite(speed))
    return false;
  double integral = 0;
  for (std::size_t i = 0; i < contact.frames; ++i) {
    const double x = contact.force_newtons[i];
    if (!std::isfinite(x) || std::abs(x) > in.max_force_newtons)
      return false;
    integral += x / in.sample_rate;
  }
  return std::isfinite(integral) &&
         std::isfinite(contact.impulse_newton_seconds) &&
         std::abs(contact.impulse_newton_seconds) <=
             in.max_impulse_newton_seconds &&
         std::abs(integral - contact.impulse_newton_seconds) <=
             1e-12 * std::max(1.0, std::abs(contact.impulse_newton_seconds));
}
// Final admission is callback-owned, immediately before dated force delivery,
// or under stopped/acknowledged exclusive custody. This is not a control-thread
// convenience overload: it reads the sole resident body's current basis/time.
inline bool contact_matches_body(const PreparedContactForce &contact,
                                 const PhysicalBody &body) noexcept {
  return contact_matches_preparation(contact, body.preparation(),
                                     body.samples_elapsed());
}
} // namespace ql
#endif
