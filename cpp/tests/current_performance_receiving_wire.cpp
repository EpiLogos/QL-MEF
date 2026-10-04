// Actual Rust source/receiving owners -> existing worker Control -> sole A/P.
// In-memory native proof only: installed UI/device/C export remain separate.
#include <cmath>
#include <iostream>
#include <ql/performance_management_wire.hpp>
#include <vector>
using namespace ql::performance;
namespace wire = checkpoint_transport;
using Json = wire::Json;
using J = json_object;
using ql::require;
static Json copy(J *value) {
  return ql::physical_wire::parse_native(
      json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN));
}
struct Resident {
  management_transport::Control control;
  J *fixture;
  Json last{nullptr, json_object_put};
  explicit Resident(J *source) : fixture(source) {
    auto request = wire::object();
    auto *p = packet::field(source, "native_preparation");
    auto *d = packet::field(p, "determination");
    wire::text(request.get(), "schema", "ql.performance-control/v1");
    wire::text(request.get(), "operation", "prepare");
    wire::text(request.get(), "session_ref",
               "native:current-receiving/session");
    for (const char *key : {"packet", "current_source_packet"})
      wire::put(request.get(), key, json_object_get(p));
    wire::put(request.get(), "actual_native_basis",
              json_object_get(packet::field(source, "native_basis")));
    wire::flag(request.get(), "m1_pratibimba",
               packet::integer(packet::field(d, "m1_face")) == 1);
    auto *physical = packet::field(p, "physical_body");
    auto *coordinate = packet::field(physical, "source_coordinate");
    wire::flag(request.get(), "physical_pratibimba",
               packet::string(packet::field(coordinate, "face")) ==
                   "pratibimba");
    auto standing = wire::object();
    wire::text(standing.get(), "kind", "sourceForm");
    wire::text(
        standing.get(), "recipe_ref",
        packet::string(packet::field(
            packet::field(packet::field(p, "source_form_recipe"), "provenance"),
            "reference")));
    wire::u64(standing.get(), "validated_m3_generation",
              packet::integer(packet::field(physical, "source_generation")));
    wire::put(request.get(), "body_source", standing.release());
    auto *receiving = packet::field(packet::field(source, "current_receiving"),
                                    "native_admission");
    wire::put(request.get(), "receiving_admission", json_object_get(receiving));
    wire::put(request.get(), "current_receiving_admission",
              json_object_get(receiving));
    last = control.execute(request.get());
    require(packet::boolean(packet::field(last.get(), "accepted")),
            "actual current source prepare refused");
    auto catalog = command("catalog");
    wire::put(catalog.get(), "cells",
              json_object_get(packet::field(source, "native_catalog")));
    wire::put(catalog.get(), "transpose", json_object_new_uint64(0));
    apply(catalog.get());
    auto *reading = packet::field(last.get(), "reading");
    packet::array(
        packet::field(packet::field(reading, "physical"), "positions_metres"),
        12);
    auto *keys = packet::field(reading, "keys");
    packet::array(keys, 36);
    unsigned available = 0;
    for (std::size_t i = 0; i < 36; ++i)
      available += packet::boolean(
          packet::field(json_object_array_get_idx(keys, i), "available"));
    require(available == 21,
            "actual sparse seven/five source availability lost");
  }
  Json command(const char *operation) const {
    auto out = wire::object();
    auto *r = packet::field(last.get(), "reading");
    auto *d = packet::field(packet::field(fixture, "native_preparation"),
                            "determination");
    wire::text(out.get(), "schema", "ql.performance-control/v1");
    wire::text(out.get(), "operation", operation);
    wire::text(out.get(), "session_ref", "native:current-receiving/session");
    wire::put(out.get(), "expected_transport_epoch",
              json_object_get(packet::field(r, "transport_epoch")));
    wire::put(out.get(), "expected_source",
              json_object_get(packet::field(d, "identity")));
    wire::put(out.get(), "expected_body_revision",
              json_object_get(packet::field(d, "body_revision")));
    return out;
  }
  void apply(J *request) {
    last = control.execute(request);
    if (!packet::boolean(packet::field(last.get(), "accepted"))) {
      // Controlled native floor diagnostics only: no original identity/natal,
      // protected occasion, complete source assets or PCM is printed here.
      auto diagnostic = wire::object();
      wire::text(diagnostic.get(), "schema",
                 "ql.current-receiving-test-refusal/v1");
      wire::text(diagnostic.get(), "operation",
                 packet::string(packet::field(request, "operation")));
      wire::put(diagnostic.get(), "reason",
                json_object_get(packet::field(last.get(), "reason")));
      auto *reading = packet::field(last.get(), "reading");
      for (const char *key :
           {"samples_elapsed", "accepted_sequence", "transport_epoch",
            "active_voices", "active_touches", "recording_available"}) {
        J *value = nullptr;
        if (json_object_object_get_ex(reading, key, &value))
          wire::put(diagnostic.get(), key, json_object_get(value));
      }
      auto *payload = packet::field(last.get(), "payload");
      J *chunk = nullptr;
      if (json_object_object_get_ex(payload, "chunk", &chunk)) {
        for (const char *key :
             {"result", "reason", "start_sample", "committed_cursor",
              "state_committed", "capture_complete"}) {
          J *value = nullptr;
          if (json_object_object_get_ex(chunk, key, &value))
            wire::put(diagnostic.get(), key, json_object_get(value));
        }
      }
      wire::put(diagnostic.get(), "recording",
                json_object_get(packet::field(last.get(), "recording")));
      std::cerr << json_object_to_json_string_ext(diagnostic.get(),
                                                  JSON_C_TO_STRING_PLAIN)
                << '\n';
    }
    require(packet::boolean(packet::field(last.get(), "accepted")),
            "actual native worker operation refused");
  }
  void attack() {
    auto *p = packet::field(fixture, "native_preparation");
    Operation op{};
    op.kind = Kind::NoteOn;
    op.identity = packet::identity(
        packet::field(packet::field(p, "determination"), "identity"));
    op.note =
        packet::note(json_object_array_get_idx(packet::field(p, "notes"), 0));
    op.sequence = 1;
    op.sample = 0;
    op.value = .7;
    auto request = command("score");
    wire::put(request.get(), "event", wire::operation(op).release());
    wire::text(request.get(), "input_ref", "native:original-janko/input");
    apply(request.get());
    auto *receipt =
        packet::field(packet::field(last.get(), "payload"), "score_admission");
    require(packet::string(packet::field(receipt, "schema")) ==
                    "ql.native-score-admission/v1" &&
                packet::boolean(packet::field(receipt, "queued")),
            "actual score admission receipt missing");
    auto admitted = wire::read_operation(packet::field(receipt, "event"));
    require(admitted.sequence == 1 && admitted.has_requested_sample &&
                admitted.requested_sample == 0 && admitted.sample == 0 &&
                admitted.note.touch == op.note.touch &&
                admitted.note.identity == op.note.identity,
            "actual source score receipt lost original/queued note");
    require(wire::decimal(packet::field(receipt, "transport_epoch")) == 1 &&
                wire::decimal(packet::field(receipt, "queue_cursor")) == 0 &&
                packet::string(packet::field(receipt, "input_ref")) ==
                    "native:original-janko/input",
            "score admission detached from actual manager input/cursor");
  }
  Json checkpoint() {
    auto request = command("checkpoint");
    apply(request.get());
    return copy(
        packet::field(packet::field(last.get(), "payload"), "checkpoint"));
  }
  void restore(J *saved) {
    auto request = command("restore");
    wire::put(request.get(), "checkpoint", json_object_get(saved));
    wire::u64(request.get(), "expected_cursor", 0);
    wire::text(request.get(), "transaction_ref",
               "native:current-receiving/reopen");
    wire::text(request.get(), "checkpoint_ref",
               "native:current-receiving/checkpoint");
    apply(request.get());
  }
  std::vector<double> render(unsigned frames) {
    require(frames > 0 && frames <= 512, "finite callback block required");
    auto request = command("offline-render");
    auto *r = packet::field(last.get(), "reading");
    auto scope = wire::object();
    wire::text(scope.get(), "schema", "ql.native-offline-render-scope/v1");
    wire::text(scope.get(), "session_ref", "native:current-receiving/session");
    wire::text(scope.get(), "scene_ref",
               "native:current-receiving/declared-numerical-probe");
    wire::text(scope.get(), "performance_revision", "1");
    // Exact content identity of the actual producer's controlled probe inputs;
    // this is deliberately NOT qualification of a C-authored Act/file export.
    wire::text(scope.get(), "performance_digest",
               packet::string(packet::field(fixture, "probe_scope_digest")));
    wire::text(scope.get(), "basis_seal",
               "native:current-receiving/actual-basis");
    wire::text(scope.get(), "event_prefix_seal",
               "native:current-receiving/actual-probe-prefix");
    wire::text(scope.get(), "checkpoint_ref",
               "native:current-receiving/checkpoint");
    auto *d = packet::field(packet::field(fixture, "native_preparation"),
                            "determination");
    wire::put(scope.get(), "expected_source",
              json_object_get(packet::field(d, "identity")));
    wire::put(scope.get(), "expected_body_revision",
              json_object_get(packet::field(d, "body_revision")));
    wire::put(scope.get(), "expected_cursor",
              json_object_get(packet::field(r, "samples_elapsed")));
    wire::put(scope.get(), "expected_accepted_sequence",
              json_object_get(packet::field(r, "accepted_sequence")));
    wire::put(request.get(), "scope", scope.release());
    wire::put(request.get(), "frames", json_object_new_uint64(frames));
    apply(request.get());
    auto *chunk = packet::field(packet::field(last.get(), "payload"), "chunk");
    require(packet::string(packet::field(chunk, "result")) == "accepted",
            "actual captured callback output refused");
    auto *same_callback = packet::field(last.get(), "reading");
    require(json_object_equal(packet::field(same_callback, "samples_elapsed"),
                              packet::field(chunk, "committed_cursor")) &&
                json_object_equal(
                    packet::field(packet::field(same_callback, "physical"),
                                  "samples_elapsed"),
                    packet::field(chunk, "committed_cursor")),
            "offline management lost actual callback audio/body readback");
    auto *pcm = packet::field(chunk, "interleaved_f32");
    packet::array(pcm, frames);
    std::vector<double> out;
    for (unsigned i = 0; i < frames; ++i)
      out.push_back(packet::number(json_object_array_get_idx(pcm, i)));
    return out;
  }
};
// The real native Pluto driver is 140Hz and the admitted M1 Pratibimba
// carrier starts at 30 degrees. At sample400 its true sinusoid is exactly1.
// Recursive double quadrature must remain in the mathematical unit range,
// including when saved at that peak, without relaxing any P Newton bound.
static Json actual_route_unit_peak(J *source) {
  auto original = std::make_unique<Resident>(source);
  original->attack();
  original->render(400);
  auto saved = original->checkpoint();
  auto *audio =
      packet::field(packet::field(saved.get(), "native_pair"), "audio");
  require(wire::decimal(packet::field(audio, "cursor")) == 400 &&
              packet::boolean(packet::field(audio, "has_route_programs")),
          "actual unit-peak checkpoint lost current native programmes");
  auto *set = packet::field(audio, "route_programs");
  auto *programs = packet::field(set, "programs");
  packet::array(programs, 9);
  std::size_t pluto = 9;
  for (std::size_t i = 0; i < 9; ++i) {
    auto *program = json_object_array_get_idx(programs, i);
    const auto sine = packet::number(packet::field(program, "sine"));
    const auto cosine = packet::number(packet::field(program, "cosine"));
    require(std::abs(sine) <= 1 && std::abs(cosine) <= 1 &&
                std::abs(sine * sine + cosine * cosine - 1) <= 1e-10,
            "actual evolved checkpoint quadrature exceeds native unit range");
    auto *handle = packet::field(program, "handle");
    if (packet::number(packet::field(handle, "source_hertz")) == 140)
      pluto = i;
  }
  require(pluto < 9 && packet::number(packet::field(
                           json_object_array_get_idx(programs, pluto),
                           "sine")) > .999999999999,
          "actual native 140Hz source did not reach its sample400 unit peak");
  auto resumed = std::make_unique<Resident>(source);
  resumed->restore(saved.get());
  require(original->render(1) == resumed->render(1),
          "actual unit-peak Newton sample or exact reopen failed");
  for (unsigned block = 0; block < 2; ++block)
    require(original->render(512) == resumed->render(512),
            "actual nine programmes lost exact continuation across unit peak");
  auto continued = original->checkpoint();
  auto continued_reopened = resumed->checkpoint();
  auto *continued_pair = packet::field(continued.get(), "native_pair");
  auto *reopened_pair = packet::field(continued_reopened.get(), "native_pair");
  require(
      json_object_equal(packet::field(continued_pair, "physical"),
                        packet::field(reopened_pair, "physical")) &&
          json_object_equal(
              packet::field(packet::field(continued_pair, "audio"),
                            "route_programs"),
              packet::field(packet::field(reopened_pair, "audio"),
                            "route_programs")),
      "unit-peak continuation lost actual modal q/v or programme phase/gain");
  auto corrupt = copy(saved.get());
  auto *corrupt_programs = packet::field(
      packet::field(
          packet::field(packet::field(corrupt.get(), "native_pair"), "audio"),
          "route_programs"),
      "programs");
  wire::real(json_object_array_get_idx(corrupt_programs, pluto), "sine",
             std::nextafter(1.0, 2.0));
  auto refused = std::make_unique<Resident>(source);
  auto before = refused->checkpoint();
  bool rejected = false;
  try {
    refused->restore(corrupt.get());
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  require(rejected, "corrupted super-unit checkpoint was silently accepted");
  auto after = refused->checkpoint();
  require(json_object_equal(before.get(), after.get()),
          "super-unit checkpoint refusal changed original native state");
  auto receipt = wire::object();
  wire::u64(receipt.get(), "peak_sample", 400);
  wire::u64(receipt.get(), "continued_frames", 1025);
  wire::flag(receipt.get(), "all_nine_unit_components", true);
  wire::flag(receipt.get(), "exact_reopen", true);
  wire::flag(receipt.get(), "super_unit_checkpoint_refused_atomically", true);
  wire::put(receipt.get(), "native_programmes_at_peak", json_object_get(set));
  wire::put(receipt.get(), "native_physical_at_peak",
            json_object_get(packet::field(
                packet::field(saved.get(), "native_pair"), "physical")));
  wire::put(receipt.get(), "native_physical_after_continuation",
            json_object_get(packet::field(continued_pair, "physical")));
  return receipt;
}
static Json actual_native_timing(J *world) {
  auto resident = std::make_unique<Resident>(world);
  const auto timing = [&](const char *moment, std::uint64_t ordinal) {
    auto request = resident->command("timing");
    wire::text(request.get(), "moment", moment);
    wire::u64(request.get(), "ordinal", ordinal);
    resident->apply(request.get());
    return copy(packet::field(packet::field(resident->last.get(), "payload"),
                              "timing_fact"));
  };
  auto initial = timing("boundary", 0);
  auto *binding = packet::field(initial.get(), "binding");
  require(
      packet::string(packet::field(binding, "owner_ref")) ==
              "native:current-receiving/session" &&
          packet::string(packet::field(binding, "domain")) ==
              "native_samples" &&
          packet::string(packet::field(binding, "epoch_ref")) ==
              "ql:performance/transport-epoch/1" &&
          wire::decimal(packet::field(initial.get(), "committed_cursor")) ==
              0 &&
          !packet::boolean(packet::field(initial.get(), "queued")),
      "native time domain inferred epoch/clock or fabricated queue admission");
  resident->attack();
  auto queued = timing("score", 1);
  require(wire::decimal(packet::field(queued.get(), "requested_cursor")) == 0 &&
              wire::decimal(packet::field(queued.get(), "admitted_cursor")) ==
                  0 &&
              packet::boolean(packet::field(queued.get(), "queued")),
          "actual original NoteOn queue timing was lost");
  resident->render(128);
  auto committed = timing("applied", 1);
  require(
      wire::decimal(packet::field(committed.get(), "applied_cursor")) == 0 &&
          wire::decimal(packet::field(committed.get(), "committed_cursor")) ==
              128,
      "native applied date retagged to the later copied cursor");
  auto *packet_value = packet::field(world, "native_preparation");
  const auto source = packet::identity(
      packet::field(packet::field(packet_value, "determination"), "identity"));
  Operation future{};
  future.kind = Kind::Parameter;
  future.identity = source;
  future.sequence = 2;
  future.sample = 48000;
  future.parameter = Parameter::MasterLinear;
  future.value = .8;
  auto future_request = resident->command("score");
  wire::put(future_request.get(), "event", wire::operation(future).release());
  management_transport::null(future_request.get(), "input_ref");
  resident->apply(future_request.get());
  auto future_fact = timing("score", 2);
  require(wire::decimal(packet::field(future_fact.get(), "requested_cursor")) ==
                  48000 &&
              wire::decimal(
                  packet::field(future_fact.get(), "admitted_cursor")) == 48000,
          "native future reservation was redated to copied cursor");
  Operation release{};
  release.kind = Kind::NoteOff;
  release.identity = source;
  release.sequence = 3;
  release.sample = 0;
  release.touch = packet::note(json_object_array_get_idx(
                                   packet::field(packet_value, "notes"), 0))
                      .touch;
  auto release_request = resident->command("score");
  wire::put(release_request.get(), "event", wire::operation(release).release());
  wire::text(release_request.get(), "input_ref", "native:original-janko/input");
  resident->apply(release_request.get());
  auto late = timing("score", 3);
  require(wire::decimal(packet::field(late.get(), "requested_cursor")) == 0 &&
              wire::decimal(packet::field(late.get(), "admitted_cursor")) ==
                  128,
          "timing witness confused original and resolved late release");
  auto saved = resident->checkpoint();
  auto restore = resident->command("restore");
  wire::put(restore.get(), "checkpoint", json_object_get(saved.get()));
  wire::u64(restore.get(), "expected_cursor", 128);
  wire::text(restore.get(), "transaction_ref", "native:timing/restore");
  wire::text(restore.get(), "checkpoint_ref",
             "native:timing/pending-checkpoint");
  resident->apply(restore.get());
  auto restored = timing("boundary", 0);
  require(packet::string(packet::field(packet::field(restored.get(), "binding"),
                                       "epoch_ref")) ==
                  "ql:performance/transport-epoch/2" &&
              wire::decimal(
                  packet::field(restored.get(), "committed_cursor")) == 128,
          "restore timing epoch inferred from unchanged native instance");
  bool historical_queue_refused = false;
  try {
    timing("score", 3);
  } catch (const std::invalid_argument &) {
    historical_queue_refused = true;
  }
  require(historical_queue_refused,
          "imported previous-epoch queue fact became a current timing grant");
  auto historical_refusal_reply = copy(resident->last.get());
  require(
      packet::string(packet::field(historical_refusal_reply.get(), "schema")) ==
              "ql.performance-worker-reply/v1" &&
          !packet::boolean(
              packet::field(historical_refusal_reply.get(), "accepted")) &&
          !packet::string(
               packet::field(historical_refusal_reply.get(), "reason"))
               .empty() &&
          wire::decimal(packet::field(
              packet::field(historical_refusal_reply.get(), "reading"),
              "samples_elapsed")) == 128 &&
          wire::decimal(packet::field(
              packet::field(historical_refusal_reply.get(), "recording"),
              "dropped_applications")) == 0,
      "ordinary timing refusal lost native copied boundary/recording status");
  packet::array(packet::field(historical_refusal_reply.get(), "applications"),
                0);
  packet::array(packet::field(historical_refusal_reply.get(), "input_history"),
                0);
  resident->render(128);
  auto applied = timing("applied", 2);
  require(wire::decimal(packet::field(applied.get(), "requested_cursor")) ==
                  0 &&
              wire::decimal(packet::field(applied.get(), "admitted_cursor")) ==
                  128 &&
              wire::decimal(packet::field(applied.get(), "applied_cursor")) ==
                  128 &&
              wire::decimal(packet::field(applied.get(), "committed_cursor")) ==
                  256,
          "actual restored release timing or application ordinal was lost");
  bool invented_clock_refused = false;
  try {
    timing("clock", 3);
  } catch (const std::invalid_argument &) {
    invented_clock_refused = true;
  }
  require(invented_clock_refused,
          "closed native device fabricated an AUHAL timestamp mapping");
  auto unavailable_clock_reply = copy(resident->last.get());
  require(!packet::boolean(
              packet::field(unavailable_clock_reply.get(), "accepted")) &&
              packet::string(
                  packet::field(unavailable_clock_reply.get(), "schema")) ==
                  "ql.performance-worker-reply/v1" &&
              wire::decimal(packet::field(
                  packet::field(unavailable_clock_reply.get(), "reading"),
                  "samples_elapsed")) == 256,
          "unavailable AUHAL observation replaced complete Management refusal "
          "reply");
  packet::field(unavailable_clock_reply.get(), "applications");
  packet::field(unavailable_clock_reply.get(), "input_history");
  packet::field(unavailable_clock_reply.get(), "recording");
  auto receipt = wire::object();
  for (const char *key : {"original_queue_preserved", "future_queue_preserved",
                          "late_requested_admitted_applied_preserved",
                          "old_epoch_refused", "invented_device_clock_refused"})
    wire::flag(receipt.get(), key, true);
  wire::put(receipt.get(), "historical_queue_refusal_reply",
            historical_refusal_reply.release());
  wire::put(receipt.get(), "unavailable_clock_reply",
            unavailable_clock_reply.release());
  wire::text(
      receipt.get(), "callback_interleaving_gate",
      "unexecuted: real native device callback between requests must prove "
      "fresh unread applications/journal survive unavailable timing selector; "
      "offline Control always pulses, no simulated replacement");
  wire::u64(receipt.get(), "committed_cursor", 256);
  wire::text(receipt.get(), "standing",
             "actual producer-driven native Control/A/P "
             "facts only; private selected-Act E factory and running AUHAL "
             "gate remain separate");
  return receipt;
}
static Json actual_restored_unread_timing(J *world) {
  auto resident = std::make_unique<Resident>(world);
  auto *p = packet::field(world, "native_preparation");
  auto *basis = packet::field(world, "native_basis");
  const bool m1_prime = packet::integer(packet::field(
                            packet::field(p, "determination"), "m1_face")) == 1;
  const bool body_prime =
      packet::string(packet::field(
          packet::field(packet::field(p, "physical_body"), "source_coordinate"),
          "face")) == "pratibimba";
  auto native = prepare_source_performance_packet(
      json_object_to_json_string_ext(p, JSON_C_TO_STRING_PLAIN), p, basis,
      m1_prime, body_prime);
  const auto immutable = native.body->preparation();
  auto owner = std::make_unique<PerformanceManagement>(
      std::move(native), reference("native:current-receiving/session"));
  auto *admission = packet::field(packet::field(world, "current_receiving"),
                                  "native_admission");
  auto source = std::make_shared<const AdmittedNativeReceivingSource>(
      read_native_receiving_admission(admission, admission, owner->native(),
                                      basis, immutable, {}, 0));
  auto binding = std::make_shared<PhysicalRoutesPortBinding>(
      owner->native().body, source, immutable, owner->native().determination,
      basis, m1_prime, 0);
  auto programmes = std::make_unique<NativeRouteProgramSet>();
  programmes->manifest = binding->manifest();
  programmes->program_count = programmes->manifest.route_count;
  require(programmes->program_count == 0,
          "actual unread timing trial requires native neutral World");
  programmes->scalar_note_enabled = programmes->manifest.scalar_note_enabled;
  programmes->scalar_note_gain = programmes->manifest.scalar_note_gain;
  require(owner->admit_distinct_receiving(
                   physical_routes_port(physical_port(owner->native().body),
                                        binding),
                   *programmes, 0)
                  .result == Result::Accepted,
          "actual World same-body timing port refused");
  Operation attack{};
  attack.kind = Kind::NoteOn;
  attack.identity = owner->native().determination.identity;
  attack.note = owner->native().notes.front();
  attack.value = .7;
  attack.sequence = 1;
  attack.sample = 0;
  const auto input = reference("native:original-janko/input");
  require(owner->enqueue_score_input(attack, input) == Result::Accepted,
          "actual unread timing attack admission refused");
  std::array<float, 128> output{};
  require(owner->offline_advance(output.data(), 128, 0),
          "actual unread timing callback refused");
  // Deliberately no pulse: retain the REAL unread callback application and
  // admitted original input journal in the actual native stopped checkpoint.
  auto saved = owner->stopped_checkpoint();
  require(saved->native_pair.audio.applied_application_ordinal == 1 &&
              saved->native_pair.audio.applications.write -
                      saved->native_pair.audio.applications.read ==
                  1,
          "actual checkpoint did not retain original unread application");
  auto checkpoint = management_checkpoint_transport::checkpoint_wire(*saved);
  auto restore = resident->command("restore");
  wire::put(restore.get(), "checkpoint", checkpoint.release());
  wire::u64(restore.get(), "expected_cursor", 0);
  wire::text(restore.get(), "transaction_ref", "native:timing/unread-restore");
  wire::text(restore.get(), "checkpoint_ref",
             "native:timing/unread-checkpoint");
  resident->apply(restore.get());
  auto *history = packet::field(resident->last.get(), "applications");
  packet::array(history, 1);
  const auto old =
      wire::read_application(json_object_array_get_idx(history, 0));
  require(old.applied && old.sequence == 1 &&
              old.applied_application_ordinal == 1 &&
              old.requested_sample == 0 && old.applied_sample == 0 &&
              old.committed_cursor == 128,
          "restore discarded or redated original historical application");
  auto query = resident->command("timing");
  wire::text(query.get(), "moment", "applied");
  wire::u64(query.get(), "ordinal", 1);
  bool historical_refused = false;
  try {
    resident->apply(query.get());
  } catch (const std::invalid_argument &) {
    historical_refused = true;
  }
  require(
      historical_refused,
      "restored unread historical application minted new-epoch timing witness");
  Operation release{};
  release.kind = Kind::NoteOff;
  release.identity = attack.identity;
  release.touch = attack.note.touch;
  release.sequence = 2;
  release.sample = 128;
  auto command = resident->command("score");
  wire::put(command.get(), "event", wire::operation(release).release());
  wire::ref(command.get(), "input_ref", input);
  resident->apply(command.get());
  resident->render(128);
  query = resident->command("timing");
  wire::text(query.get(), "moment", "applied");
  wire::u64(query.get(), "ordinal", 2);
  resident->apply(query.get());
  auto *fact = packet::field(packet::field(resident->last.get(), "payload"),
                             "timing_fact");
  require(wire::decimal(packet::field(fact, "restored_application_floor")) ==
                  1 &&
              wire::decimal(packet::field(fact, "transport_epoch")) == 2 &&
              wire::decimal(packet::field(fact, "applied_cursor")) == 128 &&
              wire::decimal(packet::field(fact, "committed_cursor")) == 256,
          "actual new-epoch release failed original restored high-water fence");
  auto receipt = wire::object();
  wire::flag(receipt.get(), "historical_application_retained", true);
  wire::flag(receipt.get(), "historical_current_witness_refused", true);
  wire::flag(receipt.get(), "new_application_after_actual_restore_accepted",
             true);
  return receipt;
}
int main() {
  try {
    std::string bytes;
    char c;
    while (std::cin.get(c)) {
      bytes.push_back(c);
      require(bytes.size() <= 16 * 1024 * 1024,
              "native receiving fixture exceeds bound");
    }
    auto root = ql::physical_wire::parse_native(bytes.c_str());
    require(packet::string(packet::field(root.get(), "schema")) ==
                "ql.current-native-receiving-worker-fixture/v1",
            "actual native producer fixture required");
    auto results = wire::array();
    for (const char *kind : {"world", "personal", "shared"}) {
      auto *source = packet::field(root.get(), kind);
      auto baseline = std::make_unique<Resident>(source);
      auto playing = std::make_unique<Resident>(source);
      playing->attack();
      bool different = false;
      for (unsigned i = 0; i < 4; ++i)
        different = baseline->render(512) != playing->render(512) || different;
      require(
          different,
          "actual Janko producer disconnected from common native body/pickup");
      auto *r = packet::field(playing->last.get(), "reading");
      auto *actual_body = packet::field(r, "physical");
      auto *baseline_body = packet::field(
          packet::field(baseline->last.get(), "reading"), "physical");
      require(
          !json_object_equal(packet::field(actual_body, "positions_metres"),
                             packet::field(baseline_body, "positions_metres")),
          "actual note force detached from same visible body state");
      require(wire::decimal(packet::field(actual_body, "samples_elapsed")) ==
                      2048 &&
                  wire::decimal(packet::field(r, "samples_elapsed")) == 2048,
              "same callback body/audio copied cursor diverged");
      auto *role = packet::field(packet::field(r, "consumer_roles"),
                                 "personal_nine_force_routes");
      const bool personal = std::string(kind) != "world";
      require(packet::boolean(packet::field(role, "available")) == personal,
              "native receiving context availability differs");
      require(packet::integer(packet::field(role, "route_count")) ==
                  (personal ? 9 : 0),
              "native nine sources merged or fabricated");
      auto saved = playing->checkpoint();
      auto resumed = std::make_unique<Resident>(source);
      resumed->restore(saved.get());
      for (unsigned i = 0; i < 2; ++i)
        require(
            playing->render(512) == resumed->render(512),
            "same native route/note/body checkpoint lost exact continuation");
      auto receipt = wire::object();
      wire::text(receipt.get(), "context", kind);
      wire::flag(receipt.get(), "note_causal", different);
      wire::u64(receipt.get(), "route_count", personal ? 9 : 0);
      wire::u64(receipt.get(), "continued_frames", 1024);
      wire::u64(receipt.get(), "committed_cursor", 3072);
      wire::append(results.get(), receipt.release());
    }
    auto unit_peak =
        actual_route_unit_peak(packet::field(root.get(), "personal"));
    auto out = wire::object();
    wire::put(out.get(), "native_route_unit_peak", unit_peak.release());
    wire::text(out.get(), "schema",
               "ql.current-native-receiving-worker-receipt/v1");
    wire::put(out.get(), "contexts", results.release());
    wire::put(
        out.get(), "native_timing_facts",
        actual_native_timing(packet::field(root.get(), "world")).release());
    wire::put(out.get(), "restored_unread_native_timing",
              actual_restored_unread_timing(packet::field(root.get(), "world"))
                  .release());
    std::cout << json_object_to_json_string_ext(out.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
