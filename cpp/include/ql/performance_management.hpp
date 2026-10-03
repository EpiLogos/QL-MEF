#ifndef QL_PERFORMANCE_MANAGEMENT_HPP
#define QL_PERFORMANCE_MANAGEMENT_HPP
#include <map>
#include <ql/audio_device_macos.hpp>
#include <ql/performance_management_checkpoint.hpp>
#include <ql/performance_offline.hpp>
#include <set>
#include <string>
#include <vector>

namespace ql::performance {
// This object belongs to the existing field worker's serial control owner.
// It has one resident P/Engine/device; copied observations never integrate P.
struct ManagementAdmission {
  Result result = Result::Unavailable;
  NativeClockAdmission clock{};
  std::string reason;
};
// Stamped only by the same native management owner after real Engine
// enqueue. Transport epoch is not the independent AudioUnit clock epoch.
class NativeScoreAdmission {
  friend class PerformanceManagement;
  Result result_ = Result::Unavailable;
  NativeQueueAdmission queue_{};
  Ref session_{}, input_{};
  std::uint64_t epoch_ = 0;
  static NativeScoreAdmission refused(Result result) {
    NativeScoreAdmission out{};
    out.result_ = result;
    return out;
  }

public:
  Result result() const noexcept { return result_; }
  const NativeQueueAdmission &queue() const noexcept { return queue_; }
  const Ref &session_ref() const noexcept { return session_; }
  const Ref &input_ref() const noexcept { return input_; }
  std::uint64_t transport_epoch() const noexcept { return epoch_; }
};
struct KeyboardCell {
  std::uint8_t row = 0, column = 0;
  NoteTarget native_target{};
  std::string label;
  bool available = true, address_admitted = false;
  std::uint8_t address_key = 0, address_pitch_class = 0;
  std::int8_t address_register = 0;
  bool has_source_degree = false;
  std::uint16_t source_degree = 0;
  Ref reduction_policy{}, source_collection{}, source_receipt{};
  std::string unavailable_reason;
};
struct ManagementPulse {
  bool has_readback = false;
  Readback reading{};
  std::vector<NativeGestureApplication> applications;
  RecordingStatus recording{};
  DeviceReceipt device{};
  std::vector<InputBindingRecord> input_history;
  bool release_pending = false, release_zero_proven = false;
  std::uint64_t release_request = 0, release_sequence = 0,
                release_proof_cursor = 0;
};
struct TransportAcknowledgement {
  std::uint64_t previous_epoch = 0, epoch = 0, previous_cursor = 0,
                previous_sequence = 0, target_sample = 0, accepted_sequence = 0;
  Ref transaction{}, checkpoint{};
};

class PerformanceManagement {
  using Input = NativeInputBinding;
  NativePerformance native_;
  MacAudioDevice device_;
  Ref session_{};
  std::uint64_t transport_epoch_ = 1;
  NativeInputBindings bindings_{};
  std::array<Input, max_touches> &inputs_ = bindings_.inputs();
  bool release_pending_ = false, panic_applied_ = false;
  std::uint64_t release_request_ = 0, release_sequence_ = 0,
                release_proof_cursor_ = 0;
  std::vector<KeyboardCell> catalog_;
  Identity catalog_identity_{};
  std::uint64_t catalog_revision_ = 0;
  Readback latest_{};
  bool has_latest_ = false;
  Parameters baseline_{};
  // Undo/learn belongs to the existing C document owner, never a private store.
  struct ParameterHistoryPort {
    void *owner = nullptr;
    bool (*undo_value)(void *, Parameter, double &) = nullptr;
    bool (*learn)(void *, Parameter) = nullptr;
  } history_{};
  bool control_recording_failed_ = false;

