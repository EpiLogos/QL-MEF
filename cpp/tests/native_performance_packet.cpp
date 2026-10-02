// Actual Rust producer packets feed actual C++ excitation and P body. Neither
// the independent basis fixture nor its receipt is host authentication.
#include <cassert>
#include <fstream>
#include <iostream>
#include <ql/performance_packet.hpp>
using namespace ql::performance;
static std::string read(const std::string &path) {
  std::ifstream stream(path, std::ios::binary);
  if (!stream)
    throw std::invalid_argument("missing source-qualified native fixture");
  stream.seekg(0, std::ios::end);
  const auto size = stream.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("native fixture budget exceeded");
  std::string out(std::size_t(size), '\0');
  stream.seekg(0);
  if (!stream.read(out.data(), size))
    throw std::invalid_argument("incomplete native fixture");
  return out;
}
static ql::physical_wire::Json parse(const std::string &text) {
  auto *token = json_tokener_new_ex(64);
  if (!token)
    throw std::bad_alloc();
  json_tokener_set_flags(token, JSON_TOKENER_STRICT);
  auto value = ql::physical_wire::own(
      json_tokener_parse_ex(token, text.data(), int(text.size())));
  const auto error = json_tokener_get_error(token);
  const auto end = json_tokener_get_parse_end(token);
  json_tokener_free(token);
  if (error != json_tokener_success || !value ||
      text.find_first_not_of(" \t\n\r", end) != std::string::npos)
    throw std::invalid_argument("strict complete native fixture required");
  return value;
}
int main(int argc, char **argv) {
  if (argc != 2) {
    std::cerr << "usage: native_performance_packet "
                 "SOURCE_QUALIFIED_FIXTURE_DIRECTORY\n";
    return 2;
  }
  try {
    const std::string directory = argv[1];
    const auto baseline_text = read(directory + "/baseline.packet.json"),
               changed_text = read(directory + "/changed.packet.json");
    auto baseline_basis = parse(read(directory + "/baseline.basis.json")),
         changed_basis = parse(read(directory + "/changed.basis.json"));
    auto baseline = prepare_performance_packet(
        baseline_text, baseline_basis.get(), true, true);
    auto changed = prepare_performance_packet(changed_text, changed_basis.get(),
                                              true, true);
    assert(baseline.notes.size() == 12 && changed.notes.size() == 12);
    for (std::size_t i = 0; i < 12; ++i)
      assert(baseline.notes[i].hertz == changed.notes[i].hertz);
    assert(baseline.body->preparation().eigenbasis_identity() ==
           changed.body->preparation().eigenbasis_identity());
    assert(baseline.determination.audio_octet_hz !=
           changed.determination.audio_octet_hz);
    for (auto *session : {&baseline, &changed}) {
      session->engine->enable_capture(true);
      Operation attack{};
      attack.kind = Kind::NoteOn;
      attack.identity = session->determination.identity;
      attack.sequence = 1;
      attack.note = session->notes[4];
      attack.value = 0.8;
      assert(session->engine->enqueue(attack) == Result::Accepted);
    }
    bool force_diff = false, pcm_diff = false;
    std::uint64_t nonzero = 0;
    for (unsigned start = 0; start < 4096; start += 128) {
      std::array<float, 128> a{}, b{};
      assert(baseline.engine->render(a.data(), 128, start));
      assert(changed.engine->render(b.data(), 128, start));
      Capture ca{}, cb{};
      assert(baseline.engine->pop_capture(ca) &&
             changed.engine->pop_capture(cb));
      Readback ra{}, rb{};
      assert(baseline.engine->pop_readback(ra) &&
             changed.engine->pop_readback(rb));
      assert(ra.physical.samples_elapsed == start + 128 &&
             rb.physical.samples_elapsed == start + 128);
      assert(ra.physical.body_revision ==
                 baseline.determination.body_revision &&
             rb.physical.body_revision == changed.determination.body_revision);
      assert(std::abs(ra.physical.visible_positions_metres[1][0] - 1 -
                      ca.pickup_linear[127] / 1000) < 1e-10);
      for (unsigned i = 0; i < 128; ++i) {
        force_diff |= ca.force_newtons[i] != cb.force_newtons[i];
        pcm_diff |= ca.pickup_linear[i] != cb.pickup_linear[i];
        nonzero += a[i] != 0;
      }
    }
    assert(force_diff && pcm_diff && nonzero);
    // Populated envelope severed from independently retained actual native
    // basis is refused, even with all schema/source labels still present.
    auto detached = parse(baseline_text);
    auto *values = packet::field(packet::field(detached.get(), "determination"),
                                 "audio_octet_hz");
    json_object_array_put_idx(
        values, 0,
        json_object_new_double(baseline.determination.audio_octet_hz[0] *
                               1.01));
    bool refused = false;
    try {
      prepare_performance_packet(json_object_to_json_string_ext(
                                     detached.get(), JSON_C_TO_STRING_PLAIN),
                                 baseline_basis.get(), true, true);
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    refused = false;
    try {
      prepare_performance_packet(changed_text, baseline_basis.get(), true,
                                 true);
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    std::cout << "actual M1/K/M2/P packet -> Newton force -> same-state "
                 "PCM/positions passed; detached populated bus refused\n";
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
