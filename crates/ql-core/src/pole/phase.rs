//! The source-qualified phase bridge: explicit conversions between the
//! native angle registers that coexist in the kernel.
//!
//! Three registers, three native step laws:
//!
//! 1. ENCODER REGISTER — `quat_codon_state` (`super::quaternion`, porting the
//!    vendor `m3_quat_codon_state`, `m3.c:124-134`, declared `m3.h:198`):
//!    the state angle is 45° per state and the rotor is built at the HALF
//!    angle (`cosf(angle * 0.5f)`, `m3.c:131`), so the encoder steps the
//!    PHYSICAL rotation 45° per state while the rotor's own half-angle moves
//!    22.5°. Unchanged by the #312 §5 amendment.
//! 2. READER REGISTER — `quat_active_state` (`m3.h:200`; port
//!    `pole/quaternion.rs`): CORRECTED by the #312 §5 amendment to the
//!    vendor's all-axis law (`m3.c:136-152`): the full rotation angle about
//!    the composed axis, `2·atan2(|v|, w)` with `|v| = sqrt(x²+y²+z²)`,
//!    normalised to [0°, 360°] PHYSICAL and binned at 45° PHYSICAL per bin —
//!    eight bins tiling the 360° circle. All three matrix axes (i, j, k)
//!    contribute through `|v|`. The predecessor read the i-only half-angle
//!    `atan2(x, w)` — 45° of argument = 90° physical, eight bins tiling the
//!    720° double cover — and is RETIRED: it is retained as
//!    `quat_active_state_retired_i_plane`, `quat_argument_bin_retired_i_plane`
//!    and `quat_clock_steps_retired_i_plane`, the regression witness of the
//!    predecessor register (#312 §5), not a live conversion.
//! 3. CLOCK REGISTER — `M3Clock` (`crate::m3_clock`): unwrapped steps, one
//!    step = 1°; `degree720 = steps % 720`; `layer = degree720 / 360`;
//!    `tick12 = degree360 / 30` (m3_clock.rs:56-68). The 720° field is the
//!    SU(2) double cover — the first 360° the bimba sheet, the second the
//!    pratibimba sheet (deep M3 matrix §11, recorded in-repo at
//!    `docs/kernel-rebuild/m123-scene-map/m3-clock.md:16,144`; the layer-bit
//!    reading's execution evidence is `c/src/m3.c:146` and
//!    `m3_clock.rs:56-81`). Unchanged by the amendment.
//!
//! The primary conversion is now `quat_rotation_degrees`: the physical SO(3)
//! rotation a bare quaternion carries, read on the corrected register. A bare
//! quaternion has no SHEET: the 720° double-cover position is traversal
//! history the M3Clock owns, not a property of a bare quaternion (q and −q
//! are one rotation; the reader's choice between θ and 360° − θ is only the
//! hemisphere the representative's `w` sign puts it in, not a different
//! rotation).
//!
//! Derived relations (pinned by `tests/phase_bridge.rs`):
//!
//! - Encoder/reader agreement on the ascending half: for an argument-zero
//!   seed (outer coin value == inner value, e.g. AAA, seed = (Σ, 0, 0, 0)) the
//!   state rotor `quat_codon_state(codon, s)` classifies to bin `s mod 8` —
//!   the encoder's 45°-per-state physical steps and the corrected reader's
//!   45° physical bins are the SAME register, and the 2π edge folds back to 0
//!   (the `&0x07`, per the vendor's own comment). The retired law read the
//!   same rotors at half value; that disagreement is the correction, not a
//!   defect.
//! - THE MIRROR, NOT +4: for rotors interior to the 45° grid,
//!   `quat_argument_bin(−q) = (7 − quat_argument_bin(q)) mod 8` — the octant
//!   mirror n → 9−n. Proof: `|v|` is negation-invariant and
//!   `atan2(|v|, −w) = π − atan2(|v|, w)` for `|v| ≥ 0`, so the corrected
//!   angle mirrors, `φ(−q) = 360° − φ(q)`; a point interior to bin k mirrors
//!   into the interior of bin 7−k. At an exact 45° multiple the mirrored
//!   angle is also exact (`45·(8−k)`) and the floor convention reads bin
//!   `(8−k) mod 8`; a boundary a hair off in f32 can flip each floor by one
//!   bin, which is why interior and boundary cases are tested separately.
//!   The retired law's +4 shift (180° of i-plane argument, doubled into a
//!   sheet flip) belongs to the retired register only.
//! - Retired register (witness): `quat_clock_steps_retired_i_plane` publishes
//!   the predecessor conversion `clock720 = 2·signed_argument` normalised to
//!   [0, 720); `quat_codon_state(codon, s)` reads `(45·s + 2·seed_argument)
//!   mod 720` there — the w-x subplane of a Hamilton product multiplies as
//!   complex numbers, with `seed_argument = atan2(x_seed, w_seed)`; exact for
//!   the argument-zero seeds the retired test pins.
//!
//! The ring LUT and the clock: `RING_QUATERNION_LUT` (`pole/quaternion.rs`;
//! vendor `m1.c:29`, declared `m1.h:515`) steps 60° of physical rotation per
//! tick (argument half-angle 30°/tick): ticks 0-5 ascend 0..300°, ticks 6-11
//! return, tick 11 is (−1, 0, 0, 0), the spinorial second identity. The
//! CORRECTED rotation reading of the LUT is direction-blind — `|v|` cannot
//! tell rotation by θ about n̂ from rotation by θ about −n̂ — so the return
//! ticks 6..11 read the same magnitudes as ticks 1..5 (`60°·(t−5)`), with
//! tick 11 at `atan2(0, −1) = π → 360°`, the folded edge. The clock register
//! covers the same 720° in 24 ticks of 30° — owner ruling D5, "One tick =
//! 30°" (`docs/kernel-rebuild/M123-SCENE-STRUCTURAL-MAP.md:146`), the ratio
//! the vendor header itself asserts (`m1.h:512`,
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
//! This bridge is a conversion publication, not a new law: every conversion
//! here reads the existing registers through their own recorded arithmetic.
//! The #312 §5 amendment re-points the reader-side conversions at the
//! corrected register and retires (without deleting) the i-plane ones.

