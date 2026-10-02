#include <cassert>
#include <iostream>
#include <iterator>
#include <ql/performance_receiving_admission.hpp>
using namespace ql::performance;
using namespace ql::physical_wire;
static NativePerformance native(json_object *bundle) {
  auto *prepared = field(bundle, "native_preparation");
  const auto *serialized =
      json_object_to_json_string_ext(prepared, JSON_C_TO_STRING_PLAIN);
  return prepare_performance_packet(serialized, field(bundle, "native_basis"),
                                    true, true);
}
template <class F> static void refused(F operation) {
  bool rejected = false;
  try {
    operation();
  } catch (const std::invalid_argument &) {
    rejected = true;
  }
  assert(rejected);
}
int main() {
  const std::string bytes((std::istreambuf_iterator<char>(std::cin)), {});
  assert(!bytes.empty() && bytes.size() < 16 * 1024 * 1024);
  auto tokener = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  json_tokener_set_flags(tokener.get(),
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto root = own(
      json_tokener_parse_ex(tokener.get(), bytes.data(), int(bytes.size())));
  assert(json_tokener_get_error(tokener.get()) == json_tokener_success &&
         json_tokener_get_parse_end(tokener.get()) == bytes.size());
  auto *baseline = field(root.get(), "baseline");
  auto resident = native(baseline);
  auto admitted = read_native_receiving_admission(
      baseline, baseline, resident, field(baseline, "native_basis"),
      resident.body->preparation(), {}, 0);
  assert(admitted.matches_current(resident.determination,
                                  field(baseline, "native_basis")) &&
         admitted.routes()->routes.route_count() == 0);
  auto *alternatives = field(root.get(), "alternatives");
  assert(count(alternatives, 3) == 3);
  for (std::size_t i = 0; i < 3; ++i) {
    auto *alternative = at(alternatives, i);
    auto *bundle = field(alternative, "admission");
    auto other = native(bundle); // Real alternative M1/M2/K/P native consumers.
    assert(other.determination.identity == resident.determination.identity);
    refused([&] {
      read_native_receiving_admission(bundle, baseline, resident,
                                      field(baseline, "native_basis"),
                                      resident.body->preparation(), {}, 0);
    });
    // Even replacing both candidate/current N bundles does not qualify them
    // against the original resident audio producer. Pose changes are tested
    // through original N recompile + full source currentness on Rust above.
    // Old typed admission is stale after EVERY actual full native basis
    // change, including a valid same-generation pose with unchanged D.
    assert(!admitted.matches_current(other.determination,
                                     field(bundle, "native_basis")));
    refused([&] {
      read_native_receiving_admission(bundle, bundle, resident,
                                      field(baseline, "native_basis"),
                                      resident.body->preparation(), {}, 0);
    });
    auto valid = read_native_receiving_admission(
        bundle, bundle, other, field(bundle, "native_basis"),
        other.body->preparation(), {}, 0);
    assert(valid.matches_current(other.determination,
                                 field(bundle, "native_basis")));
  }
  std::cout << "actual-native-receiving-admission valid=4 disconnected=6 "
               "source-replay=Rust\n";
}
