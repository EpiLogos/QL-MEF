#ifndef QL_PERFORMANCE_MANAGEMENT_WIRE_HPP
#define QL_PERFORMANCE_MANAGEMENT_WIRE_HPP
#include <ql/performance_offline_wire.hpp>
#include <ql/performance_source_packet.hpp>

namespace ql::performance::management_transport {
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
inline void null(J *out, const char *key) {
  require(json_object_object_add(out, key, nullptr) == 0,
          "native null allocation failed");
}
inline unsigned bounded(J *value, unsigned max) {
  const auto n = packet::integer(value);
  require(n <= max, "native bounded ordinal differs");
  return unsigned(n);
}
inline Json physical(const ql::PhysicalSnapshot &p) {
  auto out = wire::object();
  wire::text(out.get(), "preparation_ref", p.preparation_ref.data());
  wire::text(out.get(), "state_ref", p.state_ref.data());
  wire::u64(out.get(), "body_revision", p.body_revision);
  wire::u64(out.get(), "samples_elapsed", p.samples_elapsed);
  wire::u64(out.get(), "source_generation", p.source_generation);
  wire::put(out.get(), "sample_rate", json_object_new_uint64(p.sample_rate));
  auto ids = wire::array(), positions = wire::array();
  require(p.node_count <= 32, "native physical snapshot node bound differs");
  for (unsigned i = 0; i < p.node_count; ++i) {
    wire::append(ids.get(), json_object_new_string(
                                std::to_string(p.node_identity[i]).c_str()));
    auto xyz = wire::array();
    for (double v :
         {p.visible_positions_metres[i].x, p.visible_positions_metres[i].y,
          p.visible_positions_metres[i].z})
      wire::append(xyz.get(), json_object_new_double(v));
    wire::append(positions.get(), xyz.release());
  }
  wire::put(out.get(), "node_ids", ids.release());
  wire::put(out.get(), "positions_metres", positions.release());
  wire::real(out.get(), "pickup_linear", p.pickup_linear);
  wire::real(out.get(), "energy_joules", p.mechanical_energy_joules);
  // Original native manifest travels alongside U's bounded presentation view.
  wire::text(out.get(), "event_ref", p.event_ref.data());
  wire::text(out.get(), "subject_ref", p.subject_ref.data());
  wire::text(out.get(), "source_coordinate", p.source_coordinate.data());
  wire::text(out.get(), "source_revision", p.source_revision.data());
  wire::text(out.get(), "eigenbasis_identity", p.eigenbasis_identity.data());
  wire::flag(out.get(), "pratibimba", p.pratibimba);
  return out;
}
inline const char *device_state(DeviceState state) {
  switch (state) {
  case DeviceState::Closed:
    return "closed";
  case DeviceState::Prepared:
    return "prepared";
  case DeviceState::Running:
    return "running";
  case DeviceState::Recovering:
    return "recovering";
  case DeviceState::Lost:
    return "lost";
  case DeviceState::Failed:
    return "failed";
  }
  throw std::invalid_argument("native device lifecycle invalid");
}
inline Json device(const DeviceReceipt &d) {
  auto out = wire::object();
  wire::text(out.get(), "state", device_state(d.state));
  wire::text(out.get(), "backend", d.backend);
  if (d.device.id)
    wire::put(out.get(), "device_id", json_object_new_uint64(d.device.id));
  else
    null(out.get(), "device_id");
  wire::real(out.get(), "client_rate", d.client_rate);
  wire::real(out.get(), "hardware_rate", d.hardware_rate);
  wire::put(out.get(), "buffer_frames",
            json_object_new_uint64(d.actual_buffer_frames));
  wire::real(out.get(), "reported_output_latency_ms",
             d.reported_output_latency_ms);
  wire::flag(out.get(), "sample_rate_conversion", d.sample_rate_conversion);
  if (d.error.empty())
    null(out.get(), "error");
  else
    wire::text(out.get(), "error", d.error);
  wire::u64(out.get(), "callbacks", d.callbacks);
  wire::u64(out.get(), "callback_failures", d.callback_failures);
  null(out.get(), "underruns"); // AUHAL overload notices do not count every
                                // physical underrun.
  wire::u64(out.get(), "overload_notifications", d.overload_notifications);
  wire::u64(out.get(), "capture_drops", d.capture_drops);
  wire::u64(out.get(), "timestamp_discontinuities",
            d.timestamp_discontinuities);
  wire::text(out.get(), "physical_latency_measurement", "unexecuted");
  return out;
}
inline Json devices(const std::vector<DeviceDescription> &list) {
  require(list.size() <= 64, "native output device catalog exceeds bound");
  auto out = wire::array();
  for (const auto &d : list) {
    auto value = wire::object();
    wire::put(value.get(), "id", json_object_new_uint64(d.id));
    wire::text(value.get(), "uid", d.uid);
    wire::text(value.get(), "name", d.name);
    wire::flag(value.get(), "default_output", d.default_output);
    wire::flag(value.get(), "alive", d.alive);
    wire::put(value.get(), "output_channels",
              json_object_new_uint64(d.output_channels));
    wire::real(value.get(), "nominal_rate", d.nominal_rate);
    wire::put(value.get(), "buffer_frames",
              json_object_new_uint64(d.buffer_frames));
    wire::append(out.get(), value.release());
  }
  return out;
}
inline Json key(const KeyboardCell &k) {
  auto out = wire::object();
  wire::put(out.get(), "row", json_object_new_uint64(k.row));
  wire::put(out.get(), "column", json_object_new_uint64(k.column));
  wire::put(out.get(), "key", json_object_new_uint64(k.address_key));
  wire::put(out.get(), "pitch_class",
            json_object_new_uint64(k.address_pitch_class));
  wire::put(out.get(), "register_octave",
            json_object_new_int(k.address_register));
  wire::text(out.get(), "label", k.label);
  wire::flag(out.get(), "available", k.available);
  wire::ref(out.get(), "reduction_policy", k.reduction_policy);
  wire::ref(out.get(), "source_collection", k.source_collection);
  wire::ref(out.get(), "source_receipt", k.source_receipt);
  if (k.has_source_degree)
    wire::put(out.get(), "source_degree",
              json_object_new_uint64(k.source_degree));
  else
    null(out.get(), "source_degree");
  if (k.available) {
    wire::real(out.get(), "hertz", k.native_target.hertz);
    wire::ref(out.get(), "coordinate", k.native_target.source_coordinate);
    wire::put(out.get(), "face",
              json_object_new_uint64(k.native_target.source_face));
    null(out.get(), "reason");
    if (k.native_target.exact_ratio) {
      auto r = wire::object();
      wire::u64(r.get(), "numerator", k.native_target.ratio_numerator);
      wire::u64(r.get(), "denominator", k.native_target.ratio_denominator);
      wire::put(out.get(), "ratio", r.release());
    } else
      null(out.get(), "ratio");
  } else {
    for (const char *name : {"hertz", "coordinate", "face", "ratio"})
      null(out.get(), name);
    wire::text(out.get(), "reason", k.unavailable_reason);
  }
  return out;
}
inline KeyboardCell read_key(J *k) {
  wire::keys(k, {"row", "column", "key", "pitch_class", "register_octave",
                 "label", "available", "reduction_policy", "source_collection",
                 "source_receipt", "source_degree", "reason", "native_target"});
  KeyboardCell out{};
  out.address_admitted = true;
  out.row = std::uint8_t(bounded(packet::field(k, "row"), 5));
  out.column = std::uint8_t(bounded(packet::field(k, "column"), 31));
  out.address_key = std::uint8_t(bounded(packet::field(k, "key"), 11));
  out.address_pitch_class =
      std::uint8_t(bounded(packet::field(k, "pitch_class"), 11));
  const auto reg = packet::number(packet::field(k, "register_octave"));
  require(reg >= -16 && reg <= 16 && std::floor(reg) == reg,
          "native register invalid");
  out.address_register = std::int8_t(reg);
  out.label = packet::string(packet::field(k, "label"));
  out.available = packet::boolean(packet::field(k, "available"));
  out.reduction_policy = packet::ref(k, "reduction_policy");
  out.source_collection = packet::ref(k, "source_collection");
  out.source_receipt = packet::ref(k, "source_receipt");
  J *degree = nullptr, *target = nullptr, *reason = nullptr;
  require(json_object_object_get_ex(k, "source_degree", &degree) &&
              json_object_object_get_ex(k, "native_target", &target) &&
              json_object_object_get_ex(k, "reason", &reason),
          "native availability discriminator absent");
  out.has_source_degree = degree != nullptr;
  if (degree)
    out.source_degree = std::uint16_t(bounded(degree, 65535));
  if (out.available) {
    require(target && !reason, "available native source key payload invalid");
    out.native_target = packet::note(target);
  } else {
    require(!target && reason,
            "unavailable native key has a target or no reason");
    out.unavailable_reason = packet::string(reason);
  }
  return out;
}
inline Json parameters(const PerformanceManagement &owner, const Readback &r) {
  struct Descriptor {
    Parameter id;
    const char *name, *label, *group, *unit;
    double minimum, maximum;
  };
  const Descriptor table[] = {
      {Parameter::ForceNewtons, "force-newtons", "Excitation force",
       "excitation", "N", 0, 100},
      {Parameter::AttackSeconds, "attack-seconds", "Attack", "excitation", "s",
       0.0001, 10},
      {Parameter::ReleaseSeconds, "release-seconds", "Release", "excitation",
       "s", 0.001, 30},
      {Parameter::CutoffHertz, "cutoff-hertz", "Excitation low pass",
       "excitation", "Hz", 1, 21600},
      {Parameter::MasterLinear, "master-linear", "Master gain", "mixer",
       "linear", 0, 1},
      {Parameter::BodyLinear, "body-linear", "Body pickup gain", "mixer",
       "linear", 0, 1},
      {Parameter::MonitorLinear, "monitor-linear", "Excitation monitor gain",
       "mixer", "linear", 0, 1}};
  auto out = wire::array();
  for (const auto &d : table) {
    auto p = wire::object();
    wire::text(p.get(), "target_ref",
               std::string("ql:performance/parameter/") + d.name);
    wire::text(p.get(), "action_ref", "ql:native-performance/parameter");
    wire::text(p.get(), "native_owner", "ql.performance.Engine");
    wire::text(p.get(), "label", d.label);
    wire::text(p.get(), "group", d.group);
    wire::text(p.get(), "scope", "instrument");
    wire::text(p.get(), "unit", d.unit);
    wire::real(p.get(), "minimum", d.minimum);
    wire::real(p.get(), "maximum",
               d.id == Parameter::ForceNewtons
                   ? owner.native().engine->force_parameter_maximum()
                   : d.maximum);
    wire::real(p.get(), "baseline", owner.baseline_parameter(d.id));
    wire::real(p.get(), "effective",
               PerformanceManagement::read_parameter(r.effective, d.id));
    wire::u64(p.get(), "smoothing_samples", 240);
    null(p.get(), "route_ref");
    auto caps = wire::array();
    for (const char *cap : {"set", "clear"})
      wire::append(caps.get(), json_object_new_string(cap));
    wire::put(p.get(), "capabilities", caps.release());
    null(p.get(), "unavailable_reason");
    wire::append(out.get(), p.release());
  }
  return out;
}
inline Json history(const InputBindingRecord &h) {
  auto out = wire::object();
  wire::u64(out.get(), "ordinal", h.ordinal);
  wire::u64(out.get(), "native_sequence", h.native_sequence);
  wire::put(out.get(), "change", json_object_new_uint64(unsigned(h.change)));
  wire::put(out.get(), "operation",
            json_object_new_uint64(unsigned(h.operation)));
  wire::ref(out.get(), "input_ref", h.input_ref);
  wire::put(out.get(), "target", wire::note(h.target).release());
  return out;
}
struct BodyStanding {
  bool source_form = false;
  Ref recipe{};
  std::uint64_t validated_generation = 0;
};

/// Installed into the EXISTING field worker. One management owner and one
/// P/Engine reside in this retained worker; JSON is never parsed by the
/// callback.
class Control {
  std::unique_ptr<PerformanceManagement> owner_;
  std::vector<DeviceDescription> devices_;
  BodyStanding standing_{};
  unsigned transpose_ = 0;
  bool released_ = false;
  static const char *result(Result r) {
    switch (r) {
    case Result::Accepted:
      return "accepted";
    case Result::Stale:
      return "stale";
    case Result::Invalid:
      return "invalid";
    case Result::Late:
      return "late";
    case Result::Order:
      return "order";
    case Result::Overflow:
      return "overflow";
    case Result::Unavailable:
      return "unavailable";
    case Result::Exhausted:
      return "exhausted";
    }
    throw std::invalid_argument("native result invalid");
  }
  void scope(J *request) {
    require(owner_ && !released_,
            "native retained performance owner unavailable");
    const auto &source = owner_->native().engine->source_for_native_admission();
    require(
        packet::ref(request, "session_ref") == owner_->session_ref() &&
            wire::decimal(packet::field(request, "expected_transport_epoch")) ==
                owner_->transport_epoch() &&
            packet::identity(packet::field(request, "expected_source")) ==
                source.identity &&
            wire::decimal(packet::field(request, "expected_body_revision")) ==
                source.body_revision,
        "native performance session/source/body/epoch differs");
    // Live admission deliberately does not compare a UI-provided exact sample
    // cursor. AUHAL dates the event from its actual native output-clock anchor.
  }
  Json reading(const ManagementPulse &pulse) {
    require(owner_ && pulse.has_readback,
            "native callback/control snapshot unavailable");
    const auto &r = pulse.reading;
    auto out = wire::object();
    wire::text(out.get(), "schema", "ql.performance-management/v1");
    wire::ref(out.get(), "session_ref", owner_->session_ref());
    wire::u64(out.get(), "transport_epoch", owner_->transport_epoch());
    auto s = wire::object();
    wire::ref(s.get(), "instance_ref", r.identity.instance);
    wire::ref(s.get(), "event_ref", r.identity.event);
    wire::ref(s.get(), "subject_ref", r.identity.subject);
    wire::u64(s.get(), "m1_revision", r.identity.m1_revision);
    wire::u64(s.get(), "m2_generation", r.identity.m2_generation);
    wire::u64(s.get(), "body_revision", r.body_revision);
    wire::ref(s.get(), "preparation_ref", r.determination.body_preparation_ref);
    wire::ref(s.get(), "state_ref", r.determination.body_state_ref);
    wire::put(out.get(), "scope", s.release());
    auto b = wire::object();
    wire::text(b.get(), "kind",
               standing_.source_form ? "sourceForm" : "referenceMetric");
    if (standing_.source_form) {
      wire::ref(b.get(), "recipe_ref", standing_.recipe);
      wire::u64(b.get(), "validated_m3_generation",
                standing_.validated_generation);
    } else {
      null(b.get(), "recipe_ref");
      null(b.get(), "validated_m3_generation");
    }
    wire::put(out.get(), "body_source", b.release());
    wire::u64(out.get(), "samples_elapsed", r.samples_elapsed);
    wire::u64(out.get(), "accepted_sequence",
              owner_->native().engine->accepted_sequence());
    wire::u64(out.get(), "last_applied_application_ordinal",
              r.last_applied_application_ordinal);
    auto roles = wire::object();
    wire::text(roles.get(), "physical",
               standing_.source_form ? "canonical-source-form-scalar-excitation"
                                     : "reference-metric-scalar-excitation");
    auto personal = wire::object();
    wire::flag(personal.get(), "available", false);
    wire::text(personal.get(), "reason",
               "native N9 programme/route admission not installed in this "
               "scalar management consumer");
    wire::put(roles.get(), "personal_nine_force_routes", personal.release());
    auto sky = wire::object();
    wire::flag(sky.get(), "available", false);
    wire::text(sky.get(), "reason",
               "original sky contributors retained as source assets; distinct "
               "common-body forcing not joined");
    wire::put(roles.get(), "sky_ten_source_forcing", sky.release());
    wire::put(out.get(), "consumer_roles", roles.release());
    wire::flag(out.get(), "available",
               r.available && owner_->recording_available() &&
                   !pulse.release_pending);
    if (r.available && owner_->recording_available() && !pulse.release_pending)
      null(out.get(), "reason");
    else
      wire::text(out.get(), "reason",
                 pulse.release_pending ? "native physical release pending"
                 : !owner_->recording_available()
                     ? "native recording/readback lost; owner held"
                     : "native A/P callback unavailable");
    auto capabilities = wire::array();
    for (const char *name :
         {"performance-inspect", "performance-gesture", "performance-sustain",
          "performance-panic", "performance-parameter", "performance-transpose",
          "performance-device-enumerate", "performance-device-open",
          "performance-device-start", "performance-device-stop",
          "performance-device-recover"})
      wire::append(capabilities.get(), json_object_new_string(name));
    wire::put(out.get(), "capabilities", capabilities.release());
    auto keys = wire::array();
    for (const auto &cell : owner_->catalog())
      wire::append(keys.get(), key(cell).release());
    wire::put(out.get(), "keys", keys.release());
    wire::put(out.get(), "transpose", json_object_new_uint64(transpose_));
    wire::put(out.get(), "parameters", parameters(*owner_, r).release());
    wire::put(out.get(), "devices", devices(devices_).release());
    wire::put(out.get(), "device", device(pulse.device).release());
    wire::put(out.get(), "physical", physical(r.physical).release());
    wire::put(out.get(), "active_voices",
              json_object_new_uint64(r.active_voices));
    wire::put(out.get(), "active_touches",
              json_object_new_uint64(r.active_touches));
    wire::flag(out.get(), "sustain", r.sustain);
    wire::real(out.get(), "peak_linear", r.peak);
    wire::real(out.get(), "rms_linear", r.rms);
    wire::u64(out.get(), "clipping_samples", r.clipping_samples);
    wire::real(out.get(), "scalar_force_budget_newtons",
               r.scalar_force_budget_newtons);
    wire::u64(out.get(), "force_limited_samples", r.force_limited_samples);
    auto e = wire::object();
    wire::ref(e.get(), "policy_ref", r.determination.excitation.policy_ref);
    wire::ref(e.get(), "standing", r.determination.excitation.standing);
    wire::put(e.get(), "audio_octet_hz",
              wire::doubles(r.determination.audio_octet_hz).release());
    auto nodes = wire::array();
    for (const auto &n : r.determination.nodal_quartet) {
      auto value = wire::object();
      wire::put(value.get(), "position", json_object_new_uint64(n.position));
      wire::put(value.get(), "face", json_object_new_uint64(n.face));
      wire::put(value.get(), "m", json_object_new_uint64(n.m));
      wire::put(value.get(), "n", json_object_new_uint64(n.n));
      wire::append(nodes.get(), value.release());
    }
    wire::put(e.get(), "nodal_quartet", nodes.release());
    wire::put(out.get(), "excitation", e.release());
    return out;
  }

public:
  bool active() const noexcept { return owner_ && !released_; }
  void hold() noexcept {
    if (owner_)
      owner_->hold();
  }
  Json execute(J *request) {
    require(packet::string(packet::field(request, "schema")) ==
                "ql.performance-control/v1",
            "native performance control schema differs");
    const auto op = packet::string(packet::field(request, "operation"));
    ManagementAdmission admission{};
    bool accepted = false;
    std::string reason;
    Json payload = wire::object();
    if (op == "prepare") {
      wire::keys(request,
                 {"schema", "operation", "session_ref", "packet",
                  "actual_native_basis", "m1_pratibimba", "physical_pratibimba",
                  "body_source", "current_source_packet"});
      require(!owner_, "one native performance session already resides in this "
                       "retained worker");
      auto packet_value = packet::field(request, "packet");
      const std::string encoded =
          json_object_to_json_string_ext(packet_value, JSON_C_TO_STRING_PLAIN);
      J *sparse = nullptr;
      auto native =
          json_object_object_get_ex(packet_value, "source_key_admission",
                                    &sparse)
              ? prepare_source_performance_packet(
                    encoded, packet::field(request, "current_source_packet"),
                    packet::field(request, "actual_native_basis"),
                    packet::boolean(packet::field(request, "m1_pratibimba")),
                    packet::boolean(
                        packet::field(request, "physical_pratibimba")))
              : prepare_performance_packet(
                    encoded, packet::field(request, "actual_native_basis"),
                    packet::boolean(packet::field(request, "m1_pratibimba")),
                    packet::boolean(
                        packet::field(request, "physical_pratibimba")));
      auto source = packet::field(request, "body_source");
      wire::keys(source, {"kind", "recipe_ref", "validated_m3_generation"});
      const auto kind = packet::string(packet::field(source, "kind"));
      J *recipe = nullptr, *generation = nullptr;
      require(json_object_object_get_ex(source, "recipe_ref", &recipe) &&
                  json_object_object_get_ex(source, "validated_m3_generation",
                                            &generation),
              "native body source standing absent");
      BodyStanding stamp{};
      if (kind == "sourceForm") {
        require(recipe && generation &&
                    packet::field(packet_value, "source_form_recipe"),
                "native canonical source form recipe absent");
        stamp.source_form = true;
        stamp.recipe = reference(packet::string(recipe).c_str());
        stamp.validated_generation = wire::decimal(generation);
        require(stamp.validated_generation ==
                    native.body->preparation().input().source_generation,
                "native source-form generation differs");
      } else
        require(kind == "referenceMetric" && !recipe && !generation,
                "reference body cannot claim source form standing");
      auto next = std::make_unique<PerformanceManagement>(
          std::move(native), packet::ref(request, "session_ref"));
      owner_ = std::move(next);
      standing_ = stamp;
      accepted = true;
      const auto &prepared = owner_->native().body->preparation();
      auto descriptor = wire::object();
      wire::text(descriptor.get(), "schema",
                 "ql.native-physical-descriptor/v1");
      wire::put(descriptor.get(), "physical_preparation",
                json_object_get(packet::field(packet_value, "physical_body")));
      wire::text(descriptor.get(), "eigenbasis_identity",
                 prepared.eigenbasis_identity());
      auto frequencies = wire::array();
      for (std::size_t i = 0; i < prepared.mode_count(); ++i)
        wire::append(frequencies.get(),
                     json_object_new_double(prepared.frequency_hz(i)));
      wire::put(descriptor.get(), "mode_frequencies_hz", frequencies.release());
      wire::u64(descriptor.get(), "native_cursor",
                owner_->last_readback().samples_elapsed);
      wire::put(payload.get(), "body_descriptor", descriptor.release());
    } else {
      scope(request);
      const std::map<std::string, std::vector<std::string>> fields = {
          {"catalog", {"cells", "transpose"}},
          {"press",
           {"input_ref", "row", "column", "native_target", "velocity"}},
          {"release", {"input_ref"}},
          {"expression", {"input_ref", "pressure"}},
          {"sustain", {"down"}},
          {"panic", {}},
          {"hold", {}},
          {"parameter", {"parameter", "value"}},
          {"parameter-clear", {"parameter"}},
          {"parameter-undo", {"parameter"}},
          {"parameter-learn", {"parameter"}},
          {"device-enumerate", {}},
          {"device-open", {"device_id", "sample_rate", "buffer_frames"}},
          {"device-start", {}},
          {"device-stop", {}},
          {"device-recover", {}},
          {"device-close", {}},
          {"score", {"event", "input_ref"}},
          {"checkpoint", {}},
          {"offline-render", {"scope", "frames"}},
          {"restore",
           {"checkpoint", "expected_cursor", "transaction_ref",
            "checkpoint_ref"}},
          {"inspect", {}}};
      const auto selected = fields.find(op);
      require(selected != fields.end(), "unknown native performance operation");
      std::set<std::string> allowed = {
          "schema",          "operation",
          "session_ref",     "expected_transport_epoch",
          "expected_source", "expected_body_revision"};
      allowed.insert(selected->second.begin(), selected->second.end());
      require(json_object_object_length(request) == int(allowed.size()),
              "missing or unknown native performance fields");
      json_object_object_foreach(request, name, value) {
        (void)value;
        require(allowed.count(name), "unknown native performance field");
      }
      if (op == "catalog") {
        auto cells = packet::field(request, "cells");
        require(json_object_is_type(cells, json_type_array) &&
                    json_object_array_length(cells) <= 192,
                "native catalog bound differs");
        std::vector<KeyboardCell> catalog;
        catalog.reserve(json_object_array_length(cells));
        for (std::size_t i = 0; i < json_object_array_length(cells); ++i)
          catalog.push_back(read_key(json_object_array_get_idx(cells, i)));
        owner_->admit_catalog(std::move(catalog));
        transpose_ = bounded(packet::field(request, "transpose"), 11);
        accepted = true;
      } else if (op == "press") {
        admission = owner_->press(
            packet::ref(request, "input_ref"),
            std::uint8_t(bounded(packet::field(request, "row"), 5)),
            std::uint8_t(bounded(packet::field(request, "column"), 31)),
            packet::note(packet::field(request, "native_target")),
            packet::number(packet::field(request, "velocity")));
      } else if (op == "release")
        admission = owner_->release(packet::ref(request, "input_ref"));
      else if (op == "expression")
        admission = owner_->expression(
            packet::ref(request, "input_ref"),
            packet::number(packet::field(request, "pressure")));
      else if (op == "sustain")
        admission =
            owner_->sustain(packet::boolean(packet::field(request, "down")));
      else if (op == "panic")
        admission = owner_->panic();
      else if (op == "hold") {
        wire::u64(payload.get(), "release_request", owner_->hold());
        accepted = true;
      } else if (op == "parameter")
        admission = owner_->set_parameter(
            Parameter(bounded(packet::field(request, "parameter"),
                              unsigned(Parameter::MonitorLinear))),
            packet::number(packet::field(request, "value")));
      else if (op == "parameter-clear")
        admission = owner_->clear_parameter(
            Parameter(bounded(packet::field(request, "parameter"),
                              unsigned(Parameter::MonitorLinear))));
      else if (op == "parameter-undo")
        admission = owner_->undo_parameter(
            Parameter(bounded(packet::field(request, "parameter"),
                              unsigned(Parameter::MonitorLinear))));
      else if (op == "parameter-learn")
        admission = owner_->learn_parameter(
            Parameter(bounded(packet::field(request, "parameter"),
                              unsigned(Parameter::MonitorLinear))));
      else if (op == "device-enumerate") {
        devices_ = PerformanceManagement::enumerate_devices();
        accepted = true;
      } else if (op == "device-open") {
        DeviceConfig config{};
        config.device_id =
            bounded(packet::field(request, "device_id"), UINT32_MAX);
        config.sample_rate =
            bounded(packet::field(request, "sample_rate"), 48000);
        config.buffer_frames =
            bounded(packet::field(request, "buffer_frames"), 256);
        require(config.sample_rate == 48000 && (config.buffer_frames == 128 ||
                                                config.buffer_frames == 256),
                "native playable device format invalid");
        accepted = owner_->open_device(config);
      } else if (op == "device-start")
        accepted = owner_->start_device();
      else if (op == "device-stop")
        accepted = owner_->stop_device();
      else if (op == "device-recover")
        accepted = owner_->recover_device();
      else if (op == "device-close")
        accepted = owner_->close_device();
      else if (op == "score") {
        auto operation = wire::read_operation(packet::field(request, "event"));
        json_object *input_ref = packet::field(request, "input_ref");
        const Ref original_input =
            (!input_ref || json_object_is_type(input_ref, json_type_null))
                ? Ref{}
                : reference(packet::string(input_ref));
        const auto r = owner_->enqueue_score_input(operation, original_input);
        accepted = r == Result::Accepted;
        reason = result(r);
      } else if (op == "checkpoint") {
        auto saved = owner_->stopped_checkpoint();
        wire::put(
            payload.get(), "checkpoint",
            management_checkpoint_transport::checkpoint_wire(*saved).release());
        accepted = true;
      } else if (op == "offline-render") {
        auto scope =
            offline_transport::read_scope(packet::field(request, "scope"));
        auto chunk = offline_transport::render_chunk_wire(
            *owner_, scope,
            bounded(packet::field(request, "frames"), max_frames));
        accepted =
            packet::string(packet::field(chunk.get(), "result")) == "accepted";
        wire::put(payload.get(), "chunk", chunk.release());
      } else if (op == "restore") {
        auto saved = management_checkpoint_transport::read_checkpoint_wire(
            packet::field(request, "checkpoint"));
        TransportAcknowledgement ack{};
        accepted = owner_->stopped_restore(
            *saved, wire::decimal(packet::field(request, "expected_cursor")),
            packet::ref(request, "transaction_ref"),
            packet::ref(request, "checkpoint_ref"), ack);
        if (accepted) {
          auto a = wire::object();
          wire::u64(a.get(), "previous_epoch", ack.previous_epoch);
          wire::u64(a.get(), "epoch", ack.epoch);
          wire::u64(a.get(), "previous_cursor", ack.previous_cursor);
          wire::u64(a.get(), "previous_sequence", ack.previous_sequence);
          wire::u64(a.get(), "target_sample", ack.target_sample);
          wire::u64(a.get(), "accepted_sequence", ack.accepted_sequence);
          wire::ref(a.get(), "transaction_ref", ack.transaction);
          wire::ref(a.get(), "checkpoint_ref", ack.checkpoint);
          wire::put(payload.get(), "transport_ack", a.release());
        }
      } else
        require(op == "inspect", "unknown native performance operation");
      if (op == "inspect")
        accepted = true;
      if (admission.clock.sequence || admission.result != Result::Unavailable ||
          !admission.reason.empty()) {
        accepted = admission.result == Result::Accepted;
        reason = admission.reason;
        auto a = wire::object();
        wire::u64(a.get(), "sequence", admission.clock.sequence);
        wire::u64(a.get(), "sample", admission.clock.accepted_sample);
        wire::u64(a.get(), "clock_epoch", admission.clock.epoch);
        wire::u64(a.get(), "anchor_ordinal", admission.clock.anchor_ordinal);
        wire::u64(a.get(), "trigger_host_ticks",
                  admission.clock.trigger_host_ticks);
        wire::u64(a.get(), "admitted_host_ticks",
                  admission.clock.admitted_host_ticks);
        wire::real(a.get(), "mapping_uncertainty_samples",
                   admission.clock.mapping_uncertainty_samples);
        wire::flag(a.get(), "input_transit_unknown",
                   admission.clock.input_transit_unknown);
        wire::put(payload.get(), "admission", a.release());
      }
    }
    if (!accepted && reason.empty())
      reason = owner_->device_receipt().error.empty()
                   ? "native operation refused"
                   : owner_->device_receipt().error;
    auto pulse = owner_->pulse();
    auto out = wire::object();
    wire::text(out.get(), "schema", "ql.performance-worker-reply/v1");
    wire::text(out.get(), "operation", op);
    wire::flag(out.get(), "accepted", accepted);
    wire::text(out.get(), "reason", reason);
    wire::put(out.get(), "reading", reading(*pulse).release());
    wire::put(out.get(), "payload", payload.release());
    auto applied = wire::array();
    for (const auto &a : pulse->applications)
      wire::append(applied.get(), wire::application(a).release());
    wire::put(out.get(), "applications", applied.release());
    auto journal = wire::array();
    for (const auto &h : pulse->input_history)
      wire::append(journal.get(), history(h).release());
    wire::put(out.get(), "input_history", journal.release());
    wire::put(out.get(), "recording",
              wire::recording(pulse->recording).release());
    wire::u64(out.get(), "last_native_touch", owner_->last_native_touch());
    wire::u64(out.get(), "last_native_member", owner_->last_native_member());
    wire::flag(out.get(), "recording_available", owner_->recording_available());
    wire::flag(out.get(), "release_pending", pulse->release_pending);
    wire::flag(out.get(), "release_zero_proven", pulse->release_zero_proven);
    wire::u64(out.get(), "release_proof_cursor", pulse->release_proof_cursor);
    return out;
  }
};
} // namespace ql::performance::management_transport
#endif