  static double parameter_value(const Parameters &p, Parameter id) {
    switch (id) {
    case Parameter::ForceNewtons:
      return p.force_newtons;
    case Parameter::AttackSeconds:
      return p.attack_seconds;
    case Parameter::ReleaseSeconds:
      return p.release_seconds;
    case Parameter::CutoffHertz:
      return p.cutoff_hertz;
    case Parameter::MasterLinear:
      return p.master_linear;
    case Parameter::BodyLinear:
      return p.body_linear;
    case Parameter::MonitorLinear:
      return p.monitor_linear;
    }
    throw std::invalid_argument("unknown native performance parameter");
  }
  std::uint64_t next_sequence() const {
    const auto previous = native_.engine->accepted_sequence();
    if (previous == std::numeric_limits<std::uint64_t>::max())
      throw std::overflow_error("native queue ordinal exhausted");
    return previous + 1;
  }
  ManagementAdmission live(Operation op) {
    op.sequence = next_sequence();
    auto admission = device_.enqueue_bridge_gesture(op);
    return {admission.result, admission,
            admission.result == Result::Accepted
                ? std::string{}
                : "native output clock/device/source admission refused"};
  }
  Input *input(const Ref &ref) noexcept {
    for (auto &i : inputs_)
      if (i.active && i.input_ref == ref)
        return &i;
    return nullptr;
  }
  bool
  capture_stopped_reading_from(const Engine::Checkpoint &saved,
                               const Engine::StoppedCustody &guard) noexcept {
    if (!guard)
      return false;
    const auto *cp = &saved;
    Readback next{};
    next.identity = cp->determination.identity;
    next.determination = cp->determination;
    next.samples_elapsed = cp->cursor;
    next.body_revision = cp->determination.body_revision;
    next.last_sequence = cp->applied_sequence;
    next.last_applied_application_ordinal = cp->applied_application_ordinal;
    next.source = cp->source;
    next.effective = cp->effective;
    next.sustain = cp->sustain;
    next.scalar_force_budget_newtons =
        cp->has_route_programs
            ? native_.engine->force_parameter_maximum()
            : native_.body->preparation().input().max_force_newtons;
    next.has_route_programs = cp->has_route_programs;
    next.routes_suspended =
        cp->has_route_programs && cp->route_programs.owner_suspended;
    next.clipping_samples = cp->clipping;
    next.recording = cp->recording;
    next.force_zero_samples = cp->force_zero_samples;
    next.emergency_requested = cp->emergency_requested;
    next.emergency_observed = cp->emergency_observed;
    next.emergency_applied_sample = cp->emergency_applied_sample;
    for (std::size_t i = 0; i < cp->touches.size(); ++i)
      next.held_touch_tokens[i] = cp->touches[i].token;
    for (const auto &tail : cp->tails)
      next.active_tails += tail.left != 0;
    next.available = !cp->fault;
    for (const auto &t : cp->touches)
      next.active_touches += bool(t.token);
    for (const auto &v : cp->voices)
      if (v.active) {
        auto &seen = next.voices[next.active_voices++];
        seen.member = v.note.member;
        seen.m1_revision = v.note.identity.m1_revision;
        seen.m2_generation = v.note.identity.m2_generation;
        seen.effective_hertz = v.frequency;
        seen.target_hertz = v.target_frequency;
        seen.envelope = v.envelope;
        seen.releasing = v.release;
      }
    if (!ql::write_physical_snapshot(*native_.body, next.physical,
                                     next.body_revision, next.samples_elapsed))
      return false;
    next.has_receiving = cp->has_receiving;
    if (next.has_receiving && !native_.engine->write_stopped_receiving_readback(
                                  next.receiving, guard, next.samples_elapsed))
      return false;
    latest_ = next;
    has_latest_ = true;
    return true;
  }

  void capture_stopped_reading(const Engine::StoppedCustody &guard) {
    auto cp = std::make_unique<Engine::Checkpoint>();
    native_.engine->write_checkpoint(*cp, guard);
    if (!capture_stopped_reading_from(*cp, guard))
      throw std::logic_error("resident stopped audio/body snapshot refused");
  }

public:
  // Preparation is the actual Rust/P native packet consumer. Generic metric
  // construction carries reference/provider standing. SourceForm standing is
  // established by the Rust host's canonical regeneration API before this call.
  PerformanceManagement(NativePerformance native, Ref session,
                        Parameters baseline = {})
      : native_(std::move(native)), session_(session), baseline_(baseline) {
    if (!valid_ref(session_) || !native_.body || !native_.engine ||
        !native_.engine->owns_physical_owner(native_.body.get()) ||
        !native_.engine->has_physical_custody())
      throw std::invalid_argument(
          "one qualified resident native owner required");
    auto guard = native_.engine->acquire_stopped_custody();
    capture_stopped_reading(guard);
  }
  PerformanceManagement(const PerformanceManagement &) = delete;
  PerformanceManagement &operator=(const PerformanceManagement &) = delete;
  const Ref &session_ref() const noexcept { return session_; }
  std::uint64_t transport_epoch() const noexcept { return transport_epoch_; }
  const NativePerformance &native() const noexcept { return native_; }
  const Readback &last_readback() const {
    if (!has_latest_)
      throw std::logic_error("no qualified native snapshot");
    return latest_;
  }
  std::uint64_t last_native_touch() const noexcept {
    return bindings_.last_touch_token();
  }
  std::uint64_t last_native_member() const noexcept {
    return bindings_.last_member_token();
  }
  static double read_parameter(const Parameters &p, Parameter id) {
    return parameter_value(p, id);
  }
  double baseline_parameter(Parameter id) const {
    return parameter_value(baseline_, id);
  }
  const std::vector<KeyboardCell> &catalog() const noexcept { return catalog_; }

