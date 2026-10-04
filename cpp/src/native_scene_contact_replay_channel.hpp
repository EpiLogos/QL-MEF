#ifndef QL_NATIVE_SCENE_CONTACT_REPLAY_CHANNEL_HPP
#define QL_NATIVE_SCENE_CONTACT_REPLAY_CHANNEL_HPP
// Private worker numerical compiler. The closed C Act and original native
// Rust source replay own the request. Reading this transport creates no Scene
// witness, queue admission, Engine, mutable P or active performance.
#include <ql/performance_management_wire.hpp>
#include <ql/physical_scene_contact_replay.hpp>
namespace ql::performance::scene_contact_transport {
inline constexpr const char *replay_request_schema =
    "ql.native-scene-contact-replay-request/v1";
inline ql::PreparedPhysicalBody replay_body(J *packet_value, J *basis,
                                            bool physical_pratibimba) {
  require_exact(packet::field(packet_value, "native_basis"), basis,
                "Contact replay detached the actual native body/source basis");
  return ql::PreparedPhysicalBody(
      ql::physical_wire::read_prepared_physical_body(
          packet::field(packet_value, "physical_body"),
          packet::field(basis, "m3"), physical_pratibimba));
}
inline void
verify_current_physical_checkpoint(const ql::PreparedPhysicalBody &current,
                                   const ql::PhysicalBodyCheckpoint &saved) {
  const auto &in = current.input();
  ql::require(
      saved.event_ref == in.event_ref && saved.subject_ref == in.subject_ref &&
          saved.preparation_ref == in.preparation_ref &&
          saved.state_ref == in.state_ref &&
          saved.source_coordinate == in.source_coordinate &&
          saved.source_revision == in.source_revision &&
          saved.pratibimba == in.pratibimba &&
          saved.source_generation == in.source_generation &&
          saved.geometry_ref == in.geometry_ref &&
          saved.geometry_revision == in.geometry_revision &&
          saved.geometry_source_ref == in.geometry_source_ref &&
          saved.geometry_standing == in.geometry_standing &&
          saved.material_ref == in.material.reference &&
          saved.material_revision == in.material.revision &&
          saved.material_source_ref == in.material.source_ref &&
          saved.material_standing == in.material.standing &&
          saved.eigenbasis_identity == current.eigenbasis_identity() &&
          saved.body_revision == in.body_revision &&
          saved.sample_rate == in.sample_rate &&
          saved.displacement_modal_metres.size() == current.mode_count() &&
          saved.velocity_modal_metres_per_second.size() == current.mode_count(),
      "Contact saved current P lost full original source/eigenbasis/face");
  // The existing paired stopped restore still independently validates q/v,
  // pickup/energy, all Engine queues/voices and receiving numerical state
  // before it mutates the one resident P. This pure compiler cannot replace it.
}
inline Json verify_replay(J *request) {
  ql::physical_wire::keys(request,
                          {"schema", "operation", "instance_ref", "session_ref",
                           "checkpoint_ref", "current_native_preparation",
                           "current_native_basis", "physical_pratibimba",
                           "rows", "checkpoint_wire"});
  ql::require(packet::string(packet::field(request, "schema")) ==
                      replay_request_schema &&
                  packet::string(packet::field(request, "operation")) ==
                      "contact-history-verify",
              "private original Contact replay operation differs");
  const auto instance = packet::ref(request, "instance_ref");
  const auto session = packet::ref(request, "session_ref");
  const auto checkpoint_ref = packet::ref(request, "checkpoint_ref");
  auto current = replay_body(
      packet::field(request, "current_native_preparation"),
      packet::field(request, "current_native_basis"),
      packet::boolean(packet::field(request, "physical_pratibimba")));
  const auto determination = packet::determination(packet::field(
      packet::field(request, "current_native_preparation"), "determination"));
  ql::require(determination.identity.instance == instance &&
                  determination.body_revision ==
                      current.input().body_revision &&
                  determination.body_preparation_ref ==
                      reference(current.input().preparation_ref.c_str()) &&
                  determination.body_state_ref ==
                      reference(current.input().state_ref.c_str()),
              "Contact selected current source/body instance differs");
  auto *rows = packet::field(request, "rows");
  ql::require(json_object_is_type(rows, json_type_array),
              "complete original Contact replay rows absent");
  std::vector<ReplayedContact> history;
  const auto count = json_object_array_length(rows);
  history.reserve(count);
  auto verified = wire::array();
  std::uint64_t previous_request = 0, previous_epoch = 0, previous_sequence = 0;
  for (std::size_t i = 0; i < count; ++i) {
    auto *row = json_object_array_get_idx(rows, i);
    ql::physical_wire::keys(row, {"native_preparation", "native_basis",
                                  "physical_pratibimba", "source",
                                  "native_admission"});
    auto *record = packet::field(row, "source");
    const auto ordinal =
        wire::decimal(packet::field(record, "original_native_request_id"));
    auto body =
        replay_body(packet::field(row, "native_preparation"),
                    packet::field(row, "native_basis"),
                    packet::boolean(packet::field(row, "physical_pratibimba")));
    const auto before = packet::determination(packet::field(
        packet::field(row, "native_preparation"), "determination"));
    auto replayed = compile_admission(body, before, record,
                                      packet::field(row, "native_admission"));
    const auto boundary_session =
        packet::ref(packet::field(record, "native_boundary"), "session_ref");
    ql::require(
        ordinal > previous_request && before.identity.instance == instance &&
            boundary_session == session && replayed.epoch >= previous_epoch &&
            (replayed.epoch != previous_epoch ||
             replayed.sequence > previous_sequence) &&
            (history.empty() ||
             replayed.occurrence.constructor_lineage ==
                 history.back().occurrence.constructor_lineage),
        "Contact original full history order/Scene/source/epoch changed");
    previous_request = ordinal;
    previous_epoch = replayed.epoch;
    previous_sequence = replayed.sequence;
    history.push_back(std::move(replayed));
    wire::append(verified.get(), json_object_get(record));
  }
  const auto original = management_transport::checkpoint_text(
      packet::field(request, "checkpoint_wire"));
  auto tokener = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  ql::require(bool(tokener), "original Contact checkpoint parser absent");
  json_tokener_set_flags(tokener.get(),
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto decoded = wire::own(json_tokener_parse_ex(tokener.get(), original.data(),
                                                 int(original.size())));
  ql::require(decoded &&
                  json_tokener_get_error(tokener.get()) ==
                      json_tokener_success &&
                  json_tokener_get_parse_end(tokener.get()) == original.size(),
              "complete original Contact checkpoint UTF-8/JSON differs");
  auto saved =
      management_checkpoint_transport::read_checkpoint_wire(decoded.get());
  const auto &audio = saved->native_pair.audio;
  ql::require(
      saved->session == session && audio.determination == determination &&
          audio.cursor == saved->native_pair.physical.samples_elapsed &&
          saved->transport_epoch >= previous_epoch &&
          (saved->transport_epoch != previous_epoch ||
           audio.accepted_sequence >= previous_sequence),
      "Contact original checkpoint source/queue/physical clock differs");
  verify_current_physical_checkpoint(current, saved->native_pair.physical);
  ql::require(
      history.empty() == !audio.contacts.history_present &&
          audio.version == (history.empty() ? 2u : 3u),
      "Contact original checkpoint lost/introduced full occurrence history");
  if (!history.empty())
    verify_contact_checkpoint(audio.contacts, current, audio.cursor,
                              audio.accepted_sequence, history);
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-scene-contact-replay-reply/v1");
  wire::text(out.get(), "operation", "contact-history-verify");
  wire::flag(out.get(), "accepted", true);
  wire::ref(out.get(), "instance_ref", instance);
  wire::ref(out.get(), "session_ref", session);
  wire::ref(out.get(), "checkpoint_ref", checkpoint_ref);
  wire::text(out.get(), "checkpoint_wire", original);
  wire::put(out.get(), "verified_sources", verified.release());
  return out;
}
} // namespace ql::performance::scene_contact_transport
#endif
