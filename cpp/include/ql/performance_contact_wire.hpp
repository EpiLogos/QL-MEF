#ifndef QL_PERFORMANCE_CONTACT_WIRE_HPP
#define QL_PERFORMANCE_CONTACT_WIRE_HPP
#include <ql/performance_contact_slots.hpp>
#include <ql/performance_packet.hpp>
#include <ql/physical_checkpoint_wire.hpp>
namespace ql::performance::contact_transport {
using J = json_object;
using Json = ql::physical_wire::Json;
using packet::field;
using ql::physical_wire::keys;
inline Json object() {
  return ql::physical_wire::own(json_object_new_object());
}
inline Json array() { return ql::physical_wire::own(json_object_new_array()); }
inline void put(J *o, const char *k, J *v) {
  ql::physical_wire::checkpoint_put(o, k, v);
}
inline void append(J *o, J *v) {
  ql::require(v && json_object_array_add(o, v) == 0,
              "contact array insertion failed");
}
inline void text(J *o, const char *k, const std::string &s) {
  ql::require(s.size() <= 4096 && std::strlen(s.c_str()) == s.size(),
              "contact wire string invalid");
  put(o, k, json_object_new_string_len(s.data(), s.size()));
}
template <std::size_t N>
inline void ref(J *o, const char *k, const std::array<char, N> &r) {
  const auto end = std::find(r.begin(), r.end(), char(0));
  ql::require(end != r.end(), "contact bounded reference invalid");
  text(o, k, std::string(r.begin(), end));
}
inline std::string string(J *v) {
  ql::require(v && json_object_is_type(v, json_type_string),
              "contact string required");
  const auto n = json_object_get_string_len(v);
  const char *s = json_object_get_string(v);
  ql::require(n >= 0 && n <= 4096 && std::strlen(s) == std::size_t(n),
              "contact original reference invalid");
  return std::string(s, n);
}
template <std::size_t N> inline std::array<char, N> read_ref(J *v) {
  const auto s = string(v);
  ql::require(s.size() < N, "contact reference exceeds fixed bound");
  std::array<char, N> out{};
  std::copy(s.begin(), s.end(), out.begin());
  return out;
}
inline void u64(J *o, const char *k, std::uint64_t v) {
  text(o, k, std::to_string(v));
}
inline void real(J *o, const char *k, double v) {
  ql::require(std::isfinite(v), "finite contact number required");
  put(o, k, json_object_new_double(v));
}
template <class T> inline T bounded_integer(J *v) {
  const auto n = packet::integer(v);
  ql::require(n <= std::numeric_limits<T>::max(),
              "contact integer exceeds native bound");
  return T(n);
}
inline void flag(J *o, const char *k, bool v) {
  put(o, k, json_object_new_boolean(v));
}
inline Json vector(const ql::Vec3 &v) {
  auto o = array();
  for (double x : v) {
    ql::require(std::isfinite(x), "finite contact vector required");
    append(o.get(), json_object_new_double(x));
  }
  return o;
}
inline Json handle(const NativeContactHandle &v) {
  auto o = object();
  u64(o.get(), "generation", v.generation);
  u64(o.get(), "slot", v.slot);
  return o;
}
inline NativeContactHandle read_handle(J *o) {
  keys(o, {"generation", "slot"});
  return {packet::integer(field(o, "generation")),
          packet::byte(field(o, "slot"))};
}
inline Json occurrence(const NativeContactOccurrence &v) {
  auto o = object();
  ref(o.get(), "constructor_lineage", v.constructor_lineage);
  u64(o.get(), "original_request_id", v.original_request_id);
  return o;
}
inline NativeContactOccurrence read_occurrence(J *o) {
  keys(o, {"constructor_lineage", "original_request_id"});
  NativeContactOccurrence v{};
  v.constructor_lineage =
      read_ref<std::tuple_size_v<decltype(v.constructor_lineage)>>(
          field(o, "constructor_lineage"));
  v.original_request_id = bounded_integer<decltype(v.original_request_id)>(
      field(o, "original_request_id"));
  return v;
}
inline Json source(const NativeContactSourceIdentity &v) {
  auto o = object();
  ref(o.get(), "instance", v.instance);
  ref(o.get(), "event", v.event);
  ref(o.get(), "subject", v.subject);
  u64(o.get(), "m1_revision", v.m1_revision);
  u64(o.get(), "m2_generation", v.m2_generation);
  return o;
}
inline NativeContactSourceIdentity read_source(J *o) {
  keys(o, {"instance", "event", "subject", "m1_revision", "m2_generation"});
  NativeContactSourceIdentity v{};
  v.instance =
      read_ref<std::tuple_size_v<decltype(v.instance)>>(field(o, "instance"));
  v.event = read_ref<std::tuple_size_v<decltype(v.event)>>(field(o, "event"));
  v.subject =
      read_ref<std::tuple_size_v<decltype(v.subject)>>(field(o, "subject"));
  v.m1_revision =
      bounded_integer<decltype(v.m1_revision)>(field(o, "m1_revision"));
  v.m2_generation =
      bounded_integer<decltype(v.m2_generation)>(field(o, "m2_generation"));
  return v;
}
inline Json operands(const NativeContactOperands &v) {
  auto o = object();
  ref(o.get(), "contact_ref", v.contact_ref);
  ref(o.get(), "particle_ref", v.particle_ref);
  ref(o.get(), "collider_ref", v.collider_ref);
  ref(o.get(), "route_ref", v.route_ref);
  ref(o.get(), "source_ref", v.source_ref);
  ref(o.get(), "policy_ref", v.policy_ref);
  ref(o.get(), "policy_revision", v.policy_revision);
  ref(o.get(), "standing", v.standing);
  ref(o.get(), "seed_standing", v.seed_standing);
  ref(o.get(), "preparation_ref", v.preparation_ref);
  ref(o.get(), "state_ref", v.state_ref);
  ref(o.get(), "eigenbasis", v.eigenbasis);
  ref(o.get(), "source_coordinate", v.source_coordinate);
  ref(o.get(), "source_revision", v.source_revision);
  put(o.get(), "plane_position_metres",
      vector(v.plane_position_metres).release());
  put(o.get(), "normal", vector(v.normal).release());
  put(o.get(), "impact_velocity_metres_per_second",
      vector(v.impact_velocity_metres_per_second).release());
  real(o.get(), "height_metres", v.height_metres);
  real(o.get(), "initial_normal_velocity_metres_per_second",
       v.initial_normal_velocity_metres_per_second);
  real(o.get(), "gravity_metres_per_second_squared",
       v.gravity_metres_per_second_squared);
  real(o.get(), "mass_kg", v.mass_kg);
  real(o.get(), "restitution", v.restitution);
  real(o.get(), "transfer_fraction", v.transfer_fraction);
  real(o.get(), "minimum_impact_speed_metres_per_second",
       v.minimum_impact_speed_metres_per_second);
  real(o.get(), "impact_seconds", v.impact_seconds);
  real(o.get(), "impact_speed_metres_per_second",
       v.impact_speed_metres_per_second);
  real(o.get(), "planned_impulse_newton_seconds",
       v.planned_impulse_newton_seconds);
  real(o.get(), "force_newtons", v.force_newtons);
  u64(o.get(), "trigger_sample", v.trigger_sample);
  u64(o.get(), "impact_sample", v.impact_sample);
  u64(o.get(), "body_revision", v.body_revision);
  u64(o.get(), "source_generation", v.source_generation);
  u64(o.get(), "seed", v.seed);
  u64(o.get(), "sample_rate", v.sample_rate);
  u64(o.get(), "duration_samples", v.duration_samples);
  flag(o.get(), "pratibimba", v.pratibimba);
  return o;
}
inline NativeContactOperands read_operands(J *o) {
  keys(o, {"contact_ref",
           "particle_ref",
           "collider_ref",
           "route_ref",
           "source_ref",
           "policy_ref",
           "policy_revision",
           "standing",
           "seed_standing",
           "preparation_ref",
           "state_ref",
           "eigenbasis",
           "source_coordinate",
           "source_revision",
           "plane_position_metres",
           "normal",
           "impact_velocity_metres_per_second",
           "height_metres",
           "initial_normal_velocity_metres_per_second",
           "gravity_metres_per_second_squared",
           "mass_kg",
           "restitution",
           "transfer_fraction",
           "minimum_impact_speed_metres_per_second",
           "impact_seconds",
           "impact_speed_metres_per_second",
           "planned_impulse_newton_seconds",
           "force_newtons",
           "trigger_sample",
           "impact_sample",
           "body_revision",
           "source_generation",
           "seed",
           "sample_rate",
           "duration_samples",
           "pratibimba"});
  NativeContactOperands v{};
  v.contact_ref = read_ref<std::tuple_size_v<decltype(v.contact_ref)>>(
      field(o, "contact_ref"));
  v.particle_ref = read_ref<std::tuple_size_v<decltype(v.particle_ref)>>(
      field(o, "particle_ref"));
  v.collider_ref = read_ref<std::tuple_size_v<decltype(v.collider_ref)>>(
      field(o, "collider_ref"));
  v.route_ref =
      read_ref<std::tuple_size_v<decltype(v.route_ref)>>(field(o, "route_ref"));
  v.source_ref = read_ref<std::tuple_size_v<decltype(v.source_ref)>>(
      field(o, "source_ref"));
  v.policy_ref = read_ref<std::tuple_size_v<decltype(v.policy_ref)>>(
      field(o, "policy_ref"));
  v.policy_revision = read_ref<std::tuple_size_v<decltype(v.policy_revision)>>(
      field(o, "policy_revision"));
  v.standing =
      read_ref<std::tuple_size_v<decltype(v.standing)>>(field(o, "standing"));
  v.seed_standing = read_ref<std::tuple_size_v<decltype(v.seed_standing)>>(
      field(o, "seed_standing"));
  v.preparation_ref = read_ref<std::tuple_size_v<decltype(v.preparation_ref)>>(
      field(o, "preparation_ref"));
  v.state_ref =
      read_ref<std::tuple_size_v<decltype(v.state_ref)>>(field(o, "state_ref"));
  v.eigenbasis = read_ref<std::tuple_size_v<decltype(v.eigenbasis)>>(
      field(o, "eigenbasis"));
  v.source_coordinate =
      read_ref<std::tuple_size_v<decltype(v.source_coordinate)>>(
          field(o, "source_coordinate"));
  v.source_revision = read_ref<std::tuple_size_v<decltype(v.source_revision)>>(
      field(o, "source_revision"));
  v.plane_position_metres = packet::vector(field(o, "plane_position_metres"));
  v.normal = packet::vector(field(o, "normal"));
  v.impact_velocity_metres_per_second =
      packet::vector(field(o, "impact_velocity_metres_per_second"));
  v.height_metres = packet::number(field(o, "height_metres"));
  v.initial_normal_velocity_metres_per_second =
      packet::number(field(o, "initial_normal_velocity_metres_per_second"));
  v.gravity_metres_per_second_squared =
      packet::number(field(o, "gravity_metres_per_second_squared"));
  v.mass_kg = packet::number(field(o, "mass_kg"));
  v.restitution = packet::number(field(o, "restitution"));
  v.transfer_fraction = packet::number(field(o, "transfer_fraction"));
  v.minimum_impact_speed_metres_per_second =
      packet::number(field(o, "minimum_impact_speed_metres_per_second"));
  v.impact_seconds = packet::number(field(o, "impact_seconds"));
  v.impact_speed_metres_per_second =
      packet::number(field(o, "impact_speed_metres_per_second"));
  v.planned_impulse_newton_seconds =
      packet::number(field(o, "planned_impulse_newton_seconds"));
  v.force_newtons = packet::number(field(o, "force_newtons"));
  v.trigger_sample =
      bounded_integer<decltype(v.trigger_sample)>(field(o, "trigger_sample"));
  v.impact_sample =
      bounded_integer<decltype(v.impact_sample)>(field(o, "impact_sample"));
  v.body_revision =
      bounded_integer<decltype(v.body_revision)>(field(o, "body_revision"));
  v.source_generation = bounded_integer<decltype(v.source_generation)>(
      field(o, "source_generation"));
  v.seed = bounded_integer<decltype(v.seed)>(field(o, "seed"));
  v.sample_rate =
      bounded_integer<decltype(v.sample_rate)>(field(o, "sample_rate"));
  v.duration_samples = bounded_integer<decltype(v.duration_samples)>(
      field(o, "duration_samples"));
  v.pratibimba = packet::boolean(field(o, "pratibimba"));
  return v;
}
inline Json original(const ql::GravityContactInput &v) {
  auto o = object();
  text(o.get(), "contact_ref", v.contact_ref);
  text(o.get(), "particle_ref", v.particle_ref);
  text(o.get(), "collider_ref", v.collider_ref);
  text(o.get(), "route_ref", v.route_ref);
  text(o.get(), "source_ref", v.source_ref);
  text(o.get(), "policy_ref", v.policy_ref);
  text(o.get(), "policy_revision", v.policy_revision);
  text(o.get(), "standing", v.standing);
  put(o.get(), "plane_position_metres",
      vector(v.plane_position_metres).release());
  put(o.get(), "normal", vector(v.normal).release());
  real(o.get(), "height_metres", v.height_metres);
  real(o.get(), "initial_normal_velocity_metres_per_second",
       v.initial_normal_velocity_metres_per_second);
  real(o.get(), "gravity_metres_per_second_squared",
       v.gravity_metres_per_second_squared);
  real(o.get(), "mass_kg", v.mass_kg);
  real(o.get(), "restitution", v.restitution);
  real(o.get(), "transfer_fraction", v.transfer_fraction);
  real(o.get(), "minimum_impact_speed_metres_per_second",
       v.minimum_impact_speed_metres_per_second);
  u64(o.get(), "start_sample", v.start_sample);
  u64(o.get(), "duration_samples", v.duration_samples);
  return o;
}
inline ql::GravityContactInput read_original(J *o) {
  keys(o, {"contact_ref", "particle_ref", "collider_ref", "route_ref",
           "source_ref", "policy_ref", "policy_revision", "standing",
           "plane_position_metres", "normal", "height_metres",
           "initial_normal_velocity_metres_per_second",
           "gravity_metres_per_second_squared", "mass_kg", "restitution",
           "transfer_fraction", "minimum_impact_speed_metres_per_second",
           "start_sample", "duration_samples"});
  ql::GravityContactInput v{};
  v.contact_ref = string(field(o, "contact_ref"));
  v.particle_ref = string(field(o, "particle_ref"));
  v.collider_ref = string(field(o, "collider_ref"));
  v.route_ref = string(field(o, "route_ref"));
  v.source_ref = string(field(o, "source_ref"));
  v.policy_ref = string(field(o, "policy_ref"));
  v.policy_revision = string(field(o, "policy_revision"));
  v.standing = string(field(o, "standing"));
  v.plane_position_metres = packet::vector(field(o, "plane_position_metres"));
  v.normal = packet::vector(field(o, "normal"));
  v.height_metres = packet::number(field(o, "height_metres"));
  v.initial_normal_velocity_metres_per_second =
      packet::number(field(o, "initial_normal_velocity_metres_per_second"));
  v.gravity_metres_per_second_squared =
      packet::number(field(o, "gravity_metres_per_second_squared"));
  v.mass_kg = packet::number(field(o, "mass_kg"));
  v.restitution = packet::number(field(o, "restitution"));
  v.transfer_fraction = packet::number(field(o, "transfer_fraction"));
  v.minimum_impact_speed_metres_per_second =
      packet::number(field(o, "minimum_impact_speed_metres_per_second"));
  v.start_sample =
      bounded_integer<decltype(v.start_sample)>(field(o, "start_sample"));
  v.duration_samples = bounded_integer<decltype(v.duration_samples)>(
      field(o, "duration_samples"));
  return v;
}
inline Json material(const ql::PhysicalMaterial &v) {
  auto o = object();
  text(o.get(), "reference", v.reference);
  text(o.get(), "revision", v.revision);
  text(o.get(), "source_ref", v.source_ref);
  text(o.get(), "standing", v.standing);
  real(o.get(), "young_modulus_pa", v.young_modulus_pa);
  real(o.get(), "density_kg_per_m3", v.density_kg_per_m3);
  real(o.get(), "damping_alpha_per_second", v.damping_alpha_per_second);
  real(o.get(), "damping_beta_seconds", v.damping_beta_seconds);
  return o;
}
inline ql::PhysicalMaterial read_material(J *o) {
  keys(o, {"reference", "revision", "source_ref", "standing",
           "young_modulus_pa", "density_kg_per_m3", "damping_alpha_per_second",
           "damping_beta_seconds"});
  ql::PhysicalMaterial v{};
  v.reference = string(field(o, "reference"));
  v.revision = string(field(o, "revision"));
  v.source_ref = string(field(o, "source_ref"));
  v.standing = string(field(o, "standing"));
  v.young_modulus_pa = packet::number(field(o, "young_modulus_pa"));
  v.density_kg_per_m3 = packet::number(field(o, "density_kg_per_m3"));
  v.damping_alpha_per_second =
      packet::number(field(o, "damping_alpha_per_second"));
  v.damping_beta_seconds = packet::number(field(o, "damping_beta_seconds"));
  return v;
}
inline Json projection(const ql::PhysicalProjection &v) {
  auto o = object();
  put(o.get(), "axis", vector(v.axis).release());
  auto w = array();
  for (double x : v.node_weights) {
    ql::require(std::isfinite(x), "finite projection required");
    append(w.get(), json_object_new_double(x));
  }
  put(o.get(), "node_weights", w.release());
  return o;
}
inline ql::PhysicalProjection read_projection(J *o, std::size_t nodes) {
  keys(o, {"axis", "node_weights"});
  ql::PhysicalProjection v;
  v.axis = packet::vector(field(o, "axis"));
  auto w = field(o, "node_weights");
  packet::array(w, nodes);
  for (std::size_t i = 0; i < nodes; ++i)
    v.node_weights.push_back(packet::number(json_object_array_get_idx(w, i)));
  return v;
}
inline Json body_input(const ql::PhysicalBodyInput &v) {
  auto o = object();
  text(o.get(), "event_ref", v.event_ref);
  text(o.get(), "subject_ref", v.subject_ref);
  text(o.get(), "source_coordinate", v.source_coordinate);
  text(o.get(), "source_revision", v.source_revision);
  text(o.get(), "geometry_ref", v.geometry_ref);
  text(o.get(), "geometry_revision", v.geometry_revision);
  text(o.get(), "geometry_source_ref", v.geometry_source_ref);
  text(o.get(), "geometry_standing", v.geometry_standing);
  text(o.get(), "preparation_ref", v.preparation_ref);
  text(o.get(), "state_ref", v.state_ref);
  u64(o.get(), "source_generation", v.source_generation);
  u64(o.get(), "body_revision", v.body_revision);
  u64(o.get(), "sample_rate", v.sample_rate);
  real(o.get(), "pickup_linear_per_metre", v.pickup_linear_per_metre);
  real(o.get(), "max_force_newtons", v.max_force_newtons);
  real(o.get(), "max_impulse_newton_seconds", v.max_impulse_newton_seconds);
  real(o.get(), "max_displacement_metres", v.max_displacement_metres);
  flag(o.get(), "pratibimba", v.pratibimba);
  u64(o.get(), "family", unsigned(v.family));
  put(o.get(), "material", material(v.material).release());
  put(o.get(), "exciter", projection(v.exciter).release());
  put(o.get(), "pickup", projection(v.pickup).release());
  auto nodes = array();
  for (const auto &n : v.nodes) {
    auto o = object();
    u64(o.get(), "identity", n.identity);
    text(o.get(), "constituent", n.constituent);
    put(o.get(), "rest_metres", vector(n.rest_metres).release());
    real(o.get(), "additional_mass_kg", n.additional_mass_kg);
    auto f = array();
    for (bool b : n.fixed)
      append(f.get(), json_object_new_boolean(b));
    put(o.get(), "fixed", f.release());
    append(nodes.get(), o.release());
  }
  put(o.get(), "nodes", nodes.release());
  auto edges = array();
  for (const auto &e : v.edges) {
    auto o = object();
    u64(o.get(), "first", e.first);
    u64(o.get(), "second", e.second);
    real(o.get(), "section_m2", e.section_m2);
    real(o.get(), "prestress_newtons", e.prestress_newtons);
    append(edges.get(), o.release());
  }
  put(o.get(), "edges", edges.release());
  return o;
}
inline ql::PhysicalBodyInput read_body_input(J *o) {
  keys(o, {"event_ref",
           "subject_ref",
           "source_coordinate",
           "source_revision",
           "geometry_ref",
           "geometry_revision",
           "geometry_source_ref",
           "geometry_standing",
           "preparation_ref",
           "state_ref",
           "source_generation",
           "body_revision",
           "sample_rate",
           "pickup_linear_per_metre",
           "max_force_newtons",
           "max_impulse_newton_seconds",
           "max_displacement_metres",
           "pratibimba",
           "family",
           "material",
           "exciter",
           "pickup",
           "nodes",
           "edges"});
  ql::PhysicalBodyInput v{};
  v.event_ref = string(field(o, "event_ref"));
  v.subject_ref = string(field(o, "subject_ref"));
  v.source_coordinate = string(field(o, "source_coordinate"));
  v.source_revision = string(field(o, "source_revision"));
  v.geometry_ref = string(field(o, "geometry_ref"));
  v.geometry_revision = string(field(o, "geometry_revision"));
  v.geometry_source_ref = string(field(o, "geometry_source_ref"));
  v.geometry_standing = string(field(o, "geometry_standing"));
  v.preparation_ref = string(field(o, "preparation_ref"));
  v.state_ref = string(field(o, "state_ref"));
  v.source_generation = bounded_integer<decltype(v.source_generation)>(
      field(o, "source_generation"));
  v.body_revision =
      bounded_integer<decltype(v.body_revision)>(field(o, "body_revision"));
  v.sample_rate =
      bounded_integer<decltype(v.sample_rate)>(field(o, "sample_rate"));
  v.pickup_linear_per_metre =
      packet::number(field(o, "pickup_linear_per_metre"));
  v.max_force_newtons = packet::number(field(o, "max_force_newtons"));
  v.max_impulse_newton_seconds =
      packet::number(field(o, "max_impulse_newton_seconds"));
  v.max_displacement_metres =
      packet::number(field(o, "max_displacement_metres"));
  v.pratibimba = packet::boolean(field(o, "pratibimba"));
  const auto family = packet::integer(field(o, "family"));
  ql::require(family <= unsigned(ql::BodyFamily::PrestressedTensionNetwork),
              "contact body family differs");
  v.family = ql::BodyFamily(family);
  v.material = read_material(field(o, "material"));
  auto nodes = field(o, "nodes");
  ql::require(json_object_is_type(nodes, json_type_array) &&
                  json_object_array_length(nodes) <= ql::physical_max_nodes,
              "contact body node budget exceeded");
  for (std::size_t i = 0; i < json_object_array_length(nodes); ++i) {
    auto o = json_object_array_get_idx(nodes, i);
    keys(o, {"identity", "constituent", "rest_metres", "additional_mass_kg",
             "fixed"});
    ql::PhysicalNode n{};
    n.identity = packet::integer(field(o, "identity"));
    n.constituent = string(field(o, "constituent"));
    n.rest_metres = packet::vector(field(o, "rest_metres"));
    n.additional_mass_kg = packet::number(field(o, "additional_mass_kg"));
    auto f = field(o, "fixed");
    packet::array(f, 3);
    for (std::size_t j = 0; j < 3; ++j)
      n.fixed[j] = packet::boolean(json_object_array_get_idx(f, j));
    v.nodes.push_back(n);
  }
  auto edges = field(o, "edges");
  ql::require(json_object_is_type(edges, json_type_array) &&
                  json_object_array_length(edges) <= ql::physical_max_edges,
              "contact body edge budget exceeded");
  for (std::size_t i = 0; i < json_object_array_length(edges); ++i) {
    auto o = json_object_array_get_idx(edges, i);
    keys(o, {"first", "second", "section_m2", "prestress_newtons"});
    v.edges.push_back({packet::integer(field(o, "first")),
                       packet::integer(field(o, "second")),
                       packet::number(field(o, "section_m2")),
                       packet::number(field(o, "prestress_newtons"))});
  }
  v.exciter = read_projection(field(o, "exciter"), v.nodes.size());
  v.pickup = read_projection(field(o, "pickup"), v.nodes.size());
  return v;
}
inline Json delivery(const NativeContactDelivery &v) {
  auto o = object();
  put(o.get(), "handle", handle(v.handle).release());
  u64(o.get(), "status", unsigned(v.status));
  u64(o.get(), "refusal", unsigned(v.refusal));
  u64(o.get(), "admission_sequence", v.admission_sequence);
  u64(o.get(), "start_application_ordinal", v.start_application_ordinal);
  u64(o.get(), "committed_cursor", v.committed_cursor);
  u64(o.get(), "requested_impact_sample", v.requested_impact_sample);
  u64(o.get(), "admitted_impact_sample", v.admitted_impact_sample);
  u64(o.get(), "actual_impact_sample", v.actual_impact_sample);
  u64(o.get(), "delivered_frames", v.delivered_frames);
  u64(o.get(), "planned_frames", v.planned_frames);
  real(o.get(), "delivered_impulse_newton_seconds",
       v.delivered_impulse_newton_seconds);
  real(o.get(), "planned_impulse_newton_seconds",
       v.planned_impulse_newton_seconds);
  return o;
}
inline NativeContactDelivery read_delivery(J *o) {
  keys(o,
       {"handle", "status", "refusal", "admission_sequence",
        "start_application_ordinal", "committed_cursor",
        "requested_impact_sample", "admitted_impact_sample",
        "actual_impact_sample", "delivered_frames", "planned_frames",
        "delivered_impulse_newton_seconds", "planned_impulse_newton_seconds"});
  NativeContactDelivery v{};
  v.handle = read_handle(field(o, "handle"));
  const auto status = packet::integer(field(o, "status")),
             refusal = packet::integer(field(o, "refusal"));
  ql::require(status <= unsigned(NativeContactStatus::Interrupted) &&
                  refusal <= unsigned(NativeContactRefusal::ForceBudgetChanged),
              "contact progress enum differs");
  v.status = NativeContactStatus(status);
  v.refusal = NativeContactRefusal(refusal);
  v.admission_sequence = bounded_integer<decltype(v.admission_sequence)>(
      field(o, "admission_sequence"));
  v.start_application_ordinal =
      bounded_integer<decltype(v.start_application_ordinal)>(
          field(o, "start_application_ordinal"));
  v.committed_cursor = bounded_integer<decltype(v.committed_cursor)>(
      field(o, "committed_cursor"));
  v.requested_impact_sample =
      bounded_integer<decltype(v.requested_impact_sample)>(
          field(o, "requested_impact_sample"));
  v.admitted_impact_sample =
      bounded_integer<decltype(v.admitted_impact_sample)>(
          field(o, "admitted_impact_sample"));
  v.actual_impact_sample = bounded_integer<decltype(v.actual_impact_sample)>(
      field(o, "actual_impact_sample"));
  v.delivered_frames = bounded_integer<decltype(v.delivered_frames)>(
      field(o, "delivered_frames"));
  v.planned_frames =
      bounded_integer<decltype(v.planned_frames)>(field(o, "planned_frames"));
  v.delivered_impulse_newton_seconds =
      packet::number(field(o, "delivered_impulse_newton_seconds"));
  v.planned_impulse_newton_seconds =
      packet::number(field(o, "planned_impulse_newton_seconds"));
  return v;
}
inline Json checkpoint(const NativeContactCheckpoint &cp) {
  auto o = object();
  text(o.get(), "schema", NativeContactCheckpoint::schema);
  flag(o.get(), "history_present", cp.history_present);
  ref(o.get(), "constructor_lineage", cp.constructor_lineage);
  u64(o.get(), "original_request_high_water", cp.original_request_high_water);
  auto g = array();
  for (auto n : cp.slot_generations)
    append(g.get(), json_object_new_string(std::to_string(n).c_str()));
  put(o.get(), "slot_generations", g.release());
  auto slots = array();
  for (const auto &v : cp.slots) {
    if (!v.present)
      continue;
    auto o = object();
    put(o.get(), "handle", handle(v.handle).release());
    put(o.get(), "occurrence", occurrence(v.occurrence).release());
    put(o.get(), "source", source(v.source).release());
    u64(o.get(), "admission_sequence", v.admission_sequence);
    put(o.get(), "original", original(v.original).release());
    put(o.get(), "original_body", body_input(v.original_body).release());
    put(o.get(), "operands", operands(v.operands).release());
    put(o.get(), "delivery", delivery(v.delivery).release());
    auto f = array();
    for (double n : v.force_newtons) {
      ql::require(std::isfinite(n), "contact retained force invalid");
      append(f.get(), json_object_new_double(n));
    }
    put(o.get(), "force_newtons", f.release());
    append(slots.get(), o.release());
  }
  put(o.get(), "slots", slots.release());
  return o;
}
inline NativeContactCheckpoint read_checkpoint(J *o) {
  keys(o, {"schema", "history_present", "constructor_lineage",
           "original_request_high_water", "slot_generations", "slots"});
  ql::require(string(field(o, "schema")) == NativeContactCheckpoint::schema,
              "contact checkpoint schema differs");
  NativeContactCheckpoint cp;
  cp.history_present = packet::boolean(field(o, "history_present"));
  cp.constructor_lineage = read_ref<256>(field(o, "constructor_lineage"));
  cp.original_request_high_water =
      packet::integer(field(o, "original_request_high_water"));
  auto g = field(o, "slot_generations");
  packet::array(g, contact_slot_capacity);
  for (std::size_t i = 0; i < contact_slot_capacity; ++i)
    cp.slot_generations[i] = packet::integer(json_object_array_get_idx(g, i));
  auto slots = field(o, "slots");
  ql::require(json_object_is_type(slots, json_type_array) &&
                  json_object_array_length(slots) <= contact_slot_capacity,
              "contact checkpoint slot budget exceeded");
  for (std::size_t i = 0; i < json_object_array_length(slots); ++i) {
    auto o = json_object_array_get_idx(slots, i);
    keys(o, {"handle", "occurrence", "source", "admission_sequence", "original",
             "original_body", "operands", "delivery", "force_newtons"});
    const auto h = read_handle(field(o, "handle"));
    ql::require(h.valid() && !cp.slots[h.slot].present,
                "contact checkpoint duplicate/invalid slot");
    auto &v = cp.slots[h.slot];
    v.present = true;
    v.handle = h;
    v.occurrence = read_occurrence(field(o, "occurrence"));
    v.source = read_source(field(o, "source"));
    v.admission_sequence = bounded_integer<decltype(v.admission_sequence)>(
        field(o, "admission_sequence"));
    v.original = read_original(field(o, "original"));
    v.original_body = read_body_input(field(o, "original_body"));
    v.operands = read_operands(field(o, "operands"));
    v.delivery = read_delivery(field(o, "delivery"));
    auto f = field(o, "force_newtons");
    packet::array(f, ql::physical_max_frames);
    for (std::size_t j = 0; j < ql::physical_max_frames; ++j)
      v.force_newtons[j] = packet::number(json_object_array_get_idx(f, j));
  }
  return cp;
}
} // namespace ql::performance::contact_transport
#endif
