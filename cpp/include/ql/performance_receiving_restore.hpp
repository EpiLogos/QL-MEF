#ifndef QL_PERFORMANCE_RECEIVING_RESTORE_HPP
#define QL_PERFORMANCE_RECEIVING_RESTORE_HPP
// Exact continuation through the retained P/A/Management owners. The existing
// private source/Act/occasion owner must qualify the current receiving port
// before this numerical transaction. A checkpoint or route seal is no grant.
#include <ql/performance_management.hpp>
namespace ql::performance {
// CONTROL only, with the actual output device stopped. The retained candidate
// holds both ORIGINAL saved CP and OPERATIVE source-readmitted audio CP plus
// retired route custody until the existing control/Act owner records the ACK.
// No callback allocation, replacement body owner, clock, journal or store.
inline bool restore_current_receiving_checkpoint(
    PerformanceManagement &manager, const ManagementCheckpoint &saved,
    const PhysicalPort &fresh_port, const NativeRouteProgramSet &fresh_seed,
    std::vector<NoteTarget> current_notes,
    std::vector<KeyboardCell> current_catalog, std::uint64_t expected_cursor,
    Ref transaction, Ref checkpoint_ref,
    std::unique_ptr<PerformanceManagement::PreparedReceivingRestore> &retained,
    TransportAcknowledgement &ack,
    const ReceivingPort *saved_receiver = nullptr) {
  if (retained || !valid_ref(transaction) || !valid_ref(checkpoint_ref))
    return false;
  auto &engine = *manager.native().engine;
  auto &body = *manager.native().body;
  auto guard = engine.acquire_stopped_custody();
  if (!guard || !engine.owns_physical_owner(&body) ||
      body.samples_elapsed() != expected_cursor)
    return false;
  auto prepared =
      std::make_unique<PerformanceManagement::PreparedReceivingRestore>();
  if (!manager.preflight_stopped_receiving_restore(
          saved, fresh_port, fresh_seed, std::move(current_notes),
          std::move(current_catalog), guard, expected_cursor, *prepared,
          saved_receiver))
    return false;
  // Existing P numerical validator runs on a bounded heap copy before ANY
  // resident mutation. The candidate uses the identical prepared eigenbasis
  // and validates complete source, units, finite q/v and pickup consistency.
  auto physical_candidate = body.copy_stopped_numerical_candidate();
  if (!physical_candidate->restore_checkpoint(prepared->physical_checkpoint(),
                                              body.body_revision(),
                                              expected_cursor) ||
      !manager.receiving_restore_current(*prepared, guard))
    return false;
  static_assert(noexcept(body.adopt_stopped_numerical_candidate(
                    std::move(*physical_candidate))),
                "stopped paired numerical publication must not fail");
  // The uninterrupted stopped guard excludes producer and callback. The
  // native source owner still holds its current Act/receiving lease. All
  // remaining operations are nofail publication of prevalidated state.
  body.adopt_stopped_numerical_candidate(std::move(*physical_candidate));
  manager.commit_stopped_receiving_restore(*prepared, guard, transaction,
                                           checkpoint_ref, ack);
  retained.swap(prepared);
  return true;
}
} // namespace ql::performance
#endif
