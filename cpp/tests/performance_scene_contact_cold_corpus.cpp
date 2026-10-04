// Detect the real private Scene/Act producer's original full cold requests.
// Imported numerical evidence never supplies a witness or activates P here.
#include "../src/native_scene_contact_replay_channel.hpp"
#include <cassert>
#include <fstream>
#include <iostream>
namespace sc = ql::performance::scene_contact_transport;
using J = json_object;
using Json = ql::physical_wire::Json;
static Json parse(const std::string &bytes) {
  ql::require(!bytes.empty() && bytes.size() <= 32 * 1024 * 1024,
              "complete original Contact corpus exceeds worker bound");
  auto tok = std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
      json_tokener_new_ex(64), json_tokener_free);
  ql::require(bool(tok), "actual native corpus parser absent");
  json_tokener_set_flags(tok.get(),
                         JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
  auto out = sc::wire::own(
      json_tokener_parse_ex(tok.get(), bytes.data(), int(bytes.size())));
  ql::require(out &&
                  json_tokener_get_error(tok.get()) == json_tokener_success &&
                  json_tokener_get_parse_end(tok.get()) == bytes.size(),
              "original actual Contact corpus is incomplete");
  return out;
}
static Json copy(J *value) {
  J *out = nullptr;
  ql::require(json_object_deep_copy(value, &out, nullptr) == 0 && out,
              "bounded native corpus copy failed");
  return sc::wire::own(out);
}
template <class F> static void refusal(J *request, F mutate) {
  auto changed = copy(request);
  mutate(changed.get());
  bool refused = false;
  try {
    (void)sc::verify_replay(changed.get());
  } catch (const std::invalid_argument &) {
    refused = true;
  }
  assert(refused);
}
static J *record(J *request) {
  return sc::packet::field(
      json_object_array_get_idx(sc::packet::field(request, "rows"), 0),
      "source");
}
static void replace_wire(J *request, J *wire) {
  sc::wire::text(request, "checkpoint_wire",
                 std::string(json_object_to_json_string_ext(
                     wire, JSON_C_TO_STRING_PLAIN)));
}
int main(int argc, char **argv) {
  ql::require(
      argc == 2,
      "genuine original C Scene/Act contact corpus required; no fallback");
  std::ifstream input(argv[1], std::ios::binary);
  ql::require(bool(input), "actual private Contact corpus absent");
  input.seekg(0, std::ios::end);
  const auto bytes = input.tellg();
  ql::require(bytes > 0 && bytes <= 32 * 1024 * 1024,
              "actual Contact corpus bound differs");
  input.seekg(0);
  std::string text(std::size_t(bytes), '\0');
  ql::require(bool(input.read(text.data(), bytes)),
              "actual Contact corpus truncated");
  auto corpus = parse(text);
  ql::require(sc::packet::string(sc::packet::field(corpus.get(), "schema")) ==
                  "ql.actual-native-contact-cold-replay/v1",
              "actual closed owner contact producer required");
  auto *trials = sc::packet::field(corpus.get(), "trials");
  ql::require(json_object_is_type(trials, json_type_array),
              "actual native contact trial array absent");
  bool pending = false, active = false;
  std::size_t force_refusals = 0;
  for (std::size_t trial = 0; trial < json_object_array_length(trials);
       ++trial) {
    auto *row = json_object_array_get_idx(trials, trial);
    const auto cut = sc::packet::string(sc::packet::field(row, "cut"));
    ql::require(cut == "pending" || cut == "active",
                "actual pending/active contact cut required");
    pending |= cut == "pending";
    active |= cut == "active";
    auto *request = sc::packet::field(row, "original_request");
    auto repeated = sc::verify_replay(request);
    sc::require_exact(
        repeated.get(), sc::packet::field(row, "original_reply"),
        "independent full original native Contact replay differs");
    ql::require(json_object_array_length(sc::packet::field(request, "rows")) >
                    0,
                "actual original Contact programme absent");
    for (std::size_t sample = 0; sample < ql::physical_max_frames; ++sample) {
      refusal(request, [&](J *changed) {
        auto *force = sc::packet::field(record(changed), "force_newtons");
        const double actual =
            sc::packet::number(json_object_array_get_idx(force, sample));
        json_object_array_put_idx(
            force, sample,
            json_object_new_double(std::nextafter(actual, INFINITY)));
      });
      ++force_refusals;
    }
    refusal(request, [](J *changed) {
      auto *force = sc::packet::field(record(changed), "force_newtons");
      json_object_array_del_idx(force, ql::physical_max_frames - 1, 1);
    });
    refusal(request, [](J *changed) {
      auto *body = sc::packet::field(record(changed), "original_body");
      sc::wire::flag(
          body, "pratibimba",
          !sc::packet::boolean(sc::packet::field(body, "pratibimba")));
    });
    refusal(request, [](J *changed) {
      auto *nodes = sc::packet::field(
          sc::packet::field(record(changed), "original_body"), "nodes");
      json_object_array_del_idx(nodes, json_object_array_length(nodes) - 1, 1);
    });
    refusal(request, [](J *changed) {
      sc::wire::text(
          sc::packet::field(record(changed), "original_gravity_input"),
          "route_ref", "unqualified:disconnected-route");
    });
    refusal(request, [](J *changed) {
      auto *operands = sc::packet::field(record(changed), "native_operands");
      const auto original =
          sc::wire::decimal(sc::packet::field(operands, "impact_sample"));
      const auto different =
          original == UINT64_MAX ? original - 1 : original + 1;
      sc::wire::u64(operands, "impact_sample", different);
    });
    refusal(request, [](J *changed) {
      auto wire = parse(ql::performance::management_transport::checkpoint_text(
          sc::packet::field(changed, "checkpoint_wire")));
      auto *contact = sc::packet::field(
          sc::packet::field(sc::packet::field(wire.get(), "native_pair"),
                            "audio"),
          "contacts");
      sc::wire::u64(contact, "original_request_high_water", 0);
      replace_wire(changed, wire.get());
    });
    refusal(request, [](J *changed) {
      auto wire = parse(ql::performance::management_transport::checkpoint_text(
          sc::packet::field(changed, "checkpoint_wire")));
      auto *physical = sc::packet::field(
          sc::packet::field(wire.get(), "native_pair"), "physical");
      sc::wire::text(sc::packet::field(physical, "basis"),
                     "eigenbasis_identity", "unqualified:stale-eigenbasis");
      replace_wire(changed, wire.get());
    });
    refusal(request, [](J *changed) {
      auto *rows = sc::packet::field(changed, "rows");
      json_object_array_del_idx(rows, json_object_array_length(rows) - 1, 1);
    });
    // Independently repeat the complete original numerical activity after
    // every mutation cohort. Original source and checkpoint bytes remain held.
    auto again = sc::verify_replay(request);
    sc::require_exact(again.get(), repeated.get(),
                      "original contact activity changed after refusals");
  }
  ql::require(pending && active,
              "complete real pending+active native Contact corpus required");
  std::cout << "native-contact-cold-corpus PASS original pending+active; "
               "force_refusals="
            << force_refusals << "\n";
}
