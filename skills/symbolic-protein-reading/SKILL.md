---
name: symbolic-protein-reading
description: "METHOD: Mythos's in-session reading of the M4 symbolic-protein codon arc as a provisional archetype under the current M1/M2/M3 weather, with four provenance links and a guard against reifying the pattern into identity. Use when a kairos pulse calls for a pattern reading of a session and a governed chain projection is actually available; otherwise return the named gap."
---

# Symbolic-protein reading

## Contract metadata

- Semantic ref: `ql:skill:symbolic-protein-reading` (`skill/ql/symbolic-protein-reading`)
- Native owner of the reading law: `EpiLogos/QL-MEF` (M4 Nara domain; weather from the M1/M2/M3 kernel and the K8 sky boundary `providers/sky/kerykeion_snapshot.py`, contract `docs/kernel-rebuild/K8-SKY-CONTRACT.md`).
- **Native reader: to build.** The source's `modules/symbolic-protein-reader.ts` has no current native counterpart; see `docs/integrations/epi-logos/ANIMA-ALETHEIA-TEAMS.md` (tool crosswalk) for its specification. Until it lands, this Method stops at the gap.
- Source: `Body/S/S4/ta-onta/S4-4p-anima/S4'/skills/symbolic-protein-reading/SKILL.md`, HEAD blob `de90247808e439b542c713a245dd52ed54a0a69f` (not in the AW0 pin set), with `voice-templates/default.md`; owner per `S4-4p-anima/CONTRACT.md`: Mythos.
- Used by: `agent/anima-mythos` (owner). Frame: CF4 `(0/1/2/3)`, CT3, CP 4.3, CFP0 (CFP3 only when several pattern readers are explicitly fused).

## Inputs (governed only)

- `chain_projection`: chain position and fingerprint — the only chain-derived input admitted; never codons or the protected protein body.
- `weather`: the global 1-2-3 state — M1 spanda tick, M2 cymatic phase, M3 codon-transcription state grounded in live sky degrees (a `ql.sky-snapshot/v1` from the K8 sky boundary).
- Four provenance refs: session, chain-position bookmark, weather snapshot, triggering kairos pulse (from `agent/aletheia-mercurius`).

## Procedure

1. Confirm a governed chain projection exists for this session. If not, return `unavailable: no native symbolic-protein reader or governed chain projection` and stop. Do not read protected M4 bodies to substitute.
2. Obtain the weather with its provenance; never read the chain without it. The same chain may name a different archetype under different weather — that sensitivity is required.
3. Name one dominant Major Arcana archetype and up to two secondary patterns, provisional.
4. Write the reading in the voice below and attach all four provenance refs.
5. Record it in the active NOW as a `T3` pattern thought (`central.now.thoughts.append`, `actor` `agent/anima-mythos`), not in any protected store.

## Voice

One paragraph, paraphrasable as: *This session's codon arc figures archetype {dominant} under cosmic weather M1 {tick}, M2 {phase}, M3 {transcription}; secondary patterns {secondary} remain provisional and provenance-bound at {session}, {chain position}, {weather snapshot} and {kairos pulse}.*

Say "figures", "suggests", "names", "shows as a provisional pattern". Never "this session is X". Do not collapse the archetype into identity, destiny, diagnosis or final truth.

## Output

```ts
interface MythosArchetypeReading {
  dominant_chromosome_arcana: string;
  secondary_pattern_arcanas: string[];
  narrative_summary: string;
  provenance: { session_ref: string; chain_position_ref: string; weather_snapshot_ref: string; kairos_pulse_ref: string };
}
```

## Limits

A pattern without its four provenance links is noise, not Mythos. The reader does not own protected M4 bodies, the M3 card table, configuration parsing, or persistence.
