# K4 — automated M1/M2/M3 pre-vertical coverage census

Status: **executed** (this revision; rerunnable, see below)
Ticket: EpiLogos/QL-MEF#128. Depends on the K0–K3 floor (#143, #144, #145, #146, #147).
Consumed by: #129 (K5 / M1), #130 (K6 / M2), #131 (K7 / M3).

## What ran

The census joined three bodies of evidence to the accepted M ledger, one
coordinate at a time:

1. **The live Bimba graph** (read-only capture of the operator's Docker Neo4j,
   `neo4j 5.26.20-community`, captured 2026-09-11T01:33:17Z): 2,141
   coordinate-bearing nodes and 11,774 relations between them. This is the
   most complete Bimba body and it is richer than the serialized snapshot.
   The committed, normalized capture is
   `fixtures/kernel/census/live-bimba-capture-v1.json` (names, labels and
   architectural-function content per coordinate, plus the full observation
   receipt). Credentials are operator-supplied per session via
   `$EPI_BIMBA_PASSWORD`; nothing is embedded in the repository and the graph
   was never written to.
2. **The K2 registry** (`fixtures/kernel/m-tree-v1.json`, revision
   `259a2f49…`): 1,875 serialized M coordinates, their relations, source
   records and alternate spellings — the accepted structural identity field.
3. **The real implementation bodies**: the vendored C M0–M5 kernel
   (`vendor/epi-kernel/reference/src/**`), the native C centre (`c/src/**`),
   and the current Rust crates (`crates/**`, excluding tests).

## How the join is traced (not guessed)

- **Coordinate identity.** Live spellings (`M1-2-0`) join to registry spellings
  (`#1-2-0`) only through the one alias the K2 registry itself sanctions
  (`M<N>` ↔ `#<N>`, including descendants) plus equal-numeric-path matching,
  which is exactly how the registry already models its 8 alternate-notation
  groups. Prime spellings (`M1'`, `M5-3'`) are deep-instrument coordinates,
  not M-tree nodes: they carry no numeric-path identity and stay live-only
  deltas. A bad join here would have silently mapped `M1'` onto the `#1`
  root; the census tests pin that this cannot happen.
- **Implementation anchors.** A construct (C function/table, Rust
  fn/struct/enum/const) binds to a coordinate two ways, both recorded in the
  ledger rationale: an exact coordinate spelling in the construct's own text,
  or a **full-token name anchor** — every distinctive token of the construct's
  identifier is explained by that coordinate's own Bimba names/labels or its
  inherited ancestor names, with the deepest such coordinate winning. So
  `ANANDA_BIMBA` → `#1-2-0` ("Matrix 0: The Original (Bimba)", under ancestor
  "Ananda"), `spanda_pass_seed` → `#1-3-0`, `ANANDA_QUINTESSENCE` → `#1-2-5`.
  Incidental name words cannot bind: memory machinery (`arena.c`) is
  dispositioned infrastructural, and generic identifier vocabulary is stopped.
- **Classification.** A coordinate is `implemented` (a construct's deepest
  anchor is the coordinate itself), `partial` (covered only through its
  descendants' constructs, which are carried as the coordinate's aggregate
  bindings), or `unimplemented`. Presence is all that is claimed: readiness
  warrants are "implemented" on implementation-discovery receipts, never
  "tested"/"observed" execution, and all five parity axes remain unassessed —
  proving C↔Rust parity is K5/K6/K7 work.

## Results

Per-root counts (coordinate rows in `fixtures/kernel/m-ledger-v1.json`;
workbooks in `fixtures/kernel/census/census-m{1,2,3}.json`):

| Root | Coordinates | C implemented | C partial | C unimplemented | Rust implemented | Rust partial | Rust unimplemented |
|---|---:|---:|---:|---:|---:|---:|---:|
| M1 | 43 | 22 | 0 | 21 | 16 | 4 | 23 |
| M2 | 597 | 29 | 14 | 554 | 61 | 20 | 516 |
| M3 | 996 | 10 | 8 | 978 | 30 | 8 | 958 |

