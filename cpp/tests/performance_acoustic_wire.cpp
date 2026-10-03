// Real NativePerformanceReceivingSource acoustic compiler -> existing Control
// -> sole A/P pickup transport. Numerical component proof; private C31 live/
// retained reader installation is a separate required owner gate.
#include <iostream>
#include <ql/performance_acoustic_wire.hpp>
#include <ql/performance_management_wire.hpp>
#include <ql/performance_receiving_restore.hpp>
#include <vector>
using namespace ql::performance;
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
using ql::require;
static Json copy(J *v) {
  return ql::physical_wire::parse_native(
      json_object_to_json_string_ext(v, JSON_C_TO_STRING_PLAIN));
}
struct Resident {
  management_transport::Control control;
  J *fixture;
  Json last{nullptr, json_object_put};
  Json applications = wire::array(), input_history = wire::array();
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
    for (auto pair : {std::make_pair("applications", applications.get()),
                      std::make_pair("input_history", input_history.get())}) {
      auto *rows = packet::field(last.get(), pair.first);
      const auto count = json_object_array_length(rows);
      for (std::size_t i = 0; i < count; ++i)
        wire::append(pair.second,
                     json_object_get(json_object_array_get_idx(rows, i)));
    }
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
static void install(Resident &owner, J *acoustic, J *current = nullptr) {
  auto request = owner.command("receiving-transport-install");
  wire::put(request.get(), "prepared_acoustic", json_object_get(acoustic));
  wire::put(request.get(), "current_acoustic",
            json_object_get(current ? current : acoustic));
  wire::u64(request.get(), "expected_sample", 0);
  owner.apply(request.get());
  auto *reading = packet::field(owner.last.get(), "reading");
  auto *rx = packet::field(reading, "receiving_transport");
  require(wire::decimal(packet::field(rx, "samples_elapsed")) == 0 &&
              wire::decimal(packet::field(packet::field(rx, "manifest"),
                                          "history_origin_sample")) == 0,
          "actual installed receiving lost original native birth");
}
static std::vector<double> advance(Resident &owner, unsigned end) {
  std::vector<double> output;
  while (wire::decimal(packet::field(packet::field(owner.last.get(), "reading"),
                                     "samples_elapsed")) < end) {
    auto cursor = wire::decimal(packet::field(
        packet::field(owner.last.get(), "reading"), "samples_elapsed"));
    auto chunk =
        owner.render(unsigned(std::min<std::uint64_t>(128, end - cursor)));
    output.insert(output.end(), chunk.begin(), chunk.end());
  }
  return output;
}
static Json physical(J *checkpoint) {
  return copy(
      packet::field(packet::field(checkpoint, "native_pair"), "physical"));
}
// A separate genuine Management trial preserves the original Control trial
// below. All geometry/source packets come from the same Rust owner, including
// the privately produced after receiver segment. This is numerical custody,
// not a closed Scene/Act-reader authority positive.
struct SegmentSession {
  std::unique_ptr<PerformanceManagement> owner;
  std::shared_ptr<MovingReceivingPortBinding> receiving;
  std::unique_ptr<ql::PreparedPhysicalBody> immutable;
  std::vector<NativeGestureApplication> applications;
  std::vector<InputBindingRecord> journal;
  explicit SegmentSession(J *input, bool sounding = true) {
    auto native = prepare_source(input);
    immutable =
        std::make_unique<ql::PreparedPhysicalBody>(native.body->preparation());
    require(immutable->input().nodes.size() == 12 &&
                immutable->input().edges.size() == 34 &&
                native.notes.size() == 7,
            "actual segment sourceForm/sparse source changed");
    auto *current = packet::field(packet::field(input, "current_receiving"),
                                  "native_admission");
    auto *sources =
        packet::field(packet::field(current, "operation"), "sources");
    const auto count = json_object_array_length(sources);
    require(count == 0 || count == 9,
            "native context route cardinality differs");
    std::vector<std::string> program_refs;
    for (std::size_t i = 0; i < count; ++i)
      program_refs.push_back(
          packet::string(packet::field(json_object_array_get_idx(sources, i),
                                       "driver_ref")) +
          "/m1-excitation-program");
    auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
        read_native_receiving_admission(current, current, native,
                                        packet::field(input, "native_basis"),
                                        *immutable, program_refs, 0));
    auto routes = std::make_shared<PhysicalRoutesPortBinding>(
        native.body, admitted, *immutable, native.determination,
        packet::field(input, "native_basis"), native.determination.m1_face == 1,
        0);
    auto programs = std::make_unique<NativeRouteProgramSet>();
    programs->manifest = routes->manifest();
    programs->program_count = programs->manifest.route_count;
    programs->scalar_note_enabled = programs->manifest.scalar_note_enabled;
    programs->scalar_note_gain = programs->manifest.scalar_note_gain;
    for (std::size_t i = 0; i < programs->program_count; ++i) {
      auto &program = programs->programs[i];
      program.handle = programs->manifest.programs[i];
      program.phase_source_ref = programs->manifest.m1_coordinate;
      program.sine = native.notes.front().phase_sin;
      program.cosine = native.notes.front().phase_cos;
    }
    owner = std::make_unique<PerformanceManagement>(
        native, reference("native:acoustic/segment-manager"));
    require(owner->admit_distinct_receiving(
                     physical_routes_port(physical_port(native.body), routes),
                     *programs, 0)
                    .result == Result::Accepted,
            "actual native segment N9/scalar source admission refused");
    auto *catalog = packet::field(input, "native_catalog");
    std::vector<KeyboardCell> cells;
    for (std::size_t i = 0; i < json_object_array_length(catalog); ++i)
      cells.push_back(management_transport::read_key(
          json_object_array_get_idx(catalog, i)));
    owner->admit_catalog(std::move(cells));
    if (!sounding) {
      native.engine->enable_capture(true);
      return;
    }
    auto *acoustic_packet =
        packet::field(packet::field(input, "acoustic"), "packet");
    auto prepared = acoustic_wire::read_prepared_acoustic(
        acoustic_packet, acoustic_packet,
        packet::field(packet::field(input, "native_preparation"),
                      "physical_body"),
        *immutable, 0);
    receiving = std::make_shared<MovingReceivingPortBinding>(
        native.body, *immutable, 0, std::move(prepared));
    {
      auto guard = native.engine->acquire_stopped_custody();
      require(guard && native.engine->install_receiving_port(
                           receiving->port(receiving), guard, 0),
              "actual first acoustic native port refused");
      owner->refresh_stopped_reading(guard);
    }
    Operation note{};
    note.kind = Kind::NoteOn;
    note.identity = native.determination.identity;
    note.note = native.notes.front();
    note.touch = note.note.touch;
    note.sequence = 1;
    note.sample = 0;
    note.value = .7;
    const auto queued = owner->enqueue_score_input_admission(
        note, reference("native:acoustic/segment-original-input"));
    require(queued.result() == Result::Accepted && queued.queue().queued() &&
                queued.queue().operation().requested_sample == 0 &&
                queued.queue().operation().sample == 0,
            "actual original segment note queue failed");
    native.engine->enable_capture(true);
  }
  static NativePerformance prepare_source(J *input) {
    auto *p = packet::field(input, "native_preparation");
    auto *d = packet::field(p, "determination");
    auto *physical = packet::field(p, "physical_body");
    return prepare_source_performance_packet(
        json_object_to_json_string_ext(p, JSON_C_TO_STRING_PLAIN), p,
        packet::field(input, "native_basis"),
        packet::integer(packet::field(d, "m1_face")) == 1,
        packet::string(packet::field(
            packet::field(physical, "source_coordinate"), "face")) ==
            "pratibimba");
  }
  void reserve_future() {
    Operation parameter{};
    parameter.kind = Kind::Parameter;
    parameter.identity = owner->native().determination.identity;
    parameter.sequence = 2;
    parameter.sample = 9000;
    parameter.parameter = Parameter::MasterLinear;
    parameter.value = .61;
    auto admitted = owner->enqueue_score_input_admission(parameter);
    require(admitted.result() == Result::Accepted &&
                admitted.queue().operation().sample == 9000,
            "actual future segment automation refused");
    Operation release{};
    release.kind = Kind::NoteOff;
    release.identity = owner->native().determination.identity;
    release.sequence = 3;
    release.sample = 10000;
    release.touch = owner->native().notes.front().touch;
    auto off = owner->enqueue_score_input_admission(
        release, reference("native:acoustic/segment-original-input"));
    require(off.result() == Result::Accepted &&
                off.queue().operation().sample == 10000,
            "actual future original segment release refused");
  }
  struct Output {
    std::vector<float> pickup, received, pcm;
  };
  Output advance(std::uint64_t end, std::size_t block) {
    Output out;
    std::array<float, 512> pcm{};
    while (owner->native().engine->samples_elapsed() < end) {
      const auto start = owner->native().engine->samples_elapsed();
      const auto frames = std::min<std::size_t>(block, end - start);
      require(owner->offline_advance(pcm.data(), frames, start),
              "actual native segment A/P/receiving callback failed");
      Capture capture{};
      require(owner->pop_audio_capture(capture) && capture.frames == frames &&
                  capture.start_sample == start && capture.has_receiving,
              "same callback segment capture missing");
      auto pulse = owner->pulse();
      require(pulse->has_readback &&
                  pulse->reading.samples_elapsed == start + frames &&
                  pulse->reading.physical.samples_elapsed == start + frames &&
                  pulse->reading.has_receiving &&
                  pulse->reading.receiving.samples_elapsed == start + frames &&
                  pulse->recording.failure == RecordingFailure::None &&
                  same_receiving_manifest(pulse->reading.receiving.manifest,
                                          receiving->manifest()),
              "actual native segment pulse detached/lost recording");
      applications.insert(applications.end(), pulse->applications.begin(),
                          pulse->applications.end());
      journal.insert(journal.end(), pulse->input_history.begin(),
                     pulse->input_history.end());
      for (std::size_t i = 0; i < frames; ++i)
        require(capture.output_linear[i] == pcm[i],
                "actual segment output differs from capture");
      out.pickup.insert(out.pickup.end(), capture.pickup_linear.begin(),
                        capture.pickup_linear.begin() + frames);
      out.received.insert(out.received.end(), capture.received_linear.begin(),
                          capture.received_linear.begin() + frames);
      out.pcm.insert(out.pcm.end(), pcm.begin(), pcm.begin() + frames);
    }
    return out;
  }
};
#include "performance_acoustic_fresh_restore_cases.hpp"

