#ifndef QL_PERFORMANCE_SCENE_CONTACT_WIRE_HPP
#define QL_PERFORMANCE_SCENE_CONTACT_WIRE_HPP
#include <ql/performance_management_wire.hpp>
#include <ql/performance_scene_contact_owner.hpp>
#include <ql/physical_scene_contact_replay.hpp>

namespace ql::performance::scene_contact_transport {
using J = json_object;
using Json = ql::physical_wire::Json;
namespace ct = contact_transport;
namespace mt = management_transport;
inline constexpr const char *request_schema =
    "ql.native-scene-contact-owner-request/v1";
// ONLY the retained worker dispatcher instantiates this channel. Root's Rust
// typed method admits the operation after the real C Scene/source/occurrence
// lease. Public Host/Performance commands and Control::execute do not dispatch
// this schema. A direct numeric test of this channel cannot prove that lease.
class Channel {
  struct Pending {
    std::unique_ptr<NativeSceneContactOccurrenceOwner> issuer;
    Json record = ql::physical_wire::own(nullptr);
    Json boundary = ql::physical_wire::own(nullptr);
    Json constructor = ql::physical_wire::own(nullptr);
    Identity source{};
    std::uint64_t epoch = 0;
    RetentionReservation retention{};
  };
  std::unique_ptr<Pending> pending_;
  bool last_queued_ = false;

