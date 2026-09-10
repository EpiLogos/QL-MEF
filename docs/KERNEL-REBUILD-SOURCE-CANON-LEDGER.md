# K0 — kernel rebuild source / canon ledger

**Programme:** [Kernel Rebuild Wayfinder](KERNEL-REBUILD-WAYFINDER.md), #135.  
**Acceptance scope:** #124, source ground only.  
**Machine companion:** [source-canon-ledger-v1.json](kernel-rebuild/source-canon-ledger-v1.json).  
**Observed:** 2026-09-10. No private Control, source repository or Neo4j database was changed.

## 1. The accepted floor

PR #136 is merged. Its resulting main was read back as **`d658b3478e42d96b947a5781f5468edf1886e511`**. This is K0's immutable input floor, not a claim that a future main must retain that SHA.

#138 is complete through PR #142, merge **`374cf8db5569401980a0f88ae38da1e2d11129b1`**. **`ql.vak-composition/v1` is the QL-owned producer contract.** Its public surfaces are:

```text
ql_mef::vak_composition::VakComposition
ql_cli::vak_composition::execute_request
ql vak compose <request.json> --json
```

The verifier compares the actual producer implementation, fixture and test objects at the K0 floor with #142. #123 remains architectural provenance. This ledger does not reopen Vāk reconciliation, reproduce the parallel articulation theory, or make downstream adoption a prerequisite for producer completion.

The current holographic contract is `ql.holographic-kernel-contract/v1`, semantic version `1.1.0`. Keep its source D-development identities distinct from `ql.structural/v2`; keep generic structural carrier/shape contracts distinct from source-owned semantic assertions. The seven Context Frames are settled. Historical `(4/5/0)` is superseded **in the CF register**; the Personal `4/5/0` application relation is a different subject and is not invalidated by that determination.

## 2. Authority is scoped, not chronological

For each claim ask: **what subject is being determined, by which source or explicit owner ratification, with what warrant and what implementation evidence?** A newer commit, convenient implementation, old package path, source address or passing test does not answer that question by itself.

| Class | Current use and boundary |
|---|---|
| Canonical structural source | Accepted formal/fixture contracts determine shared identities and operations. Their existence does not prove every C binding is on main. |
| Canonical deep-coordinate source | Live Neo4j Bimba is the deep semantic/relational home. The pinned serialized Map is an exact, source-bearing corpus, not an observed current database snapshot. Structural promotion still requires explicit reconciliation. |
| Current C implementation | Main's native `c/src` contains only `primitive.c`. The corrected imported full body and unmerged recovery centre are separate inputs. |
| Current / imported Rust | Current `crates/**` and source `Body/S/S0/portal-core/**` are rebuild bodies. An implementation arrangement is not a second kernel authority. |
| Returned correction / ratification | `M3-COIN-1` has an explicit owner determination, original hashes, patch and corrected digests. This is not permission to alter other reference data to make parity green. |
| Research proposition | Authored learning/embodiment/correspondence proposals retain their own warrant. They are neither discarded nor promoted to operative capability by appearing in a matrix. |
| Implementation evidence | Old runs, source-shaped implementations, bounded Nara coverage and compiler outputs prove only the revision and scope exercised. |
| Stale representation | Old status, evidence-only disposition, narrowed export and tranche-lock wording remains traceable, with the precise supersession or unresolved difference recorded below. |

**Imported C and Rust are kernel-rebuild implementation bodies, not disposable historical evidence.** Retaining a frozen specimen and its provenance is compatible with rebuilding it. The Wayfinder's explicit owner clarification changes the development disposition; it does not erase original provenance, make old pointer/package coupling mandatory, or declare every old algorithm correct.

The source repository can hold both historical implementation arrangements and source-authoritative authored Bimba material. Do not apply one repository-wide historical/current label to every subject. It remains read-only in this programme; source corrections are accounted for in QL-MEF and the ledger, not silently written back to the prototype.

## 3. Exact repository and object pins

| Input checkout | Immutable revision | Standing |
|---|---|---|
| QL-MEF K0 floor | `d658b3478e42d96b947a5781f5468edf1886e511` | Verified main after #136. |
| QL-MEF #76 recovery | `e5f612653c71c8a9ae74199ef2e8b326e3e9deba` | Closed, **unmerged**. Useful C foundation to reconcile, not revive wholesale. |
| Epi-Logos-C-Experiments source | `daa660cbc1b8c5da83828698665a753852cb0287` | Re-read current main, also the existing C and Bimba lock. |
| Existing Nara implementation witness | `52e7433bee54561a767833895b59f553cca62149` | Existing Bimba conformance input, not whole-M or live-source authority. |

