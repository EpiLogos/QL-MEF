#include <cassert>
#include <cmath>
#include <iostream>
#include <ql/audio_device_macos.hpp>
using namespace ql::performance;
int main() {
  MacAudioDevice owner;
  const auto initial = owner.receipt();
  assert(initial.state == DeviceState::Closed);
#if defined(__APPLE__)
  assert(initial.backend == "CoreAudio-AUHAL");
  std::cout << "actual Darwin AUHAL provider linked; no device started\n";
#else
  assert(initial.backend == "unavailable-platform" && !initial.error.empty());
  assert(MacAudioDevice::enumerate().empty());
  assert(!owner.open({}, DeviceConfig{}));
  const auto refused = owner.receipt();
  assert(refused.state == DeviceState::Failed && refused.callbacks == 0 &&
         refused.rendered_frames == 0 && refused.client_rate == 0 &&
         refused.hardware_rate == 0 && !refused.error.empty());
  assert(!owner.start() && !owner.recover({}) && !owner.recovery_needed());
  DeviceCapture capture;
  assert(!owner.pop_capture(capture));
  assert(owner.enqueue_bridge_gesture({}).result == Result::Unavailable);
  assert(owner.enqueue_native_event_gesture({}, {}).result ==
         Result::Unavailable);
  assert(std::isnan(MacAudioDevice::host_ticks_to_seconds(1)));
  assert(owner.stop() && owner.close());
  assert(owner.receipt().state == DeviceState::Closed);
  std::cout << "unsupported native platform refuses output/clock/capture; real "
               "in-memory manager remains usable\n";
#endif
}
