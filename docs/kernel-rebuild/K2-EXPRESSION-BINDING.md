# K² Expression binding: native events to the live instrument, and Nara reception

Owner: #135 (M1–M3 engine effect), consumer O:I #335. Shared binding for the
parallel M4 identity work (#201). Implementation: `crates/ql-mef/src/continuous/k2.rs`,
`continuous/host.rs`, `adapters/retained-field/instrument-session.mjs`.

## What the engine receives

The K² played torus is the retained M1 topological body on which M2's eight
Vimarśā voices sound. One composed coupled event (M1, M2, M3 through the accepted
composer) decides the whole body; the K8 continuous owner integrates it; O:I's
retained GPU body and audio device present it. There is one numerical owner per
operation and no browser-side synthesis or geometry.

| Determinant | Through | Effect | Units / range | Timing | Warrant |
|---|---|---|---|---|---|
| M1 tick, lens, Context Frame; M3 pose | `m2.vimarsha.reading.audio_octet_hz[i]` | voice `i` pitch: PCM and surface oscillation | Hz, ~98–4000 | per determinant event | source-defined |
| same, plus M3 codon/rotation | `nodal_quartet[i % 4].{m,n}` | nodal lines of voice `i` on the torus | 1..12 | per event, explicit `replace-shapes` | mode numbers source-defined; torus reading of the plate term source-directed |
| M1 lens + tick % 6 | `derivation.mef_table_index` (address72) | phase of every surface term | `(address72+i+1)·π/36` | per event | source-defined |
| M1 torus `#1-5-1` | `m1::torus`, R = 16/9, r = 1 | rest body | torus units × `metres_per_unit` | per instance | source-defined |
| coupled clock | K8 native C clock | rotation of attached samples | half-degrees + winding | continuous | source-defined |
| material policy | damping, strike, gain, scale | decay, loudness, visible amplitude | 1/s, m, 1/m, m | per instance | **declared policy — not source** |

The surface term is the M2′ Chladni law on the torus with L = 2π
(M2-ARCHITECTURE §5.3.1): `χᵢ(φ,θ) = aᵢ sin(mᵢφ/2) sin(nᵢθ/2) + bᵢ cos(mᵢφ/2) cos(nᵢθ/2)`,
`aᵢ = wᵢ cos ψᵢ`, `bᵢ = wᵢ sin ψᵢ`, `wᵢ = fᵢ / max f`, displaced along the
outward normal. φ is toroidal, θ poloidal.

A native M1 advance (`m1-advance`) or a changed event (`replace-event`) re-issues
the event under the next M2 generation, recomposes it, optionally strikes the
voices from the declared strike amplitude, and — when the nodal quartet or
address changed — reshapes the same voices through K8 `replace-shapes`, which
keeps resident modal state, clock and PCM. The generation therefore rises by one
or two; the sample cursor does not move.

`ql.expression-influence/v1` (host operation `influence`) names every effect
above with the current values, the exact M1 revision, M3 generation, shape
reference and material standing. It is the Inspect-depth basis for people and
agents.

## Opening

`ql kernel k2-binding <request>` with `ql.k2-binding-request/v1`
`{schema, instance_ref, texture:[w,h], units_per_metre, event?, sky?, field?, geometry?, material?, reception?}`
returns `oi.native-expression-binding/v1` `{schema, host: ql.k2-expression-config/v1, presentation:{units_per_metre, slots_a, slots_b}}`.
No event means QL's default starting event (`fixtures/kernel/k2-default-event-v1.json`).
`sky` is an accepted `ql.sky-snapshot/v1` (from `ql-sky`); its ten bodies become the
M2 world observations. Particle `p` follows sample `p mod N` in both targets — a
stated correspondence. `ql-field-host` opens the `host` value unchanged.

## Nara reception (the shared binding for #201)

A K² owner may be one person's reception of the event: `host.reception` carries
that subject's K10 `PersonalConstitution`. Constitution subject, M3 subject and
field subject must be one. The host then admits:

- `receive-personal {input: PersonalEventInput}` — seven supplied centre inputs,
  each `{ordinal, m1, m2, m3: WorldContribution{basis_ref, source_ref, value}}`,
  citing the owner's **current** `EventBasisRefs` (event, profile generation,
  M1 revision, M2/M3 source refs). Stale or cross-event input is refused. The
  material field does not change.
- `personal {}` — `ql.k2-nara-reception/v1` `{event, state: PersonalFieldState|null, current}`.
  After any determinant event `current` is false until a new reception cites the
  new basis; an older reception is never relabelled current.

Protected personal state stays with the calling host: O:I's kernel writes the
config to a 0600 file for the host and returns only presentation and receipts to
the webview.

What the identity work supplies, and what remains open:

1. **Constitution** — identity/transit/activity layers, EarthBody frame and seven
   receiver constitutions (`world_weights`, `orientation`, gains). The rule from
   identity evidence to these values is #201's to recover or record as open.
2. **Centre contribution rule** — how the current event's native values become
   each centre's M1/M2/M3 `WorldContribution.value`. No source defines it yet;
   K8 and K10 never manufacture it. Until #201 recovers it, inputs are supplied
   and labelled with their standing.
3. **Engine presentation of centres** — centres are receivers, not cymatic
   stations. How a centre's resonance is seen or heard in the K² body is a
   presentation decision for the O:I consumer with the owner.

Evidence: `tests/k2_nara_reception.rs` — two differently constituted Naras over
one event and one sounding body produce distinct, independently varying
seven-centre states; a determinant event makes only that owner's reception stale
and refuses its old basis.

## Open design questions (owner)

- **Sky → condition.** LIVING-INSTRUMENT-ARCHITECTURE §3.2 directs "Sky → M2
  planetary/decan/aspect condition → source-backed musical/elemental/chakral
  relations → M1–M3 current composition", and M2-ARCHITECTURE makes maqam
  transitions "Kerykeion + planetary-hour driven". Today the dated sky reaches
  only M2 world readings (decans, aspects); the condition's maqam, and so the
  voices' element, colour and condition pitches, is still supplied by the event.
  Measured 27 Sep: the eight voices at the current sky equal those of the
  sky-less default event. Which rule selects the condition — the planetary-hour
  ruler (needs an observer and sunrise), the Sun's decan ruler, or another — is
  not fixed by source and is not invented here.

- Material policy: damping (0.35/s), strike (0.08 m), strike on each event, gain
  (1/m) and scale are declared defaults, not source.
- Tick cadence: PPS gives 12 ticks/s as the user-facing rate; M4′ gives a 1 Hz
  world clock. The instrument offers both; neither is fixed by the M1 contract.
- Odd `m` makes the L = 2π term anti-periodic in φ (a seam at φ = 0). This fits
  the spinor double cover of the carrier but no source says so; the deep sources
  also give two other shape forms (wavenumber = Hz; a 72-sample ring).
- M2's 72 modal coefficients are a supplied operand with no producer; the
  default event carries the accepted acceptance values.
- M3 physical form, codon annulus and clock wheel are not yet material in the
  body; they remain inspectable readings.
