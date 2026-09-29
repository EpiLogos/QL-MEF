# The `#` → `M` spelling migration

**Standing:** plan, not yet executed. Law: `skills/ql-law/SKILL.md` (the prefix is `M`; after a 4 the separator is `.`).

## Where things stand

- The Bimba map already spells `M2-4.3`. Only the QL registry stores `#2-4.3`. It gets there through one function, `scripts/bimba_map.py` `ql_spelling`, which turns `M` into `#` and drops the context-frame brackets. The exact map spelling survives only as an alias when it has brackets.
- Lookup already accepts `M`:
  - C `c/src/m_tree.c` (`#` or `M`);
  - Rust `crates/ql-mef/src/m_tree.rs`, which maps `M…` to `#…`;
  - `scripts/m-ledger.py`, whose `COORDINATE` accepts either prefix.
- Bare `#` is not this prefix. It stays unchanged in every role: the kernel address/taproot (`c/src/holographic.c`, `aw1_world.rs` `KERNEL_TAPROOT_REF`), and the map's own `#`, `#0`…`#5` meta nodes (read as `bimba:#…` endpoints).
- There are 248 registry coordinates that break the nesting threshold. They are recorded as `k2-grammar-nesting-threshold:*` ledger discrepancies and corrected in the map by `migration/bimba-map/2026-09-29-nesting-threshold.cypher`. That migration has been dry-run only and needs the owner's OK.

## Sequencing: one registry move, not two

Node IDs are `sha256(domain, spelling)`, and `registry_revision` is pinned in 23 fixtures. The M2/M3 finite proofs pin the registry, and the census pins the ledger. The map correction already moves 248 spellings, and with them their IDs and every revision. Doing the `M` migration separately would move every pin and successor proof a second time. So both land in **one** refresh, after the owner applies the map correction:

1. **Owner applies the map correction.** Run `python3 migration/bimba-map/run.py migration/bimba-map/2026-09-29-nesting-threshold.cypher --commit`. Then read the map again and confirm the 248 are gone: `scripts/coordinate-grammar.py map` should report 0.
2. **Spelling change** in one commit:
   - `ql_spelling` becomes the identity. The registry `source_ref` is the exact map spelling, brackets included (`M2-5-(0/1)-1`).
   - The old `#` form (brackets dropped) becomes an alias, so recorded `#` references still resolve through `by_alias` / the C alias field.
   - Change `generate-m-tree.py` `COORD` and the lexical-parent regex to `M[0-5]…` and accept `(`…`)` segments. Change `ref[1]` root-index reads to the same offset (the prefix is still one character).
   - `m_census.py` `normalize_spelling`, `compile-epi-bimba-map*.py`, `k8-structure.py` `validate()` literals, `m2-ledger.py` / `m3-ledger.py` / `m3-source-parity.py` / `generate-m3.py` / `check-pre-k8-lock.py` / `aw0.py` / `migration/bimba-map/generate_d0_and_coin.py`: spell `M`.
   - Rust: `m_map.rs` `ParsedSourceCoordinate::parse` (accept `M`, drop the `replacen`), `m_tree.rs` lookup (map legacy `#` to `M`), and the `format!("#…")` / `starts_with("#…")` sites:
     - `m1.rs`, `m1_engine.rs`, `m3_engine.rs`, `m3_inscription.rs`, `scene_sky.rs`, `epi_agent.rs`, `ql-cli/src/epi_agent_command.rs`.
     - Find them with `rg '"#[0-5]|#\{|starts_with\("#' crates`.
   - C: `migration/epi-kernel/k2-m-tree-probe.c`, `k8/native.c`.
   - Tests: `scripts/tests/test_m_tree.py` (fixed counts, spellings), `crates/ql-mef/tests/native_m_tree.rs`, plus the remaining `scripts/tests/*` hits.
   - `scripts/coordinate-grammar.py`: once `source_ref` is the map spelling, `map_spelling()` returns it directly.
3. **Refresh and re-pin:** `scripts/refresh-from-bimba-map.sh --endpoint http://100.92.62.101:7474` (generator, K8 structure, M2 catalogue/correspondences/field report, M3 source parity, M3 domain). Then `python3 scripts/m-ledger.py refresh`.
4. **Ledger review:** move the `k2-grammar-nesting-threshold:*` records to `applied`, with the migration and the new map read as evidence. `coordinate-grammar.py check` fails on any live record whose subjects have left the registry, so this step cannot be skipped. Then run `python3 scripts/coordinate-grammar.py check`.
5. **Successor proofs from Linux CI artifacts**, never from a macOS run:
   - `python3 scripts/m2-ledger.py publish` and `python3 scripts/m3-ledger.py publish`, run against the CI acceptance artifacts (`target/m2-receipt/`, `target/m3-acceptance/`) of the refreshed head;
   - then `python3 scripts/k8-census.py --write-receipt`.
6. **Gate:** `scripts/verify full` on CI. Check that `git diff --exit-code` is clean after the invariants step.

## Consumers outside this repo

- O:I and the `ql` CLI read `ql:m-coordinate:M…` refs, which `m_map.rs` already emits with `M`. They are unaffected apart from the 248 respelled coordinates.
- Before step 2, grep O:I for literal `#[0-5]` coordinate strings.
- `ProjectCentral/user/ql.html` shows a `#` label. Change it with step 2.

## Who carries it

The M1–M3 integrator lane (`m123-engine`) owns in-flight registry work. This migration lands after that lane's open ledger changes, or is rebased onto them: the grammar records are regenerated deterministically by `coordinate-grammar.py record`.