  // Every cell is resolved by native K's catalog helper. No chromatic/fifths
  // table is reproduced here. Three physical copies must agree exactly.
private:
  void validate_catalog(std::vector<KeyboardCell> &cells,
                        const Determination &source) const {
    if (cells.empty() || cells.size() > 192)
      throw std::invalid_argument("bounded native keyboard catalog required");
    std::array<bool, 192> occupied{};
    std::set<std::uint8_t> rows;
    std::map<std::pair<unsigned, int>,
             std::pair<const KeyboardCell *, unsigned>>
        addresses;
    for (auto &cell : cells) {
      if (cell.row >= 6 || cell.column >= 32 ||
          occupied[cell.row * 32 + cell.column] || cell.label.empty() ||
          cell.label.size() > 128)
        throw std::invalid_argument("native physical address is malformed");
      if (cell.available) {
        const auto &n = cell.native_target;
        if (!(n.identity == source.identity) ||
            !native_.engine->validate_candidate_note_target(n, source) ||
            !cell.unavailable_reason.empty())
          throw std::invalid_argument(
              "available native source key is disconnected");
        if (cell.address_admitted &&
            (cell.address_key != n.key ||
             cell.address_pitch_class != n.pitch_class ||
             cell.address_register != n.register_octave))
          throw std::invalid_argument(
              "physical source address differs from native target");
        cell.address_key = n.key;
        cell.address_pitch_class = n.pitch_class;
        cell.address_register = n.register_octave;
      } else {
        // An unavailable key retains only its physical native address and
        // source reason. It has no numerical or fabricated NoteTarget payload.
        const auto &n = cell.native_target;
        if (!cell.address_admitted || cell.address_key >= 12 ||
            cell.address_pitch_class >= 12 || cell.address_register < -16 ||
            cell.address_register > 16 || cell.unavailable_reason.empty() ||
            cell.unavailable_reason.size() > 2048 || n.member || n.touch ||
            n.hertz || n.fundamental_hz || n.source_coordinate[0] ||
            n.identity.event[0] || !valid_ref(cell.reduction_policy) ||
            !valid_ref(cell.source_collection) ||
            !valid_ref(cell.source_receipt))
          throw std::invalid_argument(
              "unavailable native key contains a playable target or lacks "
              "source standing");
      }
      occupied[cell.row * 32 + cell.column] = true;
      rows.insert(cell.row);
      auto key = std::make_pair(unsigned(cell.address_key),
                                int(cell.address_register));
      auto old = addresses.find(key);
      if (old == addresses.end())
        addresses.emplace(key, std::make_pair(&cell, 1));
      else {
        const auto &a = *old->second.first;
        if (a.available != cell.available ||
            a.address_pitch_class != cell.address_pitch_class ||
            a.has_source_degree != cell.has_source_degree ||
            a.source_degree != cell.source_degree ||
            a.reduction_policy != cell.reduction_policy ||
            a.source_collection != cell.source_collection ||
            a.source_receipt != cell.source_receipt ||
            a.unavailable_reason != cell.unavailable_reason)
          throw std::invalid_argument(
              "physical source key copies differ in availability/provenance");
        if (cell.available) {
          const auto &left = a.native_target, &right = cell.native_target;
          if (left.hertz != right.hertz ||
              left.source_coordinate != right.source_coordinate ||
              left.source_face != right.source_face ||
              left.tuning_ref != right.tuning_ref ||
              left.ratio_numerator != right.ratio_numerator ||
              left.ratio_denominator != right.ratio_denominator ||
              left.exact_ratio != right.exact_ratio)
            throw std::invalid_argument(
                "repeated physical keys differ in native source tuning");
        }
        ++old->second.second;
      }
    }
    if (rows.size() != 6 || addresses.size() < 12 ||
        std::any_of(addresses.begin(), addresses.end(),
                    [](const auto &p) { return p.second.second != 3; }))
      throw std::invalid_argument("six-row Janko physical copies incomplete");
  }

public:
  void admit_catalog(std::vector<KeyboardCell> cells) {
    const auto source = native_.engine->source_for_native_admission();
    validate_catalog(cells, source);
    if (catalog_revision_ == std::numeric_limits<std::uint64_t>::max())
      throw std::overflow_error("native catalog revision exhausted");
    catalog_ = std::move(cells);
    catalog_identity_ = source.identity;
    ++catalog_revision_;
  }
  // Heap-held control proposal, never a second history/store or callback clock.
  class PreparedCombinedRevision {
    friend class PerformanceManagement;
    PerformanceManagement *owner_ = nullptr;
    std::unique_ptr<Engine::PreparedCombinedRevision> engine_;
    std::vector<KeyboardCell> catalog_;
    std::vector<NoteTarget> notes_;
    Identity before_catalog_{};
    std::uint64_t before_catalog_revision_ = 0;
    std::uint64_t transport_epoch_ = 0;
    bool ready_ = false;

