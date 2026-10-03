#ifndef QL_PERFORMANCE_RESIDENT_REGISTRY_WIRE_HPP
#define QL_PERFORMANCE_RESIDENT_REGISTRY_WIRE_HPP
#include <iomanip>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
#include <sstream>
namespace ql::performance::management_transport {
namespace resident_wire {
namespace wire = checkpoint_transport;
inline std::string token_text(const ql::NativeResidentToken &token) {
  ql::require(token.valid(), "actual native resident token absent");
  std::ostringstream out;
  out << "native-resident:v1:" << std::hex << std::setfill('0') << std::setw(16)
      << token.process_nonce[0] << std::setw(16) << token.process_nonce[1]
      << ':' << std::dec << token.ordinal;
  return out.str();
}
inline const char *role_text(NativeResidentRole role) {
  switch (role) {
  case NativeResidentRole::AudioEngine:
    return "audio_engine";
  case NativeResidentRole::PhysicalBody:
    return "physical_body";
  case NativeResidentRole::AcousticReceiving:
    return "acoustic_receiving";
  }
  throw std::invalid_argument("unknown native resident role");
}
// Serialize the complete original callback/stopped PhysicalSnapshot copy.
// No extra P read, native poll, cursor mapping or default-node completion.
inline wire::Json physical_snapshot(const ql::PhysicalSnapshot &p) {
  ql::require(p.version == 1 && p.node_count <= ql::physical_max_nodes &&
                  p.resident.valid(),
              "actual bounded native physical snapshot required");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-physical-snapshot/v1");
  wire::put(out.get(), "version", json_object_new_uint64(p.version));
  wire::text(out.get(), "resident_instance_ref", token_text(p.resident));
  wire::text(out.get(), "event_ref", p.event_ref.data());
  wire::text(out.get(), "subject_ref", p.subject_ref.data());
  wire::text(out.get(), "preparation_ref", p.preparation_ref.data());
  wire::text(out.get(), "state_ref", p.state_ref.data());
  wire::text(out.get(), "source_coordinate", p.source_coordinate.data());
  wire::text(out.get(), "source_revision", p.source_revision.data());
  wire::text(out.get(), "geometry_ref", p.geometry_ref.data());
  wire::text(out.get(), "geometry_revision", p.geometry_revision.data());
  wire::text(out.get(), "material_ref", p.material_ref.data());
  wire::text(out.get(), "material_revision", p.material_revision.data());
  wire::text(out.get(), "eigenbasis_identity", p.eigenbasis_identity.data());
  wire::u64(out.get(), "source_generation", p.source_generation);
  wire::u64(out.get(), "body_revision", p.body_revision);
  wire::u64(out.get(), "samples_elapsed", p.samples_elapsed);
  wire::put(out.get(), "sample_rate", json_object_new_uint64(p.sample_rate));
  wire::put(out.get(), "node_count", json_object_new_uint64(p.node_count));
  wire::flag(out.get(), "pratibimba", p.pratibimba);
  wire::real(out.get(), "pickup_linear", p.pickup_linear);
  wire::real(out.get(), "mechanical_energy_joules", p.mechanical_energy_joules);
  auto ids = wire::array(), rest = wire::array(), visible = wire::array();
  for (unsigned i = 0; i < p.node_count; ++i) {
    wire::append(ids.get(), json_object_new_string(
                                std::to_string(p.node_identity[i]).c_str()));
    auto r = wire::array(), v = wire::array();
    for (unsigned axis = 0; axis < 3; ++axis) {
      wire::append(r.get(),
                   json_object_new_double(p.rest_positions_metres[i][axis]));
      wire::append(v.get(),
                   json_object_new_double(p.visible_positions_metres[i][axis]));
    }
    wire::append(rest.get(), r.release());
    wire::append(visible.get(), v.release());
  }
  wire::put(out.get(), "node_identity", ids.release());
  wire::put(out.get(), "rest_positions_metres", rest.release());
  wire::put(out.get(), "visible_positions_metres", visible.release());
  return out;
}
inline wire::Json registry(const NativeResidentRegistry &registry) {
  ql::require(registry.version == 1 &&
                  (registry.count == 2 || registry.count == 3),
              "native resident registry absent/unbounded");
  const auto &r = registry.boundary;
  const auto &p = r.physical;
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-resident-consumer-registry/v1");
  wire::u64(out.get(), "transport_epoch", registry.transport_epoch);
  wire::u64(out.get(), "sample", r.samples_elapsed);
  wire::put(out.get(), "source", wire::identity(r.identity).release());
  wire::u64(out.get(), "m3_source_generation", p.source_generation);
  wire::u64(out.get(), "body_revision", p.body_revision);
  wire::text(out.get(), "preparation_ref", p.preparation_ref.data());
  wire::text(out.get(), "state_ref", p.state_ref.data());
  wire::text(out.get(), "source_coordinate", p.source_coordinate.data());
  wire::text(out.get(), "source_revision", p.source_revision.data());
  wire::text(out.get(), "eigenbasis_identity", p.eigenbasis_identity.data());
  wire::flag(out.get(), "physical_pratibimba", p.pratibimba);
  wire::text(out.get(), "origin", "actual-native-constructors-and-same-pulse");
  auto roles = wire::array();
  for (unsigned i = 0; i < registry.count; ++i) {
    const auto &v = registry.consumers[i];
    auto entry = wire::object();
    wire::text(entry.get(), "role", role_text(v.role));
    wire::text(entry.get(), "instance_ref", token_text(v.token));
    wire::u64(entry.get(), "generation", v.generation);
    wire::text(entry.get(), "generation_domain",
               "native-resident-construction");
    wire::u64(entry.get(), "sample", v.samples_elapsed);
    wire::flag(entry.get(), "callback_output_committed",
               v.callback_output_committed);
    wire::append(roles.get(), entry.release());
  }
  wire::put(out.get(), "required_consumers", roles.release());
  auto nodes = wire::array();
  for (unsigned i = 0; i < p.node_count; ++i) {
    auto node = wire::object();
    wire::u64(node.get(), "native_node_id", p.node_identity[i]);
    auto rest = wire::array(), visible = wire::array();
    for (unsigned axis = 0; axis < 3; ++axis) {
      wire::append(rest.get(),
                   json_object_new_double(p.rest_positions_metres[i][axis]));
      wire::append(visible.get(),
                   json_object_new_double(p.visible_positions_metres[i][axis]));
    }
    wire::put(node.get(), "rest_metres", rest.release());
    wire::put(node.get(), "visible_metres", visible.release());
    wire::append(nodes.get(), node.release());
  }
  wire::put(out.get(), "native_nodes", nodes.release());
  wire::text(out.get(), "document_address_correspondence",
             "requires-private-current-document-source-recipe-CAS");
  auto audio = wire::object();
  wire::text(audio.get(), "instance_ref", token_text(r.audio_resident));
  wire::u64(audio.get(), "sample", r.samples_elapsed);
  wire::u64(audio.get(), "last_applied_sequence", r.last_sequence);
  wire::u64(audio.get(), "last_applied_application_ordinal",
            r.last_applied_application_ordinal);
  wire::flag(audio.get(), "callback_output_committed",
             r.callback_output_committed);
  wire::flag(audio.get(), "available",
             r.available && r.recording.failure == RecordingFailure::None);
  wire::u64(audio.get(), "capture_drops", r.dropped_captures);
  wire::u64(audio.get(), "readback_drops", r.dropped_readbacks);
  wire::real(audio.get(), "peak_linear", r.peak);
  wire::real(audio.get(), "rms_linear", r.rms);
  wire::put(audio.get(), "recording", wire::recording(r.recording).release());
  wire::put(out.get(), "audio_observation", audio.release());
  auto physical = wire::object();
  wire::text(physical.get(), "instance_ref", token_text(p.resident));
  wire::u64(physical.get(), "sample", p.samples_elapsed);
  wire::real(physical.get(), "pickup_linear", p.pickup_linear);
  wire::real(physical.get(), "energy_joules", p.mechanical_energy_joules);
  wire::put(physical.get(), "snapshot", physical_snapshot(p).release());
  wire::put(out.get(), "physical_observation", physical.release());
  if (r.has_receiving) {
    auto receiving = wire::object();
    wire::text(receiving.get(), "instance_ref",
               token_text(r.receiving.resident));
    wire::u64(receiving.get(), "sample", r.receiving.samples_elapsed);
    wire::put(receiving.get(), "manifest",
              wire::receiving_manifest(r.receiving.manifest).release());
    wire::put(out.get(), "receiving_observation", receiving.release());
  } else {
    wire::put(out.get(), "receiving_observation", json_object_new_null());
  }
  return out;
}
} // namespace resident_wire
} // namespace ql::performance::management_transport
#endif
