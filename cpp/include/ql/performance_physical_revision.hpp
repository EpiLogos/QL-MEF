#ifndef QL_PERFORMANCE_PHYSICAL_REVISION_HPP
#define QL_PERFORMANCE_PHYSICAL_REVISION_HPP
// The existing serial native owner prepares this under acknowledged stopped
// custody. Running sample-dated changes use the same transaction's numerical
// commit, but require the existing host's scheduling/commit receipt separately.
#include <ql/performance_management.hpp>
#include <ql/performance_physical_routes.hpp>
#include <ql/physical_transition.hpp>
namespace ql::performance {
class PreparedRetainedBodyRevision {
  PerformanceManagement *owner_ = nullptr;
  std::shared_ptr<ql::PhysicalBody> body_;
  std::shared_ptr<const AdmittedNativeReceivingSource> after_source_;
  std::shared_ptr<PhysicalRoutesPortBinding> after_port_;
  std::unique_ptr<ql::PreparedPhysicalTransition> physical_;
  PerformanceManagement::PreparedCombinedRevision musical_;
  bool ready_ = false, committed_ = false;

public:
  // AFTER source/receiving is privately recompiled by the actual native host
  // from its current full basis/context/occasion, before invoking this seam.
  // A renderer-provided label, hash or matching candidate is no authority.
  PreparedRetainedBodyRevision(
      PerformanceManagement &owner,
      std::unique_ptr<ql::PreparedPhysicalTransition> physical,
      std::shared_ptr<const AdmittedNativeReceivingSource> after_source,
      const Determination &after, json_object *actual_after_native_basis,
      const NativeRouteProgramSet &after_seed, std::vector<NoteTarget> notes,
      std::vector<KeyboardCell> cells, const Engine::StoppedCustody &guard)
      : owner_(&owner), body_(owner.native().body),
        after_source_(std::move(after_source)), physical_(std::move(physical)) {
    if (!guard || !body_ || !physical_ || physical_->committed() ||
        !after_source_ ||
        !after_source_->matches_current(after, actual_after_native_basis))
      throw std::invalid_argument(
          "current complete native receiving/body revision required");
    const auto &prepared_after = physical_->pending_after_preparation();
    // Constructor consumes only immutable AFTER preparation and original
    // source custody. It retains the ONE original mutable P owner.
    after_port_ = std::make_shared<PhysicalRoutesPortBinding>(
        body_, after_source_, prepared_after, after, actual_after_native_basis,
        after.m1_face == 1, physical_->sample());
    const auto port = physical_routes_port(physical_port(body_), after_port_);
    if (!physical_->preflight(*body_) ||
        !owner.preflight_stopped_combined_revision(
            after, port, after_seed, std::move(notes), std::move(cells), guard,
            physical_->sample(), musical_))
      throw std::invalid_argument(
          "atomic physical/musical revision preflight refused");
    ready_ = true;
  }
  PreparedRetainedBodyRevision(const PreparedRetainedBodyRevision &) = delete;
  PreparedRetainedBodyRevision &
  operator=(const PreparedRetainedBodyRevision &) = delete;
  bool current(const Engine::StoppedCustody &guard) noexcept {
    return ready_ && !committed_ && owner_ &&
           owner_->native().body.get() == body_.get() &&
           owner_->combined_revision_current(musical_, guard) &&
           physical_->preflight(*body_);
  }
  // Same uninterrupted serial guard as preparation. Every check finishes
  // before P mutates; after P's deterministic projection the musical half
  // cannot fail. All retired allocations stay in this object until the host
  // records the actual acknowledgement. No queue/input/history is replaced.
  bool commit(const Engine::StoppedCustody &guard,
              ql::PhysicalLiveTransitionReceipt &receipt) noexcept {
    if (!current(guard) || !physical_->apply(*body_, receipt))
      return false;
    owner_->commit_stopped_combined_revision(musical_, guard);
    ready_ = false;
    committed_ = true;
    return true;
  }
  bool committed() const noexcept { return committed_; }
  const auto &after_source() const noexcept { return after_source_; }
  const auto &after_port() const noexcept { return after_port_; }
  const auto &physical_transition() const noexcept { return *physical_; }
};
} // namespace ql::performance
#endif