  public:
    PreparedCombinedRevision() = default;
    PreparedCombinedRevision(const PreparedCombinedRevision &) = delete;
    PreparedCombinedRevision &
    operator=(const PreparedCombinedRevision &) = delete;
    bool ready() const noexcept { return ready_; }
    const Engine::PreparedCombinedRevision &engine_candidate() const {
      return *engine_;
    }
  };
  // The existing native owner supplies the independently prepared source/body,
  // catalogue and full receiving source. All control allocation/validation is
  // completed BEFORE P or Engine mutates; unknown JSON cannot mint this token.
  bool preflight_stopped_combined_revision(
      const Determination &after, const PhysicalPort &port,
      const NativeRouteProgramSet &after_seed, std::vector<NoteTarget> notes,
      std::vector<KeyboardCell> cells, const Engine::StoppedCustody &guard,
      std::uint64_t expected_cursor, PreparedCombinedRevision &out) {
    if (out.owner_ || out.ready_ || notes.empty() || notes.size() > 192 ||
        release_pending_ || control_recording_failed_ ||
        catalog_revision_ == std::numeric_limits<std::uint64_t>::max())
      return false;
    for (const auto &note : notes)
      if (!native_.engine->validate_candidate_note_target(note, after))
        return false;
    validate_catalog(cells, after);
    auto candidate = std::make_unique<Engine::PreparedCombinedRevision>();
    if (!native_.engine->preflight_stopped_combined_revision(
            after, port, after_seed, guard, expected_cursor, *candidate))
      return false;
    out.engine_ = std::move(candidate);
    out.catalog_ = std::move(cells);
    out.notes_ = std::move(notes);
    out.before_catalog_ = catalog_identity_;
    out.before_catalog_revision_ = catalog_revision_;
    out.transport_epoch_ = transport_epoch_;
    out.owner_ = this;
    out.ready_ = true;
    return true;
  }
  bool combined_revision_current(
      const PreparedCombinedRevision &candidate,
      const Engine::StoppedCustody &guard) const noexcept {
    return candidate.ready_ && candidate.owner_ == this && candidate.engine_ &&
           transport_epoch_ == candidate.transport_epoch_ &&
           catalog_identity_ == candidate.before_catalog_ &&
           catalog_revision_ == candidate.before_catalog_revision_ &&
           catalog_revision_ != std::numeric_limits<std::uint64_t>::max() &&
           !release_pending_ && !control_recording_failed_ &&
           native_.engine->combined_revision_current(*candidate.engine_, guard);
  }
  // Called only after all P/body/receiver/private source preflights and P's
  // actual projection have succeeded under this same serial stopped custody.
  // Retired vectors and route allocations remain in candidate until the owner
  // records the genuine transaction acknowledgement. Input bindings/history,
  // touch/member high waters, device owner and transport epoch remain intact.
  void commit_stopped_combined_revision(
      PreparedCombinedRevision &candidate,
      const Engine::StoppedCustody &guard) noexcept {
    if (!combined_revision_current(candidate, guard))
      std::terminate();
    native_.engine->commit_stopped_combined_revision(*candidate.engine_, guard);
    native_.determination = candidate.engine_->after_determination();
    std::swap(native_.notes, candidate.notes_);
    std::swap(catalog_, candidate.catalog_);
    catalog_identity_ = native_.determination.identity;
    ++catalog_revision_; // nonoverflow was preflighted before any P mutation
    has_latest_ = false; // copy actual AFTER snapshot under this same guard
    candidate.ready_ = false;
  }
  void refresh_stopped_reading(const Engine::StoppedCustody &guard) {
    capture_stopped_reading(guard);
  }
  // The Rust host resolves this exact independent KeyTouch against its current
  // immutable binding, and passes both selected cell and returned NoteTarget.
  ManagementAdmission press(Ref input_ref, std::uint8_t row,
                            std::uint8_t column, const NoteTarget &resolved,
                            double velocity) {
    if (release_pending_ || control_recording_failed_)
      return {Result::Unavailable,
              {},
              "native owner held/pending physical release"};
    if (!bindings_.can_record())
      return {Result::Exhausted, {}, "native input journal must be drained"};
    if (!valid_ref(input_ref) || input(input_ref) || !std::isfinite(velocity) ||
        velocity < 0 || velocity > 1)
      return {Result::Invalid, {}, "invalid/repeated native input"};
    if (!(catalog_identity_ ==
          native_.engine->source_for_native_admission().identity))
      return {Result::Stale,
              {},
              "native keyboard requires current source admission"};
    const auto cell =
        std::find_if(catalog_.begin(), catalog_.end(), [&](const auto &c) {
          return c.row == row && c.column == column;
        });
    if (cell != catalog_.end() && !cell->available)
      return {Result::Unavailable, {}, cell->unavailable_reason};
    if (cell == catalog_.end() || cell->native_target.key != resolved.key ||
        cell->native_target.register_octave != resolved.register_octave ||
        cell->native_target.hertz != resolved.hertz ||
        cell->native_target.source_coordinate != resolved.source_coordinate ||
        cell->native_target.source_face != resolved.source_face ||
        !(resolved.identity == catalog_identity_))
      return {Result::Stale,
              {},
              "resolved K touch differs from selected native cell"};
    auto free = std::find_if(inputs_.begin(), inputs_.end(),
                             [](const auto &i) { return !i.active; });
    if (free == inputs_.end())
      return {Result::Exhausted, {}, "native touch binding limit"};
    for (const auto &i : inputs_)
      if (i.active && i.target.touch == resolved.touch)
        return {Result::Invalid, {}, "native physical touch token reused"};
    Operation op{};
    op.kind = Kind::NoteOn;
    op.identity = resolved.identity;
    op.note = resolved;
    op.value = velocity;
    auto admitted = live(op);
    if (admitted.result == Result::Accepted)
      if (!bindings_.bind(input_ref, resolved, admitted.clock.sequence)) {
        hold();
        return {Result::Unavailable, admitted.clock,
                "exact native input journal exhausted; held for release"};
      }
    return admitted;
  }
  // Exact authored-score samples are a separate native C admission. This
  // method never dates live gestures or fabricates an AudioUnit epoch. The
  // existing native score/source owner supplies qualified NoteTargets; no UI
  // endpoint accepts an Operation/NoteTarget packet from this seam.
  NativeScoreAdmission enqueue_score_input_admission(Operation op,
                                                     Ref original_input = {}) {
    if (control_recording_failed_ || release_pending_)
      return NativeScoreAdmission::refused(Result::Unavailable);
    if (op.sequence != next_sequence())
      return NativeScoreAdmission::refused(Result::Order);
    Input *held = valid_ref(original_input) ? input(original_input) : nullptr;
    const bool touch_operation = op.kind == Kind::NoteOn ||
                                 op.kind == Kind::NoteOff ||
                                 op.kind == Kind::Expression;
    if (touch_operation) {
      if (!valid_ref(original_input) || !bindings_.can_record())
        return NativeScoreAdmission::refused(Result::Invalid);
      if (op.kind == Kind::NoteOn) {
        if (held || !op.note.touch || !op.note.member ||
            std::none_of(inputs_.begin(), inputs_.end(),
                         [](const auto &i) { return !i.active; }))
          return NativeScoreAdmission::refused(Result::Exhausted);
        for (const auto &i : inputs_)
          if (i.active && i.target.touch == op.note.touch)
            return NativeScoreAdmission::refused(Result::Invalid);
        auto checkpoint = std::make_unique<NativeInputBindings::State>();
        bindings_.write_checkpoint(*checkpoint);
        if (op.note.touch <= checkpoint->last_touch_token)
          return NativeScoreAdmission::refused(Result::Invalid);
      } else if (!held || held->release_pending ||
                 held->target.touch != op.touch ||
                 (op.kind == Kind::NoteOff &&
                  !(op.identity == held->target.identity)))
        return NativeScoreAdmission::refused(Result::Stale);
    } else if (original_input != Ref{})
      return NativeScoreAdmission::refused(Result::Invalid);
    NativeScoreAdmission admitted{};
    admitted.queue_ = native_.engine->enqueue_with_receipt(op);
    admitted.result_ = admitted.queue_.result();
    if (!admitted.queue_.queued())
      return admitted;
    admitted.session_ = session_;
    admitted.input_ = original_input;
    admitted.epoch_ = transport_epoch_;
    bool recorded = true;
    if (op.kind == Kind::NoteOn)
      recorded = bindings_.bind(original_input, op.note, op.sequence);
    else if (op.kind == Kind::NoteOff)
      recorded = bindings_.release_admitted(original_input, op.sequence);
    if (!recorded) {
      hold();
      // The queued native fact remains visible even when its input journal
      // failed. The caller must retain it as failed recording, never retry.
      admitted.result_ = Result::Unavailable;
    }
    return admitted;
  }
  Result enqueue_score_input(Operation op, Ref original_input = {}) {
    return enqueue_score_input_admission(op, original_input).result();
  }

