# QL builds from the Bimba map

The Bimba map, the owner's Neo4j graph, is the authority. QL's coordinate
registry and every table derived from the map are generated from a read-only
read of it. Until 28 September 2026 they were compiled from the July
pre-migration dataset exports (Epi-Logos-C-Experiments `daa660c`,
`Idea/Bimba/Map/datasets/`), which upstream has since frozen as a seed archive.

## Refresh

```bash
scripts/refresh-from-bimba-map.sh --endpoint http://100.92.62.101:7474
```

This command:
- reads the map with `accessMode: READ` (the server refuses writes);
- rebuilds the registry extract and every map-derived table;
- re-bases the owner-ratified K8 promotion onto the new registry.

Review the diff. If an M2 or M3 proof input changed, publish its successor
(`scripts/m2-ledger.py publish --reason ...` after a full acceptance), then run
`m-ledger.py refresh`, `m-ledger.py check` and `k8-census.py --write-receipt`.

## What is generated from what

| Output | From |
|---|---|
| `c/registry/m-tree-source-v1.json` | the map read: M0–M5 nodes, their `c_1_name`, and every relation touching them. Stores property keys and payload hashes, not values. |
| `fixtures/kernel/m-tree-v1.json`, `c/src/m_tree_data.inc` | that extract (`generate-m-tree.py`; CI regenerates and diffs) |
| `fixtures/kernel/m2-correspondences-v1.json`, `c/src/m2_correspondence_data.inc` | the registry's relations, plus `c_1_name` and `c_1_yantra_form` from the map read |
| `fixtures/kernel/m2-field-census-summary-v1.json` | the map read against the retained C catalogue |
| `fixtures/kernel/m3-domain-v1.json`, `fixtures/kernel/m3-source-bindings-v1.json` | the M3 projection of the map read. The domain file holds the whole projection, so CI audits it without a map read. |

M2 cross-table links (`m2::linked_readings`) follow the registry's relations,
the map's own relations. Each link carries `via`, the relation types.

## Spelling

- **Coordinates:** QL's `#` notation is the map's coordinate with `M` → `#` and
  the context-frame brackets dropped: `M2-5-(0/1)` is `#2-5-0/1`.
- **Bracketed spellings:** the exact map spelling is kept as the node's alias.
- **Endpoints outside the M tree:** S/L/C lattices, primes, and the map's own
  `#0`…`#5` meta nodes are spelled `bimba:<map coordinate>`.
- **Relation types:** these keep the map's spelling (`HAS_UPPER_TRIGRAM`,
  `USES_PAIR`).

## What the map does not carry

These fields existed in the July seed and QL used them. The map does not hold
them, so they are explicit gaps and nothing is invented to replace them:

| Seed field | Where it was used | Now |
|---|---|---|
| `intervalStructure` (full maqam scale spelling) | Bimba-spelled 24-TET tuning | Empty. The map states tonic, dominant and ajnas. The retained C intervals stay the kernel's tuning. |
| `planetaryMode` | correspondence literal | Empty. The map has `c_0_modal_signature` prose. |
| `elementalCorrespondence` (chakra) | correspondence literal | Empty. The material fibre stays from the retained C chakra row. |
| King Wen number, hexagram/trigram binaries | M3 audit | Not read. Trigram bits are the kernel's declared table. |
| `USES_PAIR` role, `LINE_CHANGE` line number | M3 audit | Not read. Line changes are checked as single-bit flips. |
| backbone `codonSequence` / `hexagramNumber` | `ql_m3_clock_projection` | Gap. The codon arrives with the map's `EMBODIES_PALINDROMIC_CODON` relation. There is no backbone→hexagram fact. |

Non-dual matrix cells hold one `YIELDS_CODON` edge in the map, where the seed
had a parallel positive/negative pair.

## Ledger transition (28 September 2026)

- **Evidence:** every entry named the July registry revision. All are
  re-pinned to the map registry: same coordinates, same artifacts, new
  registry. Artifacts whose bytes changed are re-locked.
- **K4 spelling discrepancies:** the four `k4-live-spelling-composite-*`
  discrepancies were about the doubling coordinate the seed spelled
  `#0-4.4.0-4.4/5` and the map spells `M0-4.(4.0/1-4.4/5)`. They are applied:
  the registry takes the map's spelling, `#0-4.4.0/1-4.4/5`.

## Still reading the July seed

These consumers are outside the registry and generator port:
- the C kernel reference pin (`vendor/epi-kernel`, moving in #257);
- the VAK language map (`data/epi-bimba-map/anuttara-language-map.md`,
  `vak.rs`);
- the AW0 original-law source check (`aw-field.yml`);
- the historical K0 ground verification (`kernel-rebuild-source-ground.yml`),
  which checks what the kernel rebuild started from.

`compile-epi-bimba-map*.py` and `data/epi-bimba-map/source-lock.json` remain for
that last check only.