Every machine input is an immutable **repository + commit + path + object kind**. Where independently read, its Git object ID is recorded too. The verification receipt resolves all of them to object IDs and byte SHA-256s, recursively inventories tree contents, and refuses missing/mismatched objects. A commit/path pin is not a moving branch reference.

Key body/source objects at those revisions:

| Body | Git tree / blob |
|---|---|
| QL native `c` | `87f404db6493e3d419b36088882829cc54954eb6` |
| QL current `crates` | `2b1664b2a7cfdb86b655733af17f35719da1158c` |
| Corrected `vendor/epi-kernel/reference` | `f64e1fb2150ca02fc1aca9486b8a38a1e4fa63f8` |
| Original `Body/S/S0/epi-lib` | `f2995865292f90c8fab9370ba90ff8530c75f2dc` |
| Original Rust `Body/S/S0/portal-core` | `727bcd861affb424e4b2251983b4c6d893ffa3fe` |
| `Idea/Bimba/Map/datasets` | `cd4f4f77c13f27e2563c5a6753d2f8bf2b605f15` |
| Bimba `source-lock.json` | `865f61cc38a8c84c4e01cfc3b216b505d2d43fd7` |
| C `migration/epi-kernel/source-lock.json` | `8973e734916558af02e555bd1240b2874db9e543` |
| Matrix registry `.oi/product.json` | `b19dea7d6ea1fcac94c5f8751d78f0205e6fb52f` |
| Rust `m_map.rs` | `bea337c6a20416b6511bf0d01395e78ad2d98aa5` |

`bimba-corpus` covers **all of `Idea/Bimba/Map/**`**, not only the lock's required files. The verifier separately checks every required source-lock blob. Deep/low-detail, meta-source, alternate spelling, straggler and migration material remains present and independently attributable. Migration scripts are source evidence; K0 never executes graph writes.

### The matrices are not one flattened register

Resolve the six families declared by the pinned `.oi/product.json#capability_matrices`. Their 30 declared carriers include the suite profile, readiness, Ta-Onta Agent-world, normalized M0–M5 capability field/partitions, four M1–M4 deep coordinate matrix pairs, and three relational field pairs.

The [field index](integrations/epi-logos/EPI-CAPABILITY-MATRIX-FIELD-INDEX.md) and [source trace](integrations/epi-logos/EPI-CAPABILITY-MATRIX-SOURCE-TRACE.md) retain rationale and source warrants. The richer deep v2 matrices are not partitions of the normalized 36-capability field. Neither the suite profile nor those 36 capabilities substitutes for full recursive M coverage. K0 generates an exact carrier inventory, **not** K3's future capability ledger or K4's coverage census.

Embedded cross-product revisions in older matrix source traces remain explicitly dated implementation evidence. K0 pins their carrier bytes; it does not pretend to have reverified all those external products at current main. Unmerged #140/#141 were inspected and excluded from the accepted floor; neither is overwritten or merged by this work.

## 4. Discrepancy / stale-carrier account

These IDs are shared with the machine ledger. No source is deleted to remove a discrepancy.

