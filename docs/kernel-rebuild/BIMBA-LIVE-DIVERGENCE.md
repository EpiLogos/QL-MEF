# Bimba seed pin and the live graph: divergence and the decision it needs

**Standing:** agent-measured on 27 September 2026, for the owner's decision. This
records measurement only. It does not move the pin, promote anything, or pick a
side. The K2 registry (`fixtures/kernel/m-tree-v1.json`, compiled into
`c/src/m_tree_data.inc`) remains the executable coordinate tree at its seed pin.
Every difference below is an open discrepancy in `fixtures/kernel/m-ledger-v1.json`
(ids `bimba-live-*`), with the registry named as current executable authority and
no decision recorded.

## The two sides

| | Seed pin (what QL compiles) | Live graph (upstream's declared authority) |
|---|---|---|
| Source | EpiLogos/Epi-Logos-C-Experiments `daa660c`, `Idea/Bimba/Map/datasets/` (tree `cd4f4f7`) | Neo4j 5.26.30 on Omarchy `bimba-neo4j`, database `neo4j` |
| Identity | git revision + per-file blob + per-record payload sha256 | capture content sha256 `afbe9435…d4ab`, bookmark `FB:kcwQmaxjOZ6CT7+cxvaAEzWE/MlV3pA=`, 2026-09-27T21:46Z |
| Nodes | 1,876 (1,875 coordinates + master `M`) | 2,141 `:Bimba` (3,713 nodes in the database) |
| Relations | 21,083 records = 9,871 distinct triples (most edges appear in both the low-detail and deep files) | 11,774 between `:Bimba` nodes (13,844 in the database; the rest touch Graphiti `Entity`/`Episodic` nodes) |

Upstream `de04069d` (2026-07-13) declared the datasets a deprecated seed archive.
Its `parashakti-deep/nodes-full-detail.json` has still changed since `daa660c`
(+12 lines).

## How it was measured

```bash
python3 scripts/bimba-live-divergence.py capture --endpoint http://100.92.62.101:7474
python3 scripts/bimba-live-divergence.py compare --epi-repo ~/Documents/epi/Epi-Logos-C-Experiments
python3 scripts/bimba-live-divergence.py ledger
```

- **Read-only is enforced by the server.** Every statement runs through the Query
  API with `accessMode: READ`. A write is refused by Neo4j with
  `Neo.ClientError.Statement.AccessMode`; this was checked before the capture.
- **Nothing new is committed.** The capture and the report stay under
  `target/bimba-live/`. The registry and the ledger remain the only committed
  records.
- **Seed values are verified record by record.** Each seed record is read by git
  blob id and checked against the registry's `payload_sha256` before it is used.
- **Translation uses upstream's own code.** Coordinates (`#` → `M`,
  context-frame wrapping) and property keys (`name` → `c_1_name`, …) come from
  upstream's `generate-deep-regional-cypher.mjs` (blob `b0be97b`, run with node).
  That is the script that seeded the live graph. The QL census numeric-path join
  cross-checks it and agrees on all 1,851 unique pairs.
- **The ledger step is idempotent and refuses rewrites.** Re-running it adds
  nothing. If an entry with the same id exists with different detail, it fails
  instead of rewriting it.

## Findings

### Node set

- **The registry is fully present live.** All 1,875 registry coordinates exist in
  the live graph. The four `k4-live-spelling-composite-*` discrepancies from the
  K4 census are spelling only: upstream's context-frame rule maps
  `#0-4.4.0-4.4/5…` onto `M0-4.(4.0/1-4.4/5)…`. That is evidence for those
  entries; they have not been transitioned.
- **16 live M-grammar coordinates have no registry node:**
  - the prime roots `M0'`…`M5'` and `M5-2'`, `M5-3'`, `M5-4'`;
  - a `:Coordinate` family `M4-0`…`M4-5`, separate from the Nara branch nodes
    `M4.0`…`M4.5` that the registry holds;
  - `M2-3-1-51000`, a `:VaultArtifact` with `content_hash: "sha256:live-contract"`
    from `hen_compiler_core`. It looks like test residue in the live graph.
