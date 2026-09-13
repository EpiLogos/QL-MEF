# K8 native PCM presentation and device lifecycle

This is a K8.3 presentation consumer of the existing
[`ql.continuous-field/v1`](K8-CONTINUOUS-CONTRACT.md) packet. The
[`ql.coupled-event/v1`](K8-COUPLED-EVENT-CONTRACT.md) owner continues to retain the
complete original/current M1, M2 and M3 bases. Neither a PCM block nor an audio
receipt replaces that full event. The numerical engines, musical/source tables,
C++ modal state, planetary provider and retained GPU integrator are unchanged.

Implementation: `adapters/retained-field/native-audio.mjs`, exported
`NativeAudioBinding`. It has no JavaScript package dependencies. `make -C cpp
install PREFIX=...` installs it and the existing retained target receiver under
`share/ql-mef-c/adapters/retained-field/`. The retained target receiver still uses
the host's existing Three.js/Point-Cloud renderer; this does not install a second
renderer. The host supplies and owns the AudioContext; this module does not
discover or select a physical output device.

## One producer, one sound, readable companions

The trusted native host creates one binding with its **actual session owner
object**, a usable AudioContext and an acknowledged native **read** frame whose
`audio` array is empty. Ownership is exclusive for that object within this
JavaScript realm. An owner object is not a permission token; cross-process or
detached-window authority still belongs to the native host. A companion reads the
binding's copied receipt; it does not create an extra native producer or sound.

```js
import { NativeAudioBinding } from './native-audio.mjs';

const sound = new NativeAudioBinding(audioContext, {
  owner: actualNativeSessionHandle,
  initialFrame: currentCoupledRead.field,
  gain: 0.1,
  muted: true,
  leadSeconds: 0.04,
  maxQueuedFrames: Math.min(32768, audioContext.sampleRate * 2),
  maxBlocks: 64,
});

// Called by the existing single native producer's delivery route, not a
// second animation/audio timer and not once per open view.
const playback = sound.apply(nextCoupledEvent.field);
const readableCompanionReceipt = sound.lastReceipt;
```

The host obtains browser/user permission, resumes the AudioContext from its
actual interaction route and explicitly unmutes. The class does not call
`resume()`, change source authority, discover a microphone or invent an Action.
Its defaults are muted and an explicit presentation gain of 0.1. Mute changes a
GainNode only. It does not reset modal amplitudes, clock axes, sample identities,
source generations or the native scheduler. The native worker's own muted PCM
also remains silence when received.

## Exact native intervals and the device clock

Native `generation` and `samples_elapsed` remain decimal u64 strings. If a packet
contains N PCM values and ends at E, it owns **[E−N,E)**. It must begin exactly
where the preceding admitted native interval ended. Absolute native counters are
never converted to JavaScript Number. Only a bounded difference from the selected
device epoch is converted to seconds. Values above 2^53 are covered by tests.

The device sample rate must equal the native sample rate: 8–192 kHz. There is no
implicit resampling, playback-speed change, second oscillator, retuning, frequency
lookup, limiter or automatic normalisation. Native PCM is copied once to a mono
AudioBuffer; the browser's AudioBufferSourceNode plays it at rate 1 and detune 0.
The explicit fixed presentation gain must keep peak absolute output at or below
1. Over-range output is refused rather than silently clipped. These are linear
presentation units, not sound pressure or measured loudness.

The initial device origin is rounded **up** to a device sample boundary after the
configured lead. Each later block is scheduled from its native offset to that
same origin. `ql.native-audio-receipt/v1` retains original event/subject/registry,
material/model/geometry, current M2 identity and native clock, plus its device
epoch, native origin, device origin, exact interval, gain, mute and status.

The same field packet's target geometry is its **end-of-block** state. The
receipt exposes `target_context_seconds` equal to the scheduled audio interval's
end; source-read/control receipts retain that same presentation point. The native presentation host can align its retained target submission to
that point without reconstructing symbols from particles. This adapter does not
claim sample-locked screen pixels, zero output latency, whole-desktop mounting or
physical-speaker playback. Original/current full source inspection stays in the
coupled owner, not in the audio callback.

