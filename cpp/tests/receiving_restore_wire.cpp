// Genuine Rust current source/receiving -> actual P/A/Management continuation.
// This native component test grants no installed Act/lease or consent
// authority.
#include <iostream>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_receiving_restore.hpp>
using namespace ql::performance;
using ql::require;
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;

static std::vector<KeyboardCell> catalog(J *fixture) {
  auto *cells = packet::field(fixture, "native_catalog");
  packet::array(cells, 36);
  std::vector<KeyboardCell> result;
  for (std::size_t i = 0; i < 36; ++i)
    result.push_back(
        management_transport::read_key(json_object_array_get_idx(cells, i)));
  return result;
}
static std::unique_ptr<PerformanceManagement> owner(J *fixture) {
  auto *p = packet::field(fixture, "native_preparation");
  auto *d = packet::field(p, "determination");
  auto *coordinate =
      packet::field(packet::field(p, "physical_body"), "source_coordinate");
  auto native = prepare_source_performance_packet(
      json_object_to_json_string_ext(p, JSON_C_TO_STRING_PLAIN), p,
      packet::field(fixture, "native_basis"),
      packet::integer(packet::field(d, "m1_face")) == 1,
      packet::string(packet::field(coordinate, "face")) == "pratibimba");
  auto result = std::make_unique<PerformanceManagement>(
      std::move(native),
      reference("native:receiving-restore/retained-session"));
  result->admit_catalog(catalog(fixture));
  return result;
}
struct Receiving {
  std::shared_ptr<PhysicalRoutesPortBinding> binding;
  std::unique_ptr<NativeRouteProgramSet> seed;
  PhysicalPort port;
};
static Receiving receiving(PerformanceManagement &manager, J *fixture,
                           J *current, std::uint64_t cursor) {
  auto *basis = packet::field(fixture, "native_basis");
  auto *admission = packet::field(current, "native_admission");
  auto *sources =
      packet::field(packet::field(admission, "operation"), "sources");
  std::vector<std::string> refs;
  for (std::size_t i = 0; i < json_object_array_length(sources); ++i)
    refs.push_back(packet::string(packet::field(
                       json_object_array_get_idx(sources, i), "driver_ref")) +
                   "/m1-excitation-program");
  // Body is stopped here. Numerical preparation is immutable; no control
  // query of the live q/v cursor provides the admitted saved cursor.
  const auto immutable = manager.native().body->preparation();
  auto typed = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(admission, admission, manager.native(),
                                      basis, immutable, refs, cursor));
  const auto &d = manager.native().determination;
  auto binding = std::make_shared<PhysicalRoutesPortBinding>(
      manager.native().body, typed, immutable, d, basis, d.m1_face == 1,
      cursor);
  auto seed = std::make_unique<NativeRouteProgramSet>();
  seed->manifest = binding->manifest();
  seed->program_count = seed->manifest.route_count;
  seed->scalar_note_enabled = seed->manifest.scalar_note_enabled;
  seed->scalar_note_gain = seed->manifest.scalar_note_gain;
  for (std::size_t i = 0; i < seed->program_count; ++i) {
    auto &program = seed->programs[i];
    program.handle = seed->manifest.programs[i];
    program.phase_source_ref = seed->manifest.m1_coordinate;
    program.sine = manager.native().notes.front().phase_sin;
    program.cosine = manager.native().notes.front().phase_cos;
  }
  auto port =
      physical_routes_port(physical_port(manager.native().body), binding);
  return {std::move(binding), std::move(seed), std::move(port)};
}
static std::vector<float> render(PerformanceManagement &manager,
                                 unsigned frames) {
  std::vector<float> result(frames);
  require(manager.offline_advance(result.data(), frames,
                                  manager.native().engine->samples_elapsed()),
          "actual native restored callback refused");
  auto pulse = manager.pulse();
  require(pulse->has_readback && pulse->reading.available &&
              pulse->reading.samples_elapsed ==
                  pulse->reading.physical.samples_elapsed &&
              pulse->recording.failure == RecordingFailure::None,
          "actual restored callback/readback/recording disconnected");
  return result;
}
static bool same_json(Json a, Json b) {
  return json_object_equal(a.get(), b.get());
}
static Json clone_json(J *value) {
  J *raw = nullptr;
  const auto result = json_object_deep_copy(value, &raw, nullptr);
  auto copy = wire::own(raw);
  require(result == 0 && copy, "actual native test input copy failed");
  return copy;
}
// The actual retained Control operation, on genuine native source producers.
// Separate numerical owners are independent test trials, never app owners.
static Json control_trial(J *fixture) {
  auto original = owner(fixture);
  auto initial =
      receiving(*original, fixture, packet::field(fixture, "initial"), 0);
  if (initial.seed->program_count) {
    initial.seed->programs[0].enabled = false;
    initial.seed->programs[0].target_gain = .42;
    initial.seed->programs[0].effective_gain = .11;
    initial.seed->programs[1].target_gain = .63;
    initial.seed->programs[1].effective_gain = .29;
  }
  require(original->admit_distinct_receiving(initial.port, *initial.seed, 0)
                  .result == Result::Accepted,
          "real Control source initial receiving refused");
  Operation attack{};
  attack.kind = Kind::NoteOn;
  attack.identity = original->native().determination.identity;
  attack.sequence = 1;
  attack.note = original->native().notes.front();
  attack.value = .7;
  require(original->enqueue_score_input(
              attack, reference("native:receiving-restore/control-input")) ==
              Result::Accepted,
          "real Control source note refused");
  Operation future{};
  future.kind = Kind::Parameter;
  future.identity = attack.identity;
  future.sequence = 2;
  future.sample = 48000;
  future.parameter = Parameter::MasterLinear;
  future.value = .8;
  require(original->enqueue_score_input(future) == Result::Accepted,
          "real Control source future parameter refused");
  std::array<float, max_frames> buffer{};
  require(original->offline_advance(buffer.data(), 128, 0) &&
              original->offline_advance(buffer.data(), 512, 128),
          "real Control source callback refused");
  // No pulse has consumed the genuine NoteOn application's/input history.
  auto saved = original->stopped_checkpoint();
  require(saved->native_pair.audio.cursor == 640 &&
              saved->bindings.write > saved->bindings.read &&
              saved->native_pair.audio.applications.write >
                  saved->native_pair.audio.applications.read,
          "actual unread application/input history required for drain proof");
  auto saved_wire = management_checkpoint_transport::checkpoint_wire(*saved);
  std::string original_text =
      json_object_to_json_string_ext(saved_wire.get(), JSON_C_TO_STRING_PLAIN);
  bool c_original_verified = false;
  J *c_original = nullptr;
  if (json_object_object_get_ex(fixture, "control_original_checkpoint_wire",
                                &c_original)) {
    const auto supplied = management_transport::checkpoint_text(c_original);
    auto tokener = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
        json_tokener_new_ex(64), json_tokener_free);
    require(bool(tokener), "second native checkpoint parser allocation failed");
    json_tokener_set_flags(tokener.get(),
                           JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
    auto parsed = wire::own(json_tokener_parse_ex(
        tokener.get(), supplied.data(), int(supplied.size())));
    require(json_tokener_get_error(tokener.get()) == json_tokener_success &&
                parsed &&
                json_tokener_get_parse_end(tokener.get()) == supplied.size(),
            "second C-emitted checkpoint text is incomplete or invalid UTF8");
    auto qualified =
        management_checkpoint_transport::read_checkpoint_wire(parsed.get());
    auto qualified_wire =
        management_checkpoint_transport::checkpoint_wire(*qualified);
    require(json_object_equal(saved_wire.get(), qualified_wire.get()),
            "C-emitted checkpoint differs from independently regenerated "
            "native source state");
    // Exact native target bits, including signed zero. Only these four fields
    // cross C's typed Scalar serialization; no recursive float tolerance.
    const auto same_target_bits = [](const NoteTarget &left,
                                     const NoteTarget &right) {
      const auto bits = [](double value) {
        std::uint64_t out;
        std::memcpy(&out, &value, sizeof(out));
        return out;
      };
      return bits(left.hertz) == bits(right.hertz) &&
             bits(left.fundamental_hz) == bits(right.fundamental_hz) &&
             bits(left.phase_sin) == bits(right.phase_sin) &&
             bits(left.phase_cos) == bits(right.phase_cos);
    };
    for (std::size_t i = 0; i < saved->bindings.inputs.size(); ++i)
      if (saved->bindings.inputs[i].active)
        require(same_target_bits(saved->bindings.inputs[i].target,
                                 qualified->bindings.inputs[i].target),
                "C-emitted checkpoint changed a native input target bit");
    for (auto i = saved->bindings.read; i < saved->bindings.write; ++i)
      require(same_target_bits(saved->bindings.history[i % 256].target,
                               qualified->bindings.history[i % 256].target),
              "C-emitted checkpoint changed a native history target bit");
    original_text = supplied; // retain exact actual C-emitted text, not a retag
    c_original_verified = true;
  }
  std::array<std::vector<float>, 2> expected;
  for (auto &chunk : expected)
    chunk = render(*original, 512);
  auto final_original = original->stopped_checkpoint();
  management_transport::Control control;
  auto *packet_value = packet::field(fixture, "native_preparation");
  auto *d = packet::field(packet_value, "determination");
  auto *physical = packet::field(packet_value, "physical_body");
  auto prepare = wire::object();
  wire::text(prepare.get(), "schema", "ql.performance-control/v1");
  wire::text(prepare.get(), "operation", "prepare");
  wire::text(prepare.get(), "session_ref",
             "native:receiving-restore/retained-session");
  wire::put(prepare.get(), "packet", clone_json(packet_value).release());
  wire::put(prepare.get(), "current_source_packet",
            clone_json(packet_value).release());
  wire::put(prepare.get(), "actual_native_basis",
            clone_json(packet::field(fixture, "native_basis")).release());
  wire::flag(prepare.get(), "m1_pratibimba",
             packet::integer(packet::field(d, "m1_face")) == 1);
  wire::flag(
      prepare.get(), "physical_pratibimba",
      packet::string(packet::field(packet::field(physical, "source_coordinate"),
                                   "face")) == "pratibimba");
  auto body_source = wire::object();
  wire::text(body_source.get(), "kind", "sourceForm");
  wire::put(body_source.get(), "recipe_ref",
            json_object_get(packet::field(
                packet::field(packet::field(packet_value, "source_form_recipe"),
                              "provenance"),
                "reference")));
  wire::u64(body_source.get(), "validated_m3_generation",
            packet::integer(packet::field(physical, "source_generation")));
  wire::put(prepare.get(), "body_source", body_source.release());
  auto *at_zero =
      packet::field(packet::field(fixture, "initial"), "native_admission");
  wire::put(prepare.get(), "receiving_admission",
            clone_json(at_zero).release());
  wire::put(prepare.get(), "current_receiving_admission",
            clone_json(at_zero).release());
  auto prepared = control.execute(prepare.get());
  require(packet::boolean(packet::field(prepared.get(), "accepted")),
          "actual Control prepare refused");
  const auto base = [&](const char *op, J *reply) {
    auto request = wire::object();
    wire::text(request.get(), "schema", "ql.performance-control/v1");
    wire::text(request.get(), "operation", op);
    wire::text(request.get(), "session_ref",
               "native:receiving-restore/retained-session");
    auto *reading = packet::field(reply, "reading");
    wire::put(request.get(), "expected_transport_epoch",
              json_object_get(packet::field(reading, "transport_epoch")));
    wire::put(request.get(), "expected_source",
              clone_json(packet::field(d, "identity")).release());
    wire::put(request.get(), "expected_body_revision",
              json_object_get(packet::field(packet::field(reading, "scope"),
                                            "body_revision")));
    return request;
  };
  auto catalog_request = base("catalog", prepared.get());
  wire::put(catalog_request.get(), "cells",
            clone_json(packet::field(fixture, "native_catalog")).release());
  wire::put(catalog_request.get(), "transpose", json_object_new_uint64(0));
  auto current = control.execute(catalog_request.get());
  require(packet::boolean(packet::field(current.get(), "accepted")),
          "actual Control catalog refused");
  const auto checkpoint = [&](J *reading) {
    auto request = base("checkpoint", reading);
    auto reply = control.execute(request.get());
    require(packet::boolean(packet::field(reply.get(), "accepted")),
            "actual Control checkpoint refused");
    return clone_json(
        packet::field(packet::field(reply.get(), "payload"), "checkpoint"));
  };
  auto before = checkpoint(current.get());
  auto request = base("restore-current-receiving", current.get());
  wire::text(request.get(), "original_checkpoint_wire", original_text);
  wire::u64(request.get(), "expected_cursor", 0);
  wire::text(request.get(), "transaction_ref",
             "native:receiving-restore/control-transaction");
  wire::text(request.get(), "checkpoint_ref",
             "native:receiving-restore/control-checkpoint");
  wire::put(request.get(), "current_source_packet",
            clone_json(packet_value).release());
  wire::put(request.get(), "actual_native_basis",
            clone_json(packet::field(fixture, "native_basis")).release());
  auto *fresh = packet::field(fixture, "saved_current");
  auto *admission = packet::field(fresh, "native_admission");
  wire::put(request.get(), "receiving_admission",
            clone_json(admission).release());
  wire::put(request.get(), "current_receiving_admission",
            clone_json(admission).release());
  wire::put(request.get(), "current_receiving", clone_json(fresh).release());
  wire::put(request.get(), "native_catalog",
            clone_json(packet::field(fixture, "native_catalog")).release());
  require(original_text.size() > 256,
          "actual complete checkpoint must exercise full text transport");
  const auto malformed_checkpoint = [&](const std::string &text) {
    auto invalid = clone_json(request.get());
    wire::put(invalid.get(), "original_checkpoint_wire",
              json_object_new_string_len(text.data(), int(text.size())));
    bool refused = false;
    try {
      (void)control.execute(invalid.get());
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    require(refused &&
                same_json(clone_json(before.get()), checkpoint(current.get())),
            "malformed checkpoint text changed resident native state");
  };
  malformed_checkpoint(original_text.substr(0, original_text.size() - 1));
  malformed_checkpoint(original_text + std::string(1, '\0'));
  malformed_checkpoint(std::string(32 * 1024 * 1024 + 1, 'x'));
  auto invalid_utf8 = original_text;
  const auto schema_text = invalid_utf8.find("ql.performance-management");
  require(schema_text != std::string::npos,
          "actual Management schema string missing from UTF8 trial");
  invalid_utf8[schema_text] = char(0xff);
  malformed_checkpoint(invalid_utf8);
  // A genuine independently produced other context cannot replace the saved
  // context, even when its own admission is numerically valid at sample640.
  auto wrong_context = clone_json(request.get());
  auto *other = packet::field(fixture, "other_valid_context");
  auto *other_admission = packet::field(other, "native_admission");
  wire::put(wrong_context.get(), "receiving_admission",
            clone_json(other_admission).release());
  wire::put(wrong_context.get(), "current_receiving_admission",
            clone_json(other_admission).release());
  wire::put(wrong_context.get(), "current_receiving",
            clone_json(other).release());
  auto wrong_reply = control.execute(wrong_context.get());
  require(
      !packet::boolean(packet::field(wrong_reply.get(), "accepted")) &&
          same_json(clone_json(before.get()), checkpoint(wrong_reply.get())),
      "actual Control other-context refusal mutated retained state");
  // Mutating the original caller's prepare tree cannot rewrite the resident
  // immutable full source qualification or qualify a fresh source packet.
  wire::text(packet::field(prepare.get(), "packet"), "callback_contract",
             "forged:source-contract");
  auto changed_source = clone_json(request.get());
  wire::put(changed_source.get(), "current_source_packet",
            clone_json(packet::field(prepare.get(), "packet")).release());
  bool refused_source = false;
  try {
    (void)control.execute(changed_source.get());
  } catch (const std::invalid_argument &) {
    refused_source = true;
  }
  require(refused_source &&
              same_json(clone_json(before.get()), checkpoint(current.get())),
          "caller prepare mutation changed resident source admission");
  auto reply = control.execute(request.get());
  require(packet::boolean(packet::field(reply.get(), "accepted")),
          "actual Control receiving continuation refused");
  auto readmission_reply = clone_json(reply.get());
  auto *evidence =
      packet::field(packet::field(readmission_reply.get(), "payload"),
                    "receiving_readmission");
  // A pulse materializes the unread callback application in serial input
  // custody before draining it. Preserve the original PressAdmitted entry AND
  // require its actual Applied entry; expecting one row discards real history.
  auto expected_applications = clone_json(packet::field(
      packet::field(packet::field(saved_wire.get(), "native_pair"), "audio"),
      "applications"));
  auto expected_journal = wire::array();
  for (auto i = saved->bindings.read; i < saved->bindings.write; ++i)
    wire::append(expected_journal.get(),
                 management_transport::history(saved->bindings.history[i % 256])
                     .release());
  require(saved->bindings.write == saved->bindings.read + 1 &&
              saved->bindings.last_ordinal <
                  std::numeric_limits<std::uint64_t>::max() &&
              saved->bindings.history[saved->bindings.read % 256].change ==
                  InputBindingChange::PressAdmitted,
          "actual Control source needs its unread original press admission");
  auto expected_applied = saved->bindings.history[saved->bindings.read % 256];
  expected_applied.ordinal = saved->bindings.last_ordinal + 1;
  expected_applied.native_sequence = attack.sequence;
  expected_applied.change = InputBindingChange::Applied;
  expected_applied.operation = Kind::NoteOn;
  wire::append(expected_journal.get(),
               management_transport::history(expected_applied).release());
  const bool original_text_preserved =
      management_transport::checkpoint_text(
          packet::field(evidence, "original_checkpoint_wire")) == original_text;
  const bool receiving_preserved =
      json_object_equal(packet::field(evidence, "current_receiving"), fresh);
  const bool source_preserved = json_object_equal(
      packet::field(evidence, "current_source_packet"), packet_value);
  const bool applications_preserved =
      json_object_equal(packet::field(reply.get(), "applications"),
                        packet::field(expected_applications.get(), "entries"));
  const bool journal_preserved = json_object_equal(
      packet::field(reply.get(), "input_history"), expected_journal.get());
  if (!(original_text_preserved && receiving_preserved && source_preserved &&
        applications_preserved && journal_preserved)) {
    // Preserve the complete genuine emitted pulse before this detecting guard
    // refuses, so the original child stdout artifact names the exact operand.
    auto frontier = wire::object();
    wire::text(frontier.get(), "schema",
               "ql.receiving-control-custody-frontier/v1");
    wire::put(frontier.get(), "native_pulse",
              clone_json(reply.get()).release());
    wire::put(frontier.get(), "expected_applications",
              expected_applications.release());
    wire::put(frontier.get(), "expected_input_history",
              expected_journal.release());
    wire::flag(frontier.get(), "original_checkpoint_text_preserved",
               original_text_preserved);
    wire::flag(frontier.get(), "current_receiving_preserved",
               receiving_preserved);
    wire::flag(frontier.get(), "current_source_preserved", source_preserved);
    wire::flag(frontier.get(), "original_applications_preserved",
               applications_preserved);
    wire::flag(frontier.get(), "original_and_applied_journal_preserved",
               journal_preserved);
    std::cout << json_object_to_json_string_ext(frontier.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  }
  require(
      original_text_preserved && receiving_preserved && source_preserved &&
          applications_preserved && journal_preserved,
      "actual Control lost original source/observer application/input custody");
  auto operative_json = ql::physical_wire::parse_native(
      management_transport::checkpoint_text(
          packet::field(evidence, "operative_checkpoint_wire"))
          .c_str());
  auto after_json = ql::physical_wire::parse_native(
      management_transport::checkpoint_text(
          packet::field(evidence, "after_checkpoint_wire"))
          .c_str());
  auto operative = management_checkpoint_transport::read_checkpoint_wire(
      operative_json.get());
  auto after =
      management_checkpoint_transport::read_checkpoint_wire(after_json.get());
  require(
      operative->transport_epoch == saved->transport_epoch &&
          after->transport_epoch == 2 &&
          operative->bindings.read == saved->bindings.read &&
          after->bindings.read == after->bindings.write &&
          operative->native_pair.audio.applications.read ==
              saved->native_pair.audio.applications.read &&
          after->native_pair.audio.applications.read ==
              after->native_pair.audio.applications.write &&
          operative->native_pair.audio.heap_size ==
              saved->native_pair.audio.heap_size &&
          same_json(
              ql::physical_wire::checkpoint_wire(saved->native_pair.physical),
              ql::physical_wire::checkpoint_wire(after->native_pair.physical)),
      "actual Control operative/after transition lost queues/qv or fabricated "
      "observer drain");
  for (const auto &chunk : expected) {
    auto *reading = packet::field(reply.get(), "reading");
    offline_transport::Scope scope;
    scope.performance_digest =
        packet::string(packet::field(fixture, "scope_performance_digest"));
    scope.native.session = saved->session;
    scope.native.scene = reference("controlled:receiving-restore/source-scene");
    scope.native.performance_revision =
        reference("controlled:receiving-restore/source-revision");
    scope.native.basis_seal =
        reference("controlled:receiving-restore/source-basis");
    scope.native.event_prefix_seal =
        reference("controlled:receiving-restore/source-prefix");
    scope.native.checkpoint_ref =
        reference("native:receiving-restore/control-checkpoint");
    scope.native.expected_source =
        saved->native_pair.audio.determination.identity;
    scope.native.expected_body_revision =
        saved->native_pair.audio.determination.body_revision;
    scope.native.expected_cursor =
        wire::decimal(packet::field(reading, "samples_elapsed"));
    scope.native.expected_accepted_sequence =
        wire::decimal(packet::field(reading, "accepted_sequence"));
    auto render_request = base("offline-render", reply.get());
    wire::put(render_request.get(), "scope",
              offline_transport::scope_wire(scope).release());
    wire::put(render_request.get(), "frames", json_object_new_uint64(512));
    reply = control.execute(render_request.get());
    require(packet::boolean(packet::field(reply.get(), "accepted")),
            "actual continued Control callback refused");
    auto *pcm = packet::field(
        packet::field(packet::field(reply.get(), "payload"), "chunk"),
        "interleaved_f32");
    require(json_object_array_length(pcm) == chunk.size(),
            "actual Control PCM chunk length differs");
    for (std::size_t i = 0; i < chunk.size(); ++i)
      require(packet::number(json_object_array_get_idx(pcm, i)) ==
                  double(chunk[i]),
              "actual Control continuation changed native PCM bit value");
  }
  auto final = management_checkpoint_transport::read_checkpoint_wire(
      checkpoint(reply.get()).get());
  require(
      same_json(ql::physical_wire::checkpoint_wire(final->native_pair.physical),
                ql::physical_wire::checkpoint_wire(
                    final_original->native_pair.physical)),
      "actual continued Control physical q/v differs");
  auto result = wire::object();
  wire::flag(result.get(), "actual_control_exact_continuation", true);
  wire::flag(result.get(), "actual_control_other_context_refused", true);
  wire::flag(result.get(), "actual_control_caller_source_mutation_refused",
             true);
  wire::flag(result.get(), "actual_control_observer_custody_preserved", true);
  wire::flag(result.get(), "actual_control_full_checkpoint_text", true);
  wire::flag(result.get(), "actual_control_c_emitted_original_verified",
             c_original_verified);
  wire::flag(result.get(), "actual_control_malformed_checkpoint_text_refused",
             true);
  // Original complete native pulse/evidence retained before later callbacks.
  wire::put(result.get(), "readmission_reply", readmission_reply.release());
  return result;
}
static Json trial(J *fixture) {
  auto original = owner(fixture);
  auto initial =
      receiving(*original, fixture, packet::field(fixture, "initial"), 0);
  if (initial.seed->program_count) {
    initial.seed->programs[0].enabled = false;
    initial.seed->programs[0].target_gain = .42;
    initial.seed->programs[0].effective_gain = .11;
    initial.seed->programs[1].target_gain = .63;
    initial.seed->programs[1].effective_gain = .29;
  }
  require(original->admit_distinct_receiving(initial.port, *initial.seed, 0)
                  .result == Result::Accepted,
          "actual initial receiving refused");
  Operation attack{};
  attack.kind = Kind::NoteOn;
  attack.identity = original->native().determination.identity;
  attack.sequence = 1;
  attack.note = original->native().notes.front();
  attack.value = .7;
  require(original->enqueue_score_input(
              attack, reference("native:receiving-restore/original-input")) ==
              Result::Accepted,
          "real native note admission refused");
  Operation future{};
  future.kind = Kind::Parameter;
  future.identity = attack.identity;
  future.sequence = 2;
  future.sample = 48000;
  future.parameter = Parameter::MasterLinear;
  future.value = .8;
  require(original->enqueue_score_input(future) == Result::Accepted,
          "real future reservation refused");
  render(*original, 128);
  render(*original, 512);
  auto saved = original->stopped_checkpoint();
  const auto original_wire =
      management_checkpoint_transport::checkpoint_wire(*saved);
  require(saved->native_pair.audio.cursor == 640 &&
              saved->native_pair.audio.accepted_sequence == 2 &&
              saved->native_pair.audio.applied_application_ordinal == 1,
          "real saved source cursor/queue/application differs");
  auto resumed = owner(fixture);
  auto fresh = receiving(*resumed, fixture,
                         packet::field(fixture, "saved_current"), 640);
  auto wrong = receiving(*resumed, fixture,
                         packet::field(fixture, "other_valid_context"), 640);
  auto before = resumed->stopped_checkpoint();
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    auto rejected =
        std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
    require(!resumed->preflight_stopped_receiving_restore(
                *saved, wrong.port, *wrong.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *rejected),
            "other valid native context replaced saved original receiving");
  }
  auto after_wrong = resumed->stopped_checkpoint();
  require(
      same_json(management_checkpoint_transport::checkpoint_wire(*before),
                management_checkpoint_transport::checkpoint_wire(*after_wrong)),
      "other valid context refusal changed native state");
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    auto corrupted = std::make_unique<ManagementCheckpoint>(*saved);
    // A genuine differently produced receiving definition is a NONderivative
    // identity. Admission seals cannot launder it into the saved source.
    // World has zero routes, so this trial discriminates that path too.
    const auto other_definition = wrong.binding->manifest().definition_ref;
    require(other_definition !=
                saved->native_pair.audio.route_programs.manifest.definition_ref,
            "true distinct native definition missing from refusal trial");
    corrupted->native_pair.audio.route_programs.manifest.definition_ref =
        other_definition;
    auto rejected =
        std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
    require(!resumed->preflight_stopped_receiving_restore(
                *corrupted, fresh.port, *fresh.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *rejected),
            "derivative route requalification accepted a changed native "
            "definition");
    if (saved->native_pair.audio.route_programs.program_count) {
      *corrupted = *saved;
      ++corrupted->native_pair.audio.route_programs.programs[0]
            .handle.share_denominator;
      ++corrupted->native_pair.audio.route_programs.manifest.programs[0]
            .share_denominator;
      require(!resumed->preflight_stopped_receiving_restore(
                  *corrupted, fresh.port, *fresh.seed, resumed->native().notes,
                  catalog(fixture), guard, 0, *rejected),
              "derivative route requalification altered original all-ten "
              "denominator");
    }
  }
  auto after_nonderivative = resumed->stopped_checkpoint();
  require(same_json(management_checkpoint_transport::checkpoint_wire(*before),
                    management_checkpoint_transport::checkpoint_wire(
                        *after_nonderivative)),
          "nonderivative manifest/handle refusal changed native state");
  auto stale_guard =
      std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    require(resumed->preflight_stopped_receiving_restore(
                *saved, fresh.port, *fresh.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *stale_guard),
            "actual stopped restore preflight refused");
  }
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    require(!resumed->receiving_restore_current(*stale_guard, guard),
            "reacquired unchanged guard reused prior private restore token");
  }
  auto stale_catalog =
      std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
  {
    auto guard = resumed->native().engine->acquire_stopped_custody();
    require(resumed->preflight_stopped_receiving_restore(
                *saved, fresh.port, *fresh.seed, resumed->native().notes,
                catalog(fixture), guard, 0, *stale_catalog),
            "real catalog preflight refused");
    auto changed = catalog(fixture);
    changed.front().label += " changed";
    resumed->admit_catalog(std::move(changed));
    require(!resumed->receiving_restore_current(*stale_catalog, guard),
            "same-source catalog mutation left private restore token current");
  }
  resumed->admit_catalog(catalog(fixture));
  std::unique_ptr<PerformanceManagement::PreparedReceivingRestore> candidate;
  TransportAcknowledgement ack{};
  const auto numerical_owner = resumed->native().body.get();
  {
    auto before_physical_refusal = resumed->stopped_checkpoint();
    auto corrupted = std::make_unique<ManagementCheckpoint>(*saved);
    corrupted->native_pair.physical.eigenbasis_identity += "/different";
    require(
        !restore_current_receiving_checkpoint(
            *resumed, *corrupted, fresh.port, *fresh.seed,
            resumed->native().notes, catalog(fixture), 0,
            reference("native:receiving-restore/transaction"),
            reference("native:receiving-restore/saved-checkpoint"), candidate,
            ack),
        "actual paired owner accepted a different saved physical eigenbasis");
    require(
        !candidate && ack.epoch == 0,
        "refused physical checkpoint published a candidate or transport ACK");
    auto after_physical_refusal = resumed->stopped_checkpoint();
    require(same_json(management_checkpoint_transport::checkpoint_wire(
                          *before_physical_refusal),
                      management_checkpoint_transport::checkpoint_wire(
                          *after_physical_refusal)),
            "physical preflight refusal changed actual A/P/Management state");
  }
  require(restore_current_receiving_checkpoint(
              *resumed, *saved, fresh.port, *fresh.seed,
              resumed->native().notes, catalog(fixture), 0,
              reference("native:receiving-restore/transaction"),
              reference("native:receiving-restore/saved-checkpoint"), candidate,
              ack),
          "actual paired owner fresh receiving restore refused");
  require(resumed->native().body.get() == numerical_owner &&
              resumed->native().engine->owns_physical_owner(numerical_owner) &&
              ack.previous_epoch == 1 && ack.epoch == 2 &&
              ack.previous_cursor == 0 && ack.target_sample == 640 &&
              ack.accepted_sequence == 2,
          "native stopped restore changed owner or inferred epoch/cursor");
  auto restored = resumed->stopped_checkpoint();
  require(
      same_json(
          ql::physical_wire::checkpoint_wire(saved->native_pair.physical),
          ql::physical_wire::checkpoint_wire(restored->native_pair.physical)),
      "saved qv/body/source changed during receiving requalification");
  require(
      same_json(management_checkpoint_transport::checkpoint_wire(*saved),
                management_checkpoint_transport::checkpoint_wire(
                    candidate->original_checkpoint())) &&
          json_object_equal(
              original_wire.get(),
              management_checkpoint_transport::checkpoint_wire(*saved).get()),
      "original saved source/checkpoint bytes changed");
  // Compare complete dynamic audio/queues/history after replacing ONLY the
  // declared derivative manifest/handles in a separate test comparison copy.
  auto expected = std::make_unique<ManagementCheckpoint>(*saved);
  expected->transport_epoch = restored->transport_epoch;
  auto &expected_programmes = expected->native_pair.audio.route_programs;
  const auto &actual_programmes = restored->native_pair.audio.route_programs;
  expected_programmes.manifest.admitted_cursor =
      actual_programmes.manifest.admitted_cursor;
  expected_programmes.manifest.source_basis_seal =
      actual_programmes.manifest.source_basis_seal;
  require(saved->native_pair.audio.route_programs.manifest.admitted_cursor ==
                  0 &&
              actual_programmes.manifest.admitted_cursor == 640,
          "native saved-to-operative admission cursor relation absent");
  for (std::size_t i = 0; i < expected_programmes.program_count; ++i) {
    const auto &actual_handle = actual_programmes.programs[i].handle;
    expected_programmes.programs[i].handle.preparation_seal =
        actual_handle.preparation_seal;
    expected_programmes.programs[i].handle.program_seal =
        actual_handle.program_seal;
    expected_programmes.manifest.programs[i].preparation_seal =
        actual_handle.preparation_seal;
    expected_programmes.manifest.programs[i].program_seal =
        actual_handle.program_seal;
  }
  require(
      same_json(management_checkpoint_transport::checkpoint_wire(*expected),
                management_checkpoint_transport::checkpoint_wire(*restored)),
      "saved queues/touch/input/journal/phase/gains were reset or redated");
  for (unsigned block = 0; block < 2; ++block)
    require(render(*original, 512) == render(*resumed, 512),
            "fresh receiving restore lost exact native PCM continuation");
  auto final_original = original->stopped_checkpoint();
  auto final_resumed = resumed->stopped_checkpoint();
  require(same_json(ql::physical_wire::checkpoint_wire(
                        final_original->native_pair.physical),
                    ql::physical_wire::checkpoint_wire(
                        final_resumed->native_pair.physical)),
          "fresh restore changed same causal modal q/v");
  auto out = wire::object();
  for (const char *key :
       {"same_body", "original_checkpoint_unchanged", "exact_pcm_continuation",
        "exact_modal_qv", "original_queues_inputs_preserved",
        "nondefault_programmes_preserved", "valid_other_context_refused",
        "guard_reacquisition_refused", "catalog_replacement_refused",
        "physical_eigenbasis_refused"})
    wire::flag(out.get(), key, true);
  wire::u64(out.get(), "saved_cursor", 640);
  wire::u64(out.get(), "resumed_cursor", 1664);
  wire::put(out.get(), "native_routes",
            json_object_new_uint64(fresh.seed->program_count));
  wire::u64(out.get(), "saved_route_admitted_cursor",
            saved->native_pair.audio.route_programs.manifest.admitted_cursor);
  wire::u64(
      out.get(), "fresh_route_admitted_cursor",
      restored->native_pair.audio.route_programs.manifest.admitted_cursor);
  wire::u64(out.get(), "engine_bytes", sizeof(Engine));
  wire::u64(out.get(), "checkpoint_bytes", sizeof(Engine::Checkpoint));
  wire::u64(out.get(), "manager_checkpoint_bytes",
            sizeof(ManagementCheckpoint));
  wire::u64(out.get(), "engine_token_bytes",
            sizeof(Engine::PreparedReceivingRestore));
  wire::u64(out.get(), "manager_token_bytes",
            sizeof(PerformanceManagement::PreparedReceivingRestore));
  wire::put(out.get(), "control", control_trial(fixture).release());
  return out;
}
int main() {
  try {
    std::string input;
    char ch;
    while (std::cin.get(ch)) {
      input.push_back(ch);
      require(input.size() < 16 * 1024 * 1024, "native fixture exceeds bound");
    }
    auto root = ql::physical_wire::parse_native(input.c_str());
    require(packet::string(packet::field(root.get(), "schema")) ==
                "ql.receiving-restore-native-fixture/v1",
            "genuine native source fixture required");
    auto out = wire::object();
    wire::text(out.get(), "schema", "ql.receiving-restore-native-receipt/v1");
    J *context_kind = nullptr;
    if (json_object_object_get_ex(root.get(), "context_kind", &context_kind)) {
      // Closed test carrier only. Full actual native source/context validators
      // still execute inside trial; selecting a name grants no native work.
      ql::physical_wire::keys(root.get(),
                              {"schema", "context_kind", "context"});
      const auto kind = packet::string(context_kind);
      require(kind == "world" || kind == "personal" || kind == "shared",
              "unknown named native receiving context carrier");
      auto *fixture = packet::field(root.get(), "context");
      auto *actual_context = packet::field(
          packet::field(packet::field(packet::field(fixture, "initial"),
                                      "native_admission"),
                        "operation"),
          "context");
      require(packet::string(packet::field(actual_context, "kind")) == kind,
              "named context carrier differs from actual native producer");
      wire::put(out.get(), kind.c_str(), trial(fixture).release());
    } else {
      // Preserve the original complete-three fixture when it fits the same cap.
      for (const char *kind : {"world", "personal", "shared"})
        wire::put(out.get(), kind,
                  trial(packet::field(root.get(), kind)).release());
    }
    std::cout << json_object_to_json_string_ext(out.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &failure) {
    std::cerr << failure.what() << '\n';
    return 1;
  }
}