  ManagementAdmission release(Ref input_ref) {
    auto *held = input(input_ref);
    if (!held || held->release_pending)
      return {
          Result::Invalid, {}, "native input release already absent/pending"};
    Operation op{};
    op.kind = Kind::NoteOff;
    op.identity = held->target.identity;
    op.touch = held->target.touch;
    auto admitted = live(op);
    if (admitted.result == Result::Accepted) {
      if (!bindings_.release_admitted(input_ref, admitted.clock.sequence)) {
        hold();
        return {Result::Unavailable, admitted.clock,
                "release admitted but input journal unavailable; owner held"};
      }
    } else
      hold();
    return admitted;
  }
  ManagementAdmission expression(Ref input_ref, double pressure,
                                 const NoteTarget *native_pitch = nullptr) {
    auto *held = input(input_ref);
    if (!held || held->release_pending)
      return {Result::Invalid, {}, "native input is not held"};
    Operation op{};
    op.kind = Kind::Expression;
    op.identity = native_.engine->source_for_native_admission().identity;
    op.touch = held->target.touch;
    op.value = pressure;
    if (native_pitch) {
      op.note = *native_pitch;
      op.pitch_hz = native_pitch->hertz;
    }
    return live(op);
  }
  ManagementAdmission sustain(bool down) {
    Operation op{};
    op.kind = Kind::Sustain;
    op.identity = native_.engine->source_for_native_admission().identity;
    op.value = down ? 1 : 0;
    auto admitted = live(op);
    if (!down && admitted.result != Result::Accepted)
      hold();
    return admitted;
  }
  ManagementAdmission panic() {
    Operation op{};
    op.kind = Kind::Panic;
    op.identity = native_.engine->source_for_native_admission().identity;
    auto admitted = live(op);
    release_pending_ = true;
    panic_applied_ = false;
    release_proof_cursor_ = 0;
    // Public all-output Panic requires zero actual Newton force on every
    // native route. The queued performed Panic remains a real application for
    // C, while this independent emergency fence suspends N programmes and
    // releases authentic M1 touches/tails under the same callback ownership.
    release_request_ = native_.engine->request_panic();
    release_sequence_ = 0;
    if (!release_request_) {
      control_recording_failed_ = true;
      return {Result::Exhausted, admitted.clock,
              "native all-route release token exhausted; owner held"};
    }
    return admitted;
  }
  ManagementAdmission set_parameter(Parameter target, double value) {
    if (unsigned(target) > unsigned(Parameter::MonitorLinear))
      return {Result::Invalid, {}, "unknown native parameter"};
    Operation op{};
    op.kind = Kind::Parameter;
    op.identity = native_.engine->source_for_native_admission().identity;
    op.parameter = target;
    op.value = value;
    auto admitted = live(op);
    return admitted;
  }
  // Installed C history supplies the value at its native document CAS. It
  // consumes the actual resulting application receipt for retained automation.
  void attach_parameter_history(void *owner,
                                bool (*undo_value)(void *, Parameter, double &),
                                bool (*learn)(void *, Parameter)) {
    history_ = {owner, undo_value, learn};
  }
  ManagementAdmission undo_parameter(Parameter target) {
    double value = 0;
    if (unsigned(target) > unsigned(Parameter::MonitorLinear) ||
        !history_.owner || !history_.undo_value ||
        !history_.undo_value(history_.owner, target, value))
      return {Result::Unavailable,
              {},
              "existing native C parameter undo unavailable"};
    return set_parameter(target, value);
  }
  ManagementAdmission clear_parameter(Parameter target) {
    if (unsigned(target) > unsigned(Parameter::MonitorLinear))
      return {Result::Invalid, {}, "unknown native parameter"};
    return set_parameter(target, parameter_value(baseline_, target));
  }
  ManagementAdmission learn_parameter(Parameter target) {
    if (unsigned(target) > unsigned(Parameter::MonitorLinear) ||
        !history_.owner || !history_.learn ||
        !history_.learn(history_.owner, target))
      return {Result::Unavailable,
              {},
              "existing native C modulation-route learn unavailable"};
    return {Result::Accepted, {}, std::string{}};
  }
  static std::vector<DeviceDescription> enumerate_devices() {
    return MacAudioDevice::enumerate();
  }
  bool open_device(const DeviceConfig &config) {
    return device_.open(native_.engine, config);
  }
  bool start_device() { return device_.start(); }
  bool stop_device() { return device_.stop(); }
  bool recover_device() { return device_.recover(native_.engine); }
  bool close_device() { return device_.close(); }
  DeviceReceipt device_receipt() const { return device_.receipt(); }
  bool pop_audio_capture(Capture &out) noexcept {
    return native_.engine->pop_capture(out);
  }
  bool pop_device_capture(DeviceCapture &out) noexcept {
    return device_.pop_capture(out);
  }
  std::uint64_t hold() noexcept {
    release_request_ = native_.engine->request_panic();
    release_sequence_ = 0;
    release_proof_cursor_ = 0;
    release_pending_ = true;
    panic_applied_ = false;
    control_recording_failed_ = true;
    // This return is only a requested native release token. The existing host
    // MUST keep its acknowledgement pending until pulse() supplies zero proof.
    return release_request_;
  }

