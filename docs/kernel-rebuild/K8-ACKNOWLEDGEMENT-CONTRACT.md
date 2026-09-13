# K8 management acknowledgement admission

Scope: K8.2–K8.3 / #132; A03, A07 and A16. The installed C++ owner remains
`ql.continuous-field/v1`. This is its Rust management boundary, not a new
simulation, symbolic table, agent identity or numerical clock implementation.

## What changes for the host

`FieldSession::open`, `read`, `advance`, `set_axis` and `replace_modes` keep their
existing signatures. A reply is now admitted against the actual operation,
retained subject/event/source/material, complete M2 identity, stable ordered
constituents, dimensions, numerical bounds and exact control/physical cursors.
Recognising a schema string alone does not establish any of those relations.

Initialization must acknowledge the supplied clock, phase/winding, model and
sample basis at zero elapsed samples. Read and zero-advance cannot evolve or
reset resident state. An independent axis acknowledgement preserves the other
axis and modal amplitudes. Replacement must acknowledge a newer M2 generation
on the same material/mode identities and retain resident amplitudes/targets
unless the caller explicitly requested state replacement. Source-profile,
field-control, physical-sample and clock generations are not interchangeable.

Clock origins, rates, centre, exact lifted phase encoding and cover consistency
are checked. Native C still owns trajectory calculation; this validator is not
a parallel numerical oracle. Finite, dimensioned audio/target/amplitude output
is checked, including silent output during mute. A complete valid-looking reply
is not cryptographic authentication or empirical validation of the supplied
material model. The native host still chooses and authorises the executable.

## Refusal is different from an uncertain outcome

An explicit `ql.field-error/v1` reply is recoverable only with its complete
three-field envelope, textual error and `state_committed: false`. A malformed,
foreign, inconsistent, lost, oversized, timed-out or post-commit reply poisons
the connection and terminates its worker. No subsequent operation is sent to
that connection and no unknown operation is automatically retried.

`last_receipt()` remains the last **acknowledged** state. `original_basis()` and
`current_basis()` do not advance on a failed acknowledgement. `available()` is
false: the old receipt must not be labelled current/live merely because it
remains inspectable. CoupledFieldSession inherits this boundary, so a lost
material adoption does not publish a half-updated M1/M2/M3 whole.

Re-entry is explicit. A caller can open a new owner with retained original
inputs and replay its known commands, then compare receipts. The receipt alone
is not a C++ checkpoint and does not authorise blindly repeating an uncertain
external action. GPU restoration remains the distinct retained position/velocity
checkpoint operation in the [continuous contract](K8-CONTINUOUS-CONTRACT.md).
Companion views read this owner rather than spawning another clock.

Native management integers now require canonical decimal strings: `01`, `00`,
`-0`, whitespace, exponent notation and overflow are refused, not silently
normalised. This removes aliases between native control and the retained GPU
receiver's already-exact cursor grammar. Typed C/C++ APIs are unchanged.

## Shared AW registration

K8 integrates the unchanged `vak_profile.rs` and its tests authored in #166 at
`6a35d04caed98c61a1df903020fa05860790fad7`. The actual current registry resolves
`#0-4` to `7e282e4c1833faea` and `#0-5` to `c2424584fbe5268e`; those are the
reviewed cross-coordinate bindings for this bounded original C′ profile over
`VakComposition`. Helpers/tests receive explicit infrastructural dispositions.
The original module and test bytes remain AW-owned, not a K8 reinterpretation.

The current census is regenerated separately. Historical K4 workbooks, source
assessments and ledger bytes are not rewritten. Profile compilation and native
thread obligations do not prove AW0 full coverage, AW1 graph/property execution,
AW2 actual performance, AW3 Return, or #134's protected Nara reception. Those
lanes continue against the now-available shared contracts.

## Evidence and boundaries

The executable tests are `continuous/receipt/tests.rs`, the actual installed
`k8_field` / `k8_coupled` consumers, `cpp/tests/continuous_field.cpp`, and
`scripts/test-k8-continuous.py`. Controlled subprocesses inject transport faults;
they are not substitutes for numerical or live-provider acceptance. The
continuous workflow retains their separate logs alongside real native/GPU
receipts and exact source/toolchain identity.

The initial implementation was checked locally against the exact main
`1dbd5150c0b264b2180bdf654cd944c59037ea24` source. Its original 36 native commands
replayed byte-equivalent JSON values after the change. A red/green native probe
showed the old worker advancing 32 samples for cursor strings `01`/`00`, while
the patched worker refused without mutation. At the supplied analytic case,
48 kHz and 96 kHz error was respectively `2.97497e-15` and `3.17736e-14`;
equal-mode cancellation, partition independence, mute continuation, independent
subjects and exact replay passed. These measurements qualify that controlled
model, not an empirical plate, audio device or owner GPU.

PR/issue receipts record the final tested head, workflow runs and artifacts.
Owner-machine/desktop and human-experience acceptance are later and separate.
The existing UX publication remains H-ratification-pending.
