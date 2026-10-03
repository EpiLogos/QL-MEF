// Actual Rust source/receiving owners -> existing worker Control -> sole A/P.
// In-memory native proof only: installed UI/device/C export remain separate.
#include <iostream>
#include <ql/performance_management_wire.hpp>
#include <vector>
using namespace ql::performance;
namespace wire = checkpoint_transport;
using Json = wire::Json;
using J = json_object;
static Json copy(J *value) {
  return ql::physical_wire::parse_native(
      json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN));
}
struct Resident {
  management_transport::Control control;
  J *fixture;
  Json last;
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
    auto *pcm = packet::field(chunk, "interleaved_f32");
    packet::array(pcm, frames);
    std::vector<double> out;
    for (unsigned i = 0; i < frames; ++i)
      out.push_back(packet::number(json_object_array_get_idx(pcm, i)));
    return out;
  }
};
int main() {
  try {
    std::string bytes;
    char c;
    while (std::cin.get(c)) {
      bytes.push_back(c);
      require(bytes.size() <= 16 * 1024 * 1024,
              "native receiving fixture exceeds bound");
    }
    auto root = ql::physical_wire::parse_native(bytes);
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
    auto out = wire::object();
    wire::text(out.get(), "schema",
               "ql.current-native-receiving-worker-receipt/v1");
    wire::put(out.get(), "contexts", results.release());
    std::cout << json_object_to_json_string_ext(out.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
