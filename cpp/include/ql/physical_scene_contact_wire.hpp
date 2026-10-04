#ifndef QL_PHYSICAL_SCENE_CONTACT_WIRE_HPP
#define QL_PHYSICAL_SCENE_CONTACT_WIRE_HPP
// Pure numerical transport/replay. Nothing in this header constructs a Scene
// owner, occurrence witness, queue admission or source permission.
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/physical_scene_contact.hpp>
namespace ql::performance::scene_contact_transport {
using J = json_object;
using Json = ql::physical_wire::Json;
namespace wire = checkpoint_transport;
namespace ct = contact_transport;
inline constexpr const char *definition_schema =
    "ql.authored-body-local-plane-contact/v1";
inline Json copy(J *original) {
  J *out = nullptr;
  const auto result = json_object_deep_copy(original, &out, nullptr);
  auto owned = ql::physical_wire::own(out);
  ql::require(result == 0 && owned, "native contact immutable copy failed");
  return owned;
}
inline ql::PhysicalSceneContactDefinition read_definition(J *value) {
  ql::physical_wire::keys(
      value,
      {"schema", "contact_ref", "particle_ref", "collider_ref", "policy_ref",
       "policy_revision", "standing", "particle_position_metres",
       "particle_velocity_metres_per_second", "plane_position_metres",
       "outward_normal", "gravity_metres_per_second_squared", "mass_kg",
       "restitution", "transfer_fraction",
       "minimum_impact_speed_metres_per_second", "duration_samples"});
  ql::require(packet::string(packet::field(value, "schema")) ==
                  definition_schema,
              "native authored contact definition schema differs");
  ql::PhysicalSceneContactDefinition out{};
  out.contact_ref =
      ql::physical_wire::text(packet::field(value, "contact_ref"));
  out.particle_ref =
      ql::physical_wire::text(packet::field(value, "particle_ref"));
  out.collider_ref =
      ql::physical_wire::text(packet::field(value, "collider_ref"));
  out.policy_ref = ql::physical_wire::text(packet::field(value, "policy_ref"));
  out.policy_revision =
      ql::physical_wire::text(packet::field(value, "policy_revision"));
  out.standing = ql::physical_wire::text(packet::field(value, "standing"));
  out.particle_position_metres =
      ql::physical_wire::vec(packet::field(value, "particle_position_metres"));
  out.particle_velocity_metres_per_second = ql::physical_wire::vec(
      packet::field(value, "particle_velocity_metres_per_second"));
  out.plane_position_metres =
      ql::physical_wire::vec(packet::field(value, "plane_position_metres"));
  out.outward_normal =
      ql::physical_wire::vec(packet::field(value, "outward_normal"));
  out.gravity_metres_per_second_squared = ql::physical_wire::vec(
      packet::field(value, "gravity_metres_per_second_squared"));
  out.mass_kg = ql::physical_wire::number(packet::field(value, "mass_kg"));
  out.restitution =
      ql::physical_wire::number(packet::field(value, "restitution"));
  out.transfer_fraction =
      ql::physical_wire::number(packet::field(value, "transfer_fraction"));
  out.minimum_impact_speed_metres_per_second = ql::physical_wire::number(
      packet::field(value, "minimum_impact_speed_metres_per_second"));
  const auto duration =
      packet::integer(packet::field(value, "duration_samples"));
  ql::require(duration > 0 && duration <= ql::physical_max_frames,
              "native contact duration exceeds the physical block bound");
  out.duration_samples = std::uint32_t(duration);
  return out;
}
inline void constructor_transport_shape(J *constructor) {
  // This is ONLY bounded transport coherence. C's actual S registry Arc and
  // OS/image-qualified private channel establish authority upstream; JSON
  // facts, labels, hashes and this check cannot establish that authority.
  ql::physical_wire::keys(
      constructor, {"schema", "expression_ref", "scene_ref", "instance_ref",
                    "construction_generation", "generation_domain",
                    "initial_document_revision", "initial_document_sha256",
                    "document_revision", "document_sha256"});
  ql::require(
      packet::string(packet::field(constructor, "schema")) ==
              "oi.native-document-scene-constructor/v1" &&
          packet::string(packet::field(constructor, "generation_domain")) ==
              "native-document-scene-construction" &&
          packet::integer(
              packet::field(constructor, "construction_generation")) != 0,
      "private Scene constructor transport domain differs");
  for (const char *key : {"expression_ref", "scene_ref", "instance_ref",
                          "initial_document_sha256", "document_sha256"})
    ql::reference(packet::string(packet::field(constructor, key)));
  (void)packet::integer(
      packet::field(constructor, "initial_document_revision"));
  (void)packet::integer(packet::field(constructor, "document_revision"));
}
inline bool same_constructor_lifetime(J *before, J *current) {
  constructor_transport_shape(current);
  for (const char *key :
       {"schema", "expression_ref", "scene_ref", "instance_ref",
        "construction_generation", "generation_domain",
        "initial_document_revision", "initial_document_sha256"})
    if (!json_object_equal(packet::field(before, key),
                           packet::field(current, key)))
      return false;
  return true;
}

struct RetentionReservation {
  std::uint64_t source_record_bytes_limit = 0, native_pulse_bytes_limit = 0;
};
inline RetentionReservation read_retention_reservation(J *value) {
  ql::physical_wire::keys(value, {"schema", "source_record_bytes_limit",
                                  "native_pulse_bytes_limit"});
  ql::require(packet::string(packet::field(value, "schema")) ==
                  "ql.native-contact-retention-reservation/v1",
              "native contact retention reservation schema differs");
  RetentionReservation out{
      wire::decimal(packet::field(value, "source_record_bytes_limit")),
      wire::decimal(packet::field(value, "native_pulse_bytes_limit"))};
  ql::require(out.source_record_bytes_limit > 0 &&
                  out.native_pulse_bytes_limit > 0 &&
                  out.source_record_bytes_limit <= 4 * 1024 * 1024 &&
                  out.native_pulse_bytes_limit <= 4 * 1024 * 1024,
              "native contact reservation exceeds unchanged C part bound");
  return out;
}
inline std::size_t retained_json_upper_bound(J *value) {
  // C reserves the complete next asset (8MiB) and part (4MiB) FIRST. This
  // conservative transport bound accounts for re-escaping complete nested
  // JSON in that asset/part; it is a byte allowance, never source authority.
  // Six bytes per original serialized byte covers JSON unicode/control
  // escapes; the added 1024 covers decimal/float canonicalisation and wrapper
  // punctuation. Original wire is always retained, never reduced to a digest.
  const char *encoded =
      json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN);
  ql::require(encoded, "native contact complete wire encoding failed");
  const auto bytes = std::strlen(encoded);
  ql::require(bytes <= (std::numeric_limits<std::size_t>::max() - 1024) / 6,
              "native contact retained byte bound overflow");
  return 6 * bytes + 1024;
}

} // namespace ql::performance::scene_contact_transport
#endif