The ledger now carries 1,828 rows (185 source capabilities + 7 index rows +
1,636 census rows) and 1,873 implementation records: 90 C (75 coordinate-bound
plus the explicitly infrastructural arena machinery), 148 Rust, 1,635
live-graph structural-index records (the joined live coordinate nodes). 76
source capability rows gained `bound` dispositions from intersecting
discoveries. Readiness claims are row-scoped to four committed evidence
records: `k4-live-capture` (observation), `k4-c-discovery` and
`k4-rust-discovery` (implementation receipts in
`fixtures/kernel/census/discovery-{c,rust}-v1.json`), beside the K3
`k2-index-declaration`.

Known depth that the join could **not** confidently seat is preserved in
`fixtures/kernel/census/reports/orphan-implementations.json` (257 C and 672
Rust constructs with recorded reasons and candidate tokens). The largest C
clusters are the closed-#76 native foundation (`c/src/kernel.c`,
`c/src/holographic.c`, `c/src/primitive.c`) and engine machinery
(`engine.c`, `psychoid_numbers.c`, `qv_data.c`): these implement the QL
family/position/face field whose coordinates are outside the M-grammar
registry, so seating them is a registry-extension decision, not a census
judgement call. The largest Rust clusters are the Vāk composition stack and
`music.rs` internals — operation vocabulary, not Bimba name vocabulary.

## Deltas and disagreements (preserved, not resolved)

- **Live vs serialized.** 1,851 live coordinates joined uniquely; 16 joined
  into the registry's 8 alternate-notation groups (recorded as ambiguous);
  18 live spellings have no registry identity at all — the six M-root primes
  `M0'`–`M5'`, three M5 sub-primes, nine spellings of two composite-segment
  families (e.g. live `M0-4.(4.0/1-4.4/5)` vs serialized `#0-4.4.0-4.4/5`),
  and one genuinely new live node (`M2-3-1-51000`, a hen-compiler promotion
  placeholder). 8 serialized coordinates have no live node. All are listed in
  `fixtures/kernel/census/reports/live-serialized-delta.json`.
- **Ledger discrepancies.** The two composite-segment families carry open
  ledger discrepancies (`k4-live-spelling-composite-m0-…`, `…-m1-…`, peers
  bimba→c, axis coordinate) beside the retained K3 `k2-parent-0`. Current
  authority stays with the K2 registry at its accepted source lock; promoting
  the live spellings (or the new live node) is a reviewed registry
  regeneration with a new source lock, not a census decision.
- **Reports.** `missing-c-structural-bindings.json` (1,553 coordinates whose
  Bimba content exists without a discovered C body), `missing-partial-rust-ports.json`
  (49 C-implemented coordinates without Rust; 73 Rust-only coordinates),
  plus the orphan and delta reports above.

## Reproducing and refreshing

```sh
# One-time, with operator credentials for the live graph:
EPI_BIMBA_PASSWORD=... python3 scripts/m_census.py capture-live

# The census itself (uses a fresh capture when present, otherwise the
# committed capture artifact — both produce byte-identical output):
python3 scripts/m_census.py census
python3 scripts/m-ledger.py check
cargo run -p ql-cli --locked -- kernel validate-ledger --json
cargo run -p ql-cli --locked -- kernel coverage M1 --json   # likewise M2, M3
```

Reruns are byte-deterministic: same capture and tree, same bytes. CI enforces
this (`M ledger / matrix lock` workflow: refresh + census + `git diff
--exit-code`). When the live graph changes, re-run `capture-live`, review the
delta report, and re-run the census; new disagreement stays explicit.

## Standing of this census

What this census establishes: coordinate-by-coordinate implementation
presence and aggregate coverage in C and Rust, live-graph structural
presence, the unseated orphan inventory, and the live/serialized delta —
all as ledger rows, evidence and reports.

What it does not establish: C↔Rust computational parity (all parity axes
unassessed), execution of any operation, experiential acceptance, and
canonical resolution of any recorded discrepancy. Those belong to the
verticals (#129/#130/#131), which consume their root's workbook, work graph
(`fixtures/kernel/census/work-graph.json`) and reports instead of
rediscovering the subsystem.
