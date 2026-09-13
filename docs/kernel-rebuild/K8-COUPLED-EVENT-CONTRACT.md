# K8 coupled event v1

Standing: executable producer and installed acceptance consumer under #132 / #169.
Acceptance attaches to exact source/test receipts, not this declaration. Read
[K8 continuous contract](K8-CONTINUOUS-CONTRACT.md), the
[M1 contract](../KERNEL-M1-ENGINE-CONTRACT.md), [M2 contract](m2-engine-v1.md),
[M3 contract](../KERNEL-M3-ENGINE-CONTRACT.md) and
[lens identity](LENS-IDENTITY-AND-ENCODINGS.md) together.

## Producer and native continuation

`ql_mef::continuous::coupled::CoupledInput::compose` accepts
`ql.coupled-event-request/v1`. Its full original M1 `EngineConfig`, M2
`M2Request`, M3 `M3Request` and bounded M3 command history are retained beside
all three complete native outputs. It rejects mismatched events and invalid
native input before producing a new basis. Native provisional M3 refusals
remain qualified command receipts; they do not acquire invented geometry.

The harmonic selector chooses an admitted M1 source row or one of the eight
existing source-derived ratio bases. An unsupported row has no unity fallback.
Native lens identity travels through `SublensRef` and
`Reading72::from_sublens`; M1's interleaved slot 1 remains L0', not M2 row 1.
The existing Context Frame / musical-mode relation and actual lawful M3 pose
feed the existing M2 Vimarsha owner. Its eight frequencies can be deliberately
bound by existing mode ID and octet index to supplied material modes. Unbound
modes keep their supplied frequency. Every original M2 observation, material
parameter, modal coefficient, selection and condition stays available.

This is a declared musical instrument mapping, **not** evidence that symbolic
frequencies are measured material eigenvalues. The existing physical supplier
still owns geometry, mode shapes and material/model provenance. The composition
does not add a topology, fold, colour, Nara or ephemeris engine.

`CoupledFieldSession` wraps the existing `FieldSession`, which owns one installed
C++ worker. A full replacement is composed and validated first. It is published
only after that same worker acknowledges its mode replacement; resident modal
amplitudes, constituent/sample identities and coupled-clock phase are preserved.
A stale/invalid request leaves the previous acknowledged whole intact. A lost
reply retains the last basis with unavailable/unknown continuation standing;
there is no blind retry. `read` and `snapshot` do not integrate another step.
The session retains both its original full basis and original geometry/clock
input. Replay reproduces the original operations, not a later reinterpretation.

`ql.coupled-event/v1` contains the full current `basis`, the unchanged
`ql.continuous-field/v1` acknowledgement under `field`, and availability. A GPU
receiver consumes `field`; source/inspection companions can read the same
whole basis without rebuilding symbolic state from particle coordinates.

## Time and attribution

M1 source revision/logical clock, M3 operation generation/clock, M2 composition
generation, astronomical epoch/observation receipt, native sample cursor and
presentation frames are related but not interchangeable. The derived M2 input
is stamped as this composition; original source versions remain in `input` and
native outputs. Full host-admitted source receipts, including dated sky, can be
retained without copying them into an audio callback. Retention does not
authenticate a provider, authorise private disclosure, or grant an operation.
The native host continues to own executable and subject authority.

## Runnable acceptance

```sh
make -C cpp test install PREFIX="$PWD/target/k8-cpp"
cargo build --locked -p ql-mef --example m2_engine
python scripts/test-k8-continuous.py target/k8-cpp/bin/ql-field-worker
cargo test --locked -p ql-mef --test k8_coupled
cargo run --locked -p ql-mef --example k8_coupled -- \
  target/k8-cpp/bin/ql-field-worker target/k8-continuous/basis.json \
  target/k8-continuous/initial.json target/k8-coupled
```

The tests exercise every lawful M3 pose, all twelve MEF identities, seven
Context Frames and eight native harmonic bases; retain full native outputs;
and refuse invalid/cross-event input. The installed consumer uses the actual
dated provider output, applies real M1 harmonic and M3 matrix/pose/RNA/clock
operations, tests atomic adoption and no reseed, and repeats the complete
operation trace. The continuous workflow then feeds this very trace to the
retained Point-Cloud GPGPU implementation, rather than an unrelated animation.
Inputs, original/changed/last whole events, field frames and acceptance are
published under `target/k8-coupled` in the exact-head native evidence artifact.

## Remaining distinct proof

The installed specimen uses explicitly controlled source-linked geometry and
subject handles. It does not claim #134's full Nara constitution or independently
active receiving centres, AW2 performance-event reception, a universal physical
material/fold solver, acoustic-device equivalence or owner-machine desktop
acceptance. Full source matrices and A/B obligations remain the scope of #132
and its parallel owners. Root/property/graph promotion remains K8's shared-core
integration responsibility; this operational join neither duplicates those
registries nor makes their unlanded work disappear. The newer owner-built
renderer is retained for later exact-version local harmonisation.
