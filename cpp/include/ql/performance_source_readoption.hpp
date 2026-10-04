#ifndef QL_PERFORMANCE_SOURCE_READOPTION_HPP
#define QL_PERFORMANCE_SOURCE_READOPTION_HPP
// Private stopped numerical handoff for the selected genuine native Act/source
// epoch. The Root closed lease/replay producer is REQUIRED before this seam.
// Neither these numerical operands nor a matching CP mint source authority.
#include <cstring>
#include <limits>
#include <memory>
#include <ql/performance_management.hpp>
#include <string>
#include <utility>

namespace ql::performance {
class NativeStoppedSourceReadoption {
public:
  // Noncopyable original/operative custody. Retired P/ports remain alive until
  // the same native owner retains the original transaction/pulse ACK.
  class Retained {
    friend class NativeStoppedSourceReadoption;
    NativePerformance selected_{};
    PhysicalPort port_{};
    ReceivingPort receiver_{};
    std::vector<KeyboardCell> catalog_;
    std::unique_ptr<ManagementCheckpoint> before_, original_, pre_pulse_,
        operative_;
    PerformanceManagement *owner_ = nullptr;
    bool committed_ = false, pulse_completed_ = false;
    Retained() = default;

  public:
    Retained(const Retained &) = delete;
    Retained &operator=(const Retained &) = delete;
    bool committed() const noexcept { return committed_; }
    bool pulse_completed() const noexcept { return pulse_completed_; }
    const ManagementCheckpoint &pre_pulse_checkpoint() const {
      return *pre_pulse_;
    }
    const ManagementCheckpoint &before_checkpoint() const { return *before_; }
    const ManagementCheckpoint &original_checkpoint() const {
      return *original_;
    }
    const ManagementCheckpoint &operative_checkpoint() const {
      return *operative_;
    }
  };

private:
  static bool exact_before(const ManagementCheckpoint &a,
                           const ManagementCheckpoint &b) {
    // Both are complete actual native typed snapshots serialized by the SAME
    // production serializer. Text comparison retains -0 and every float bit
    // represented by the native round-trip format; no numeric Value coercion.
    auto left = management_checkpoint_transport::checkpoint_wire(a);
    auto right = management_checkpoint_transport::checkpoint_wire(b);
    const std::string l =
        json_object_to_json_string_ext(left.get(), JSON_C_TO_STRING_PLAIN);
    const std::string r =
        json_object_to_json_string_ext(right.get(), JSON_C_TO_STRING_PLAIN);
    return l == r;
  }
  static std::string text(const checkpoint_transport::Json &value) {
    return json_object_to_json_string_ext(value.get(), JSON_C_TO_STRING_PLAIN);
  }
  static bool same_application(const NativeGestureApplication &a,
                               const NativeGestureApplication &b) {
    return text(checkpoint_transport::application(a)) ==
           text(checkpoint_transport::application(b));
  }
  static checkpoint_transport::Json history(const InputBindingRecord &h) {
    using namespace checkpoint_transport;
    auto out = object();
    u64(out.get(), "ordinal", h.ordinal);
    u64(out.get(), "native_sequence", h.native_sequence);
    put(out.get(), "change", json_object_new_uint64(unsigned(h.change)));
    put(out.get(), "operation", json_object_new_uint64(unsigned(h.operation)));
    ref(out.get(), "input_ref", h.input_ref);
    put(out.get(), "target", note(h.target).release());
    return out;
  }
  static std::unique_ptr<ManagementCheckpoint>
  snapshot(PerformanceManagement &manager,
           const Engine::StoppedCustody &guard) {
    auto saved = std::make_unique<ManagementCheckpoint>();
    manager.native_.engine->write_checkpoint(saved->native_pair.audio, guard);
    saved->native_pair.physical = manager.native_.body->checkpoint();
    manager.bindings_.write_checkpoint(saved->bindings);
    saved->session = manager.session_;
    saved->transport_epoch = manager.transport_epoch_;
    saved->recording_failed = manager.control_recording_failed_;
    saved->release_pending = manager.release_pending_;
    saved->panic_applied = manager.panic_applied_;
    saved->release_request = manager.release_request_;
    saved->release_sequence = manager.release_sequence_;
    saved->release_proof_cursor = manager.release_proof_cursor_;
    if (!valid_management_checkpoint(*saved))
      throw std::logic_error(
          "selected-source retained before checkpoint disconnected");
    return saved;
  }

public:
  // SAME actual Manager/Engine/AUHAL callback pointer. selected is a scratch
  // canonical native producer at birth0, never activated/rendered. fresh_port,
  // seed and receiver are privately regenerated at the SAVED cursor. Every
  // validation, allocation and failure occurs before the first resident write.
  static bool
  restore(PerformanceManagement &manager, NativePerformance selected,
          const ManagementCheckpoint &original_before,
          const ManagementCheckpoint &saved, const PhysicalPort &fresh_port,
          const NativeRouteProgramSet *fresh_seed,
          std::vector<KeyboardCell> selected_catalog,
          std::uint64_t expected_epoch, std::uint64_t expected_cursor,
          std::uint64_t expected_sequence, Ref transaction, Ref checkpoint_ref,
          std::unique_ptr<Retained> &retained, TransportAcknowledgement &ack,
          const ReceivingPort *saved_receiver = nullptr) {
    if (retained || !valid_ref(transaction) || !valid_ref(checkpoint_ref) ||
        !selected.body || !selected.engine ||
        selected.engine.get() == manager.native_.engine.get() ||
        selected.body.get() == manager.native_.body.get() ||
        selected.engine->device_callbacks_running() ||
        selected.engine->samples_elapsed() != 0 ||
        selected.body->samples_elapsed() != 0 ||
        !selected.engine->owns_physical_owner(selected.body.get()) ||
        selected.notes.empty() || selected.notes.size() > 192 ||
        saved.session != manager.session_ ||
        original_before.session != manager.session_ ||
        manager.transport_epoch_ != expected_epoch ||
        original_before.transport_epoch != expected_epoch ||
        expected_epoch == std::numeric_limits<std::uint64_t>::max() ||
        manager.catalog_revision_ ==
            std::numeric_limits<std::uint64_t>::max() ||
        !valid_management_checkpoint(saved) ||
        !valid_management_checkpoint(original_before) ||
        saved.native_pair.audio.cursor !=
            saved.native_pair.physical.samples_elapsed ||
        original_before.native_pair.audio.cursor != expected_cursor ||
        original_before.native_pair.audio.accepted_sequence !=
            expected_sequence ||
        fresh_port.owner != selected.body.get() ||
        fresh_port.custody.get() != selected.body.get() ||
        fresh_port.sample_rate != manager.native_.engine->sample_rate() ||
        !Engine::same_prepared_determination(
            saved.native_pair.audio.determination, selected.determination) ||
        !Engine::same_lineage(manager.native_.engine->current_source().identity,
                              selected.determination.identity) ||
        saved.native_pair.audio.has_receiving != bool(saved_receiver) ||
        saved.native_pair.audio.has_route_programs != bool(fresh_seed))
      return false;
    const auto state = manager.device_.receipt().state;
    if (state != DeviceState::Closed && state != DeviceState::Prepared)
      return false;
    auto &engine = *manager.native_.engine;
    auto old_guard = engine.acquire_stopped_custody();
    if (!old_guard || engine.samples_elapsed() != expected_cursor ||
        engine.accepted_sequence() != expected_sequence ||
        manager.native_.body->samples_elapsed() != expected_cursor)
      return false;
    if (engine.combined_control_revision_ ==
        std::numeric_limits<std::uint64_t>::max())
      return false;
    auto selected_guard = selected.engine->acquire_stopped_custody();
    if (!selected_guard)
      return false;
    auto next = std::unique_ptr<Retained>(new Retained());
    next->before_ = snapshot(manager, old_guard);
    if (!exact_before(*next->before_, original_before))
      return false;
    next->original_ = std::make_unique<ManagementCheckpoint>(saved);
    next->operative_ = std::make_unique<ManagementCheckpoint>(saved);
    next->operative_->transport_epoch = expected_epoch + 1;
    next->owner_ = &manager;
    auto &admitted = next->operative_->native_pair.audio;
    for (const auto &note : selected.notes)
      if (!selected.engine->validate_candidate_note_target(
              note, admitted.determination))
        return false;
    manager.validate_catalog(selected_catalog, admitted.determination);
    if (fresh_seed) {
      if (!fresh_port.route_manifest ||
          fresh_seed->manifest.admitted_cursor != admitted.cursor ||
          !selected.engine->valid_route_programs(*fresh_seed, fresh_port,
                                                 admitted.determination) ||
          admitted.route_programs.program_count != fresh_seed->program_count)
        return false;
      // Only derivative native admission seals/cursor change. All original
      // phase/Hz/share/calibration/route identities and dynamics remain saved.
      auto &programmes = admitted.route_programs;
      programmes.manifest.admitted_cursor =
          fresh_seed->manifest.admitted_cursor;
      programmes.manifest.source_basis_seal =
          fresh_seed->manifest.source_basis_seal;
      for (std::size_t i = 0; i < programmes.program_count; ++i) {
        const auto &fresh = fresh_seed->programs[i].handle;
        auto &handle = programmes.programs[i].handle;
        handle.preparation_seal = fresh.preparation_seal;
        handle.program_seal = fresh.program_seal;
        programmes.manifest.programs[i].preparation_seal =
            fresh.preparation_seal;
        programmes.manifest.programs[i].program_seal = fresh.program_seal;
      }
    }
    if (!selected.engine->validate_checkpoint_for_port(
            admitted, selected_guard, 0, fresh_port, saved_receiver,
            admitted.cursor))
      return false;
    auto physical = selected.body->copy_stopped_numerical_candidate();
    if (!physical->restore_checkpoint(saved.native_pair.physical,
                                      selected.body->body_revision(), 0))
      return false;
    static_assert(noexcept(selected.body->adopt_stopped_numerical_candidate(
        std::move(*physical))));
    // Restore only the OFFSIDE numerical P, then qualify the exact public
    // snapshots before any resident pointer/state is changed.
    selected.body->adopt_stopped_numerical_candidate(std::move(*physical));
    ql::PhysicalSnapshot physical_snapshot{};
    if (!ql::write_physical_snapshot(*selected.body, physical_snapshot,
                                     admitted.determination.body_revision,
                                     admitted.cursor))
      return false;
    if (saved_receiver) {
      NativeReceivingReadback receiver_snapshot{};
      if (!saved_receiver->observe(saved_receiver->owner, receiver_snapshot,
                                   admitted.cursor))
        return false;
    }
    next->selected_ = std::move(selected);
    next->port_ = fresh_port;
    if (saved_receiver)
      next->receiver_ = *saved_receiver;
    next->catalog_ = std::move(selected_catalog);
    // Precharge/retain the complete exact pre-pulse state before publication.
    // Normal feedback legitimately drains original input/application journals.
    next->pre_pulse_ =
        std::make_unique<ManagementCheckpoint>(*next->operative_);
    // The complete original before source/queues/P/M4/input history is checked
    // again under the same uninterrupted stopped guard. Concurrent emergency
    // state or a changed epoch/cursor cannot be hidden behind matching labels.
    auto now = snapshot(manager, old_guard);
    if (manager.transport_epoch_ != expected_epoch ||
        engine.samples_elapsed() != expected_cursor ||
        engine.accepted_sequence() != expected_sequence ||
        !exact_before(*now, *next->before_))
      return false;
    // Publish only nofail swaps and complete prevalidated numeric state.
    // No OS/device operation, queue minting, partial-J relabel, P advance
    // or callback runs here. All old resource custody remains in next.
    std::swap(engine.body_, next->port_);
    std::swap(engine.receiving_, next->receiver_);
    engine.commit_checkpoint_state(admitted);
    std::swap(manager.native_.body, next->selected_.body);
    std::swap(manager.native_.notes, next->selected_.notes);
    manager.native_.determination = admitted.determination;
    std::swap(manager.catalog_, next->catalog_);
    manager.catalog_identity_ = admitted.determination.identity;
    ++manager.catalog_revision_;
    manager.bindings_.restore_validated(saved.bindings);
    manager.control_recording_failed_ = saved.recording_failed;
    manager.release_pending_ = saved.release_pending;
    manager.panic_applied_ = saved.panic_applied;
    manager.release_request_ = saved.release_request;
    manager.release_sequence_ = saved.release_sequence;
    manager.release_proof_cursor_ = saved.release_proof_cursor;
    ack = {expected_epoch,    expected_epoch + 1, expected_cursor,
           expected_sequence, admitted.cursor,    admitted.accepted_sequence,
           transaction,       checkpoint_ref};
    ++manager.transport_epoch_;
    if (!manager.capture_stopped_reading_from(admitted, old_guard))
      std::terminate();
    next->committed_ = true;
    retained.swap(next);
    return true;
  }

