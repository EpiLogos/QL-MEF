//! The Jankó surface's acceptance over the accepted musical object
//! (docs/music/JANKO-QL-INSTRUMENT-FIGURE.md §5): two interleaved whole-tone
//! families, three touch-points per sounding note, cross-row semitone
//! adjacency, the full twelve-class substrate, transposition-preserving
//! fingering under every lens anchor, and the fifths overlay over one Z12.
use ql_mef::{
    ALL_PITCH_CLASSES, COLUMN_PERIOD, JankoKey, JankoSurface, MusicalBasis, PitchClass, ROWS,
    TOUCH_POINTS, directed_pitch_delta, lens_anchors, transpose,
};

fn pitch(key: JankoKey) -> PitchClass {
    JankoSurface::new().project(key).sounding_pitch_class
}

#[test]
fn six_rows_are_two_interleaved_whole_tone_families() {
    for row in 0..ROWS {
        // Along a row the next key is a whole tone away: the chromatic
        // generator, read from the basis, never a table.
        for column in 0..COLUMN_PERIOD - 1 {
            let step = directed_pitch_delta(
                pitch(JankoKey::new(row, column)),
                pitch(JankoKey::new(row, column + 1)),
            );
            assert_eq!(step, MusicalBasis::Chromatic.generator_semitones());
        }
        // The neighbouring interleaved row is the semitone-shifted
        // complementary whole-tone collection.
        if row + 1 < ROWS {
            for column in 0..COLUMN_PERIOD {
                let step = directed_pitch_delta(
                    pitch(JankoKey::new(row, column)),
                    pitch(JankoKey::new(row + 1, column)),
                );
                // One semitone apart in either direction: the interleaved
                // family sits on the conjugate axis of the basis.
                assert!(
                    step == 1 || step == 12 - MusicalBasis::Chromatic.conjugate_axis_semitones(),
                    "row {row} -> {} at column {column}: {step}",
                    row + 1
                );
            }
        }
    }
    // The two families' rows are exact shifted copies: one sounding note
    // sits at column c on the first touch-point row, one key left on the
    // second, two keys left on the third (c - p, periodic in the row).
    for column in 0..COLUMN_PERIOD {
        let direct = JankoSurface::coordinate(JankoKey::new(0, column));
        assert_eq!(
            JankoSurface::coordinate(JankoKey::new(2, column + COLUMN_PERIOD - 1)),
            direct
        );
        assert_eq!(
            JankoSurface::coordinate(JankoKey::new(4, column + COLUMN_PERIOD - 2)),
            direct
        );
        let conjugate = JankoSurface::coordinate(JankoKey::new(1, column));
        assert_eq!(
            JankoSurface::coordinate(JankoKey::new(3, column + COLUMN_PERIOD - 1)),
            conjugate
        );
        assert_eq!(
            JankoSurface::coordinate(JankoKey::new(5, column + COLUMN_PERIOD - 2)),
            conjugate
        );
    }
}

#[test]
fn each_pitch_class_has_exactly_three_touch_points_in_one_family() {
    for pitch_class in ALL_PITCH_CLASSES {
        let keys = JankoSurface::new()
            .touch_points(pitch_class)
            .unwrap_or_else(|| panic!("{pitch_class} is outside the accepted substrate"));
        let family = keys[0].row % 2;
        for (p, key) in keys.into_iter().enumerate() {
            assert_eq!(key.row % 2, family, "touch-points span families");
            assert_eq!(key.row / 2, p as u8, "touch point {p} misnumbered");
            let projection = JankoSurface::new().project(key);
            assert_eq!(projection.sounding_pitch_class, pitch_class);
            assert_eq!(projection.whole_tone_row_family, family);
        }
        assert_eq!(keys[0].row / 2, 0);
    }
}

#[test]
fn one_window_exhausts_the_accepted_substrate() {
    let window = JankoSurface::new().window(3);
    assert_eq!(window.len(), usize::from(ROWS) * usize::from(COLUMN_PERIOD));
    let mut counts = [0_u8; 12];
    for projection in &window {
        counts[usize::from(projection.sounding_pitch_class)] += 1;
        // The projection names the kernel coordinate's own pitch: the
        // surface never substitutes a second pitch map.
        assert_eq!(
            MusicalBasis::Chromatic.pitch_at(projection.coordinate),
            projection.sounding_pitch_class
        );
        assert_eq!(
            projection.direct_prime_face,
            projection.coordinate.face.kernel_code()
        );
    }
    for pitch_class in ALL_PITCH_CLASSES {
        assert_eq!(
            counts[usize::from(pitch_class)],
            TOUCH_POINTS,
            "{pitch_class}"
        );
    }
}

#[test]
fn lens_anchors_transpose_every_key_and_preserve_fingering() {
    let figure = [
        JankoKey::new(0, 0),
        JankoKey::new(1, 0),
        JankoKey::new(0, 1),
        JankoKey::new(1, 2),
        JankoKey::new(0, 3),
    ];
    let plain: Vec<PitchClass> = figure.iter().map(|key| pitch(*key)).collect();
    for anchor in lens_anchors(MusicalBasis::Chromatic) {
        let surface = JankoSurface::anchored(anchor.lens);
        for key in figure {
            let projection = surface.project(key);
            assert_eq!(
                projection.sounding_pitch_class,
                transpose(pitch(key), anchor.pitch),
                "lens {:?} key {key:?}",
                anchor.lens,
            );
            assert_eq!(projection.coordinate, JankoSurface::coordinate(key));
            assert_eq!(projection.lens_tonic_pitch, Some(anchor.pitch));
        }
        // Fingering shape: the melodic interval pattern is the anchor's
        // constant transposition, so the same keys play the transposed
        // figure under every one of the twelve anchors.
        let lensed: Vec<PitchClass> = figure
            .iter()
            .map(|key| surface.project(*key).sounding_pitch_class)
            .collect();
        for (plain_pair, lensed_pair) in plain.windows(2).zip(lensed.windows(2)) {
            assert_eq!(
                directed_pitch_delta(plain_pair[0], plain_pair[1]),
                directed_pitch_delta(lensed_pair[0], lensed_pair[1]),
                "anchor {:?}",
                anchor.lens
            );
        }
    }
}

#[test]
fn fifths_overlay_addresses_the_same_classes_over_one_substrate() {
    let mut chromatic = MusicalBasis::Chromatic.substrate();
    let mut fifths = MusicalBasis::Fifths.substrate();
    chromatic.sort_unstable();
    fifths.sort_unstable();
    assert_eq!(chromatic, ALL_PITCH_CLASSES);
    assert_eq!(fifths, ALL_PITCH_CLASSES);
    let surface = JankoSurface::new();
    for projection in surface.window(0) {
        let overlay = surface.fifths_overlay(projection.key);
        assert_eq!(
            MusicalBasis::Fifths.pitch_at(overlay),
            projection.sounding_pitch_class,
            "overlay of {:?}",
            projection.key
        );
    }
}