## Admission, bounded resources and interruption

A block has at most 8,192 samples. Queue capacity is explicit: 128 to two seconds'
worth of device frames and at most 256 blocks; defaults are bounded to 32,768
frames and 64 blocks. `capacity` reports remaining conservative frame and block
budgets. Finished nodes are disconnected and removed; the receiver retains only
its current source receipt, last PCM block and bounded active queue, not an
unbounded playback history. Source retention remains with the native owner.

Validation precedes AudioBuffer allocation or scheduling. It checks event,
subject, registry, material/model/geometry identity, native M2/clock basis,
monotone revisions and exact PCM continuity. Same-cursor contradictory PCM or
source metadata, stale/overlapping/gapped intervals, nonfinite data, malformed
u64s, rate mismatch, clipping, excess queue and late arrival are refused. A
byte-equivalent last block is acknowledged as `duplicate-not-scheduled`; a read
or parameter-generation update at the current cursor creates no playback node.

Web Audio would otherwise start a late scheduled source immediately. This
adapter refuses that silent clock remapping, including a second deadline check
after allocation. Allocation/start failure does not advance this receiver's
admitted cursor. This is not a rollback of a C++ operation already acknowledged
upstream. The host retains or explicitly discards that block; it never repeats a
native `advance` to disguise lost transport.

`hold(reason)` stops/disconnects queued presentation, records discarded blocks
and retains the last source basis. It does not claim that no portion was already
heard. `rebase(currentReadFrame, reason)` validates an explicit current native read
before discarding anything, then establishes a new device epoch without native
state reset. Missed sound is not generated retrospectively and dropped native
intervals are not passed off as heard. A context interruption after running
holds playback; returning to `running` does **not** silently resume old queues.
A closed context disposes the binding. Re-entry uses a current native read in a
new binding/device context. Caller policy decides whether a lost/stale planetary
provider permits continued last-known modelling or requires `hold`; this class
neither queries nor fabricates the sky.

## Source inventory and executable checks

The same current-source census records this JavaScript module as **infrastructure**
with no invented M-coordinate or C/Rust implementation identity. Its source hash
and concrete test owners are in the existing K8 inventory. The original M ledger,
K4 assessments, engine source matrices and historical proof files are unchanged.
A declared module record is not a runtime parity assertion.

```sh
make -C cpp test install PREFIX="$PWD/target/k8-cpp"
cargo build --locked -p ql-mef --example m2_engine
python scripts/test-k8-continuous.py target/k8-cpp/bin/ql-field-worker
node --test adapters/retained-field/native-audio.test.mjs
python adapters/retained-field/test-audio-browser.py
python scripts/k8-census.py --check
bash scripts/test-k8-native.sh
```

The browser acceptance requires installed Playwright Chromium or an explicit
`--chromium /path/to/chromium`. `--inline` executes the exact local module bodies
without HTTP hosting; it does not assert module-loading/server integration.
Normal CI uses HTTP modules. The acceptance reads the **installed** receiver and
refuses a source/install hash mismatch.

The Node tests exercise admission, exact large counters, independent owners,
queue bounds, device failures, mute, source reads, provider-loss rebase,
interruption and re-entry. Their control-plane device double is not acoustic
proof. Separately the installed C++ worker supplies actual PCM at 44.1, 48 and
96 kHz; original command replay is checked exactly. OfflineAudioContext executes
real browser graph playback and compares every output sample with that native
PCM, including mute/unmute, provider return and a replacement context. Exact
hashes, numerical error, rates and environment are written to `target/k8-audio/`
and retained by the existing continuous-field CI workflow.

This is bounded native/browser evidence for **A07 and A16**, not completion of
all their clauses. It does not close #132: qualified shared root/property/graph
integration remains coordinated with #94, actual Nara centres/identity with
#134, and whole-field focused UI/desktop use with #133. Full original musical
C′/Oikonomia execution is #94's producer, not replaced by this material PCM bus.
No actual graph application/rollback, personal-data admission, human response,
physical material correspondence, therapeutic effect or owner-machine desktop
acceptance is asserted by these tests.