  Json reply(mt::Control &control, const std::string &operation, bool accepted,
             const std::string &reason, Json payload) {
    // Exactly ONE actual same-owner pulse supplies application/input custody.
    auto pulse = control.owner_->pulse();
    control.timing_.committed(*pulse, control.owner_->transport_epoch());
    return control.serialize_pulse(operation, accepted, reason,
                                   std::move(payload), *pulse);
  }
  Json prepare(mt::Control &control, J *request, bool trigger = false) {
    ql::physical_wire::keys(
        request, {"schema", "operation", "session_ref",
                  "expected_transport_epoch", "expected_source",
                  "expected_body_revision", "original_native_request_id",
                  "scene_constructor", "definition", "retention_reservation"});
    control.scope(request);
    auto *constructor = packet::field(request, "scene_constructor");
    constructor_transport_shape(constructor);
    const auto ordinal =
        wire::decimal(packet::field(request, "original_native_request_id"));
    ql::require(
        ordinal != 0 &&
            (!pending_ ||
             ordinal > pending_->issuer->occurrence_.original_request_id),
        "native contact prepare lost its original Manager order");
    auto &native = control.owner_->native();
    ql::require(native.body && native.engine &&
                    native.engine->owns_physical_owner(native.body.get()),
                "native contact is detached from the sole retained P");
    const auto &prepared_body = native.body->preparation();
    const auto cursor = native.engine->admission_horizon();
    const auto &source = native.engine->source_for_native_admission();
    auto next = std::make_unique<Pending>();
    NativeContactOccurrence occurrence{};
    occurrence.constructor_lineage =
        ct::read_ref<256>(packet::field(constructor, "instance_ref"));
    occurrence.original_request_id = ordinal;
    next->issuer.reset(new NativeSceneContactOccurrenceOwner(
        occurrence, prepared_body, cursor,
        read_definition(packet::field(request, "definition"))));
    next->retention = read_retention_reservation(
        packet::field(request, "retention_reservation"));
    next->source = source.identity;
    next->epoch = control.owner_->transport_epoch();
    next->constructor = copy(constructor);
    next->boundary = wire::object();
    wire::text(next->boundary.get(), "schema",
               "ql.native-scene-contact-boundary/v1");
    wire::ref(next->boundary.get(), "session_ref",
              control.owner_->session_ref());
    wire::u64(next->boundary.get(), "transport_epoch", next->epoch);
    wire::put(next->boundary.get(), "determination",
              wire::determination(source).release());
    wire::put(next->boundary.get(), "physical_body",
              ct::body_input(prepared_body.input()).release());
    wire::text(next->boundary.get(), "physical_eigenbasis",
               prepared_body.eigenbasis_identity());
    wire::u64(next->boundary.get(), "native_trigger_sample", cursor);
    wire::u64(next->boundary.get(), "queue_cursor",
              native.engine->samples_elapsed());
    wire::u64(next->boundary.get(), "queue_horizon", cursor);
    wire::u64(next->boundary.get(), "accepted_sequence",
              native.engine->accepted_sequence());
    next->record = wire::object();
    wire::text(next->record.get(), "schema",
               "ql.native-scene-contact-source/v1");
    wire::u64(next->record.get(), "original_native_request_id", ordinal);
    wire::put(next->record.get(), "scene_constructor",
              copy(constructor).release());
    wire::put(next->record.get(), "authored_definition",
              copy(packet::field(request, "definition")).release());
    wire::put(next->record.get(), "native_boundary",
              copy(next->boundary.get()).release());
    wire::put(next->record.get(), "occurrence",
              ct::occurrence(occurrence).release());
    wire::put(next->record.get(), "original_gravity_input",
              ct::original(next->issuer->contact_.native_input).release());
    wire::put(next->record.get(), "native_operands",
              ct::operands(next->issuer->programme_->operands()).release());
    wire::put(
        next->record.get(), "original_body",
        ct::body_input(next->issuer->programme_->original_body()).release());
    auto force = wire::array();
    for (double sample : next->issuer->programme_->force().force_newtons)
      wire::append(force.get(), json_object_new_double(sample));
    wire::put(next->record.get(), "force_newtons", force.release());
    wire::put(
        next->record.get(), "exciter_position_metres",
        ct::vector(next->issuer->contact_.exciter_position_metres).release());
    auto payload = wire::object();
    wire::flag(payload.get(), "prepared_only", true);
    wire::flag(payload.get(), "queue_committed", false);
    wire::put(payload.get(), "prepared_contact",
              copy(next->record.get()).release());
    // Commit only control-owned numerical preparation, with no queue/sequence,
    // occurrence high-water, physical body or performed event mutation.
    pending_.swap(next);
    if (trigger)
      // The already-authored current Scene definition is qualified upstream.
      // One native operation chooses its clock and queues the collision; no
      // stale candidate is imported/redated and no intermediate pulse drains
      // applications before the actual queue receipt can be retained.
      return admit(control, "contact-trigger");
    return reply(control, "contact-prepare", true, "", std::move(payload));
  }
  Json apply(mt::Control &control, J *request) {
    ql::physical_wire::keys(request,
                            {"schema", "operation", "session_ref",
                             "expected_transport_epoch", "expected_source",
                             "expected_body_revision",
                             "original_native_request_id", "native_boundary",
                             "scene_constructor", "retention_reservation"});
    control.scope(request);
    ql::require(bool(pending_),
                "actual resident native contact preparation absent");
    const auto ordinal =
        wire::decimal(packet::field(request, "original_native_request_id"));
    ql::require(ordinal == pending_->issuer->occurrence_.original_request_id &&
                    json_object_equal(packet::field(request, "native_boundary"),
                                      pending_->boundary.get()) &&
                    same_constructor_lifetime(
                        pending_->constructor.get(),
                        packet::field(request, "scene_constructor")),
                "original Scene contact candidate/constructor changed before "
                "admission");
    const auto &engine = *control.owner_->native().engine;
    ql::require(engine.samples_elapsed() ==
                        wire::decimal(packet::field(pending_->boundary.get(),
                                                    "queue_cursor")) &&
                    engine.admission_horizon() ==
                        wire::decimal(packet::field(pending_->boundary.get(),
                                                    "queue_horizon")) &&
                    engine.accepted_sequence() ==
                        wire::decimal(packet::field(pending_->boundary.get(),
                                                    "accepted_sequence")),
                "actual native contact queue/cursor changed after preparation");
    pending_->retention = read_retention_reservation(
        packet::field(request, "retention_reservation"));
    return admit(control, "contact-apply");
  }
  Json admit(mt::Control &control, const std::string &operation) {
    const auto &native = control.owner_->native();
    const auto actual_body = ct::body_input(native.body->preparation().input());
    ql::require(pending_->epoch == control.owner_->transport_epoch() &&
                    pending_->source ==
                        native.engine->source_for_native_admission().identity &&
                    json_object_equal(actual_body.get(),
                                      packet::field(pending_->record.get(),
                                                    "original_body")) &&
                    native.body->preparation().eigenbasis_identity() ==
                        packet::string(packet::field(pending_->boundary.get(),
                                                     "physical_eigenbasis")),
                "native contact lost the complete actual current "
                "physical/source owner");
    // Recompile complete native long-double/ceil/force evidence against this
    // exact immutable owner BEFORE queue publication. No JSON record issues a
    // witness; the actual private Scene issuer below remains sole constructor.
    (void)compile_record(native.body->preparation(),
                         native.engine->source_for_native_admission(),
                         pending_->record.get());
    // Keep ONE actual pulse before the queue commit. Its original callback
    // applications and input journal travel unchanged on success or capacity
    // refusal. The serial native queue counter is refreshed below; no performed
    // Contact application is fabricated from admission.
    auto pulse = control.owner_->pulse();
    control.timing_.committed(*pulse, control.owner_->transport_epoch());
    auto payload = wire::object();
    wire::put(payload.get(), "contact_source",
              copy(pending_->record.get()).release());
    wire::flag(payload.get(), "queue_committed", false);
    wire::flag(payload.get(), "application_committed", false);
    wire::text(payload.get(), "native_result", "unavailable");
    auto out = control.serialize_pulse(
        operation, false, "actual native contact queue admission refused",
        std::move(payload), *pulse);
    // This local size envelope is NEVER returned as a receipt. Max-width
    // handles/counters bound the actual queued wire; ONLY enqueue below can
    // produce an admission and advance the original occurrence high-water.
    auto envelope = copy(out.get());
    auto *envelope_payload = packet::field(envelope.get(), "payload");
    const auto &future = native.engine->source_at_native_contact_sample(
        pending_->issuer->programme_->operands().impact_sample);
    const auto &operands = pending_->issuer->programme_->operands();
    auto future_wire = wire::determination(future);
    if (!exact(future_wire.get(),
               packet::field(pending_->boundary.get(), "determination"))) {
      wire::text(out.get(), "reason",
                 "native contact future source changed before impact");
      wire::text(packet::field(out.get(), "payload"), "native_result", "stale");
      return out;
    }
    Operation bounded_event{};
    bounded_event.kind = Kind::Contact;
    bounded_event.contact = {std::numeric_limits<std::uint64_t>::max(), 15};
    bounded_event.identity = future.identity;
    bounded_event.sequence = std::numeric_limits<std::uint64_t>::max();
    bounded_event.sample = bounded_event.requested_sample =
        operands.impact_sample;
    bounded_event.has_requested_sample = true;
    auto bounded_admission = wire::object();
    wire::text(bounded_admission.get(), "schema",
               "ql.native-score-admission/v1");
    wire::flag(bounded_admission.get(), "queued", true);
    wire::ref(bounded_admission.get(), "session_ref",
              control.owner_->session_ref());
    for (const char *key : {"transport_epoch", "queue_cursor", "queue_horizon"})
      wire::u64(bounded_admission.get(), key,
                std::numeric_limits<std::uint64_t>::max());
    ct::ref(bounded_admission.get(), "input_ref", operands.contact_ref);
    wire::put(bounded_admission.get(), "event",
              wire::operation(bounded_event).release());
    wire::put(bounded_admission.get(), "source",
              wire::determination(future).release());
    wire::put(envelope_payload, "score_admission", bounded_admission.release());
    wire::u64(packet::field(envelope.get(), "reading"), "accepted_sequence",
              std::numeric_limits<std::uint64_t>::max());
    const bool fits = retained_json_upper_bound(pending_->record.get()) <=
                          pending_->retention.source_record_bytes_limit &&
                      retained_json_upper_bound(envelope.get()) <=
                          pending_->retention.native_pulse_bytes_limit;
    if (!fits) {
      wire::text(out.get(), "reason",
                 "complete native contact retention reservation refused");
      wire::text(packet::field(out.get(), "payload"), "native_result",
                 "overflow");
      return out;
    }
    auto witness = pending_->issuer->witness();
    auto admitted =
        control.enqueue_scene_contact(pending_->issuer->programme_, witness);
    last_queued_ = admitted.queue().queued();
    auto *actual_payload = packet::field(out.get(), "payload");
    wire::flag(actual_payload, "queue_committed", last_queued_);
    wire::text(actual_payload, "native_result",
               mt::Control::result(admitted.result()));
    wire::flag(out.get(), "accepted", admitted.result() == Result::Accepted);
    wire::text(out.get(), "reason",
               admitted.result() == Result::Accepted
                   ? ""
                   : "actual native contact queue admission refused");
    wire::u64(packet::field(out.get(), "reading"), "accepted_sequence",
              native.engine->accepted_sequence());
    if (last_queued_) {
      wire::put(actual_payload, "score_admission",
                mt::score_admission(admitted).release());
      control.timing_.score(admitted);
      pending_.reset();
    }
    return out;
  }

public:
  Channel() = default;
  Channel(const Channel &) = delete;
  Channel &operator=(const Channel &) = delete;
  bool last_queue_committed() const noexcept { return last_queued_; }
  Json execute(mt::Control &control, J *request) {
    last_queued_ = false;
    ql::require(control.active() && packet::string(packet::field(
                                        request, "schema")) == request_schema,
                "private native Scene contact worker channel unavailable");
    const auto operation = packet::string(packet::field(request, "operation"));
    if (operation == "contact-prepare")
      return prepare(control, request);
    if (operation == "contact-trigger")
      return prepare(control, request, true);
    ql::require(operation == "contact-apply",
                "unknown private native Scene contact operation");
    return apply(control, request);
  }
};
} // namespace ql::performance::scene_contact_transport
#endif
