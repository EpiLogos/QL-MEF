#ifndef QL_PHYSICAL_SCENE_CONTACT_REPLAY_HPP
#define QL_PHYSICAL_SCENE_CONTACT_REPLAY_HPP
// Pure original-program compiler. It creates immutable numerical preparations
// only; it never constructs a mutable P, Scene witness, Engine or queue grant.
#include <cstring>
#include <ql/physical_scene_contact_wire.hpp>
namespace ql::performance::scene_contact_transport {
inline std::uint64_t bits(double value) noexcept {
  static_assert(sizeof(value) == sizeof(std::uint64_t));
  std::uint64_t out = 0;
  std::memcpy(&out, &value, sizeof(out));
  return out;
}
inline bool exact(J *a, J *b) {
  if (!a || !b)
    return a == b;
  const auto ta = json_object_get_type(a), tb = json_object_get_type(b);
  if ((ta == json_type_double || ta == json_type_int) &&
      (tb == json_type_double || tb == json_type_int)) {
    if (ta == json_type_double || tb == json_type_double) {
      const double x = json_object_get_double(a), y = json_object_get_double(b);
      return std::isfinite(x) && std::isfinite(y) && bits(x) == bits(y);
    }
    return json_object_equal(a, b);
  }
  if (ta != tb)
    return false;
  if (ta == json_type_array) {
    const auto n = json_object_array_length(a);
    if (n != json_object_array_length(b))
      return false;
    for (std::size_t i = 0; i < n; ++i)
      if (!exact(json_object_array_get_idx(a, i),
                 json_object_array_get_idx(b, i)))
        return false;
    return true;
  }
  if (ta == json_type_object) {
    if (json_object_object_length(a) != json_object_object_length(b))
      return false;
    json_object_object_foreach(a, key, value) {
      J *other = nullptr;
      if (!json_object_object_get_ex(b, key, &other) || !exact(value, other))
        return false;
    }
    return true;
  }
  return json_object_equal(a, b);
}
inline void require_exact(J *a, J *b, const char *why) {
  ql::require(exact(a, b), why);
}
inline void verify_original_programme(const PreparedContactProgram &programme,
                                      J *original, J *body, J *operands,
                                      J *force) {
  auto expected_original = ct::original(programme.original());
  auto expected_body = ct::body_input(programme.original_body());
  auto expected_operands = ct::operands(programme.operands());
  require_exact(expected_original.get(), original,
                "original contact route/geometry/policy differs");
  require_exact(expected_body.get(), body,
                "original contact body/descendant/face differs");
  require_exact(expected_operands.get(), operands,
                "original contact native operands differ");
  packet::array(force, ql::physical_max_frames);
  for (std::size_t i = 0; i < ql::physical_max_frames; ++i)
    ql::require(bits(packet::number(json_object_array_get_idx(force, i))) ==
                    bits(programme.force().force_newtons[i]),
                "contact lost or changed an original native force sample");
}
struct ReplayedContact {
  std::shared_ptr<const PreparedContactProgram> programme;
  NativeContactOccurrence occurrence{};
  NativeContactHandle handle{};
  Identity source{};
  std::uint64_t sequence = 0, epoch = 0;
};
inline std::shared_ptr<const PreparedContactProgram>
compile_record(const ql::PreparedPhysicalBody &body,
               const Determination &determination, J *record) {
  ql::physical_wire::keys(
      record, {"schema", "original_native_request_id", "scene_constructor",
               "authored_definition", "native_boundary", "occurrence",
               "original_gravity_input", "native_operands", "original_body",
               "force_newtons", "exciter_position_metres"});
  ql::require(packet::string(packet::field(record, "schema")) ==
                  "ql.native-scene-contact-source/v1",
              "original native contact source schema differs");
  auto *boundary = packet::field(record, "native_boundary");
  ql::physical_wire::keys(boundary, {"schema", "session_ref", "transport_epoch",
                                     "determination", "physical_body",
                                     "physical_eigenbasis",
                                     "native_trigger_sample", "queue_cursor",
                                     "queue_horizon", "accepted_sequence"});
  ql::require(packet::string(packet::field(boundary, "schema")) ==
                  "ql.native-scene-contact-boundary/v1",
              "original native contact boundary schema differs");
  auto *constructor = packet::field(record, "scene_constructor");
  constructor_transport_shape(constructor);
  const auto occurrence =
      ct::read_occurrence(packet::field(record, "occurrence"));
  const auto ordinal =
      wire::decimal(packet::field(record, "original_native_request_id"));
  ql::require(
      ordinal != 0 && occurrence.original_request_id == ordinal &&
          ct::read_ref<256>(packet::field(constructor, "instance_ref")) ==
              occurrence.constructor_lineage,
      "original contact occurrence lost actual constructor/Manager ordinal");
  const auto cursor =
      wire::decimal(packet::field(boundary, "native_trigger_sample"));
  ql::require(
      wire::decimal(packet::field(boundary, "queue_cursor")) <= cursor &&
          wire::decimal(packet::field(boundary, "queue_horizon")) == cursor &&
          wire::decimal(packet::field(boundary, "transport_epoch")) != 0,
      "original contact lost its native admission boundary");
  auto actual_body = ct::body_input(body.input());
  auto actual_determination = wire::determination(determination);
  require_exact(actual_body.get(), packet::field(boundary, "physical_body"),
                "contact boundary lost complete original body");
  require_exact(actual_body.get(), packet::field(record, "original_body"),
                "contact record relabelled its original physical body");
  require_exact(actual_determination.get(),
                packet::field(boundary, "determination"),
                "contact record lost original M1/M2/full source determination");
  ql::require(
      body.eigenbasis_identity() ==
          packet::string(packet::field(boundary, "physical_eigenbasis")),
      "contact original eigenbasis no longer reproduces");
  auto derived = ql::prepare_physical_scene_contact(
      body, cursor,
      read_definition(packet::field(record, "authored_definition")));
  auto programme = std::make_shared<const PreparedContactProgram>(
      body, cursor, derived.native_input);
  auto original = ct::original(derived.native_input);
  auto operands = ct::operands(programme->operands());
  auto position = ct::vector(derived.exciter_position_metres);
  require_exact(original.get(), packet::field(record, "original_gravity_input"),
                "contact geometry/gravity/route no longer reproduces");
  require_exact(
      operands.get(), packet::field(record, "native_operands"),
      "contact complete force/body/policy operands no longer reproduce");
  require_exact(position.get(),
                packet::field(record, "exciter_position_metres"),
                "contact current body exciter anchor was replaced");
  verify_original_programme(*programme,
                            packet::field(record, "original_gravity_input"),
                            packet::field(record, "original_body"),
                            packet::field(record, "native_operands"),
                            packet::field(record, "force_newtons"));
  return programme;
}
inline ReplayedContact compile_admission(const ql::PreparedPhysicalBody &body,
                                         const Determination &determination,
                                         J *record, J *admission) {
  ReplayedContact out;
  out.programme = compile_record(body, determination, record);
  out.occurrence = ct::read_occurrence(packet::field(record, "occurrence"));
  auto *boundary = packet::field(record, "native_boundary");
  const auto operation = packet::string(packet::field(admission, "operation"));
  ql::require(
      packet::string(packet::field(admission, "schema")) ==
              "ql.performance-worker-reply/v1" &&
          packet::boolean(packet::field(admission, "accepted")) &&
          (operation == "contact-trigger" || operation == "contact-apply"),
      "contact sidecar lacks the original actual accepted native pulse");
  auto *payload = packet::field(admission, "payload");
  ql::require(
      packet::boolean(packet::field(payload, "queue_committed")) &&
          !packet::boolean(packet::field(payload, "application_committed")) &&
          packet::string(packet::field(payload, "native_result")) == "accepted",
      "contact sidecar substituted a performed application for queue "
      "admission");
  require_exact(record, packet::field(payload, "contact_source"),
                "actual Contact ACK lost its complete original source");
  auto *queue = packet::field(payload, "score_admission");
  ql::physical_wire::keys(
      queue, {"schema", "queued", "session_ref", "transport_epoch",
              "queue_cursor", "queue_horizon", "input_ref", "event", "source"});
  ql::require(packet::string(packet::field(queue, "schema")) ==
                      "ql.native-score-admission/v1" &&
                  packet::boolean(packet::field(queue, "queued")),
              "contact original queue fact absent");
  const auto event = wire::read_operation(packet::field(queue, "event"));
  const auto &operands = out.programme->operands();
  ql::require(
      event.kind == Kind::Contact && event.contact.valid() &&
          event.has_requested_sample &&
          event.sample == operands.impact_sample &&
          event.requested_sample == operands.impact_sample &&
          event.touch == 0 && event.value == 0 && event.pitch_hz == 0 &&
          !event.late_admitted && event.identity == determination.identity &&
          event.sequence ==
              wire::decimal(packet::field(boundary, "accepted_sequence")) + 1 &&
          wire::decimal(packet::field(boundary, "accepted_sequence")) !=
              std::numeric_limits<std::uint64_t>::max(),
      "contact original handle/date/ordinal/source is detached from its queue");
  auto native_determination = wire::determination(determination);
  require_exact(native_determination.get(), packet::field(queue, "source"),
                "Contact ACK lost original complete queued source");
  require_exact(packet::field(queue, "session_ref"),
                packet::field(boundary, "session_ref"),
                "Contact ACK changed the native session");
  require_exact(packet::field(queue, "transport_epoch"),
                packet::field(boundary, "transport_epoch"),
                "Contact ACK changed the original transport epoch");
  ql::require(ct::read_ref<contact_reference_bytes>(
                  packet::field(queue, "input_ref")) == operands.contact_ref &&
                  wire::decimal(packet::field(queue, "queue_cursor")) >=
                      wire::decimal(packet::field(boundary, "queue_cursor")) &&
                  wire::decimal(packet::field(queue, "queue_cursor")) <=
                      operands.impact_sample &&
                  wire::decimal(packet::field(queue, "queue_horizon")) >=
                      operands.trigger_sample &&
                  wire::decimal(packet::field(queue, "queue_horizon")) <=
                      operands.impact_sample,
              "Contact ACK lost original native input/bounded admission clock");
  auto *reading = packet::field(admission, "reading");
  ql::require(
      wire::decimal(packet::field(reading, "accepted_sequence")) ==
              event.sequence &&
          wire::decimal(packet::field(reading, "transport_epoch")) ==
              wire::decimal(packet::field(queue, "transport_epoch")),
      "Contact original pulse no longer reports its actual queue sequence");
  out.handle = event.contact;
  out.source = event.identity;
  out.sequence = event.sequence;
  out.epoch = wire::decimal(packet::field(queue, "transport_epoch"));
  return out;
}
inline void
verify_contact_checkpoint(const NativeContactCheckpoint &checkpoint,
                          const ql::PreparedPhysicalBody &current,
                          std::uint64_t cursor, std::uint64_t accepted_sequence,
                          const std::vector<ReplayedContact> &history) {
  ql::require(
      !history.empty() && checkpoint.history_present &&
          checkpoint.constructor_lineage ==
              history.back().occurrence.constructor_lineage &&
          checkpoint.original_request_high_water ==
              history.back().occurrence.original_request_id,
      "contact checkpoint lost the complete original occurrence high-water");
  std::array<const ReplayedContact *, contact_slot_capacity> latest{};
  for (const auto &row : history) {
    ql::require(row.handle.valid(),
                "original Contact admission handle invalid");
    auto *previous = latest[row.handle.slot];
    ql::require(!previous ||
                    row.handle.generation > previous->handle.generation,
                "original Contact slot generation duplicated/regressed");
    latest[row.handle.slot] = &row;
  }
  for (std::size_t slot = 0; slot < contact_slot_capacity; ++slot) {
    const auto &saved = checkpoint.slots[slot];
    const auto *row = latest[slot];
    ql::require(saved.present == bool(row) &&
                    checkpoint.slot_generations[slot] ==
                        (row ? row->handle.generation : 0),
                "contact checkpoint lost an original admitted slot generation");
    if (!row)
      continue;
    auto original = ct::original(row->programme->original());
    auto body = ct::body_input(row->programme->original_body());
    auto operands = ct::operands(row->programme->operands());
    auto saved_original = ct::original(saved.original);
    auto saved_body = ct::body_input(saved.original_body);
    auto saved_operands = ct::operands(saved.operands);
    require_exact(original.get(), saved_original.get(),
                  "contact CP lost authored original geometry/force");
    require_exact(body.get(), saved_body.get(),
                  "contact CP relabelled an old body as current");
    require_exact(operands.get(), saved_operands.get(),
                  "contact CP lost full actual operands");
    ql::require(
        saved.handle == row->handle && saved.occurrence == row->occurrence &&
            saved.admission_sequence == row->sequence &&
            saved.source ==
                NativeContactSourceIdentity{
                    row->source.instance, row->source.event,
                    row->source.subject, row->source.m1_revision,
                    row->source.m2_generation} &&
            saved.force_newtons == row->programme->force().force_newtons,
        "contact CP lost actual queue/occurrence/source/force custody");
  }
  // Existing native numerical decoder verifies every partial impulse, status,
  // date, active-original/current body and zero-force suffix BEFORE P restore.
  (void)NativeContactSlots::prepare_restore(checkpoint, current, cursor,
                                            accepted_sequence);
}
} // namespace ql::performance::scene_contact_transport
#endif
