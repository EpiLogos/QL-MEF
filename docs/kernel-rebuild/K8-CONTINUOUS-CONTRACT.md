# K8 continuous modal and coupled-clock owner

Owner: #132, successor to #165/#168. This contract implements a persistent C++
continuation over the accepted native M2 resonator and C coupled-clock laws.
It does not replace the accepted M1–M3 engines, the retained GPU solver or Nara.

## Numerical ownership and units

`cpp/include/ql/continuous_field.hpp` is an installed C++17 API over the installed
native C registry/clock. The host supplies actual constituent samples and signed
mode shape-functions, not only a chosen address. For each supplied mode it solves

`dz/dt = (-damping_per_second + i·2π·frequency_hz) z + drive_metres_per_second`.

Amplitude/displacement is in metres, shape functions are dimensionless, damping
is per second and frequency is Hz. Audio output uses an explicit linear gain per
metre, not a claim of calibrated sound pressure. The small/zero-rate limit uses
a stable `expm1(z)/z` series. Equal modes retain independent identity; opposite
amplitudes and drives cancel. This is a declared linear modal continuation, not
a constitutive law or eigenvalue solver for every supplied geometry/material.
A provider must warrant its frequencies and shapes separately. Source-backed
symbolic/music correspondence remains distinct from measured material efficacy.

C++ owns these modal amplitudes and the once-applied transform of their target
samples. The retained GPU body owns resident particle positions, velocities and
its supported integration toward those targets. Audio, visual reads and detached
views do not each start another domain integration. Source/graph/provider/JSON
work is outside real-time callbacks. `render_audio` and `write_targets` operate on
caller-provided buffers without constructing new vectors or querying providers.

The installed typed API supports 1–4096 modes, 1–1,048,576 samples, at most
16,777,216 shape vectors, 8–192 kHz and blocks of at most 8192 samples. Each
frequency must be below 0.45 of sample rate. Finite bounds are checked before
state/output mutation. These are safety ceilings, not a claim that every maximum
combination meets a real-time deadline; the host budgets and measures its device.
The management worker has tighter 65,536-sample/262,144-shape-vector JSON limits.

## Stable state and independent clocks

Sample identity/order and source constituent are fixed for one instantiated
geometry. A new frequency, damping or forcing changes the next integration from
the **resident amplitude**, not the new request's initial amplitude. Explicit
`replace_state` is different. Mode count, identity or topology changes require a
new explicitly chosen geometry instance; they cannot silently reseed a live one.

The field's local control revision, M2's original/current event generation,
physical sample cursor, native coupled-clock generation and astronomical epoch
remain separate. An axis operation changes the local control revision, not the
historical M2 receipt. A later M2 generation must be newer than its retained M2
basis, not numerically equal to an unrelated control counter. Expected control
revision **and sample cursor** prevent replay of a stale advance.

Native C owns independent inscription/lensing phase lifts, exact rational axis
rates, grid origins, remainders and 360°/720° disclosure. The C++ owner accumulates
sample time into the declared half-degree-per-second driver with a remainder;
block partitioning cannot silently discard sub-step time. Only the native C API
advances or sets axes. No second Rust/JavaScript clock law is introduced.

A sample can remain fixed, attach to inscription, or attach to lensing. The
implemented target transform is a declared Z rotation; it is not falsely called
the complete origami/topological/toroidal mechanism. Full native pose/mesh
providers and the owner's retained topological implementation remain owners of
those geometries. Their samples and supported transforms must be bound without
resampling them into unrelated particles. Winding stays in the receipt even
though a displayed rotation alone does not distinguish whole turns.

Mute suppresses audio output without deleting modal state. Repeated target reads
are projections, not additional integration. Distinct subject instances have
independent local state over the same world clock. Controlled subject references
in these tests are **not** proof of #134's identity/activity/receiving composition.

## Installed management process

```sh
make -C cpp test install PREFIX="$PWD/target/k8-cpp"
target/k8-cpp/bin/ql-field-worker
```

`ql-field-worker` uses json-c for bounded newline-delimited management messages.
Each process owns one native instance. The host chooses the trusted executable,
authorises subject access and serializes its operations. Arbitrary submitted M2
output is not authenticated merely because it parses or has a known registry.
The existing Rust M2 request validator/producer remains the proper upstream.
The library has no JSON dependency; native audio hosts use the typed API directly.

`ql.field-control/v1` operations are `initialize`, `read`, `advance`, `set-axis`
and `replace-modes`. `initialize` takes the complete native M2 frame and a field
input with subject, sample rate, native clock, driver ratio, explicit units,
audio gains and identified supplied samples/shape functions. M2 must actually
contain a resonator. A missing resonator is refused rather than filled with a
palette, preset oscillator or invented physical interpretation.

`advance` additionally requires exact decimal-string `expected_generation` and
`expected_samples_elapsed`, a bounded frame count and Boolean mute. `set-axis`
uses the same expected state and an explicit axis plus lifted phase. A mode
replacement carries a complete newer native M2 frame with the same event,
geometry, material/model and mode identities. `read` never changes time.

Wire lifts, generations, rates and remainders use decimal strings where their
native integer range exceeds exact JavaScript numbers. Source sample IDs are
ordered, unique and at most 2^53−1. There is no modulo truncation of winding.
The output is `ql.continuous-field/v1`: exact event/subject/source/model refs,
current control revision/sample cursor, full native clock, actual modal
amplitudes, bounded audio block, stable target samples and retained M2 identity.
The full original/current M2 bodies remain attached to the host's event, not
replaced by this smaller transfer. Presentation scale is explicit.

