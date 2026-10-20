# L5 Technē Instrument Surfaces — refinement home

The seven Technē apertures of the **one M′ field with two readings**
(`docs/L5-TECHNE-DUAL-READING-LOCK.md`), in the QL-MEF system where the
L5 / Para Vāk articulation lives, so surface refinement happens next to the
contract, the Vāk grammar and the conformance fixtures.

| Aperture | Office | Reading |
|---|---|---|
| `project` | Project / Wiki / Knowledge Map — ground, step 0 | 4:2-deep |
| `canvas` | Canvas / Constellation — authored spatial material | 4:2-deep |
| `timeline` | Relation Field — movement, causation, recurrence; chronology its strongest projection | 4:2-deep |
| `journey` | Journey / Scenes — formal mediation, the hinge into 3:3 | 4:2-deep |
| `place` | Places / World — situation; the solar→Earth→geography→Nara frame chain | 4:2-deep |
| `palace` | Palace / Integral Whole — articulation, memory, Return | 4:2-deep |
| `expressions` | the conjugate **3:3 Expression reading** (instantiation, composition, M′ embodiment) | 3:3-conjugate |

The instruments carry no M′ numbers: the owner withdrew the four-and-two reading on 19 Oct 2026 (the lock's §0), and
`ql.techne/v1` no longer carries `m_prime`. The reading's wire names (`4:2-deep`, `3:3-conjugate`) stay because the
cradle's copy of the contract validates them. The per-instrument agent states here still name their roles by coordinate
(`techne_1_binding` on the Canvas, `TECHNE_3_ROLE` on the Journey, `technae_0` on the Project, the Technē_2 timeline
fixture); O:I's cradle copy does the same. Both are renamed together under the sync law below (O-I
`desktop/cradle/docs/world-shell/wayfinder/tickets/T-AGENCY-the-agent-world-and-the-construction-chain.md` K9).

## Layout

```text
cradle/                 the surface tree in Cradle layout (imports resolve identically)
  src/techne/**         the surfaces: contract, session, adapter, host, one directory per instrument
  src/{surface,expression,instrument,stage,workspace}/**   verbatim host-closure copies the tree composes with
  src/kernel/**         documented port bindings (type subset + transport binding; canonical files in O-I)
  tests/                the techne suite (node --test; register tests ride vite SSR)
  package.json          standalone install/typecheck/test
vendor/oi-design-system/ the vendored Technē slice of the O:I design system (techne.css chrome tier, theme derivation, surface lifecycle)
```

## Provenance and the sync law

Ported 2026-09-16 from EpiLogos/O-I branch `techne/convergence`
(`8685da40` + `ec3100c1`, Cradle `desktop/cradle/src/techne/**`), grounded on
TB0 `techne/tb0-connective-base` @ `2c704dc`, contract tag `ql.techne/v1`.

- `cradle/src/techne/**` is **verbatim** — refine here, then port back to the
  runtime host (O:I Cradle) through the TB0 fan-out lanes (#213–#219) or a
  direct sync; the contract itself changes only through #212 with a
  compatibility statement. One reading, one tree — no private fork.
- `cradle/src/kernel/**` are **port bindings** (type subset + transport
  binding), banner-marked; the canonical kernel transport stays in O-I.
- `vendor/oi-design-system/` is the design-system **slice**; the canonical
  package stays in O-I `packages/oi-design-system`.

## Commands

```sh
cd adapters/techne/surfaces/cradle
npm install
npm test          # 130 techne tests, including the conformance fixtures
npm run typecheck # strict tsc over the whole ported tree
```

Contract-level checks live at the repo root: `cargo test -p ql-adapters`
(schema/fixture law, agency and cut law, cross-cut identity) and
`python3 scripts/techne-gates.py` (G0–G7 evidence runner).
