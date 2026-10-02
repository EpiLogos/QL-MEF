#ifndef QL_PERFORMANCE_CHECKPOINT_WIRE_HPP
#define QL_PERFORMANCE_CHECKPOINT_WIRE_HPP
#include <ql/performance_checkpoint.hpp>
#include <ql/performance_packet.hpp>
#include <ql/physical_checkpoint_wire.hpp>
namespace ql::performance::checkpoint_transport {
using J = json_object;
using Json = ql::physical_wire::Json;
using packet::boolean;
using packet::byte;
using packet::field;
using packet::integer;
using packet::number;
using ql::physical_wire::keys;
using ql::physical_wire::own;
inline Json object() {
  auto value = own(json_object_new_object());
  require(bool(value), "checkpoint object allocation failed");
  return value;
}
inline Json array() {
  auto value = own(json_object_new_array());
  require(bool(value), "checkpoint array allocation failed");
  return value;
}
inline void put(J *object, const char *key, J *value) {
  ql::physical_wire::checkpoint_put(object, key, value);
}
inline void text(J *object, const char *key, const std::string &value) {
  ql::physical_wire::checkpoint_string(object, key, value);
}
inline void ref(J *object, const char *key, const Ref &value) {
  require(valid_ref(value), "checkpoint source reference required");
  text(object, key, value.data());
}
inline void u64(J *object, const char *key, std::uint64_t value) {
  ql::physical_wire::checkpoint_integer(object, key, value);
}
inline void real(J *object, const char *key, double value) {
  require(std::isfinite(value), "nonfinite audio checkpoint state");
  put(object, key, json_object_new_double(value));
}
inline void flag(J *object, const char *key, bool value) {
  put(object, key, json_object_new_boolean(value));
}
inline std::uint64_t decimal(J *value) {
  return ql::physical_wire::checkpoint_decimal(value);
}
inline void append(J *array, J *value) {
  require(value && json_object_array_add(array, value) == 0,
          "checkpoint array allocation failed");
}
template <std::size_t N>
inline Json doubles(const std::array<double, N> &values) {
  auto out = array();
  for (double v : values) {
    require(std::isfinite(v), "nonfinite checkpoint array");
    append(out.get(), json_object_new_double(v));
  }
  return out;
}
template <std::size_t N>
inline void read_doubles(J *input, std::array<double, N> &out) {
  packet::array(input, N);
  for (std::size_t i = 0; i < N; ++i)
    out[i] = number(json_object_array_get_idx(input, i));
}
inline Json identity(const Identity &value) {
  auto out = object();
  ref(out.get(), "instance", value.instance);
  ref(out.get(), "event", value.event);
  ref(out.get(), "subject", value.subject);
  u64(out.get(), "m1_revision", value.m1_revision);
  u64(out.get(), "m2_generation", value.m2_generation);
  return out;
}
inline Json determination(const Determination &d) {
  auto out = object();
  put(out.get(), "identity", identity(d.identity).release());
  for (const auto &entry : {std::pair{"m1_coordinate", &d.m1_coordinate},
                            {"m2_writer", &d.m2_writer},
                            {"registry_revision", &d.registry_revision},
                            {"source_revision", &d.source_revision},
                            {"relation_plan_ref", &d.relation_plan_ref},
                            {"tuning_ref", &d.tuning_ref},
                            {"native_receipt_ref", &d.native_receipt_ref},
                            {"body_preparation_ref", &d.body_preparation_ref},
                            {"body_state_ref", &d.body_state_ref}})
    ref(out.get(), entry.first, *entry.second);
  for (const auto &entry : {std::pair{"m1_face", std::uint64_t(d.m1_face)},
                            {"m2_face", d.m2_face},
                            {"tick12", d.tick12},
                            {"degree720", d.degree720},
                            {"basis", d.basis},
                            {"lens12", d.lens12},
                            {"context_frame", d.context_frame}})
    put(out.get(), entry.first, json_object_new_uint64(entry.second));
  u64(out.get(), "body_revision", d.body_revision);
  flag(out.get(), "tuning_available", d.tuning_available);
  put(out.get(), "audio_octet_hz", doubles(d.audio_octet_hz).release());
  auto nodal = array();
  for (const auto &n : d.nodal_quartet) {
    auto row = object();
    put(row.get(), "position", json_object_new_int(n.position));
    put(row.get(), "face", json_object_new_int(n.face));
    put(row.get(), "m", json_object_new_int(n.m));
    put(row.get(), "n", json_object_new_int(n.n));
    append(nodal.get(), row.release());
  }
  put(out.get(), "nodal_quartet", nodal.release());
  const auto &p = d.excitation;
  auto policy = object();
  ref(policy.get(), "policy_ref", p.policy_ref);
  ref(policy.get(), "standing", p.standing);
  put(policy.get(), "scaling", json_object_new_int(unsigned(p.scaling)));
  real(policy.get(), "reference_hertz", p.reference_hertz);
  real(policy.get(), "root_linear", p.root_linear);
  real(policy.get(), "octet_linear", p.octet_linear);
  put(policy.get(), "weights", doubles(p.weights).release());
  put(out.get(), "excitation", policy.release());
  return out;
}
inline Json note(const NoteTarget &n) {
  auto out = object();
  put(out.get(), "identity", identity(n.identity).release());
  ref(out.get(), "source_coordinate", n.source_coordinate);
  ref(out.get(), "tuning_ref", n.tuning_ref);
  ref(out.get(), "touch_ref", n.touch_ref);
  for (const auto &e : {std::pair{"member", n.member},
                        {"touch", n.touch},
                        {"ratio_numerator", n.ratio_numerator},
                        {"ratio_denominator", n.ratio_denominator}})
    u64(out.get(), e.first, e.second);
  for (const auto &e : {std::pair{"key", unsigned(n.key)},
                        {"position", unsigned(n.position)},
                        {"coordinate_face", unsigned(n.coordinate_face)},
                        {"source_face", unsigned(n.source_face)},
                        {"pitch_class", unsigned(n.pitch_class)}})
    put(out.get(), e.first, json_object_new_int(e.second));
  put(out.get(), "register_octave", json_object_new_int(n.register_octave));
  real(out.get(), "fundamental_hz", n.fundamental_hz);
  real(out.get(), "hertz", n.hertz);
  real(out.get(), "phase_sin", n.phase_sin);
  real(out.get(), "phase_cos", n.phase_cos);
  flag(out.get(), "exact_ratio", n.exact_ratio);
  return out;
}
inline Json parameters(const Parameters &p) {
  auto out = object();
  for (const auto &e : {std::pair{"force_newtons", p.force_newtons},
                        {"attack_seconds", p.attack_seconds},
                        {"release_seconds", p.release_seconds},
                        {"cutoff_hertz", p.cutoff_hertz},
                        {"master_linear", p.master_linear},
                        {"body_linear", p.body_linear},
                        {"monitor_linear", p.monitor_linear}})
    real(out.get(), e.first, e.second);
  return out;
}
inline Parameters read_parameters(J *in) {
  keys(in, {"force_newtons", "attack_seconds", "release_seconds",
            "cutoff_hertz", "master_linear", "body_linear", "monitor_linear"});
  return {
      number(field(in, "force_newtons")),   number(field(in, "attack_seconds")),
      number(field(in, "release_seconds")), number(field(in, "cutoff_hertz")),
      number(field(in, "master_linear")),   number(field(in, "body_linear")),
      number(field(in, "monitor_linear"))};
}
inline Json voice(const Engine::Voice &v) {
  auto out = object();
  put(out.get(), "note", note(v.note).release());
  u64(out.get(), "born", v.born);
  flag(out.get(), "release", v.release);
  for (const auto &e : {std::pair{"sine", v.sine},
                        {"cosine", v.cosine},
                        {"envelope", v.envelope},
                        {"velocity", v.velocity},
                        {"pressure", v.pressure},
                        {"frequency", v.frequency},
                        {"target_frequency", v.target_frequency},
                        {"filter", v.filter},
                        {"root_gain", v.root_gain},
                        {"octet_gain", v.octet_gain}})
    real(out.get(), e.first, e.second);
  put(out.get(), "release_remaining",
      json_object_new_uint64(v.release_remaining));
  put(out.get(), "octet_sine", doubles(v.octet_sine).release());
  put(out.get(), "octet_cosine", doubles(v.octet_cosine).release());
  put(out.get(), "octet_frequency", doubles(v.octet_frequency).release());
  put(out.get(), "octet_weight", doubles(v.octet_weight).release());
  return out;
}
inline Engine::Voice read_voice(J *in) {
  keys(in, {"note", "born", "release", "sine", "cosine", "envelope", "velocity",
            "pressure", "frequency", "target_frequency", "filter", "root_gain",
            "octet_gain", "release_remaining", "octet_sine", "octet_cosine",
            "octet_frequency", "octet_weight"});
  Engine::Voice out{};
  out.active = true;
  out.note = packet::note(field(in, "note"));
  out.born = decimal(field(in, "born"));
  out.release = boolean(field(in, "release"));
  for (const auto &e : {std::pair{"sine", &out.sine},
                        {"cosine", &out.cosine},
                        {"envelope", &out.envelope},
                        {"velocity", &out.velocity},
                        {"pressure", &out.pressure},
                        {"frequency", &out.frequency},
                        {"target_frequency", &out.target_frequency},
                        {"filter", &out.filter},
                        {"root_gain", &out.root_gain},
                        {"octet_gain", &out.octet_gain}})
    *e.second = number(field(in, e.first));
  const auto remaining = integer(field(in, "release_remaining"));
  require(remaining <= std::numeric_limits<std::uint32_t>::max(),
          "release counter overflow");
  out.release_remaining = std::uint32_t(remaining);
  read_doubles(field(in, "octet_sine"), out.octet_sine);
  read_doubles(field(in, "octet_cosine"), out.octet_cosine);
  read_doubles(field(in, "octet_frequency"), out.octet_frequency);
  read_doubles(field(in, "octet_weight"), out.octet_weight);
  return out;
}
inline Json native_clock(const NativeClockMetadata &clock) {
  auto out = object();
  u64(out.get(), "epoch", clock.epoch);
  u64(out.get(), "anchor_ordinal", clock.anchor_ordinal);
  u64(out.get(), "trigger_host_ticks", clock.trigger_host_ticks);
  u64(out.get(), "admitted_host_ticks", clock.admitted_host_ticks);
  real(out.get(), "mapping_uncertainty_samples",
       clock.mapping_uncertainty_samples);
  flag(out.get(), "input_transit_unknown", clock.input_transit_unknown);
  return out;
}
inline NativeClockMetadata read_native_clock(J *in) {
  J *clock = nullptr;
  if (!json_object_object_get_ex(in, "native_clock", &clock))
    return {}; // Original v1 scored checkpoints remain readable.
  keys(clock,
       {"epoch", "anchor_ordinal", "trigger_host_ticks", "admitted_host_ticks",
        "mapping_uncertainty_samples", "input_transit_unknown"});
  return {decimal(field(clock, "epoch")),
          decimal(field(clock, "anchor_ordinal")),
          decimal(field(clock, "trigger_host_ticks")),
          decimal(field(clock, "admitted_host_ticks")),
          number(field(clock, "mapping_uncertainty_samples")),
          boolean(field(clock, "input_transit_unknown"))};
}
inline void operation_keys(J *in, std::initializer_list<const char *> allowed) {
  require(in && json_object_get_type(in) == json_type_object,
          "checkpoint operation object required");
  J *clock = nullptr;
  const bool has_clock = json_object_object_get_ex(in, "native_clock", &clock);
  J *requested = nullptr;
  const bool has_requested =
      json_object_object_get_ex(in, "requested_sample", &requested);
  require(json_object_object_length(in) ==
              int(allowed.size()) + int(has_clock) + int(has_requested),
          "checkpoint operation fields missing/unknown");
  json_object_object_foreach(in, key, value) {
    (void)value;
    require((has_clock && std::strcmp(key, "native_clock") == 0) ||
                (has_requested && std::strcmp(key, "requested_sample") == 0) ||
                std::any_of(allowed.begin(), allowed.end(),
                            [&](const char *known) {
                              return std::strcmp(key, known) == 0;
                            }),
            "unknown checkpoint operation field");
  }
}
inline Json operation(const Operation &op) {
  auto out = object();
  put(out.get(), "identity", identity(op.identity).release());
  put(out.get(), "kind", json_object_new_int(unsigned(op.kind)));
  u64(out.get(), "sequence", op.sequence);
  u64(out.get(), "sample", op.sample);
  if (op.has_requested_sample)
    u64(out.get(), "requested_sample", op.requested_sample);
  u64(out.get(), "touch", op.touch);
  real(out.get(), "value", op.value);
  real(out.get(), "pitch_hz", op.pitch_hz);
  put(out.get(), "parameter", json_object_new_int(unsigned(op.parameter)));
  flag(out.get(), "late_admitted", op.late_admitted);
  put(out.get(), "native_clock", native_clock(op.native_clock).release());
  // Discriminated optional objects have explicit presence flags. This avoids
  // emitting invalid unused source refs or trusting an unparsed shadow value.
  flag(out.get(), "has_note",
       op.kind == Kind::NoteOn ||
           (op.kind == Kind::Expression && op.pitch_hz > 0));
  if (op.kind == Kind::NoteOn ||
      (op.kind == Kind::Expression && op.pitch_hz > 0))
    put(out.get(), "note", note(op.note).release());
  flag(out.get(), "has_determination", op.kind == Kind::Determination);
  if (op.kind == Kind::Determination)
    put(out.get(), "determination", determination(op.determination).release());
  return out;
}
inline Operation read_operation(J *in) {
  const bool has_note = boolean(field(in, "has_note")),
             has_determination = boolean(field(in, "has_determination"));
  if (has_note && has_determination)
    throw std::invalid_argument("operation has incompatible payloads");
  if (has_note)
    operation_keys(in, {"identity", "kind", "sequence", "sample", "touch",
                        "value", "pitch_hz", "parameter", "late_admitted",
                        "has_note", "has_determination", "note"});
  else if (has_determination)
    operation_keys(in, {"identity", "kind", "sequence", "sample", "touch",
                        "value", "pitch_hz", "parameter", "late_admitted",
                        "has_note", "has_determination", "determination"});
  else
    operation_keys(in, {"identity", "kind", "sequence", "sample", "touch",
                        "value", "pitch_hz", "parameter", "late_admitted",
                        "has_note", "has_determination"});
  Operation out{};
  out.identity = packet::identity(field(in, "identity"));
  out.kind = Kind(byte(field(in, "kind")));
  out.sequence = decimal(field(in, "sequence"));
  out.sample = decimal(field(in, "sample"));
  J *requested = nullptr;
  out.has_requested_sample =
      json_object_object_get_ex(in, "requested_sample", &requested);
  if (out.has_requested_sample)
    out.requested_sample = decimal(requested);
  out.touch = decimal(field(in, "touch"));
  out.value = number(field(in, "value"));
  out.pitch_hz = number(field(in, "pitch_hz"));
  out.parameter = Parameter(byte(field(in, "parameter")));
  out.late_admitted = boolean(field(in, "late_admitted"));
  out.native_clock = read_native_clock(in);
  require(has_note == (out.kind == Kind::NoteOn ||
                       (out.kind == Kind::Expression && out.pitch_hz > 0)) &&
              has_determination == (out.kind == Kind::Determination),
          "operation payload discriminator differs");
  if (has_note)
    out.note = packet::note(field(in, "note"));
  if (has_determination)
    out.determination = packet::determination(field(in, "determination"));
  return out;
}
inline Json release(const ReleaseOperation &op) {
  auto out = object();
  put(out.get(), "identity", identity(op.identity).release());
  put(out.get(), "kind", json_object_new_int(unsigned(op.kind)));
  u64(out.get(), "sequence", op.sequence);
  u64(out.get(), "sample", op.sample);
  if (op.has_requested_sample)
    u64(out.get(), "requested_sample", op.requested_sample);
  u64(out.get(), "touch", op.touch);
  flag(out.get(), "late_admitted", op.late_admitted);
  put(out.get(), "native_clock", native_clock(op.native_clock).release());
  return out;
}
inline ReleaseOperation read_release(J *in) {
  operation_keys(
      in, {"identity", "kind", "sequence", "sample", "touch", "late_admitted"});
  ReleaseOperation out{
      Kind(byte(field(in, "kind"))),  packet::identity(field(in, "identity")),
      decimal(field(in, "sequence")), decimal(field(in, "sample")),
      decimal(field(in, "touch")),    boolean(field(in, "late_admitted")),
      read_native_clock(in)};
  J *requested = nullptr;
  out.has_requested_sample =
      json_object_object_get_ex(in, "requested_sample", &requested);
  if (out.has_requested_sample)
    out.requested_sample = decimal(requested);
  return out;
}
inline const char *operation_name(Kind kind) {
  switch (kind) {
  case Kind::NoteOn:
    return "note_on";
  case Kind::NoteOff:
    return "note_off";
  case Kind::Sustain:
    return "sustain";
  case Kind::Expression:
    return "expression";
  case Kind::Panic:
    return "panic";
  case Kind::Parameter:
    return "parameter";
  case Kind::Determination:
    return "determination";
  }
  throw std::invalid_argument("unknown applied operation");
}
inline Json application(const NativeGestureApplication &a) {
  auto out = object();
  text(out.get(), "schema", "ql.performance-applied-event/v2");
  if (a.has_requested_sample)
    u64(out.get(), "requested_sample", a.requested_sample);
  text(out.get(), "operation", operation_name(a.kind));
  put(out.get(), "kind", json_object_new_int(unsigned(a.kind)));
  text(out.get(), "status", a.applied ? "applied" : "refused");
  flag(out.get(), "applied", a.applied);
  put(out.get(), "identity", identity(a.identity).release());
  put(out.get(), "native_clock", native_clock(a.clock).release());
  for (const auto &e :
       {std::pair{"applied_application_ordinal", a.applied_application_ordinal},
        {"sequence", a.sequence},
        {"admitted_sample", a.admitted_sample},
        {"applied_sample", a.applied_sample},
        {"committed_cursor", a.committed_cursor},
        {"body_revision", a.body_revision},
        {"touch", a.touch}})
    u64(out.get(), e.first, e.second);
  ref(out.get(), "preparation_ref", a.preparation_ref);
  ref(out.get(), "state_ref", a.state_ref);
  put(out.get(), "parameter", json_object_new_int(unsigned(a.parameter)));
  real(out.get(), "value", a.value);
  real(out.get(), "pitch_hz", a.pitch_hz);
  flag(out.get(), "has_determination", a.has_determination);
  if (a.has_determination)
    put(out.get(), "determination",
        determination(a.determined_source).release());
  else
    require(json_object_object_add(out.get(), "determination", nullptr) == 0,
            "null determination allocation failed");
  flag(out.get(), "has_note", a.has_note);
  if (a.has_note)
    put(out.get(), "note", note(a.note).release());
  else
    require(json_object_object_add(out.get(), "note", nullptr) == 0,
            "null note field allocation failed");
  flag(out.get(), "late_admitted", a.late_admitted);
  auto manifest = object();
  ref(manifest.get(), "event_ref", a.physical_event);
  ref(manifest.get(), "subject_ref", a.physical_subject);
  ref(manifest.get(), "source_coordinate", a.physical_source_coordinate);
  ref(manifest.get(), "source_revision", a.physical_source_revision);
  ref(manifest.get(), "eigenbasis_identity", a.eigenbasis);
  u64(manifest.get(), "source_generation", a.physical_source_generation);
  put(manifest.get(), "sample_rate",
      json_object_new_uint64(a.physical_sample_rate));
  flag(manifest.get(), "pratibimba", a.physical_pratibimba);
  put(out.get(), "physical_manifest", manifest.release());
  return out;
}
inline void
requested_application_keys(J *in, std::initializer_list<const char *> allowed) {
  require(in && json_object_get_type(in) == json_type_object,
          "applied event object required");
  J *requested = nullptr;
  const bool present =
      json_object_object_get_ex(in, "requested_sample", &requested);
  require(json_object_object_length(in) == int(allowed.size()) + int(present),
          "applied event fields missing/unknown");
  json_object_object_foreach(in, key, value) {
    (void)value;
    require((present && std::strcmp(key, "requested_sample") == 0) ||
                std::any_of(allowed.begin(), allowed.end(),
                            [&](const char *known) {
                              return std::strcmp(key, known) == 0;
                            }),
            "unknown applied event field");
  }
}
inline NativeGestureApplication read_application(J *in) {
  requested_application_keys(in, {"schema",
                                  "operation",
                                  "kind",
                                  "status",
                                  "applied",
                                  "identity",
                                  "native_clock",
                                  "applied_application_ordinal",
                                  "sequence",
                                  "admitted_sample",
                                  "applied_sample",
                                  "committed_cursor",
                                  "body_revision",
                                  "touch",
                                  "preparation_ref",
                                  "state_ref",
                                  "parameter",
                                  "value",
                                  "pitch_hz",
                                  "has_note",
                                  "note",
                                  "has_determination",
                                  "determination",
                                  "late_admitted",
                                  "physical_manifest"});
  require(packet::string(field(in, "schema")) ==
              "ql.performance-applied-event/v2",
          "applied-event schema differs");
  NativeGestureApplication out{};
  out.kind = Kind(byte(field(in, "kind")));
  out.applied = boolean(field(in, "applied"));
  require(packet::string(field(in, "operation")) == operation_name(out.kind) &&
              packet::string(field(in, "status")) ==
                  (out.applied ? "applied" : "refused"),
          "applied-event discriminator differs");
  out.identity = packet::identity(field(in, "identity"));
  out.clock = read_native_clock(in);
  out.applied_application_ordinal =
      decimal(field(in, "applied_application_ordinal"));
  require(out.applied_application_ordinal != 0,
          "zero committed application ordinal");
  out.sequence = decimal(field(in, "sequence"));
  out.admitted_sample = decimal(field(in, "admitted_sample"));
  J *requested = nullptr;
  out.has_requested_sample =
      json_object_object_get_ex(in, "requested_sample", &requested);
  if (out.has_requested_sample)
    out.requested_sample = decimal(requested);
  out.applied_sample = decimal(field(in, "applied_sample"));
  out.committed_cursor = decimal(field(in, "committed_cursor"));
  out.body_revision = decimal(field(in, "body_revision"));
  out.touch = decimal(field(in, "touch"));
  out.preparation_ref = packet::ref(in, "preparation_ref");
  out.state_ref = packet::ref(in, "state_ref");
  out.parameter = Parameter(byte(field(in, "parameter")));
  out.value = number(field(in, "value"));
  out.pitch_hz = number(field(in, "pitch_hz"));
  out.has_note = boolean(field(in, "has_note"));
  out.has_determination = boolean(field(in, "has_determination"));
  J *d = nullptr;
  require(json_object_object_get_ex(in, "determination", &d) &&
              out.has_determination == (out.kind == Kind::Determination),
          "applied determination discriminator differs");
  if (out.has_determination)
    out.determined_source = packet::determination(d);
  else
    require(d == nullptr || json_object_is_type(d, json_type_null),
            "unexpected determination payload");
  out.late_admitted = boolean(field(in, "late_admitted"));
  auto manifest = field(in, "physical_manifest");
  keys(manifest, {"event_ref", "subject_ref", "source_coordinate",
                  "source_revision", "eigenbasis_identity", "source_generation",
                  "sample_rate", "pratibimba"});
  out.physical_event = packet::ref(manifest, "event_ref");
  out.physical_subject = packet::ref(manifest, "subject_ref");
  out.physical_source_coordinate = packet::ref(manifest, "source_coordinate");
  out.physical_source_revision = packet::ref(manifest, "source_revision");
  out.eigenbasis = packet::ref(manifest, "eigenbasis_identity");
  out.physical_source_generation =
      decimal(field(manifest, "source_generation"));
  auto rate = packet::integer(field(manifest, "sample_rate"));
  require(rate >= 8000 && rate <= 192000, "historical physical rate differs");
  out.physical_sample_rate = std::uint32_t(rate);
  out.physical_pratibimba = boolean(field(manifest, "pratibimba"));
  require(out.physical_event == out.identity.event &&
              out.physical_subject == out.identity.subject &&
              out.physical_pratibimba,
          "original applied physical/event/face identity differs");
  J *n = nullptr;
  require(json_object_object_get_ex(in, "note", &n),
          "applied note field missing");
  if (out.has_note)
    out.note = packet::note(n);
  else
    require(n == nullptr || json_object_is_type(n, json_type_null),
            "unqualified applied note payload");
  require(unsigned(out.parameter) <= unsigned(Parameter::MonitorLinear) &&
              out.sequence && out.committed_cursor > out.applied_sample &&
              out.applied_sample >= out.admitted_sample &&
              (!out.has_requested_sample ||
               (out.requested_sample <= out.admitted_sample &&
                (out.requested_sample == out.admitted_sample ||
                 out.late_admitted))),
          "applied-event time/parameter bounds differ");
  return out;
}
inline Json recording(const RecordingStatus &status) {
  auto out = object();
  put(out.get(), "failure", json_object_new_int(unsigned(status.failure)));
  u64(out.get(), "dropped_applications", status.dropped_applications);
  u64(out.get(), "first_failed_sequence", status.first_failed_sequence);
  u64(out.get(), "first_failed_sample", status.first_failed_sample);
  return out;
}
inline RecordingStatus read_recording(J *in) {
  keys(in, {"failure", "dropped_applications", "first_failed_sequence",
            "first_failed_sample"});
  auto failure = byte(field(in, "failure"));
  require(failure <= unsigned(RecordingFailure::ApplicationScratchOverflow),
          "unknown recording failure");
  return {RecordingFailure(failure), decimal(field(in, "dropped_applications")),
          decimal(field(in, "first_failed_sequence")),
          decimal(field(in, "first_failed_sample"))};
}
// Original committed v2 had no route state. A complete route pair is the
// additive v2 encoding; neither a partial pair nor unknown state is admitted.
// Engine restore still requires exact port presence, so an N9 owner cannot
// restore a legacy scalar record or a record whose route pair was stripped.
enum class AudioCheckpointEncoding { OriginalV2Scalar, V2WithRoutePrograms };
inline AudioCheckpointEncoding audio_checkpoint_encoding(J *in) {
  require(in && json_object_is_type(in, json_type_object),
          "audio checkpoint object required");
  J *value = nullptr;
  const bool flag_present =
      json_object_object_get_ex(in, "has_route_programs", &value);
  const bool state_present =
      json_object_object_get_ex(in, "route_programs", &value);
  require(flag_present == state_present,
          "partial native route checkpoint extension");
  return flag_present ? AudioCheckpointEncoding::V2WithRoutePrograms
                      : AudioCheckpointEncoding::OriginalV2Scalar;
}
inline void audio_keys(J *in, std::initializer_list<const char *> names) {
  require(in && json_object_is_type(in, json_type_object),
          "audio checkpoint object required");
  J *v = nullptr;
  const bool apps = json_object_object_get_ex(in, "applications", &v);
  require(apps == bool(json_object_object_get_ex(in, "recording", &v)),
          "partial recording checkpoint extension");
  const bool proof = json_object_object_get_ex(in, "release_proof", &v);
  const bool routes = audio_checkpoint_encoding(in) ==
                      AudioCheckpointEncoding::V2WithRoutePrograms;
  require(json_object_object_length(in) == int(names.size()) + (apps ? 2 : 0) +
                                               (proof ? 1 : 0) +
                                               (routes ? 2 : 0),
          "audio checkpoint fields missing/unknown");
  json_object_object_foreach(in, key, value) {
    (void)value;
    require((routes && (std::strcmp(key, "has_route_programs") == 0 ||
                        std::strcmp(key, "route_programs") == 0)) ||
                (proof && std::strcmp(key, "release_proof") == 0) ||
                (apps && (std::strcmp(key, "applications") == 0 ||
                          std::strcmp(key, "recording") == 0)) ||
                std::any_of(
                    names.begin(), names.end(),
                    [&](const char *n) { return std::strcmp(key, n) == 0; }),
            "unknown audio checkpoint field");
  }
}
template <class T, std::size_t N, class Writer>
inline Json queue(const typename Spsc<T, N>::State &state, Writer writer) {
  require(Spsc<T, N>::valid_state(state), "checkpoint queue bounds differ");
  auto out = object();
  u64(out.get(), "read", state.read);
  u64(out.get(), "write", state.write);
  auto entries = array();
  for (auto i = state.read; i < state.write; ++i)
    append(entries.get(), writer(state.storage[i % N]).release());
  put(out.get(), "entries", entries.release());
  return out;
}
template <class T, std::size_t N, class Reader>
inline void read_queue(J *in, typename Spsc<T, N>::State &out, Reader reader) {
  keys(in, {"read", "write", "entries"});
  out.read = decimal(field(in, "read"));
  out.write = decimal(field(in, "write"));
  require(Spsc<T, N>::valid_state(out), "checkpoint queue budget exceeded");
  auto entries = field(in, "entries");
  packet::array(entries, std::size_t(out.write - out.read));
  for (auto i = out.read; i < out.write; ++i)
    out.storage[i % N] =
        reader(json_object_array_get_idx(entries, i - out.read));
}
inline Json route_handle(const ql::PhysicalForceRouteProgramHandle &v) {
  auto out = object();
  ref(out.get(), "driver_ref", v.driver_ref);
  ref(out.get(), "target_ref", v.target_ref);
  ref(out.get(), "program_ref", v.program_ref);
  ref(out.get(), "planet_coordinate", v.planet_coordinate);
  ref(out.get(), "chakra_coordinate", v.chakra_coordinate);
  ref(out.get(), "projection_ref", v.projection_ref);
  ref(out.get(), "calibration_ref", v.calibration_ref);
  ref(out.get(), "calibration_revision", v.calibration_revision);
  ref(out.get(), "calibration_source_ref", v.calibration_source_ref);
  ref(out.get(), "calibration_standing", v.calibration_standing);
  u64(out.get(), "route_index", v.route_index);
  u64(out.get(), "preparation_seal", v.preparation_seal);
  u64(out.get(), "program_seal", v.program_seal);
  u64(out.get(), "planet_node_id", v.planet_node_id);
  u64(out.get(), "chakra_node_id", v.chakra_node_id);
  u64(out.get(), "native_planet_index", v.native_planet_index);
  u64(out.get(), "centre_ordinal", v.centre_ordinal);
  u64(out.get(), "share_numerator", v.share_numerator);
  u64(out.get(), "share_denominator", v.share_denominator);
  real(out.get(), "source_hertz", v.source_hertz);
  real(out.get(), "original_denominator_share", v.original_denominator_share);
  real(out.get(), "peak_force_newtons", v.peak_force_newtons);
  return out;
}
inline ql::PhysicalForceRouteProgramHandle read_route_handle(J *in) {
  keys(in, {"driver_ref",
            "target_ref",
            "program_ref",
            "planet_coordinate",
            "chakra_coordinate",
            "projection_ref",
            "calibration_ref",
            "calibration_revision",
            "calibration_source_ref",
            "calibration_standing",
            "route_index",
            "preparation_seal",
            "program_seal",
            "planet_node_id",
            "chakra_node_id",
            "native_planet_index",
            "centre_ordinal",
            "share_numerator",
            "share_denominator",
            "source_hertz",
            "original_denominator_share",
            "peak_force_newtons"});
  ql::PhysicalForceRouteProgramHandle v{};
  v.driver_ref = packet::ref(in, "driver_ref");
  v.target_ref = packet::ref(in, "target_ref");
  v.program_ref = packet::ref(in, "program_ref");
  v.planet_coordinate = packet::ref(in, "planet_coordinate");
  v.chakra_coordinate = packet::ref(in, "chakra_coordinate");
  v.projection_ref = packet::ref(in, "projection_ref");
  v.calibration_ref = packet::ref(in, "calibration_ref");
  v.calibration_revision = packet::ref(in, "calibration_revision");
  v.calibration_source_ref = packet::ref(in, "calibration_source_ref");
  v.calibration_standing = packet::ref(in, "calibration_standing");
  const auto route_index = decimal(field(in, "route_index"));
  require(route_index <= std::numeric_limits<decltype(v.route_index)>::max(),
          "route integer width exceeded");
  v.route_index = decltype(v.route_index)(route_index);
  const auto preparation_seal = decimal(field(in, "preparation_seal"));
  require(preparation_seal <=
              std::numeric_limits<decltype(v.preparation_seal)>::max(),
          "route integer width exceeded");
  v.preparation_seal = decltype(v.preparation_seal)(preparation_seal);
  const auto program_seal = decimal(field(in, "program_seal"));
  require(program_seal <= std::numeric_limits<decltype(v.program_seal)>::max(),
          "route integer width exceeded");
  v.program_seal = decltype(v.program_seal)(program_seal);
  const auto planet_node_id = decimal(field(in, "planet_node_id"));
  require(planet_node_id <=
              std::numeric_limits<decltype(v.planet_node_id)>::max(),
          "route integer width exceeded");
  v.planet_node_id = decltype(v.planet_node_id)(planet_node_id);
  const auto chakra_node_id = decimal(field(in, "chakra_node_id"));
  require(chakra_node_id <=
              std::numeric_limits<decltype(v.chakra_node_id)>::max(),
          "route integer width exceeded");
  v.chakra_node_id = decltype(v.chakra_node_id)(chakra_node_id);
  const auto native_planet_index = decimal(field(in, "native_planet_index"));
  require(native_planet_index <=
              std::numeric_limits<decltype(v.native_planet_index)>::max(),
          "route integer width exceeded");
  v.native_planet_index = decltype(v.native_planet_index)(native_planet_index);
  const auto centre_ordinal = decimal(field(in, "centre_ordinal"));
  require(centre_ordinal <=
              std::numeric_limits<decltype(v.centre_ordinal)>::max(),
          "route integer width exceeded");
  v.centre_ordinal = decltype(v.centre_ordinal)(centre_ordinal);
  const auto share_numerator = decimal(field(in, "share_numerator"));
  require(share_numerator <=
              std::numeric_limits<decltype(v.share_numerator)>::max(),
          "route integer width exceeded");
  v.share_numerator = decltype(v.share_numerator)(share_numerator);
  const auto share_denominator = decimal(field(in, "share_denominator"));
  require(share_denominator <=
              std::numeric_limits<decltype(v.share_denominator)>::max(),
          "route integer width exceeded");
  v.share_denominator = decltype(v.share_denominator)(share_denominator);
  v.source_hertz = number(field(in, "source_hertz"));
  v.original_denominator_share =
      number(field(in, "original_denominator_share"));
  v.peak_force_newtons = number(field(in, "peak_force_newtons"));
  return v;
}
inline Json route_manifest(const ql::PhysicalForceRoutePortManifest &v) {
  auto out = object();
  ref(out.get(), "event_ref", v.event_ref);
  ref(out.get(), "subject_ref", v.subject_ref);
  ref(out.get(), "registry_revision", v.registry_revision);
  ref(out.get(), "source_revision", v.source_revision);
  ref(out.get(), "definition_ref", v.definition_ref);
  ref(out.get(), "source_instance_ref", v.source_instance_ref);
  ref(out.get(), "determination_ref", v.determination_ref);
  ref(out.get(), "preparation_ref", v.preparation_ref);
  ref(out.get(), "state_ref", v.state_ref);
  ref(out.get(), "eigenbasis_identity", v.eigenbasis_identity);
  ref(out.get(), "m1_coordinate", v.m1_coordinate);
  ref(out.get(), "m2_writer_coordinate", v.m2_writer_coordinate);
  ref(out.get(), "native_basis_sha256", v.native_basis_sha256);
  ref(out.get(), "m3_state_sha256", v.m3_state_sha256);
  u64(out.get(), "version", v.version);
  u64(out.get(), "sample_rate", v.sample_rate);
  u64(out.get(), "route_count", v.route_count);
  u64(out.get(), "source_basis_seal", v.source_basis_seal);
  u64(out.get(), "body_revision", v.body_revision);
  u64(out.get(), "admitted_cursor", v.admitted_cursor);
  u64(out.get(), "m1_revision", v.m1_revision);
  u64(out.get(), "m2_generation", v.m2_generation);
  u64(out.get(), "m3_generation", v.m3_generation);
  u64(out.get(), "m3_input_generation", v.m3_input_generation);
  u64(out.get(), "earth_frame_node_id", v.earth_frame_node_id);
  u64(out.get(), "tick12", v.tick12);
  u64(out.get(), "degree720", v.degree720);
  u64(out.get(), "temporal_phase", v.temporal_phase);
  real(out.get(), "scalar_note_gain", v.scalar_note_gain);
  real(out.get(), "legacy_native_scalar_gain", v.legacy_native_scalar_gain);
  real(out.get(), "max_force_newtons", v.max_force_newtons);
  flag(out.get(), "m1_pratibimba", v.m1_pratibimba);
  flag(out.get(), "m2_pratibimba", v.m2_pratibimba);
  flag(out.get(), "scalar_note_enabled", v.scalar_note_enabled);
  flag(out.get(), "legacy_native_scalar_enabled",
       v.legacy_native_scalar_enabled);
  require(v.route_count <= ql::physical_max_personal_force_routes,
          "route manifest bound exceeded");
  auto programs = array();
  for (std::size_t i = 0; i < v.route_count; ++i)
    append(programs.get(), route_handle(v.programs[i]).release());
  put(out.get(), "programs", programs.release());
  return out;
}
inline ql::PhysicalForceRoutePortManifest read_route_manifest(J *in) {
  keys(in, {"event_ref",
            "subject_ref",
            "registry_revision",
            "source_revision",
            "definition_ref",
            "source_instance_ref",
            "determination_ref",
            "preparation_ref",
            "state_ref",
            "eigenbasis_identity",
            "m1_coordinate",
            "m2_writer_coordinate",
            "native_basis_sha256",
            "m3_state_sha256",
            "version",
            "sample_rate",
            "route_count",
            "source_basis_seal",
            "body_revision",
            "admitted_cursor",
            "m1_revision",
            "m2_generation",
            "m3_generation",
            "m3_input_generation",
            "earth_frame_node_id",
            "tick12",
            "degree720",
            "temporal_phase",
            "scalar_note_gain",
            "legacy_native_scalar_gain",
            "max_force_newtons",
            "m1_pratibimba",
            "m2_pratibimba",
            "scalar_note_enabled",
            "legacy_native_scalar_enabled",
            "programs"});
  ql::PhysicalForceRoutePortManifest v{};
  v.event_ref = packet::ref(in, "event_ref");
  v.subject_ref = packet::ref(in, "subject_ref");
  v.registry_revision = packet::ref(in, "registry_revision");
  v.source_revision = packet::ref(in, "source_revision");
  v.definition_ref = packet::ref(in, "definition_ref");
  v.source_instance_ref = packet::ref(in, "source_instance_ref");
  v.determination_ref = packet::ref(in, "determination_ref");
  v.preparation_ref = packet::ref(in, "preparation_ref");
  v.state_ref = packet::ref(in, "state_ref");
  v.eigenbasis_identity = packet::ref(in, "eigenbasis_identity");
  v.m1_coordinate = packet::ref(in, "m1_coordinate");
  v.m2_writer_coordinate = packet::ref(in, "m2_writer_coordinate");
  v.native_basis_sha256 = packet::ref(in, "native_basis_sha256");
  v.m3_state_sha256 = packet::ref(in, "m3_state_sha256");
  const auto version = decimal(field(in, "version"));
  require(version <= std::numeric_limits<decltype(v.version)>::max(),
          "route integer width exceeded");
  v.version = decltype(v.version)(version);
  const auto sample_rate = decimal(field(in, "sample_rate"));
  require(sample_rate <= std::numeric_limits<decltype(v.sample_rate)>::max(),
          "route integer width exceeded");
  v.sample_rate = decltype(v.sample_rate)(sample_rate);
  const auto route_count = decimal(field(in, "route_count"));
  require(route_count <= std::numeric_limits<decltype(v.route_count)>::max(),
          "route integer width exceeded");
  v.route_count = decltype(v.route_count)(route_count);
  const auto source_basis_seal = decimal(field(in, "source_basis_seal"));
  require(source_basis_seal <=
              std::numeric_limits<decltype(v.source_basis_seal)>::max(),
          "route integer width exceeded");
  v.source_basis_seal = decltype(v.source_basis_seal)(source_basis_seal);
  const auto body_revision = decimal(field(in, "body_revision"));
  require(body_revision <=
              std::numeric_limits<decltype(v.body_revision)>::max(),
          "route integer width exceeded");
  v.body_revision = decltype(v.body_revision)(body_revision);
  const auto admitted_cursor = decimal(field(in, "admitted_cursor"));
  require(admitted_cursor <=
              std::numeric_limits<decltype(v.admitted_cursor)>::max(),
          "route integer width exceeded");
  v.admitted_cursor = decltype(v.admitted_cursor)(admitted_cursor);
  const auto m1_revision = decimal(field(in, "m1_revision"));
  require(m1_revision <= std::numeric_limits<decltype(v.m1_revision)>::max(),
          "route integer width exceeded");
  v.m1_revision = decltype(v.m1_revision)(m1_revision);
  const auto m2_generation = decimal(field(in, "m2_generation"));
  require(m2_generation <=
              std::numeric_limits<decltype(v.m2_generation)>::max(),
          "route integer width exceeded");
  v.m2_generation = decltype(v.m2_generation)(m2_generation);
  const auto m3_generation = decimal(field(in, "m3_generation"));
  require(m3_generation <=
              std::numeric_limits<decltype(v.m3_generation)>::max(),
          "route integer width exceeded");
  v.m3_generation = decltype(v.m3_generation)(m3_generation);
  const auto m3_input_generation = decimal(field(in, "m3_input_generation"));
  require(m3_input_generation <=
              std::numeric_limits<decltype(v.m3_input_generation)>::max(),
          "route integer width exceeded");
  v.m3_input_generation = decltype(v.m3_input_generation)(m3_input_generation);
  const auto earth_frame_node_id = decimal(field(in, "earth_frame_node_id"));
  require(earth_frame_node_id <=
              std::numeric_limits<decltype(v.earth_frame_node_id)>::max(),
          "route integer width exceeded");
  v.earth_frame_node_id = decltype(v.earth_frame_node_id)(earth_frame_node_id);
  const auto tick12 = decimal(field(in, "tick12"));
  require(tick12 <= std::numeric_limits<decltype(v.tick12)>::max(),
          "route integer width exceeded");
  v.tick12 = decltype(v.tick12)(tick12);
  const auto degree720 = decimal(field(in, "degree720"));
  require(degree720 <= std::numeric_limits<decltype(v.degree720)>::max(),
          "route integer width exceeded");
  v.degree720 = decltype(v.degree720)(degree720);
  const auto temporal_phase = decimal(field(in, "temporal_phase"));
  require(temporal_phase <=
              std::numeric_limits<decltype(v.temporal_phase)>::max(),
          "route integer width exceeded");
  v.temporal_phase = decltype(v.temporal_phase)(temporal_phase);
  v.scalar_note_gain = number(field(in, "scalar_note_gain"));
  v.legacy_native_scalar_gain = number(field(in, "legacy_native_scalar_gain"));
  v.max_force_newtons = number(field(in, "max_force_newtons"));
  v.m1_pratibimba = boolean(field(in, "m1_pratibimba"));
  v.m2_pratibimba = boolean(field(in, "m2_pratibimba"));
  v.scalar_note_enabled = boolean(field(in, "scalar_note_enabled"));
  v.legacy_native_scalar_enabled =
      boolean(field(in, "legacy_native_scalar_enabled"));
  require(v.route_count <= ql::physical_max_personal_force_routes,
          "route manifest bound exceeded");
  auto programs = field(in, "programs");
  packet::array(programs, v.route_count);
  for (std::size_t i = 0; i < v.route_count; ++i)
    v.programs[i] = read_route_handle(json_object_array_get_idx(programs, i));
  return v;
}
inline Json route_programs(const NativeRouteProgramSet &v) {
  auto out = object();
  text(out.get(), "schema", NativeRouteProgramSet::schema);
  u64(out.get(), "version", v.version);
  flag(out.get(), "owner_suspended", v.owner_suspended);
  flag(out.get(), "scalar_note_enabled", v.scalar_note_enabled);
  real(out.get(), "scalar_note_gain", v.scalar_note_gain);
  put(out.get(), "manifest", route_manifest(v.manifest).release());
  require(v.program_count <= ql::physical_max_personal_force_routes,
          "route program bound exceeded");
  auto programs = array();
  for (std::size_t i = 0; i < v.program_count; ++i) {
    const auto &p = v.programs[i];
    auto entry = object();
    put(entry.get(), "handle", route_handle(p.handle).release());
    ref(entry.get(), "phase_source_ref", p.phase_source_ref);
    u64(entry.get(), "waveform", unsigned(p.waveform));
    flag(entry.get(), "enabled", p.enabled);
    real(entry.get(), "target_gain", p.target_gain);
    real(entry.get(), "effective_gain", p.effective_gain);
    real(entry.get(), "sine", p.sine);
    real(entry.get(), "cosine", p.cosine);
    append(programs.get(), entry.release());
  }
  put(out.get(), "programs", programs.release());
  return out;
}
inline NativeRouteProgramSet read_route_programs(J *in) {
  keys(in, {"schema", "version", "owner_suspended", "scalar_note_enabled",
            "scalar_note_gain", "manifest", "programs"});
  require(packet::string(field(in, "schema")) ==
                  NativeRouteProgramSet::schema &&
              decimal(field(in, "version")) == 1,
          "route programme contract differs");
  NativeRouteProgramSet v{};
  v.owner_suspended = boolean(field(in, "owner_suspended"));
  v.scalar_note_enabled = boolean(field(in, "scalar_note_enabled"));
  v.scalar_note_gain = number(field(in, "scalar_note_gain"));
  v.manifest = read_route_manifest(field(in, "manifest"));
  auto programs = field(in, "programs");
  require(json_object_is_type(programs, json_type_array),
          "route programs required");
  v.program_count = json_object_array_length(programs);
  require(v.program_count <= ql::physical_max_personal_force_routes &&
              v.program_count == v.manifest.route_count,
          "route count differs");
  for (std::size_t i = 0; i < v.program_count; ++i) {
    auto p = json_object_array_get_idx(programs, i);
    keys(p, {"handle", "phase_source_ref", "waveform", "enabled", "target_gain",
             "effective_gain", "sine", "cosine"});
    auto &out = v.programs[i];
    out.handle = read_route_handle(field(p, "handle"));
    out.phase_source_ref = packet::ref(p, "phase_source_ref");
    require(decimal(field(p, "waveform")) == 0, "unknown route waveform");
    out.waveform = NativeRouteWaveform::Sinusoid;
    out.enabled = boolean(field(p, "enabled"));
    out.target_gain = number(field(p, "target_gain"));
    out.effective_gain = number(field(p, "effective_gain"));
    out.sine = number(field(p, "sine"));
    out.cosine = number(field(p, "cosine"));
  }
  return v;
}

inline Json audio_wire(const Engine::Checkpoint &cp) {
  auto out = object();
  text(out.get(), "schema", Engine::Checkpoint::schema);
  flag(out.get(), "has_route_programs", cp.has_route_programs);
  if (cp.has_route_programs)
    put(out.get(), "route_programs",
        route_programs(cp.route_programs).release());
  else
    require(json_object_object_add(out.get(), "route_programs", nullptr) == 0,
            "null route programs allocation failed");
  put(out.get(), "version", json_object_new_uint64(cp.version));
  text(out.get(), "model_revision", contract);
  put(out.get(), "sample_rate", json_object_new_uint64(cp.sample_rate));
  put(out.get(), "determination", determination(cp.determination).release());
  put(out.get(), "producer_determination",
      determination(cp.producer_determination).release());
  put(out.get(), "producer_identity", identity(cp.producer_identity).release());
  auto schedule = array();
  require(cp.source_schedule_size > 0 && cp.source_schedule_size <= 8,
          "checkpoint schedule budget exceeded");
  for (std::size_t i = 0; i < cp.source_schedule_size; ++i) {
    auto entry = object();
    u64(entry.get(), "from_sample", cp.source_schedule[i].from_sample);
    put(entry.get(), "determination",
        determination(cp.source_schedule[i].value).release());
    append(schedule.get(), entry.release());
  }
  put(out.get(), "source_schedule", schedule.release());
  auto voices = array();
  for (std::size_t i = 0; i < max_voices; ++i)
    if (cp.voices[i].active) {
      auto entry = object();
      put(entry.get(), "slot", json_object_new_uint64(i));
      put(entry.get(), "voice", voice(cp.voices[i]).release());
      append(voices.get(), entry.release());
    }
  put(out.get(), "voices", voices.release());
  auto touches = array();
  for (std::size_t i = 0; i < max_touches; ++i)
    if (cp.touches[i].token) {
      const auto &t = cp.touches[i];
      auto entry = object();
      put(entry.get(), "slot", json_object_new_uint64(i));
      u64(entry.get(), "token", t.token);
      u64(entry.get(), "member", t.member);
      real(entry.get(), "velocity", t.velocity);
      real(entry.get(), "pressure", t.pressure);
      ref(entry.get(), "source_touch_ref", t.source_touch_ref);
      put(entry.get(), "source_identity",
          identity(t.source_identity).release());
      if (t.original_note.member)
        put(entry.get(), "original_note", note(t.original_note).release());
      append(touches.get(), entry.release());
    }
  put(out.get(), "touches", touches.release());
  auto tails = array();
  for (std::size_t i = 0; i < max_tails; ++i)
    if (cp.tails[i].left) {
      auto entry = object();
      put(entry.get(), "slot", json_object_new_uint64(i));
      put(entry.get(), "left", json_object_new_uint64(cp.tails[i].left));
      flag(entry.get(), "active", cp.tails[i].voice.active);
      if (cp.tails[i].voice.active)
        put(entry.get(), "voice", voice(cp.tails[i].voice).release());
      append(tails.get(), entry.release());
    }
  put(out.get(), "tails", tails.release());
  put(out.get(), "operations",
      queue<Operation, queue_capacity>(cp.operations, operation).release());
  put(out.get(), "releases",
      queue<ReleaseOperation, 64>(cp.releases, release).release());
  auto pending = array();
  for (std::size_t i = 0; i < queue_capacity; ++i)
    if (cp.pending_operations[i].active) {
      auto entry = object();
      put(entry.get(), "slot", json_object_new_uint64(i));
      put(entry.get(), "operation",
          operation(cp.pending_operations[i].operation).release());
      append(pending.get(), entry.release());
    }
  put(out.get(), "pending_operations", pending.release());
  auto heap = array();
  require(cp.heap_size <= queue_capacity, "checkpoint heap budget exceeded");
  for (std::size_t i = 0; i < cp.heap_size; ++i)
    append(heap.get(), json_object_new_uint64(cp.operation_heap[i]));
  put(out.get(), "operation_heap", heap.release());
  auto pending_release = array();
  for (std::size_t i = 0; i < 64; ++i)
    if (cp.pending_releases[i].active) {
      auto entry = object();
      put(entry.get(), "slot", json_object_new_uint64(i));
      put(entry.get(), "release",
          release(cp.pending_releases[i].operation).release());
      append(pending_release.get(), entry.release());
    }
  put(out.get(), "pending_releases", pending_release.release());
  put(out.get(), "applications",
      queue<NativeGestureApplication, 256>(cp.applications, application)
          .release());
  put(out.get(), "recording", recording(cp.recording).release());
  auto proof = object();
  u64(proof.get(), "force_zero_samples", cp.force_zero_samples);
  u64(proof.get(), "emergency_requested", cp.emergency_requested);
  u64(proof.get(), "emergency_observed", cp.emergency_observed);
  u64(proof.get(), "emergency_applied_sample", cp.emergency_applied_sample);
  put(out.get(), "release_proof", proof.release());
  put(out.get(), "source_parameters", parameters(cp.source).release());
  put(out.get(), "effective_parameters", parameters(cp.effective).release());
  for (const auto &e :
       {std::pair{"cursor", cp.cursor},
        {"accepted_sequence", cp.accepted_sequence},
        {"accepted_sample", cp.accepted_sample},
        {"applied_sequence", cp.applied_sequence},
        {"applied_application_ordinal", cp.applied_application_ordinal},
        {"refused", cp.refused},
        {"late", cp.late},
        {"stolen", cp.stolen},
        {"dropped_readbacks", cp.dropped_readbacks},
        {"dropped_captures", cp.dropped_captures},
        {"clipping", cp.clipping},
        {"force_limited", cp.force_limited},
        {"overflow_count", cp.overflow_count},
        {"panic_fence", cp.panic_fence}})
    u64(out.get(), e.first, e.second);
  flag(out.get(), "emergency", cp.emergency);
  flag(out.get(), "capture", cp.capture);
  flag(out.get(), "fault", cp.fault);
  flag(out.get(), "sustain", cp.sustain);
  return out;
}
template <class Reader>
inline void read_slots(J *in, std::size_t limit, Reader reader) {
  require(in && json_object_is_type(in, json_type_array),
          "checkpoint slot array required");
  const auto count = json_object_array_length(in);
  require(count <= limit, "checkpoint slot budget exceeded");
  std::array<bool, queue_capacity> seen{};
  for (std::size_t i = 0; i < count; ++i) {
    auto row = json_object_array_get_idx(in, i);
    const auto slot = integer(field(row, "slot"));
    require(slot < limit && !seen[slot],
            "duplicate or invalid checkpoint slot");
    seen[slot] = true;
    reader(std::size_t(slot), row);
  }
}
inline void read_audio(J *in, Engine::Checkpoint &cp) {
  audio_keys(in, {"schema",
                  "version",
                  "model_revision",
                  "sample_rate",
                  "determination",
                  "producer_determination",
                  "producer_identity",
                  "source_schedule",
                  "voices",
                  "touches",
                  "tails",
                  "operations",
                  "releases",
                  "pending_operations",
                  "operation_heap",
                  "pending_releases",
                  "source_parameters",
                  "effective_parameters",
                  "cursor",
                  "accepted_sequence",
                  "accepted_sample",
                  "applied_sequence",
                  "applied_application_ordinal",
                  "refused",
                  "late",
                  "stolen",
                  "dropped_readbacks",
                  "dropped_captures",
                  "clipping",
                  "force_limited",
                  "overflow_count",
                  "panic_fence",
                  "emergency",
                  "capture",
                  "fault",
                  "sustain"});
  require(packet::string(field(in, "schema")) == Engine::Checkpoint::schema &&
              integer(field(in, "version")) == 2 &&
              packet::string(field(in, "model_revision")) == contract,
          "unsupported performance checkpoint model");
  cp.has_route_programs = false;
  cp.route_programs = {};
  if (audio_checkpoint_encoding(in) ==
      AudioCheckpointEncoding::V2WithRoutePrograms) {
    cp.has_route_programs = boolean(field(in, "has_route_programs"));
    J *routes = nullptr;
    require(json_object_object_get_ex(in, "route_programs", &routes),
            "route programs field missing");
    if (cp.has_route_programs)
      cp.route_programs = read_route_programs(routes);
    else
      require(!routes || json_object_is_type(routes, json_type_null),
              "inactive routes contain programme state");
  }
  const auto rate = integer(field(in, "sample_rate"));
  require(rate >= 8000 && rate <= 192000,
          "checkpoint sample rate outside budget");
  cp.sample_rate = unsigned(rate);
  cp.determination = packet::determination(field(in, "determination"));
  cp.producer_determination =
      packet::determination(field(in, "producer_determination"));
  cp.producer_identity = packet::identity(field(in, "producer_identity"));
  auto schedule = field(in, "source_schedule");
  require(json_object_is_type(schedule, json_type_array),
          "checkpoint source schedule required");
  cp.source_schedule_size = json_object_array_length(schedule);
  require(cp.source_schedule_size > 0 && cp.source_schedule_size <= 8,
          "checkpoint source budget exceeded");
  for (std::size_t i = 0; i < cp.source_schedule_size; ++i) {
    auto entry = json_object_array_get_idx(schedule, i);
    keys(entry, {"from_sample", "determination"});
    cp.source_schedule[i] = {
        packet::determination(field(entry, "determination")),
        decimal(field(entry, "from_sample"))};
  }
  read_slots(field(in, "voices"), max_voices, [&](auto slot, J *entry) {
    keys(entry, {"slot", "voice"});
    cp.voices[slot] = read_voice(field(entry, "voice"));
  });
  read_slots(field(in, "touches"), max_touches, [&](auto slot, J *entry) {
    J *original = nullptr;
    const bool has_original =
        json_object_object_get_ex(entry, "original_note", &original);
    if (has_original)
      keys(entry, {"slot", "token", "member", "velocity", "pressure",
                   "source_touch_ref", "source_identity", "original_note"});
    else
      keys(entry, {"slot", "token", "member", "velocity", "pressure",
                   "source_touch_ref", "source_identity"});
    cp.touches[slot] = {decimal(field(entry, "token")),
                        decimal(field(entry, "member")),
                        number(field(entry, "velocity")),
                        number(field(entry, "pressure")),
                        packet::ref(entry, "source_touch_ref"),
                        packet::identity(field(entry, "source_identity")),
                        has_original ? packet::note(original) : NoteTarget{}};
    require(cp.touches[slot].token != 0, "empty serialized touch");
  });
  read_slots(field(in, "tails"), max_tails, [&](auto slot, J *entry) {
    const bool active = boolean(field(entry, "active"));
    if (active)
      keys(entry, {"slot", "left", "active", "voice"});
    else
      keys(entry, {"slot", "left", "active"});
    const auto left = integer(field(entry, "left"));
    require(left > 0 && left <= 64, "checkpoint tail budget exceeded");
    cp.tails[slot].left = std::uint32_t(left);
    if (active)
      cp.tails[slot].voice = read_voice(field(entry, "voice"));
  });
  read_queue<Operation, queue_capacity>(field(in, "operations"), cp.operations,
                                        read_operation);
  read_queue<ReleaseOperation, 64>(field(in, "releases"), cp.releases,
                                   read_release);
  read_slots(field(in, "pending_operations"), queue_capacity,
             [&](auto slot, J *entry) {
               keys(entry, {"slot", "operation"});
               cp.pending_operations[slot] = {
                   true, read_operation(field(entry, "operation"))};
             });
  auto heap = field(in, "operation_heap");
  require(json_object_is_type(heap, json_type_array),
          "checkpoint heap required");
  cp.heap_size = json_object_array_length(heap);
  require(cp.heap_size <= queue_capacity, "checkpoint heap budget exceeded");
  for (std::size_t i = 0; i < cp.heap_size; ++i) {
    const auto slot = integer(json_object_array_get_idx(heap, i));
    require(slot < queue_capacity, "checkpoint heap index outside budget");
    cp.operation_heap[i] = std::size_t(slot);
  }
  read_slots(field(in, "pending_releases"), 64, [&](auto slot, J *entry) {
    keys(entry, {"slot", "release"});
    cp.pending_releases[slot] = {true, read_release(field(entry, "release"))};
  });
  J *apps = nullptr;
  if (json_object_object_get_ex(in, "applications", &apps)) {
    read_queue<NativeGestureApplication, 256>(apps, cp.applications,
                                              read_application);
    cp.recording = read_recording(field(in, "recording"));
  } else {
    cp.applications = {};
    cp.recording = {};
  }
  J *proof = nullptr;
  cp.force_zero_samples = cp.emergency_requested = cp.emergency_observed =
      cp.emergency_applied_sample = 0;
  if (json_object_object_get_ex(in, "release_proof", &proof)) {
    keys(proof, {"force_zero_samples", "emergency_requested",
                 "emergency_observed", "emergency_applied_sample"});
    cp.force_zero_samples = decimal(field(proof, "force_zero_samples"));
    cp.emergency_requested = decimal(field(proof, "emergency_requested"));
    cp.emergency_observed = decimal(field(proof, "emergency_observed"));
    cp.emergency_applied_sample =
        decimal(field(proof, "emergency_applied_sample"));
  }
  cp.source = read_parameters(field(in, "source_parameters"));
  cp.effective = read_parameters(field(in, "effective_parameters"));
  for (const auto &e :
       {std::pair{"cursor", &cp.cursor},
        {"accepted_sequence", &cp.accepted_sequence},
        {"accepted_sample", &cp.accepted_sample},
        {"applied_sequence", &cp.applied_sequence},
        {"applied_application_ordinal", &cp.applied_application_ordinal},
        {"refused", &cp.refused},
        {"late", &cp.late},
        {"stolen", &cp.stolen},
        {"dropped_readbacks", &cp.dropped_readbacks},
        {"dropped_captures", &cp.dropped_captures},
        {"clipping", &cp.clipping},
        {"force_limited", &cp.force_limited},
        {"overflow_count", &cp.overflow_count},
        {"panic_fence", &cp.panic_fence}})
    *e.second = decimal(field(in, e.first));
  cp.emergency = boolean(field(in, "emergency"));
  cp.capture = boolean(field(in, "capture"));
  cp.fault = boolean(field(in, "fault"));
  cp.sustain = boolean(field(in, "sustain"));
}
inline Json checkpoint_wire(const PairedCheckpoint &saved) {
  auto out = object();
  text(out.get(), "schema", PairedCheckpoint::schema);
  put(out.get(), "audio", audio_wire(saved.audio).release());
  put(out.get(), "physical",
      ql::physical_wire::checkpoint_wire(saved.physical).release());
  return out;
}
inline std::unique_ptr<PairedCheckpoint> read_checkpoint_wire(J *input) {
  keys(input, {"schema", "audio", "physical"});
  require(packet::string(field(input, "schema")) == PairedCheckpoint::schema,
          "unsupported paired checkpoint");
  auto out = std::make_unique<PairedCheckpoint>();
  read_audio(field(input, "audio"), out->audio);
  out->physical =
      ql::physical_wire::read_checkpoint_wire(field(input, "physical"));
  require(out->audio.cursor == out->physical.samples_elapsed,
          "checkpoint causal cursor disconnected");
  return out;
}
} // namespace ql::performance::checkpoint_transport
#endif