A rejected request returns `ql.field-error/v1` and whether state committed.
Post-commit response failure must not be labelled an atomic refusal. A transport
loss has unknown operation standing; a host must not auto-retry an advance and
assume it did nothing. Preserve the last acknowledged receipt and original basis
for inspection/replay, stop live claims, and reconcile through an explicit new
instance or supported host recovery. No source or private journal is rewritten.

## Evidence and scope

`cpp/tests/continuous_field.cpp` executes the actual native clock and tests
analytic 48/96 kHz agreement, exact equal-mode cancellation, zero-rate limit,
block-partition invariance, stable samples, repeated views, independent axes,
mode continuation, mute, input budgets, refusal and original replay. Installed
headers/archive and a sanitizer build are separately executed by CI.

`scripts/test-k8-continuous.py` calculates a real current sky, runs the full
accepted native M2, obtains all eight Vimarśā frequencies from its actual output,
supplies an explicitly controlled signed square basis and returns that resonator
through the same complete M2 producer before invoking C++. It tests mode update,
independent clock operations, shared-view reads, mute, separate controlled
subjects, invalid/stale refusal and exact replay of the full operation stream.
The supplied square functions do not assert that the eight symbolic frequencies
are measured eigenvalues of that geometry. Every full input/event remains in
the evidence. This is numerical/native integration acceptance, not Personal,
acoustic-device, retained-GPU or owner-desktop acceptance.

The remaining K8 obligations include full shared root/family/property promotion
with AW1, graph application, full M1/M3 coupled event embodiment, actual retained
renderer transport and the independent #134 Nara producer. #133 owns the focused
UX/disclosure path. These scopes must not be silently certified by a passing
modal test. Local suite/desktop harmonisation remains a later stage.

## Management acknowledgement admission (K8 continuation)

The Rust session validates the complete returned identity, stable sample basis
and operation cursor before publishing it. Invalid/lost acknowledgements retain
the last known basis with unavailable/unknown standing, never an automatic retry
or silent new owner. See [acknowledgement admission](K8-ACKNOWLEDGEMENT-CONTRACT.md)
for exact refusal/re-entry semantics, controlled fault tests and the shared AW94
profile registration. The numerical and retained-GPU owners above are unchanged.

## The Torus's M1′ session, tuning and Fundamental (O-I #850)

The scene owner (`continuous/scene_field.rs`) holds the Torus's M1′ session
(`m1_prime.rs`, `ql.m1-prime-session/v1`): the rate of the one pulse (Free, one
step a year to 40,000 steps a second; Sync, one step per note length of 1/64 to
8 bars at the set's tempo) with its band beside it (Cosmic < 0.001, Breath to
0.5, Rhythm to 20, Pitch to 20,000, Beyond), the Fundamental, the Carrier, the
Shape, the Tuning, Polyphony 1–32 and the comma. The pulse runs on the owner's
own sample clock and keeps its phase across any change of rate. The comma is one
Pythagorean comma (531441:524288, six of QL's 9:8 epogdoons over the octave,
`m1::ratio_basis`) per full return of twelve steps; it starts from nothing when
the owner opens and grows only as its samples run.

Every tuning group is resident in the one modal basis from open, since the
native basis is fixed at open: the sky's nine planets (`scene:planet/…`, the
sky bus), Vimarśā's octet (`scene:logic/0..8`, `m2.vimarsha.reading.audio_octet_hz`)
and, where the event's own M2 condition admits its eight pitches (a v2/v3 event
whose tuning the map supplies pitches for), the maqām (`scene:maqam/0..8`,
`m2.condition.musical.pitches_hz`). The Tuning chooses the group a strike
reaches; the others rest and keep any tail. An event excites only the group in
force. The four boundaries (`nodal_quartet`) reach the body as the voices'
nodal numbers and are disclosed as `boundaries`, never as voices.

The Fundamental (`CoupledInput.fundamental_hz`, 20–2,000 Hz, absent = the map's
1:1, `CoupledInput::map_hz`) scales every bound voice of every bus by one ratio,
so each voice keeps its ratio over it. Absent, the scale is exactly 1 and every
earlier input composes bit-for-bit as before.

| Host operation | Native effect | Generation |
|---|---|---|
| `torus-state {}` | none: the session reading as `torus` | unchanged |
| `torus-set {edit}` | Rate, Carrier, Shape, Tuning, Polyphony, tempo: none. Fundamental: one modes continuation of the same resident voices, no strike, no reshape | unchanged, or +1 for a Fundamental change |
| `torus-play {keys, amplitude, at_samples_elapsed?}` | each key's moving point at the pulse (`janko::JankoSurface::point`) struck on the tuning in force: one `strike-at` | +1 |
| `torus-release {keys}` | none: the body rings on | unchanged |

Every answer carries `torus`; `torus-set` adds `influence` (its `tuning`,
sounding `voices`, `boundaries`, `resident_tunings`, `fundamental_hz`);
`torus-play` adds `torus_play` (`ql.torus-play/v1`: each note's place, point and
own cell) and `strike_at` (`ql.strike-at/v1`). Not built, and disclosed as such
in every reading: the Carrier's drive of the body between strikes (the value is
held and read back with `drive: "unbuilt"`) and a per-note voice bank sounding
each note's own cell (QL-MEF #287 packet A). Tests:
`crates/ql-mef/tests/{m1_prime_session,scene_tuning,torus_host}.rs`.
