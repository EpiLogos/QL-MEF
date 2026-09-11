# K6 consumer handoff for the two-parent / six-depth integration

This is the implementation cross-reference for
[PARENT-SURFACES-INTEGRATION.md](PARENT-SURFACES-INTEGRATION.md) §4 and
QL-MEF #130/#132/#133/#134. It does not replace that plan or introduce a new
stage, coordinate registry, renderer or agent runtime.

The K6 implementation was accepted through #148 (`c421080`), followed by the
installed-C++ consumer test in #155 (`7dda4d8`). Its public producer is
`ql_mef::m2_engine::M2Request::{from_json, execute}`. The contract remains
[`ql.m2-engine/v1`](m2-engine-v1.md), with optional `ql.m2-condition/v1`.

## What the added requirements consume

| Requirement from the integration plan | Existing producer surface | Consumer responsibility |
|---|---|---|
| One M2 state for Cosmic, Personal and M2′ | The enclosing `M2Frame`, its `identity`, `at_unix_ms`, registry/ledger revisions and optional `condition` | K8/K9/K10 keep the full frame and its generation together; do not construct separate per-view musical/colour states. |
| Actual musical → planetary → chakral path | `condition.source_path`: maqam/planet/chakra/tattva coordinates and node IDs, both relation groups, literal properties, record pointers and digests | Bimba map/M0′ opens these exact objects. Epii and the person inspect the same path; the renderer does not own another correspondence table. |
| MEF and differentiated musical operation | `active_mef_condition`, `sublens_ref`, `musical`, `drive`, plus enclosing `vimarsha` | Preserve lens/local position and named tuning policy. The source-derived musical pitches and Vimarśā 8+4 reading are distinct inputs to K8's acoustic composition, not automatically identical buses. |
| Named elemental and colour/material output | `source_path.material_fibre`, `element_literal`, `colour.source_name`, explicit stamped palette and `physical_material` | Keep source correspondence, palette conversion and supplied physical state distinguishable. Missing colour is not permission to invent a rainbow mapping. |
| Modal response | Enclosing `modal`; supplied resonator modes with carrier weights, amplitude, excitation, damping and nodal/antinodal refs; unit-bearing material parameters | K8 supplies physical integration and supported constitutive/response parameters. Elasticity, particle affinity or envelope controls must retain their actual provider/policy provenance rather than being inferred from a colour name. |
| Situated world and decanic condition | Enclosing `world`/`aspects`; the selected condition's supplied planetary observation, source revision and age | The host supplies observations. Missing observation is not a current ephemeris; material input is not a demonstrated solver. |
| M3 handoff | Explicit scalar and retained-mask outputs, three named transform policies, and `m3_form_potential_ref` into the enclosing frame | Preserve scalar, bitmask and distributed laws separately. EFWA order and distributed folds 16→0 / 17→8 remain unchanged. |

The compact `condition` is **not a replacement for the complete M2 frame**.
In particular, `m3_form_potential_ref` points inside that frame. Consumers need
its distributed 72/64 state and the remaining domain holdings when working at
depth, not only a copied condition or a winning address.

## Concrete consumers and ownership

**K8 / #132** consumes the M2 producer for continuous sound, resonant/material
response and the already-built seven-centre point-cloud receiver. It owns
atomic snapshot admission, stale-generation handling, numerical/audio/render
cadences, mute/resume and lifecycle. The same field may contain several material
elements; it must not be restricted to the selected correspondence's one fibre.

**K9 / #133** uses those engines for `epi.cosmic.123` and the full M2′ instrument.
Parent/deep opening retains current state and exact refs. Returning to the parent
must not re-run a fresh condition merely because the view changed. K9 also
supplies the shared open/operate/return path consumed by K10.

**K10 / #134** receives the same M2 world-condition through Nara's personal
constitution for `epi.personal.450` and M4′. Its subject/identity revisions,
Day/NOW/Flow relations and episode history surround the exact M2 event and
generation; the M2 event reference is not a substitute for those personal or
temporal identities. M0′ is the Bimba map and M5′ is the canonical Epii agent.
K10's actual map operations and authorised Epii Actions consume these source and
state refs through the native product owners. K6 does not add another agent
loop, transcript store or editable temporal authority.

K10's census and C/Rust producer work remain parallel-capable from K4. The
continuous/desktop joins wait only on the K8/K9 outputs they actually consume.
Neither the map/agent functional identity nor that scheduling rule is changed
by this handoff. The installed centres, glyphs and desktop embedding are retained.

## Executable guarantees and remaining acceptance

The existing `m2_condition`, `m2_engine`, `m2_vimarsha` and installed-C++ tests
exercise the native producer, source operations and failure paths. The additional
`m2_parent_handoff` integration tests exercise the real public Rust request and
frame, without mocked surface implementations:

- all **127** published source paths resolve their actual map node/relation IDs
  and retain selected music, colour/palette, world observation and supplied
  material in the same output generation;
- all **17** missing paths remain unavailable even with a complete named palette;
- a new generation with conjugate/tuning changes does not mutate the saved
  original request, and re-executing that original request reproduces the original
  frame against the same compiled source/registry/ledger;
- mismatched event or generation in the Vimarśā, palette or material input fails
  before production.

```sh
cargo test -p ql-mef --test m2_parent_handoff --locked
cargo run -p ql-mef --example m2_engine --locked -- \
  fixtures/kernel/m2-condition-request-v1.json
```

These tests prove producer co-reference and deterministic replay of supplied
inputs, **not** persisted Day/NOW replay, a working map UI, an Epii Action,
physical acoustic/visual equivalence or cross-subject desktop lifecycle.
Those remain the explicit K8/K9/K10 joined acceptance scenarios. A later
interpretation or changed source revision is a new attributable operation, not
silent rewriting of an earlier generation. The ledger's unresolved source
readings and partial capability dispositions remain intact.
