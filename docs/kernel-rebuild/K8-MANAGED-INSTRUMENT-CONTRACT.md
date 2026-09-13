# K8 managed continuous instrument

Owner: #132, extending the accepted `K8-COUPLED-EVENT-CONTRACT.md`,
`K8-ACKNOWLEDGEMENT-CONTRACT.md` and `K8-AUDIO-OUTPUT-CONTRACT.md`.
This is executable local hosting and delivery, not a new domain engine,
registry, authority service, desktop shell or Nara implementation.

## One native process, two rates of disclosure

`ql_mef::continuous::host::FieldHost` contains exactly one existing
`CoupledFieldSession`. The installed `ql-field-host` binary supervises that
session over bounded newline-delimited JSON on standard input/output:

```sh
cargo build --locked -p ql-mef --bin ql-field-host
ql-field-host /absolute/path/ql-field-worker /absolute/path/host-config.json
```

The configuration has exactly `instance_ref`, `basis` (the complete existing
`CoupledInput`) and `field` (`FieldInput`). The instance reference is supplied by
the material host; it is not the astronomical event, Nara identity, or permission.
The native O:I/Workcell host controls access to the process and pipe. No network
listener, shell interpolation, ambient source access or authentication substitute
is introduced. Start/re-entry through another process is a deliberate instance
lifecycle operation, not automatic replay of a timed-out command.

Ready and command replies use `ql.field-host-receipt/v1`. They carry
`instance_ref`, `request_id`, `last_request_id`, `status`, `available`, `error`,
`field` and an explicit standing. Status is `ready`, `ok`, `refused` or
`unavailable`. The `field` is the existing `ql.continuous-field/v1`, never a new
interpretation of M1/M2/M3. Initial, error and source-inspection responses contain
no PCM. Only an acknowledged advance emits its newly produced interval.

The normal data plane returns compact native targets, PCM and exact bases/clock
references. `Inspect` returns the full original/current `CoupledBasis` and original
`FieldInput` under `sources`. Inspection is separately requested because repeated
copies of the full source matrices and sky receipts are not audio data. The full
objects remain available in the same owner; this is not a reduced replacement
for any accepted engine.

## Versioned command and acknowledgement law

`ql.field-host-request/v1` has exactly:

```json
{
  "schema": "ql.field-host-request/v1",
  "instance_ref": "host-supplied-instance",
  "event_ref": "existing-event",
  "subject_ref": "existing-subject",
  "request_id": "1",
  "expected_generation": "1",
  "expected_samples_elapsed": "0",
  "command": {"operation": "advance", "frames": 512, "muted": false}
}
```

Operations are `read`, `inspect`, `advance`, `set-axis` and `replace`. `set-axis`
receives the existing `axis` and `phase: LiftInput`; `replace` receives the full
new `basis: CoupledInput`. No arbitrary operation, unknown fields, implicit mute
or changed subject is accepted. All numerical mutation goes through the current
Rust owner and the checked C++ acknowledgement path.

The request sequence, native control generation, native physical sample cursor,
M1/M2/M3 source generations, astronomical epoch and device/presentation time are
distinct. All three host cursor strings are canonical unsigned decimal u64.
A scoped, well-formed envelope consumes the next request sequence even if its
command is refused. Foreign scope, malformed envelopes and non-next request IDs
do not. Replies disclose the last admitted request sequence. Every operation
also compares the caller's two expected native cursors before native dispatch.
A repeated request cannot repeat a numerical act.

A complete explicit refusal retains the old native state and keeps the owner
readable. A missing, malformed, oversized, timed-out or post-commit native
acknowledgement is unavailable, not a clean refusal. The process closes and does
not retry. Its last receipt remains evidence of an earlier acknowledgement,
not an assertion of present native state or rollback.

The pipe caps input/configuration at 32 MiB, output at 64 MiB, advance at 8,192
samples, and each native request timeout at five seconds. Oversized or incomplete
framing closes the owner rather than interpreting a suffix as another command.
Source/geometry bounds remain those of the existing producer. No graph, provider,
model, input parsing or source inspection runs inside an audio callback.

