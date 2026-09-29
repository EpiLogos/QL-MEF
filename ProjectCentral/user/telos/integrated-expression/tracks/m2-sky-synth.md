# Track: M2 as the synth driven by the live sky

Why: M2 is the sky (`#2-5`), and M2 is the sound design. It shapes M1's
carrier; it is not a second source. The signal chain is in #254 "The M2 signal
chain".

Work:
- `WorldObservation` keeps speed, retrograde, and sunrise/sunset for the observer's place.
- The hour ruler comes from the Chaldean order.
- Maqam selection goes through `TONIC/DOMINANT_PLANETARY_RESONANCE`.
- The Vimarśā degrees are tuned to the planetary just octave.
- Filters and resonators come from the lenses and apertures.
- A tattva → material table (proposed, tunable).
- The `nodal_quartet` drives the cymatic skin; port the Chladni solver from C-Experiments.
- One tunables table, where every magnitude carries its standing (D30).

Depends: map-and-kernel, m1-body (carrier). Done-when: moving one planet's
longitude changes exactly the predicted modulation, filter and pitch targets,
checked in C++ state readback and PCM.

State (29 Sep):
- The sky bus is on main (#270). Observed planets voice modes at M1's root times their map just ratio.
- `fixtures/kernel/m2-sky-v1.json` carries M2-5's map values.
- `ql scene compose` gives one integrated event from a dated sky.
- Still open: maqam and hour ruler selection from the map relations, and tattva → material.
