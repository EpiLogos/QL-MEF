//! The Jankó controller projection of the accepted #31 musical object
//! (`docs/music/JANKO-QL-INSTRUMENT-FIGURE.md`).
//!
//! The surface consumes the kernel derivation and holds no pitch table: every
//! pitch is read through [`MusicalBasis`] operators, so no controller-specific
//! representation becomes a second authority (figure §5.7). The geometry is
//! the chromatic whole-tone construction: six rows are two interleaved
//! whole-tone families (the kernel's direct/conjugate helices) held three
//! times as repeated touch-points; the neighbouring interleaved row is the
//! semitone-shifted complementary collection through the basis's own
//! conjugate axis. The fifths basis enters only as a second traversal/address
//! overlay over the same twelve classes, never as a second surface geometry.
//!
//! A key's point on the torus (PROGRAMME R34; figure §4a): the key stands at
//! one place of its helix — helix h (`whole_tone_row_family`) and position p
//! (`coordinate`) — and that place is the arc 6h + p of the spanda ring of
//! twelve, the arcs `spanda_field::codon_advance` reads (helix = arc ≥ 6,
//! position = arc mod 6). Twelve arcs make the 720° cover, six to a 360°
//! sheet (`RING_HALF`), so the second helix takes the second sheet. The torus
//! turns with the pulse, one arc a step, and carries the place across the
//! surface: at cover angle α = 60°·(arc + steps) the point is φ = α mod 360°
//! round the ring and θ = α/2 round the tube — the spinor half-angle of
//! `m1::Clock::spinor`, which changes sign after one sheet and returns after
//! two. A semitone (the next row, the other helix) is the half-turn across the
//! tube at the same ring place. [`JankoSurface::point`] is that law; the scene
//! body strikes whatever point it is given (`scene_field::ShapeBasis::strike_at`).
//!
//! Provenance boundary (figure §0): the 3:3 / 4:2 white/black colour counts
//! are the historical instrument's own facts and their QL reading is a
//! project-level Figure; neither enters this projection's mathematics.

use ql_core::{QlCoordinate, QlFace, QlPosition};
use serde::{Deserialize, Serialize};

use crate::LensId;
use crate::music::{MusicalBasis, PitchClass, pitch_at_lens, pitch_name};

fn position(value: u8) -> QlPosition {
    QlPosition::new(value).expect("canonical musical positions are modulo six")
}

/// One physical key of the six-row surface. Columns are unbounded; the
/// projection is periodic with a six-column period (one helix traversal).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JankoKey {
    /// 0..6. Even rows carry the direct whole-tone family, odd rows the
    /// conjugate (semitone-shifted complementary) family.
    pub row: u8,
    pub column: u16,
}

impl JankoKey {
    pub const fn new(row: u8, column: u16) -> Self {
        Self { row, column }
    }
}

pub const ROWS: u8 = 6;
/// Positions per helix traversal: one column period of every row.
pub const COLUMN_PERIOD: u16 = 6;
pub const TOUCH_POINTS: u8 = 3;

/// The law of a key's moving point, carried with every point disclosed.
pub const POINT_LAW: &str = "a key stands at one place of its helix: arc = 6·helix + position of the spanda ring of twelve (spanda_field::codon_advance: helix = arc ≥ 6, position = arc mod 6); twelve arcs are the 720° cover, six to a sheet, so the second helix takes the second sheet; the torus turns one arc a step and carries the place: α = 60°·(arc + steps), φ = α mod 360° round the ring, θ = α/2 mod 360° round the tube (the spinor half-angle, m1::Clock::spinor); a semitone is the half-turn across the tube at the same ring place (PROGRAMME R34; Jankó figure §4a)";

/// The place of a key on its helix: it never moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HelixPlace {
    /// 0 bimba (WT₀), 1 pratibimba (WT₁).
    pub helix: u8,
    /// 0..6 along the helix.
    pub position: u8,
    /// 0..12 on the spanda ring: 6·helix + position.
    pub arc: u8,
    /// Its place on the 720° cover, in whole degrees.
    pub cover_degrees: u16,
}

impl HelixPlace {
    /// The place at a ring arc (0..12): helix = arc ≥ 6, position = arc mod 6.
    pub fn of_arc(arc: u8) -> Self {
        let arc = arc % crate::spanda_field::RING_SIZE;
        let half = crate::spanda_field::RING_HALF;
        Self {
            helix: arc / half,
            position: arc % half,
            arc,
            cover_degrees: u16::from(arc) * 60,
        }
    }
}

