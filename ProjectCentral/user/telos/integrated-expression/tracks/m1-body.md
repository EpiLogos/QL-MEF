# Track: the M1 body and oscillator

Why: M1 is the generator. The Spanda dual oscillator is
`φ̇ = Δω − a·sin φ − 2b·sin 2φ`, with a = 1, b = 9/16 and a beat of about
2.5 Hz. The `#1-5` torus (16/9) is a chromatic × fifths 12-fold lattice, and
θ = ecliptic degree.

Work:
- port the oscillator natively; its phase and beat are the carrier;
- the tick is 30° (D5);
- carry the two Hopf sheets with the SU(2) sign that C++ drops today;
- rule and apply the #254 m1 records (D16, D17).

Depends: map-and-kernel. Done-when: the native oscillator matches the C
reference, and codons advance on the Spanda state, never on `tick12`.

State (28 Sep): the Spanda field is native (`ql_mef::spanda_field`, #264), with
C parity over 2,029 observations. Codons advance on the ring state
(`M3Operation::SpandaAdvance`). Still open: the two Hopf sheets with the SU(2)
sign in C++, and the D16/D17 records.
