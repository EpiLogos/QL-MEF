#ifndef QL_PERFORMANCE_FORM_WIRE_HPP
#define QL_PERFORMANCE_FORM_WIRE_HPP
// Numerical consumer of the retained Rust owner's private source transaction.
// No fixture/JSON/hash issues a private Scene, Act or source permission here.
#include <ql/performance_acoustic_wire.hpp>
#include <ql/performance_management_checkpoint.hpp>
#include <ql/performance_physical_revision.hpp>
namespace ql::performance::form_transport {
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
inline J *nullable(J *object, const char *name) {
  J *value = nullptr;
  require(json_object_object_get_ex(object, name, &value),
          "missing native Form optional field");
  return value;
}
inline Json copy(J *source) {
  J *target = nullptr;
  const int result = json_object_deep_copy(source, &target, nullptr);
  auto retained = wire::own(target);
  require(result == 0 && retained, "native Form source copy failed");
  return retained;
}
inline Json descriptor(J *packet, const ql::PreparedPhysicalBody &prepared,
                       std::uint64_t cursor) {
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-physical-descriptor/v1");
  wire::put(
      out.get(), "physical_preparation",
      json_object_get(ql::performance::packet::field(packet, "physical_body")));
  wire::text(out.get(), "eigenbasis_identity", prepared.eigenbasis_identity());
  auto frequencies = wire::array();
  for (std::size_t i = 0; i < prepared.mode_count(); ++i)
    wire::append(frequencies.get(),
                 json_object_new_double(prepared.frequency_hz(i)));
  wire::put(out.get(), "mode_frequencies_hz", frequencies.release());
  wire::u64(out.get(), "native_cursor", cursor);
  return out;
}
struct Applied {
  Json source = wire::own(nullptr), basis = wire::own(nullptr),
       acoustic = wire::own(nullptr);
  Json acknowledgement = wire::own(nullptr),
       body_descriptor = wire::own(nullptr);
  Ref recipe{};
  std::uint64_t generation = 0, request_id = 0;
  std::size_t route_count = 0;
};
inline Applied apply(PerformanceManagement &owner, J *request,
                     J *retained_source, J *retained_basis,
                     J *retained_acoustic,
                     std::uint64_t last_physical_request_id,
                     std::vector<KeyboardCell> catalog) {
  const auto device = owner.device_receipt().state;
  require(
      device == DeviceState::Closed || device == DeviceState::Prepared,
      "native physical source edit requires actual attached device stopped");
  auto &engine = *owner.native().engine;
  auto guard = engine.acquire_stopped_custody();
  require(bool(guard), "native physical exclusive stopped custody absent");
  const auto sample = wire::decimal(packet::field(request, "expected_sample"));
  const auto ordinal =
      wire::decimal(packet::field(request, "original_request_id"));
  require(ordinal > last_physical_request_id &&
              sample == engine.samples_elapsed() &&
              owner.native().body->samples_elapsed() == sample &&
              retained_source && retained_basis,
          "native physical edit request/cursor/original producer is stale");
  require(json_object_equal(packet::field(request, "before_packet"),
                            retained_source) &&
              json_object_equal(packet::field(request, "before_native_basis"),
                                retained_basis),
          "native physical edit lost complete original operative preparation");
  auto *after_packet = packet::field(request, "after_packet");
  auto *actual_packet = packet::field(request, "actual_after_packet");
  auto *basis = packet::field(request, "actual_after_native_basis");
  require(json_object_equal(after_packet, actual_packet) &&
              json_object_equal(packet::field(actual_packet, "native_basis"),
                                basis),
          "native physical current complete AFTER producer differs");
  const auto kind = packet::string(packet::field(request, "kind"));
  require(kind == "material" || kind == "form",
          "unknown actual physical source operation");
  auto after = prepare_performance_packet(
      json_object_to_json_string_ext(after_packet, JSON_C_TO_STRING_PLAIN),
      basis, owner.native().determination.m1_face == 1,
      owner.native().body->preparation().input().pratibimba);
  const auto &pending = after.body->preparation();
  const auto &original = owner.native().body->preparation();
  require(pending.input().body_revision == original.input().body_revision + 1,
          "native physical edit skipped its actual body revision");
  auto *standing = packet::field(request, "body_source");
  wire::keys(standing, {"kind", "recipe_ref", "validated_m3_generation"});
  require(
      packet::string(packet::field(standing, "kind")) == "sourceForm" &&
          wire::decimal(packet::field(standing, "validated_m3_generation")) ==
              pending.input().source_generation,
      "native physical edit lost genuine source-form generation");
  const auto recipe = packet::ref(standing, "recipe_ref");
  auto *admission = packet::field(request, "receiving_admission");
  auto *current = packet::field(request, "current_receiving_admission");
  auto *sources = packet::field(packet::field(current, "operation"), "sources");
  require(json_object_is_type(sources, json_type_array) &&
              json_object_array_length(sources) <= 9 && !after.notes.empty(),
          "native physical AFTER source/M1 phase absent");
  std::vector<std::string> programme_refs;
  for (std::size_t i = 0; i < json_object_array_length(sources); ++i)
    programme_refs.push_back(
        packet::string(packet::field(json_object_array_get_idx(sources, i),
                                     "driver_ref")) +
        "/m1-excitation-program");
  auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(admission, current, after, basis, pending,
                                      programme_refs, sample));
  auto routes = std::make_shared<PhysicalRoutesPortBinding>(
      owner.native().body, admitted, pending, after.determination, basis,
      after.determination.m1_face == 1, sample);
  NativeRouteProgramSet seed{};
  seed.manifest = routes->manifest();
  seed.program_count = seed.manifest.route_count;
  seed.scalar_note_enabled = seed.manifest.scalar_note_enabled;
  seed.scalar_note_gain = seed.manifest.scalar_note_gain;
  for (std::size_t i = 0; i < seed.program_count; ++i) {
    seed.programs[i].handle = seed.manifest.programs[i];
    seed.programs[i].phase_source_ref = seed.manifest.m1_coordinate;
    seed.programs[i].sine = after.notes.front().phase_sin;
    seed.programs[i].cosine = after.notes.front().phase_cos;
  }
  for (const auto &note : after.notes)
    require(note.phase_sin == after.notes.front().phase_sin &&
                note.phase_cos == after.notes.front().phase_cos,
            "native physical AFTER M1 quadrature differs");
  auto *cells = packet::field(request, "native_catalog");
  require(json_object_is_type(cells, json_type_array) &&
              json_object_array_length(cells) <= 192,
          "native physical catalogue exceeds existing bound");
  require(catalog.size() == json_object_array_length(cells),
          "native physical catalogue was not completely decoded");
  auto transition = std::make_unique<ql::PreparedPhysicalTransition>(
      original, pending, ordinal, sample,
      kind == "material" ? ql::PhysicalLiveUpdateKind::Material
                         : ql::PhysicalLiveUpdateKind::FormOrBoundary,
      ql::PhysicalFormTransition::ProjectCorrespondingNodes,
      packet::string(packet::field(request, "cause_ref")));
  auto before = std::make_unique<Engine::Checkpoint>();
  engine.write_checkpoint(*before, guard);
  const bool receiving = before->has_receiving;
  auto *before_acoustic = nullable(request, "before_acoustic");
  auto *candidate_acoustic = nullable(request, "prepared_acoustic");
  auto *current_acoustic = nullable(request, "current_acoustic");
  std::shared_ptr<MovingReceivingPortBinding> receiver;
  ReceivingPort receiver_port{};
  if (receiving) {
    require(retained_acoustic && before_acoustic && candidate_acoustic &&
                current_acoustic &&
                json_object_equal(before_acoustic, retained_acoustic),
            "native physical edit lost actual original acoustic producer");
    require(
        json_object_equal(packet::field(before_acoustic, "configuration"),
                          packet::field(candidate_acoustic, "configuration")) &&
            json_object_equal(packet::field(before_acoustic, "context"),
                              packet::field(candidate_acoustic, "context")),
        "native physical edit substituted original receiver/source trajectory");
    for (const char *name :
         {"origin_sample", "end_sample", "history_origin_sample"})
      require(
          json_object_equal(packet::field(before_acoustic, name),
                            packet::field(candidate_acoustic, name)),
          "native physical edit reset original receiver date/history birth");
    const auto origin =
        wire::decimal(packet::field(candidate_acoustic, "origin_sample"));
    auto spatial = acoustic_wire::read_prepared_acoustic(
        candidate_acoustic, current_acoustic,
        packet::field(after_packet, "physical_body"), pending, origin,
        acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
    receiver = MovingReceivingPortBinding::from_prepared_body_transition(
        owner.native().body, *transition, std::move(spatial),
        before->receiving);
    receiver_port = receiver->port(receiver);
  } else {
    require(!retained_acoustic && !before_acoustic && !candidate_acoustic &&
                !current_acoustic,
            "native physical edit invented an uninstalled receiver");
  }
  auto transaction = std::make_unique<PreparedRetainedBodyRevision>(
      owner, std::move(transition), admitted, after.determination, basis, seed,
      after.notes, std::move(catalog), guard,
      receiving ? &receiver_port : nullptr);
  require(transaction->current(guard),
          "native physical full BEFORE/AFTER transaction changed");
  Applied result;
  result.source = copy(after_packet);
  result.basis = copy(basis);
  if (receiving)
    result.acoustic = copy(candidate_acoustic);
  result.recipe = recipe;
  result.generation = pending.input().source_generation;
  result.request_id = ordinal;
  result.route_count = seed.program_count;
  result.body_descriptor = descriptor(after_packet, pending, sample);
  auto ack = wire::object();
  wire::text(ack.get(), "schema", "ql.native-physical-source-application/v1");
  wire::u64(ack.get(), "original_native_request_id", ordinal);
  wire::u64(ack.get(), "native_sample", sample);
  wire::text(ack.get(), "kind", kind);
  wire::text(ack.get(), "policy", "project-corresponding-nodes");
  wire::u64(ack.get(), "before_body_revision", original.input().body_revision);
  wire::u64(ack.get(), "after_body_revision", pending.input().body_revision);
  wire::text(ack.get(), "before_eigenbasis_identity",
             original.eigenbasis_identity());
  wire::text(ack.get(), "after_eigenbasis_identity",
             pending.eigenbasis_identity());
  wire::put(ack.get(), "before_native_preparation",
            json_object_get(retained_source));
  wire::put(ack.get(), "after_native_preparation",
            json_object_get(after_packet));
  wire::u64(ack.get(), "transport_epoch", owner.transport_epoch());
  wire::u64(ack.get(), "accepted_sequence", engine.accepted_sequence());
  for (const char *name :
       {"before_energy_joules", "after_energy_joules", "external_work_joules"})
    wire::real(ack.get(), name, 0.);
  ql::PhysicalLiveTransitionReceipt receipt{};
  require(transaction->current(guard) && transaction->commit(guard, receipt),
          "native physical same-owner transaction refused before mutation");
  json_object_set_double(packet::field(ack.get(), "before_energy_joules"),
                         receipt.before_energy_joules);
  json_object_set_double(packet::field(ack.get(), "after_energy_joules"),
                         receipt.after_energy_joules);
  json_object_set_double(packet::field(ack.get(), "external_work_joules"),
                         receipt.external_work_joules);
  owner.refresh_stopped_reading(guard);
  result.acknowledgement = std::move(ack);
  return result;
}
} // namespace ql::performance::form_transport
#endif
