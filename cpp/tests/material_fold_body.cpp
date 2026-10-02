// Paired producer test. Execute with the actual Rust m3_material_fold fixture.
// The existing native Engine owns excitation/time; PhysicalBody owns q/v.
#include <cassert>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <ql/performance_packet.hpp>
#include <string>
#include <vector>
using namespace ql::performance;
using ql::physical_wire::J;
using ql::physical_wire::Json;
static void put(J *object, const char *key, J *value) {
  assert(value && json_object_object_add(object, key, value) == 0);
}
static void string(J *object, const char *key, const std::string &value) {
  put(object, key, json_object_new_string_len(value.data(), int(value.size())));
}
static void counter(J *object, const char *key, std::uint64_t value) {
  string(object, key, std::to_string(value));
}
static Json copy(J *source) {
  J *value = nullptr;
  assert(json_object_deep_copy(source, &value, nullptr) == 0);
  return ql::physical_wire::own(value);
}
static Json parse_file(const char *path) {
  std::ifstream input(path, std::ios::binary);
  if (!input)
    throw std::invalid_argument("actual native fixture absent");
  input.seekg(0, std::ios::end);
  const auto size = input.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("native fixture budget exceeded");
  std::string text(std::size_t(size), '\0');
  input.seekg(0);
  if (!input.read(text.data(), size))
    throw std::invalid_argument("incomplete native fixture");
  auto token = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  if (!token)
    throw std::bad_alloc();
  json_tokener_set_flags(token.get(),
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto value = ql::physical_wire::own(
      json_tokener_parse_ex(token.get(), text.data(), int(text.size())));
  const auto end = json_tokener_get_parse_end(token.get());
  if (json_tokener_get_error(token.get()) != json_tokener_success || !value ||
      text.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("strict complete native fixture required");
  return value;
}
static Json snapshot(const ql::PhysicalSnapshot &value) {
  auto out = ql::physical_wire::own(json_object_new_object());
  put(out.get(), "version", json_object_new_int(int(value.version)));
  string(out.get(), "event_ref", value.event_ref.data());
  string(out.get(), "subject_ref", value.subject_ref.data());
  string(out.get(), "preparation_ref", value.preparation_ref.data());
  string(out.get(), "state_ref", value.state_ref.data());
  string(out.get(), "source_coordinate", value.source_coordinate.data());
  string(out.get(), "source_revision", value.source_revision.data());
  string(out.get(), "geometry_ref", value.geometry_ref.data());
  string(out.get(), "geometry_revision", value.geometry_revision.data());
  string(out.get(), "material_ref", value.material_ref.data());
  string(out.get(), "material_revision", value.material_revision.data());
  string(out.get(), "eigenbasis_identity", value.eigenbasis_identity.data());
  put(out.get(), "source_generation",
      json_object_new_uint64(value.source_generation));
  put(out.get(), "body_revision", json_object_new_uint64(value.body_revision));
  counter(out.get(), "samples_elapsed", value.samples_elapsed);
  put(out.get(), "sample_rate", json_object_new_int(int(value.sample_rate)));
  put(out.get(), "node_count", json_object_new_int(int(value.node_count)));
  put(out.get(), "pratibimba", json_object_new_boolean(value.pratibimba));
  put(out.get(), "pickup_linear", json_object_new_double(value.pickup_linear));
  put(out.get(), "mechanical_energy_joules",
      json_object_new_double(value.mechanical_energy_joules));
  auto *ids = json_object_new_array(), *positions = json_object_new_array();
  for (unsigned i = 0; i < value.node_count; i++) {
    json_object_array_add(ids, json_object_new_uint64(value.node_identity[i]));
    auto *position = json_object_new_array();
    for (double axis : value.visible_positions_metres[i])
      json_object_array_add(position, json_object_new_double(axis));
    json_object_array_add(positions, position);
  }
  put(out.get(), "node_identity", ids);
  put(out.get(), "visible_positions_metres", positions);
  return out;
}
static void little(std::ofstream &out, std::uint32_t value, unsigned bytes) {
  for (unsigned i = 0; i < bytes; i++)
    out.put(char((value >> (8 * i)) & 255));
}
static void wav(const char *path, const std::vector<float> &pcm,
                unsigned rate) {
  static_assert(sizeof(float) == 4, "IEEE binary32 native PCM required");
  std::ofstream out(path, std::ios::binary);
  if (!out)
    throw std::invalid_argument("captured audio destination absent");
  out.write("RIFF", 4);
  little(out, std::uint32_t(pcm.size() * 4 + 36), 4);
  out.write("WAVEfmt ", 8);
  little(out, 16, 4);
  little(out, 3, 2);
  little(out, 1, 2);
  little(out, rate, 4);
  little(out, rate * 4, 4);
  little(out, 4, 2);
  little(out, 32, 2);
  out.write("data", 4);
  little(out, std::uint32_t(pcm.size() * 4), 4);
  for (float sample : pcm) {
    std::uint32_t bits;
    std::memcpy(&bits, &sample, 4);
    little(out, bits, 4);
  }
  if (!out)
    throw std::invalid_argument("captured audio write failed");
}
int main(int argc, char **argv) {
  if (argc != 4) {
    std::cerr
        << "UNEXECUTED: material_fold_body requires actual-native-fold.json "
           "observed-body.json captured-native.wav\n";
    return 2;
  }
  try {
    auto fixture = parse_file(argv[1]);
    auto *raw = packet::field(fixture.get(), "performance"),
         *basis = packet::field(fixture.get(), "native_basis");
    const auto text = std::string(
        json_object_to_json_string_ext(raw, JSON_C_TO_STRING_PLAIN));
    auto native = prepare_performance_packet(text, basis, true, true);
    assert(native.notes.size() == 12 &&
           native.body->preparation().input().nodes.size() == 12);
    auto *material = packet::field(fixture.get(), "body_material");
    assert(json_object_equal(packet::field(material, "body"),
                             packet::field(raw, "physical_body")));
    assert(json_object_equal(
        packet::field(packet::field(fixture.get(), "plan"), "native_state"),
        packet::field(basis, "m3")));
    auto altered = copy(raw);
    put(packet::field(altered.get(), "physical_body"), "source_generation",
        json_object_new_int(9000));
    bool refused = false;
    try {
      prepare_performance_packet(
          json_object_to_json_string_ext(altered.get(), JSON_C_TO_STRING_PLAIN),
          basis, true, true);
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    altered = copy(raw);
    put(packet::field(packet::field(altered.get(), "physical_body"),
                      "source_coordinate"),
        "face", json_object_new_string("bimba"));
    refused = false;
    try {
      prepare_performance_packet(
          json_object_to_json_string_ext(altered.get(), JSON_C_TO_STRING_PLAIN),
          basis, true, true);
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    ql::PhysicalSnapshot observed{};
    const auto revision = native.body->body_revision();
    assert(ql::write_physical_snapshot(*native.body, observed, revision, 0));
    auto output = ql::physical_wire::own(json_object_new_object());
    string(output.get(), "schema", "ql.m3-material-observation-fixture/v1");
    auto *snapshots = json_object_new_array();
    json_object_array_add(snapshots, snapshot(observed).release());
    const auto stopped = observed;
    assert(
        !ql::write_physical_snapshot(*native.body, observed, revision + 1, 0));
    assert(observed.visible_positions_metres ==
           stopped.visible_positions_metres);
    native.engine->enable_capture(true);
    Operation attack{};
    attack.kind = Kind::NoteOn;
    attack.identity = native.determination.identity;
    attack.sequence = 1;
    attack.note = native.notes[4];
    attack.value = .8;
    auto wrong = attack;
    wrong.identity.subject = reference("wrong:subject");
    assert(native.engine->enqueue(wrong) == Result::Stale);
    assert(native.engine->enqueue(attack) == Result::Accepted);
    assert(native.engine->enqueue(attack) == Result::Order);
    const auto rate = native.body->preparation().input().sample_rate;
    assert(rate == 48000);
    const unsigned frames = rate * 2;
    std::vector<float> pcm;
    pcm.reserve(frames);
    std::uint64_t nonzero = 0;
    double sum = 0, peak = 0, max_pickup_error = 0, max_visible_error = 0,
           max_force = 0;
    auto *capture_prefix = json_object_new_array(),
         *applications = json_object_new_array();
    const auto started = std::chrono::steady_clock::now();
    for (unsigned start = 0; start < frames; start += 128) {
      if (start == rate) {
        Operation release{};
        release.kind = Kind::NoteOff;
        release.identity = native.determination.identity;
        release.sequence = 2;
        release.touch = attack.note.touch;
        release.sample = start;
        assert(native.engine->enqueue(release) == Result::Accepted);
      }
      std::array<float, 128> block{};
      assert(native.engine->render(block.data(), 128, start));
      Capture capture{};
      Readback readback{};
      assert(native.engine->pop_capture(capture) &&
             native.engine->pop_readback(readback));
      assert(readback.available &&
             readback.physical.samples_elapsed == start + 128 &&
             readback.physical.body_revision == revision);
      assert(capture.start_sample == start && capture.frames == 128 &&
             capture.body_revision == revision);
      std::array<ql::Vec3, ql::physical_max_nodes> displacement{};
      assert(native.body->write_displacements(displacement.data(), 12, revision,
                                              start + 128));
      const auto &body = native.body->preparation().input();
      double pickup = 0;
      for (unsigned node = 0; node < 12; node++)
        for (unsigned axis = 0; axis < 3; axis++) {
          max_visible_error = std::max(
              max_visible_error,
              std::abs(readback.physical.visible_positions_metres[node][axis] -
                       body.nodes[node].rest_metres[axis] -
                       displacement[node][axis]));
          pickup += displacement[node][axis] * body.pickup.axis[axis] *
                    body.pickup.node_weights[node] *
                    body.pickup_linear_per_metre;
        }
      max_pickup_error = std::max(
          max_pickup_error, std::abs(pickup - capture.pickup_linear[127]));
      assert(max_visible_error < 1e-12 && max_pickup_error < 1e-6);
      if (start == 0 || start == 128 || start + 128 == 4096 ||
          start + 128 == frames)
        json_object_array_add(snapshots, snapshot(readback.physical).release());
      for (unsigned i = 0; i < 128; i++) {
        assert(std::isfinite(block[i]) && block[i] == capture.output_linear[i]);
        pcm.push_back(block[i]);
        nonzero += block[i] != 0;
        sum += double(block[i]) * block[i];
        peak = std::max(peak, std::abs(double(block[i])));
        max_force = std::max(max_force, std::abs(capture.force_newtons[i]));
        if (start < 256) {
          auto *item = json_object_new_array();
          json_object_array_add(
              item, json_object_new_double(capture.force_newtons[i]));
          json_object_array_add(
              item, json_object_new_double(capture.pickup_linear[i]));
          json_object_array_add(
              item, json_object_new_double(capture.output_linear[i]));
          json_object_array_add(capture_prefix, item);
        }
      }
      NativeGestureApplication applied{};
      while (native.engine->pop_gesture_application(applied)) {
        auto *item = json_object_new_object();
        counter(item, "admission_sequence", applied.sequence);
        counter(item, "admitted_sample", applied.admitted_sample);
        counter(item, "applied_sample", applied.applied_sample);
        counter(item, "committed_cursor", applied.committed_cursor);
        put(item, "applied", json_object_new_boolean(applied.applied));
        put(item, "body_revision",
            json_object_new_uint64(applied.body_revision));
        json_object_array_add(applications, item);
      }
      if (start == 0) {
        std::array<float, 128> rejected{};
        assert(!native.engine->render(rejected.data(), 128, 0));
        assert(native.body->samples_elapsed() == 128);
        const auto saved = readback.physical;
        observed = saved;
        assert(
            !ql::write_physical_snapshot(*native.body, observed, revision, 0));
        assert(observed.visible_positions_metres ==
               saved.visible_positions_metres);
      }
    }
    const auto elapsed = std::chrono::duration<double, std::milli>(
                             std::chrono::steady_clock::now() - started)
                             .count();
    assert(nonzero > 0 && peak > 0 && max_force > 0 &&
           native.body->samples_elapsed() == frames);
    wav(argv[3], pcm, rate);
    put(output.get(), "snapshots", snapshots);
    put(output.get(), "force_pickup_output_prefix", capture_prefix);
    put(output.get(), "applied_occurrences", applications);
    auto *measurement = json_object_new_object();
    put(measurement, "frames", json_object_new_int(int(frames)));
    put(measurement, "sample_rate", json_object_new_int(int(rate)));
    put(measurement, "nonzero_output_samples", json_object_new_uint64(nonzero));
    put(measurement, "pcm_peak", json_object_new_double(peak));
    put(measurement, "pcm_rms",
        json_object_new_double(std::sqrt(sum / frames)));
    put(measurement, "max_force_newtons", json_object_new_double(max_force));
    put(measurement, "max_visible_error_metres",
        json_object_new_double(max_visible_error));
    put(measurement, "max_pickup_error_linear",
        json_object_new_double(max_pickup_error));
    put(measurement, "component_elapsed_ms", json_object_new_double(elapsed));
    put(measurement, "negative_cases", json_object_new_int(7));
    put(output.get(), "measurements", measurement);
    string(output.get(), "audio_path", argv[3]);
    string(
        output.get(), "scope",
        "actual native source/form/performance producers to same PhysicalBody "
        "q-v/PCM and copied positions; no installed app/device acceptance");
    std::ofstream result(argv[2], std::ios::binary);
    result << json_object_to_json_string_ext(output.get(),
                                             JSON_C_TO_STRING_PRETTY)
           << '\n';
    if (!result)
      throw std::invalid_argument("observation fixture write failed");
    std::cout << "actual native material/performance/body q-v -> copied "
                 "positions and PCM passed; "
              << frames << " samples, peak " << peak << ", RMS "
              << std::sqrt(sum / frames) << '\n';
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
  return 0;
}
