#ifndef QL_PERFORMANCE_MANAGEMENT_WIRE_HPP
#define QL_PERFORMANCE_MANAGEMENT_WIRE_HPP
#include <ql/performance_acoustic_wire.hpp>
#include <ql/performance_capture_wire.hpp>
#include <ql/performance_form_wire.hpp>
#include <ql/performance_offline_wire.hpp>
#include <ql/performance_physical_routes.hpp>
#include <ql/performance_receiving_restore.hpp>
#include <ql/performance_receiving_wire.hpp>
#include <ql/performance_resident_registry_wire.hpp>
#include <ql/performance_source_packet.hpp>
#include <ql/performance_timing.hpp>

namespace ql::performance::scene_contact_transport {
class Channel;
}
namespace ql::performance::selected_source_transport {
class Channel;
}
namespace ql::performance::management_transport {
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
inline void null(J *out, const char *key) {
  require(json_object_object_add(out, key, nullptr) == 0,
          "native null allocation failed");
}
// Complete saved Management checkpoint text is transport data, not a Ref.
// The caller below parses all bytes with strict UTF-8 validation before any
// paired owner mutation. Narrow native references retain packet::string.
inline std::string checkpoint_text(J *value) {
  require(value && json_object_is_type(value, json_type_string),
          "native checkpoint text string required");
  const auto length = json_object_get_string_len(value);
  const char *text = json_object_get_string(value);
  require(length > 0 && std::size_t(length) <= 32 * 1024 * 1024 && text &&
              std::memchr(text, '\0', std::size_t(length)) == nullptr,
          "original receiving checkpoint exceeds native text bound or contains "
          "NUL");
  return std::string(text, std::size_t(length));
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
    for (double v : p.visible_positions_metres[i])
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
inline Json score_admission(const NativeScoreAdmission &a) {
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-score-admission/v1");
  wire::flag(out.get(), "queued", a.queue().queued());
  wire::ref(out.get(), "session_ref", a.session_ref());
  wire::u64(out.get(), "transport_epoch", a.transport_epoch());
  wire::u64(out.get(), "queue_cursor", a.queue().queue_cursor());
  wire::u64(out.get(), "queue_horizon", a.queue().queue_horizon());
  if (valid_ref(a.input_ref()))
    wire::ref(out.get(), "input_ref", a.input_ref());
  else
    null(out.get(), "input_ref");
  wire::put(out.get(), "event",
            wire::operation(a.queue().operation()).release());
  wire::put(out.get(), "source",
            wire::determination(a.queue().source()).release());
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
       "excitation", "Hz", 1, .45 * owner.native().engine->sample_rate()},
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
    const auto &engine = *owner.native().engine;
    wire::u64(p.get(), "sample_rate", engine.sample_rate());
    wire::u64(p.get(), "smoothing_samples",
              std::uint64_t(std::ceil(
                  engine.parameter_smoothing_time_constant_samples())));
    wire::real(p.get(), "smoothing_seconds", parameter_smoothing_seconds);
    wire::real(p.get(), "smoothing_time_constant_samples",
               engine.parameter_smoothing_time_constant_samples());
    wire::real(p.get(), "smoothing_coefficient",
               engine.parameter_smoothing_coefficient());
    wire::text(p.get(), "smoothing_algorithm",
               "one-pole-exponential-every-native-sample");
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
// The exact batch and independent native tail travel together. This is the
// same copied ManagementPulse; no observer drain or synthesized last-row tail.
inline void put_input_history(J *out, const ManagementPulse &pulse) {
  auto journal = wire::array();
  for (const auto &h : pulse.input_history)
    wire::append(journal.get(), history(h).release());
  wire::put(out, "input_history", journal.release());
  wire::u64(out, "last_input_ordinal", pulse.last_input_ordinal);
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
  friend class ql::performance::scene_contact_transport::Channel;
  friend class ql::performance::selected_source_transport::Channel;
  // Only the already-friended private native channels can record their
  // actual same-owner admission and pulse. Timing authority stays in Control.
  void retain_private_channel_score(const NativeScoreAdmission &admission) {
    timing_.score(admission);
  }
  void commit_private_channel_pulse(const ManagementPulse &pulse) {
    timing_.committed(pulse, owner_->transport_epoch());
  }
  Json serialize_pulse(const std::string &op, bool accepted,
                       const std::string &reason, Json payload,
                       const ManagementPulse &pulse) {
    auto out = wire::object();
    wire::text(out.get(), "schema", "ql.performance-worker-reply/v1");
    wire::text(out.get(), "operation", op);
    wire::flag(out.get(), "accepted", accepted);
    wire::text(out.get(), "reason", reason);
    wire::put(out.get(), "reading", reading(pulse).release());
    wire::put(out.get(), "payload", payload.release());
    auto applied = wire::array();
    for (const auto &a : pulse.applications)
      wire::append(applied.get(), wire::application(a).release());
    wire::put(out.get(), "applications", applied.release());
    put_input_history(out.get(), pulse);
    wire::put(out.get(), "recording",
              wire::recording(pulse.recording).release());
    auto registry = std::make_unique<NativeResidentRegistry>();
    if (owner_->write_resident_registry(pulse, *registry)) {
      wire::put(out.get(), "resident_consumers",
                resident_wire::registry(*registry).release());
      wire::put(out.get(), "native_timing_owner",
                resident_wire::timing_owner(*owner_, *registry).release());
    } else {
      null(out.get(), "resident_consumers");
      null(out.get(), "native_timing_owner");
      wire::text(out.get(), "resident_registry_reason",
                 "actual native same-pulse registration unavailable");
    }
    wire::put(out.get(), "native_capture",
              native_capture_transport::batch(*owner_, pulse).release());
    wire::u64(out.get(), "last_native_touch", owner_->last_native_touch());
    wire::u64(out.get(), "last_native_member", owner_->last_native_member());
    wire::flag(out.get(), "recording_available", owner_->recording_available());
    wire::flag(out.get(), "release_pending", pulse.release_pending);
    wire::flag(out.get(), "release_zero_proven", pulse.release_zero_proven);
    wire::u64(out.get(), "release_proof_cursor", pulse.release_proof_cursor);
    return out;
  }
  std::unique_ptr<PerformanceManagement> owner_;
  // Immutable output of the actual preparation on this serial worker. These
  // retained bytes are coherence evidence, never a private Source/Act grant.
  Json prepared_source_ = wire::own(nullptr),
       prepared_basis_ = wire::own(nullptr);
  // Exact independently decoded numerical producer retained only after its
  // actual same-owner installation commits. This never grants Scene custody.
  Json retained_acoustic_ = wire::own(nullptr);
  std::unique_ptr<PerformanceManagement::PreparedReceivingRestore>
      receiving_restore_;
  std::vector<DeviceDescription> devices_;
  BodyStanding standing_{};
  unsigned transpose_ = 0;
  bool released_ = false;
  std::size_t admitted_route_count_ = 0;
  NativePerformanceTimingOwner timing_;
  std::uint64_t last_physical_request_id_ = 0;
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
    wire::u64(out.get(), "last_input_ordinal", pulse.last_input_ordinal);
    auto roles = wire::object();
    wire::text(roles.get(), "physical",
               standing_.source_form ? "canonical-source-form-scalar-excitation"
                                     : "reference-metric-scalar-excitation");
    // Publish the actual validated preparation standing on every native
    // reading. Activation replaces its source roles with this producer.
    if (standing_.source_form)
      wire::text(roles.get(), "legacy_mode_frequency_remapping",
                 "retired-in-this-explicit-physical-projection");
    else
      null(roles.get(), "legacy_mode_frequency_remapping");
    auto personal = wire::object();
    const bool routes_admitted =
        r.has_route_programs && admitted_route_count_ == 9;
    wire::flag(personal.get(), "available", routes_admitted);
    wire::text(personal.get(), "standing",
               routes_admitted
                   ? "native-current-source-nine-programmes-same-body"
                   : "not-admitted");
    wire::text(personal.get(), "admission",
               routes_admitted && r.physical_routes.route_count == 9 &&
                       r.physical_routes.end_sample == r.samples_elapsed
                   ? "applied"
               : routes_admitted ? "prepared"
                                 : "unavailable");
    wire::put(personal.get(), "route_count",
              json_object_new_uint64(admitted_route_count_));
    wire::flag(personal.get(), "suspended", r.routes_suspended);
    if (routes_admitted)
      null(personal.get(), "reason");
    else
      wire::text(personal.get(), "reason",
                 "actual original native N9 programmes/projections not "
                 "admitted for this receiving context");
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
    if (r.has_receiving) {
      require(r.receiving.samples_elapsed == r.samples_elapsed,
              "receiving copied observation detached from native body cursor");
      wire::put(out.get(), "receiving_transport",
                receiving_transport::readback(r.receiving).release());
    }
    wire::put(out.get(), "active_voices",
              json_object_new_uint64(r.active_voices));
    wire::put(out.get(), "active_touches",
              json_object_new_uint64(r.active_touches));
    wire::flag(out.get(), "sustain", r.sustain);
    wire::real(out.get(), "peak_linear", r.peak);
    wire::real(out.get(), "raw_peak_linear", r.raw_peak);
    wire::real(out.get(), "rms_linear", r.rms);
    wire::u64(out.get(), "clipping_samples", r.clipping_samples);
    wire::real(out.get(), "scalar_force_budget_newtons",
               r.scalar_force_budget_newtons);
    wire::real(out.get(), "contact_reserved_force_newtons",
               r.contact_reserved_force_newtons);
    wire::real(out.get(), "note_headroom_newtons", r.note_headroom_newtons);
    auto contacts = wire::array();
    for (const auto &delivery : r.contacts)
      if (delivery.status != NativeContactStatus::Empty)
        wire::append(contacts.get(),
                     contact_transport::delivery(delivery).release());
    wire::put(out.get(), "contacts", contacts.release());
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
  // Trusted retained worker only. The private Scene occurrence owner has
  // already qualified full current source/Act/history and exact immutable
  // programme custody before issuing this nonserializable witness. Generic
  // execute(JSON), score import and renderer operations cannot construct it.
  NativeScoreAdmission enqueue_scene_contact(
      const std::shared_ptr<const PreparedContactProgram> &programme,
      const NativeContactOccurrenceWitness &witness) {
    require(active(), "contact requires the retained active native worker");
    return owner_->enqueue_contact_admission(programme, witness);
  }
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
    Json readmission = wire::own(nullptr);
    if (op == "prepare") {
      J *receiving = nullptr, *current_receiving = nullptr;
      const bool has_receiving =
          json_object_object_get_ex(request, "receiving_admission", &receiving);
      const bool has_current_receiving = json_object_object_get_ex(
          request, "current_receiving_admission", &current_receiving);
      require(has_receiving == has_current_receiving,
              "native receiving candidate/current pair incomplete");
      if (has_receiving) {
        wire::keys(request, {"schema", "operation", "session_ref", "packet",
                             "actual_native_basis", "m1_pratibimba",
                             "physical_pratibimba", "body_source",
                             "current_source_packet", "receiving_admission",
                             "current_receiving_admission"});
      } else {
        wire::keys(request, {"schema", "operation", "session_ref", "packet",
                             "actual_native_basis", "m1_pratibimba",
                             "physical_pratibimba", "body_source",
                             "current_source_packet"});
      }
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
      // The body is still stopped and unpublished here. Capture only its
      // immutable preparation before any AudioUnit/consumer can own q/v.
      const auto prepared = native.body->preparation();
      std::shared_ptr<PhysicalRoutesPortBinding> routes;
      NativeRouteProgramSet programs{};
      if (has_receiving) {
        require(receiving && current_receiving && !native.notes.empty(),
                "current native receiving/M1 programme phase absent");
        auto *operation = packet::field(current_receiving, "operation");
        auto *sources = packet::field(operation, "sources");
        require(json_object_is_type(sources, json_type_array) &&
                    json_object_array_length(sources) <= 9,
                "native receiving source bound differs");
        std::vector<std::string> program_refs;
        for (std::size_t i = 0; i < json_object_array_length(sources); ++i) {
          const auto driver = packet::string(packet::field(
              json_object_array_get_idx(sources, i), "driver_ref"));
          program_refs.push_back(driver + "/m1-excitation-program");
        }
        auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
            read_native_receiving_admission(
                receiving, current_receiving, native,
                packet::field(request, "actual_native_basis"), prepared,
                program_refs, 0));
        routes = std::make_shared<PhysicalRoutesPortBinding>(
            native.body, admitted, prepared, native.determination,
            packet::field(request, "actual_native_basis"),
            packet::boolean(packet::field(request, "m1_pratibimba")), 0);
        programs.manifest = routes->manifest();
        programs.program_count = programs.manifest.route_count;
        programs.scalar_note_enabled = programs.manifest.scalar_note_enabled;
        programs.scalar_note_gain = programs.manifest.scalar_note_gain;
        for (std::size_t i = 0; i < programs.program_count; ++i) {
          auto &program = programs.programs[i];
          program.handle = programs.manifest.programs[i];
          program.phase_source_ref = programs.manifest.m1_coordinate;
          program.sine = native.notes.front().phase_sin;
          program.cosine = native.notes.front().phase_cos;
        }
        for (const auto &note : native.notes)
          require(note.phase_sin == native.notes.front().phase_sin &&
                      note.phase_cos == native.notes.front().phase_cos,
                  "native M1 source quadrature differs across supplied source "
                  "targets");
      }
      auto next = std::make_unique<PerformanceManagement>(
          std::move(native), packet::ref(request, "session_ref"));
      if (routes) {
        const auto result = next->admit_distinct_receiving(
            physical_routes_port(physical_port(next->native().body), routes),
            programs, 0);
        require(
            result.result == Result::Accepted,
            "actual source-qualified native receiving route install refused");
      }
      J *source_copy = nullptr, *basis_copy = nullptr;
      const auto source_copied =
          json_object_deep_copy(packet_value, &source_copy, nullptr);
      auto immutable_source = wire::own(source_copy);
      const auto basis_copied = json_object_deep_copy(
          packet::field(request, "actual_native_basis"), &basis_copy, nullptr);
      auto immutable_basis = wire::own(basis_copy);
      require(source_copied == 0 && basis_copied == 0 && immutable_source &&
                  immutable_basis,
              "immutable resident native preparation copy failed");
      prepared_source_ = std::move(immutable_source);
      prepared_basis_ = std::move(immutable_basis);
      owner_ = std::move(next);
      timing_.reset();
      last_physical_request_id_ = 0;
      admitted_route_count_ = routes ? programs.program_count : 0;
      standing_ = stamp;
      accepted = true;
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
          {"receiving-transport-install",
           {"prepared_acoustic", "current_acoustic", "expected_sample"}},
          {"receiving-transport-replace",
           {"before_acoustic", "prepared_acoustic", "current_acoustic",
            "expected_sample"}},
          {"source-body-transition",
           {"original_request_id", "expected_sample", "kind", "cause_ref",
            "before_packet", "before_native_basis", "after_packet",
            "actual_after_packet", "actual_after_native_basis",
            "receiving_admission", "current_receiving_admission",
            "native_catalog", "body_source", "before_acoustic",
            "prepared_acoustic", "current_acoustic"}},
          {"timing", {"moment", "ordinal"}},
          {"checkpoint", {}},
          {"offline-render", {"scope", "frames"}},
          {"restore",
           {"checkpoint", "expected_cursor", "transaction_ref",
            "checkpoint_ref"}},
          {"restore-current-receiving",
           {"original_checkpoint_wire", "expected_cursor", "transaction_ref",
            "checkpoint_ref", "current_source_packet", "actual_native_basis",
            "receiving_admission", "current_receiving_admission",
            "current_receiving", "native_catalog"}},
          {"inspect", {}}};
      const auto selected = fields.find(op);
      require(selected != fields.end(), "unknown native performance operation");
      std::set<std::string> allowed = {
          "schema",          "operation",
          "session_ref",     "expected_transport_epoch",
          "expected_source", "expected_body_revision"};
      allowed.insert(selected->second.begin(), selected->second.end());
      J *original_saved_acoustic = nullptr;
      const bool has_original_saved_acoustic =
          op == "restore-current-receiving" &&
          json_object_object_get_ex(request, "original_saved_acoustic",
                                    &original_saved_acoustic);
      if (has_original_saved_acoustic)
        allowed.insert("original_saved_acoustic");
      J *authored_source_transition = nullptr;
      const bool has_authored_source_transition =
          op == "receiving-transport-replace" &&
          json_object_object_get_ex(request, "authored_source_transition",
                                    &authored_source_transition);
      if (has_authored_source_transition)
        allowed.insert("authored_source_transition");
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
        accepted = native_capture_transport::start_device(*owner_);
      else if (op == "device-stop")
        accepted = owner_->stop_device();
      else if (op == "device-recover")
        accepted = owner_->recover_device();
      else if (op == "device-close")
        accepted = owner_->close_device();
      else if (op == "score") {
        auto operation = wire::read_operation(packet::field(request, "event"));
        json_object *input_ref = nullptr;
        require(json_object_object_get_ex(request, "input_ref", &input_ref),
                "missing native performance field: input_ref");
        const Ref original_input =
            (!input_ref || json_object_is_type(input_ref, json_type_null))
                ? Ref{}
                : reference(packet::string(input_ref).c_str());
        const auto admitted =
            owner_->enqueue_score_input_admission(operation, original_input);
        accepted = admitted.result() == Result::Accepted;
        reason = result(admitted.result());
        if (admitted.queue().queued())
          wire::put(payload.get(), "score_admission",
                    score_admission(admitted).release());
        timing_.score(admitted);
      } else if (op == "timing") {
        // Numerical selector only. Actual native facts are retained privately
        // by this Control; the caller cannot supply a receipt or epoch grant.
        const auto moment = packet::string(packet::field(request, "moment"));
        require(moment == "boundary" || moment == "score" ||
                    moment == "clock" || moment == "applied",
                "unsupported actual native timing moment");
        wire::decimal(packet::field(request, "ordinal"));
        accepted = true;
      } else if (op == "source-body-transition") {
        auto *cells = packet::field(request, "native_catalog");
        require(json_object_is_type(cells, json_type_array) &&
                    json_object_array_length(cells) <= 192,
                "native Form catalogue bound differs");
        std::vector<KeyboardCell> catalog;
        for (std::size_t i = 0; i < json_object_array_length(cells); ++i)
          catalog.push_back(read_key(json_object_array_get_idx(cells, i)));
        auto applied = form_transport::apply(
            *owner_, request, prepared_source_.get(), prepared_basis_.get(),
            retained_acoustic_.get(), last_physical_request_id_,
            std::move(catalog));
        prepared_source_ = std::move(applied.source);
        prepared_basis_ = std::move(applied.basis);
        retained_acoustic_ = std::move(applied.acoustic);
        standing_.source_form = true;
        standing_.recipe = applied.recipe;
        standing_.validated_generation = applied.generation;
        admitted_route_count_ = applied.route_count;
        last_physical_request_id_ = applied.request_id;
        wire::put(payload.get(), "physical_transition",
                  applied.acknowledgement.release());
        wire::put(payload.get(), "body_descriptor",
                  applied.body_descriptor.release());
        accepted = true;
      } else if (op == "receiving-transport-install" ||
                 op == "receiving-transport-replace") {
        // The private current Scene/Act reader qualifies the complete source
        // before this numerical worker seam. No packet supplied here mints it.
        auto &engine = *owner_->native().engine;
        const auto state = owner_->device_receipt().state;
        require(state == DeviceState::Closed || state == DeviceState::Prepared,
                "native acoustic change requires the attached device stopped");
        auto guard = engine.acquire_stopped_custody();
        require(bool(guard),
                "native acoustic exclusive stopped custody absent");
        const auto cursor =
            wire::decimal(packet::field(request, "expected_sample"));
        require(cursor == engine.samples_elapsed() && prepared_source_ &&
                    owner_->native().body->samples_elapsed() == cursor,
                "native acoustic change detached from actual P/audio cursor");
        auto *candidate_packet = packet::field(request, "prepared_acoustic");
        auto *current_packet = packet::field(request, "current_acoustic");
        auto *source_body =
            packet::field(prepared_source_.get(), "physical_body");
        const auto &immutable = owner_->native().body->preparation();
        // Full candidate producer text is copied before the first mutation.
        J *candidate_copy = nullptr;
        const auto copied =
            json_object_deep_copy(candidate_packet, &candidate_copy, nullptr);
        auto next_source = wire::own(candidate_copy);
        require(copied == 0 && bool(next_source),
                "native acoustic immutable copy failed");
        if (op == "receiving-transport-install") {
          require(!retained_acoustic_,
                  "native acoustic first installation already retained");
          auto prepared = acoustic_wire::read_prepared_acoustic(
              candidate_packet, current_packet, source_body, immutable, cursor);
          auto numerical = std::make_shared<MovingReceivingPortBinding>(
              owner_->native().body, immutable, cursor, std::move(prepared));
          accepted = engine.install_receiving_port(numerical->port(numerical),
                                                   guard, cursor);
          if (accepted) {
            retained_acoustic_ = std::move(next_source);
            owner_->refresh_stopped_reading(guard);
          } else {
            reason = "native receiving first-install preflight refused";
          }
        } else {
          auto *original_packet = packet::field(request, "before_acoustic");
          require(
              retained_acoustic_ &&
                  json_object_equal(original_packet, retained_acoustic_.get()),
              "native receiver change lost the complete original operative "
              "producer");
          auto saved = std::make_unique<Engine::Checkpoint>();
          engine.write_checkpoint(*saved, guard);
          require(saved->has_receiving && saved->cursor == cursor &&
                      valid_receiving_checkpoint(saved->receiving,
                                                 saved->receiving.manifest),
                  "native receiver change has no actual full retained ring");
          const auto original_origin =
              wire::decimal(packet::field(original_packet, "origin_sample"));
          const auto original_birth = wire::decimal(
              packet::field(original_packet, "history_origin_sample"));
          require(
              original_birth == saved->receiving.history_start_sample &&
                  original_birth <= original_origin &&
                  original_origin <= cursor,
              "native receiver change reset actual immutable history birth");
          auto original = acoustic_wire::read_prepared_acoustic(
              original_packet, retained_acoustic_.get(), source_body, immutable,
              original_origin,
              acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
          auto original_binding = std::make_shared<MovingReceivingPortBinding>(
              owner_->native().body, immutable, original_origin,
              std::move(original));
          auto original_manifest = original_binding->manifest();
          // A later segment retains the original history birth. The actual
          // ring independently fences that value above; all remaining fields
          // come from the full original numerical producer, not caller labels.
          original_manifest.history_origin_sample = original_birth;
          require(valid_receiving_manifest(original_manifest) &&
                      same_receiving_manifest(original_manifest,
                                              saved->receiving.manifest),
                  "native original numerical receiver differs from the actual "
                  "retained port");
          require(
              json_object_equal(packet::field(original_packet, "context"),
                                packet::field(candidate_packet, "context")),
              "geometry receiver change substituted complete original context");
          auto *before_config = packet::field(original_packet, "configuration");
          auto *after_config = packet::field(candidate_packet, "configuration");
          require(
              wire::decimal(
                  packet::field(candidate_packet, "history_origin_sample")) ==
                      original_birth &&
                  ql::physical_wire::exact(
                      packet::field(after_config, "revision")) >
                      ql::physical_wire::exact(
                          packet::field(before_config, "revision")),
              "native receiver change lost birth or original revision order");
          acoustic_wire::validate_receiver_replacement_source(
              original_packet, candidate_packet, prepared_source_.get(),
              authored_source_transition, cursor);
          // The actual dated receiver owner appends this effective source
          // epoch while retaining preceding retarded intervals and the ring.
          // The same private source/Act caller qualifies the complete AFTER
          // producer; packet equality grants no source authority.
          auto prepared = acoustic_wire::read_prepared_acoustic(
              candidate_packet, current_packet, source_body, immutable, cursor,
              acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
          auto numerical = std::make_shared<MovingReceivingPortBinding>(
              owner_->native().body, immutable, cursor, std::move(prepared),
              saved->receiving);
          auto token = std::make_unique<
              PerformanceManagement::PreparedReceivingReplacement>();
          const auto sequence = engine.accepted_sequence(),
                     epoch = owner_->transport_epoch();
          accepted = owner_->prepare_stopped_receiving_replacement(
                         numerical->port(numerical), guard, cursor, *token) &&
                     owner_->receiving_replacement_current(*token, guard);
          if (accepted) {
            // Prepare the complete acknowledgement before swapping the same
            // stopped port. It describes numerical custody, never authority.
            auto acknowledgement = wire::object();
            wire::text(acknowledgement.get(), "schema",
                       "ql.native-receiving-replacement/v1");
            wire::u64(acknowledgement.get(), "sample", cursor);
            wire::u64(acknowledgement.get(), "transport_epoch", epoch);
            wire::u64(acknowledgement.get(), "accepted_sequence", sequence);
            wire::put(acknowledgement.get(), "before_manifest",
                      wire::receiving_manifest(original_manifest).release());
            wire::put(
                acknowledgement.get(), "after_manifest",
                wire::receiving_manifest(token->after_manifest()).release());
            owner_->commit_stopped_receiving_replacement(*token, guard);
            retained_acoustic_ = std::move(next_source);
            owner_->refresh_stopped_reading(guard);
            wire::put(payload.get(), "receiving_replacement",
                      acknowledgement.release());
          } else {
            reason = "native receiver replacement "
                     "source/history/queue/catalogue preflight refused";
          }
        }
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
        // This is the actual stopped A/P callback verdict. An unavailable
        // output device is unrelated to this operation and cannot explain it.
        if (!accepted)
          reason = packet::string(packet::field(chunk.get(), "reason"));
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
      } else if (op == "restore-current-receiving") {
        // The existing private C/Rust owner holds the selected Act, original
        // occasion and current receiving lease across this exchange. This
        // worker validates complete numerical/source coherence on SAME P.
        require(prepared_source_ && prepared_basis_ &&
                    json_object_equal(
                        prepared_source_.get(),
                        packet::field(request, "current_source_packet")) &&
                    json_object_equal(
                        prepared_basis_.get(),
                        packet::field(request, "actual_native_basis")),
                "current receiving source differs from resident preparation");
        const auto original =
            checkpoint_text(packet::field(request, "original_checkpoint_wire"));
        require(!original.empty() && original.size() <= 32 * 1024 * 1024,
                "original receiving checkpoint exceeds native transport bound");
        auto tokener =
            std::unique_ptr<json_tokener, decltype(&json_tokener_free)>(
                json_tokener_new_ex(64), json_tokener_free);
        require(bool(tokener), "native checkpoint parser allocation failed");
        json_tokener_set_flags(tokener.get(), JSON_TOKENER_STRICT |
                                                  JSON_TOKENER_VALIDATE_UTF8);
        auto original_value = wire::own(json_tokener_parse_ex(
            tokener.get(), original.data(), int(original.size())));
        require(json_tokener_get_error(tokener.get()) == json_tokener_success &&
                    original_value &&
                    json_tokener_get_parse_end(tokener.get()) ==
                        original.size(),
                "original native receiving checkpoint text is incomplete");
        auto saved = management_checkpoint_transport::read_checkpoint_wire(
            original_value.get());
        const auto saved_cursor = saved->native_pair.audio.cursor;
        require(has_original_saved_acoustic ==
                    saved->native_pair.audio.has_receiving,
                "saved receiver requires exactly its original source operand");
        std::shared_ptr<MovingReceivingPortBinding> saved_receiver_owner;
        ReceivingPort saved_receiver{};
        wire::Json saved_acoustic_source{nullptr, json_object_put};
        if (has_original_saved_acoustic) {
          // The private Rust/Act lease regenerated this ORIGINAL segment.
          // Numerical decoding grants no current source or protected occasion.
          const auto original_origin = wire::decimal(
              packet::field(original_saved_acoustic, "origin_sample"));
          const auto original_birth = wire::decimal(
              packet::field(original_saved_acoustic, "history_origin_sample"));
          auto original_preparation = acoustic_wire::read_prepared_acoustic(
              original_saved_acoustic, original_saved_acoustic,
              packet::field(prepared_source_.get(), "physical_body"),
              owner_->native().body->preparation(), original_origin,
              acoustic_wire::AcousticReadKind::RetainedReceiverSegment);
          saved_receiver_owner =
              MovingReceivingPortBinding::from_saved_preparation(
                  owner_->native().body, owner_->native().body->preparation(),
                  saved_cursor, std::move(original_preparation), original_birth,
                  saved->native_pair.audio.receiving);
          saved_receiver = saved_receiver_owner->port(saved_receiver_owner);
          // Retain the complete producer before the first P/A mutation.
          J *copied = nullptr;
          const auto copied_ok =
              json_object_deep_copy(original_saved_acoustic, &copied, nullptr);
          saved_acoustic_source = wire::own(copied);
          require(copied_ok == 0 && bool(saved_acoustic_source),
                  "saved original acoustic producer copy failed");
        }
        const auto expected_cursor =
            wire::decimal(packet::field(request, "expected_cursor"));
        auto *basis = packet::field(request, "actual_native_basis");
        auto *current = packet::field(request, "current_receiving_admission");
        auto *candidate = packet::field(request, "receiving_admission");
        auto *complete = packet::field(request, "current_receiving");
        require(json_object_equal(packet::field(complete, "native_admission"),
                                  current),
                "complete native receiving disagrees with actual admission");
        auto *sources =
            packet::field(packet::field(current, "operation"), "sources");
        require(json_object_is_type(sources, json_type_array) &&
                    json_object_array_length(sources) <= 9 &&
                    !owner_->native().notes.empty(),
                "actual receiving programmes or M1 quadrature absent");
        std::vector<std::string> refs;
        for (std::size_t i = 0; i < json_object_array_length(sources); ++i)
          refs.push_back(
              packet::string(packet::field(
                  json_object_array_get_idx(sources, i), "driver_ref")) +
              "/m1-excitation-program");
        const auto immutable = owner_->native().body->preparation();
        auto admitted = std::make_shared<const AdmittedNativeReceivingSource>(
            read_native_receiving_admission(candidate, current,
                                            owner_->native(), basis, immutable,
                                            refs, saved_cursor));
        auto routes = std::make_shared<PhysicalRoutesPortBinding>(
            owner_->native().body, admitted, immutable,
            owner_->native().determination, basis,
            owner_->native().determination.m1_face == 1, saved_cursor);
        auto seed = std::make_unique<NativeRouteProgramSet>();
        seed->manifest = routes->manifest();
        seed->program_count = seed->manifest.route_count;
        seed->scalar_note_enabled = seed->manifest.scalar_note_enabled;
        seed->scalar_note_gain = seed->manifest.scalar_note_gain;
        for (std::size_t i = 0; i < seed->program_count; ++i) {
          auto &program = seed->programs[i];
          program.handle = seed->manifest.programs[i];
          program.phase_source_ref = seed->manifest.m1_coordinate;
          program.sine = owner_->native().notes.front().phase_sin;
          program.cosine = owner_->native().notes.front().phase_cos;
        }
        auto *cells = packet::field(request, "native_catalog");
        require(json_object_is_type(cells, json_type_array) &&
                    json_object_array_length(cells) == owner_->catalog().size(),
                "fresh receiving catalog differs from native current owner");
        std::vector<KeyboardCell> catalog;
        auto actual_catalog = wire::array();
        for (const auto &cell : owner_->catalog()) {
          auto original_key = key(cell);
          for (const char *name : {"hertz", "coordinate", "face", "ratio"})
            json_object_object_del(original_key.get(), name);
          if (cell.available)
            wire::put(original_key.get(), "native_target",
                      wire::note(cell.native_target).release());
          else
            null(original_key.get(), "native_target");
          wire::append(actual_catalog.get(), original_key.release());
        }
        require(
            json_object_equal(cells, actual_catalog.get()),
            "fresh receiving catalog changed original addresses or targets");
        for (std::size_t i = 0; i < json_object_array_length(cells); ++i)
          catalog.push_back(read_key(json_object_array_get_idx(cells, i)));
        auto before = owner_->stopped_checkpoint();
        auto before_wire =
            management_checkpoint_transport::checkpoint_wire(*before);
        std::unique_ptr<PerformanceManagement::PreparedReceivingRestore> next;
        TransportAcknowledgement ack{};
        accepted = restore_current_receiving_checkpoint(
            *owner_, *saved,
            physical_routes_port(physical_port(owner_->native().body), routes),
            *seed, owner_->native().notes, std::move(catalog), expected_cursor,
            packet::ref(request, "transaction_ref"),
            packet::ref(request, "checkpoint_ref"), next, ack,
            has_original_saved_acoustic ? &saved_receiver : nullptr);
        if (accepted) {
          retained_acoustic_ = std::move(saved_acoustic_source);
          auto operative = std::make_unique<ManagementCheckpoint>(
              next->original_checkpoint());
          operative->native_pair.audio =
              next->engine_candidate().admitted_checkpoint();
          auto operative_wire =
              management_checkpoint_transport::checkpoint_wire(*operative);
          readmission = wire::object();
          wire::text(readmission.get(), "schema",
                     "ql.native-receiving-readmission/v1");
          wire::text(readmission.get(), "original_checkpoint_wire", original);
          if (has_original_saved_acoustic)
            wire::put(readmission.get(), "original_saved_acoustic",
                      json_object_get(retained_acoustic_.get()));
          wire::text(readmission.get(), "operative_checkpoint_wire",
                     json_object_to_json_string_ext(operative_wire.get(),
                                                    JSON_C_TO_STRING_PLAIN));
          wire::text(readmission.get(), "before_checkpoint_wire",
                     json_object_to_json_string_ext(before_wire.get(),
                                                    JSON_C_TO_STRING_PLAIN));
          wire::put(readmission.get(), "current_receiving",
                    json_object_get(complete));
          wire::put(
              readmission.get(), "current_source_packet",
              json_object_get(packet::field(request, "current_source_packet")));
          wire::put(readmission.get(), "actual_native_basis",
                    json_object_get(basis));
          auto a = wire::object();
          wire::u64(a.get(), "previous_epoch", ack.previous_epoch);
          wire::u64(a.get(), "epoch", ack.epoch);
          wire::u64(a.get(), "previous_cursor", ack.previous_cursor);
          wire::u64(a.get(), "previous_sequence", ack.previous_sequence);
          wire::u64(a.get(), "target_sample", ack.target_sample);
          wire::u64(a.get(), "accepted_sequence", ack.accepted_sequence);
          wire::ref(a.get(), "transaction_ref", ack.transaction);
          wire::ref(a.get(), "checkpoint_ref", ack.checkpoint);
          wire::put(readmission.get(), "transport_ack",
                    json_object_get(a.get()));
          wire::put(payload.get(), "transport_ack", a.release());
          receiving_restore_.swap(next);
          admitted_route_count_ = seed->program_count;
        } else
          reason = "current native receiving stopped continuation refused";
      } else
        require(op == "inspect", "unknown native performance operation");
      if (op == "inspect")
        accepted = true;
      if (admission.has_stopped_queue) {
        accepted = admission.result == Result::Accepted;
        reason = admission.reason;
        if (admission.stopped_queue.queue().queued())
          wire::put(payload.get(), "score_admission",
                    score_admission(admission.stopped_queue).release());
        timing_.score(admission.stopped_queue);
      } else if (admission.clock.sequence ||
                 admission.result != Result::Unavailable ||
                 !admission.reason.empty()) {
        accepted = admission.result == Result::Accepted;
        reason = admission.reason;
        auto a = wire::object();
        wire::u64(a.get(), "sequence", admission.clock.sequence);
        wire::u64(a.get(), "sample", admission.clock.accepted_sample);
        wire::u64(a.get(), "requested_sample",
                  admission.clock.requested_sample);
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
        if (accepted)
          timing_.clock(admission.clock,
                        owner_->native().engine->source_for_native_admission(),
                        owner_->transport_epoch());
      }
    }
    if (!accepted && reason.empty())
      reason = owner_->device_receipt().error.empty()
                   ? "native operation refused"
                   : owner_->device_receipt().error;
    auto pulse = owner_->pulse();
    timing_.committed(*pulse, owner_->transport_epoch());
    if (op == "timing") {
      try {
        wire::put(payload.get(), "timing_fact",
                  timing_
                      .fact(*owner_, *pulse,
                            packet::string(packet::field(request, "moment")),
                            wire::decimal(packet::field(request, "ordinal")))
                      .release());
      } catch (const std::invalid_argument &refusal) {
        // This SAME pulse already owns the native applications/input journal.
        // Ordinary unavailable observation must return them intact below;
        // throwing here would destroy them before C/S could retain history.
        accepted = false;
        reason = refusal.what();
      }
    }
    if (op == "checkpoint" && accepted) {
      // The untouched payload checkpoint precedes this original same pulse.
      // Capture its actual feedback-complete successor without another pulse
      // or P/audio clock. Selected-source BEFORE uses these native bytes.
      auto after_pulse = owner_->stopped_checkpoint();
      auto after_wire =
          management_checkpoint_transport::checkpoint_wire(*after_pulse);
      wire::text(payload.get(), "checkpoint_after_pulse_wire",
                 json_object_to_json_string_ext(after_wire.get(),
                                                JSON_C_TO_STRING_PLAIN));
    }
    if (readmission) {
      // This checkpoint follows the actual pulse's observer drain. It proves
      // the unchanged operative->after FIFO transition in the original C24D
      // comparator instead of inventing an empty journal or a second clock.
      auto after = owner_->stopped_checkpoint();
      auto after_wire =
          management_checkpoint_transport::checkpoint_wire(*after);
      wire::text(readmission.get(), "after_checkpoint_wire",
                 json_object_to_json_string_ext(after_wire.get(),
                                                JSON_C_TO_STRING_PLAIN));
      wire::put(payload.get(), "receiving_readmission", readmission.release());
    }
    return serialize_pulse(op, accepted, reason, std::move(payload), *pulse);
  }
};
} // namespace ql::performance::management_transport
#endif
