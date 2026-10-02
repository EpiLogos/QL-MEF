#ifndef QL_AUDIO_DEVICE_MACOS_HPP
#define QL_AUDIO_DEVICE_MACOS_HPP
#include <ql/performance_audio.hpp>
#include <memory>
#include <string>
#include <vector>
namespace ql::performance {
enum class DeviceState { Closed, Prepared, Running, Recovering, Lost, Failed };
struct DeviceDescription {
    std::uint32_t id=0;
    std::string uid,name;
    double nominal_rate=0;
    std::uint32_t buffer_frames=0, output_channels=0;
    bool default_output=false, alive=false;
};
struct DeviceConfig {
    // zero selects the system's existing default output at open time.
    std::uint32_t device_id=0, sample_rate=48000, buffer_frames=128;
    bool restore_device_settings=true;
};
struct DeviceReceipt {
    DeviceState state=DeviceState::Closed;
    DeviceDescription device{};
    DeviceConfig requested{};
    std::string backend="CoreAudio-AUHAL", error;
    std::int32_t os_status=0;
    double client_rate=0, hardware_rate=0;
    std::uint32_t actual_buffer_frames=0, device_latency_frames=0,
        stream_latency_frames=0, safety_offset_frames=0;
    std::uint64_t callbacks=0, rendered_frames=0, callback_failures=0,
        timestamp_discontinuities=0, overload_notifications=0, capture_drops=0;
    bool simple_pcm_conversion=false, sample_rate_conversion=false;
    // API-reported pipeline latency excludes input-event/ear/physical capture.
    double reported_output_latency_ms=0;
};
struct DeviceCapture {
    std::uint64_t host_time=0, callback_begin_host_time=0, callback_end_host_time=0,
        native_start_sample=0;
    double device_sample_time=0;
    std::uint32_t frames=0;
    std::array<float,max_frames> output_linear{};
};
// Every lifecycle method runs on the serial control owner. Property listeners
// only set atomic flags. recover() stops/disposes the unit before replacing its
// callback pointer; it does not start a graph/provider/UI from audio thread.
class MacAudioDevice {
    struct Impl;
    std::unique_ptr<Impl> impl_;
public:
    MacAudioDevice();
    ~MacAudioDevice();
    MacAudioDevice(const MacAudioDevice&)=delete;
    MacAudioDevice& operator=(const MacAudioDevice&)=delete;
    static std::vector<DeviceDescription> enumerate();
    bool open(std::shared_ptr<Engine> engine,const DeviceConfig& config);
    bool start();
    bool stop();
    bool close();
    // Change flags/loss are read without restarting implicitly. The host calls
    // recover with its still-resident native owner or a newly prepared owner.
    bool recovery_needed() const noexcept;
    bool recover(std::shared_ptr<Engine> engine);
    DeviceReceipt receipt() const;
    bool pop_capture(DeviceCapture& out) noexcept;
    static double host_ticks_to_seconds(std::uint64_t ticks) noexcept;
};
} // namespace ql::performance
#endif
