#ifndef QL_PERFORMANCE_CAPTURE_WIRE_HPP
#define QL_PERFORMANCE_CAPTURE_WIRE_HPP
// Original callback observations only. This control-thread codec never
// integrates P, renders a block, reconstructs PCM or creates a clock/device.
#include <ql/performance_management.hpp>
namespace ql::performance::native_capture_transport {
namespace wire = checkpoint_transport;
using J = json_object;
using Json = wire::Json;
template <class T, std::size_t N>
inline Json samples(const std::array<T, N> &input, std::uint32_t frames) {
  require(frames > 0 && frames <= N, "native captured frame bound differs");
  auto out = wire::array();
  for (std::uint32_t i = 0; i < frames; ++i) {
    require(std::isfinite(input[i]),
            "actual native capture contains nonfinite output");
    wire::append(out.get(), json_object_new_double(double(input[i])));
  }
  return out;
}
inline Json audio(const Capture &v) {
  require(v.frames > 0 && v.frames <= max_frames && v.sample_rate >= 8000 &&
              v.sample_rate <= 192000 && valid_ref(v.preparation_ref) &&
              valid_ref(v.state_ref) && v.body_revision &&
              v.end_applied_sequence >= v.start_applied_sequence &&
              v.end_application_ordinal >= v.start_application_ordinal &&
              v.start_sample <=
                  std::numeric_limits<std::uint64_t>::max() - v.frames &&
              v.route_count <= ql::physical_max_personal_force_routes,
          "actual captured source/body/sequence/frame scope differs");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-audio-capture/v1");
  wire::put(out.get(), "identity", wire::identity(v.identity).release());
  wire::put(out.get(), "end_identity",
            wire::identity(v.end_identity).release());
  wire::ref(out.get(), "preparation_ref", v.preparation_ref);
  wire::ref(out.get(), "state_ref", v.state_ref);
  wire::u64(out.get(), "body_revision", v.body_revision);
  wire::put(out.get(), "sample_rate", json_object_new_uint64(v.sample_rate));
  wire::u64(out.get(), "start_sample", v.start_sample);
  wire::put(out.get(), "frames", json_object_new_uint64(v.frames));
  wire::u64(out.get(), "start_applied_sequence", v.start_applied_sequence);
  wire::u64(out.get(), "end_applied_sequence", v.end_applied_sequence);
  wire::u64(out.get(), "start_application_ordinal",
            v.start_application_ordinal);
  wire::u64(out.get(), "end_application_ordinal", v.end_application_ordinal);
  for (const auto &pair : {std::pair{"force_newtons", &v.force_newtons},
                           {"note_force_newtons", &v.note_force_newtons},
                           {"contact_force_newtons", &v.contact_force_newtons},
                           {"body_gain_linear", &v.body_gain_linear},
                           {"monitor_gain_linear", &v.monitor_gain_linear},
                           {"force_scale_newtons", &v.force_scale_newtons}})
    wire::put(out.get(), pair.first, samples(*pair.second, v.frames).release());
  for (const auto &pair : {std::pair{"pickup_linear", &v.pickup_linear},
                           {"received_linear", &v.received_linear},
                           {"output_linear", &v.output_linear}})
    wire::put(out.get(), pair.first, samples(*pair.second, v.frames).release());
  wire::flag(out.get(), "has_receiving", v.has_receiving);
  if (v.has_receiving)
    wire::put(out.get(), "receiving_manifest",
              wire::receiving_manifest(v.receiving_manifest).release());
  else
    wire::put_null(out.get(), "receiving_manifest");
  auto routes = wire::array();
  for (std::size_t i = 0; i < v.route_count; ++i)
    wire::append(routes.get(),
                 samples(v.route_force_newtons[i], v.frames).release());
  wire::put(out.get(), "route_force_newtons", routes.release());
  return out;
}
inline Json device(const DeviceCapture &v) {
  require(v.frames > 0 && v.frames <= max_frames && v.device_epoch &&
              v.device_id && v.sample_rate >= 8000 && v.sample_rate <= 192000 &&
              v.native_start_sample <=
                  std::numeric_limits<std::uint64_t>::max() - v.frames &&
              std::isfinite(v.device_sample_time) &&
              v.callback_end_host_time >= v.callback_begin_host_time,
          "actual AUHAL capture lost native device/clock/frame scope");
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-device-capture/v1");
  wire::put(out.get(), "device_id", json_object_new_uint64(v.device_id));
  wire::u64(out.get(), "device_epoch", v.device_epoch);
  wire::put(out.get(), "sample_rate", json_object_new_uint64(v.sample_rate));
  wire::u64(out.get(), "native_start_sample", v.native_start_sample);
  wire::put(out.get(), "frames", json_object_new_uint64(v.frames));
  wire::flag(out.get(), "has_host_time", v.has_host_time);
  wire::flag(out.get(), "has_device_sample_time", v.has_device_sample_time);
  wire::flag(out.get(), "clock_continuous", v.clock_continuous);
  wire::real(out.get(), "device_sample_time", v.device_sample_time);
  wire::u64(out.get(), "host_time", v.host_time);
  wire::u64(out.get(), "callback_begin_host_time", v.callback_begin_host_time);
  wire::u64(out.get(), "callback_end_host_time", v.callback_end_host_time);
  wire::put(out.get(), "output_linear",
            samples(v.output_linear, v.frames).release());
  return out;
}
inline Json batch(PerformanceManagement &owner, const ManagementPulse &pulse) {
  auto out = wire::object();
  wire::text(out.get(), "schema", "ql.native-performance-capture-batch/v1");
  wire::ref(out.get(), "session_ref", owner.session_ref());
  wire::u64(out.get(), "transport_epoch", owner.transport_epoch());
  wire::put(out.get(), "sample_rate",
            json_object_new_uint64(owner.native().engine->sample_rate()));
  wire::flag(out.get(), "capture_enabled",
             owner.native().engine->capture_enabled());
  auto captured = wire::array(), devices = wire::array();
  // The existing serial Control consumes these queues while serializing its
  // ORIGINAL pulse. Direct numerical Manager readers keep their original
  // pop_capture APIs; there is no second tap or callback-owned collection.
  if (pulse.has_readback) {
    Capture v{};
    for (unsigned i = 0;
         i < capture_capacity &&
         owner.pop_audio_capture_up_to(v, pulse.reading.samples_elapsed);
         ++i)
      wire::append(captured.get(), audio(v).release());
    DeviceCapture d{};
    for (unsigned i = 0;
         i < device_capture_capacity &&
         owner.pop_device_capture_up_to(d, pulse.reading.samples_elapsed);
         ++i)
      wire::append(devices.get(), device(d).release());
  }
  wire::put(out.get(), "audio_blocks", captured.release());
  wire::put(out.get(), "device_blocks", devices.release());
  auto counters = wire::object();
  wire::flag(counters.get(), "has_callback_readback",
             pulse.has_readback && pulse.reading.callback_output_committed);
  wire::u64(counters.get(), "dropped_audio_blocks_at_readback",
            pulse.reading.dropped_captures);
  wire::u64(counters.get(), "dropped_readbacks_at_readback",
            pulse.reading.dropped_readbacks);
  wire::u64(counters.get(), "native_queue_overflows_at_readback",
            pulse.reading.overflows);
  wire::u64(counters.get(), "device_capture_drops", pulse.device.capture_drops);
  wire::u64(counters.get(), "device_callback_failures",
            pulse.device.callback_failures);
  wire::u64(counters.get(), "device_timestamp_discontinuities",
            pulse.device.timestamp_discontinuities);
  wire::put(out.get(), "counters", counters.release());
  return out;
}
// Called by the existing DeviceStart operation before the actual AUHAL start.
// Its failure restores the original observation flag; source birth/calibration
// and saved continuation are never silently changed by a prepare/Inspect.
inline bool start_device(PerformanceManagement &owner) {
  auto &engine = *owner.native().engine;
  const bool before = engine.capture_enabled();
  engine.enable_capture(true);
  if (owner.start_device())
    return true;
  engine.enable_capture(before);
  return false;
}
} // namespace ql::performance::native_capture_transport
#endif
