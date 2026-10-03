#ifndef QL_PERFORMANCE_OFFLINE_HPP
#define QL_PERFORMANCE_OFFLINE_HPP
#include <ql/performance_physical.hpp>
namespace ql::performance {
// Existing C/Act/current native owner establishes these immutable seals from
// the actual retained Performance and its source/checkpoint. This numerical
// consumer does not ratify a browser-carried document or grant file authority.
struct NativeOfflineRenderScope {
  Ref session{}, scene{}, performance_revision{}, basis_seal{},
      event_prefix_seal{}, checkpoint_ref{};
  Identity expected_source{};
  std::uint64_t expected_body_revision = 0, expected_cursor = 0,
                expected_accepted_sequence = 0;
};
struct NativeOfflineRenderChunk {
  Result result = Result::Unavailable;
  const char *reason = "native offline render not committed";
  std::uint64_t start_sample = 0, committed_cursor = 0;
  unsigned frames = 0;
  bool state_committed = false, capture_complete = false;
  Readback reading{};
  Capture capture{};
};
inline std::unique_ptr<NativeOfflineRenderChunk> render_native_offline_chunk(
    Engine &engine, const std::shared_ptr<ql::PhysicalBody> &body,
    const NativeOfflineRenderScope &scope, float *output, std::size_t frames) {
  auto result = std::make_unique<NativeOfflineRenderChunk>();
  if (output && frames && frames <= max_frames)
    std::fill_n(output, frames, 0);
  if (!output || !frames || frames > max_frames || !body ||
      !engine.owns_physical_owner(body.get()) ||
      engine.device_callbacks_running() || !valid_ref(scope.session) ||
      !valid_ref(scope.scene) || !valid_ref(scope.performance_revision) ||
      !valid_ref(scope.basis_seal) || !valid_ref(scope.event_prefix_seal) ||
      !valid_ref(scope.checkpoint_ref)) {
    result->reason = "qualified existing-native stopped export scope required";
    return result;
  }
  {
    auto guard = engine.acquire_stopped_custody();
    if (!guard || engine.samples_elapsed() != scope.expected_cursor ||
        engine.accepted_sequence() != scope.expected_accepted_sequence ||
        !(engine.current_source().identity == scope.expected_source) ||
        body->body_revision() != scope.expected_body_revision ||
        body->samples_elapsed() != scope.expected_cursor) {
      result->result = Result::Stale;
      result->reason = "offline source/queue/body/cursor custody differs";
      return result;
    }
    // Drains occur under actual stopped custody, before this finite render.
    // The existing serial management owner excludes another producer/consumer.
    Readback reading{};
    for (unsigned i = 0; i < 64 && engine.pop_readback(reading); ++i) {
    }
    Capture capture{};
    for (unsigned i = 0; i < capture_capacity && engine.pop_capture(capture);
         ++i) {
    }
    engine.enable_capture(true);
    body->clear_advance_refusal();
  }
  result->start_sample = scope.expected_cursor;
  result->frames = unsigned(frames);
  if (!engine.render(output, frames, scope.expected_cursor)) {
    result->committed_cursor = engine.samples_elapsed();
    result->state_committed = result->committed_cursor != scope.expected_cursor;
    result->reason =
        body->advance_refusal_reason() ? body->advance_refusal_reason()
        : body->samples_elapsed() == scope.expected_cursor
            ? "actual native A/P callback refused before physical commit"
            : "actual native A/P callback refused after physical commit";
    return result;
  }
  result->state_committed = true;
  result->committed_cursor = engine.samples_elapsed();
  result->capture_complete =
      engine.pop_capture(result->capture) &&
      engine.pop_readback(result->reading) &&
      result->capture.start_sample == scope.expected_cursor &&
      result->capture.frames == frames &&
      result->reading.samples_elapsed == scope.expected_cursor + frames &&
      result->reading.physical.samples_elapsed ==
          result->reading.samples_elapsed &&
      result->reading.body_revision == scope.expected_body_revision &&
      result->reading.available &&
      result->reading.recording.failure == RecordingFailure::None;
  if (!result->capture_complete) {
    result->reason = "native render committed but capture/readback incomplete; "
                     "export failed";
    return result;
  }
  for (std::size_t i = 0; i < frames; ++i)
    if (!std::isfinite(output[i]) ||
        output[i] != result->capture.output_linear[i]) {
      result->capture_complete = false;
      result->reason = "captured native PCM differs; export failed";
      return result;
    }
  result->result = Result::Accepted;
  result->reason = nullptr;
  return result;
}
} // namespace ql::performance
#endif