/// Where a place of the ring stands on the turning torus after `pulse_steps`
/// ([`POINT_LAW`]): a key's place, or a codon's (O-I #771, a singlet strikes at
/// its codon's place, carried by the same law). The place never moves; only the
/// torus has turned.
pub fn point_of(place: HelixPlace, pulse_steps: f64) -> KeyPoint {
    let ring = f64::from(crate::spanda_field::RING_SIZE);
    let half = f64::from(crate::spanda_field::RING_HALF);
    // Arcs on the cover, in [0, 12); `rem_euclid` can round up to 12.
    let mut arcs = (f64::from(place.arc) + pulse_steps).rem_euclid(ring);
    if arcs >= ring {
        arcs = 0.0;
    }
    let sheet_arcs = arcs.rem_euclid(half);
    let sheet_arcs = if sheet_arcs >= half { 0.0 } else { sheet_arcs };
    let pi = std::f64::consts::PI;
    KeyPoint {
        place,
        pulse_steps,
        cover_degrees: arcs * 60.0,
        sheet: u8::from(arcs >= half),
        arc: (arcs.floor() as u8) % crate::spanda_field::RING_SIZE,
        // Half the cover angle round the tube; the cover angle mod 360° round the ring.
        theta: arcs * (pi / half),
        phi: sheet_arcs * (pi / (half / 2.0)),
    }
}

/// Where a key's place stands on the turning torus after `pulse_steps`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct KeyPoint {
    pub place: HelixPlace,
    pub pulse_steps: f64,
    /// The place carried by the pulse, degrees on the 720° cover.
    pub cover_degrees: f64,
    /// 0 or 1: which 360° sheet of the cover the point stands on.
    pub sheet: u8,
    /// The ring arc (0..12) the point stands on now.
    pub arc: u8,
    /// Round the tube (poloidal), radians in [0, 2π).
    pub theta: f64,
    /// Round the ring (toroidal), radians in [0, 2π).
    pub phi: f64,
}

/// Where one of the twelve classes stands on the turning torus: the one place
/// of its helix that every touch-point of it shares, and that place's point
/// after the pulse has turned ([`POINT_LAW`]). `key` is the class's key on the
/// first row of its family, in the first column period.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct ClassPoint {
    pub pitch_class: PitchClass,
    pub pitch_name: &'static str,
    pub key: JankoKey,
    pub point: KeyPoint,
}

/// What one key means: the figure's §2 controller-neutral projection
/// contract, filled entirely from the accepted musical state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JankoSurfaceProjection {
    pub key: JankoKey,
    pub sounding_pitch_class: PitchClass,
    pub pitch_name: &'static str,
    /// 0 = direct whole-tone family, 1 = conjugate (semitone-shifted) family.
    pub whole_tone_row_family: u8,
    /// 0..3: which of the family's three duplicated rows carries this key.
    pub repeated_touch_point: u8,
    /// The kernel position and direct/prime face this key projects.
    pub coordinate: QlCoordinate,
    /// The kernel face code (`direct` / `prime`) of `coordinate`.
    pub direct_prime_face: &'static str,
    pub musical_basis: MusicalBasis,
    /// The L/L′ tonic anchor this surface is held by, if any.
    pub lens: Option<LensId>,
    pub lens_tonic_pitch: Option<PitchClass>,
}

/// The six-row Jankó surface over the accepted chromatic substrate, optionally
/// anchored by a lens tonic. The fifths basis is reachable per key through
/// [`JankoSurface::fifths_overlay`].
#[derive(Debug, Clone, Copy)]
pub struct JankoSurface {
    lens: Option<LensId>,
}

impl JankoSurface {
    pub const fn new() -> Self {
        Self { lens: None }
    }
    pub const fn anchored(lens: LensId) -> Self {
        Self { lens: Some(lens) }
    }

    fn face(row: u8) -> QlFace {
        if row % 2 == 0 {
            QlFace::Direct
        } else {
            QlFace::Conjugate
        }
    }

    /// The key's kernel coordinate: family face, position advanced one
    /// whole-tone step per column plus the touch-point's whole-tone offset.
    pub fn coordinate(key: JankoKey) -> QlCoordinate {
        let touch_point = usize::from(key.row / 2);
        let index = (usize::from(key.column) + touch_point) % usize::from(COLUMN_PERIOD);
        QlCoordinate::new(position(index as u8), Self::face(key.row))
    }

    /// The key's place on its helix: helix, position and its ring arc.
    pub fn helix_place(key: JankoKey) -> HelixPlace {
        let helix = key.row % 2;
        let position = Self::coordinate(key).position.value();
        let arc = helix * crate::spanda_field::RING_HALF + position;
        HelixPlace {
            helix,
            position,
            arc,
            cover_degrees: u16::from(arc) * 60,
        }
    }