  // Exactly the original normal pulse already returned by SAME native owner.
  // This never calls pulse(), advances, renders or creates an application.
  // Failure retains pre-pulse/original/current evidence and requires a hold;
  // the caller must not retry the committed source transaction.
  static bool complete_pulse(PerformanceManagement &manager,
                             const ManagementPulse &pulse, Retained &retained) {
    if (!retained.committed_ || retained.pulse_completed_ ||
        retained.owner_ != &manager || !retained.pre_pulse_ ||
        !pulse.has_readback || !manager.has_latest_ ||
        manager.transport_epoch_ != retained.pre_pulse_->transport_epoch)
      return false;
    auto &engine = *manager.native_.engine;
    const auto state = manager.device_.receipt().state;
    if (state != DeviceState::Closed && state != DeviceState::Prepared)
      return false;
    auto guard = engine.acquire_stopped_custody();
    if (!guard ||
        engine.samples_elapsed() !=
            retained.pre_pulse_->native_pair.audio.cursor ||
        engine.accepted_sequence() !=
            retained.pre_pulse_->native_pair.audio.accepted_sequence)
      return false;
    auto expected =
        std::make_unique<ManagementCheckpoint>(*retained.pre_pulse_);
    auto &audio = expected->native_pair.audio;
    const auto &native_reading = manager.latest_;
    if (pulse.reading.samples_elapsed != audio.cursor ||
        pulse.reading.last_sequence != audio.applied_sequence ||
        pulse.reading.last_applied_application_ordinal !=
            audio.applied_application_ordinal ||
        !(pulse.reading.identity == audio.determination.identity) ||
        !Engine::same_prepared_determination(pulse.reading.determination,
                                             audio.determination) ||
        pulse.reading.held_touch_tokens != native_reading.held_touch_tokens ||
        native_reading.samples_elapsed != audio.cursor)
      return false;
    NativeInputBindings bindings;
    bindings.restore_validated(expected->bindings);
    std::size_t applied_count = 0;
    auto &applications = audio.applications;
    for (unsigned count = 0;
         count < 256 && applications.read != applications.write; ++count) {
      const auto &actual = applications.storage[applications.read % 256];
      if (actual.applied_application_ordinal >
          audio.applied_application_ordinal)
        break;
      if (applied_count == pulse.applications.size() ||
          !same_application(actual, pulse.applications[applied_count]) ||
          !bindings.application(actual))
        return false;
      ++applied_count;
      ++applications.read;
      if (actual.kind == Kind::Panic && actual.applied &&
          actual.sequence == expected->release_sequence)
        expected->panic_applied = true;
    }
    if (applied_count != pulse.applications.size() ||
        !bindings.retire_absent(native_reading))
      return false;
    std::size_t history_count = 0;
    InputBindingRecord record{};
    for (unsigned count = 0; count < 256 && bindings.pop_history(record);
         ++count) {
      if (history_count == pulse.input_history.size() ||
          text(history(record)) !=
              text(history(pulse.input_history[history_count])))
        return false;
      ++history_count;
    }
    if (history_count != pulse.input_history.size() ||
        pulse.last_input_ordinal != bindings.history_ordinal())
      return false;
    bindings.write_checkpoint(expected->bindings);
    if (expected->release_pending &&
        (expected->release_request || expected->panic_applied) &&
        release_zero_proven(native_reading, expected->release_request,
                            expected->release_sequence)) {
      expected->release_pending = false;
      expected->release_proof_cursor = native_reading.samples_elapsed;
    }
    if (audio.recording.failure != RecordingFailure::None)
      expected->recording_failed = true;
    if (pulse.release_pending != expected->release_pending ||
        pulse.release_request != expected->release_request ||
        pulse.release_sequence != expected->release_sequence ||
        pulse.release_proof_cursor != expected->release_proof_cursor ||
        pulse.release_zero_proven !=
            (expected->release_proof_cursor != 0 && !expected->release_pending))
      return false;
    auto after = snapshot(manager, guard);
    if (!exact_before(*expected, *after))
      return false;
    retained.operative_ = std::move(after);
    retained.pulse_completed_ = true;
    return true;
  }
};
} // namespace ql::performance
#endif
