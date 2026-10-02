#ifndef QL_PERFORMANCE_SOURCE_PACKET_HPP
#define QL_PERFORMANCE_SOURCE_PACKET_HPP
#include <ql/performance_packet.hpp>

namespace ql::performance {
// Current native Rust source preparation is supplied independently by the
// serial owner, with its actual B/K private collection and complete condition
// compiler replay. Neither a hash nor the candidate envelope grants authority.
inline NativePerformance prepare_source_performance_packet(
    const std::string &candidate, json_object *current_source_packet,
    json_object *actual_native_basis, bool admitted_m1_pratibimba,
    bool admitted_physical_pratibimba, Parameters parameters = {}) {
  if (!current_source_packet || candidate.empty() ||
      candidate.size() > 4 * 1024 * 1024)
    throw std::invalid_argument(
        "bounded independently current native source packet required");
  auto *tok = json_tokener_new_ex(64);
  if (!tok)
    throw std::bad_alloc();
  json_tokener_set_flags(tok, JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto raw = ql::physical_wire::own(
      json_tokener_parse_ex(tok, candidate.data(), int(candidate.size())));
  const auto error = json_tokener_get_error(tok);
  const auto end = json_tokener_get_parse_end(tok);
  json_tokener_free(tok);
  if (error != json_tokener_success || !raw ||
      candidate.find_first_not_of(" \r\n\t", end) != std::string::npos ||
      !json_object_equal(raw.get(), current_source_packet))
    throw std::invalid_argument(
        "sparse performance candidate differs from actual native producer");
  const auto admission = packet::field(raw.get(), "source_key_admission");
  if (packet::string(packet::field(admission, "schema")) !=
      "ql.source-performance-admission/v1")
    throw std::invalid_argument(
        "actual native sparse source admission required");
  const auto receipts = packet::field(admission, "touch_receipts");
  const auto notes = packet::field(raw.get(), "notes");
  if (!json_object_is_type(notes, json_type_array))
    throw std::invalid_argument("native available sparse notes required");
  const auto count = json_object_array_length(notes);
  packet::array(receipts, count);
  for (std::size_t i = 0; i < count; ++i) {
    auto receipt = json_object_array_get_idx(receipts, i);
    if (!packet::boolean(packet::field(receipt, "available")))
      throw std::invalid_argument(
          "unavailable sparse source cannot admit a native note");
    const auto source = packet::field(receipt, "native_target");
    const auto note = packet::note(json_object_array_get_idx(notes, i));
    if (packet::byte(packet::field(receipt, "key")) != note.key ||
        packet::number(packet::field(source, "hertz")) != note.hertz ||
        packet::ref(source, "touch_ref") != note.touch_ref)
      throw std::invalid_argument(
          "native note disconnected from its sparse source receipt");
  }
  // Existing consumer rechecks actual M1/M2 bus/current M3 and creates the
  // ordinary Engine/P once. No source-frequency override or second renderer.
  return prepare_performance_packet(candidate, actual_native_basis,
                                    admitted_m1_pratibimba,
                                    admitted_physical_pratibimba, parameters);
}
} // namespace ql::performance
#endif