- **250 live `:Bimba` nodes are outside the M grammar:**
  - the lattices S 86, L 86, P 14 and T 13;
  - C 13 (`C`, `C0`…`C5`, and their primes);
  - 13 in the `CF*`/`CP*`/`CS`/`CT` group;
  - 8 bare-M nodes (`M`, `M'`, `M-0`…`M-5`);
  - 7 `#` meta nodes, 6 `Family_*` and 4 `Weave_*`.

  The S and L lattices are the ones restored on 2026-07-29. The registry grammar
  (`#0`…`#5`) covers none of these nodes.

### Relations

- **9,440 of the 9,871 seed triples appear live unchanged.**
- **The other 431 seed triples were renamed, not lost.** The same endpoint pairs
  appear under a new type spelling:

  | Seed type | Live type | Triples |
  |---|---|---|
  | `USES_Pair` | `USES_PAIR` | 282 |
  | `HAS_LOWER_Trigram` | `HAS_LOWER_TRIGRAM` | 64 |
  | `HAS_UPPER_Trigram` | `HAS_UPPER_TRIGRAM` | 64 |
  | `DEFINES_Pair` | `DEFINES_PAIR` | 12 |
  | `MÖBIUS_RETURN` | `M_BIUS_RETURN` | 6 |
  | `MÖBIUS_REFLECTION` | `M_BIUS_REFLECTION` | 1 |
  | `COMPLEMENTS_AS_MÖBIUS_TORUS` | `COMPLEMENTS_AS_M_BIUS_TORUS` | 1 |
  | `HORIZONTAL_SYMMETry` | `HORIZONTAL_SYMMETRY` | 1 |

  The `Ö` became `_`, which lost a character from the type name.
- **After the renames, no seed relation is missing live.**
- **1,903 live triples have no seed counterpart:**
  - 25 connect two M-grammar coordinates. Examples: `INVERTS_TO` ×6; Neptune and
    Pluto's `HAS_CHAKRAL_ANCHOR`, `HARMONICALLY_LEADS_TO`,
    `SPIRALS_THROUGH_VORTEX` and `CLOSES_TO_GROUND`; `M2-5 HAS_INTERNAL_COMPONENT`
    to `M2-5-8`/`-9`.
  - 987 connect an M coordinate to the S/L/prime lattices.
  - 891 lie entirely within those lattices.
- **QL's compiler skips 9 seed relations.** `parashakti-stragglers-relations.json`
  stores relations as per-node `outgoing`/`incoming` lists, and
  `scripts/compile-epi-bimba-map.py` reads 0 records from that shape. So the
  Neptune/Pluto edges are in the seed and in the live graph, but never reached the
  registry. The fix is a small compiler change, but it regenerates the registry.
  It therefore goes through the same decision below
  (`bimba-live-relation-stragglers-uncompiled`).
- **450 registry relation records have an endpoint that is not a registry
  coordinate** (null targets, names). They were not compared.

### Property values

Two kinds of comparison, depending on whether upstream maps the seed key:

- **Mapped keys** use the upstream `mappings` table: 14,827 seed values on the
  1,875 shared coordinates.
- **Unmapped keys** have no entry in that table: 22,373 seed values under 2,330
  keys. They were compared against any live key with the same snake-case suffix.
  A suffix match is a report, not a mapping.

| Outcome | Count |
|---|---|
| Mapped value equal live | 11,902 |
| Mapped value equal, but under a different live key form (e.g. `m_3_degree` → `m_3_5_degree`, `m_2_arabic_text` → `m_2_4_arabic_text`, `c_1_architectural_function` → `c_2_…`) | 2,758 (37 key pairs) |
| Mapped value different live | 47, plus 84 under a different key form |
| Mapped value absent live | 36 (`c_1_architectural_function`) |
| Seed conflicts with itself (deep vs low-detail file) | 14 (`#1-5*` names and core natures) |
| Unmapped value equal under a same-suffix live key | 16,914 |
| Unmapped value different under a same-suffix live key | 1,613 |
| **Unmapped value with no live counterpart** | **3,846** |

