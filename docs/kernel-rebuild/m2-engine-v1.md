# K6 — Paraśakti native engine and continuous-embodiment contract

**Owner:** M2 / Paraśakti. **Issue:** #130; implementation PR #148.
**Producer:** `ql.m2-engine/v1`; request `ql.m2-engine-request/v1`.
**Native ABI:** installed `<ql/m2.h>` and `libql-mef-c.a`.

This is the M2 producer, not an assertion that a bell, MEF, astrology or
`TemplateureField` is the subsystem. It preserves a differentiated Power field:
M2-0 numerical potentiation; M2-1 epistemic refraction and active Vimarśā;
M2-2 ontological/tattvic depth; M2-3 situated decanic faces; the asymmetric M2-4
symbolic-musical arena; and M2-5 planetary/chakral synthesis and the M3 threshold.
The source matrix remains authoritative about this Whole, including its M0-4,
M1, M3, M4, S2/AIKit, S2′/Pleroma and M2′ relations.

## Source and coordinate authority

The only coordinate tree is `fixtures/kernel/m-tree-v1.json`. The only readiness
ledger is `fixtures/kernel/m-ledger-v1.json`, using the unchanged K3 schema.
K6 adds namespaced `k6-m2:*` rows/bindings/evidence/discrepancies and relates the
existing 31 M2 source capabilities; it does not rewrite their source coordinates.

K4 is accepted through #153 at `5b24b95`; #154 at `9bf69a4` adds the
parent-surface producer requirement. This lane consumes all 597 K4 M2 rows,
source locks and live/serialized deltas. Its detailed M2 source observation
complements the shared census; neither replaces the other. The census is
presence evidence. Narrow executed operations have their own K6 records.

`scripts/m2-field-report.py` verifies the original pinned Bimba bytes at
`EpiLogos/Epi-Logos-C-Experiments@daa660cbc1b8c5da83828698665a753852cb0287`:
595 deep node records, 16 planet records and 4,017 deep relation records. The
source-preserving common registry has 597 M2 coordinates and 8,876 incident
relation records, including cross-subsystem relations and repeated source
assertions. These counts are different domains, not deduplicated synonyms.
The full per-coordinate property/pointer/binding census is an acceptance artifact
`target/m2-receipt/m2-field-census-v1.json`; its exact hash and source locks are
checked into `m2-field-census-summary-v1.json`. Source JSON remains byte-identical,
including the already accepted BOM/control-character parse policy.

## What is native

The independent frozen-C probe exports all 764 numeric descriptors in sixteen
retained tables: carrier, MEF, tattva, decan, planet, chakra, Shem, ratios, musical
maqām, spiritual stations, Asmāʾ, mantra, element, DET, resonance and routing.
The deterministic catalogue seats explicit associations into current K2 node
IDs. `coordinate_id == 0` / `exact_coordinate: null` means a branch-level law or
an unresolved binding, never a fabricated neighbouring coordinate. A table
association is **structural-index-only** in the ledger, not proof that the
coordinate's whole computational or semantic body is complete.

Native C and Rust separately execute four 72-register encodings, elementary
signatures, tattva traversal, Asmāʾ routing/digital-root operations, outer-planet
preemption, finite aspects, retained 24-TET playback, distinct DET transforms,
and bounded modal operations. Rust also joins the actual source relation field,
current MEF/Context Frames, paired decan readings and the versioned engine frame.
Every narrow operation is separately inventoried rather than promoting a broad
source capability because a lookup function exists.

### Active Vimarśā, not only lens lookup

`m2_vimarsha` ports the frozen portal-core M2-1 reader. Its original source is
retained byte-for-byte as `fixtures/kernel/m2-reference-vimarsha.rs` (Git blob
`7b0bc34cee3388a8d22a615ad2c9facc28016e30`). Native C and Rust match that executed
reader across 32,256 supplied-input cases. The original reader's M1 ratio and M3
pose dependencies are injected equally into the oracle and port: this is reader
formula parity, **not** proof of either neighbouring subsystem.

The engine consumes the M1 harmonic ratio under its event stamp and resolves an
actual pose through existing `ql_core::all_poses`. It emits eight frequencies in
Hz and four typed nodal constraints (position, helix, m, n). Its retained
experimental timbre policy is named, not upgraded into a universal acoustic law.
The historical 84↔472 selection policy is not reintroduced as a competing core.
The seven **musical modes**, seven **Context Frames**, and six internal MEF
positions are different structures and stay different.

## Operational distinctions that must survive embodiment

C stores six direct lenses then six prime lenses. Rust's `LensId::ALL` interleaves
each lens with its prime. The explicit bridge preserves all 72 conditions and
all **seven** canonical Context Frames; it does not invent a new twelve-node tree.

The 72 carrier supports MEF 12×6, tattva 36×2, decan 4×3×3×2 and Shem 8×9 readings.
Reinterpreting a carrier index is not evidence that their semantic values agree.
The five-element index, decanic Fire/Earth/Air/Water order, and material
Earth/Fire/Water/Air (EFWA) order are likewise explicit separate encodings.
Decan-to-element links resolve through the retained element throughline, not by
shared numeric IDs; the quintessence decan links to Akasha without a fake planet.