use super::quaternion::Quat;

/// Semantic identity of the source-qualified phase-bridge publication.
pub const PHASE_BRIDGE_REF: &str = "ql.pole.phase-bridge/v1";

/// The i-plane signed argument `atan2(x, w)` of `q`, in radians in (−π, π].
/// This is the register the RETIRED reader quantised and the register in
/// which the ring LUT's components are written; the corrected reader
/// (`2·atan2(|v|, w)`) does not read it. Retained for the retired witness
/// conversions.
pub fn quat_signed_argument(q: &Quat) -> f32 {
    q.x.atan2(q.w)
}

/// The corrected conversion: the physical SO(3) rotation angle `q` carries,
/// `2·atan2(|v|, w)` with `|v| = sqrt(x²+y²+z²)`, in degrees, rounded, in
/// [0, 360].
///
/// The bare quaternion carries the SO(3) angle only: the 720° double-cover
/// position (which sheet the reading sits on) is traversal history the
/// M3Clock owns, not a property of a bare quaternion — q and −q are ONE
/// rotation, and the register's choice between θ and 360° − θ follows only
/// the hemisphere of the representative's `w`. The LUT's tick 11 (−1, 0, 0, 0)
/// reads 360 here: `atan2(0, −1) = π`, the folded 2π edge (its bin folds to 0
/// through the quantiser's `&0x07`).
///
/// Provenance: vendor `m3.c:136-152` (the corrected `m3_quat_active_state`,
/// #312 §5 amendment); the SU(2) half-angle construction it doubles
/// (`m3.c:131`); M3 matrix §11 for the sheet/clock relation
/// (`docs/kernel-rebuild/m123-scene-map/m3-clock.md:16,144`).
pub fn quat_rotation_degrees(q: &Quat) -> u16 {
    let vmag = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
    let degrees = (2.0 * vmag.atan2(q.w)).to_degrees();
    degrees.round() as u16
}

/// RETIRED (witness): the i-plane conversion `clock720 = 2·signed_argument`
/// normalised to [0, 720), rounded to the nearest step — the same body the
/// bridge published as `quat_clock_steps` before the #312 §5 amendment.
///
/// The retired register read the FULL SU(2) double-cover position from a bare
/// quaternion's i-plane argument; the corrected register reads the physical
/// rotation (`quat_rotation_degrees`) and leaves the 720° position to the
/// M3Clock's own traversal history. Used only by the retired witness tests;
/// the primary conversion is `quat_rotation_degrees`.
pub fn quat_clock_steps_retired_i_plane(q: &Quat) -> u64 {
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

/// The raw eight-bin classifier index under the CORRECTED all-axis law, i.e.
/// `quat_active_state`'s quantiser made total:
/// `bin = floor((2·atan2(|v|, w) wrapped to [0, 2π)) / 45°) & 7`. Exposed so
/// tests and callers can state the antipodal relation on the classifier
/// itself without re-deriving a codon. Mirrors the quantiser of
/// `quat_active_state` (`pole/quaternion.rs`) exactly; it classifies any
/// quaternion, including environments not composed from a codon.
///
/// Antipodal law (interior rotors): the octant MIRROR,
/// `quat_argument_bin(−q) = (7 − quat_argument_bin(q)) mod 8` — see the
/// module derivation. The retired i-plane quantiser's +4 is this function's
/// predecessor's diagnostic, retained on
/// [`quat_argument_bin_retired_i_plane`].
pub fn quat_argument_bin(q: &Quat) -> u8 {
    let vmag = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
    let mut angle = 2.0 * vmag.atan2(q.w);
    if angle < 0.0 {
        angle += core::f32::consts::TAU;
    }
    ((angle / core::f32::consts::FRAC_PI_4) as u8) & 0x07
}

/// RETIRED (witness): the i-plane quantiser made total — the raw eight-bin
/// classifier index of the retired half-angle argument
/// `bin = floor((atan2(x, w) wrapped to [0, 2π)) / 45°) & 7`, the exact
/// quantiser `quat_active_state_retired_i_plane` applies. Its +4 antipodal
/// shift (negation shifts the i-plane argument by 180°) is a diagnostic of
/// THIS function only, not of the corrected law. Used by the retired witness
/// tests.
pub fn quat_argument_bin_retired_i_plane(q: &Quat) -> u8 {
    let mut angle = q.x.atan2(q.w);
    if angle < 0.0 {
        angle += core::f32::consts::TAU;
    }
    ((angle / core::f32::consts::FRAC_PI_4) as u8) & 0x07
}
