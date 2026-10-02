#include <array>
#include <cassert>
#include <fstream>
#include <iostream>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>
using namespace ql::performance;
static std::string file(const std::string &path) {
  std::ifstream in(path, std::ios::binary);
  if (!in)
    throw std::invalid_argument("actual producer fixture absent");
  in.seekg(0, std::ios::end);
  const auto size = in.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("fixture size refused");
  std::string out(std::size_t(size), '\0');
  in.seekg(0);
  if (!in.read(out.data(), size))
    throw std::invalid_argument("fixture read refused");
  return out;
}
static ql::physical_wire::Json parse(const std::string &text) {
  auto *tok = json_tokener_new_ex(64);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(tok, text.data(), int(text.size())));
  const auto error = json_tokener_get_error(tok);
  const auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  if (error != json_tokener_success || !out ||
      text.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("strict fixture JSON refused");
  return out;
}
static Operation op(const Determination &d, Kind kind, std::uint64_t id,
                    std::uint64_t sample) {
  Operation out{};
  out.identity = d.identity;
  out.kind = kind;
  out.sequence = id;
  out.sample = sample;
  return out;
}
static bool render(Engine &engine, float *output, std::size_t frames) {
  return engine.render(output, frames, engine.samples_elapsed());
}
static void trial(const std::string &dir, bool panic) {
  auto basis = parse(file(dir + "/baseline.basis.json"));
  auto native = prepare_performance_packet(file(dir + "/baseline.packet.json"),
                                           basis.get(), true, true);
  assert(native.notes.size() == 12);
  auto attack = op(native.determination, Kind::NoteOn, 1, 37);
  attack.note = native.notes[4];
  attack.value = .8;
  assert(native.engine->enqueue(attack) == Result::Accepted);
  auto future = op(native.determination, Kind::Parameter, 2, 48000);
  future.parameter = Parameter::MasterLinear;
  future.value = .2;
  assert(native.engine->enqueue(future) == Result::Accepted);
  std::array<float, 512> pcm{}, continued{};
  assert(render(*native.engine, pcm.data(), 128));
  auto release =
      op(native.determination, panic ? Kind::Panic : Kind::NoteOff, 3, 0);
  release.touch = attack.note.touch;
  assert(native.engine->enqueue(release) == Result::Accepted);
  assert(render(*native.engine, pcm.data(), 128));
  std::unique_ptr<PairedCheckpoint> saved;
  {
    auto guard = native.engine->acquire_stopped_custody();
    assert(guard);
    saved = checkpoint_heap(*native.engine, *native.body, guard);
  }
  assert(saved->audio.applied_application_ordinal == 2 &&
         saved->audio.accepted_sequence == 3);
  assert(saved->audio.applications.write - saved->audio.applications.read == 2);
  const auto &first =
      saved->audio.applications.storage[saved->audio.applications.read % 256];
  const auto &second = saved->audio.applications
                           .storage[(saved->audio.applications.read + 1) % 256];
  assert(first.sequence == 1 && first.applied_application_ordinal == 1 &&
         first.applied);
  assert(second.sequence == 3 && second.applied_application_ordinal == 2 &&
         second.applied && second.applied_sample == 128);
  assert(saved->audio.heap_size == 1);
  assert(saved->audio.pending_operations[saved->audio.operation_heap[0]]
             .operation.sequence == 2);
  assert(saved->audio.pending_operations[saved->audio.operation_heap[0]]
             .operation.sample == 48000);
  auto encoded = checkpoint_transport::checkpoint_wire(*saved);
  auto decoded = checkpoint_transport::read_checkpoint_wire(encoded.get());
  auto body = std::make_shared<ql::PhysicalBody>(native.body->preparation());
  auto reopened = std::make_shared<Engine>(native.determination, 48000,
                                           physical_port(body));
  {
    auto restored = reopened->acquire_stopped_custody();
    assert(restored);
    assert(restore_checkpoint(*reopened, *body, *decoded, restored, 0));
    auto &corrupt = decoded->audio.applications
                        .storage[(decoded->audio.applications.read + 1) % 256];
    const auto original = corrupt.applied_application_ordinal;
    corrupt.applied_application_ordinal = 1;
    assert(!reopened->validate_checkpoint(decoded->audio, restored, 256));
    assert(body->samples_elapsed() == 256);
    corrupt.applied_application_ordinal = original;
  }
  NativeGestureApplication a{}, b{};
  for (unsigned i = 0; i < 2; ++i) {
    assert(native.engine->pop_gesture_application_up_to(a, i + 1));
    assert(reopened->pop_gesture_application_up_to(b, i + 1));
    assert(a.sequence == b.sequence &&
           a.applied_application_ordinal == b.applied_application_ordinal);
    assert(!native.engine->pop_gesture_application_up_to(a, i + 1));
    assert(!reopened->pop_gesture_application_up_to(b, i + 1));
  }
  Readback reading{}, reopened_reading{};
  while (native.engine->samples_elapsed() < 48128) {
    const auto frames = std::size_t(
        std::min<std::uint64_t>(512, 48128 - native.engine->samples_elapsed()));
    assert(render(*native.engine, pcm.data(), frames));
    assert(render(*reopened, continued.data(), frames));
    assert(std::equal(pcm.begin(), pcm.begin() + frames, continued.begin()));
    while (native.engine->pop_readback(reading)) {
    }
    while (reopened->pop_readback(reopened_reading)) {
    }
  }
  assert(native.engine->pop_gesture_application(a) &&
         reopened->pop_gesture_application(b));
  assert(a.sequence == 2 && a.applied_application_ordinal == 3 && a.applied &&
         a.applied_sample == 48000);
  assert(b.sequence == 2 && b.applied_application_ordinal == 3 &&
         b.applied_sample == 48000);
  assert(native.engine->recording_status().failure == RecordingFailure::None);
  assert(!native.engine->pop_gesture_application(a));
  assert(reading.last_applied_application_ordinal == 3 &&
         reading.samples_elapsed == 48128);
  assert(reopened_reading.last_applied_application_ordinal == 3 &&
         reopened_reading.samples_elapsed == 48128);
  assert(reading.dropped_readbacks == 0 &&
         reopened_reading.dropped_readbacks == 0);
  assert(native.engine->samples_elapsed() == 48128 &&
         body->samples_elapsed() == 48128);
  auto receipt = checkpoint_transport::application(b);
  std::cout << json_object_to_json_string_ext(receipt.get(),
                                              JSON_C_TO_STRING_PLAIN)
            << '\n';
  std::cout << "{\"schema\":\"ql.application-order-regression/"
               "v1\",\"admission_order\":[1,2,3],\"application_operation_"
               "ids\":[1,3,2],\"application_ordinals\":[1,2,3],\"checkpoint_"
               "cursor\":\"256\",\"continuation_cursor\":\"48128\",\"scope\":"
               "\"actual native producer A/P in-memory; no device claim\"}\n";
}
int main(int argc, char **argv) {
  try {
    if (argc != 2)
      throw std::invalid_argument(
          "usage: performance_application_order_packet-test "
          "ACTUAL_NATIVE_PRODUCER_FIXTURE_DIRECTORY");
    trial(argv[1], false);
    trial(argv[1], true);
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