There are three distinct 72→64 operations:

* Historical scalar addressing uses floor(8i/9); its floor(9j/8) expansion is not
  an inverse or an amplitude transduction.
* The retained OR-mask DET preserves its original bitmask lookup, separately named.
* The current material law is `I4 ⊗ T18→16`, using the existing authoritative Rust
  transducer: within each fibre, 16 folds to 0 and **17 folds to 8**. Amplitudes
  add coherently; the fibre does not change. This is not index modulo 16.

`Q` rotates exact complex coefficients, `Q⁴ = I`, and preserves modal power.
Coherent folding generally does **not** preserve total power. Kernel components
are bounded to ±1,000,000; malformed/overflow-prone inputs fail before arithmetic.
Native C failures leave caller outputs unchanged. The old unchecked Templateure
API is not silently redefined; the M2 producer wraps it with checked bounds.

## Wire contract and runnable consumer

The actual Rust types in `m2_engine.rs`, `m2.rs` and `m2_vimarsha.rs` define the
wire surface. `scripts/m2-contract.py` emits the closed request/frame JSON Schemas
and checks them against a real emitted frame. Native validation additionally
checks event identity, chronology, exact registry membership, unique selections
and elemental locality; a JSON Schema pass alone is not operational readiness.

```sh
cargo run -p ql-mef --example m2_engine --locked -- --fixture
cargo run -p ql-mef --example m2_engine --locked -- /path/to/request.json
```

The request contains one event/generation, registry revision, tick12/degree720,
72 complex integer coefficients, MEF/Context-Frame/descriptor selections, and
optional stamped M1 excitation, active Vimarśā inputs, supplied world observations
and resonator state. The frame retains these stamps, exact coordinate/ledger
revisions, source locks, all sixteen domain holdings, selected/linked descriptors,
72 modal coefficients and quadrature, 64 form-potential amplitudes, and explicit
provider standing. It never labels supplied observations authenticated or live.

Descriptor bitmasks and other u64 values serialize as **canonical decimal strings**;
coordinate IDs remain the common fixed-width hexadecimal strings. Power is also
a decimal string. Event generations and millisecond timestamps are limited to
2⁵³−1 so JavaScript cannot silently round them. Request JSON is bounded to 32 MiB.

## K8/K9 modal/material handoff

The continuous provider supplies its own geometry, material and constitutive-model
references, sourced physical parameters with units, and **1..4096 actual modes**.
Each mode has a stable identity, actual M2 source coordinate, material fibre,
1..18 explicit same-fibre carrier weights, frequency, complex amplitude and
excitation, damping per second, and nodal/antinodal state references. The number
of physical eigenmodes is not assumed to be 72. A consumer cannot substitute a
physical mode index for an M2 coordinate or silently move weight between elements.

`M1Excitation` is an explicitly supplied adapter payload with a source contract
reference, not a claim to reproduce the full forthcoming K5 ABI. K8 joins that
final producer, the shared M3 form/pose authority and this M2 frame under one
event. K8/K9 remain responsible for physical eigenstructure, integration/renderer
behaviour, and experiential validation. Missing resonator input is
`unavailable-no-physical-solver-implied`, not a fabricated working resonator.
No private Control, live Neo4j, ephemeris, personal chart or physical device is
read or modified by this producer or its fixture tests.

## Explicit unresolved source readings

The common ledger retains separate discrepancies, not a blanket parity flag:

| Field | Preserved difference / disposition |
|---|---|
| Tattva | Vidyā and Ear have no exact current K2 bindings; their retained records remain available. |
| Planet | C mod-10 IDs differ from Bimba suffixes; bind by unique name. Uranus is unbound; Earth is separate ground. |
| Asmāʾ | 122 matched-name field disagreements, 14 stored/arithmetic digital-root disagreements, and 34 unmatched names. Hidden 99 keeps its sentinel. |
| Spiritual stations | C's Shukr/Khawf/Raja differ from Bimba's Wara/Zuhd/Faqr. Name-based binding preserves 15 levels and leaves 9 unresolved. |
| Mantra | 50 Mātṛkā leaves bind; 50 further retained records stay branch-held without inventing Mālinī leaves or phonemic permutation parity. |
| Musical maqām | All 72 Bimba literal interval declarations remain beside the retained C patterns in the full census; authentic tuning equivalence is not asserted. |
| Shem | Structural 8×9 association does not resolve differing choir labels. |
| Resonance | Estimated historical same-position masks are not the actual graph; the latter has 672 retained `CAUSAL_RESONANCE` records. |

These are usable, separately addressed readings, not permission to discard a
source or an invitation to overwrite Bimba with the strongest convenient table.
Broad capabilities and external embodiments stay partial/planned where the
narrow finite engine does not establish them.

## Acceptance and regeneration

```sh
M2_BIMBA_SOURCE=/read-only/pinned/source bash scripts/test-m2-engine.sh
python3 scripts/m2-ledger.py refresh   # only after actual acceptance
python3 scripts/m-ledger.py check
python3 scripts/m2-ledger.py check
cargo test --workspace --all-targets --locked
```

