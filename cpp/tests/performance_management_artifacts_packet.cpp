// Actual Rust production packet -> actual A/P -> complete retained receipts.
// Geometry/material keep their producer-declared Reference standing. No M3
// generation, source authority, material transition or device result is
// invented.
#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <ql/performance_management_checkpoint.hpp>
#include <ql/performance_offline_wire.hpp>
#include <string>

using namespace ql::performance;
namespace wire = ql::performance::checkpoint_transport;

static std::string read_file(const std::filesystem::path &path) {
  std::ifstream stream(path, std::ios::binary);
  if (!stream)
    throw std::invalid_argument("missing actual native producer input");
  stream.seekg(0, std::ios::end);
  const auto size = stream.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("actual producer input exceeds finite bound");
  std::string out(std::size_t(size), '\0');
  stream.seekg(0);
  if (!stream.read(out.data(), size))
    throw std::invalid_argument("incomplete actual producer input");
  return out;
}
static wire::Json parse_file(const std::filesystem::path &path) {
  const auto source = read_file(path);
  auto *tok = json_tokener_new_ex(64);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto out =
      wire::own(json_tokener_parse_ex(tok, source.data(), int(source.size())));
  const auto error = json_tokener_get_error(tok);
  const auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  if (error != json_tokener_success || !out ||
      source.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("complete actual native JSON input required");
  return out;
}
static void save(const std::filesystem::path &directory, const char *name,
                 json_object *value) {
  const auto path = directory / name;
  if (!std::filesystem::is_directory(directory) ||
      std::filesystem::exists(path))
    throw std::invalid_argument(
        "existing caller-owned fresh artifact directory required");
  const char *encoded =
      json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN);
  if (!encoded || std::strlen(encoded) > 32 * 1024 * 1024)
    throw std::invalid_argument("native artifact exceeds finite output bound");
  std::ofstream stream(path, std::ios::binary);
  if (!stream || !(stream << encoded << '\n') || !stream.flush())
    throw std::runtime_error("native artifact write failed");
}
static Operation operation(const NativePerformance &native, Kind kind,
                           std::uint64_t sequence, std::uint64_t sample) {
  Operation op{};
  op.identity = native.determination.identity;
  op.kind = kind;
  op.sequence = sequence;
  op.sample = sample;
  return op;
}
static std::unique_ptr<ManagementCheckpoint>
checkpoint(NativePerformance &native, const NativeInputBindings &inputs) {
  auto out = std::make_unique<ManagementCheckpoint>();
  auto guard = native.engine->acquire_stopped_custody();
  if (!guard)
    throw std::logic_error("actual stopped callback custody required");
  native.engine->write_checkpoint(out->native_pair.audio, guard);
  out->native_pair.physical = native.body->checkpoint();
  inputs.write_checkpoint(out->bindings);
  out->session = reference("reference:actual-native-baseline/session");
  out->transport_epoch = 1;
  assert(valid_management_checkpoint(*out));
  return out;
}
static wire::Json history_entry(const InputBindingRecord &entry) {
  auto out = wire::object();
  wire::u64(out.get(), "ordinal", entry.ordinal);
  wire::u64(out.get(), "native_sequence", entry.native_sequence);
  wire::put(out.get(), "change",
            json_object_new_uint64(unsigned(entry.change)));
  wire::put(out.get(), "operation",
            json_object_new_uint64(unsigned(entry.operation)));
  wire::ref(out.get(), "input_ref", entry.input_ref);
  wire::put(out.get(), "target", wire::note(entry.target).release());
  return out;
}
int main(int argc, char **argv) {
  if (argc != 3 && argc != 4) {
    std::cerr
        << "usage: performance_management_artifacts_packet-test "
           "ACTUAL_NATIVE_PRODUCER_DIRECTORY EXISTING_FRESH_OUTPUT_DIRECTORY "
           "[NATIVE_C_PERFORMANCE_SHA256]\n";
    return 2;
  }
  try {
    const std::filesystem::path input_directory(argv[1]),
        output_directory(argv[2]);
    if (!std::filesystem::is_directory(output_directory))
      throw std::invalid_argument(
          "caller must admit an existing output directory");
    auto basis = parse_file(input_directory / "baseline.basis.json");
    auto native = prepare_performance_packet(
        read_file(input_directory / "baseline.packet.json"), basis.get(), true,
        true);
    assert(native.notes.size() == 12);
    auto inputs = std::make_unique<NativeInputBindings>();
    const auto input_ref = reference("reference:original-input/pointer-42");
    const auto &note = native.notes[4];
    assert(input_ref != note.touch_ref);
    auto attack = operation(native, Kind::NoteOn, 1, 37);
    attack.note = note;
    attack.value = 0.8;
    auto expression = operation(native, Kind::Expression, 2, 101);
    expression.touch = note.touch;
    expression.value = 0.63;
    auto parameter = operation(native, Kind::Parameter, 3, 149);
    parameter.parameter = Parameter::MasterLinear;
    parameter.value = 0.35;
    auto release = operation(native, Kind::NoteOff, 4, 256);
    release.touch = note.touch;
    assert(native.engine->enqueue(attack) == Result::Accepted);
    assert(inputs->bind(input_ref, note, attack.sequence));
    assert(native.engine->enqueue(expression) == Result::Accepted);
    assert(native.engine->enqueue(parameter) == Result::Accepted);
    assert(native.engine->enqueue(release) == Result::Accepted);
    assert(inputs->release_admitted(input_ref, release.sequence));
    native.engine->enable_capture(true);
    std::array<float, 128> output{};
    assert(native.engine->render(output.data(), output.size(), 0));
    auto pending = checkpoint(native, *inputs);
    assert(pending->native_pair.audio.cursor == 128 &&
           pending->native_pair.physical.samples_elapsed == 128 &&
           pending->bindings.inputs[0].release_pending &&
           pending->native_pair.audio.applications.write -
                   pending->native_pair.audio.applications.read ==
               2);
    assert(pending->native_pair.physical.source_generation ==
           native.body->preparation().input().source_generation);
    auto pending_wire =
        management_checkpoint_transport::checkpoint_wire(*pending);
    auto round = management_checkpoint_transport::read_checkpoint_wire(
        pending_wire.get());
    assert(round->bindings.inputs[0].input_ref == input_ref &&
           round->bindings.inputs[0].target.touch_ref == note.touch_ref);
    save(output_directory, "baseline.pending.management.json",
         pending_wire.get());
    save(output_directory, "baseline.basis.json", basis.get());
    // This output retains the original actual source/body before any explicit
    // test-owned numerical material regression in the separate suite.
    auto applications = wire::array();
    std::uint64_t last = 0;
    auto drain = [&] {
      NativeGestureApplication entry{};
      while (native.engine->pop_gesture_application(entry)) {
        assert(entry.applied && entry.sequence == ++last &&
               entry.identity == native.determination.identity &&
               entry.body_revision == native.determination.body_revision &&
               entry.preparation_ref ==
                   native.determination.body_preparation_ref &&
               entry.physical_source_generation ==
                   native.body->preparation().input().source_generation);
        assert(inputs->application(entry));
        wire::append(applications.get(), wire::application(entry).release());
      }
    };
    drain();
    assert(inputs->find(input_ref) && inputs->find(input_ref)->press_applied);
    assert(native.engine->render(output.data(), output.size(), 128));
    drain();
    assert(native.engine->render(output.data(), output.size(), 256));
    drain();
    assert(last == 4 && !inputs->find(input_ref));
    auto events = wire::object();
    wire::text(events.get(), "schema", "ql.native-applied-event-artifact/v1");
    wire::text(events.get(), "standing",
               "actual-native-producer-applied; reference-metric-body; "
               "no-device-claim");
    wire::put(events.get(), "applications", applications.release());
    save(output_directory, "baseline.applied-events.json", events.get());
    auto history = wire::array();
    InputBindingRecord record{};
    std::uint64_t ordinal = 0;
    while (inputs->pop_history(record)) {
      assert(record.ordinal == ++ordinal && record.input_ref == input_ref &&
             record.target.touch_ref == note.touch_ref &&
             record.change != InputBindingChange::Retired);
      wire::append(history.get(), history_entry(record).release());
    }
    assert(ordinal == 5); // admissions + applied attack/expression/release.
    auto journal = wire::object();
    wire::text(journal.get(), "schema", "ql.native-input-journal-artifact/v1");
    wire::put(journal.get(), "entries", history.release());
    save(output_directory, "baseline.input-journal.json", journal.get());
    auto current = checkpoint(native, *inputs);
    auto current_wire =
        management_checkpoint_transport::checkpoint_wire(*current);
    save(output_directory, "baseline.current.management.json",
         current_wire.get());
    if (argc == 4) {
      // The supplied digest is independently established by the existing C
      // owner. This finite numeric fixture does not authenticate C file/Act
      // custody by accepting a digest-shaped argument.
      offline_transport::Scope scope{};
      scope.performance_digest = argv[3];
      assert(offline_transport::digest_valid(scope.performance_digest));
      scope.native.session = current->session;
      scope.native.scene = reference("reference:actual-native-baseline/scene");
      scope.native.performance_revision =
          reference("reference:actual-applied-score/revision");
      scope.native.basis_seal =
          reference("reference:original-native-basis/fixture");
      scope.native.event_prefix_seal =
          reference("reference:actual-applied-sequences/1-4");
      scope.native.checkpoint_ref =
          reference("reference:baseline.current.management.json");
      scope.native.expected_source = native.determination.identity;
      scope.native.expected_body_revision = native.body->body_revision();
      scope.native.expected_cursor = 384;
      scope.native.expected_accepted_sequence = 4;
      auto encoded_scope = offline_transport::scope_wire(scope);
      scope = offline_transport::read_scope(encoded_scope.get());
      save(output_directory, "baseline.offline.scope.json",
           encoded_scope.get());
      // Reopen the original exact checkpoint, including phases/q-v/tails and
      // queue order, and compare every exported captured float to that ordinary
      // A/P continuation. There is no second synth or test waveform renderer.
      auto reopened = prepare_performance_packet(
          read_file(input_directory / "baseline.packet.json"), basis.get(),
          true, true);
      {
        auto guard = reopened.engine->acquire_stopped_custody();
        assert(guard && restore_checkpoint(*reopened.engine, *reopened.body,
                                           current->native_pair, guard, 0));
      }
      auto chunks = wire::array();
      std::array<float, max_frames> pcm{}, continued{};
      for (std::size_t frames : {std::size_t(512), std::size_t(129)}) {
        auto chunk = render_native_offline_chunk(
            *native.engine, native.body, scope.native, pcm.data(), frames);
        assert(chunk->result == Result::Accepted && chunk->capture_complete);
        assert(reopened.engine->render(continued.data(), frames,
                                       scope.native.expected_cursor));
        for (std::size_t i = 0; i < frames; ++i)
          assert(pcm[i] == continued[i] &&
                 pcm[i] == chunk->capture.output_linear[i]);
        auto encoded = offline_transport::chunk_wire(scope, *chunk);
        assert(packet::string(wire::field(encoded.get(), "result")) ==
               "accepted");
        packet::array(wire::field(encoded.get(), "interleaved_f32"), frames);
        for (std::size_t i = 0; i < frames; ++i)
          assert(float(packet::number(json_object_array_get_idx(
                     wire::field(encoded.get(), "interleaved_f32"), i))) ==
                 pcm[i]);
        wire::append(chunks.get(), encoded.release());
        scope.native.expected_cursor = chunk->committed_cursor;
      }
      assert(scope.native.expected_cursor == 1025);
      save(output_directory, "baseline.offline.chunks.json", chunks.get());
      ++scope.native.expected_source.m2_generation;
      auto refused = render_native_offline_chunk(*native.engine, native.body,
                                                 scope.native, pcm.data(), 128);
      assert(refused->result == Result::Stale && !refused->state_committed &&
             native.engine->samples_elapsed() == 1025);
      auto refused_wire = offline_transport::chunk_wire(scope, *refused);
      assert(packet::string(wire::field(refused_wire.get(), "result")) ==
             "refused");
      packet::array(wire::field(refused_wire.get(), "interleaved_f32"), 0);
      save(output_directory, "baseline.offline.stale-refusal.json",
           refused_wire.get());
    }
    auto manifest = wire::object();
    wire::text(manifest.get(), "schema", "ql.native-management-artifacts/v1");
    wire::text(manifest.get(), "source_qualification",
               "actual-Rust-native-M1-K-M2-B-and-M3-consumer; "
               "unchanged-original-basis; metric-Reference-standing");
    wire::text(manifest.get(), "kind6_coverage",
               "not-emitted; "
               "sequential-authenticated-native-Determination-producer-not-"
               "admitted-by-this-fixture");
    wire::text(manifest.get(), "material_transition",
               "absent; "
               "after-material-test-checkpoint-from-separate-suite-must-not-"
               "use-this-original-basis-as-authenticated-successor");
    wire::text(manifest.get(), "device_standing",
               "offline-actual-A-P-only; "
               "no-installed-output-or-physical-latency-claim");
    wire::put(manifest.get(), "identity",
              wire::identity(native.determination.identity).release());
    wire::u64(manifest.get(), "body_revision", native.body->body_revision());
    wire::u64(manifest.get(), "source_generation",
              native.body->preparation().input().source_generation);
    wire::u64(manifest.get(), "pending_checkpoint_cursor", 128);
    wire::u64(manifest.get(), "current_checkpoint_cursor", 384);
    wire::u64(manifest.get(), "applied_events", last);
    wire::u64(manifest.get(), "input_history_records", ordinal);
    save(output_directory, "manifest.json", manifest.get());
    std::cout << json_object_to_json_string_ext(manifest.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
