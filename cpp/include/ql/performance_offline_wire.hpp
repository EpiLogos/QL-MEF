#ifndef QL_PERFORMANCE_OFFLINE_WIRE_HPP
#define QL_PERFORMANCE_OFFLINE_WIRE_HPP
// Serial native management transport over the existing A/P render operation.
// The existing Rust Scene/Act/file owner admits the digest/prefix/source before
// invoking this consumer. Candidate JSON does not grant source/file authority.
#include <ql/performance_management.hpp>

namespace ql::performance::offline_transport {
namespace wire = ql::performance::checkpoint_transport;
using J = json_object;
using Json = wire::Json;
struct Scope {
  NativeOfflineRenderScope native{};
  std::string performance_digest;
};
inline bool digest_valid(const std::string &digest) noexcept {
  // The existing native C Performance owner uses this exact qualified
  // content identity; removing its prefix changes the retained source ref.
  return digest.size() == 71 && digest.compare(0, 7, "sha256:") == 0 &&
         std::all_of(digest.begin() + 7, digest.end(), [](char c) {
           return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f');
         });
}
inline Scope read_scope(J *value) {
  wire::keys(value,
             {"schema", "session_ref", "scene_ref", "performance_revision",
              "performance_digest", "basis_seal", "event_prefix_seal",
              "checkpoint_ref", "expected_source", "expected_body_revision",
              "expected_cursor", "expected_accepted_sequence"});
  require(packet::string(wire::field(value, "schema")) ==
              "ql.native-offline-render-scope/v1",
          "native offline render scope schema differs");
  Scope out{};
  out.performance_digest =
      packet::string(wire::field(value, "performance_digest"));
  require(digest_valid(out.performance_digest),
          "native retained Performance digest required");
  out.native.session = packet::ref(value, "session_ref");
  out.native.scene = packet::ref(value, "scene_ref");
  out.native.performance_revision = packet::ref(value, "performance_revision");
  out.native.basis_seal = packet::ref(value, "basis_seal");
  out.native.event_prefix_seal = packet::ref(value, "event_prefix_seal");
  out.native.checkpoint_ref = packet::ref(value, "checkpoint_ref");
  out.native.expected_source =
      packet::identity(wire::field(value, "expected_source"));
  out.native.expected_body_revision =
      wire::decimal(wire::field(value, "expected_body_revision"));
  out.native.expected_cursor =
      wire::decimal(wire::field(value, "expected_cursor"));
  out.native.expected_accepted_sequence =
      wire::decimal(wire::field(value, "expected_accepted_sequence"));
  return out;
}
inline Json scope_wire(const Scope &scope) {
  require(digest_valid(scope.performance_digest),
          "native retained Performance digest required");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-offline-render-scope/v1");
  wire::ref(out.get(), "session_ref", scope.native.session);
  wire::ref(out.get(), "scene_ref", scope.native.scene);
  wire::ref(out.get(), "performance_revision",
            scope.native.performance_revision);
  wire::text(out.get(), "performance_digest", scope.performance_digest);
  wire::ref(out.get(), "basis_seal", scope.native.basis_seal);
  wire::ref(out.get(), "event_prefix_seal", scope.native.event_prefix_seal);
  wire::ref(out.get(), "checkpoint_ref", scope.native.checkpoint_ref);
  wire::put(out.get(), "expected_source",
            wire::identity(scope.native.expected_source).release());
  wire::u64(out.get(), "expected_body_revision",
            scope.native.expected_body_revision);
  wire::u64(out.get(), "expected_cursor", scope.native.expected_cursor);
  wire::u64(out.get(), "expected_accepted_sequence",
            scope.native.expected_accepted_sequence);
  return out;
}
inline Json chunk_wire(const Scope &scope,
                       const NativeOfflineRenderChunk &chunk) {
  require(digest_valid(scope.performance_digest),
          "native retained Performance digest required");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-offline-render-chunk/v1");
  const bool accepted =
      chunk.result == Result::Accepted && chunk.state_committed &&
      chunk.capture_complete && chunk.frames && chunk.frames <= max_frames &&
      chunk.start_sample <=
          std::numeric_limits<std::uint64_t>::max() - chunk.frames &&
      chunk.committed_cursor == chunk.start_sample + chunk.frames &&
      chunk.capture.start_sample == chunk.start_sample &&
      chunk.capture.frames == chunk.frames &&
      chunk.reading.physical.samples_elapsed == chunk.committed_cursor &&
      chunk.reading.samples_elapsed == chunk.committed_cursor &&
      chunk.reading.available && !chunk.reading.dropped_captures &&
      chunk.reading.recording.failure == RecordingFailure::None;
  wire::text(out.get(), "result", accepted ? "accepted" : "refused");
  wire::text(out.get(), "performance_digest", scope.performance_digest);
  wire::text(out.get(), "reason",
             chunk.reason
                 ? chunk.reason
                 : (accepted ? "" : "native chunk commit/capture differs"));
  wire::flag(out.get(), "state_committed", chunk.state_committed);
  wire::flag(out.get(), "capture_complete", chunk.capture_complete);
  wire::u64(out.get(), "start_sample", chunk.start_sample);
  wire::u64(out.get(), "end_sample", chunk.committed_cursor);
  wire::u64(out.get(), "committed_cursor", chunk.committed_cursor);
  wire::u64(out.get(), "capture_drops", chunk.reading.dropped_captures);
  // This is the finite offline A/P callback verdict, not an OS output-device
  // callback counter. Refused/partial output is never advertised as valid PCM.
  wire::u64(out.get(), "callback_failures", accepted ? 0 : 1);
  wire::put(out.get(), "sample_rate",
            json_object_new_uint64(chunk.reading.physical.sample_rate));
  wire::put(out.get(), "channels", json_object_new_uint64(1));
  wire::text(out.get(), "sample_format",
             "linear-f32; mono actual A/P output; no resampling");
  if (accepted)
    wire::put(out.get(), "source",
              wire::identity(chunk.capture.end_identity).release());
  else
    require(json_object_object_add(out.get(), "source", nullptr) == 0,
            "refused source null allocation failed");
  wire::u64(out.get(), "body_revision", chunk.reading.body_revision);
  wire::u64(out.get(), "accepted_native_sequence",
            scope.native.expected_accepted_sequence);
  wire::u64(out.get(), "applied_native_sequence", chunk.reading.last_sequence);
  auto pcm = wire::array();
  if (accepted)
    for (unsigned i = 0; i < chunk.frames; ++i) {
      require(std::isfinite(chunk.capture.output_linear[i]),
              "nonfinite captured native PCM");
      wire::append(pcm.get(),
                   json_object_new_double(chunk.capture.output_linear[i]));
    }
  wire::put(out.get(), "interleaved_f32", pcm.release());
  return out;
}
inline Json render_chunk_wire(PerformanceManagement &owner, const Scope &scope,
                              std::size_t frames) {
  require(frames && frames <= max_frames,
          "native offline transport block must be 1..512 frames");
  require(digest_valid(scope.performance_digest),
          "native retained Performance digest required");
  std::array<float, max_frames> output{};
  auto chunk = owner.offline_render(scope.native, output.data(), frames);
  // offline_render already requires captured PCM == emitted output sample for
  // sample. JSON conversion occurs only after callback ownership is released.
  return chunk_wire(scope, *chunk);
}
} // namespace ql::performance::offline_transport
#endif