The acceptance executes the independent frozen C oracle, native C and Rust
observers, original Vimarśā reader, all-source census, failure cases, ASan/UBSan,
closed wire shapes and a C++17 consumer using **installed** headers/static library.
`m2-finite-proof-v1.json` locks the executed working-tree inputs; CI supplies the
exact Git SHA in `target/m2-receipt/acceptance.json`. Neither an index-only seed nor
an earlier workflow run stands in for executed implementation evidence.


## Joint musical–colour–material condition

`M2Request.condition` selects a maqam, tonic/dominant relation, active MEF
condition, named tuning policy and tonic in Hz. It requires the existing stamped
Vimarśā drive; the lens and local position must agree with that MEF selection.
The optional `M2Frame.condition` is `ql.m2-condition/v1`. Older requests remain
valid with no joint condition.

Run the committed full-chain request through the same producer:

```sh
cargo run -p ql-mef --example m2_engine --locked -- \
  fixtures/kernel/m2-condition-request-v1.json
```

The source compiler `scripts/m2-correspondences.py` follows exact K2 relation IDs:
`TONIC_PLANETARY_RESONANCE` or `DOMINANT_PLANETARY_RESONANCE` from the musical
maqam to a planetary coordinate, then `PLANETARY_RESONANCE` to its chakral
reception. The compiled C table and Rust field share node IDs, relation IDs,
source revision and elemental/material holding. Rust also retains every repeated
source assertion, source-file digest, record/payload digest and four exact
property pointers. The underlying Bimba files and shared coordinate registry
remain unchanged.

This source cut supplies 127 unique role paths; 17 selections have no unique
source-held path. The colour admission uses the literal yantra declarations:
root's **yellow square**, sacral's **silver crescent moon**, navel's **red
triangle**. Other centres have no admitted explicit colour name in this cut.
This is not a conventional chakra-rainbow palette. A named source colour is not
RGB: a separately stamped `RenderPalette` supplies an explicit linear-RGBA
conversion policy. Missing names, missing palette entries and unsupported paths
remain unavailable rather than acquiring a guessed colour.

There are two independent tuning policies:

- `retained24_tet` plays the retained C quarter-tone patterns and separately
  reports their retained ruler index.
- `bimba_spelled24_tet` interprets explicit eight-note source spelling under a
  named 24-TET policy. Flat, sharp, natural and half-sharp spelling are supported;
  ambiguous downward-arrow and non-note declarations remain unsupported. This
  admits 115 of the 127 paths. It does not claim authentic performance tuning or
  equality with the retained patterns, and never silently falls back to them.

The condition carries the active lens/sublens, eight musical pitches, existing
8+4 Vimarśā drive, actual planetary/chakral/tattva path, material fibre, source
colour and explicit palette policy. Supplied planetary observations retain their
provider, source revision and age; both decanic faces come from the existing
world reading. A supplied continuous material keeps its geometry, constitutive
model, unit-bearing parameters, mode weights, phase/amplitude/excitation,
damping and nodal/antinodal references. Neither missing world input nor missing
physical input is presented as an observation or working solver.

Its material is a supplied embodiment of the same coordinate-backed event,
not a physical property derived from the word yellow. The full-chain test uses
an explicitly controlled Earth mode at the very tattva coordinate on the selected
source path, with the same event/generation as the palette, harmonic drive and
world reading. Conjugate MEF selection and supported tuning changes update their
respective musical outputs while retaining the source path. Cross-generation
palette/material inputs fail before production.

The three M3 transformations remain separately identified. Scalar and retained
mask outputs explicitly use the selected MEF carrier index; the distributed
form-potential reference points to the complete engine modal amplitudes. None
substitutes for the shared M3 clock/pose or a future continuous runtime.

### Executed producer checks and regeneration

The mandatory M2 suite independently executes 127 C/Rust path descriptors and
6,912 tuning/role/degree/tonic cases, then compares sanitizer output and exercises
the installed C++ header/static library. Rust full-chain fixtures cover source
path, joint generation, supplied material, planetary/decan reception, palette,
conjugate and tuning changes, and explicit unavailable behaviour. This is
producer acceptance, not acoustic/visual or experiential acceptance.

```sh
python3 scripts/m2-correspondences.py refresh --source-root target/m2-bimba-source
python3 scripts/m2-contract.py refresh
bash scripts/test-m2-engine.sh
python3 scripts/m2-ledger.py refresh
python3 scripts/m_census.py census
python3 scripts/m2-ledger.py check
python3 scripts/m-ledger.py check
```

Refresh joins `M2…` to `#2…` using the registry's sanctioned root alias. The shared
census determines a binding's stratum from its implementation inventory, never
its ID prefix; thus K5/K6/K7 namespaced bindings are not downgraded by discovery.
Refresh preserves K4 rows, foreign reviewed assessments and recorded discrepancy
proposals/decisions. Regeneration and the shared Rust/Python mutation tests must
pass together. The accepted K4 capture remains the original observation, while
regenerated implementation workbooks describe the changed tree.
