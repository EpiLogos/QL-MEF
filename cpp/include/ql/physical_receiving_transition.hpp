#ifndef QL_PHYSICAL_RECEIVING_TRANSITION_HPP
#define QL_PHYSICAL_RECEIVING_TRANSITION_HPP
// Source-qualified metric receiver movement/requalification at A's existing
// sample boundary. The camera is absent. No new clock, store or body
// integrator.
#include <ql/physical_receiving.hpp>
namespace ql {
struct ReceivingLiveTransitionReceipt {
  std::uint64_t transaction = 0, samples_elapsed = 0,
                before_receiving_revision = 0, after_receiving_revision = 0;
  std::uint64_t before_body_revision = 0, after_body_revision = 0;
  unsigned transition_samples = 0;
  double before_effective_gain = 0, before_effective_delay_samples = 0,
         target_gain = 0, target_delay_samples = 0;
};
class PreparedReceivingTransition {
  PreparedSpatialReceiving before_, after_, candidate_;
  std::string cause_ref_;
  std::uint64_t transaction_ = 0, sample_ = 0;
  bool committed_ = false;
  static void exchange(PreparedSpatialReceiving &a,
                       PreparedSpatialReceiving &b) noexcept {
    using std::swap;
    auto &x = a.input_;
    auto &y = b.input_;
    swap(x.receiver_ref, y.receiver_ref);
    swap(x.context_ref, y.context_ref);
    swap(x.source_ref, y.source_ref);
    swap(x.policy_ref, y.policy_ref);
    swap(x.policy_revision, y.policy_revision);
    swap(x.standing, y.standing);
    swap(x.revision, y.revision);
    swap(x.source_translation_metres, y.source_translation_metres);
    swap(x.receiver_position_metres, y.receiver_position_metres);
    swap(x.receiver_forward, y.receiver_forward);
    swap(x.speed_metres_per_second, y.speed_metres_per_second);
    swap(x.minimum_distance_metres, y.minimum_distance_metres);
    swap(x.directivity, y.directivity);
    swap(x.propagation_delay, y.propagation_delay);
    swap(x.transition_samples, y.transition_samples);
    swap(a.body_preparation_ref_, b.body_preparation_ref_);
    swap(a.body_state_ref_, b.body_state_ref_);
    swap(a.eigenbasis_identity_, b.eigenbasis_identity_);
    swap(a.event_ref_, b.event_ref_);
    swap(a.subject_ref_, b.subject_ref_);
    swap(a.identity_, b.identity_);
    swap(a.body_revision_, b.body_revision_);
    swap(a.sample_rate_, b.sample_rate_);
    swap(a.source_position_metres_, b.source_position_metres_);
    swap(a.distance_metres_, b.distance_metres_);
    swap(a.gain_, b.gain_);
    swap(a.delay_samples_, b.delay_samples_);
  }

public:
  // Both arguments are retained immutable preparations. Never read the live
  // body's preparation or receiving state on the control thread. Prepare the
  // after receiver against the qualified after body for a paired body update.
  PreparedReceivingTransition(const PreparedSpatialReceiving &before,
                              PreparedSpatialReceiving after,
                              std::uint64_t transaction, std::uint64_t sample,
                              std::string cause_ref)
      : before_(before), after_(after), candidate_(std::move(after)),
        cause_ref_(std::move(cause_ref)), transaction_(transaction),
        sample_(sample) {
    reference(cause_ref_);
    require(transaction_ > 0,
            "receiving transition requires native transaction identity");
    require(
        before_.input_.receiver_ref == after_.input_.receiver_ref &&
            before_.input_.context_ref == after_.input_.context_ref &&
            before_.body_state_ref_ == after_.body_state_ref_ &&
            before_.event_ref_ == after_.event_ref_ &&
            before_.subject_ref_ == after_.subject_ref_ &&
            before_.sample_rate_ == after_.sample_rate_ &&
            after_.body_revision_ >= before_.body_revision_ &&
            after_.input_.revision >= before_.input_.revision,
        "receiving transition has disconnected event/context/body/receiver");
    require(after_.eigenbasis_identity_ == before_.eigenbasis_identity_
                ? after_.body_revision_ == before_.body_revision_
                : after_.body_revision_ > before_.body_revision_,
            "changed body basis needs an advancing native body revision");
    require(
        after_.input_.revision > before_.input_.revision ||
            (after_.body_revision_ > before_.body_revision_ &&
             after_.eigenbasis_identity_ != before_.eigenbasis_identity_),
        "receiving transition must advance a native receiver or body revision");
  }
  PreparedReceivingTransition(const PreparedReceivingTransition &) = delete;
  PreparedReceivingTransition &
  operator=(const PreparedReceivingTransition &) = delete;
  PreparedReceivingTransition(PreparedReceivingTransition &&) noexcept =
      default;
  PreparedReceivingTransition &
  operator=(PreparedReceivingTransition &&) noexcept = default;
  const PreparedSpatialReceiving &before_preparation() const noexcept {
    return before_;
  }
  const PreparedSpatialReceiving &after_preparation() const noexcept {
    return after_;
  }
  const std::string &before_body_basis_identity() const noexcept {
    return before_.eigenbasis_identity_;
  }
  const std::string &after_body_basis_identity() const noexcept {
    return after_.eigenbasis_identity_;
  }
  const std::string &cause_ref() const noexcept { return cause_ref_; }
  std::uint64_t transaction() const noexcept { return transaction_; }
  std::uint64_t sample() const noexcept { return sample_; }
  bool committed() const noexcept { return committed_; }
  // The existing native action/context owner admits source generation and
  // policy before construction. A printable cause names provenance and
  // confers no authority. The full prepared fingerprints preserve that cut.
  // For a pair, use the body's pending immutable after preparation and actual
  // admitted cursor. Neither resident body nor receiving state is mutated.
  bool preflight_after_body(const PreparedPhysicalBody &admitted_after_body,
                            const SpatialReceiving &receiving,
                            std::uint64_t admitted_cursor) const noexcept {
    return !committed_ && admitted_cursor == sample_ &&
           receiving.elapsed_ == sample_ &&
           receiving.prepared_.identity_ == before_.identity_ &&
           candidate_.matches_preparation(admitted_after_body) &&
           std::isfinite(receiving.gain_) && receiving.gain_ >= 0 &&
           receiving.gain_ <= 1 && std::isfinite(receiving.delay_) &&
           receiving.delay_ >= 0 &&
           receiving.delay_ <= receiving_history_samples - 2;
  }
  bool preflight(const PhysicalBody &body,
                 const SpatialReceiving &receiving) const noexcept {
    return preflight_after_body(body.preparation(), receiving,
                                body.samples_elapsed());
  }
  // Sole callback owner, after a successful paired body update at this same
  // boundary if any, and before the next pickup advance. On refusal neither
  // transport history/state nor output changes. Keep this object until the
  // callback acknowledgement; it owns every retired string after the swap.
  bool apply(const PhysicalBody &body, SpatialReceiving &receiving,
             ReceivingLiveTransitionReceipt &output) noexcept {
    if (!preflight(body, receiving))
      return false;
    const ReceivingLiveTransitionReceipt receipt{
        transaction_,
        sample_,
        before_.input_.revision,
        after_.input_.revision,
        before_.body_revision_,
        after_.body_revision_,
        after_.input_.transition_samples,
        receiving.gain_,
        receiving.delay_,
        after_.gain_,
        after_.delay_samples_};
    exchange(receiving.prepared_, candidate_);
    receiving.transition_remaining_ = after_.input_.transition_samples;
    if (!receiving.transition_remaining_) {
      receiving.gain_ = after_.gain_;
      receiving.delay_ = after_.delay_samples_;
    }
    committed_ = true;
    output = receipt;
    return true;
  }
};
} // namespace ql
#endif
