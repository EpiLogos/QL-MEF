# K8 native resource measurement

This is the bounded K8 part of B07/B10/B12 under
[Optimisation and Learning](OPTIMISATION-AND-LEARNING.md), not a substitute for
composed desktop, model/provider, protected Nara or human acceptance. Run the
existing complete v2 musical/dated-sky producer first, then:

```sh
python -m unittest discover -s scripts/tests -p test_k8_resource.py
K8_COUPLED_DIR="$PWD/target/k8-coupled-v2" python scripts/test-k8-resource.py
```

The continuous workflow runs the same commands against its installed
`ql-field-host` and C++ worker. It retains conditions, operation journals and
`ql.k8-native-resource-acceptance/v1` under `target/k8-resource`. Conditions name
the exact source/tree, input and executable digests, toolchain/platform/CPU,
mode/sample counts and the policy **before** observations. Geometry and subjects
are explicitly controlled; dated sky and full M1/M2/M3 bases are real producer
outputs. This does not invent an AW performance or Nara producer.

## What is exercised

Three independently opened and disposed owners each run eight warm-up blocks
then 128 measured 4,096-frame blocks. Each has exactly one numerical child.
Mute/unmute, native read, full source inspection and a real M1/M2/M3 basis
replacement are interleaved. Original amplitudes, target identities, clock and
sample cursor survive replacement. Read and deliberate idle periods do not
advance time or spawn work. Full original/current source bases remain inspectable.
Every operation and canonical acknowledgement digest is streamed to a journal;
a separately opened original replay must reproduce every acknowledgement.

The harness measures native host/pipe round-trip latency, nearest-rank p50/p95/p99
and maximum, host and child RSS, file descriptors, idle CPU, source-operation
latency and peak response bytes. Wall duration includes the measuring consumer;
its observation/validation overhead is reported separately from command time.
The measuring consumer clears the old driver's in-memory trace each operation
and retains only bounded samples plus the streamed journal. It neither trims
slow commands nor suppresses an input/source/Return to improve a score.

## Standing and ceilings

Memory limits are 512 MiB for the host, 256 MiB for the child, and 32 MiB measured
within-run RSS growth. File-descriptor growth is zero. A stopped native owner has
at most 0.05 CPU seconds during the declared 0.2-second idle probe. Child disposal,
source/clock/identity preservation, bounded output and exact original replay are
mandatory assertions. These conservative diagnostic bounds are not optimal
product budgets or claims of a long-duration leak proof.

`measurement_valid`, `resource_pass`, `native_deadline_pass` and
`source_operation_budget_pass` are separate results. The native deadline is one
block duration (about 85.3 ms at 48 kHz); **every** miss counts. Source-operation
budget is 400 ms. A successful measurement job must not be reported as meeting
these timing budgets when either timing flag is false. Preserve the actual
failure count as an optimisation target; do not increase a budget post hoc to
make its original receipt green. Unit tests specifically reject empty/invalid
observations and distinguish a slow or growing run from a pass.

## Unsuspended browser / audio / retained-field receipt

The same continuous workflow separately executes
`adapters/retained-field/test-instrument-browser.py` against the installed host,
worker and adapters plus the actual retained Point-Cloud GPGPU implementation.
The earlier exact sound/field/recovery proof remains, but the browser acceptance
now also drives the existing `InstrumentSession.start()` data plane while the
WebAudio context is **running** and 65,536 retained particles are being integrated.
This is measurement around the existing owner and scheduler, not another
simulation loop or audio generator.

A controlled live window spans 72 animation frames and at least four real
4,096-sample native advances. It records frame p50/p95/p99/max, queue high-water
marks, acknowledged-to-presented generation age, native scheduling slack and
attributable native-request-to-coherent-field-application time. NativeAudio's
existing hard admission rejects a block that reaches WebAudio late; any resulting
hold fails the live acceptance. The controlled stale-display ceiling is 0.5 s and
the catastrophic presentation-stall ceiling is 500 ms. The declared queue
ceilings remain eight blocks / 8 MiB. These fixed values are CI diagnostics, not
post-hoc desktop budgets.

The live window then performs an explicit hold and current-native read recovery.
It must return with zero presentation backlog, unchanged owner identity and no
particle reseed. Coalesced presentation frames remain explicit; PCM/native
commands are not silently discarded. The result is retained inside
`ql.k8-managed-browser-acceptance/v1` as `sustained_live_measurement`, alongside
the exact native request/response trace and original replay proof.

This controlled Chromium/SwiftShader receipt measures browser scheduling and
retained-field continuity but not owner GPU/VRAM, physical speaker latency,
long-duration desktop thermal behaviour, Epii/model work, remote delivery,
protected Nara reception, whole Bimba/Epii/background-task composition or human
sensory judgement. The native resource receipt supplies RSS/CPU/file-descriptor
measurements; it does not relabel those as browser GPU observations. Those wider
relations remain joined B07/B10/B12 obligations with their actual owners.

## Build-profile comparison without changing the instrument

The continuous workflow retains the same-source debug binary as the diagnostic
baseline and installs the release binary as the native instrument host. It runs
the declared resource protocol for both. The release run replays every original
operation through the **debug binary**, not merely a second release process, and
requires identical canonical acknowledgement digests, including full original
and changed source inspection. C++ worker, dated inputs, source/tree, parameters,
policy, sample identities and all musical modes remain identical.

`compare-k8-resource.py` refuses comparisons with changed controls, omitted
cycles, invalid observations or missing cross-build replay. It records individual
cycle measurements and median ratios for block p95, source-operation maximum,
RSS and deadline misses. A lower median does not erase a missed fixed deadline.
`comparison.json` explicitly distinguishes a complete comparison from meeting
both unchanged runtime budgets. Sequential same-runner observations are not a
randomised causal study; no universal speed-up, GPU or provider cost is inferred.

The workflow's longer execution allowance covers two separately built profiles,
two complete protocols and cross-build replay; runtime policy ceilings are not
raised. `K8_RESOURCE_HOST`, `K8_RESOURCE_PROFILE`, `K8_RESOURCE_REPLAY_HOST` and
`K8_RESOURCE_OUT_DIR` name these explicit measurement inputs. Omitting a profile
records `unspecified`; it does not infer optimisation from an executable name.
