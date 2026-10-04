#ifndef QL_PHYSICAL_SCENE_CONTACT_HPP
#define QL_PHYSICAL_SCENE_CONTACT_HPP
// Authored geometry is numerical input, never Scene/source permission. The
// retained native Scene channel qualifies its origin separately. This solver
// derives the particle-plane collision against the ACTUAL immutable body
// exciter; it does not accept a caller-computed height, force or impact date.
#include <ql/physical_contact.hpp>

namespace ql {
struct PhysicalSceneContactDefinition {
  std::string contact_ref, particle_ref, collider_ref, policy_ref,
      policy_revision, standing;
  // All geometry is in the current prepared body's local metre frame. World
  // propagation and receiving remain the same retained M4 owner's operation.
  Vec3 particle_position_metres{}, particle_velocity_metres_per_second{},
      plane_position_metres{}, outward_normal{},
      gravity_metres_per_second_squared{};
  double mass_kg = 0, restitution = 0, transfer_fraction = 1,
         minimum_impact_speed_metres_per_second = 0;
  std::uint32_t duration_samples = 1;
};
struct PreparedPhysicalSceneContact {
  PhysicalSceneContactDefinition original;
  GravityContactInput native_input;
  Vec3 exciter_position_metres{};
  PreparedContactForce force;
};
inline double contact_dot(const Vec3 &a, const Vec3 &b) noexcept {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}
inline PreparedPhysicalSceneContact prepare_physical_scene_contact(
    const PreparedPhysicalBody &body, std::uint64_t actual_native_cursor,
    const PhysicalSceneContactDefinition &definition) {
  for (const auto *ref : {&definition.contact_ref, &definition.particle_ref,
                          &definition.collider_ref, &definition.policy_ref,
                          &definition.policy_revision, &definition.standing})
    reference(*ref);
  const auto &in = body.input();
  require(in.exciter.node_weights.size() == in.nodes.size(),
          "native contact exciter does not span the actual physical body");
  bool conducts_into_physical_mode = false;
  for (std::size_t mode = 0; mode < body.mode_count(); ++mode) {
    const auto &shape = body.mode_shape(mode);
    double projection = 0;
    for (std::size_t node = 0; node < in.nodes.size(); ++node)
      for (unsigned axis = 0; axis < 3; ++axis)
        projection += shape[node][axis] * in.exciter.axis[axis] *
                      in.exciter.node_weights[node];
    conducts_into_physical_mode |= std::isfinite(projection) && projection != 0;
  }
  require(
      conducts_into_physical_mode,
      "contact body exciter is disconnected from every actual physical mode");
  PreparedPhysicalSceneContact out{};
  out.original = definition;
  for (std::size_t node = 0; node < in.nodes.size(); ++node)
    for (unsigned axis = 0; axis < 3; ++axis)
      out.exciter_position_metres[axis] +=
          in.exciter.node_weights[node] * in.nodes[node].rest_metres[axis];
  Vec3 plane_to_exciter{}, exciter_to_particle{};
  for (unsigned axis = 0; axis < 3; ++axis) {
    const auto finite = [](double x, double bound) {
      return std::isfinite(x) && std::abs(x) <= bound;
    };
    require(
        finite(definition.particle_position_metres[axis], 1e6) &&
            finite(definition.plane_position_metres[axis], 1e6) &&
            finite(definition.particle_velocity_metres_per_second[axis], 1e4) &&
            finite(definition.gravity_metres_per_second_squared[axis], 1e4) &&
            finite(definition.outward_normal[axis], 1),
        "native Scene contact geometry is nonfinite or out of range");
    plane_to_exciter[axis] = out.exciter_position_metres[axis] -
                             definition.plane_position_metres[axis];
    exciter_to_particle[axis] = definition.particle_position_metres[axis] -
                                out.exciter_position_metres[axis];
  }
  const auto &normal = definition.outward_normal;
  require(std::abs(contact_dot(normal, normal) - 1) <= 1e-10,
          "native Scene contact normal is not unit length");
  // The current body has ONE declared distributed exciter. A different
  // contact point must not pretend to create a spatial actuator. The plane
  // contains that exciter, and this one-dimensional particle hits its anchor.
  require(std::abs(contact_dot(plane_to_exciter, normal)) <= 1e-10,
          "contact collider is detached from the current native body exciter");
  const double height = contact_dot(exciter_to_particle, normal);
  const double velocity =
      contact_dot(definition.particle_velocity_metres_per_second, normal);
  const double gravity =
      -contact_dot(definition.gravity_metres_per_second_squared, normal);
  for (unsigned axis = 0; axis < 3; ++axis) {
    require(std::abs(exciter_to_particle[axis] - height * normal[axis]) <=
                    1e-10 &&
                std::abs(definition.particle_velocity_metres_per_second[axis] -
                         velocity * normal[axis]) <= 1e-10 &&
                std::abs(definition.gravity_metres_per_second_squared[axis] +
                         gravity * normal[axis]) <= 1e-10,
            "contact needs the declared one-dimensional native collision");
  }
  require(std::abs(contact_dot(normal, in.exciter.axis)) > 1e-10,
          "contact normal does not conduct force into the native body exciter");
  auto &input = out.native_input;
  input.contact_ref = definition.contact_ref;
  input.particle_ref = definition.particle_ref;
  input.collider_ref = definition.collider_ref;
  // These are actual native body provenance, not authored routing labels.
  input.route_ref = in.preparation_ref;
  input.source_ref = in.geometry_source_ref;
  input.policy_ref = definition.policy_ref;
  input.policy_revision = definition.policy_revision;
  input.standing = definition.standing;
  input.plane_position_metres = out.exciter_position_metres;
  input.normal = normal;
  input.height_metres = height;
  input.initial_normal_velocity_metres_per_second = velocity;
  input.gravity_metres_per_second_squared = gravity;
  input.mass_kg = definition.mass_kg;
  input.restitution = definition.restitution;
  input.transfer_fraction = definition.transfer_fraction;
  input.minimum_impact_speed_metres_per_second =
      definition.minimum_impact_speed_metres_per_second;
  input.start_sample = actual_native_cursor;
  input.duration_samples = definition.duration_samples;
  out.force = prepare_gravity_contact(body, actual_native_cursor, input);
  require(contact_matches_preparation(out.force, body, actual_native_cursor),
          "derived contact differs from the actual native body and clock");
  return out;
}
} // namespace ql
#endif
