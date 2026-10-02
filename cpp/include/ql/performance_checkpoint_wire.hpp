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
  require(json_object_object_length(in) == int(allowed.size()) + int(has_clock),
          "checkpoint operation fields missing/unknown");
  json_object_object_foreach(in, key, value) {
    (void)value;
    require((has_clock && std::strcmp(key, "native_clock") == 0) ||
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
  u64(out.get(), "touch", op.touch);
  flag(out.get(), "late_admitted", op.late_admitted);
  put(out.get(), "native_clock", native_clock(op.native_clock).release());
  return out;
}
inline ReleaseOperation read_release(J *in) {
  operation_keys(
      in, {"identity", "kind", "sequence", "sample", "touch", "late_admitted"});
  return {
      Kind(byte(field(in, "kind"))),  packet::identity(field(in, "identity")),
      decimal(field(in, "sequence")), decimal(field(in, "sample")),
      decimal(field(in, "touch")),    boolean(field(in, "late_admitted")),
      read_native_clock(in)};
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
inline Json audio_wire(const Engine::Checkpoint &cp) {
  auto out = object();
  text(out.get(), "schema", Engine::Checkpoint::schema);
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
  put(out.get(), "source_parameters", parameters(cp.source).release());
  put(out.get(), "effective_parameters", parameters(cp.effective).release());
  for (const auto &e : {std::pair{"cursor", cp.cursor},
                        {"accepted_sequence", cp.accepted_sequence},
                        {"accepted_sample", cp.accepted_sample},
                        {"applied_sequence", cp.applied_sequence},
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
  keys(in, {"schema",
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
              integer(field(in, "version")) == 1 &&
              packet::string(field(in, "model_revision")) == contract,
          "unsupported performance checkpoint model");
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
    keys(entry, {"slot", "token", "member", "velocity", "pressure",
                 "source_touch_ref", "source_identity"});
    cp.touches[slot] = {decimal(field(entry, "token")),
                        decimal(field(entry, "member")),
                        number(field(entry, "velocity")),
                        number(field(entry, "pressure")),
                        packet::ref(entry, "source_touch_ref"),
                        packet::identity(field(entry, "source_identity"))};
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
  cp.source = read_parameters(field(in, "source_parameters"));
  cp.effective = read_parameters(field(in, "effective_parameters"));
  for (const auto &e : {std::pair{"cursor", &cp.cursor},
                        {"accepted_sequence", &cp.accepted_sequence},
                        {"accepted_sample", &cp.accepted_sample},
                        {"applied_sequence", &cp.applied_sequence},
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
