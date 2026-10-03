#ifndef QL_PERFORMANCE_ACOUSTIC_WIRE_HPP
#define QL_PERFORMANCE_ACOUSTIC_WIRE_HPP
// Private Rust source owner emits candidate/current producer packets on the
// existing worker pipe after original C Act/lease qualification. Numerical
// equality/body compatibility here does not grant private authority.
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_physical_receiving.hpp>
namespace ql::performance::acoustic_wire {
enum class AcousticReadKind { FirstInstallation, RetainedReceiverSegment };
inline ql::PreparedMovingSpatialReceiving read_prepared_acoustic(
    json_object *candidate, json_object *actual_current,
    json_object *retained_source_body,
    const ql::PreparedPhysicalBody &immutable,
    std::uint64_t native_admitted_cursor,
    AcousticReadKind kind = AcousticReadKind::FirstInstallation) {
  namespace p = ql::physical_wire;
  namespace w = checkpoint_transport;
  p::keys(candidate, {"schema", "source_body", "configuration", "context",
                      "source_position_metres", "origin_sample", "end_sample",
                      "history_origin_sample", "units"});
  require(json_object_equal(candidate, actual_current),
          "complete current native acoustic producer differs");
  require(p::text(p::field(candidate, "schema")) ==
                  "ql.native-acoustic-receiving-preparation/v1" &&
              json_object_equal(p::field(candidate, "source_body"),
                                retained_source_body),
          "native acoustic body is not actual retained source preparation");
  auto units = p::field(candidate, "units");
  p::keys(units, {"distance", "velocity", "cursor", "signal"});
  require(p::text(p::field(units, "distance")) == "m" &&
              p::text(p::field(units, "velocity")) == "m/s" &&
              p::text(p::field(units, "cursor")) == "native-audio-sample" &&
              p::text(p::field(units, "signal")) == "linear-pickup",
          "native acoustic physical units differ");
  const auto origin = w::decimal(p::field(candidate, "origin_sample"));
  const auto end = w::decimal(p::field(candidate, "end_sample"));
  const auto birth = w::decimal(p::field(candidate, "history_origin_sample"));
  require(origin == native_admitted_cursor && birth <= origin &&
              (kind == AcousticReadKind::RetainedReceiverSegment ||
               birth == origin),
          "native acoustic segment/birth detached from actual admitted cursor");
  auto config = p::field(candidate, "configuration");
  p::keys(config,
          {"schema", "source_ref", "source_motion_ref", "receiver_motion_ref",
           "policy_ref", "policy_revision", "standing", "revision",
           "source_translation_metres", "receiver_position_metres",
           "receiver_forward", "source_velocity_metres_per_second",
           "receiver_velocity_metres_per_second", "speed_metres_per_second",
           "minimum_distance_metres", "directivity", "propagation_delay",
           "span_samples"});
  require(p::text(p::field(config, "schema")) ==
              "ql.native-acoustic-receiving-configuration/v1",
          "native acoustic configuration schema differs");
  auto context = p::field(candidate, "context");
  ql::SpatialReceivingInput spatial;
  spatial.receiver_ref =
      p::text(p::field(p::field(context, "receiver"), "reference"));
  spatial.context_ref =
      p::text(p::field(p::field(context, "context"), "reference"));
  spatial.source_ref = p::text(p::field(config, "source_ref"));
  spatial.policy_ref = p::text(p::field(config, "policy_ref"));
  spatial.policy_revision = p::text(p::field(config, "policy_revision"));
  spatial.standing = p::text(p::field(config, "standing"));
  spatial.revision = p::exact(p::field(config, "revision"));
  spatial.source_translation_metres =
      p::vec(p::field(config, "source_translation_metres"));
  spatial.receiver_position_metres =
      p::vec(p::field(config, "receiver_position_metres"));
  spatial.receiver_forward = p::vec(p::field(config, "receiver_forward"));
  spatial.speed_metres_per_second =
      p::number(p::field(config, "speed_metres_per_second"));
  spatial.minimum_distance_metres =
      p::number(p::field(config, "minimum_distance_metres"));
  const auto directivity = p::text(p::field(config, "directivity"));
  require(directivity == "omnidirectional" || directivity == "cardioid",
          "native acoustic directivity differs");
  spatial.directivity = directivity == "cardioid"
                            ? ql::ReceivingDirectivity::Cardioid
                            : ql::ReceivingDirectivity::Omnidirectional;
  spatial.propagation_delay = w::boolean(p::field(config, "propagation_delay"));
  spatial.transition_samples =
      0; // First install; replacement is separate custody.
  ql::SpatialMotionInput motion;
  motion.source_motion_ref = p::text(p::field(config, "source_motion_ref"));
  motion.receiver_motion_ref = p::text(p::field(config, "receiver_motion_ref"));
  motion.policy_ref = spatial.policy_ref;
  motion.policy_revision = spatial.policy_revision;
  motion.standing = spatial.standing;
  motion.source_velocity_metres_per_second =
      p::vec(p::field(config, "source_velocity_metres_per_second"));
  motion.receiver_velocity_metres_per_second =
      p::vec(p::field(config, "receiver_velocity_metres_per_second"));
  // Actual original emitter anchor stays in the retained configuration.
  // Both native producers use the same fixed order before the P pickup sum.
  const double elapsed_seconds =
      double(origin - birth) / immutable.input().sample_rate;
  if (origin != birth)
    for (unsigned axis = 0; axis < 3; ++axis)
      spatial.source_translation_metres[axis] +=
          motion.source_velocity_metres_per_second[axis] * elapsed_seconds;
  motion.origin_sample = origin;
  motion.end_sample = end;
  require(
      end > origin &&
          end - origin == p::exact(p::field(config, "span_samples")),
      "native acoustic trajectory span differs from original source choice");
  ql::PreparedMovingSpatialReceiving prepared(immutable, std::move(spatial),
                                              std::move(motion));
  const auto source = p::vec(p::field(candidate, "source_position_metres"));
  require(
      source == prepared.spatial().source_position_metres(),
      "native acoustic metric point detached from actual P pickup geometry");
  return prepared;
}
} // namespace ql::performance::acoustic_wire
#endif