## Sound and retained field delivery

The installed `adapters/retained-field/instrument-session.mjs` exports
`InstrumentSession`. It receives the real host's ready receipt, a host-authorised
`transport.request(request)` adapter, the actual retained field binding, an audio
context and the single owner object. An adapter may also provide `close()` to
release its endpoint on disposal or uncertainty. Subject refs are not grants.

`pump()` admits at most one native block. Only one command can be in flight.
`start()` repeats that bounded operation outside the audio callback; it does not
integrate another movement. Lookahead, block count, retained bytes, number of
companion views, sample rate and timeout have explicit finite ceilings. There is
no catch-up burst after suspension and no unbounded queue per companion.

The same acknowledged frame supplies native PCM to `NativeAudioBinding` and
identified targets to `RetainedFieldBinding`. The new pure `validate(frame)`
method on the existing GPU binding checks a future frame **without** changing
textures, source cursors, positions or velocities. Its existing `apply(frame)`
continues to own actual target mutation; no numerical implementation is copied.

The target positions are END-of-block state. They are queued for the exact device
time reported for the end of their PCM interval, not applied as soon as a future
block arrives. `present()` applies the newest due frame. Missed presentation
frames are explicitly coalesced; sound is not dropped or replayed to match display
refresh. The renderer retains ownership of GPU stepping and its resident buffers.
The delivery owner never calls `step`, `seedInitialState`, a geometric sampler,
M1/M2/M3 derivation or another oscillator.

`operate()` serializes explicit `set-axis`/`replace` domain operations through the
same host. Existing queued sound retains its old source basis until its time;
the changed generation is exposed at the same native sample boundary. `inspect()`
retrieves full source objects only for the authorised owner. Mute is presentation
only: native mode state and sample continuation are retained.

Presentation failure after a valid native acknowledgement retains the new native
cursor while holding the failed presentation. It does not report rollback.
`recover(reason)` reads the current native owner and establishes a new device
epoch with an explicit discontinuity. It does not synthesise missed sound or
reset native material. An unknown native acknowledgement cannot use this recovery
route: a separately reconciled/opened native instance is required.

## Companion views and protected state

`openView()` returns only `{read, close}`. Reads return a copied latest reference,
cursor, readiness and presentation-timing reading; they do not invoke the native
process, expose a command/inspection handle or allocate sound/GPU ownership.
There is no queued history for each view. Raw targets, PCM and original/current
personal/source objects are not included. The actual host still authorises
cross-window/context disclosure: even a protected subject ref is not implicitly
public. A same-event second subject remains a separate supplied native instance.
The tests' controlled subjects do not replace #134's Nara constitution/centres.

Native object references survive disclosure/focus without becoming desktop layout
identities. #133 continues to own focused UX; #94 owns the actual original musical
performance and property producers. These receiving interfaces do not fabricate
those missing inputs from UI status, sample positions or an assumed Nara identity.

## Executable evidence

`node --test adapters/retained-field/native-audio.test.mjs
adapters/retained-field/instrument-session.test.mjs` exercises scheduling,
end-of-block target timing, no duplicate/in-flight operations, bounded queues and
views, mute, domain changes, interruption, actual-ack-versus-presentation failure
and 3,000-block continuation. Those unit tests use explicit controlled transports,
not claimed native computation.

`scripts/test-k8-host.py` executes the actual installed Rust host and C++ worker
from the existing real-dated `k8_coupled` input. It checks source-complete inspection,
100 read-only requests, compact output, explicit replacement, stable samples,
invalid/foreign/stale/duplicate refusal, mute/resume, exact original replay,
independent controlled subjects and actual worker loss. Existing coupled, native,
retained GPU, audio graph, source and ledger tests remain required in CI.

Current-source census entries classify hosting, scheduling and target delivery as
infrastructure over existing refs. They do not manufacture a new coordinate or
retarget historical K4/ledger readiness. C/C++ numerical owners and the AW profile
body are preserved. Actual device/GPU/desktop/private-source and human acceptance
remain separate exact-version installation evidence under #132/#133/#134.