    /// The twelve classes in class order, each at its place and its point after
    /// `pulse_steps` (O-I #851: a symbol's voice is the chord whose figure on
    /// the torus is the symbol's shape, so a reader needs where each class
    /// stands). The two family rows of one column period hold every class once.
    pub fn class_points(&self, pulse_steps: f64) -> Vec<ClassPoint> {
        let mut classes: Vec<ClassPoint> = (0..2)
            .flat_map(|row| (0..COLUMN_PERIOD).map(move |column| JankoKey::new(row, column)))
            .map(|key| {
                let pitch_class = self.project(key).sounding_pitch_class;
                ClassPoint {
                    pitch_class,
                    pitch_name: pitch_name(pitch_class),
                    key,
                    point: self.point(key, pulse_steps),
                }
            })
            .collect();
        classes.sort_by_key(|class| class.pitch_class);
        classes
    }

    /// The key's moving point after `pulse_steps` of the pulse ([`POINT_LAW`]).
    /// The place is the key's own; only the torus has turned.
    pub fn point(&self, key: JankoKey, pulse_steps: f64) -> KeyPoint {
        point_of(Self::helix_place(key), pulse_steps)
    }

    pub fn project(&self, key: JankoKey) -> JankoSurfaceProjection {
        let coordinate = Self::coordinate(key);
        let basis = MusicalBasis::Chromatic;
        let sounding_pitch_class = match self.lens {
            Some(lens) => pitch_at_lens(basis, lens, coordinate),
            None => basis.pitch_at(coordinate),
        };
        JankoSurfaceProjection {
            key,
            sounding_pitch_class,
            pitch_name: pitch_name(sounding_pitch_class),
            whole_tone_row_family: key.row % 2,
            repeated_touch_point: key.row / 2,
            coordinate,
            direct_prime_face: coordinate.face.kernel_code(),
            musical_basis: basis,
            lens: self.lens,
            lens_tonic_pitch: self
                .lens
                .map(|lens| crate::music::lens_anchor(basis, lens).pitch),
        }
    }

    /// The three touch-points that sound one pitch class: they all live in one
    /// whole-tone family at one kernel coordinate.
    pub fn touch_points(
        &self,
        pitch_class: PitchClass,
    ) -> Option<[JankoKey; TOUCH_POINTS as usize]> {
        let basis = MusicalBasis::Chromatic;
        for face in [QlFace::Direct, QlFace::Conjugate] {
            for (index, class) in basis.helix(face).into_iter().enumerate() {
                if class != pitch_class % 12 {
                    continue;
                }
                let family = match face {
                    QlFace::Direct => 0_u8,
                    QlFace::Conjugate => 1_u8,
                };
                let keys = std::array::from_fn(|p: usize| {
                    let column =
                        (index + usize::from(COLUMN_PERIOD) - p) % usize::from(COLUMN_PERIOD);
                    JankoKey::new(family + 2 * p as u8, column as u16)
                });
                return Some(keys);
            }
        }
        None
    }

    /// The fifths-basis traversal address of a key: the coordinate in the
    /// fifths helices sounding the same pitch class. One Z12 substrate, two
    /// addressings; the geometry never changes.
    pub fn fifths_overlay(&self, key: JankoKey) -> QlCoordinate {
        let pitch = self.project(key).sounding_pitch_class;
        let basis = MusicalBasis::Fifths;
        for face in [QlFace::Direct, QlFace::Conjugate] {
            for (index, class) in basis.helix(face).into_iter().enumerate() {
                if class == pitch {
                    return QlCoordinate::new(position(index as u8), face);
                }
            }
        }
        unreachable!("both helices exhaust one Z12 substrate")
    }

    /// One full period of the surface: six rows across `COLUMN_PERIOD`
    /// columns, ordered row-major from `first_column`.
    pub fn window(&self, first_column: u16) -> Vec<JankoSurfaceProjection> {
        let mut out = Vec::with_capacity(usize::from(ROWS) * usize::from(COLUMN_PERIOD));
        for row in 0..ROWS {
            for step in 0..COLUMN_PERIOD {
                out.push(self.project(JankoKey::new(row, first_column + step)));
            }
        }
        out
    }
}

impl JankoSurfaceProjection {
    /// The JSON disclosure a controller or CLI reads. No private state and no
    /// authority beyond the kernel operators that produced it.
    pub fn disclosure(&self) -> serde_json::Value {
        serde_json::json!({
            "key": {"row": self.key.row, "column": self.key.column},
            "sounding_pitch_class": self.sounding_pitch_class,
            "pitch_name": self.pitch_name,
            "whole_tone_row_family": self.whole_tone_row_family,
            "repeated_touch_point": self.repeated_touch_point,
            "coordinate": {
                "position": self.coordinate.position.value(),
                "face": self.direct_prime_face,
            },
            "direct_prime_face": self.direct_prime_face,
            "musical_basis": format!("{:?}", self.musical_basis).to_lowercase(),
            "lens": self.lens.map(|lens| lens.index()),
            "lens_tonic_pitch": self.lens_tonic_pitch,
        })
    }
}

impl Default for JankoSurface {
    fn default() -> Self {
        Self::new()
    }
}
