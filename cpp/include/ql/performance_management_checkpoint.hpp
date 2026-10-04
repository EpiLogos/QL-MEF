#ifndef QL_PERFORMANCE_MANAGEMENT_CHECKPOINT_HPP
#define QL_PERFORMANCE_MANAGEMENT_CHECKPOINT_HPP
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/physical_transition.hpp>
namespace ql::performance {
struct NativeInputBinding {
  bool active = false, release_pending = false, press_applied = false;
  Ref input_ref{};
  NoteTarget target{};
  std::uint64_t press_sequence = 0, release_sequence = 0;
};
enum class InputBindingChange : std::uint8_t {
  PressAdmitted,
  ReleaseAdmitted,
  Applied,
  Refused,
  Retired
};
struct InputBindingRecord {
  std::uint64_t ordinal = 0, native_sequence = 0;
  InputBindingChange change = InputBindingChange::PressAdmitted;
  Kind operation = Kind::NoteOn;
  Ref input_ref{};
  NoteTarget target{};
};
// This is native serial-control custody, not a second musical store. Each pulse
// returns exact journal entries for retention by the existing C scene owner.
// An undrained journal refuses admission; it never drops input identity.
class NativeInputBindings {
public:
  struct State {
    std::array<NativeInputBinding, max_touches> inputs{};
    std::array<InputBindingRecord, 256> history{};
    std::uint64_t read = 0, write = 0, last_ordinal = 0, last_touch_token = 0,
                  last_member_token = 0;
  };

private:
  State state_{};
  bool record(InputBindingChange change, Kind operation,
              const NativeInputBinding &input,
              std::uint64_t sequence) noexcept {
    if (!can_record() || !sequence)
      return false;
    state_.history[state_.write++ % 256] = {
        ++state_.last_ordinal, sequence,    change, operation,
        input.input_ref,       input.target};
    return true;
  }

public:
  bool can_record() const noexcept {
    return state_.write - state_.read < 256 &&
           state_.write < std::numeric_limits<std::uint64_t>::max() &&
           state_.last_ordinal < std::numeric_limits<std::uint64_t>::max();
  }
  NativeInputBinding *find(const Ref &ref) noexcept {
    for (auto &input : state_.inputs)
      if (input.active && input.input_ref == ref)
        return &input;
    return nullptr;
  }
  std::uint64_t last_touch_token() const noexcept {
    return state_.last_touch_token;
  }
  std::uint64_t last_member_token() const noexcept {
    return state_.last_member_token;
  }
  std::uint64_t history_read() const noexcept { return state_.read; }
  std::uint64_t history_write() const noexcept { return state_.write; }
  std::uint64_t history_ordinal() const noexcept { return state_.last_ordinal; }
  auto &inputs() noexcept { return state_.inputs; }
  const auto &inputs() const noexcept { return state_.inputs; }
  bool bind(const Ref &input_ref, const NoteTarget &target,
            std::uint64_t accepted_sequence) noexcept {
    if (!valid_ref(input_ref) || find(input_ref) || !target.touch ||
        !target.member || !valid_ref(target.touch_ref) ||
        target.touch <= state_.last_touch_token || !can_record())
      return false;
    for (const auto &input : state_.inputs)
      if (input.active && input.target.touch == target.touch)
        return false;
    auto slot = std::find_if(state_.inputs.begin(), state_.inputs.end(),
                             [](const auto &input) { return !input.active; });
    if (slot == state_.inputs.end())
      return false;
    NativeInputBinding candidate{
        true, false, false, input_ref, target, accepted_sequence, 0};
    if (!record(InputBindingChange::PressAdmitted, Kind::NoteOn, candidate,
                accepted_sequence))
      return false;
    *slot = candidate;
    state_.last_touch_token = target.touch;
    state_.last_member_token =
        std::max(state_.last_member_token, target.member);
    return true;
  }
  bool release_admitted(const Ref &input_ref, std::uint64_t sequence) noexcept {
    auto *input = find(input_ref);
    if (!input || input->release_pending || sequence <= input->press_sequence ||
        !record(InputBindingChange::ReleaseAdmitted, Kind::NoteOff, *input,
                sequence))
      return false;
    input->release_pending = true;
    input->release_sequence = sequence;
    return true;
  }
  bool application(const NativeGestureApplication &applied) noexcept {
    for (auto &input : state_.inputs) {
      if (!input.active ||
          (applied.kind != Kind::Panic && input.target.touch != applied.touch))
        continue;
      if (!record(applied.applied ? InputBindingChange::Applied
                                  : InputBindingChange::Refused,
                  applied.kind, input, applied.sequence))
        return false;
      if (applied.applied && applied.kind == Kind::NoteOn &&
          applied.sequence == input.press_sequence)
        input.press_applied = true;
      if ((applied.applied &&
           (applied.kind == Kind::NoteOff || applied.kind == Kind::Panic)) ||
          (!applied.applied && applied.kind == Kind::NoteOn))
        input = {};
    }
    return true;
  }
  bool retire_absent(const Readback &reading) noexcept {
    for (auto &input : state_.inputs) {
      if (!input.active || !input.press_applied ||
          input.press_sequence > reading.last_sequence ||
          std::find(reading.held_touch_tokens.begin(),
                    reading.held_touch_tokens.end(),
                    input.target.touch) != reading.held_touch_tokens.end())
        continue;
      // Callback state explicitly ended this lifetime (including voice stealing
      // or out-of-band panic). This journal entry is a retirement, not an
      // invented performed NoteOff for C's score.
      if (!record(InputBindingChange::Retired, Kind::NoteOff, input,
                  reading.last_sequence))
        return false;
      input = {};
    }
    return true;
  }
  bool pop_history(InputBindingRecord &out) noexcept {
    if (state_.read == state_.write)
      return false;
    out = state_.history[state_.read++ % 256];
    return true;
  }
  void write_checkpoint(State &out) const { out = state_; }
  void restore_validated(const State &state) noexcept { state_ = state; }
  static bool validate(const State &state,
                       const Engine::Checkpoint &audio) noexcept {
    if (state.write < state.read || state.write - state.read > 256 ||
        state.last_ordinal != state.write)
      return false;
    auto lineage = [&](const Identity &id) {
      return id.instance == audio.determination.identity.instance &&
             id.event == audio.determination.identity.event &&
             id.subject == audio.determination.identity.subject &&
             id.m1_revision <= audio.producer_identity.m1_revision &&
             id.m2_generation <= audio.producer_identity.m2_generation;
    };
    auto same_target = [](const NoteTarget &a, const NoteTarget &b) {
      return a.identity == b.identity &&
             a.source_coordinate == b.source_coordinate &&
             a.tuning_ref == b.tuning_ref && a.touch_ref == b.touch_ref &&
             a.member == b.member && a.touch == b.touch &&
             a.ratio_numerator == b.ratio_numerator &&
             a.ratio_denominator == b.ratio_denominator && a.key == b.key &&
             a.position == b.position &&
             a.coordinate_face == b.coordinate_face &&
             a.source_face == b.source_face && a.pitch_class == b.pitch_class &&
             a.register_octave == b.register_octave &&
             a.fundamental_hz == b.fundamental_hz && a.hertz == b.hertz &&
             a.phase_sin == b.phase_sin && a.phase_cos == b.phase_cos &&
             a.exact_ratio == b.exact_ratio;
    };
    auto target_ok = [&](const NoteTarget &target) {
      return target.member && target.touch &&
             target.member <= state.last_member_token &&
             target.touch <= state.last_touch_token &&
             valid_ref(target.touch_ref) &&
             valid_ref(target.source_coordinate) &&
             valid_ref(target.tuning_ref) && lineage(target.identity) &&
             target.key < 12 && target.position == target.key / 2 &&
             target.coordinate_face == target.key % 2 &&
             target.source_face <= 1 && target.pitch_class < 12 &&
             std::isfinite(target.hertz) && target.hertz >= 0.001 &&
             target.hertz <= audio.sample_rate * 0.45 &&
             std::isfinite(target.fundamental_hz) &&
             target.fundamental_hz >= 0.001 &&
             std::isfinite(target.phase_sin) &&
             std::isfinite(target.phase_cos) &&
             std::abs(target.phase_sin * target.phase_sin +
                      target.phase_cos * target.phase_cos - 1) <= 1e-10 &&
             (target.exact_ratio
                  ? target.ratio_numerator && target.ratio_denominator &&
                        std::abs(target.hertz -
                                 target.fundamental_hz *
                                     double(target.ratio_numerator) /
                                     double(target.ratio_denominator)) <=
                            1e-11 * target.hertz
                  : !target.ratio_numerator && !target.ratio_denominator);
    };
    for (std::size_t n = 0; n < state.inputs.size(); ++n) {
      const auto &input = state.inputs[n];
      if (!input.active) {
        if (input.release_pending || input.press_applied ||
            input.press_sequence || input.release_sequence ||
            input.input_ref[0])
          return false;
        continue;
      }
      if (!valid_ref(input.input_ref) || !target_ok(input.target) ||
          !input.press_sequence ||
          input.press_sequence > audio.accepted_sequence ||
          (input.release_pending
               ? input.release_sequence <= input.press_sequence ||
                     input.release_sequence > audio.accepted_sequence
               : input.release_sequence != 0))
        return false;
      for (std::size_t old = 0; old < n; ++old)
        if (state.inputs[old].active &&
            (state.inputs[old].input_ref == input.input_ref ||
             state.inputs[old].target.touch == input.target.touch))
          return false;
      // The UI binding is tied to an actual resident/queued/applied lifetime,
      // including an unread applied release. It is never reconstructed from a
      // guessed source_touch_ref. Scored lifetimes need no UI input binding.
      bool known = false;
      for (const auto &touch : audio.touches)
        known |= touch.token == input.target.touch &&
                 touch.member == input.target.member &&
                 touch.source_touch_ref == input.target.touch_ref &&
                 touch.source_identity == input.target.identity &&
                 same_target(touch.original_note, input.target);
      auto is_attack = [&](const Operation &op) {
        return op.kind == Kind::NoteOn && op.sequence == input.press_sequence &&
               op.note.touch == input.target.touch &&
               op.note.member == input.target.member &&
               same_target(op.note, input.target);
      };
      for (auto i = audio.operations.read; i < audio.operations.write; ++i)
        known |= is_attack(audio.operations.storage[i % queue_capacity]);
      for (const auto &op : audio.pending_operations)
        if (op.active)
          known |= is_attack(op.operation);
      for (auto i = audio.applications.read; i < audio.applications.write;
           ++i) {
        const auto &a = audio.applications.storage[i % 256];
        known |=
            (a.touch == input.target.touch && a.has_note &&
             a.note.member == input.target.member &&
             a.note.touch_ref == input.target.touch_ref &&
             (a.kind != Kind::NoteOn || same_target(a.note, input.target))) ||
            (a.kind == Kind::Panic && a.applied &&
             a.sequence >= input.press_sequence);
      }
      if (!known)
        return false;
    }
    for (auto i = state.read; i < state.write; ++i) {
      const auto &entry = state.history[i % 256];
      if (entry.ordinal != i + 1 || !entry.native_sequence ||
          entry.native_sequence > audio.accepted_sequence ||
          !valid_ref(entry.input_ref) || !target_ok(entry.target) ||
          unsigned(entry.change) > unsigned(InputBindingChange::Retired) ||
          unsigned(entry.operation) > unsigned(Kind::Determination))
        return false;
    }
    return true;
  }
};
struct ManagementCheckpoint {
  static constexpr const char *schema =
      "ql.performance-management-checkpoint/v1";
  PairedCheckpoint native_pair{};
  NativeInputBindings::State bindings{};
  Ref session{};
  std::uint64_t transport_epoch = 0;
  bool recording_failed = false, release_pending = false, panic_applied = false;
  std::uint64_t release_request = 0, release_sequence = 0,
                release_proof_cursor = 0;
};
inline bool
valid_management_checkpoint(const ManagementCheckpoint &saved) noexcept {
  return valid_ref(saved.session) && saved.transport_epoch &&
         NativeInputBindings::validate(saved.bindings,
                                       saved.native_pair.audio) &&
         saved.release_request <= saved.native_pair.audio.emergency_requested &&
         saved.release_sequence <= saved.native_pair.audio.accepted_sequence &&
         saved.release_proof_cursor <= saved.native_pair.audio.cursor &&
         (!saved.panic_applied || saved.release_sequence) &&
         (!saved.release_pending || !saved.release_proof_cursor) &&
         (!saved.release_request || !saved.release_sequence);
}
inline bool
release_zero_proven(const Readback &reading,
                    std::uint64_t emergency_request = 0,
                    std::uint64_t applied_panic_sequence = 0) noexcept {
  return reading.available &&
         reading.physical.samples_elapsed == reading.samples_elapsed &&
         !reading.active_touches && !reading.active_voices &&
         !reading.active_tails && !reading.sustain &&
         reading.force_zero_samples >= 512 &&
         (emergency_request
              ? reading.emergency_observed >= emergency_request &&
                    reading.samples_elapsed >=
                        reading.emergency_applied_sample &&
                    reading.samples_elapsed -
                            reading.emergency_applied_sample >=
                        512
              : applied_panic_sequence &&
                    reading.last_sequence >= applied_panic_sequence);
}
// Executed only with stopped/acknowledged custody. Both P and A preflight
// before either mutates. This is also the historical-receipt regression seam;
// it does not claim a device-running material control has been joined.
inline bool apply_stopped_physical_transition(
    Engine &engine, const std::shared_ptr<ql::PhysicalBody> &body,
    ql::PreparedPhysicalTransition &transition, const Determination &after,
    const Engine::StoppedCustody &guard,
    ql::PhysicalLiveTransitionReceipt &receipt) {
  if (!body || !engine.owns_physical_owner(body.get()))
    return false;
  auto port = physical_port(body);
  const auto &in = transition.after_input();
  port.preparation = reference(in.preparation_ref.c_str());
  port.state = reference(in.state_ref.c_str());
  port.event = reference(in.event_ref.c_str());
  port.subject = reference(in.subject_ref.c_str());
  port.sample_rate = in.sample_rate;
  port.max_force_newtons = in.max_force_newtons;
  if (after.body_revision != in.body_revision ||
      after.body_preparation_ref != port.preparation ||
      !engine.preflight_stopped_physical_revision(after, port, guard) ||
      !transition.preflight(*body))
    return false;
  if (!transition.apply(*body, receipt))
    return false;
  if (!engine.commit_stopped_physical_revision(after, port, guard))
    std::terminate();
  return true;
}
namespace management_checkpoint_transport {
using namespace checkpoint_transport;
inline Json checkpoint_wire(const ManagementCheckpoint &saved) {
  require(valid_management_checkpoint(saved),
          "management input lifetime checkpoint disconnected");
  auto out = object();
  text(out.get(), "schema", ManagementCheckpoint::schema);
  ref(out.get(), "session_ref", saved.session);
  u64(out.get(), "transport_epoch", saved.transport_epoch);
  flag(out.get(), "recording_failed", saved.recording_failed);
  flag(out.get(), "release_pending", saved.release_pending);
  flag(out.get(), "panic_applied", saved.panic_applied);
  u64(out.get(), "release_request", saved.release_request);
  u64(out.get(), "release_sequence", saved.release_sequence);
  u64(out.get(), "release_proof_cursor", saved.release_proof_cursor);
  put(out.get(), "native_pair",
      checkpoint_transport::checkpoint_wire(saved.native_pair).release());
  auto inputs = array();
  for (std::size_t i = 0; i < saved.bindings.inputs.size(); ++i) {
    const auto &input = saved.bindings.inputs[i];
    if (!input.active)
      continue;
    auto entry = object();
    put(entry.get(), "slot", json_object_new_uint64(i));
    ref(entry.get(), "input_ref", input.input_ref);
    put(entry.get(), "target", note(input.target).release());
    flag(entry.get(), "release_pending", input.release_pending);
    flag(entry.get(), "press_applied", input.press_applied);
    u64(entry.get(), "press_sequence", input.press_sequence);
    u64(entry.get(), "release_sequence", input.release_sequence);
    append(inputs.get(), entry.release());
  }
  put(out.get(), "inputs", inputs.release());
  auto history = object();
  u64(history.get(), "read", saved.bindings.read);
  u64(history.get(), "write", saved.bindings.write);
  u64(history.get(), "last_ordinal", saved.bindings.last_ordinal);
  u64(history.get(), "last_touch_token", saved.bindings.last_touch_token);
  u64(history.get(), "last_member_token", saved.bindings.last_member_token);
  auto entries = array();
  for (auto i = saved.bindings.read; i < saved.bindings.write; ++i) {
    const auto &h = saved.bindings.history[i % 256];
    auto entry = object();
    u64(entry.get(), "ordinal", h.ordinal);
    u64(entry.get(), "native_sequence", h.native_sequence);
    put(entry.get(), "change", json_object_new_uint64(unsigned(h.change)));
    put(entry.get(), "operation",
        json_object_new_uint64(unsigned(h.operation)));
    ref(entry.get(), "input_ref", h.input_ref);
    put(entry.get(), "target", note(h.target).release());
    append(entries.get(), entry.release());
  }
  put(history.get(), "entries", entries.release());
  put(out.get(), "input_history", history.release());
  return out;
}
inline std::unique_ptr<ManagementCheckpoint> read_checkpoint_wire(J *in) {
  keys(in, {"schema", "session_ref", "transport_epoch", "recording_failed",
            "native_pair", "inputs", "input_history", "release_pending",
            "panic_applied", "release_request", "release_sequence",
            "release_proof_cursor"});
  require(packet::string(field(in, "schema")) == ManagementCheckpoint::schema,
          "management checkpoint schema differs");
  auto out = std::make_unique<ManagementCheckpoint>();
  out->session = packet::ref(in, "session_ref");
  out->transport_epoch = decimal(field(in, "transport_epoch"));
  out->recording_failed = boolean(field(in, "recording_failed"));
  out->release_pending = boolean(field(in, "release_pending"));
  out->panic_applied = boolean(field(in, "panic_applied"));
  out->release_request = decimal(field(in, "release_request"));
  out->release_sequence = decimal(field(in, "release_sequence"));
  out->release_proof_cursor = decimal(field(in, "release_proof_cursor"));
  auto pair = field(in, "native_pair");
  keys(pair, {"schema", "audio", "physical"});
  require(packet::string(field(pair, "schema")) == PairedCheckpoint::schema,
          "paired checkpoint schema differs");
  read_audio(field(pair, "audio"), out->native_pair.audio);
  out->native_pair.physical =
      ql::physical_wire::read_checkpoint_wire(field(pair, "physical"));
  read_slots(field(in, "inputs"), max_touches, [&](auto slot, J *entry) {
    keys(entry, {"slot", "input_ref", "target", "release_pending",
                 "press_applied", "press_sequence", "release_sequence"});
    out->bindings.inputs[slot] = {true,
                                  boolean(field(entry, "release_pending")),
                                  boolean(field(entry, "press_applied")),
                                  packet::ref(entry, "input_ref"),
                                  packet::note(field(entry, "target")),
                                  decimal(field(entry, "press_sequence")),
                                  decimal(field(entry, "release_sequence"))};
  });
  auto h = field(in, "input_history");
  keys(h, {"read", "write", "last_ordinal", "last_touch_token",
           "last_member_token", "entries"});
  auto &state = out->bindings;
  state.read = decimal(field(h, "read"));
  state.write = decimal(field(h, "write"));
  state.last_ordinal = decimal(field(h, "last_ordinal"));
  state.last_touch_token = decimal(field(h, "last_touch_token"));
  state.last_member_token = decimal(field(h, "last_member_token"));
  require(state.write >= state.read && state.write - state.read <= 256,
          "input journal bounds differ");
  auto entries = field(h, "entries");
  packet::array(entries, std::size_t(state.write - state.read));
  for (auto i = state.read; i < state.write; ++i) {
    auto entry = json_object_array_get_idx(entries, i - state.read);
    keys(entry, {"ordinal", "native_sequence", "change", "operation",
                 "input_ref", "target"});
    state.history[i % 256] = {decimal(field(entry, "ordinal")),
                              decimal(field(entry, "native_sequence")),
                              InputBindingChange(byte(field(entry, "change"))),
                              Kind(byte(field(entry, "operation"))),
                              packet::ref(entry, "input_ref"),
                              packet::note(field(entry, "target"))};
  }
  require(valid_management_checkpoint(*out) &&
              out->native_pair.audio.cursor ==
                  out->native_pair.physical.samples_elapsed,
          "management paired lifetime disconnected");
  return out;
}
} // namespace management_checkpoint_transport
} // namespace ql::performance
#endif
