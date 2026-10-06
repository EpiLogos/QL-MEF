//! The source-qualified phase bridge: explicit conversions between the three
//! native angle registers that coexist in the kernel.
//!
//! Three registers, three native step laws:
//!
//! 1. ENCODER REGISTER — `quat_codon_state` (`super::quaternion`, porting the
//!    vendor `m3_quat_codon_state`, `m3.c:124-134`, declared `m3.h:198`):
//!    the state angle is 45° per state and the rotor is built at the HALF
//!    angle (`cosf(angle * 0.5f)`, `m3.c:131`), so the encoder steps the
//!    PHYSICAL rotation 45° per state while the rotor's own quaternion-plane
//!    argument moves 22.5°.
//! 2. READER REGISTER — `quat_active_state` (`m3.h:200`; port
//!    `pole/quaternion.rs:247-254`): reads the composed quaternion-plane
//!    argument `atan2(x, w)` wrapped to [0, 2π) and bins it at 45° per bin.
//!    A bin is 45° of ARGUMENT = 90° of physical rotation, so the eight bins
//!    tile the 720° SU(2) double cover, not the 360° physical circle.
//! 3. CLOCK REGISTER — `M3Clock` (`crate::m3_clock`): unwrapped steps, one
//!    step = 1°; `degree720 = steps % 720`; `layer = degree720 / 360`;
//!    `tick12 = degree360 / 30` (m3_clock.rs:56-68). The 720° field is the
//!    SU(2) double cover — the first 360° the bimba sheet, the second the
//!    pratibimba sheet (deep M3 matrix §11, recorded in-repo at
//!    `docs/kernel-rebuild/m123-scene-map/m3-clock.md:16,144`; the layer-bit
//!    reading's execution evidence is `c/src/m3.c:146` and
//!    `m3_clock.rs:56-81`).
//!
//! The encoder and reader are DIFFERENT registers, not an encoder/decoder
//! pair: the encoder writes physical 45° steps through a half-angle rotor,
//! the reader quantises the argument register whose bins are 90° physical.
//! The conversion between them is `clock720 = 2 · signed_argument`
//! normalised to [0, 720) — physical rotation is twice the quaternion-plane
//! argument, the SU(2) half-angle law of the vendor construction itself.
//!
//! Derived relations (pinned by `tests/phase_bridge.rs`):
//!
//! - `quat_clock_steps(quat_codon_state(codon, s)) == (45 · s) mod 720`.
//!   The w-x subplane of a Hamilton product multiplies as complex numbers,
//!   so the general form is `(45·s + 2·seed_argument) mod 720` with
//!   `seed_argument = atan2(x_seed, w_seed)` of `quat_from_codon(codon)`;
//!   the published relation is exact for the argument-zero seeds (outer
//!   coin value == inner value, e.g. AAA), which the test pins.
//! - `quat_argument_bin(q)` and `quat_argument_bin(-q)` differ by 4 (mod 8)
//!   away from bin boundaries: negation shifts the argument by 180°, which
//!   is 360° physical — the sheet flip. At an exact boundary the two bins
//!   are `k` and `(k+4) mod 8`; a boundary a hair off in f32 can flip each
//!   floor by one bin, which is why interior and boundary cases are tested
//!   separately.
//!
//! The ring LUT and the clock: `RING_QUATERNION_LUT` (`pole/quaternion.rs:
//! 93-166`; vendor `m1.c:29`, declared `m1.h:515`) steps 60° of physical
//! rotation per tick (argument half-angle 30°/tick): ticks 0-5 ascend
//! 0..300°, ticks 6-11 return 660..360°, tick 11 is (−1, 0, 0, 0), the
//! spinorial second identity. The clock register covers the same 720° in 24
//! ticks of 30° — owner ruling D5, "One tick = 30°"
//! (`docs/kernel-rebuild/M123-SCENE-STRUCTURAL-MAP.md:146`), the ratio the
//! vendor header itself asserts (`m1.h:512`,
//! `TRIG_STEP_DEG == DEGREE_PER_TICK * 2`, with `TRIG_STEP_DEG 60`
//! `m1.h:497`, `DEGREE_PER_TICK 30` `m1.h:501`). One LUT tick therefore
//! spans two clock ticks: ascending ticks 0-5 sit at clock positions 2t and
//! 2t+1 (mod 24); return ticks 6-11 walk the same six 60° slots in reverse,
//! at clock positions `24 − 2(t−5)` and +1. The coexistence of the 30°/tick
//! and 60°/tick laws is RECORDED AS OPEN: ledger record
//! `gate0-m1:tick-degree-law`
//! (`docs/kernel-rebuild/m123-scene-map/m1-body.md:269-271`, state "open",
//! "Owner ruling needed"). This module cites that record and publishes the
//! conversion between the laws; it does not claim the ruling.
//!
//! This bridge is a conversion publication, not a new law: it adds no new
//! numeric behaviour to any existing function — every conversion here reads
//! the existing registers through their own recorded arithmetic.