  // Called on EVERY existing native host pulse. Queue drains are finite even
  // if the audio producer refills concurrently. C consumes the exact returned
  // application records; receipt loss is an explicit recording failure.
  std::unique_ptr<ManagementPulse> pulse() {
    auto out = std::make_unique<ManagementPulse>();
    out->applications.reserve(256);
    Readback reading{};
    for (unsigned i = 0; i < 64 && native_.engine->pop_readback(reading); ++i) {
      latest_ = reading;
      has_latest_ = true;
    }
    NativeGestureApplication applied{};
    for (unsigned i = 0; i < 256 && has_latest_ &&
                         native_.engine->pop_gesture_application_up_to(
                             applied, latest_.last_applied_application_ordinal);
         ++i) {
      out->applications.push_back(applied);
      if (!bindings_.application(applied))
        hold();
      if (applied.kind == Kind::Panic && applied.applied &&
          applied.sequence == release_sequence_)
        panic_applied_ = true;
    }
    if (has_latest_ && !bindings_.retire_absent(latest_))
      hold();
    out->input_history.reserve(256);
    InputBindingRecord binding{};
    for (unsigned i = 0; i < 256 && bindings_.pop_history(binding); ++i)
      out->input_history.push_back(binding);
    if (release_pending_ && has_latest_ &&
        (release_request_ || panic_applied_) &&
        ql::performance::release_zero_proven(latest_, release_request_,
                                             release_sequence_)) {
      release_pending_ = false;
      release_proof_cursor_ = latest_.samples_elapsed;
    }
    out->release_pending = release_pending_;
    out->release_zero_proven = release_proof_cursor_ != 0 && !release_pending_;
    out->release_request = release_request_;
    out->release_sequence = release_sequence_;
    out->release_proof_cursor = release_proof_cursor_;
    out->has_readback = has_latest_;
    if (has_latest_)
      out->reading = latest_;
    out->recording = native_.engine->recording_status();
    out->device = device_.receipt();
    if (out->recording.failure != RecordingFailure::None)
      control_recording_failed_ = true;
    return out;
  }
  bool recording_available() const noexcept {
    return !control_recording_failed_ &&
           native_.engine->recording_status().failure == RecordingFailure::None;
  }
  // Offline rendering is an explicit existing-native-owner operation. Device
  // callbacks exclude it; inspection never advances samples implicitly.
  bool offline_advance(float *out, std::size_t frames,
                       std::uint64_t expected_cursor) {
    if (native_.engine->device_callbacks_running())
      return false;
    return native_.engine->render(out, frames, expected_cursor);
  }
  std::unique_ptr<NativeOfflineRenderChunk>
  offline_render(const NativeOfflineRenderScope &scope, float *output,
                 std::size_t frames) {
    if (scope.session != session_) {
      auto out = std::make_unique<NativeOfflineRenderChunk>();
      out->reason = "offline scope differs from resident retained session";
      return out;
    }
    auto chunk = render_native_offline_chunk(*native_.engine, native_.body,
                                             scope, output, frames);
    // The offline consumer already removed the actual callback's readback
    // from the Engine ring to qualify captured PCM. Publish THAT same copied
    // body/cursor/application high-water to management; do not wait for a
    // nonexistent second readback or reconstruct/integrate the physical state.
    if (chunk->result == Result::Accepted && chunk->state_committed &&
        chunk->capture_complete) {
      latest_ = chunk->reading;
      has_latest_ = true;
    }
    return chunk;
  }
  std::unique_ptr<ManagementCheckpoint> stopped_checkpoint() {
    auto guard = native_.engine->acquire_stopped_custody();
    if (!guard)
      throw std::logic_error(
          "actual stopped callback checkpoint custody required");
    auto saved = std::make_unique<ManagementCheckpoint>();
    native_.engine->write_checkpoint(saved->native_pair.audio, guard);
    saved->native_pair.physical = native_.body->checkpoint();
    bindings_.write_checkpoint(saved->bindings);
    saved->session = session_;
    saved->transport_epoch = transport_epoch_;
    saved->recording_failed = control_recording_failed_;
    saved->release_pending = release_pending_;
    saved->panic_applied = panic_applied_;
    saved->release_request = release_request_;
    saved->release_sequence = release_sequence_;
    saved->release_proof_cursor = release_proof_cursor_;
    if (!valid_management_checkpoint(*saved))
      throw std::logic_error("native input lifetime checkpoint disconnected");
    return saved;
  }
  bool stopped_restore(const ManagementCheckpoint &saved,
                       std::uint64_t expected_cursor, Ref transaction,
                       Ref checkpoint_ref, TransportAcknowledgement &ack) {
    if (!valid_ref(transaction) || !valid_ref(checkpoint_ref) ||
        saved.session != session_ || !saved.transport_epoch ||
        !valid_management_checkpoint(saved) ||
        transport_epoch_ == std::numeric_limits<std::uint64_t>::max() ||
        catalog_revision_ == std::numeric_limits<std::uint64_t>::max())
      return false;
    auto guard = native_.engine->acquire_stopped_custody();
    if (!guard)
      return false;
    const auto previous_sequence = native_.engine->accepted_sequence();
    if (!restore_checkpoint(*native_.engine, *native_.body, saved.native_pair,
                            guard, expected_cursor))
      return false;
    bindings_.restore_validated(saved.bindings);
    control_recording_failed_ = saved.recording_failed;
    release_pending_ = saved.release_pending;
    panic_applied_ = saved.panic_applied;
    release_request_ = saved.release_request;
    release_sequence_ = saved.release_sequence;
    release_proof_cursor_ = saved.release_proof_cursor;
    ack = {transport_epoch_,
           transport_epoch_ + 1,
           expected_cursor,
           previous_sequence,
           saved.native_pair.audio.cursor,
           saved.native_pair.audio.accepted_sequence,
           transaction,
           checkpoint_ref};
    ++transport_epoch_;
    catalog_.clear();
    ++catalog_revision_;
    // Exact original input_ref and resolved native target are restored from
    // the paired wrapper. A source_touch_ref is never substituted for UI input.
    capture_stopped_reading(guard);
    return true;
  }
  // Private numerical candidate owned on the existing host heap. Original
  // saved wire/checkpoint remains lossless; only derivative route handles in
  // Engine's operative copy are refreshed by the native current source owner.
  class PreparedReceivingRestore {
    friend class PerformanceManagement;
    PerformanceManagement *owner_ = nullptr;
    std::unique_ptr<ManagementCheckpoint> saved_;
    std::unique_ptr<Engine::PreparedReceivingRestore> engine_;
    std::vector<KeyboardCell> catalog_;
    std::vector<NoteTarget> notes_;
    Identity before_catalog_{};
    std::uint64_t catalog_revision_ = 0, epoch_ = 0, input_read_ = 0,
                  input_write_ = 0, input_ordinal_ = 0, touch_token_ = 0,
                  member_token_ = 0, release_request_ = 0,
                  release_sequence_ = 0, release_proof_cursor_ = 0;
    bool release_pending_ = false, panic_applied_ = false,
         recording_failed_ = false, ready_ = false;

