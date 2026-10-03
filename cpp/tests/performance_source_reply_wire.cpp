#include <cassert>
#include <iostream>
#include <memory>
#include <ql/performance_management_wire.hpp>
using namespace ql::performance;
int main() {
  try {
    std::string bytes;
    std::getline(std::cin, bytes);
    if (bytes.empty() || bytes.size() > 16 * 1024 * 1024)
      throw std::invalid_argument("actual source cases bound");
    auto token = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
        json_tokener_new_ex(128), json_tokener_free);
    if (!token)
      throw std::bad_alloc();
    json_tokener_set_flags(token.get(),
                           JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
    auto input = ql::physical_wire::own(
        json_tokener_parse_ex(token.get(), bytes.data(), int(bytes.size())));
    if (json_tokener_get_error(token.get()) != json_tokener_success ||
        json_tokener_get_parse_end(token.get()) != bytes.size())
      throw std::invalid_argument("actual source cases parser");
    ql::require(packet::string(packet::field(input.get(), "schema")) ==
                    "ql.source-reply-detecting-input/v1",
                "actual source cases schema");
    json_object *case_index = nullptr;
    const bool single =
        json_object_object_get_ex(input.get(), "case_index", &case_index);
    std::uint64_t selected_index = 0;
    json_object *cases = nullptr;
    if (single) {
      ql::physical_wire::keys(input.get(), {"schema", "case_index", "case"});
      selected_index = packet::integer(case_index);
      ql::require(
          selected_index < 6,
          "actual native original five plus genuine pose case index bound");
    } else {
      // Original full-five transport remains supported under the same bound.
      cases = packet::field(input.get(), "cases");
      packet::array(cases, 5);
    }
    auto output = checkpoint_transport::object();
    checkpoint_transport::text(output.get(), "schema",
                               "ql.actual-native-source-replies/v1");
    if (single)
      checkpoint_transport::put(output.get(), "case_index",
                                json_object_new_uint64(selected_index));
    auto replies = checkpoint_transport::array();
    for (std::size_t i = 0; i < (single ? 1 : 5); ++i) {
      management_transport::Control owner;
      auto reply = owner.execute(single ? packet::field(input.get(), "case")
                                        : json_object_array_get_idx(cases, i));
      assert(packet::boolean(packet::field(reply.get(), "accepted")));
      auto reading = packet::field(reply.get(), "reading");
      const auto rate = packet::integer(
          packet::field(packet::field(reading, "physical"), "sample_rate"));
      auto descriptors = packet::field(reading, "parameters");
      packet::array(descriptors, 7);
      bool cutoff = false;
      for (std::size_t j = 0; j < 7; ++j) {
        auto descriptor = json_object_array_get_idx(descriptors, j);
        assert(checkpoint_transport::decimal(packet::field(
                   descriptor, "sample_rate")) == std::uint64_t(rate));
        assert(packet::number(packet::field(descriptor, "smoothing_seconds")) ==
               parameter_smoothing_seconds);
        const auto time_samples = packet::number(
            packet::field(descriptor, "smoothing_time_constant_samples"));
        assert(time_samples == parameter_smoothing_seconds * rate);
        assert(checkpoint_transport::decimal(
                   packet::field(descriptor, "smoothing_samples")) ==
               std::uint64_t(std::ceil(time_samples)));
        assert(packet::number(
                   packet::field(descriptor, "smoothing_coefficient")) ==
               -std::expm1(-1.0 / time_samples));
        if (packet::string(packet::field(descriptor, "target_ref")) ==
            "ql:performance/parameter/cutoff-hertz") {
          cutoff = true;
          assert(packet::number(packet::field(descriptor, "maximum")) ==
                 .45 * rate);
        }
      }
      assert(cutoff);

      // This is the production worker management implementation and actual
      // stopped P/Engine snapshot, with no fabricated body/output receipt.
      checkpoint_transport::append(replies.get(), reply.release());
    }
    checkpoint_transport::put(output.get(), "replies", replies.release());
    std::cout << json_object_to_json_string_ext(output.get(),
                                                JSON_C_TO_STRING_PLAIN)
              << '\n';
  } catch (const std::exception &error) {
    std::cerr << error.what() << '\n';
    return 1;
  }
}
