#ifndef QL_TEST_NATIVE_FIXTURE_CARRIER_HPP
#define QL_TEST_NATIVE_FIXTURE_CARRIER_HPP
// Transient test carrier only. Parts contain literal original JSON; there is
// no shared mutable source, authentication grant or native runtime projection.
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <iomanip>
#include <ql/physical_body_wire.hpp>
#include <set>
#include <sstream>
#include <vector>
namespace ql_test_native_carrier {
using Json = ql::physical_wire::Json;
using J = json_object;
constexpr std::size_t wire_limit = 8 * 1024 * 1024;
constexpr std::size_t expanded_limit = 4 * wire_limit;
inline std::string text(J *value) {
  if (!value || json_object_get_type(value) != json_type_string)
    throw std::invalid_argument("native test text required");
  const auto size = std::size_t(json_object_get_string_len(value));
  if (size > wire_limit)
    throw std::invalid_argument("native test text byte bound");
  return {json_object_get_string(value), size};
}
inline bool eligible(const std::string &key) {
  return key == "native_basis" || key == "native_preparation" ||
         key == "performance_preparation" || key == "current_m3" ||
         key == "preparation";
}
inline std::string fingerprint(const std::string &bytes) {
  std::uint64_t hash = 0xcbf29ce484222325ULL;
  for (unsigned char byte : bytes)
    hash = (hash ^ byte) * 0x100000001b3ULL;
  std::ostringstream out;
  out << "fnv1a64:" << std::hex << std::setw(16) << std::setfill('0') << hash;
  return out.str();
}
inline Json parse(const std::string &bytes) {
  auto *token = json_tokener_new_ex(128);
  if (!token)
    throw std::bad_alloc();
  json_tokener_set_flags(token,
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  // json-c requires the terminal NUL for a complete number at known EOF.
  // The byte bounds and exact parse end still reject truncated/trailing data.
  if (bytes.size() > wire_limit || bytes.find('\0') != std::string::npos) {
    json_tokener_free(token);
    throw std::invalid_argument("native test literal byte bound/NUL refused");
  }
  auto out = ql::physical_wire::own(
      json_tokener_parse_ex(token, bytes.c_str(), int(bytes.size() + 1)));
  const auto error = json_tokener_get_error(token);
  const auto end = json_tokener_get_parse_end(token);
  json_tokener_free(token);
  if (error != json_tokener_success || end < bytes.size() ||
      end > bytes.size() + 1)
    throw std::invalid_argument("native test literal JSON refused");
  return out;
}
struct Decoder {
  std::vector<std::string> parts;
  std::vector<bool> used;
  std::size_t nodes = 0, references = 0, expanded_parts = 0;
  Json node(J *value, const std::string &key = {}, std::size_t depth = 0) {
    using namespace ql::physical_wire;
    if (++nodes > 1000000 || depth > 64)
      throw std::invalid_argument("native test tree bound");
    keys(value, {"kind", "value"});
    const auto kind = ql_test_native_carrier::text(field(value, "kind"));
    auto *content = field(value, "value");
    if (kind == "part") {
      if (!eligible(key) || ++references > 1024 ||
          json_object_get_type(content) != json_type_int ||
          json_object_get_int64(content) < 0)
        throw std::invalid_argument("native test part position/index refused");
      const auto index = json_object_get_uint64(content);
      if (index >= parts.size() ||
          parts[index].size() > expanded_limit - expanded_parts)
        throw std::invalid_argument("missing/oversized native test part");
      expanded_parts += parts[index].size();
      used[index] = true;
      auto out = parse(parts[index]);
      if (json_object_get_type(out.get()) != json_type_object)
        throw std::invalid_argument("native test composite is not an object");
      return out; // each occurrence is separately parsed, never aliased
    }
    if (kind == "literal") {
      // Native objects resembling carrier instructions remain literal data.
      const auto *bytes =
          json_object_to_json_string_ext(content, JSON_C_TO_STRING_PLAIN);
      return parse(bytes ? bytes : "null");
    }
    if (json_object_get_type(content) != json_type_array)
      throw std::invalid_argument("native test node array refused");
    const auto n = json_object_array_length(content);
    if (n > 1000000)
      throw std::invalid_argument("native test node count refused");
    if (kind == "array") {
      auto out = own(json_object_new_array());
      for (std::size_t i = 0; i < n; ++i)
        json_object_array_add(
            out.get(),
            node(json_object_array_get_idx(content, i), {}, depth + 1)
                .release());
      return out;
    }
    if (kind != "object")
      throw std::invalid_argument("unknown native test node kind");
    auto out = own(json_object_new_object());
    std::set<std::string> names;
    for (std::size_t i = 0; i < n; ++i) {
      auto *pair = json_object_array_get_idx(content, i);
      if (json_object_get_type(pair) != json_type_array ||
          json_object_array_length(pair) != 2)
        throw std::invalid_argument("native test key pair refused");
      const auto name =
          ql_test_native_carrier::text(json_object_array_get_idx(pair, 0));
      if (name.find('\0') != std::string::npos || !names.insert(name).second)
        throw std::invalid_argument("duplicate/native test key refused");
      json_object_object_add(
          out.get(), name.c_str(),
          node(json_object_array_get_idx(pair, 1), name, depth + 1).release());
    }
    return out;
  }
};
inline void qualify(J *value, std::size_t depth, std::size_t &nodes) {
  if (++nodes > 1000000 || depth > 64)
    throw std::invalid_argument("native test full literal tree bound");
  if (json_object_get_type(value) == json_type_object) {
    json_object_object_foreach(value, key, child) {
      (void)key;
      qualify(child, depth + 1, nodes);
    }
  } else if (json_object_get_type(value) == json_type_array) {
    for (std::size_t i = 0; i < json_object_array_length(value); ++i)
      qualify(json_object_array_get_idx(value, i), depth + 1, nodes);
  }
}
inline Json decode(J *carrier) {
  using namespace ql::physical_wire;
  keys(carrier, {"schema", "tree", "parts"});
  same_text(field(carrier, "schema"), "ql.test-native-fixture-carrier/v1");
  auto *parts = field(carrier, "parts");
  const auto n = count(parts, 128);
  if (!n)
    throw std::invalid_argument("native test dictionary empty");
  Decoder decoder;
  std::set<std::string> originals;
  std::size_t total = 0;
  for (std::size_t i = 0; i < n; ++i) {
    auto *part = at(parts, i);
    keys(part, {"original_json", "bytes", "fingerprint"});
    auto bytes = ql_test_native_carrier::text(field(part, "original_json"));
    auto *size = field(part, "bytes");
    if (json_object_get_type(size) != json_type_int ||
        json_object_get_int64(size) < 0 ||
        json_object_get_uint64(size) != bytes.size() ||
        bytes.size() > wire_limit - total || !originals.insert(bytes).second ||
        ql_test_native_carrier::text(field(part, "fingerprint")) !=
            fingerprint(bytes))
      throw std::invalid_argument(
          "native test part bytes/fingerprint/duplicate differs");
    total += bytes.size();
    decoder.parts.push_back(std::move(bytes));
  }
  decoder.used.resize(n, false);
  auto out = decoder.node(field(carrier, "tree"));
  if (std::find(decoder.used.begin(), decoder.used.end(), false) !=
      decoder.used.end())
    throw std::invalid_argument("unused native test part");
  std::size_t nodes = 0;
  qualify(out.get(), 0, nodes);
  if (std::strlen(json_object_to_json_string_ext(
          out.get(), JSON_C_TO_STRING_PLAIN)) > expanded_limit)
    throw std::invalid_argument("native test full expanded byte bound");
  return out;
}
} // namespace ql_test_native_carrier
#endif