// The pole modules are item-re-exported from lib.rs when wired; until this
// module is wired there, rustc sees no crate-internal consumer and
// dead-codes its API. tests/phase_bridge.rs compiles the module directly
// through the pole shim (tests/pole_rotational.rs pattern), so the lint is
// silenced module-wide, not item by item.
#![allow(dead_code)]

use super::quaternion::Quat;

/// Semantic identity of the source-qualified phase-bridge publication.
pub const PHASE_BRIDGE_REF: &str = "ql.pole.phase-bridge/v1";

/// The quaternion-plane argument `atan2(x, w)` of `q`, in radians in
/// (−π, π]. This is the register `quat_active_state` quantises and the
/// register in which the ring LUT's components are written.
pub fn quat_signed_argument(q: &Quat) -> f32 {
    q.x.atan2(q.w)
}

/// The explicit register conversion: physical rotation is twice the
/// quaternion-plane argument (SU(2) half-angle law — the vendor builds every
/// rotor at `angle * 0.5`, `m3.c:131`), normalised to the unwrapped M3
/// double-cover position in [0, 720) that `q` occupies, rounded to the
/// nearest step.
///
/// Provenance: the vendor m3.h/m1.h half-angle construction (`m3.c:124-134`;
/// LUT declared `m1.h:515`, defined `m1.c:29`); M3 matrix §11 — the 720°
/// field is the SU(2) double cover, first 360° bimba sheet, second 360°
/// pratibimba (`docs/kernel-rebuild/m123-scene-map/m3-clock.md:16,144`), the
/// clock side of which is the `degree720`/`layer` register
/// (`m3_clock.rs:56-64`); the D5 tick law governs the clock's own tick
/// register (`M123-SCENE-STRUCTURAL-MAP.md:146`).
pub fn quat_clock_steps(q: &Quat) -> u64 {
    let steps = quat_signed_argument(q) * (360.0 / core::f32::consts::PI);
    (steps.rem_euclid(720.0).round() as u64) % 720
}

/// `RING_QUATERNION_LUT` tick `t` → M3 double-cover position in steps:
/// `60·t` for `t` in 0..=5; `720 − 60·(t−5)` for `t` in 6..=11. Ticks past
/// 11 wrap the 12-tick loop, matching `quat_from_ring_pos`.
///
/// One LUT tick spans 60° physical (vendor half-angle 30°/tick: `m1.h:497`
/// `TRIG_STEP_DEG 60`, `m1.h:501` `DEGREE_PER_TICK 30`, ratio asserted at
/// `m1.h:512`); the LUT covers the 720° cover in 12 ticks while M3Clock
/// covers it in 24 ticks of 30° (D5,
/// `M123-SCENE-STRUCTURAL-MAP.md:146`): LUT tick `t` ≡ clock ticks `2t` and
/// `2t+1` (mod 24) on the ascending half; the return half walks the same six
/// 60° slots in reverse, at clock positions `24 − 2(t−5)` and +1. The
/// recorded open ledger record `gate0-m1:tick-degree-law`
/// (`docs/kernel-rebuild/m123-scene-map/m1-body.md:269-271`) holds the
/// 30°/tick vs 60°/tick question open for an owner ruling; this mapping is
/// cited against it, not ruled by it.
pub const fn ring_tick_clock_steps(tick: u32) -> u64 {
    let t = (tick % 12) as u64;
    if t <= 5 { 60 * t } else { 720 - 60 * (t - 5) }
}

/// The raw eight-bin classifier index of a quaternion-plane argument, i.e.
/// `quat_active_state`'s quantisation made total:
/// `bin = floor((arg wrapped to [0, 2π)) / 45°) & 7`. Exposed so tests and
/// callers can state the antipodal relation on the classifier itself without
/// re-deriving a codon. Mirrors the quantiser of `quat_active_state`
/// (`pole/quaternion.rs:249-253`) exactly; it classifies any quaternion,
/// including environments not composed from a codon.
pub fn quat_argument_bin(q: &Quat) -> u8 {
    let mut angle = q.x.atan2(q.w);
    if angle < 0.0 {
        angle += core::f32::consts::TAU;
    }
    ((angle / core::f32::consts::FRAC_PI_4) as u8) & 0x07
}