Upstream names the key-form drift itself as "NAMED STALENESS" in its generator.

Among the 3,846 absent values are the decan `planetaryRuler` (114 values) and
`degreesRange` (36), the other session's finding, now confirmed. **QL's own
generators read some of the absent keys:**

| Key | Values absent live | Read by |
|---|---|---|
| `element` | 130 | `m2-catalogue.py` |
| `number` | 86 | `m3-source-parity.py`, `m3-observation-parity.py` |
| `intervalStructure` | 81 | `m2-correspondences.py`, `m2-field-report.py` |
| `elementalCorrespondence` | 65 | `m2-correspondences.py` |
| `upperNuclearBinary`, `lowerNuclearBinary`, `binaryCode` | 64 each | `m3-source-parity.py` |
| `codonSequence`, `hexagramNumber` | 24 each | `m3-domain.py` |
| `matrix` | 12 | `m3-observation-parity.py` |
| `binaryRepresentation` | 8 | `m3-source-parity.py` |
| `planetaryMode` | 7 | `m2-correspondences.py` |

`qlPosition` (643) and `family` (140) are also read, and differ from their
same-suffix live keys (`c_4_ql_position`, `c_4_family`). Live seems to use those
keys for a different property, not an edited value.

**Recovery and empties:**

- All 20,769 properties recovered from the transaction log on 2026-07-29 are
  still present live.
- The live graph carries 55,696 non-embedding properties across 3,026 keys. 2,958
  of those keys have no seed mapping.
- 861 live properties are empty strings. All 828 that upstream named as
  unrecoverable are still empty, and 33 are new (`p_*` +7, `q_*`/`l_*`/`c_*`
  and one `sync_*`).

## Who depends on the pin

The pin reaches these consumers, grouped by how they depend on it. Keep the C
kernel pin separate: `vendor/epi-kernel`, `migration/epi-kernel/source-lock.json`
and `sync-epi-c-reference.sh` also pin `daa660c`, but for C source, not the
datasets.

1. **Lock and compile chain.**
   - Files: `data/epi-bimba-map/source-lock.json` → `compile-epi-bimba-map{,-live}.py`
     → `generate-m-tree.py` → `c/registry/m-tree-source-v1.json`,
     `fixtures/kernel/m-tree-v1.json`, `c/src/m_tree_data.inc`
     (`c/src/m_tree.c`) → `k8-structure.py` → `m_tree_live_*.inc` and
     `c/registry/promotions/k8-apertures-v1.json`.
   - `crates/ql-mef/build.rs` panics if the K8 projection is stale.
2. **Generators that read seed values from a sha-verified checkout.**
   - Scripts: `m2-correspondences.py`, `m2-field-report.py`, `m3-source-parity.py`,
     `m3-domain.py`, `m3-observation-parity.py`, `aw0.py --verify-sources`,
     `verify-kernel-rebuild-ground.py`.
   - Shell wrappers: `test-m2-engine.sh` and `test-m3-acceptance.sh`.
   - Their outputs: `m2-correspondences-v1.json`,
     `m2-field-census-summary-v1.json`, `m3-domain-v1.json` and
     `m3-source-bindings-v1.json`.
