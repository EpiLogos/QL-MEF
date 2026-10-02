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
    auto cases = packet::field(input.get(), "cases");
    packet::array(cases, 5);
    auto output = checkpoint_transport::object();
    checkpoint_transport::text(output.get(), "schema",
                               "ql.actual-native-source-replies/v1");
    auto replies = checkpoint_transport::array();
    for (std::size_t i = 0; i < 5; ++i) {
      management_transport::Control owner;
      auto reply = owner.execute(json_object_array_get_idx(cases, i));
      assert(packet::boolean(packet::field(reply.get(), "accepted")));
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
