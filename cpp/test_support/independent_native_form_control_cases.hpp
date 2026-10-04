// Actual retained Control consumer of the complete genuine Rust Form fixture.
// Metric receiver magnitudes are an explicit architecture experiment; this
// native component activity does not issue private C/Scene/Act permission.
#include <ql/performance_management_wire.hpp>
namespace ql_test_native_form_control {
namespace wire = ql::performance::checkpoint_transport;
namespace packet = ql::performance::packet;
using namespace ql::performance;
using J = json_object;
using Json = wire::Json;
inline Json copy(J *in) {
  return ql::physical_wire::parse_native(
      json_object_to_json_string_ext(in, JSON_C_TO_STRING_PLAIN));
}
struct Trial {
  management_transport::Control control;
  J *fixture, *current;
  Json last = wire::own(nullptr), acoustic = wire::own(nullptr);
  explicit Trial(J *root, const char *branch = "before")
      : fixture(root), current(packet::field(root, branch)) {
    auto *p = packet::field(current, "performance_preparation");
    auto request = wire::object();
    wire::text(request.get(), "schema", "ql.performance-control/v1");
    wire::text(request.get(), "operation", "prepare");
    wire::text(request.get(), "session_ref", "native:form-control/same-owner");
    for (const char *name : {"packet", "current_source_packet"})
      wire::put(request.get(), name, json_object_get(p));
    wire::put(request.get(), "actual_native_basis",
              json_object_get(packet::field(p, "native_basis")));
    auto *d = packet::field(p, "determination");
    wire::flag(request.get(), "m1_pratibimba",
               packet::integer(packet::field(d, "m1_face")) == 1);
    auto *body = packet::field(p, "physical_body");
    wire::flag(
        request.get(), "physical_pratibimba",
        packet::string(packet::field(packet::field(body, "source_coordinate"),
                                     "face")) == "pratibimba");
    wire::put(request.get(), "body_source", standing(p).release());
    wire::put(request.get(), "receiving_admission",
              json_object_get(packet::field(current, "native_admission")));
    wire::put(
        request.get(), "current_receiving_admission",
        json_object_get(packet::field(current, "current_native_admission")));
    last = control.execute(request.get());
    assert(packet::boolean(packet::field(last.get(), "accepted")));
    auto catalog = command("catalog");
    wire::put(catalog.get(), "cells",
              json_object_get(packet::field(current, "native_catalog")));
    wire::put(catalog.get(), "transpose", json_object_new_int(0));
    apply(catalog.get());
    acoustic = acoustic_packet(current);
    auto install = command("receiving-transport-install");
    wire::put(install.get(), "prepared_acoustic",
              json_object_get(acoustic.get()));
    wire::put(install.get(), "current_acoustic",
              json_object_get(acoustic.get()));
    wire::u64(install.get(), "expected_sample", 0);
    apply(install.get());
  }
  static Json standing(J *p) {
    auto out = wire::object();
    wire::text(out.get(), "kind", "sourceForm");
    wire::text(
        out.get(), "recipe_ref",
        packet::string(packet::field(
            packet::field(packet::field(p, "source_form_recipe"), "provenance"),
            "reference")));
    wire::u64(out.get(), "validated_m3_generation",
              packet::integer(packet::field(packet::field(p, "physical_body"),
                                            "source_generation")));
    return out;
  }
  Json command(const char *name) {
    auto out = wire::object();
    auto *r = packet::field(last.get(), "reading");
    auto *d = packet::field(packet::field(current, "performance_preparation"),
                            "determination");
    wire::text(out.get(), "schema", "ql.performance-control/v1");
    wire::text(out.get(), "operation", name);
    wire::text(out.get(), "session_ref", "native:form-control/same-owner");
    wire::put(out.get(), "expected_transport_epoch",
              json_object_get(packet::field(r, "transport_epoch")));
    wire::put(out.get(), "expected_source",
              json_object_get(packet::field(d, "identity")));
    wire::put(out.get(), "expected_body_revision",
              json_object_get(
                  packet::field(packet::field(r, "scope"), "body_revision")));
    return out;
  }
  void apply(J *request) {
    last = control.execute(request);
    assert(packet::boolean(packet::field(last.get(), "accepted")));
  }
  Json checkpoint() {
    auto request = command("checkpoint");
    apply(request.get());
    return copy(
        packet::field(packet::field(last.get(), "payload"), "checkpoint"));
  }
  void restore(J *saved) {
    auto request = command("restore");
    wire::put(request.get(), "checkpoint", json_object_get(saved));
    wire::u64(request.get(), "expected_cursor", 0);
    wire::text(request.get(), "transaction_ref",
               "native:form-control/cold-reopen");
    wire::text(request.get(), "checkpoint_ref",
               "native:form-control/original-after512");
    apply(request.get());
    auto catalog = command("catalog");
    wire::put(catalog.get(), "cells",
              json_object_get(packet::field(current, "native_catalog")));
    wire::put(catalog.get(), "transpose", json_object_new_int(0));
    apply(catalog.get());
  }
  Json acoustic_packet(J *source) {
    const auto prepared =
        ql::PreparedPhysicalBody(ql::physical_wire::read_prepared_physical_body(
            packet::field(source, "preparation"),
            packet::field(source, "current_m3"), true));
    auto out = wire::object();
    wire::text(out.get(), "schema",
               "ql.native-acoustic-receiving-preparation/v1");
    wire::put(out.get(), "source_body",
              json_object_get(packet::field(source, "preparation")));
    auto *context =
        packet::field(packet::field(packet::field(source, "native_admission"),
                                    "receiving_definition"),
                      "context");
    wire::put(out.get(), "context", json_object_get(context));
    auto config = wire::object();
    wire::text(config.get(), "schema",
               "ql.native-acoustic-receiving-configuration/v1");
    for (auto item :
         {std::make_pair("source_ref", "native:form-control/pickup"),
          {"source_motion_ref", "native:form-control/emitter"},
          {"receiver_motion_ref", "native:form-control/receiver"},
          {"policy_ref", "native:form-control/metric-law"},
          {"policy_revision", "1"},
          {"standing", "architecture-model"}})
      wire::text(config.get(), item.first, item.second);
    wire::put(config.get(), "revision", json_object_new_int(1));
    auto xyz = [](std::array<double, 3> values) {
      auto row = wire::array();
      for (double value : values)
        wire::append(row.get(), json_object_new_double(value));
      return row;
    };
    wire::put(config.get(), "source_translation_metres",
              xyz({0., 0., 0.}).release());
    const auto &in = prepared.input();
    std::array<double, 3> point{};
    for (std::size_t i = 0; i < in.nodes.size(); ++i)
      for (unsigned a = 0; a < 3; ++a)
        point[a] += in.nodes[i].rest_metres[a] * in.pickup.node_weights[i];
    auto *initial = packet::field(fixture, "before");
    const ql::PreparedPhysicalBody initial_preparation(
        ql::physical_wire::read_prepared_physical_body(
            packet::field(initial, "preparation"),
            packet::field(initial, "current_m3"), true));
    const auto &initial_input = initial_preparation.input();
    std::array<double, 3> receiver{};
    for (std::size_t i = 0; i < initial_input.nodes.size(); ++i)
      for (unsigned a = 0; a < 3; ++a)
        receiver[a] += initial_input.nodes[i].rest_metres[a] *
                       initial_input.pickup.node_weights[i];
    receiver[0] += .37;
    // The original receiver position remains fixed across actual body turns.
    if (acoustic) {
      auto *oldconfig = packet::field(acoustic.get(), "configuration");
      wire::put(config.get(), "receiver_position_metres",
                json_object_get(
                    packet::field(oldconfig, "receiver_position_metres")));
    } else
      wire::put(config.get(), "receiver_position_metres",
                xyz(receiver).release());
    wire::put(config.get(), "receiver_forward", xyz({-1., 0., 0.}).release());
    wire::put(config.get(), "source_velocity_metres_per_second",
              xyz({0., 0., 0.}).release());
    wire::put(config.get(), "receiver_velocity_metres_per_second",
              xyz({0., 0., 0.}).release());
    wire::real(config.get(), "speed_metres_per_second", 340.);
    wire::real(config.get(), "minimum_distance_metres", .001);
    wire::text(config.get(), "directivity", "omnidirectional");
    wire::flag(config.get(), "propagation_delay", true);
    wire::put(config.get(), "span_samples", json_object_new_uint64(480000));
    wire::put(out.get(), "configuration", config.release());
    wire::put(out.get(), "source_position_metres", xyz(point).release());
    wire::u64(out.get(), "origin_sample", 0);
    wire::u64(out.get(), "end_sample", 480000);
    wire::u64(out.get(), "history_origin_sample", 0);
    auto units = wire::object();
    for (auto item : {std::make_pair("distance", "m"),
                      {"velocity", "m/s"},
                      {"cursor", "native-audio-sample"},
                      {"signal", "linear-pickup"}})
      wire::text(units.get(), item.first, item.second);
    wire::put(out.get(), "units", units.release());
    return out;
  }
  void score(Operation op, const char *input = nullptr) {
    auto request = command("score");
    wire::put(request.get(), "event", wire::operation(op).release());
    if (input)
      wire::text(request.get(), "input_ref", input);
    else
      wire::put_null(request.get(), "input_ref");
    apply(request.get());
  }
  void seed() {
    auto *p = packet::field(current, "performance_preparation");
    auto identity = packet::identity(
        packet::field(packet::field(p, "determination"), "identity"));
    for (std::uint64_t i = 0; i < 2; ++i) {
      Operation op{};
      op.kind = Kind::NoteOn;
      op.identity = identity;
      op.sequence = i + 1;
      op.note =
          packet::note(json_object_array_get_idx(packet::field(p, "notes"), i));
      op.value = .75;
      score(op,
            i ? "native:form-control/touch2" : "native:form-control/touch1");
    }
    Operation parameter{};
    parameter.kind = Kind::Parameter;
    parameter.identity = identity;
    parameter.sequence = 3;
    parameter.sample = 600;
    parameter.parameter = Parameter::MasterLinear;
    parameter.value = .25;
    score(parameter);
    Operation release{};
    release.kind = Kind::NoteOff;
    release.identity = identity;
    release.sequence = 4;
    release.sample = 900;
    release.touch =
        packet::note(json_object_array_get_idx(packet::field(p, "notes"), 0))
            .touch;
    score(release, "native:form-control/touch1");
  }
  std::vector<double> render(unsigned frames) {
    auto request = command("offline-render");
    auto scope = wire::object();
    wire::text(scope.get(), "schema", "ql.native-offline-render-scope/v1");
    for (auto item :
         {std::make_pair("session_ref", "native:form-control/same-owner"),
          {"scene_ref", "native:form-control/component"},
          {"performance_revision", "1"},
          {"basis_seal", "native:form-control/actual-basis"},
          {"event_prefix_seal", "native:form-control/actual-prefix"},
          {"checkpoint_ref", "native:form-control/checkpoint"}})
      wire::text(scope.get(), item.first, item.second);
    wire::text(scope.get(), "performance_digest",
               packet::string(packet::field(fixture, "probe_scope_digest")));
    auto *r = packet::field(last.get(), "reading"),
         *d = packet::field(packet::field(current, "performance_preparation"),
                            "determination");
    wire::put(scope.get(), "expected_source",
              json_object_get(packet::field(d, "identity")));
    wire::put(scope.get(), "expected_body_revision",
              json_object_get(
                  packet::field(packet::field(r, "scope"), "body_revision")));
    wire::put(scope.get(), "expected_cursor",
              json_object_get(packet::field(r, "samples_elapsed")));
    wire::put(scope.get(), "expected_accepted_sequence",
              json_object_get(packet::field(r, "accepted_sequence")));
    wire::put(request.get(), "scope", scope.release());
    wire::put(request.get(), "frames", json_object_new_uint64(frames));
    apply(request.get());
    auto *pcm = packet::field(
        packet::field(packet::field(last.get(), "payload"), "chunk"),
        "interleaved_f32");
    assert(json_object_array_length(pcm) == frames);
    std::vector<double> out;
    for (unsigned i = 0; i < frames; ++i)
      out.push_back(packet::number(json_object_array_get_idx(pcm, i)));
    return out;
  }
  Json transition() {
    auto *after = packet::field(fixture, "after");
    auto request = command("source-body-transition");
    wire::u64(request.get(), "original_request_id", 9);
    wire::u64(request.get(), "expected_sample", 512);
    wire::text(request.get(), "kind", "form");
    wire::text(request.get(), "cause_ref",
               "native:form-control/actual-m3-change");
    auto *before_packet = packet::field(current, "performance_preparation"),
         *after_packet = packet::field(after, "performance_preparation");
    wire::put(request.get(), "before_packet", json_object_get(before_packet));
    wire::put(request.get(), "before_native_basis",
              json_object_get(packet::field(before_packet, "native_basis")));
    for (const char *name : {"after_packet", "actual_after_packet"})
      wire::put(request.get(), name, json_object_get(after_packet));
    wire::put(request.get(), "actual_after_native_basis",
              json_object_get(packet::field(after_packet, "native_basis")));
    wire::put(request.get(), "receiving_admission",
              json_object_get(packet::field(after, "native_admission")));
    wire::put(
        request.get(), "current_receiving_admission",
        json_object_get(packet::field(after, "current_native_admission")));
    wire::put(request.get(), "native_catalog",
              json_object_get(packet::field(after, "native_catalog")));
    wire::put(request.get(), "body_source", standing(after_packet).release());
    wire::put(request.get(), "before_acoustic",
              json_object_get(acoustic.get()));
    auto after_acoustic = acoustic_packet(after);
    wire::put(request.get(), "prepared_acoustic",
              json_object_get(after_acoustic.get()));
    wire::put(request.get(), "current_acoustic", after_acoustic.release());
    return request;
  }
};
inline void activity(J *fixture) {
  Trial changed(fixture), reference(fixture);
  changed.seed();
  reference.seed();
  for (unsigned i = 0; i < 4; ++i)
    assert(changed.render(128) == reference.render(128));
  auto before = changed.checkpoint();
  auto request = changed.transition();
  for (unsigned variant = 0; variant < 6; ++variant) {
    auto wrong = copy(request.get());
    if (variant == 0)
      wire::u64(wrong.get(), "expected_sample", 511);
    else if (variant == 1)
      wire::u64(wrong.get(), "expected_body_revision", 2);
    else if (variant == 2)
      wire::u64(wrong.get(), "original_request_id", 0);
    else if (variant == 3) {
      auto *after = packet::field(wrong.get(), "actual_after_packet");
      wire::u64(packet::field(after, "determination"), "body_revision", 99);
    } else if (variant == 4)
      wire::u64(packet::field(wrong.get(), "prepared_acoustic"),
                "history_origin_sample", 1);
    else
      wire::text(packet::field(packet::field(wrong.get(), "prepared_acoustic"),
                               "context"),
                 "receiver", "foreign-receiver");
    bool refused = false;
    try {
      auto reply = changed.control.execute(wrong.get());
      refused = !packet::boolean(packet::field(reply.get(), "accepted"));
    } catch (const std::invalid_argument &) {
      refused = true;
    }
    assert(refused);
    auto after_refusal = changed.checkpoint();
    assert(json_object_equal(before.get(), after_refusal.get()));
  }
  changed.apply(request.get());
  auto *ack = packet::field(packet::field(changed.last.get(), "payload"),
                            "physical_transition");
  assert(packet::string(packet::field(ack, "schema")) ==
         "ql.native-physical-source-application/v1");
  assert(wire::decimal(packet::field(ack, "native_sample")) == 512 &&
         wire::decimal(packet::field(ack, "original_native_request_id")) == 9);
  assert(wire::decimal(packet::field(ack, "before_body_revision")) == 1 &&
         wire::decimal(packet::field(ack, "after_body_revision")) == 2);
  changed.current = packet::field(fixture, "after");
  changed.acoustic = changed.acoustic_packet(changed.current);
  auto after = changed.checkpoint();
  auto *a = packet::field(packet::field(before.get(), "native_pair"), "audio"),
       *b = packet::field(packet::field(after.get(), "native_pair"), "audio");
  for (const char *name : {"voices", "touches", "tails", "operations",
                           "releases", "pending_operations", "pending_releases",
                           "source_parameters", "effective_parameters"})
    assert(json_object_equal(packet::field(a, name), packet::field(b, name)));
  auto *old_receiver = packet::field(a, "receiving"),
       *new_receiver = packet::field(b, "receiving");
  assert(json_object_equal(packet::field(old_receiver, "history_linear"),
                           packet::field(new_receiver, "history_linear")));
  assert(
      wire::decimal(packet::field(old_receiver, "history_start_sample")) == 0 &&
      wire::decimal(packet::field(new_receiver, "history_start_sample")) == 0);
  assert(json_object_array_length(packet::field(
             packet::field(new_receiver, "source_history"), "segments")) == 2);
  Trial reopened(fixture, "after_fresh");
  reopened.restore(after.get());
  auto changed_pcm = changed.render(512), original_pcm = reference.render(512);
  assert(reopened.render(512) == changed_pcm);
  assert(changed_pcm != original_pcm &&
         std::any_of(changed_pcm.begin(), changed_pcm.end(),
                     [](double x) { return x != 0.; }));
  auto final = changed.checkpoint();
  auto *audio =
      packet::field(packet::field(final.get(), "native_pair"), "audio");
  assert(wire::decimal(packet::field(audio, "cursor")) == 1024);
  auto reopened_final = reopened.checkpoint();
  auto *final_pair = packet::field(final.get(), "native_pair"),
       *reopened_pair = packet::field(reopened_final.get(), "native_pair");
  assert(json_object_equal(packet::field(final_pair, "physical"),
                           packet::field(reopened_pair, "physical")));
  assert(json_object_equal(
      packet::field(packet::field(final_pair, "audio"), "receiving"),
      packet::field(packet::field(reopened_pair, "audio"), "receiving")));
  auto *parameters = packet::field(audio, "source_parameters");
  assert(packet::number(packet::field(parameters, "master_linear")) == .25);
}
} // namespace ql_test_native_form_control
