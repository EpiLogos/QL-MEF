#ifndef QL_PERFORMANCE_CHECKPOINT_HPP
#define QL_PERFORMANCE_CHECKPOINT_HPP
#include <exception>
#include <ql/performance_physical.hpp>
namespace ql::performance {
struct PairedCheckpoint {
  static constexpr const char *schema = "ql.performance-physical-checkpoint/v1";
  Engine::Checkpoint audio{};
  ql::PhysicalBodyCheckpoint physical{};
};
inline PairedCheckpoint checkpoint(Engine &engine, const ql::PhysicalBody &body,
                                   const Engine::StoppedCustody &guard) {
  if (!engine.owns_physical_owner(&body))
    throw std::invalid_argument("checkpoint must use resident physical owner");
  PairedCheckpoint saved{engine.checkpoint(guard), body.checkpoint()};
  if (saved.audio.cursor != saved.physical.samples_elapsed ||
      saved.audio.determination.body_revision != saved.physical.body_revision ||
      saved.audio.determination.body_preparation_ref !=
          reference(saved.physical.preparation_ref.c_str()) ||
      saved.audio.determination.body_state_ref !=
          reference(saved.physical.state_ref.c_str()))
    throw std::invalid_argument("checkpoint body/audio causality disconnected");
  return saved;
}
// Production management checkpoint factory: every large fixed audio object
// is constructed directly in host heap custody. Numerical snapshotting still
// requires this SAME owner's stopped/acknowledged exclusive guard.
inline std::unique_ptr<PairedCheckpoint>
checkpoint_heap(Engine &engine, const ql::PhysicalBody &body,
                const Engine::StoppedCustody &guard) {
  if (!engine.owns_physical_owner(&body))
    throw std::invalid_argument("checkpoint must use resident physical owner");
  auto saved = std::make_unique<PairedCheckpoint>();
  engine.write_checkpoint(saved->audio, guard);
  saved->physical = body.checkpoint();
  if (saved->audio.cursor != saved->physical.samples_elapsed ||
      saved->audio.determination.body_revision !=
          saved->physical.body_revision ||
      saved->audio.determination.body_preparation_ref !=
          reference(saved->physical.preparation_ref.c_str()) ||
      saved->audio.determination.body_state_ref !=
          reference(saved->physical.state_ref.c_str()))
    throw std::invalid_argument("checkpoint body/audio causality disconnected");
  return saved;
}
// Exact continuation of the existing owners, under stopped exclusive custody.
// A bounded copied candidate validates P's complete eigenbasis/source/q-v
// packet before either resident state commits. No replay and no second clock.
inline bool restore_checkpoint(Engine &engine, ql::PhysicalBody &body,
                               const PairedCheckpoint &saved,
                               const Engine::StoppedCustody &guard,
                               std::uint64_t expected_cursor) {
  if (!engine.owns_physical_owner(&body) ||
      saved.audio.cursor != saved.physical.samples_elapsed ||
      body.samples_elapsed() != expected_cursor ||
      !engine.validate_checkpoint(saved.audio, guard, expected_cursor))
    return false;
  ql::PhysicalBody candidate(body);
  if (!candidate.restore_checkpoint(saved.physical, body.body_revision(),
                                    expected_cursor))
    return false;
  static_assert(std::is_nothrow_move_assignable_v<ql::PhysicalBody>,
                "paired body commit must be atomic under exclusive custody");
  body = std::move(candidate);
  // Validation is deterministic and already complete; custody excludes both
  // the producer and callback. This cannot refuse after P's validated commit.
  if (!engine.restore_checkpoint(saved.audio, guard, expected_cursor))
    std::terminate();
  return true;
}
} // namespace ql::performance
#endif
