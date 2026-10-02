// V/#293: actual Rust producer and actual C++ admission, no substitute engine.
#include <cassert>
#include <fstream>
#include <iostream>
#include <utility>
#include <ql/performance_packet.hpp>
using namespace ql::performance;
static std::string file(const std::string &path) {
  std::ifstream in(path, std::ios::binary);
  if (!in) throw std::invalid_argument("actual native producer fixture absent");
  in.seekg(0, std::ios::end);
  const auto size = in.tellg();
  if (size < 1 || size > 4 * 1024 * 1024)
    throw std::invalid_argument("actual producer fixture bounds differ");
  std::string bytes(std::size_t(size), '\0');
  in.seekg(0);
  if (!in.read(bytes.data(), size)) throw std::invalid_argument("actual fixture read refused");
  return bytes;
}
static ql::physical_wire::Json parse(const std::string &bytes) {
  auto *token = json_tokener_new_ex(64);
  if (!token) throw std::bad_alloc();
  json_tokener_set_flags(token, JSON_TOKENER_STRICT);
  auto out = ql::physical_wire::own(json_tokener_parse_ex(token, bytes.data(), int(bytes.size())));
  const auto error = json_tokener_get_error(token);
  const auto end = json_tokener_get_parse_end(token);
  json_tokener_free(token);
  if (error != json_tokener_success || !out || bytes.find_first_not_of(" \r\n\t", end) != std::string::npos)
    throw std::invalid_argument("actual complete producer JSON required");
  return out;
}
int main(int argc, char **argv) {
  try {
    if (argc != 2) throw std::invalid_argument("actual native producer fixture directory required");
    const std::string dir = argv[1];
    auto basis = parse(file(dir + "/baseline.basis.json"));
    const auto original = file(dir + "/baseline.packet.json");
    // Establish the real native M1/M2/K/P baseline before every source cut.
    auto admitted = prepare_performance_packet(original, basis.get(), true, true);
    assert(admitted.engine && admitted.body && admitted.notes.size() == 12);
    for (const auto &cut : {std::pair{"tick12", 12u}, {"degree720", 720u}}) {
      auto poisoned = parse(original);
      auto *determination = packet::field(poisoned.get(), "determination");
      const auto prior = packet::integer(packet::field(determination, cut.first));
      const auto changed = (prior + 1) % cut.second;
      assert(changed != prior);
      // Same authentic producer basis, event, revision, branch, bus, body and
      // notes; only the exported A clock operand loses its native M1 source.
      json_object_object_add(determination, cut.first, json_object_new_uint64(changed));
      const std::string bytes = json_object_to_json_string_ext(poisoned.get(), JSON_C_TO_STRING_PLAIN);
      bool refused = false;
      try { (void)prepare_performance_packet(bytes, basis.get(), true, true); }
      catch (const std::invalid_argument &) { refused = true; }
      assert(refused && "A admission accepted a clock detached from actual native M1 producer");
    }
    // The original exact producer remains usable after refused attempts.
    auto repeated = prepare_performance_packet(original, basis.get(), true, true);
    assert(repeated.determination.tick12 == admitted.determination.tick12 &&
           repeated.determination.degree720 == admitted.determination.degree720);
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
