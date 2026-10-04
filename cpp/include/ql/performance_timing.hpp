#ifndef QL_PERFORMANCE_TIMING_HPP
#define QL_PERFORMANCE_TIMING_HPP
#include <optional>
#include <ql/performance_checkpoint_wire.hpp>
#include <ql/performance_management.hpp>

namespace ql::performance::management_transport {
class Control;
}
namespace ql::performance {
// Private facts of the SAME serial native control owner. No imported receipt
// or timing JSON can fill these fields. They are observations, not another
// queue, sample clock, persistence store or grant to the selected Act.
class NativePerformanceTimingOwner {
  friend class management_transport::Control;
  std::unique_ptr<const NativeScoreAdmission> score_;
  std::optional<NativeClockAdmission> clock_;
  Determination clock_source_{};
  std::uint64_t clock_transport_epoch_ = 0;
  std::optional<NativeGestureApplication> applied_;
  std::uint64_t applied_transport_epoch_ = 0;
  std::uint64_t observed_transport_epoch_ = 0, restored_application_floor_ = 0;

  void reset() noexcept {
    score_.reset();
    clock_.reset();
    applied_.reset();
    clock_transport_epoch_ = applied_transport_epoch_ = 0;
    observed_transport_epoch_ = restored_application_floor_ = 0;
  }

