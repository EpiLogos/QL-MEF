// Explicit native platform refusal; no simulated device/output clock.
#include <limits>
#include <ql/audio_device_macos.hpp>
#if defined(__APPLE__)
#error "Apple builds must link the actual audio_device_macos.cpp owner"
#endif
namespace ql::performance {
struct MacAudioDevice::Impl {
  DeviceReceipt reading{};
  Impl() {
    reading.backend = "unavailable-platform";
    reading.error = "CoreAudio-AUHAL is unavailable on this native platform";
  }
};
MacAudioDevice::MacAudioDevice() : impl_(std::make_unique<Impl>()) {}
MacAudioDevice::~MacAudioDevice() = default;
std::vector<DeviceDescription> MacAudioDevice::enumerate() { return {}; }
bool MacAudioDevice::open(std::shared_ptr<Engine>, const DeviceConfig &config) {
  impl_->reading.requested = config;
  impl_->reading.state = DeviceState::Failed;
  return false;
}
bool MacAudioDevice::start() { return false; }
bool MacAudioDevice::stop() { return true; }
bool MacAudioDevice::close() {
  impl_->reading.state = DeviceState::Closed;
  return true;
}
bool MacAudioDevice::recovery_needed() const noexcept { return false; }
bool MacAudioDevice::recover(std::shared_ptr<Engine>) { return false; }
DeviceReceipt MacAudioDevice::receipt() const { return impl_->reading; }
bool MacAudioDevice::pop_capture(DeviceCapture &) noexcept { return false; }
NativeClockAdmission
MacAudioDevice::enqueue_bridge_gesture(Operation) noexcept {
  return {};
}
NativeClockAdmission
MacAudioDevice::enqueue_native_event_gesture(Operation,
                                             NativeInputStamp) noexcept {
  return {};
}
double MacAudioDevice::host_ticks_to_seconds(std::uint64_t) noexcept {
  return std::numeric_limits<double>::quiet_NaN();
}
} // namespace ql::performance