| ID | Finding | Determination / next owner |
|---|---|---|
| D01 | Native main lacks `c/Makefile`, `holographic.c`, `kernel.c` and the R4 parity/package paths still mentioned in reference material. #76 contains them but was not merged. | #125 restores selectively. Earlier R4 green runs remain branch evidence, not current availability. |
| D02 | C-lock/prototype and secondary-witness-only Rust descriptions understate the clarified rebuild role. | #135's explicit disposition governs: retain and rebuild both bodies, retaining provenance and one-kernel ownership. |
| D03 | Original M3 coin values `{6,9,7,8}` differ from ratified `{6,9,8,7}`, with associated pair/suit corrections. | Preserve `M3-COIN-1`, its patch and corrected hashes. The verifier checks every locked original C file and every allowed corrected difference. |
| D04 | Bimba lock/compiler wording says “live source” although its immutable input is Git. | Exact serialized source verification is distinct from unobserved current Neo4j state. K4 owns live comparison. |
| D05 | Historical `fetch_bimba.py` drops relation properties and many node fields; it uses local defaults and writes into the source directory. | Retain query lineage, not a completeness warrant. Use operator-authenticated read-only capture outside source; do not run that fetcher over the pinned corpus. |
| D06 | Matrix “current locks” refer to earlier tranches, including QL `590ea5464e0b422beaddea2669039e3d8c7fcb5c`. | Preserve source meanings and receipt scope. K4 rechecks actual implementation/readiness instead of choosing by date. |
| D07 | Six normalized domains, four deep coordinate matrices and the Rust M index have different coverage and roles. | K2–K4 preserve real asymmetric topology and independent readiness. No fabricated siblings or 36-row completeness claim. |
| D08 | Pre-#142 next-action and branch-only #136 prose remains in provenance carriers; old CF `(4/5/0)` remains historical. | This opening floor supersedes status only. #123 stays provenance; completed #138 and seven CFs are not reopened. |
| D09 | Overlapping exports, separators, alternate spellings and incomplete endpoints cannot be silently merged. | Existing `MCoordinate`/`SourceRecordRef` and compiler preserve records. K4 resolves actual relations with warrants; not latest-file-wins. |
| D10 | Research/design depth can be mistaken for implemented learning or continuous/experiential embodiment. | Preserve per-source warrant and separate evidence axes; later coordinate work proves implementation. |
| D11 | #78 still calls #76 active; #69 retains old nested S4′ Ta-Onta notation. | Retain useful #51/#78/#69/#73 laws. Use the explicitly ratified current Ta-Onta S0′–S5′ mapping; old residency/status does not override it. |

## 5. Reproduce the source ground

Use a QL-MEF checkout containing this ledger. Keep all extra checkouts under ignored `target/`; no private filesystem adoption or installed World is needed. The source/recovery commits must exist locally; the verifier never silently fetches or substitutes a checkout's HEAD.

```sh
mkdir -p target/k0-sources

git fetch origin d658b3478e42d96b947a5781f5468edf1886e511
git fetch origin 374cf8db5569401980a0f88ae38da1e2d11129b1
git fetch origin e5f612653c71c8a9ae74199ef2e8b326e3e9deba
git worktree add --detach target/k0-sources/c-recovery \
  e5f612653c71c8a9ae74199ef2e8b326e3e9deba

git clone --no-checkout https://github.com/EpiLogos/Epi-Logos-C-Experiments.git \
  target/k0-sources/epi
git -C target/k0-sources/epi checkout --detach \
  daa660cbc1b8c5da83828698665a753852cb0287
git -C target/k0-sources/epi fetch origin \
  52e7433bee54561a767833895b59f553cca62149
git -C target/k0-sources/epi worktree add --detach ../nara \
  52e7433bee54561a767833895b59f553cca62149

python3 -m unittest discover -s scripts/tests -p test_kernel_rebuild_ground.py -v
python3 scripts/verify-kernel-rebuild-ground.py \
  --source-repo target/k0-sources/epi \
  --implementation-repo target/k0-sources/nara \
  --recovery-repo target/k0-sources/c-recovery \
  --out target/kernel-rebuild-k0
python3 scripts/check-capability-matrix-registry.py
python3 scripts/compile-epi-bimba-map-live.py \
  --source-repo target/k0-sources/epi \
  --implementation-repo target/k0-sources/nara \
  --source-lock data/epi-bimba-map/source-lock.json \
  --out target/kernel-rebuild-k0/bimba
bash scripts/test-epi-c-parity.sh
python3 scripts/validate-epi-holographic-manifest.py
cargo test --workspace --all-targets --locked
```

Reuse existing checkouts on repeat runs rather than overwriting them. Git worktree metadata may be created, but no pinned source contents are edited.

The read-only `Kernel rebuild K0 source ground` workflow runs those bounded checks against explicit source/recovery checkouts and uploads `kernel-rebuild-k0-source-ground`. Normal Rust CI continues independently. Missing source objects, failed compiler/parity checks and digest mismatches fail the job; no skipped-green substitute is used.

Generated outputs:

```text
summary.json                    exact input / correction / producer / K1 receipts
resolved-inputs.json             all declared pins with standing and object IDs
source-file-inventory.json       recursive per-file Git IDs and SHA-256s
matrix-carrier-inventory.json    all declared carrier IDs, protocols and hashes
live-read-request.json           read-only query payload; not a database receipt
bimba/**                        existing source compiler outputs, not a new graph
```

## 6. Live Neo4j basis and the unobserved boundary

