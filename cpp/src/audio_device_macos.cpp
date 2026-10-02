#include <ql/audio_device_macos.hpp>
#if !defined(__APPLE__)
#error "audio_device_macos.cpp requires Apple's native CoreAudio frameworks"
#endif
#include <AudioToolbox/AudioToolbox.h>
#include <CoreAudio/CoreAudio.h>
#include <CoreFoundation/CoreFoundation.h>
#include <mach/mach_time.h>
#include <sstream>

namespace ql::performance {
namespace {
AudioObjectPropertyAddress
address(AudioObjectPropertySelector selector,
        AudioObjectPropertyScope scope = kAudioObjectPropertyScopeGlobal) {
  return {selector, scope, kAudioObjectPropertyElementMain};
}
template <class T>
OSStatus get(AudioObjectID object, AudioObjectPropertyAddress a, T &result) {
  UInt32 size = sizeof(T);
  return AudioObjectGetPropertyData(object, &a, 0, nullptr, &size, &result);
}
template <class T>
OSStatus set(AudioObjectID object, AudioObjectPropertyAddress a,
             const T &value) {
  return AudioObjectSetPropertyData(object, &a, 0, nullptr, sizeof(T), &value);
}
std::string text_property(AudioObjectID object,
                          AudioObjectPropertySelector selector) {
  CFStringRef value = nullptr;
  if (get(object, address(selector), value) != noErr || !value)
    return {};
  const CFIndex size = CFStringGetMaximumSizeForEncoding(
                           CFStringGetLength(value), kCFStringEncodingUTF8) +
                       1;
  std::vector<char> bytes(static_cast<std::size_t>(size), 0);
  const bool ok =
      CFStringGetCString(value, bytes.data(), size, kCFStringEncodingUTF8);
  CFRelease(value);
  return ok ? std::string(bytes.data()) : std::string{};
}
std::uint32_t channels(AudioDeviceID device) {
  const auto a = address(kAudioDevicePropertyStreamConfiguration,
                         kAudioDevicePropertyScopeOutput);
  UInt32 bytes = 0;
  if (AudioObjectGetPropertyDataSize(device, &a, 0, nullptr, &bytes) != noErr ||
      bytes < sizeof(AudioBufferList))
    return 0;
  // Control-thread allocation only; Apple supplies a variable-sized list.
  std::vector<std::uint64_t> storage((bytes + sizeof(std::uint64_t) - 1) /
                                     sizeof(std::uint64_t));
  auto *list = reinterpret_cast<AudioBufferList *>(storage.data());
  if (AudioObjectGetPropertyData(device, &a, 0, nullptr, &bytes, list) != noErr)
    return 0;
  std::uint32_t count = 0;
  for (UInt32 i = 0; i < list->mNumberBuffers; ++i)
    count += list->mBuffers[i].mNumberChannels;
  return count;
}
DeviceDescription describe(AudioDeviceID device, AudioDeviceID default_device) {
  DeviceDescription d{};
  d.id = device;
  d.default_output = device == default_device;
  d.uid = text_property(device, kAudioDevicePropertyDeviceUID);
  d.name = text_property(device, kAudioObjectPropertyName);
  get(device, address(kAudioDevicePropertyNominalSampleRate), d.nominal_rate);
  get(device, address(kAudioDevicePropertyBufferFrameSize), d.buffer_frames);
  UInt32 alive = 0;
  get(device, address(kAudioDevicePropertyDeviceIsAlive), alive);
  d.alive = alive != 0;
  d.output_channels = channels(device);
  return d;
}
void zero(AudioBufferList *data) noexcept {
  if (!data)
    return;
  for (UInt32 i = 0; i < data->mNumberBuffers; ++i)
    if (data->mBuffers[i].mData)
      std::memset(data->mBuffers[i].mData, 0, data->mBuffers[i].mDataByteSize);
}
} // namespace
struct MacAudioDevice::Impl {
  AudioUnit unit = nullptr;
  Engine *engine = nullptr;
  std::shared_ptr<Engine> owner{};
  DeviceReceipt receipt{};
  Float64 original_rate = 0;
  UInt32 original_buffer = 0;
  bool changed_rate = false, changed_buffer = false, system_listener = false;
  unsigned device_listeners = 0;
  std::atomic<bool> dirty{false};
  std::atomic<std::uint64_t> callbacks{0}, frames{0}, failures{0},
      discontinuities{0}, overloads{0}, capture_drops{0};
  Spsc<DeviceCapture, 128> captures{};
  bool have_timestamp = false;
  double next_device_sample = 0;
  static constexpr AudioObjectPropertySelector selectors[4] = {
      kAudioDevicePropertyDeviceIsAlive, kAudioDevicePropertyNominalSampleRate,
      kAudioDevicePropertyBufferFrameSize, kAudioDeviceProcessorOverload};
  bool fail(OSStatus status, const char *operation) {
    receipt.os_status = status;
    receipt.state = DeviceState::Failed;
    receipt.error =
        std::string(operation) + " (OSStatus " + std::to_string(status) + ")";
    return false;
  }
  static OSStatus changed(AudioObjectID, UInt32 count,
                          const AudioObjectPropertyAddress *properties,
                          void *context) noexcept {
    auto &self = *static_cast<Impl *>(context);
    for (UInt32 i = 0; i < count; ++i) {
      if (properties[i].mSelector == kAudioDeviceProcessorOverload)
        self.overloads.fetch_add(1, std::memory_order_relaxed);
      else
        self.dirty.store(true, std::memory_order_release);
    }
    return noErr;
  }
  static OSStatus render(void *context, AudioUnitRenderActionFlags *flags,
                         const AudioTimeStamp *time, UInt32, UInt32 count,
                         AudioBufferList *data) noexcept {
    auto &self = *static_cast<Impl *>(context);
    self.callbacks.fetch_add(1, std::memory_order_relaxed);
    zero(data);
    if (!data || !self.engine || count == 0 || count > max_frames ||
        self.dirty.load(std::memory_order_acquire)) {
      if (flags)
        *flags |= kAudioUnitRenderAction_OutputIsSilence;
      self.failures.fetch_add(1, std::memory_order_relaxed);
      return noErr;
    }
    if (data->mNumberBuffers != self.receipt.device.output_channels) {
      self.failures.fetch_add(1, std::memory_order_relaxed);
      self.dirty.store(true, std::memory_order_release);
      return kAudioUnitErr_FormatNotSupported;
    }
    for (UInt32 channel = 0; channel < data->mNumberBuffers; ++channel)
      if (data->mBuffers[channel].mNumberChannels != 1 ||
          !data->mBuffers[channel].mData ||
          data->mBuffers[channel].mDataByteSize < count * sizeof(float)) {
        self.failures.fetch_add(1, std::memory_order_relaxed);
        self.dirty.store(true, std::memory_order_release);
        return kAudioUnitErr_FormatNotSupported;
      }
    DeviceCapture capture{};
    capture.callback_begin_host_time = mach_absolute_time();
    capture.frames = count;
    capture.native_start_sample = self.engine->samples_elapsed();
    if (time) {
      if (time->mFlags & kAudioTimeStampHostTimeValid)
        capture.host_time = time->mHostTime;
      if (time->mFlags & kAudioTimeStampSampleTimeValid) {
        capture.device_sample_time = time->mSampleTime;
        if (self.have_timestamp &&
            std::abs(time->mSampleTime - self.next_device_sample) > 0.5)
          self.discontinuities.fetch_add(1, std::memory_order_relaxed);
        self.have_timestamp = true;
        self.next_device_sample = time->mSampleTime + count;
      }
    }
    if (!self.engine->render(capture.output_linear.data(), count,
                             capture.native_start_sample)) {
      self.failures.fetch_add(1, std::memory_order_relaxed);
      self.dirty.store(true, std::memory_order_release);
      if (flags)
        *flags |= kAudioUnitRenderAction_OutputIsSilence;
      return noErr;
    }
    // Mono native pickup to the selected device's two leading channels;
    // additional channels are explicitly silent, never guessed spatial audio.
    for (UInt32 channel = 0;
         channel < std::min<UInt32>(2, data->mNumberBuffers); ++channel)
      std::memcpy(data->mBuffers[channel].mData, capture.output_linear.data(),
                  count * sizeof(float));
    self.frames.fetch_add(count, std::memory_order_relaxed);
    capture.callback_end_host_time = mach_absolute_time();
    if (!self.captures.push(capture))
      self.capture_drops.fetch_add(1, std::memory_order_relaxed);
    return noErr;
  }
  bool remove_listeners() {
    bool ok = true;
    for (unsigned i = 0; i < 4; ++i)
      if (device_listeners & (1u << i)) {
        auto a = address(selectors[i]);
        const auto status = AudioObjectRemovePropertyListener(
            receipt.device.id, &a, changed, this);
        if (status == noErr)
          device_listeners &= ~(1u << i);
        else {
          fail(status,
               "remove selected-device listener; native context retained");
          ok = false;
        }
      }
    if (system_listener) {
      auto a = address(kAudioHardwarePropertyDefaultOutputDevice);
      const auto status = AudioObjectRemovePropertyListener(
          kAudioObjectSystemObject, &a, changed, this);
      if (status == noErr)
        system_listener = false;
      else {
        fail(status, "remove default-device listener; native context retained");
        ok = false;
      }
    }
    return ok;
  }
  bool restore() {
    if (!receipt.requested.restore_device_settings || !receipt.device.id)
      return true;
    bool ok = true;
    // Restore only settings still equal to ours. A foreign user's change
    // during the session must never be overwritten by cleanup.
    UInt32 buffer = 0;
    Float64 rate = 0;
    if (changed_buffer &&
        get(receipt.device.id, address(kAudioDevicePropertyBufferFrameSize),
            buffer) == noErr &&
        buffer == receipt.requested.buffer_frames)
      ok = set(receipt.device.id, address(kAudioDevicePropertyBufferFrameSize),
               original_buffer) == noErr &&
           ok;
    if (changed_rate &&
        get(receipt.device.id, address(kAudioDevicePropertyNominalSampleRate),
            rate) == noErr &&
        rate == receipt.requested.sample_rate)
      ok =
          set(receipt.device.id, address(kAudioDevicePropertyNominalSampleRate),
              original_rate) == noErr &&
          ok;
    changed_buffer = changed_rate = false;
    return ok;
  }
};
MacAudioDevice::MacAudioDevice() : impl_(std::make_unique<Impl>()) {}
MacAudioDevice::~MacAudioDevice() {
  // A unit whose stop/disposal was refused can still own the callback. Keep
  // its context and resident Engine/P owner rather than deleting memory
  // reachable by CoreAudio. close() exposes this OS failure to the host.
  if (!close() &&
      (impl_->unit || impl_->device_listeners || impl_->system_listener))
    impl_.release();
}
std::vector<DeviceDescription> MacAudioDevice::enumerate() {
  AudioDeviceID default_device = 0;
  get(kAudioObjectSystemObject,
      address(kAudioHardwarePropertyDefaultOutputDevice), default_device);
  const auto a = address(kAudioHardwarePropertyDevices);
  UInt32 bytes = 0;
  if (AudioObjectGetPropertyDataSize(kAudioObjectSystemObject, &a, 0, nullptr,
                                     &bytes) != noErr)
    return {};
  std::vector<AudioDeviceID> ids(bytes / sizeof(AudioDeviceID));
  if (AudioObjectGetPropertyData(kAudioObjectSystemObject, &a, 0, nullptr,
                                 &bytes, ids.data()) != noErr)
    return {};
  std::vector<DeviceDescription> result;
  for (auto id : ids) {
    auto d = describe(id, default_device);
    if (d.output_channels)
      result.push_back(std::move(d));
  }
  return result;
}
bool MacAudioDevice::open(std::shared_ptr<Engine> engine,
                          const DeviceConfig &config) {
  auto &s = *impl_;
  if (s.unit || s.device_listeners || s.system_listener ||
      s.receipt.state == DeviceState::Running)
    return s.fail(kAudio_ParamError,
                  "open requires a closed device/listener owner");
  if (!engine || !engine->has_physical_custody() ||
      config.sample_rate != 48000 ||
      engine->sample_rate() != config.sample_rate ||
      (config.buffer_frames != 128 && config.buffer_frames != 256) ||
      !engine->available())
    return s.fail(
        kAudio_ParamError,
        "unsupported or unavailable native performance configuration");
  s.receipt = DeviceReceipt{};
  s.receipt.requested = config;
  s.owner = std::move(engine);
  s.engine = s.owner.get();
  AudioDeviceID default_device = 0;
  OSStatus status =
      get(kAudioObjectSystemObject,
          address(kAudioHardwarePropertyDefaultOutputDevice), default_device);
  if (status != noErr)
    return s.fail(status, "read default output device");
  const AudioDeviceID device =
      config.device_id ? config.device_id : default_device;
  if (!device)
    return s.fail(kAudioHardwareBadDeviceError, "no selected output device");
  s.receipt.device = describe(device, default_device);
  if (!s.receipt.device.alive || s.receipt.device.output_channels < 1 ||
      s.receipt.device.output_channels > 32)
    return s.fail(kAudioHardwareBadDeviceError,
                  "selected output device unavailable or unbounded");
  if ((status = get(device, address(kAudioDevicePropertyNominalSampleRate),
                    s.original_rate)) != noErr ||
      (status = get(device, address(kAudioDevicePropertyBufferFrameSize),
                    s.original_buffer)) != noErr)
    return s.fail(status, "read original device format");
  auto cleanup = [&](OSStatus error, const char *operation) {
    if (!s.remove_listeners())
      return false;
    if (s.unit) {
      const auto uninitialize = AudioUnitUninitialize(s.unit);
      if (uninitialize != noErr && uninitialize != kAudioUnitErr_Uninitialized)
        return s.fail(uninitialize,
                      "cleanup uninitialize AUHAL; native context retained");
      const auto dispose = AudioComponentInstanceDispose(s.unit);
      if (dispose != noErr)
        return s.fail(dispose,
                      "cleanup dispose AUHAL; native context retained");
      s.unit = nullptr;
    }
    if (!s.restore())
      return s.fail(kAudioHardwareUnspecifiedError,
                    "restore admitted device format");
    s.engine = nullptr;
    s.owner.reset();
    return s.fail(error, operation);
  };
  const Float64 target_rate = config.sample_rate;
  if (s.original_rate != target_rate) {
    status = set(device, address(kAudioDevicePropertyNominalSampleRate),
                 target_rate);
    if (status != noErr)
      return cleanup(status, "set admitted hardware sample rate");
    s.changed_rate = true;
  }
  if (s.original_buffer != config.buffer_frames) {
    status = set(device, address(kAudioDevicePropertyBufferFrameSize),
                 config.buffer_frames);
    if (status != noErr)
      return cleanup(status, "set admitted device frame count");
    s.changed_buffer = true;
  }
  Float64 actual_rate = 0;
  UInt32 actual_frames = 0;
  if ((status = get(device, address(kAudioDevicePropertyNominalSampleRate),
                    actual_rate)) != noErr ||
      (status = get(device, address(kAudioDevicePropertyBufferFrameSize),
                    actual_frames)) != noErr)
    return cleanup(status, "read back selected device format");
  if (actual_rate != target_rate || actual_frames != config.buffer_frames)
    return cleanup(
        kAudioUnitErr_FormatNotSupported,
        "device refused exact requested rate/buffer; no silent resampler");
  AudioComponentDescription description{};
  description.componentType = kAudioUnitType_Output;
  description.componentSubType = kAudioUnitSubType_HALOutput;
  description.componentManufacturer = kAudioUnitManufacturer_Apple;
  auto component = AudioComponentFindNext(nullptr, &description);
  if (!component)
    return cleanup(kAudioUnitErr_InvalidProperty,
                   "AUHAL component unavailable");
  if ((status = AudioComponentInstanceNew(component, &s.unit)) != noErr)
    return cleanup(status, "create AUHAL");
  const UInt32 enabled = 1, disabled = 0;
  if ((status = AudioUnitSetProperty(s.unit, kAudioOutputUnitProperty_EnableIO,
                                     kAudioUnitScope_Output, 0, &enabled,
                                     sizeof(enabled))) != noErr ||
      (status = AudioUnitSetProperty(s.unit, kAudioOutputUnitProperty_EnableIO,
                                     kAudioUnitScope_Input, 1, &disabled,
                                     sizeof(disabled))) != noErr ||
      (status = AudioUnitSetProperty(
           s.unit, kAudioOutputUnitProperty_CurrentDevice,
           kAudioUnitScope_Global, 0, &device, sizeof(device))) != noErr)
    return cleanup(status, "bind AUHAL selected output");
  AudioStreamBasicDescription hardware{};
  UInt32 size = sizeof(hardware);
  if ((status = AudioUnitGetProperty(s.unit, kAudioUnitProperty_StreamFormat,
                                     kAudioUnitScope_Output, 0, &hardware,
                                     &size)) != noErr)
    return cleanup(status, "read AUHAL hardware format");
  AudioStreamBasicDescription client{};
  client.mSampleRate = target_rate;
  client.mFormatID = kAudioFormatLinearPCM;
  client.mFormatFlags =
      kAudioFormatFlagsNativeFloatPacked | kAudioFormatFlagIsNonInterleaved;
  client.mBytesPerPacket = client.mBytesPerFrame = sizeof(float);
  client.mFramesPerPacket = 1;
  client.mChannelsPerFrame = s.receipt.device.output_channels;
  client.mBitsPerChannel = 32;
  if ((status = AudioUnitSetProperty(s.unit, kAudioUnitProperty_StreamFormat,
                                     kAudioUnitScope_Input, 0, &client,
                                     sizeof(client))) != noErr)
    return cleanup(status, "set AUHAL native float client format");
  const UInt32 ceiling = max_frames;
  AURenderCallbackStruct callback{Impl::render, &s};
  if ((status = AudioUnitSetProperty(
           s.unit, kAudioUnitProperty_MaximumFramesPerSlice,
           kAudioUnitScope_Global, 0, &ceiling, sizeof(ceiling))) != noErr ||
      (status = AudioUnitSetProperty(
           s.unit, kAudioUnitProperty_SetRenderCallback, kAudioUnitScope_Input,
           0, &callback, sizeof(callback))) != noErr ||
      (status = AudioUnitInitialize(s.unit)) != noErr)
    return cleanup(status, "prepare bounded AUHAL callback");
  for (unsigned i = 0; i < 4; ++i) {
    auto a = address(Impl::selectors[i]);
    status = AudioObjectAddPropertyListener(device, &a, Impl::changed, &s);
    if (status != noErr)
      return cleanup(status, "register selected-device listener");
    s.device_listeners |= 1u << i;
  }
  if (!config.device_id) {
    auto a = address(kAudioHardwarePropertyDefaultOutputDevice);
    if ((status = AudioObjectAddPropertyListener(kAudioObjectSystemObject, &a,
                                                 Impl::changed, &s)) != noErr)
      return cleanup(status, "register default-device listener");
    s.system_listener = true;
  }
  s.receipt.client_rate = target_rate;
  s.receipt.hardware_rate = hardware.mSampleRate;
  s.receipt.actual_buffer_frames = actual_frames;
  s.receipt.sample_rate_conversion = hardware.mSampleRate != target_rate;
  if (s.receipt.sample_rate_conversion)
    return cleanup(kAudioUnitErr_FormatNotSupported,
                   "AUHAL hardware rate mismatch");
  s.receipt.simple_pcm_conversion =
      hardware.mFormatFlags != client.mFormatFlags ||
      hardware.mBitsPerChannel != 32;
  get(device,
      address(kAudioDevicePropertyLatency, kAudioDevicePropertyScopeOutput),
      s.receipt.device_latency_frames);
  get(device,
      address(kAudioDevicePropertySafetyOffset,
              kAudioDevicePropertyScopeOutput),
      s.receipt.safety_offset_frames);
  auto streams_address =
      address(kAudioDevicePropertyStreams, kAudioDevicePropertyScopeOutput);
  UInt32 bytes = 0;
  if (AudioObjectGetPropertyDataSize(device, &streams_address, 0, nullptr,
                                     &bytes) == noErr) {
    std::vector<AudioStreamID> streams(bytes / sizeof(AudioStreamID));
    if (AudioObjectGetPropertyData(device, &streams_address, 0, nullptr, &bytes,
                                   streams.data()) == noErr)
      for (auto stream : streams) {
        UInt32 latency = 0;
        if (get(stream, address(kAudioStreamPropertyLatency), latency) == noErr)
          s.receipt.stream_latency_frames =
              std::max(s.receipt.stream_latency_frames, latency);
      }
  }
  s.receipt.reported_output_latency_ms =
      1000.0 *
      (s.receipt.device_latency_frames + s.receipt.stream_latency_frames +
       s.receipt.safety_offset_frames + actual_frames) /
      target_rate;
  s.receipt.device = describe(device, default_device);
  s.receipt.state = DeviceState::Prepared;
  s.receipt.os_status = noErr;
  s.receipt.error.clear();
  s.dirty.store(false, std::memory_order_release);
  s.have_timestamp = false;
  return true;
}
bool MacAudioDevice::start() {
  auto &s = *impl_;
  if (!s.unit || s.receipt.state != DeviceState::Prepared || s.dirty.load())
    return s.fail(kAudio_ParamError, "start requires current prepared output");
  if (!s.engine || !s.engine->begin_device_callbacks())
    return s.fail(kAudio_ParamError, "audio engine custody unavailable");
  const auto status = AudioOutputUnitStart(s.unit);
  if (status != noErr) {
    // A failed start does not establish callback quiescence. Release
    // custody only when an actual OS stop acknowledges it; otherwise the
    // resident engine/body/context remain owned for explicit cleanup.
    if (AudioOutputUnitStop(s.unit) == noErr)
      s.engine->end_device_callbacks_after_stop();
    return s.fail(status, "start AUHAL");
  }
  s.receipt.state = DeviceState::Running;
  return true;
}
bool MacAudioDevice::stop() {
  auto &s = *impl_;
  if (!s.unit)
    return true;
  const auto status = AudioOutputUnitStop(s.unit);
  if (status != noErr)
    return s.fail(status, "stop AUHAL");
  if (s.engine)
    s.engine->end_device_callbacks_after_stop();
  if (s.receipt.state == DeviceState::Running)
    s.receipt.state = DeviceState::Prepared;
  return true;
}
bool MacAudioDevice::close() {
  auto &s = *impl_;
  if (!stop())
    return false;
  bool ok = true;
  if (!s.remove_listeners())
    return false;
  if (s.unit) {
    const auto status = AudioUnitUninitialize(s.unit);
    if (status != noErr)
      return s.fail(status, "uninitialize AUHAL");
    const auto dispose = AudioComponentInstanceDispose(s.unit);
    if (dispose != noErr)
      return s.fail(dispose, "dispose AUHAL");
    s.unit = nullptr;
  }
  ok = s.restore() && ok;
  s.engine = nullptr;
  s.owner.reset();
  if (ok)
    s.receipt.state = DeviceState::Closed;
  return ok;
}
bool MacAudioDevice::recovery_needed() const noexcept {
  return impl_->dirty.load(std::memory_order_acquire);
}
bool MacAudioDevice::recover(std::shared_ptr<Engine> engine) {
  const auto config = impl_->receipt.requested;
  impl_->receipt.state = DeviceState::Recovering;
  if (!close())
    return false;
  // Native body cursor is retained by Engine; only the device epoch restarts.
  return open(std::move(engine), config) && start();
}
DeviceReceipt MacAudioDevice::receipt() const {
  auto result = impl_->receipt;
  result.callbacks = impl_->callbacks.load();
  result.rendered_frames = impl_->frames.load();
  result.callback_failures = impl_->failures.load();
  result.timestamp_discontinuities = impl_->discontinuities.load();
  result.overload_notifications = impl_->overloads.load();
  result.capture_drops = impl_->capture_drops.load();
  if (recovery_needed() && result.state == DeviceState::Running)
    result.state = DeviceState::Lost;
  return result;
}
bool MacAudioDevice::pop_capture(DeviceCapture &out) noexcept {
  return impl_->captures.take(out);
}
double MacAudioDevice::host_ticks_to_seconds(std::uint64_t ticks) noexcept {
  mach_timebase_info_data_t timebase{};
  if (mach_timebase_info(&timebase) != KERN_SUCCESS || !timebase.denom)
    return 0;
  return double(ticks) * double(timebase.numer) / double(timebase.denom) / 1e9;
}
} // namespace ql::performance