static Json segment_replacement(J *fixture) {
  auto actual = std::make_unique<SegmentSession>(fixture);
  auto unchanged = std::make_unique<SegmentSession>(fixture);
  const auto first = actual->advance(4096, 128),
             first_unmoved = unchanged->advance(4096, 128);
  require(first.pickup == first_unmoved.pickup &&
              first.pcm == first_unmoved.pcm,
          "actual segment controls have different initial cause");
  actual->reserve_future();
  unchanged->reserve_future();
  auto before = actual->owner->stopped_checkpoint();
  auto *after_packet =
      packet::field(packet::field(fixture, "after_acoustic"), "packet");
  bool initial_refused = false;
  try {
    acoustic_wire::read_prepared_acoustic(
        after_packet, after_packet,
        packet::field(packet::field(fixture, "native_preparation"),
                      "physical_body"),
        *actual->immutable, 4096);
  } catch (const std::invalid_argument &) {
    initial_refused = true;
  }
  require(initial_refused,
          "first install manufactured a receiver before its actual birth");
  auto prepared = acoustic_wire::read_prepared_acoustic(
      after_packet, after_packet,
      packet::field(packet::field(fixture, "native_preparation"),
                    "physical_body"),
      *actual->immutable, 4096,
      acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
  auto candidate = std::make_shared<MovingReceivingPortBinding>(
      actual->owner->native().body, *actual->immutable, 4096,
      std::move(prepared), before->native_pair.audio.receiving);
  {
    auto guard = actual->owner->native().engine->acquire_stopped_custody();
    auto token =
        std::make_unique<PerformanceManagement::PreparedReceivingReplacement>();
    require(guard &&
                actual->owner->prepare_stopped_receiving_replacement(
                    candidate->port(candidate), guard, 4096, *token) &&
                actual->owner->receiving_replacement_current(*token, guard),
            "actual native segment replacement preflight failed");
    actual->owner->commit_stopped_receiving_replacement(*token, guard);
    actual->receiving = candidate;
    actual->owner->refresh_stopped_reading(guard);
  }
  auto saved = actual->owner->stopped_checkpoint();
  const auto &old = before->native_pair.audio.receiving;
  const auto &next = saved->native_pair.audio.receiving;
  require(next.samples_elapsed == 4096 && next.history_start_sample == 0 &&
              next.manifest.origin_sample == 4096 &&
              next.manifest.history_origin_sample == 0 &&
              std::memcmp(old.history_linear.data(), next.history_linear.data(),
                          sizeof(old.history_linear)) == 0,
          "receiver update reset exact original pickup history/birth");
  auto original = management_checkpoint_transport::checkpoint_wire(*before),
       operative = management_checkpoint_transport::checkpoint_wire(*saved);
  // Independently compare complete P and every audio/input/queue/programme
  // field, after excluding ONLY the receiving subsystem checked above.
  auto old_physical = physical(original.get()),
       new_physical = physical(operative.get());
  require(json_object_equal(old_physical.get(), new_physical.get()),
          "receiver segment changed sole physical q/v/eigenbasis/cursor");
  json_object_object_del(
      packet::field(packet::field(original.get(), "native_pair"), "audio"),
      "receiving");
  json_object_object_del(
      packet::field(packet::field(operative.get(), "native_pair"), "audio"),
      "receiving");
  require(json_object_equal(original.get(), operative.get()),
          "receiver segment changed held voices/N9 "
          "phase/gain/touch/source/future queue");
  const auto moved = actual->advance(13000, 128),
             unmoved = unchanged->advance(13000, 512);
  require(moved.pickup == unmoved.pickup &&
              moved.received != unmoved.received && moved.pcm != unmoved.pcm &&
              std::any_of(moved.received.begin(), moved.received.begin() + 512,
                          [](float value) { return value != 0.f; }),
          "actual receiver segment is disconnected or loses immediate retained "
          "tail");
  auto final = actual->owner->stopped_checkpoint(),
       stationary_final = unchanged->owner->stopped_checkpoint();
  auto final_wire = management_checkpoint_transport::checkpoint_wire(*final),
       stationary_wire =
           management_checkpoint_transport::checkpoint_wire(*stationary_final);
  auto fp = physical(final_wire.get()), sp = physical(stationary_wire.get());
  require(json_object_equal(fp.get(), sp.get()),
          "receiver movement changed source programmes/body cause");
  require(actual->applications.size() == 3,
          "native segment queue lost original future application");
  for (std::size_t i = 0; i < 3; ++i) {
    const auto &app = actual->applications[i];
    const std::array<std::uint64_t, 3> dates{0, 9000, 10000};
    require(app.applied && app.sequence == i + 1 &&
                app.applied_application_ordinal == i + 1 &&
                app.has_requested_sample && app.requested_sample == dates[i] &&
                app.admitted_sample == dates[i] &&
                app.applied_sample == dates[i],
            "actual native receiver update lost distinct "
            "original/admitted/applied timing");
  }
  require(std::count_if(
              actual->journal.begin(), actual->journal.end(),
              [](const auto &row) {
                return row.change == InputBindingChange::ReleaseAdmitted &&
                       row.native_sequence == 3 &&
                       row.input_ref ==
                           reference("native:acoustic/segment-original-input");
              }) == 1,
          "receiver update lost original released input lifetime");
  TransportAcknowledgement acknowledgement{};
  require(actual->owner->stopped_restore(
              *saved, 13000, reference("native:acoustic/segment-restore"),
              reference("native:acoustic/segment-checkpoint-4096"),
              acknowledgement) &&
              acknowledgement.previous_cursor == 13000 &&
              acknowledgement.target_sample == 4096 &&
              acknowledgement.epoch == acknowledgement.previous_epoch + 1,
          "actual stopped segment continuation refused");
  const auto replay = actual->advance(13000, 512);
  require(replay.pickup == moved.pickup && replay.received == moved.received &&
              replay.pcm == moved.pcm,
          "native receiver history/notes/future source continuation changed "
          "exact output");
  auto repeated = actual->owner->stopped_checkpoint();
  auto final_rx =
           wire::receiving_checkpoint(final->native_pair.audio.receiving),
       repeated_rx =
           wire::receiving_checkpoint(repeated->native_pair.audio.receiving);
  require(json_object_equal(final_rx.get(), repeated_rx.get()),
          "full continued receiving ring differs after same-owner restore");
  auto output = wire::object();
  wire::text(output.get(), "schema", "ql.native-acoustic-segment-component/v1");
  wire::text(output.get(), "standing",
             "actual source producer/Management numerical replacement; closed "
             "live/retained lease not exercised");
  wire::put(output.get(), "before_receiving",
            wire::receiving_checkpoint(old).release());
  wire::put(output.get(), "after_receiving",
            wire::receiving_checkpoint(next).release());
  wire::put(output.get(), "final_receiving", final_rx.release());
  wire::put(output.get(), "physical", fp.release());
  auto apps = wire::array();
  for (std::size_t i = 0; i < 3; ++i)
    wire::append(apps.get(),
                 wire::application(actual->applications[i]).release());
  wire::put(output.get(), "applications", apps.release());
  auto journal = wire::array();
  for (const auto &row : actual->journal)
    wire::append(journal.get(), management_transport::history(row).release());
  wire::put(output.get(), "input_history", journal.release());
  auto moved_pcm = wire::array(), unchanged_pcm = wire::array();
  for (float value : moved.pcm)
    wire::append(moved_pcm.get(), json_object_new_double(value));
  for (float value : unmoved.pcm)
    wire::append(unchanged_pcm.get(), json_object_new_double(value));
  wire::put(output.get(), "continued_pcm", moved_pcm.release());
  wire::put(output.get(), "unchanged_pcm", unchanged_pcm.release());
  wire::u64(output.get(), "end_cursor", 13000);
  return output;
}

// The existing Control, rather than direct numerical Manager calls, must own
// both swaps. All packet operands below are actual Rust producer outputs.
static void control_reserve_future(Resident &resident) {
  auto *p = packet::field(resident.fixture, "native_preparation");
  auto identity = packet::identity(
      packet::field(packet::field(p, "determination"), "identity"));
  Operation parameter{};
  parameter.kind = Kind::Parameter;
  parameter.identity = identity;
  parameter.sequence = 2;
  parameter.sample = 9000;
  parameter.parameter = Parameter::MasterLinear;
  parameter.value = .61;
  auto request = resident.command("score");
  wire::put(request.get(), "event", wire::operation(parameter).release());
  wire::put_null(request.get(), "input_ref");
  resident.apply(request.get());
  auto *queue = packet::field(packet::field(resident.last.get(), "payload"),
                              "score_admission");
  auto accepted = wire::read_operation(packet::field(queue, "event"));
  require(accepted.sequence == 2 && accepted.has_requested_sample &&
              accepted.requested_sample == 9000 && accepted.sample == 9000,
          "Control changed accepted future automation timing");
  Operation release{};
  release.kind = Kind::NoteOff;
  release.identity = identity;
  release.sequence = 3;
  release.sample = 10000;
  release.touch =
      packet::note(json_object_array_get_idx(packet::field(p, "notes"), 0))
          .touch;
  request = resident.command("score");
  wire::put(request.get(), "event", wire::operation(release).release());
  wire::text(request.get(), "input_ref", "native:original-janko/input");
  resident.apply(request.get());
  queue = packet::field(packet::field(resident.last.get(), "payload"),
                        "score_admission");
  accepted = wire::read_operation(packet::field(queue, "event"));
  require(accepted.sequence == 3 && accepted.has_requested_sample &&
              accepted.requested_sample == 10000 && accepted.sample == 10000,
          "Control changed accepted original future release timing");
}
static Json control_replace_request(Resident &resident, J *before, J *after,
                                    std::uint64_t cursor) {
  auto request = resident.command("receiving-transport-replace");
  wire::put(request.get(), "before_acoustic", json_object_get(before));
  wire::put(request.get(), "prepared_acoustic", json_object_get(after));
  wire::put(request.get(), "current_acoustic", json_object_get(after));
  wire::u64(request.get(), "expected_sample", cursor);
  return request;
}
static Json receiving_checkpoint(J *checkpoint) {
  return copy(packet::field(
      packet::field(packet::field(checkpoint, "native_pair"), "audio"),
      "receiving"));
}
static void require_nonreceiver_unchanged(J *before, J *after) {
  auto left = copy(before), right = copy(after);
  json_object_object_del(
      packet::field(packet::field(left.get(), "native_pair"), "audio"),
      "receiving");
  json_object_object_del(
      packet::field(packet::field(right.get(), "native_pair"), "audio"),
      "receiving");
  require(
      json_object_equal(left.get(), right.get()),
      "Control receiver swap changed source/P/voices/N9/input/future queue");
}
static void control_refusal_unchanged(Resident &resident, J *request,
                                      J *before) {
  bool refused = false;
  try {
    auto reply = resident.control.execute(request);
    refused = !packet::boolean(packet::field(reply.get(), "accepted"));
    if (refused) {
      require(json_object_array_length(
                  packet::field(reply.get(), "applications")) == 0 &&
                  json_object_array_length(
                      packet::field(reply.get(), "input_history")) == 0,
              "stopped Control refusal discarded a real callback cohort");
      resident.last = std::move(reply);
    }
  } catch (const std::invalid_argument &) {
    refused = true;
  }
  require(refused, "Control accepted detached receiver replacement operand");
  auto after = resident.checkpoint();
  require(json_object_equal(before, after.get()),
          "Control receiver refusal mutated full original native checkpoint");
}
static Json control_commit_receiver(Resident &resident, J *before_packet,
                                    J *after_packet, std::uint64_t cursor,
                                    J *before_checkpoint) {
  const auto epoch = copy(packet::field(
      packet::field(resident.last.get(), "reading"), "transport_epoch"));
  const auto sequence = copy(packet::field(
      packet::field(resident.last.get(), "reading"), "accepted_sequence"));
  auto request =
      control_replace_request(resident, before_packet, after_packet, cursor);
  resident.apply(request.get());
  auto acknowledgement = copy(packet::field(
      packet::field(resident.last.get(), "payload"), "receiving_replacement"));
  auto *reading = packet::field(resident.last.get(), "reading");
  auto *rx_reading = packet::field(reading, "receiving_transport");
  auto old = receiving_checkpoint(before_checkpoint);
  require(packet::string(packet::field(acknowledgement.get(), "schema")) ==
                  "ql.native-receiving-replacement/v1" &&
              wire::decimal(packet::field(acknowledgement.get(), "sample")) ==
                  cursor &&
              json_object_equal(
                  packet::field(acknowledgement.get(), "transport_epoch"),
                  epoch.get()) &&
              json_object_equal(
                  packet::field(acknowledgement.get(), "accepted_sequence"),
                  sequence.get()) &&
              json_object_equal(packet::field(reading, "transport_epoch"),
                                epoch.get()) &&
              json_object_equal(packet::field(reading, "accepted_sequence"),
                                sequence.get()) &&
              json_object_equal(
                  packet::field(acknowledgement.get(), "before_manifest"),
                  packet::field(old.get(), "manifest")) &&
              json_object_equal(
                  packet::field(acknowledgement.get(), "after_manifest"),
                  packet::field(rx_reading, "manifest")) &&
              wire::decimal(packet::field(reading, "samples_elapsed")) ==
                  cursor &&
              wire::decimal(packet::field(packet::field(reading, "physical"),
                                          "samples_elapsed")) == cursor &&
              wire::decimal(packet::field(rx_reading, "samples_elapsed")) ==
                  cursor &&
              json_object_array_length(
                  packet::field(resident.last.get(), "applications")) == 0 &&
              json_object_array_length(
                  packet::field(resident.last.get(), "input_history")) == 0,
          "real stopped Control ACK advanced/redated/lost native work");
  auto after = resident.checkpoint();
  auto next = receiving_checkpoint(after.get());
  require_nonreceiver_unchanged(before_checkpoint, after.get());
  require(json_object_equal(packet::field(old.get(), "history_linear"),
                            packet::field(next.get(), "history_linear")) &&
              json_object_equal(
                  packet::field(old.get(), "history_start_sample"),
                  packet::field(next.get(), "history_start_sample")) &&
              wire::decimal(packet::field(packet::field(next.get(), "manifest"),
                                          "origin_sample")) == cursor &&
              wire::decimal(packet::field(packet::field(next.get(), "manifest"),
                                          "history_origin_sample")) == 0,
          "Control replacement lost full pickup ring or immutable birth");
  return acknowledgement;
}
static Json control_receiver_replacement(J *fixture) {
  auto actual = std::make_unique<Resident>(fixture);
  auto unchanged = std::make_unique<Resident>(fixture);
  auto *original = packet::field(packet::field(fixture, "acoustic"), "packet");
  auto *after =
      packet::field(packet::field(fixture, "after_acoustic"), "packet");
  auto *second =
      packet::field(packet::field(fixture, "second_acoustic"), "packet");
  install(*actual, original);
  install(*unchanged, original);
  actual->attack();
  unchanged->attack();
  require(advance(*actual, 4096) == advance(*unchanged, 4096),
          "Control receiver trials have different initial physical cause");
  control_reserve_future(*actual);
  control_reserve_future(*unchanged);
  auto before = actual->checkpoint();
  for (unsigned variant = 0; variant < 7; ++variant) {
    auto request = control_replace_request(*actual, original, after, 4096);
    auto altered = copy(after);
    switch (variant) {
    case 0:
      wire::put(request.get(), "before_acoustic", json_object_get(after));
      break;
    case 1:
      wire::put(request.get(), "current_acoustic", json_object_get(original));
      break;
    case 2:
      wire::u64(request.get(), "expected_sample", 4095);
      break;
    case 3:
      wire::u64(altered.get(), "history_origin_sample", 1);
      break;
    case 4:
      wire::text(
          packet::field(packet::field(altered.get(), "context"), "context"),
          "reference", "native:acoustic/valid-other-context");
      break;
    case 5:
      wire::text(packet::field(altered.get(), "configuration"),
                 "source_motion_ref", "native:acoustic/valid-other-emitter");
      break;
    default:
      wire::put(packet::field(altered.get(), "configuration"), "revision",
                json_object_new_uint64(1));
      break;
    }
    if (variant >= 3) {
      wire::put(request.get(), "prepared_acoustic",
                json_object_get(altered.get()));
      wire::put(request.get(), "current_acoustic",
                json_object_get(altered.get()));
    }
    control_refusal_unchanged(*actual, request.get(), before.get());
  }
  auto ack1 =
      control_commit_receiver(*actual, original, after, 4096, before.get());
  auto moved_first = advance(*actual, 8192),
       plain_first = advance(*unchanged, 8192);
  require(moved_first != plain_first &&
              std::any_of(moved_first.begin(), moved_first.begin() + 512,
                          [](double value) { return value != 0.; }),
          "Control receiver update has no actual PCM/tail consequence");
  auto before_second = actual->checkpoint(),
       other_second = unchanged->checkpoint();
  auto p1 = physical(before_second.get()), p2 = physical(other_second.get());
  require(json_object_equal(p1.get(), p2.get()),
          "Control receiver update changed sole native physical q/v");
  // The second BEFORE packet has segment origin4096 and immutable birth0.
  // This genuinely detects reconstructing it at the current8192 cursor or
  // overwriting the expected history birth from that later segment origin.
  auto stale = control_replace_request(*actual, original, second, 8192);
  control_refusal_unchanged(*actual, stale.get(), before_second.get());
  auto ack2 = control_commit_receiver(*actual, after, second, 8192,
                                      before_second.get());
  auto saved = actual->checkpoint();
  auto moved_last = advance(*actual, 13000),
       plain_last = advance(*unchanged, 13000);
  auto final = actual->checkpoint(), other_final = unchanged->checkpoint();
  p1 = physical(final.get());
  p2 = physical(other_final.get());
  require(moved_last != plain_last && json_object_equal(p1.get(), p2.get()),
          "later Control receiver is disconnected or changed source body");
  require(json_object_array_length(actual->applications.get()) == 3,
          "Control receiver replacement lost actual accepted future work");
  const std::array<std::uint64_t, 3> dates{0, 9000, 10000};
  for (std::size_t i = 0; i < dates.size(); ++i) {
    auto app = wire::read_application(
        json_object_array_get_idx(actual->applications.get(), i));
    require(app.applied && app.sequence == i + 1 &&
                app.applied_application_ordinal == i + 1 &&
                app.has_requested_sample && app.requested_sample == dates[i] &&
                app.admitted_sample == dates[i] &&
                app.applied_sample == dates[i],
            "Control update changed original/queued/applied future operation");
  }
  std::size_t released = 0;
  for (std::size_t i = 0;
       i < json_object_array_length(actual->input_history.get()); ++i) {
    auto *row = json_object_array_get_idx(actual->input_history.get(), i);
    require(packet::string(packet::field(row, "input_ref")) ==
                "native:original-janko/input",
            "Control receiver update lost original independent input lifetime");
    released += packet::integer(packet::field(row, "change")) ==
                    unsigned(InputBindingChange::ReleaseAdmitted) &&
                wire::decimal(packet::field(row, "native_sequence")) == 3;
  }
  require(released == 1,
          "Control receiver update lost exact admitted release journal");
  auto original_journal = copy(actual->input_history.get());
  auto restore = actual->command("restore");
  wire::put(restore.get(), "checkpoint", json_object_get(saved.get()));
  wire::u64(restore.get(), "expected_cursor", 13000);
  wire::text(restore.get(), "transaction_ref",
             "native:acoustic/control-segment-restore");
  wire::text(restore.get(), "checkpoint_ref",
             "native:acoustic/control-segment-8192");
  actual->apply(restore.get());
  require(
      advance(*actual, 13000) == moved_last,
      "Control stopped reopen lost exact receiver/voice/future continuation");
  auto repeated = actual->checkpoint();
  auto rx_final = receiving_checkpoint(final.get()),
       rx_repeated = receiving_checkpoint(repeated.get());
  p2 = physical(repeated.get());
  require(json_object_equal(rx_final.get(), rx_repeated.get()) &&
              json_object_equal(p1.get(), p2.get()),
          "Control replay lost whole exact ring/P/source cursor");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-acoustic-control-replacement/v1");
  wire::text(out.get(), "standing",
             "actual source/Control numerical replacement; private "
             "source-reader grant not exercised");
  wire::put(out.get(), "first_ack", ack1.release());
  wire::put(out.get(), "second_ack", ack2.release());
  wire::put(out.get(), "before_checkpoint", before.release());
  wire::put(out.get(), "saved_checkpoint", saved.release());
  wire::put(out.get(), "final_receiving", rx_final.release());
  wire::put(out.get(), "physical", p1.release());
  auto all_pcm = wire::array(), plain_pcm = wire::array();
  moved_first.insert(moved_first.end(), moved_last.begin(), moved_last.end());
  plain_first.insert(plain_first.end(), plain_last.begin(), plain_last.end());
  for (double value : moved_first)
    wire::append(all_pcm.get(), json_object_new_double(value));
  for (double value : plain_first)
    wire::append(plain_pcm.get(), json_object_new_double(value));
  wire::put(out.get(), "continued_pcm", all_pcm.release());
  wire::put(out.get(), "unchanged_pcm", plain_pcm.release());
  // Saved pre-restore played prefix, not duplicated replay applications.
  auto apps = wire::array();
  for (std::size_t i = 0; i < 3; ++i)
    wire::append(apps.get(), json_object_get(json_object_array_get_idx(
                                 actual->applications.get(), i)));
  wire::put(out.get(), "applications", apps.release());
  wire::put(out.get(), "input_history", original_journal.release());
  wire::u64(out.get(), "end_cursor", 13000);
  return out;
}

#include "performance_acoustic_control_restore_cases.hpp"
#include "performance_resident_registry_cases.hpp"

int main() {
  try {
    std::string input((std::istreambuf_iterator<char>(std::cin)),
                      std::istreambuf_iterator<char>());
    require(input.size() < 16 * 1024 * 1024,
            "complete acoustic source exceeds native input bound");
    auto fixture = ql::physical_wire::parse_native(input.c_str());
    require(packet::string(packet::field(fixture.get(), "schema")) ==
                "ql.native-acoustic-owner-fixture/v1",
            "actual acoustic producer required");
    J *registry_trial = nullptr;
    if (json_object_object_get_ex(fixture.get(), "resident_registry_trial",
                                  &registry_trial)) {
      require(packet::boolean(registry_trial),
              "closed native registry trial required");
      auto result = actual_resident_registry(fixture.get());
      std::cout << json_object_to_json_string_ext(result.get(),
                                                  JSON_C_TO_STRING_PLAIN)
                << '\n';
      return 0;
    }
    J *saved_trial = nullptr;
    if (json_object_object_get_ex(fixture.get(), "control_saved_trial",
                                  &saved_trial)) {
      const auto cursor =
          wire::decimal(packet::field(saved_trial, "saved_cursor"));
      require(packet::string(packet::field(saved_trial, "schema")) ==
                      "ql.native-acoustic-control-fresh-trial/v1" &&
                  (cursor == 4096 || cursor == 8192),
              "closed genuine Control saved trial required");
      auto result =
          control_fresh_saved_receiver(fixture.get(), unsigned(cursor));
      std::cout << json_object_to_json_string_ext(result.get(),
                                                  JSON_C_TO_STRING_PLAIN)
                << '\n';
      return 0;
    }
    auto *acoustic =
        packet::field(packet::field(fixture.get(), "acoustic"), "packet");
    Resident baseline(fixture.get()), received(fixture.get());
    auto before = received.checkpoint();
    auto altered = copy(acoustic);
    json_object_object_add(packet::field(altered.get(), "configuration"),
                           "revision", json_object_new_uint64(2));
    auto request = received.command("receiving-transport-install");
    wire::put(request.get(), "prepared_acoustic", altered.release());
    wire::put(request.get(), "current_acoustic", json_object_get(acoustic));
    wire::u64(request.get(), "expected_sample", 0);
    bool refused = false;
    try {
      received.control.execute(request.get());
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    require(refused, "changed original receiver policy accepted");
    auto unchanged = received.checkpoint();
    require(json_object_equal(before.get(), unchanged.get()),
            "acoustic refusal mutated actual body/audio/queue checkpoint");
    install(received, acoustic);
    baseline.attack();
    received.attack();
    auto base_pcm = advance(baseline, 4096), rx_pcm = advance(received, 4096);
    auto base_cp = baseline.checkpoint(), saved = received.checkpoint();
    auto base_physical = physical(base_cp.get()),
         rx_physical = physical(saved.get());
    require(json_object_equal(base_physical.get(), rx_physical.get()),
            "receiver changed source eigenbasis/qv/pickup/state/cursor");
    require(base_pcm != rx_pcm,
            "actual installed acoustic transport disconnected from final PCM");
    auto *audio =
        packet::field(packet::field(saved.get(), "native_pair"), "audio");
    require(packet::boolean(packet::field(audio, "has_receiving")),
            "installed receiver missing in stopped continuation");
    auto *receiving = packet::field(audio, "receiving");
    packet::array(packet::field(receiving, "history_linear"), 16384);
    require(
        wire::decimal(packet::field(receiving, "samples_elapsed")) == 4096 &&
            wire::decimal(packet::field(receiving, "history_start_sample")) ==
                0,
        "stopped receiver history birth/cursor lost");
    require(std::any_of(rx_pcm.begin(), rx_pcm.end(),
                        [](double x) { return x != 0.; }),
            "actual installed acoustic delay never delivered pickup");
    auto next = advance(received, 6144);
    Resident reopened(fixture.get());
    install(reopened, acoustic);
    reopened.restore(saved.get());
    auto repeat = advance(reopened, 6144);
    require(next == repeat,
            "private-source prepared receiver lost exact native continuation");
    auto final = received.checkpoint(), repeat_cp = reopened.checkpoint();
    auto fp = physical(final.get()), rp = physical(repeat_cp.get());
    require(json_object_equal(fp.get(), rp.get()),
            "receiver restore detached sole physical state");
    auto *fa =
             packet::field(packet::field(final.get(), "native_pair"), "audio"),
         *ra = packet::field(packet::field(repeat_cp.get(), "native_pair"),
                             "audio");
    require(json_object_equal(packet::field(fa, "receiving"),
                              packet::field(ra, "receiving")),
            "full receiving history differs after actual reopen");
    auto segment = segment_replacement(fixture.get());
    auto control_segment = control_receiver_replacement(fixture.get());
    auto fresh_segment = fresh_saved_receiver_segment(fixture.get());
    auto output = wire::object();
    wire::text(output.get(), "schema",
               "ql.native-acoustic-control-component-receipt/v1");
    wire::text(output.get(), "standing",
               "actual numerical Control source/receiving; closed "
               "live/retained reader grant not exercised");
    wire::put(output.get(), "receiver_segment", segment.release());
    wire::put(output.get(), "fresh_receiver_segment", fresh_segment.release());
    wire::put(output.get(), "control_receiver_segment",
              control_segment.release());
    wire::put(output.get(), "context_kind",
              json_object_get(packet::field(fixture.get(), "context_kind")));
    require(json_object_array_length(received.applications.get()) == 1 &&
                json_object_array_length(received.input_history.get()) == 2,
            "actual acoustic note input/application lifetime lost");
    wire::put(
        output.get(), "native_note",
        json_object_get(json_object_array_get_idx(
            packet::field(packet::field(fixture.get(), "native_preparation"),
                          "notes"),
            0)));
    wire::put(output.get(), "applications", received.applications.release());
    wire::put(output.get(), "input_history", received.input_history.release());
    wire::put(output.get(), "checkpoint", saved.release());
    wire::put(output.get(), "physical", fp.release());
    wire::put(output.get(), "reading",
              json_object_get(packet::field(received.last.get(), "reading")));
    auto plain = wire::array(), transport = wire::array(),
         continuation = wire::array();
    for (auto value : base_pcm)
      wire::append(plain.get(), json_object_new_double(value));
    for (auto value : rx_pcm)
      wire::append(transport.get(), json_object_new_double(value));
    for (auto value : next)
      wire::append(continuation.get(), json_object_new_double(value));
    wire::put(output.get(), "baseline_pcm", plain.release());
    wire::put(output.get(), "received_pcm", transport.release());
    wire::put(output.get(), "continued_pcm", continuation.release());
    std::cout << json_object_to_json_string_ext(output.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
