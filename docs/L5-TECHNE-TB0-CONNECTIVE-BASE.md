# L5 Technē TB0 — The Pinned Connective Base

**Standing:** implementation record of QL-MEF #212, 2026-09-16.
**Authority:** `docs/L5-TECHNE-DUAL-READING-LOCK.md` (conceptual) and `docs/L5-TECHNE-INTEGRATED-DEVELOPMENT-WAYFINDER.md` §4–§7 (development relation, PR #221 branch at pin time).
**Contract tag:** `ql.techne/v1` — unchanged. This is a compatible, purely additive extension. No version bump.

---

## 1. The pinned revision

```text
TB0 revision:        TB0-1
landed on:           techne/tb0-connective-base (PR carries the exact head SHA; #212 records it)
schema (reading):    schemas/techne/ql-techne-reading-v1.schema.json   ($id ql.techne/reading/v1)
schema (session):    schemas/techne/ql-techne-session-v1.schema.json   ($id ql.techne/session/v1)
language binding:    crates/ql-adapters/src/techne.rs                  (TechneAdapter, DisclosureSession, validation)
conformance fixtures:fixtures/techne/representative-subject-v1.json    (T0, unchanged)
                     fixtures/techne/absent-facets-v1.json             (T0, unchanged)
                     fixtures/techne/development-day-v1.json           (T0, unchanged)
                     fixtures/techne/tb0-connective-base-v1.json       (NEW — the rich shared specimen)
gates:               python3 scripts/techne-gates.py                   (G-TB0 added)
tests:               cargo test -p ql-adapters                         (18 tests, was 10)
```

`fixtures/techne/tb0-connective-base-v1.json` is the fixture every child lane consumes. One specimen, deliberately rich: real Central source identity for the dual-reading lock itself, a bounded whole of the six contract sources, four distinct typed relation families (two authored non-temporal, one trans-temporal `INSTANTIATES` with interpretation standing and deliberately no date, one `implemented-in` dated through a real temporal-facet ref), the full temporal family (occurrence / receipt / validity / DAY / NOW / session / run, including attempt and Return continuity refs), two place readings (a dated factual `OCCURRED_AT` place with uncertainty and historically valid hierarchy, and a `MYTH_LOCATED_AT` place that is truthfully `unlocated` with no geometry), exact source selectors with standing, a Journey-scene Expression binding with composition and profile, four native ActionRefs including the governed-write Return leg, all four situated-Agency role examples, both application cuts disclosed, and one deliberately unavailable instrument (Palace) plus two degraded facets (Canvas, Place).

## 2. What changed against the T0 revision of `ql.techne/v1`

All additions are optional fields; every T0 fixture, schema instance and binding remains valid without modification.

| Area | Field | Meaning |
|---|---|---|
| `Whole.relations[]` | `relation_ref` | Stable native relation identity; carried verbatim across instruments and cuts. |
| `Whole.relations[]` | `standing` | Evidence standing in the owner's vocabulary (fact, interpretation, myth, …). |
| `Whole.relations[]` | `source_ref`, `evidence_refs` | Where the relation claim comes from. |
| `Whole.relations[]` | `temporal_facet_ref` | Optional real temporal qualification, resolving against a `facet_ref` in the reading's `temporal[]`. Absent = trans-temporal; never manufactured. |
| `Whole.relations[]` | `derivation_ref`, `confidence` | Owner-supplied only; the adapter never infers them. |
| `PlaceFacet` | `relation` | Native place-relation type preserved verbatim (`OCCURRED_AT`, `LOCATED_IN`, `OPERATED_IN`, `TRAVELLED_TO`, `MYTH_LOCATED_AT`, …) — not interchangeable. |
| `PlaceFacet` | `uncertainty` | Owner-supplied spatial uncertainty in the owner's terms. |
| `TemporalFacet` | `attempt_ref` | Factory attempt continuity; rides a run/session facet. |
| `TemporalFacet` | `return_ref` | Return continuity (e.g. a late Factory Return). |
| `WarrantedQlReading` | `m_coordinate_ref` | Warranted canonical M-coordinate ref (M0–M5 registry coordinate) of the reading itself. M′ instrument bindings stay on the disclosure entries. |
| `WarrantedQlReading` | `return_ref` | Where the warranted reading Returns into knowledge ground, when supplied. |
| `Disclosure` | `application_cuts[]` | Cut-level availability (`4:2-deep`, `3:3-conjugate`) with a required reason when unavailable. |
| `TechneReading` | `agency[]` | The situated-Agency role floor (below). |
| `DisclosureSession` | `application_cut` | The active reading; must agree with `instrument` (expressions ↔ 3:3-conjugate, deep instruments ↔ 4:2-deep). |
| `DisclosureSession` | `whole_ref` | The selected bounded whole when whole-scoped. |
| `DisclosureSession` | `project_ref`, `world_ref`, `context_frame_ref` | Active Project / World / Context-Frame focus. |
| `DisclosureSession` | `occasion_ref` | The current shared occasion that must survive cut crossings unchanged. |
| `DisclosureSession` | `return_target_ref` | Where attributable Return from the session is addressed. |
| `DisclosureSession` | `reference_frame_ref` | Wider situated frame in focus (solar/Earth/geography chain), distinct from any single place. |
| `DisclosureSession` | `scene_focus_ref` | Focused Expression Scene; the ref stays the Expression owner's. |

### The situated-Agency role floor

`TechneReading.agency[]` entries are role bindings, not Agents and not a runtime:

```text
guardian   Guardian_i stewardship context; spans both readings
anima      Anima_i = M_i × S4′ — the expressive operator of the 3:3 reading
aletheia   Aletheia_i = M_i × S5′ — the disclosure/Return operator of the 4:2 reading
techne     Technē_i := Aletheia_i while operating the M_i′ deep instrument
```

Each binding carries `m_index` (the coordinate i), optional `reading`, `instrument`, `guardian_ref` (the anchoring canonical Guardian — present does not mean identical), `agent_session_ref` and `profile_ref` (existing native machinery, never a new runtime), and disclosure-terms `authority` / `privacy`. The binding law is enforced in `TechneReading::validate()`: Anima is 3:3 and operates only Expressions; Aletheia/Technē are 4:2; Technē operates only its own coordinate's deep instrument; Guardian stewardship is not bound to one reading.

### Cross-cut identity

`DisclosureSession::cross_cut(target_instrument)` moves a session between the 4:2 and 3:3 cuts carrying subject, selection/source/revision basis, AgentSession, occasion and Return target untouched, and refuses a crossing onto the cut already occupied. The contract test suite pins: byte-stable refs across a crossing, `co_referenced` between the two sides, and the same native ActionRefs (including the `oi.expression.open` crossing Action) on both sides. M3 Journey↔Expression Scene and M4 World↔Nara remain the strongest proving seams for the lanes; the bridge itself is general.

### Canonical identity and the compatibility transport

The canonical spellings are the contract's: `project`, `canvas`, `timeline`, `journey`, `place`, `palace`, `expressions`; cuts `4:2-deep` and `3:3-conjugate`. `TechneInstrument::from_transport_alias` maps the Research Canvas `TechneWorkspaceTransport` vocabulary (`Story`, `Place/Map/Street/Globe`, `Projects`, …) onto these for migration only — unknown names return nothing rather than guessing. The transport remains a migration bridge, never a second application service.

## 3. Compatibility statement

- **Same contract tag** (`ql.techne/v1`), same schema `$id`s, no version bump.
- **Additive-optional only**: every new field is optional with unchanged semantics elsewhere. All three T0 fixtures validate against the extended reading schema unmodified, and the extended Rust binding deserialises all T0 payloads unchanged.
- **Strict closed-object consumers** (the reading/session schemas carry `additionalProperties: false`, and the T0 O-I TS mirror validates strictly) will reject documents that use the new fields until they adopt the pinned revision. First-party consumers update in lockstep with this pin; that update is a mechanical field addition, not a semantic migration.
- **Nothing was renamed.** The instrument enum spellings, cut values, temporal kinds and place precisions are exactly the T0 set; only descriptions and additions moved.

## 4. Recorded gaps at pin time (honest absence, not defects)

- **No live producer yet emits the new relation-standing, place-relation or attempt/return facets from native owners.** The fixture composes them from real refs and declared uncertainty; lanes replace fixture evidence with native-provider evidence before claiming operative behavior that needs it.
- **Palace is deliberately unavailable in the TB0 fixture** (no bound integral composition) and **Canvas/Place are degraded** (warranted layout with no authored view; an inherently unlocated mythic place) — these exercise the honesty law, they are not failures.
- **G3's second real spatial producer** remains open (T1 successor work), as recorded by the gates.
- **The gates' live legs (G2, G7) ran against live Central `ctrl` data and the live `ql` service** at pin time; the desktop and human legs remain G8, the owner's.

## 5. Fan-out

**#213–#219 may now fan out against this exact revision (TB0-1, tag `ql.techne/v1`).** Each lane records the exact revision it consumes in its handoff receipt. After fan-out, shared-contract change returns through #212 with an explicit compatibility statement, an updated fixture and lane notification — no lane privately forks `TechneReading`, the schemas, or the instrument/cut identity.
