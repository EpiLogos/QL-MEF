# M1–M3 live Expression: grounding audit (27 September 2026)

**Standing:** agent-derived audit of the Bimba graph (Epi-Logos-C-Experiments
`daa660c`, as compiled in `fixtures/kernel/m-tree-v1.json`) and the C/Rust kernel,
made after the owner found that the K² work on #135 (QL-MEF #251, O:I #545) was
built from engine entry points and contract prose. It records checked structure
and known defects for the next session. It is not authority: where it and the
owner or the graph differ, they win. The owner has already corrected one reading
(the clock, below).

## What failed

The K² provider treated every engine "supplied input" as unconstrained and
invented policy there. It read no node properties or typed relations. Results:
voices independent of the dated sky; M2's own sound (planetary octave, maqamat)
silent; colour "unavailable"; Nara centres unnamed with an invented
M1/M2/M3-triple input; an M3 "1°/s" rotation; a surface seam; a lattice-blind
64×64 sample grid. The infrastructure beneath it (installation, K8 explicit
reshape, worker I/O, compose/kernel ops, causal tests) is sound and reusable.

## M2 — the sky is M2-5

- `#2-5` Planetary Harmonic Integration: Sun `#2-5-0/1` (holds Earth
  `#2-5-0/1-0` and chakras `-1` Mūlādhāra … `-7` Sahasrāra), Venus `#2-5-2`,
  Mercury `-3`, Moon `-4`, Saturn `-5`, Jupiter `-6`, Mars `-7`; Neptune `-8`
  and Pluto `-9` exist only in the stragglers file with no relations; no Uranus
  node. `chaldeanOrderVerified` = Sun→Venus→Mercury→Moon→Saturn→Jupiter→Mars =
  the coordinate digit order; `musicalCosmology` makes the digit the scale
  degree (just octave 1:1, 9:8, 5:4, 4:3, 3:2, 5:3, 15:8, 2:1) with
  `planetaryMode` and `modalSignature` letters per planet.
- Decans `#2-3-e-s-d` (`degreesRange`, light/shadow faces via `HAS_ASPECT`):
  `RULED_BY` / `HARMONICALLY_RESONATES_WITH` (`harmonicInterval`) /
  `SPANDA_TEMPORAL_RHYTHM` → Chaldean ruler. Shem names `#2-4.5-c-p` (5° arcs)
  → planet with `planetaryColor`, mode prose; `EXPRESSES_THROUGH` → decan face.
- Maqamat `#2-4.3`: `TONIC_/DOMINANT_PLANETARY_RESONANCE` → planet exactly by
  the note letter = planet `modalSignature`; chromatic notes via
  `TONIC_CHROMATIC_BRIDGE` / `DOMINANT_CHROMATIC_TENSION`.
- Planet → chakra `PLANETARY_RESONANCE`: Saturn→1, Jupiter→2, Mars→3, Venus→4,
  Mercury→5, Moon→6, Sun→7 (with `modalContribution`, `timingOptimal`); Sun
  `CHAKRAL_VIRTUE_RECEPTION` → all seven; Earth `FEEDS_EARTH_ELEMENT` /
  `GROUNDS_CHAKRAL_PATHWAY` → Mūlādhāra; `ASCENDS_TO` 1→…→7.
- Owner's M2′ spec §9.2 routes the moment through the planetary-hour ruler
  (needs an observer); maqam within the ruler's set is "intent + current
  lens-mode".
- **Defects:** retained C tables read by `m2_engine` disagree with the graph —
  decan rulers (triplicity vs Chaldean faces), `M2_PLANET_LUT.elem_sig`
  (planet→chakra: Moon, Jupiter, Sun differ), `M2_SHEM_DESC.planet_link`
  (synthetic cycle), maqam planet ruler. Cancer decan 3 property (Moon) vs
  relation (Saturn). 26/72 `EXPRESSES_THROUGH` edges contradict 5° geometry.
  `m2-correspondences-v1.json` ignores the chromatic relations.

## Nara — the centres are M2-5's chakras

- Nara's seven centres are `#2-5-0/1-1…7`; EarthBody is `#2-5-0/1-0` (distinct,
  "not the source of chakras"). No typed relation joins `#4*` to `#2-5*`; Nara
  holds a person's state over them (`#4.1-1` `chakraState`, context frame
  "4.0-4/5" shared with the chakras).
- **Defects:** `nara.rs` says source lacks chakra names (false); receiver
  ordinals 0..6 are off by one against `-1…-7`; per-centre `{m1,m2,m3}` inputs
  have no source basis — no M1 or M3 node relates to a chakra; reception is M2
  routing (own planet, decan ruler, maqam→planet→chakra, Sun reception, Earth).
- Magnitudes (weights, gains) are given by no source; identity layer weights
  exist only in seeds (`nara-m4-0-identity-branch-integration-map.md`).

## M3 — the clock is a codon–hexagram construction (owner correction)

- M3-5 is the totalised clock: the 64-codon charge integral closes to 360
  (1440/4; suits 84/96/92/88), 720 double cover (bimba / pratibimba), 24-fold
  backbone at 15°, `CLOCK_DEGREE_LUT[360]`; 390 graph nodes (wheel, 4 cardinals,
  24 governors with prototype codon/hexagram, Axis Mundi `#3-5-5/0`,
  Alpha-Omega, 359 degree nodes with `rotationalPhase`, `FLOWS_CLOCKWISE`,
  `POLAR_OPPOSITE`, `GOVERNS_DEGREE_ARC`, `ANCHORED_BY`).
- The sixteen static aperture lenses (1°×360 … 360°×1, reciprocal pairs p↔15−p)
  plus the Fibonacci/Pisano 60×6° ground and the 16+1 fold-aperture (22.5°) are
  **readings of the same inscription**; the 30°×12 zodiac is lens 9 only. The
  whole is K² × T²_Mahāmāyā (inscription circle × lens circle). Source:
  `docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md` §11.
- The live scene builds the actual degree field and places real ephemeris
  planets around it; astrological partitions are lenses over it.
- **Defect:** the retained C degree table carries hexagram/line/pair columns as
  *estimates* ("computed hexagram approximation (no Neo4j)"); `ql-core`
  `m3_clock.rs` rightly refuses to promote them. The degree → codon/hexagram
  inscription has not been reconciled from the M3-5 graph into the kernel.
- M3 form is a fold body (`ql_m3_form`: crease sites, hinges, 472 poses); the
  fold solver and its embedding with K² are undetermined in source.

## M1 — the body

- `#1-5-1` torus (standard embedding; R/r = 16/9 from the 64/36 percentile
  identity; portal uses R+r = 1); `#1-5-2` 4π double cover; `#1-5-3` shadow as
  phase shift. Longitude = chromatic circle, meridian = circle of fifths / tick
  circle — a 12-fold lattice; the Ananda 12×12 field is the torus texture.
  Specified motion: K² orientation slerp (orbit: generated carrier vs legacy
  ring LUT — undetermined); Hopf shadow sheet carries anti-periodic content
  (a single sheet admits only periodic harmonics). Tick rate is a declared band;
  Vimarśā is correctly the audio bus.

## Genuinely undetermined (owner rulings needed)

M1↔M3 tick law (degree/30 vs 60°/tick); orientation orbit; maqam within the
ruler's set; how M2 pitches join the Vimarśā octet (§9.8 audio profile);
magnitudes (centre weights, damping, gains, metres); Uranus/Neptune/Pluto
routes; fold solver and K²×T² embedding; which planet's 72-index feeds the M3
address; degree→inscription reconciliation where graph and C disagree.