  void score(const NativeScoreAdmission &value) {
    if (value.result() == Result::Accepted && value.queue().queued())
      score_ = std::make_unique<const NativeScoreAdmission>(value);
  }
  void clock(const NativeClockAdmission &value, const Determination &source,
             std::uint64_t transport_epoch) {
    if (value.result == Result::Accepted && value.sequence && value.epoch &&
        value.anchor_ordinal && value.trigger_host_ticks &&
        value.admitted_host_ticks) {
      clock_ = value;
      clock_source_ = source;
      clock_transport_epoch_ = transport_epoch;
    }
  }
  void committed(const ManagementPulse &pulse, std::uint64_t transport_epoch) {
    if (transport_epoch != observed_transport_epoch_) {
      // Restored unread applications remain genuine historical C facts. Their
      // saved high-water does not prove a new callback in this transport epoch.
      restored_application_floor_ =
          pulse.has_readback ? pulse.reading.last_applied_application_ordinal
                             : 0;
      observed_transport_epoch_ = transport_epoch;
      applied_.reset();
    }
    for (const auto &application : pulse.applications)
      if (application.applied_application_ordinal >
          restored_application_floor_) {
        applied_ = application;
        applied_transport_epoch_ = transport_epoch;
      }
  }
  static bool same_source(const Determination &a, const Determination &b) {
    const auto first = checkpoint_transport::determination(a);
    const auto second = checkpoint_transport::determination(b);
    return json_object_equal(first.get(), second.get());
  }
  checkpoint_transport::Json fact(const PerformanceManagement &owner,
                                  const ManagementPulse &pulse,
                                  const std::string &moment,
                                  std::uint64_t ordinal) const {
    namespace wire = checkpoint_transport;
    ql::require(pulse.has_readback && pulse.reading.available &&
                    pulse.recording.failure == RecordingFailure::None &&
                    !pulse.release_pending && owner.recording_available(),
                "current native timing/body/recording boundary unavailable");
    const auto &reading = pulse.reading;
    const auto &engine = *owner.native().engine;
    ql::require(!engine.device_callbacks_running() ||
                    (pulse.device.state == DeviceState::Running &&
                     pulse.device.callbacks != 0 &&
                     pulse.device.callback_failures == 0),
                "running native device/output callback boundary unavailable");
    const auto &current = engine.source_for_native_admission();
    ql::require(same_source(current, reading.determination) &&
                    reading.physical.samples_elapsed ==
                        reading.samples_elapsed &&
                    reading.physical.body_revision == current.body_revision &&
                    reading.samples_elapsed <= engine.admission_horizon(),
                "timing owner source or copied same-body cursor differs");
    auto out = wire::object();
    wire::text(out.get(), "schema", "ql.native-performance-timing-fact/v1");
    auto domain = wire::object();
    wire::ref(domain.get(), "owner_ref", owner.session_ref());
    wire::text(domain.get(), "domain", "native_samples");
    // This epoch is assigned by the existing native Management restore owner.
    // It is never inferred from instance_ref or the independent AUHAL epoch.
    wire::text(domain.get(), "epoch_ref",
               "ql:performance/transport-epoch/" +
                   std::to_string(owner.transport_epoch()));
    wire::u64(domain.get(), "requested_cursor", engine.admission_horizon());
    wire::put_null(domain.get(), "time_mapping_ref");
    wire::put(out.get(), "binding", domain.release());
    wire::text(out.get(), "moment", moment);
    wire::u64(out.get(), "transport_epoch", owner.transport_epoch());
    wire::u64(out.get(), "committed_cursor", reading.samples_elapsed);
    wire::u64(out.get(), "admission_horizon", engine.admission_horizon());
    wire::u64(out.get(), "accepted_sequence", engine.accepted_sequence());
    wire::u64(out.get(), "last_applied_application_ordinal",
              reading.last_applied_application_ordinal);
    wire::u64(out.get(), "restored_application_floor",
              restored_application_floor_);
    wire::put(out.get(), "source", wire::determination(current).release());
    wire::flag(out.get(), "device_callbacks_running",
               engine.device_callbacks_running());
    auto position = wire::object();
    wire::ref(position.get(), "instance_ref", reading.identity.instance);
    wire::ref(position.get(), "event_ref", reading.identity.event);
    wire::ref(position.get(), "subject_ref", reading.identity.subject);
    wire::u64(position.get(), "generation", reading.physical.source_generation);
    wire::u64(position.get(), "samples_elapsed", reading.samples_elapsed);
    wire::put(out.get(), "native_position", position.release());
    if (moment == "boundary") {
      ql::require(ordinal == 0,
                  "boundary does not accept a guessed queue ordinal");
      wire::u64(out.get(), "requested_cursor", reading.samples_elapsed);
      wire::u64(out.get(), "admitted_cursor", engine.admission_horizon());
      wire::put_null(out.get(), "applied_cursor");
      wire::flag(out.get(), "queued", false);
      wire::text(out.get(), "standing",
                 "actual current native clock boundary; "
                 "prepared timing only, not queue/application acknowledgement");
    } else if (moment == "score") {
      ql::require(
          score_ && score_->transport_epoch() == owner.transport_epoch() &&
              score_->session_ref() == owner.session_ref() &&
              score_->queue().operation().sequence == ordinal &&
              ordinal <= engine.accepted_sequence() &&
              score_->queue().operation().has_requested_sample &&
              same_source(score_->queue().source(), current),
          "exact privately admitted current score queue fact unavailable");
      const auto &operation = score_->queue().operation();
      wire::u64(out.get(), "requested_cursor", operation.requested_sample);
      wire::u64(out.get(), "admitted_cursor", operation.sample);
      wire::put_null(out.get(), "applied_cursor");
      wire::flag(out.get(), "queued", true);
      wire::put(out.get(), "native_operation",
                wire::operation(operation).release());
      wire::text(out.get(), "standing",
                 "actual native score queue admission; "
                 "no callback application inferred");
    } else if (moment == "clock") {
      ql::require(clock_ && clock_transport_epoch_ == owner.transport_epoch() &&
                      clock_->sequence == ordinal &&
                      ordinal <= engine.accepted_sequence() &&
                      same_source(clock_source_, current),
                  "exact actual AUHAL clock/queue admission unavailable");
      wire::u64(out.get(), "requested_cursor", clock_->requested_sample);
      wire::u64(out.get(), "admitted_cursor", clock_->accepted_sample);
      wire::put_null(out.get(), "applied_cursor");
      wire::flag(out.get(), "queued", true);
      auto clock = wire::object();
      wire::u64(clock.get(), "epoch", clock_->epoch);
      wire::u64(clock.get(), "anchor_ordinal", clock_->anchor_ordinal);
      wire::u64(clock.get(), "trigger_host_ticks", clock_->trigger_host_ticks);
      wire::u64(clock.get(), "admitted_host_ticks",
                clock_->admitted_host_ticks);
      wire::u64(clock.get(), "sequence", clock_->sequence);
      wire::real(clock.get(), "mapping_uncertainty_samples",
                 clock_->mapping_uncertainty_samples);
      wire::flag(clock.get(), "input_transit_unknown",
                 clock_->input_transit_unknown);
      wire::put(out.get(), "native_clock", clock.release());
      wire::text(out.get(), "standing",
                 "actual native AUHAL dated queue admission; "
                 "physical/browser input transit remains unmeasured");
    } else {
      ql::require(moment == "applied" && applied_ && applied_->applied &&
                      applied_transport_epoch_ == owner.transport_epoch() &&
                      applied_->applied_application_ordinal == ordinal &&
                      ordinal <= reading.last_applied_application_ordinal &&
                      applied_->has_requested_sample &&
                      applied_->identity == reading.identity &&
                      applied_->body_revision == reading.body_revision &&
                      applied_->preparation_ref ==
                          current.body_preparation_ref &&
                      applied_->state_ref == current.body_state_ref &&
                      applied_->committed_cursor <= reading.samples_elapsed,
                  "exact source/body-qualified committed native application "
                  "unavailable");
      wire::u64(out.get(), "requested_cursor", applied_->requested_sample);
      wire::u64(out.get(), "admitted_cursor", applied_->admitted_sample);
      wire::u64(out.get(), "applied_cursor", applied_->applied_sample);
      wire::flag(out.get(), "queued", true);
      wire::put(out.get(), "native_application",
                wire::application(*applied_).release());
      wire::text(out.get(), "standing",
                 "actual callback-committed application; "
                 "original requested/admitted/applied samples remain distinct");
    }
    return out;
  }
};
} // namespace ql::performance
#endif
