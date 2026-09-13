# K8 native resource measurement

This is the bounded native part of B07/B10/B12 under
[Optimisation and Learning](OPTIMISATION-AND-LEARNING.md), not a substitute for
composed desktop, model/provider or human acceptance. Run the existing complete
v2 musical/dated-sky producer first, then:

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

No GPU/VRAM, browser audio-device scheduling, Epii/model work, remote delivery,
whole Bimba/Epii/background-task composition, cancellation-to-effect or human
sensory judgement is measured here. Those remain explicit joined obligations.
In particular, the browser's existing suspended-prefill synchronisation specimen
is not sustained unsuspended source-change throughput. A short bounded native
run is not hours of owner-machine experience and cannot close K8 by itself.