  public:
    PreparedReceivingRestore() = default;
    PreparedReceivingRestore(const PreparedReceivingRestore &) = delete;
    PreparedReceivingRestore &
    operator=(const PreparedReceivingRestore &) = delete;
    PreparedReceivingRestore(PreparedReceivingRestore &&) = delete;
    PreparedReceivingRestore &operator=(PreparedReceivingRestore &&) = delete;
    bool ready() const noexcept { return ready_; }
    const ManagementCheckpoint &original_checkpoint() const { return *saved_; }
    const Engine::PreparedReceivingRestore &engine_candidate() const {
      return *engine_;
    }
    const ql::PhysicalBodyCheckpoint &physical_checkpoint() const {
      return saved_->native_pair.physical;
    }
  };
  // Existing private source/Act/lease/occasion owner replays the entire saved
  // source bundle and prepares this SAME body's receiving port at saved cursor.
  // This control-only preflight cannot mint that authority from imported JSON.
  bool preflight_stopped_receiving_restore(
      const ManagementCheckpoint &saved, const PhysicalPort &fresh_port,
      const NativeRouteProgramSet &fresh_seed, std::vector<NoteTarget> notes,
      std::vector<KeyboardCell> cells, const Engine::StoppedCustody &guard,
      std::uint64_t expected_cursor, PreparedReceivingRestore &out) {
    if (out.owner_ || out.ready_ || saved.session != session_ ||
        !valid_management_checkpoint(saved) || notes.empty() ||
        notes.size() > 192 ||
        saved.native_pair.audio.cursor !=
            saved.native_pair.physical.samples_elapsed ||
        saved.native_pair.audio.determination.body_revision !=
            saved.native_pair.physical.body_revision ||
        saved.native_pair.audio.determination.body_preparation_ref !=
            reference(saved.native_pair.physical.preparation_ref.c_str()) ||
        saved.native_pair.audio.determination.body_state_ref !=
            reference(saved.native_pair.physical.state_ref.c_str()) ||
        transport_epoch_ == std::numeric_limits<std::uint64_t>::max() ||
        catalog_revision_ == std::numeric_limits<std::uint64_t>::max())
      return false;
    const auto &determination = saved.native_pair.audio.determination;
    for (const auto &note : notes)
      if (!native_.engine->validate_candidate_note_target(note, determination))
        return false;
    validate_catalog(cells, determination);
    auto engine_candidate =
        std::make_unique<Engine::PreparedReceivingRestore>();
    if (!native_.engine->preflight_stopped_receiving_restore(
            saved.native_pair.audio, fresh_port, fresh_seed, guard,
            expected_cursor, *engine_candidate))
      return false;
    auto original = std::make_unique<ManagementCheckpoint>(saved);
    out.saved_ = std::move(original);
    out.engine_ = std::move(engine_candidate);
    out.catalog_ = std::move(cells);
    out.notes_ = std::move(notes);
    out.before_catalog_ = catalog_identity_;
    out.catalog_revision_ = catalog_revision_;
    out.epoch_ = transport_epoch_;
    out.input_read_ = bindings_.history_read();
    out.input_write_ = bindings_.history_write();
    out.input_ordinal_ = bindings_.history_ordinal();
    out.touch_token_ = bindings_.last_touch_token();
    out.member_token_ = bindings_.last_member_token();
    out.release_request_ = release_request_;
    out.release_sequence_ = release_sequence_;
    out.release_proof_cursor_ = release_proof_cursor_;
    out.release_pending_ = release_pending_;
    out.panic_applied_ = panic_applied_;
    out.recording_failed_ = control_recording_failed_;
    out.owner_ = this;
    out.ready_ = true;
    return true;
  }
  bool receiving_restore_current(
      const PreparedReceivingRestore &out,
      const Engine::StoppedCustody &guard) const noexcept {
    return out.ready_ && out.owner_ == this && out.engine_ && out.saved_ &&
           transport_epoch_ == out.epoch_ &&
           catalog_identity_ == out.before_catalog_ &&
           catalog_revision_ == out.catalog_revision_ &&
           catalog_revision_ != std::numeric_limits<std::uint64_t>::max() &&
           bindings_.history_read() == out.input_read_ &&
           bindings_.history_write() == out.input_write_ &&
           bindings_.history_ordinal() == out.input_ordinal_ &&
           bindings_.last_touch_token() == out.touch_token_ &&
           bindings_.last_member_token() == out.member_token_ &&
           release_request_ == out.release_request_ &&
           release_sequence_ == out.release_sequence_ &&
           release_proof_cursor_ == out.release_proof_cursor_ &&
           release_pending_ == out.release_pending_ &&
           panic_applied_ == out.panic_applied_ &&
           control_recording_failed_ == out.recording_failed_ &&
           native_.engine->receiving_restore_current(*out.engine_, guard);
  }
  // Called ONLY after Root's P restore candidate and receiving source
  // currentness preflights all succeeded before first mutation. No allocation,
  // journal drain, queued operation redating, seed reset or implicit panic.
  void
  commit_stopped_receiving_restore(PreparedReceivingRestore &out,
                                   const Engine::StoppedCustody &guard,
                                   Ref transaction, Ref checkpoint_ref,
                                   TransportAcknowledgement &ack) noexcept {
    if (!receiving_restore_current(out, guard) || !valid_ref(transaction) ||
        !valid_ref(checkpoint_ref))
      std::terminate();
    const auto previous_cursor = native_.engine->samples_elapsed();
    const auto previous_sequence = native_.engine->accepted_sequence();
    native_.engine->commit_stopped_receiving_restore(*out.engine_, guard);
    const auto &saved = *out.saved_;
    const auto &admitted = out.engine_->admitted_checkpoint();
    bindings_.restore_validated(saved.bindings);
    control_recording_failed_ = saved.recording_failed;
    release_pending_ = saved.release_pending;
    panic_applied_ = saved.panic_applied;
    release_request_ = saved.release_request;
    release_sequence_ = saved.release_sequence;
    release_proof_cursor_ = saved.release_proof_cursor;
    native_.determination = admitted.determination;
    std::swap(native_.notes, out.notes_);
    std::swap(catalog_, out.catalog_);
    catalog_identity_ = admitted.determination.identity;
    ++catalog_revision_;
    ack = {transport_epoch_,  transport_epoch_ + 1, previous_cursor,
           previous_sequence, admitted.cursor,      admitted.accepted_sequence,
           transaction,       checkpoint_ref};
    ++transport_epoch_;
    if (!capture_stopped_reading_from(admitted, guard))
      std::terminate();
    out.ready_ = false;
  }
  ManagementAdmission
  admit_distinct_receiving(PhysicalPort qualified_port,
                           const NativeRouteProgramSet &programs,
                           std::uint64_t native_admitted_cursor) {
    auto guard = native_.engine->acquire_stopped_custody();
    if (!guard)
      return {Result::Unavailable,
              {},
              "native route admission requires acknowledged callback custody"};
    if (!native_.engine->install_routes_port(
            std::move(qualified_port), programs, guard, native_admitted_cursor))
      return {
          Result::Stale,
          {},
          "native N9 source/program/projection/body admission disconnected"};
    capture_stopped_reading(guard);
    return {Result::Accepted, {}, std::string{}};
  }
  ManagementAdmission admit_distinct_receiving() const {
    return {Result::Unavailable,
            {},
            "distinct-native-force-projections-unavailable"};
  }
};
} // namespace ql::performance
#endif
