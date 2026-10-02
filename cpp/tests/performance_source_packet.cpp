#include <cassert>
#include <fstream>
#include <iostream>
#include <ql/performance_management.hpp>
#include <ql/performance_source_packet.hpp>
using namespace ql::performance;
static ql::physical_wire::Json fixture(const char *path) {
  std::ifstream in(path, std::ios::binary);
  if (!in)
    throw std::invalid_argument("actual native source fixture absent");
  in.seekg(0, std::ios::end);
  auto size = in.tellg();
  if (size < 1 || size > 16 * 1024 * 1024)
    throw std::invalid_argument("fixture bound");
  std::string text(std::size_t(size), '\0');
  in.seekg(0);
  if (!in.read(text.data(), size))
    throw std::invalid_argument("fixture incomplete");
  auto *tok = json_tokener_new_ex(128);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(tok, text.data(), int(text.size())));
  auto error = json_tokener_get_error(tok);
  auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  if (error != json_tokener_success || !out ||
      text.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("strict fixture");
  return out;
}
static std::string encoded(json_object *value) {
  return json_object_to_json_string_ext(value, JSON_C_TO_STRING_PLAIN);
}
int main(int argc, char **argv) {
  try {
    if (argc != 2)
      throw std::invalid_argument(
          "usage: performance_source_packet-test "
          "ACTUAL_RETAINED_SOURCE_PERFORMANCE_FIXTURE_JSON");
    auto root = fixture(argv[1]);
    assert(packet::string(packet::field(root.get(), "schema")) ==
           "ql.retained-source-performance-fixture/v1");
    auto current = packet::field(root.get(), "native_preparation");
    auto basis = packet::field(root.get(), "native_basis");
    auto native = prepare_source_performance_packet(encoded(current), current,
                                                    basis, true, true);
    assert(native.notes.size() == 7 &&
           native.body->preparation().input().nodes.size() == 12 &&
           native.body->preparation().input().edges.size() == 34);
    assert(json_object_equal(packet::field(packet::field(root.get(), "basis"),
                                           "audio_determination"),
                             packet::field(current, "determination")));
    auto pitches = packet::field(root.get(), "pitches");
    packet::array(pitches, 7);
    assert(packet::number(packet::field(json_object_array_get_idx(pitches, 0),
                                        "hertz")) == native.notes[0].hertz);
    // The actual sparse catalog has all physical addresses, while five source
    // keys have no target at any of their three touchpoints.
    const auto catalog = packet::field(root.get(), "native_catalog");
    packet::array(catalog, 36);
    std::array<unsigned, 12> copies{};
    unsigned available = 0;
    for (unsigned i = 0; i < 36; ++i) {
      auto cell = json_object_array_get_idx(catalog, i);
      const auto key = packet::byte(packet::field(cell, "key"));
      assert(key < 12);
      ++copies[key];
      const auto can_play = packet::boolean(packet::field(cell, "available"));
      if (can_play) {
        ++available;
        auto target = packet::note(packet::field(cell, "native_target"));
        assert(target.key == key && target.hertz > 0);
      } else {
        json_object *target = nullptr;
        assert(json_object_object_get_ex(cell, "native_target", &target) &&
               (!target || json_object_is_type(target, json_type_null)));
        assert(!packet::string(packet::field(cell, "reason")).empty());
      }
    }
    assert(available == 21 && std::all_of(copies.begin(), copies.end(),
                                          [](unsigned n) { return n == 3; }));
    ql::PhysicalSnapshot before{};
    assert(ql::write_physical_snapshot(*native.body, before, 1, 0));
    native.engine->enable_capture(true);
    Operation attack{};
    attack.identity = native.determination.identity;
    attack.kind = Kind::NoteOn;
    attack.sequence = 1;
    attack.sample = 37;
    attack.note = native.notes[0];
    attack.value = .8;
    assert(native.engine->enqueue(attack) == Result::Accepted);
    std::array<float, 128> pcm{};
    assert(native.engine->render(pcm.data(), 128, 0));
    Capture capture{};
    Readback reading{};
    assert(native.engine->pop_capture(capture) &&
           native.engine->pop_readback(reading));
    assert(std::any_of(capture.force_newtons.begin(),
                       capture.force_newtons.begin() + 128,
                       [](double x) { return x != 0; }));
    assert(std::any_of(capture.pickup_linear.begin(),
                       capture.pickup_linear.begin() + 128,
                       [](float x) { return x != 0; }));
    assert(reading.physical.node_count == 12 &&
           reading.samples_elapsed == 128 &&
           reading.physical.samples_elapsed == 128 &&
           native.body->samples_elapsed() == 128);
    bool moved = false;
    for (unsigned i = 0; i < 12; ++i) {
      assert(reading.physical.node_identity[i] == before.node_identity[i]);
      const auto a = before.visible_positions_metres[i],
                 b = reading.physical.visible_positions_metres[i];
      moved = moved || a.x != b.x || a.y != b.y || a.z != b.z;
    }
    assert(moved);
    assert(reading.physical.mechanical_energy_joules > 0);
    auto altered = fixture(argv[1]);
    auto changed = packet::field(altered.get(), "native_preparation");
    auto note = json_object_array_get_idx(packet::field(changed, "notes"), 0);
    json_object_object_add(note, "hertz",
                           json_object_new_double(native.notes[0].hertz + 1));
    bool refused = false;
    try {
      prepare_source_performance_packet(encoded(changed), current, basis, true,
                                        true);
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    refused = false;
    try {
      prepare_source_performance_packet(encoded(current), current, basis, false,
                                        true);
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    // Deliberately sever the waveform-to-body edge while preserving populated
    // actual producer/source targets: the SAME real P solver evolves zero
    // force.
    auto disconnected_body =
        std::make_shared<ql::PhysicalBody>(native.body->preparation());
    auto port = physical_port(disconnected_body);
    port.advance = [](void *owner, const double *, float *pickup,
                      std::size_t frames, std::uint64_t revision,
                      std::uint64_t start) noexcept {
      std::array<double, max_frames> zero{};
      return static_cast<ql::PhysicalBody *>(owner)->advance_force_block(
          zero.data(), pickup, frames, revision, start);
    };
    auto disconnected =
        std::make_shared<Engine>(native.determination, 48000, port);
    disconnected->enable_capture(true);
    assert(disconnected->enqueue(attack) == Result::Accepted);
    assert(disconnected->render(pcm.data(), 128, 0));
    assert(disconnected->pop_capture(capture));
    assert(std::any_of(capture.force_newtons.begin(),
                       capture.force_newtons.begin() + 128,
                       [](double x) { return x != 0; }));
    assert(std::all_of(capture.pickup_linear.begin(),
                       capture.pickup_linear.begin() + 128,
                       [](float x) { return x == 0; }));
    assert(disconnected_body->observation().mechanical_energy_joules == 0);
    std::cout << "{\"schema\":\"ql.source-performance-causal-regression/"
                 "v1\",\"nodes\":12,\"members\":34,\"physical_touchpoints\":36,"
                 "\"available\":21,\"unavailable\":15,\"cursor\":\"128\","
                 "\"scope\":\"actual native source-form/sparse M1-M2-P "
                 "in-memory; no device/GPU claim\"}\n";
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
