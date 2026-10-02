#ifndef QL_TEST_INDEPENDENT_GENUINE_CARRIER_SCALARS_HPP
#define QL_TEST_INDEPENDENT_GENUINE_CARRIER_SCALARS_HPP
// Call on the unchanged genuine Rust-produced carrier before the existing
// native consumer cases. No synthetic fixture, replacement consumer or parser.
#include <cassert>
#include <ql/physical_body_wire.hpp>
namespace ql_test_independent_carrier {
using J = json_object;
inline J *member_node(J *node, const char *name) {
  using namespace ql::physical_wire;
  assert(ql_test_native_carrier::text(field(node, "kind")) == "object");
  auto *pairs = field(node, "value");
  const auto count = json_object_array_length(pairs);
  for (std::size_t i = 0; i < count; ++i) {
    auto *pair = json_object_array_get_idx(pairs, i);
    assert(json_object_array_length(pair) == 2);
    if (ql_test_native_carrier::text(json_object_array_get_idx(pair, 0)) == name)
      return json_object_array_get_idx(pair, 1);
  }
  assert(false && "genuine native catalog field absent");
  return nullptr;
}
inline void genuine_scalar_roundtrip(J *carrier) {
  using namespace ql::physical_wire;
  auto *catalog_node = member_node(field(carrier, "tree"), "native_catalog");
  assert(ql_test_native_carrier::text(field(catalog_node, "kind")) == "array");
  auto *original_cells = field(catalog_node, "value");
  const auto cells = json_object_array_length(original_cells);
  assert(cells >= 12 && cells <= 4096);
  auto restored = ql_test_native_carrier::decode(carrier);
  auto *restored_cells = field(restored.get(), "native_catalog");
  assert(json_object_array_length(restored_cells) == cells);
  std::size_t numbers = 0, nulls = 0, booleans = 0;
  for (std::size_t i = 0; i < cells; ++i) {
    auto *original = json_object_array_get_idx(original_cells, i);
    auto *current = json_object_array_get_idx(restored_cells, i);
    for (const auto *name : {"row", "column", "key", "pitch_class",
                            "register_octave", "available", "source_degree",
                            "reason"}) {
      auto *literal = member_node(original, name);
      assert(ql_test_native_carrier::text(field(literal, "kind")) == "literal");
      auto *before = field(literal, "value");
      auto *after = field(current, name);
      assert(json_object_get_type(before) == json_object_get_type(after));
      if (!before) {
        assert(!after);
        ++nulls;
      } else {
        assert(json_object_equal(before, after));
        const auto type = json_object_get_type(before);
        numbers += type == json_type_int || type == json_type_double;
        booleans += type == json_type_boolean;
      }
    }
  }
  assert(numbers == cells * 5 && nulls == cells * 2 && booleans == cells);
}
} // namespace ql_test_independent_carrier
#endif