**No authenticated live Neo4j observation was available in this session. `database_revision` and `receipt` are null.** A connector search exposed no Neo4j connection. No local credential or database state is inferred from the historical source endpoint. K0 accepts the pinned source/access basis and reproducible serialized path; it does not certify live graph equality.

The pinned historical access source is `Idea/Bimba/Map/datasets/fetch_bimba.py`, blob `42ff8929319713de26775c99a17543f54d8d49db`. It establishes the `bimbaCoordinate` property and the HTTP transactional-read lineage, historically `http://localhost:7475/db/neo4j/tx/commit`. The actual operator must supply the endpoint/database and a read-only identity. Never reuse the script's embedded defaults.

The generated request records database components, coordinate-bearing nodes with all properties, and incident relationships in both directions with their properties and endpoint context. Numeric database IDs are **export-local joins**, never canonical QL coordinate IDs. Uncoordinated endpoints remain visible.

A later authenticated K4 capture can use the generated payload without changing the corpus:

```sh
# Set NEO4J_TX_URL and NEO4J_USER to the actual authorized read-only service.
# curl prompts for the password; do not put it in this ledger or shell history.
curl --fail --silent --show-error --user "$NEO4J_USER" \
  --header 'Content-Type: application/json' \
  --data-binary @target/kernel-rebuild-k0/live-read-request.json \
  "$NEO4J_TX_URL" > target/kernel-rebuild-k0/live-response.json
python3 - <<'PY'
import json
from pathlib import Path
response = json.loads(Path('target/kernel-rebuild-k0/live-response.json').read_text())
assert not response.get('errors'), response.get('errors')
assert len(response.get('results', [])) == 3, 'incomplete graph read'
PY
sha256sum target/kernel-rebuild-k0/live-read-request.json \
  target/kernel-rebuild-k0/live-response.json
```

Retain raw response, errors, hashes, actual database identity/version, UTC capture window and backup/transaction provenance. A single HTTP transaction request is **not** proof of snapshot isolation across concurrent writes. Establish a suitable capture consistency policy and record it before claiming snapshot parity. Compare with serialized records at K4 without overwriting them or choosing the newest representation by default. No K0 check runs this live operation.

## 7. Exact K1 handoff and K2–K4 inputs

**K1 = #125.** Start at actual main containing this accepted ledger; preserve the pinned K0 and #142 inputs even when main moves. Compare against **`e5f612653c71c8a9ae74199ef2e8b326e3e9deba`**, not a guessed resurrected branch head.

Recovery candidates, each resolved to an exact object ID by `summary.json#k1_recovery_inputs`:

```text
c/Makefile
c/include/ql/holographic.h
c/include/ql/kernel.h
c/src/holographic.c
c/src/kernel.c
migration/epi-kernel/r4-holographic-kernel-parity.c
migration/epi-kernel/r4-vak-parity.c
migration/epi-kernel/r4-package-smoke.c
scripts/test-epi-c-r4.sh
```

The old shared fixture at that recovery head is blob `5112349d363a5c01e63418c91880af67c5e60ed1`. Compare it with current accepted fixtures; do not blindly copy the old fixture, docs, Makefile or workflow. Keep current contract/operator identities, the explicit M3 correction and accepted Rust/Vāk producer behavior. #76's reported green R4 run `32662287871` is historical recovery evidence, not a fresh current-main acceptance receipt.

K1 restores and proves the native C centre and foundational C/Rust parity. It does not port all M0–M5, implement higher-order native-C Vāk by implication, create the recursive registry, or turn the current Rust architecture into another kernel.

K2 consumes the source-faithful Rust M-coordinate/index, complete serialized corpus and current formal identities. K3 consumes the intact native matrix families, source warrants and correction protocol; K0's inventory does not implement its future machinery. K4 consumes those same pinned bodies and existing recursive compiler, adds authenticated live graph evidence, and produces the actual row-level census. The machine `handoff` names the exact input IDs for all four stages.

## 8. Acceptance meaning

K0 is complete when immutable inputs resolve, the human/machine standing account agrees, the source/correction/matrix inventories verify, the serialized access path executes, and K1–K4 can enter without rediscovering or silently discarding a source body.

That acceptance must continue to report **live graph observation: NOT PERFORMED**, **capability completeness: NOT ASSESSED**, and **K1 implementation: NOT STARTED**. These are explicit later scopes, not failed checks disguised as success. No broad kernel implementation is included in this change.