3. **Readers of the registry fixture.**
   - Scripts: `m-ledger.py` (locks the registry's sha and revision), `m_census.py`,
     `k8-census.py`, `k8-graph.py`, `m2-catalogue.py` (→ `c/src/m2_data.inc`),
     `m2-ledger.py`, `m3-ledger.py`, `m3-receipt.py`, `generate-m3.py`,
     `check-m1-acceptance.py` (hardcodes registry revision `259a2f…`).
   - Rust: `m_tree.rs` (`include_str!`), `epi_agent.rs` (includes the source lock),
     `production_join.rs` (hardcodes the dataset path and revision).
   - Shell: `test-native-m-tree.sh`, `test-k8-native.sh`.
4. **Embedded counts and blobs.**
   - `test_m_tree.py` and `native_m_tree.rs`: 1,876 / 21,083 / 24,831.
   - `m_ledger.rs` and `test_m_ledger.py`: 1,876.
   - `kernel-rebuild-source-ground.yml`: exactly 44 Nara coordinates.
   - `vak.rs` and `qv.rs`: dataset file blobs.
   - Also `central-ground.ts` and `EPI-HOLOGRAPHIC-KERNEL-MANIFEST.json`.
5. **CI checkouts at `daa660c`.**
   - Workflows: `epi-bimba-map-conformance`, `kernel-rebuild-source-ground`,
     `kernel-m2`, `kernel-m3-engine`, `m3-engine`, `epi-vak-source-parity`,
     `aw-field`.
   - `verify-kernel-rebuild-ground.py:189` requires the Bimba lock and the C-kernel
     lock to share one revision. That check would have to split before the two
     pins could differ.

Nothing in `crates/*/src` reads Neo4j. The K4 census holds one earlier live
capture (`fixtures/kernel/census/live-bimba-capture-v1.json`, 2026-09-11: names and
labels only) and uses it as a join aid.

## The decision

**Question:** how should the QL registry follow the live authority, without a new
index becoming canon beside the C-compiled registry and the ledger?

**A. Keep the seed pin; the ledger carries the difference (the state after this
change).**
- Nothing breaks, and CI stays hermetic.
- The registry keeps inputs the live graph has lost: decan rulers and ranges, the
  hexagram nuclear binaries, interval structures.
- Every later capture that differs produces new ledger lifecycle entries.
- **Cost:** QL stays behind live edits indefinitely, and the ledger grows with
  each capture.

**B. Pin a live-graph export by revision hash.**
- Upstream commits a deterministic, read-only export, e.g. the `capture` shape
  of this tool, moved upstream. QL's lock then pins that upstream commit, the
  export blob and its content sha, exactly as it pins the datasets today.
- The registry is still the one compiled index. The export is upstream's own
  artifact, not a QL file, so no second index appears here.
- **Cost, done once:**
  - an input adapter for the live shape (`M` spellings, `c_N_` keys, renamed
    relation types);
  - one registry regeneration, which cascades through every consumer above
    (counts, revisions, CI refs, the split of `verify-kernel-rebuild-ground.py`);
  - **loss of the generator inputs listed above** until upstream restores them.

**Recommendation: B, staged. A holds meanwhile.**

1. **Hold A.** It is in place now.
2. **Upstream restores what QL reads.** Upstream registers and restores the seed
   properties the kernel consumes (the table above) and the decan
   `planetaryRuler`/`degreesRange`. This is the same restoration upstream's own
   generator already calls for. It also decides the `M_BIUS` spelling, the
   `M2-3-1-51000` artifact, and the 33 new empty values.
3. **Upstream commits the export**, pinned by commit, blob and content sha.
4. **One QL PR moves the pin.** In that single PR:
   - add the live-shape adapter;
   - fix the straggler compiler gap;
   - regenerate the registry and run the full consumer cascade;
   - move each `bimba-live-*` discrepancy to `applied` or `rejected`, citing the
     owner's decision.

   Re-running `compare` against the pinned export before merging shows what
   still differs.

**What the owner decides:**
1. A or B.
2. If B: whether step 2 is a precondition, or whether B may proceed with the lost
   inputs ledgered as open.
3. Whether the S/L/C/P/T lattices (250 nodes, 1,878 relations) enter the registry
   grammar or stay outside it.
