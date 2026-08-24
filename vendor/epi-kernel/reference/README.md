# Frozen Epi C reference

This directory is an **unchanged reference specimen**, not a fork in which new computation is developed.

Source:

- repository: `EpiLogos/Epi-Logos-C-Experiments`
- revision: `daa660cbc1b8c5da83828698665a753852cb0287`
- original root: `Body/S/S0/epi-lib`
- include tree: `f2b27d99197ee0f1cb9ed95ef52a5dd61a226e54`
- source tree: `a60dcda1427a6ab3cfcd44565a29f988938d0881`
- historical test tree: `9a6ef6505bb4e7622dba0922a07dced9bc49cd79`

The full source lock and individual header/source blob identities live in `migration/epi-kernel/source-lock.json`.

## Rules

- Do not edit files in `include/` or `src/` to make a new implementation pass.
- A new upstream reference revision requires an explicit source-lock transition.
- Generalized/native work belongs outside this frozen directory and is compared back to it.
- C is the computational reference and intended computational centre; this directory is not a staging area for an automatic Rust port.
- `portal-core` is not vendored here because it is a secondary implementation witness, not the computational source being promoted.
- The historical test tree is locked by Git tree identity but is not bulk-vendored in R0. R1 should import or reproduce relevant source tests deliberately and record their provenance rather than carrying historical binaries indiscriminately.
- **`src/qv_data.c` is frozen projection evidence, not semantic authority.** Its descriptive payload contains known semantic drift and must not determine coordinate-family meanings or T/T′ vocabulary. Read `docs/integrations/epi-logos/EPI-C-QV-DATA-AUTHORITY-GUARD.md` before using QV material in migration or semantic recovery.

Run `scripts/sync-epi-c-reference.sh --check /path/to/Epi-Logos-C-Experiments` to verify the committed headers/source against the locked Epi commit.
