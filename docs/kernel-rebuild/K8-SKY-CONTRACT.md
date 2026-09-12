# K8 dated sky → native M2 contract

Owner: #132 K8.1. Source authority: PRE-K8-AGENT-WORLD-LOCK, living-instrument
architecture §§2–3 and the accepted M2 producer. This is an implemented provider
boundary, not completion of Personal reception, continuous materialisation or
all K8.0–K8.3 acceptance.

## Input and actual calculation

`providers/sky/kerykeion_snapshot.py` accepts `ql.sky-request/v1` and produces
`ql.sky-snapshot/v1`. Install the exact versions in its `requirements.txt` in a
separate worker environment. Calls are offline; coordinates never go to GeoNames.
Swiss configuration is process-global, so this adapter serializes its calls;
an embedding host with other Swiss users must use a dedicated worker process.
No provider or subprocess call belongs in an audio/render callback.

```json
{"schema":"ql.sky-request/v1","epoch":"2026-09-12T12:00:00Z","timezone":"UTC","mode":"historical","perspective":"Apparent Geocentric","zodiac":"Tropical","ayanamsha":null,"observer":null,"max_age_seconds":60,"backend_policy":"allow-moshier"}
```

An offset-bearing, whole-second timestamp and matching IANA timezone are required.
The adapter accepts selected epochs in `[1800-01-01,2400-01-01)`. This is an adapter
range, not a claim of independently surveyed precision for every instant. The
factory's UTC-as-UT argument is recorded explicitly; no IERS UT1 correction is
claimed. Astronomical epoch, provider receipt and eventual domain generation are
separate. A pre-1970 epoch remains signed, not wrapped into an unsigned timestamp.

`current` additionally requires epoch ≤ receipt and age within the stated budget.
A later current consumer checks age again. `historical` means a selected dated
calculation, including a future selected ephemeris; it cannot be presented as
fresh current sky. Fixtures and extrapolation never receive a current label.

Supported perspectives are apparent geocentric, true geocentric and topocentric.
Tropical has no ayanamsha; the implemented sidereal policy is explicitly Lahiri.
Houses are not emitted. Kerykeion's auxiliary Whole-Sign houses are discarded,
not treated as public Nara natal data. A later house-bearing producer needs its
own explicit contract. Topocentric input requires a bounded observer reference,
longitude, latitude and altitude; the adapter rejects latitude beyond ±66° rather
than accepting the factory's silent polar adjustment.

## What a snapshot says—and does not say

All ten bodies have explicit name, native M2 planet identity, Swiss identity,
longitude/latitude, distance, full three-component speed and retrograde sign.
Positions are ecliptic-of-date, degrees/AU; speeds are per day. Each body's
**returned** ephemeris flags determine the actual backend. The requested SWIEPH
flag is not evidence that Swiss planetary data files were used. `allow-moshier`
permits and records analytical fallback; `require-swiss-files` fails instead.
Used Swiss files, when available, need readable content hashes and a valid epoch
range. On the controlled development host, the planetary data files are absent
and actual calculations use Moshier; this is real computation, not JPL-file proof.

The receipt includes exact Kerykeion/PySwiss/engine versions, native extension,
factory and adapter hashes, Python/timezone-library versions, requested/returned
flags and used-file identities. An injected test clock is labelled separately
from the real host clock. The snapshot SHA-256 preserves immutable content; it
is **not authentication of an arbitrary submitted provider report**. A trusted
worker/native host owns origin verification and authority.

Shared geocentric snapshots have no observer, birth data, journal, identity hash,
bioquaternion, field image or audio. Topocentric output is `observer-private`.
`attach_m2`/`execute_m2` default to shared scope and refuse private data without an
explicit matching private consumer scope. That scope argument is not permission;
#134 and native host authority remain responsible for subject access.

## Native source and whole-producer join

The adapter verifies the retained native M2 header hash and all ten named enum
bindings. Sun is the parent at native 0; Moon through Pluto are nine non-Sun
operators. EarthBody is the separate grounding anchor `#2-5-0/1-0`, not a planet
array member or eighth peer chakra. Seven receiving chakras, the source 9:8
relation, and preempted transpersonal meanings remain explicit. No planet index
is repurposed as a receiving-centre identity. No private centre composition is
invented by this provider.

`attach_m2(snapshot, request)` adds the exact ten `world_observations` to an
existing complete `ql.m2-engine-request/v1`. It preserves the other fields,
including source-qualified condition, full 72 coefficients, M1 excitation,
Vimarśā's independent 8+4 input, selections, material and resonator modes.
The snapshot remains the astronomical source; M2 v1's unsigned observation time
is its receipt time, and the enclosing event explicitly retains this distinction.

`execute_m2(snapshot, request, trusted_executable)` invokes the existing compiled
`m2_engine` producer and returns `ql.sky-m2-event/v1`: original sky, full input,
full M2 frame and scope. The producer—not Python—calculates native planetary
Power, element-first paired decans, 45 aspects, source relations and distributed
64-amplitude form potential. The native frame's qualified standing is preserved.
The JSON boundary enables `serde_json/float_roundtrip` so binary64 inputs survive
parsing and emission bit-for-bit; the default approximate parser can change a
longitude by one representable value. `k8_sky_numeric` tests exact bits against
the native standard parser, including the real historical provider inputs.
This changes serialization precision, not any M1–M3 numerical law.
A failure emits no partial event. The executable is a host-selected installation,
not a shell command supplied by an untrusted source.

```sh
cargo build --locked -p ql-mef --example m2_engine
python providers/sky/kerykeion_snapshot.py sky-request.json \
  --m2-request complete-m2-request.json \
  --m2-executable target/debug/examples/m2_engine
```

The M2 request's occurrence/receipt timestamp must accommodate the sky receipt;
this command does not silently rewrite an existing occasion to make it fit.

## Executed acceptance and remaining joins

`test_kerykeion_snapshot.py` exercises actual current/historical calculation,
J2000 and adapter endpoints, pre-1970 dates, retrograde, offset/DST ambiguity,
sidereal/true/topocentric policies, independent concurrent configurations,
fallback refusal, freshness, loss/missing body, invalid payload and replay.
Negative-provider cases use explicit fault injection; positive cases call the
installed provider. Replay verifies original bytes without calling a provider.

`scripts/test-k8-sky.py` drives actual current sky and three dated skies through
the compiled native M2 example. It retains all ten planetary and both decan
readings, 45 aspects, 72 coefficients, distributed 64 output, original replay and
a separate later generation. Excitation for this acceptance is the retained
controlled M2 fixture, not a live M1/Nara/audio source. CI archives exact source,
provider lock, four complete events and executable hash; a reusable installed
native consumer is retained separately. These are bounded A04/A05/A15 producer
proofs, not human, GPU, acoustic or whole-Personal acceptance.

The shared root/family/property integration and graph migration remain K8.0/
AW1 coordination. #134 owns identity, centres, natal/activity and source admission.
K8's coupled Rust/C++ continuous receiver still has to bind the whole event;
#133 consumes the same event for disclosure. No duplicate registry, private
Control mutation, parallel Epii loop or replacement renderer is introduced here.
