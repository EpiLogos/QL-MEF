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
//! Provenance boundary (figure §0): the 3:3 / 4:2 white/black colour counts
//! are the historical instrument's own facts and their QL reading is a
//! project-level Figure; neither enters this projection's mathematics.

use ql_core::{QlCoordinate, QlFace, QlPosition};

use crate::LensId;
use crate::music::{MusicalBasis, PitchClass, pitch_at_lens, pitch_name};

fn position(value: u8) -> QlPosition {
    QlPosition::new(value).expect("canonical musical positions are modulo six")
}

/// One physical key of the six-row surface. Columns are unbounded; the
/// projection is periodic with a six-column period (one helix traversal).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
