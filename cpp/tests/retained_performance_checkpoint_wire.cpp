// Actual Rust/C source preparation -> native A/P state -> exact continuation.
// This emits native state for OI tests, never invents an audio/body checkpoint.
#include <array>
#include <cassert>
#include <fstream>
#include <iostream>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_packet.hpp>

using namespace ql::performance;
static ql::physical_wire::Json parse(const std::string &text) {
  auto *tokener = json_tokener_new_ex(64);
  if (!tokener)
    throw std::bad_alloc();
  json_tokener_set_flags(tokener, JSON_TOKENER_STRICT);
  auto value = ql::physical_wire::own(
      json_tokener_parse_ex(tokener, text.data(), int(text.size())));
  const auto error = json_tokener_get_error(tokener);
  const auto end = json_tokener_get_parse_end(tokener);
  json_tokener_free(tokener);
  if (error != json_tokener_success || !value ||
      text.find_first_not_of(" \t\r\n", end) != std::string::npos)
    throw std::invalid_argument("complete strict native fixture required");
  return value;
}
static std::string read(const char *path) {
  std::ifstream input(path, std::ios::binary);
  if (!input)
    throw std::invalid_argument("actual native producer fixture absent");
  input.seekg(0, std::ios::end);
  const auto size = input.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("native fixture size bound exceeded");
  std::string text(std::size_t(size), '\0');
  input.seekg(0);
  if (!input.read(text.data(), size))
    throw std::invalid_argument("native fixture incomplete");
  return text;
}
static Operation operation(const Determination &d, Kind kind, std::uint64_t seq,
                           std::uint64_t sample) {
  Operation op{};
  op.kind = kind;
  op.identity = d.identity;
  op.sequence = seq;
  op.sample = sample;
  return op;
}
static std::array<float, 1024> render(NativePerformance &native) {
  std::array<float, 1024> pcm{};
  for (std::size_t i = 0; i < pcm.size(); i += 256) {
    const auto start = native.engine->samples_elapsed();
    assert(native.engine->render(pcm.data() + i, 256, start));
    Readback view{};
    assert(native.engine->pop_readback(view));
    assert(view.available && view.samples_elapsed == start + 256 &&
           view.physical.samples_elapsed == view.samples_elapsed);
  }
  return pcm;
}
int main(int argc, char **argv) {
  if (argc != 2) {
    std::cerr << "usage: retained_performance_checkpoint ACTUAL_RUST_FIXTURE\n";
    return 2;
  }
  try {
    auto fixture = parse(read(argv[1]));
    auto *prepared = packet::field(fixture.get(), "native_preparation");
    const std::string text =
        json_object_to_json_string_ext(prepared, JSON_C_TO_STRING_PLAIN);
    auto native = prepare_performance_packet(
        text, packet::field(prepared, "native_basis"), true, true);
    assert(native.notes.size() == 12);
    std::uint64_t sequence = 0;
    for (const auto &target : native.notes) {
      auto op = operation(native.determination, Kind::NoteOn, ++sequence, 137);
      op.note = target;
      op.value = 0.8;
      assert(native.engine->enqueue(op) == Result::Accepted);
    }
    auto pedal =
        operation(native.determination, Kind::Sustain, ++sequence, 600);
    pedal.value = 1;
    assert(native.engine->enqueue(pedal) == Result::Accepted);
    for (const auto &target : native.notes) {
      auto op = operation(native.determination, Kind::NoteOff, ++sequence, 900);
      op.touch = target.touch;
      assert(native.engine->enqueue(op) == Result::Accepted);
    }
    auto parameter =
        operation(native.determination, Kind::Parameter, ++sequence, 1150);
    parameter.parameter = Parameter::CutoffHertz;
    parameter.value = 900;
    assert(native.engine->enqueue(parameter) == Result::Accepted);
    const auto parameter_sequence = sequence;
    pedal = operation(native.determination, Kind::Sustain, ++sequence, 1500);
    pedal.value = 0;
    assert(native.engine->enqueue(pedal) == Result::Accepted);
    const auto pedal_sequence = sequence;
    const auto attack = render(native);
    assert(std::any_of(attack.begin(), attack.end(),
                       [](float v) { return v != 0; }));
    std::unique_ptr<PairedCheckpoint> saved;
    {
      auto guard = native.engine->acquire_stopped_custody();
      saved = checkpoint_heap(*native.engine, *native.body, guard);
    }
    assert(saved->audio.cursor == 1024 &&
           saved->physical.samples_elapsed == 1024);
    assert(saved->audio.sustain);
    auto wire = checkpoint_transport::checkpoint_wire(*saved);
    auto reread = checkpoint_transport::read_checkpoint_wire(wire.get());
    const auto uninterrupted = render(native);
    {
      auto guard = native.engine->acquire_stopped_custody();
      assert(restore_checkpoint(*native.engine, *native.body, *reread, guard,
                                2048));
    }
    const auto continued = render(native);
    assert(uninterrupted == continued);
    auto after = ql::physical_wire::checkpoint_wire(native.body->checkpoint());
    // Atomic refusal preserves actual current A/P state when body/model state
    // is corrupted. Removing velocity/source schedule is detected by parsers.
    auto broken = parse(
        json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN));
    json_object_object_del(
        packet::field(packet::field(broken.get(), "physical"), "state"),
        "velocity_modal_metres_per_second");
    bool refused = false;
    try {
      checkpoint_transport::read_checkpoint_wire(broken.get());
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    broken = parse(
        json_object_to_json_string_ext(wire.get(), JSON_C_TO_STRING_PLAIN));
    json_object_object_del(packet::field(broken.get(), "audio"),
                           "source_schedule");
    refused = false;
    try {
      checkpoint_transport::read_checkpoint_wire(broken.get());
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    auto output = ql::physical_wire::own(json_object_new_object());
    json_object_object_add(
        output.get(), "schema",
        json_object_new_string(
            "ql.retained-performance-checkpoint-fixture/v1"));
    json_object_object_add(output.get(), "native_pair", wire.release());
    json_object_object_add(output.get(), "native_physical_after_exact_replay",
                           after.release());
    auto *queued = json_object_new_array();
    for (const auto &entry :
         {std::pair{parameter_sequence, std::uint64_t(1150)},
          std::pair{pedal_sequence, std::uint64_t(1500)}}) {
      auto *receipt = json_object_new_object();
      json_object_object_add(
          receipt, "native_sequence",
          json_object_new_string(std::to_string(entry.first).c_str()));
      json_object_object_add(
          receipt, "recorded_sequence",
          json_object_new_string(std::to_string(entry.first).c_str()));
      json_object_object_add(
          receipt, "effective_sample",
          json_object_new_string(std::to_string(entry.second).c_str()));
      json_object_array_add(queued, receipt);
    }
    json_object_object_add(output.get(), "queued_events", queued);
    std::cout << json_object_to_json_string_ext(output.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
    std::cerr << "actual native checkpoint1024 -> future "
                 "parameter/pedal/release/physical tail -> exact1024-frame "
                 "continuation; dropped native fields refused\n";
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
