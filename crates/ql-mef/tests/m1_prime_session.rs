//! The M1′ session (O-I #850; DLW manual §21.2): the pulse and its bands,
//! Free and Sync rate, the Fundamental, the comma, the shapes, Polyphony, and
//! a key's moving point on the torus (PROGRAMME R34; Jankó figure §4a).
use ql_mef::janko::{JankoKey, JankoSurface};
use ql_mef::m1_prime::{
    Band, Figure, Fundamental, NoteLength, PYTHAGOREAN_COMMA, Rate, Session, Settings,
    SettingsEdit, Shape, Tuning, pythagorean_comma_cents,
};
use std::f64::consts::TAU;

const SR: u32 = 48_000;

fn open(settings: Settings) -> Session {
    Session::open(settings, SR, 0).unwrap()
}

fn free(steps_per_second: f64) -> Settings {
    Settings {
        rate: Rate::Free { steps_per_second },
        ..Settings::default()
    }
}

/// Samples for `seconds` at the session's sample rate.
fn at(seconds: f64) -> u64 {
    (seconds * f64::from(SR)).round() as u64
}

#[test]
fn the_five_bands_run_without_a_break_at_the_manuals_edges() {
    for (rate, band) in [
        (0.000_9, Band::Cosmic),
        (0.001, Band::Breath),
        (0.499_9, Band::Breath),
        (0.5, Band::Rhythm),
        (19.99, Band::Rhythm),
        (20.0, Band::Pitch),
        (20_000.0, Band::Pitch),
        (20_000.1, Band::Beyond),
    ] {
        assert_eq!(Band::of(rate), band, "{rate} steps a second");
    }
    // The default is the spanda carrier's 2.5 a second: a beat you could tap.
    let session = open(Settings::default());
    assert_eq!(session.steps_per_second(), 2.5);
    assert_eq!(session.band(), Band::Rhythm);
    assert_eq!(Settings::default().tuning, Tuning::Logic);
    assert_eq!(Settings::default().polyphony, 16);
}

#[test]
fn free_rate_runs_from_one_step_a_year_to_forty_thousand_a_second() {
    let year = 1.0 / (365.25 * 86_400.0);
    assert!(Session::open(free(year), SR, 0).is_ok());
    assert!(Session::open(free(year * 0.99), SR, 0).is_err());
    assert!(Session::open(free(40_000.0), SR, 0).is_ok());
    let refused = Session::open(free(40_000.5), SR, 0).unwrap_err();
    assert!(refused.contains("40000"), "{refused}");
}

#[test]
fn sync_steps_once_per_note_length_at_the_sets_tempo_and_follows_a_change() {
    let sync = |numerator, denominator| Settings {
        rate: Rate::Sync {
            note: NoteLength {
                numerator,
                denominator,
            },
        },
        tempo_bpm: 120.0,
        ..Settings::default()
    };
    // A quarter note at 120 BPM is half a second: two steps a second.
    assert!((open(sync(1, 4)).steps_per_second() - 2.0).abs() < 1e-12);
    // One bar (four beats) at 120 is two seconds; 1/64 is a sixteenth of a beat.
    assert!((open(sync(1, 1)).steps_per_second() - 0.5).abs() < 1e-12);
    assert!((open(sync(1, 64)).steps_per_second() - 32.0).abs() < 1e-12);
    assert!((open(sync(8, 1)).steps_per_second() - 1.0 / 16.0).abs() < 1e-12);
    // Outside 1/64 .. 8 bars is refused.
    assert!(Session::open(sync(1, 128), SR, 0).is_err());
    assert!(Session::open(sync(16, 1), SR, 0).is_err());
    // A tempo change moves the pulse: 60 BPM halves a quarter note's rate.
    let mut session = open(sync(1, 4));
    session
        .edit(
            &SettingsEdit {
                tempo_bpm: Some(60.0),
                ..SettingsEdit::default()
            },
            at(1.0),
        )
        .unwrap();
    assert!((session.steps_per_second() - 1.0).abs() < 1e-12);
}

#[test]
fn the_pulse_keeps_its_phase_across_a_change_of_rate_and_band() {
    let mut session = open(free(2.5));
    let before = session.steps_at(at(4.0));
    assert!((before - 10.0).abs() < 1e-9);
    session
        .edit(
            &SettingsEdit {
                rate: Some(Rate::Free {
                    steps_per_second: 440.0,
                }),
                ..SettingsEdit::default()
            },
            at(4.0),
        )
        .unwrap();
    assert_eq!(session.band(), Band::Pitch);
    // No break: the same moment reads the same phase under the new rate.
    assert!((session.steps_at(at(4.0)) - before).abs() < 1e-9);
    assert!((session.steps_at(at(5.0)) - (before + 440.0)).abs() < 1e-6);
}

#[test]
fn the_comma_starts_from_nothing_and_grows_one_pythagorean_comma_per_return() {
    // Six epogdoons over the octave: (9/8)^6 / 2 = 531441 / 524288.
    assert_eq!(PYTHAGOREAN_COMMA, [531_441, 524_288]);
    let cents = pythagorean_comma_cents();
    assert!((cents - 23.460_010_384_649_4).abs() < 1e-9, "{cents}");
    let session = open(free(2.5));
    assert_eq!(session.comma_cents(0), 0.0);
    // Twelve steps make one return: 4.8 s at 2.5 a second.
    assert_eq!(session.comma_cents(at(4.79)), 0.0);
    assert!((session.comma_cents(at(4.8)) - cents).abs() < 1e-12);
    assert!((session.comma_cents(at(14.4)) - 3.0 * cents).abs() < 1e-9);
    // Monotone over a run.
    let mut last = 0.0;
    for second in 0..60 {
        let now = session.comma_cents(at(f64::from(second)));
        assert!(now >= last);
        last = now;
    }
    // A Torus opened again starts from nothing.
    assert_eq!(open(free(2.5)).comma_cents(0), 0.0);
}

#[test]
fn auto_shape_follows_the_band_and_is_a_klein_bottle_for_one_step_at_each_return() {
    let figure = |rate: f64, seconds: f64| open(free(rate)).figure(at(seconds));
    assert_eq!(figure(440.0, 0.1), Some(Figure::Torus));
    // 0.1 s at 30,000 a second is step 3000, a return: sample between returns.
    assert_eq!(figure(30_000.0, 0.1), Some(Figure::KleinBottle));
    assert_eq!(figure(30_000.0, 0.1013), Some(Figure::Torus));
    assert_eq!(figure(2.5, 0.1), Some(Figure::FigureEight));
    assert_eq!(figure(0.1, 1.0), Some(Figure::Spiral));
    assert_eq!(figure(0.000_1, 1.0), Some(Figure::Spiral));
    // At 1 step a second (rhythm) the return is steps 12..13: one step of the
    // Klein bottle, then the figure-eight again.
    assert_eq!(figure(1.0, 0.5), Some(Figure::FigureEight));
    assert_eq!(figure(1.0, 12.5), Some(Figure::KleinBottle));
    assert_eq!(figure(1.0, 13.5), Some(Figure::FigureEight));
    // A held shape holds; Sequence is the scene's and QL names no figure.
    let held = Settings {
        shape: Shape::Spiral,
        ..free(440.0)
    };
    assert_eq!(open(held).figure(at(1.0)), Some(Figure::Spiral));
    let sequence = Settings {
        shape: Shape::Sequence,
        ..free(440.0)
    };
    assert_eq!(open(sequence).figure(at(1.0)), None);
}

#[test]
fn polyphony_counts_notes_and_the_oldest_makes_way() {
    let mut session = open(Settings {
        polyphony: 2,
        ..Settings::default()
    });
    let key = |row, column| JankoKey::new(row, column);
    let first = session.strike(&[key(0, 0)], 0);
    assert_eq!(first.admitted.len(), 1);
    assert!(first.made_way.is_empty());
    let chord = session.strike(&[key(1, 0), key(2, 3)], 10);
    assert_eq!(chord.admitted.len(), 2);
    assert_eq!(chord.made_way.len(), 1);
    assert_eq!(chord.made_way[0].key, key(0, 0));
    assert_eq!(session.sounding().len(), 2);
    let released = session.release(&[key(1, 0)]);
    assert_eq!(released.len(), 1);
    assert_eq!(session.sounding().len(), 1);
    // Polyphony outside 1..32 is refused.
    for polyphony in [0, 33] {
        assert!(
            Session::open(
                Settings {
                    polyphony,
                    ..Settings::default()
                },
                SR,
                0
            )
            .is_err()
        );
    }
}

#[test]
fn the_fundamental_is_map_or_twenty_to_two_thousand_hertz() {
    let session = open(Settings::default());
    assert_eq!(session.settings().fundamental, Fundamental::Map);
    assert_eq!(session.fundamental_hz(130.812_79), 130.812_79);
    let tuned = open(Settings {
        fundamental: Fundamental::Hz { hz: 261.6 },
        ..Settings::default()
    });
    assert_eq!(tuned.fundamental_hz(130.812_79), 261.6);
    for hz in [19.9, 2000.1] {
        assert!(
            Session::open(
                Settings {
                    fundamental: Fundamental::Hz { hz },
                    ..Settings::default()
                },
                SR,
                0
            )
            .is_err()
        );
    }
}

#[test]
fn a_held_key_strikes_a_moving_point_and_always_the_same_place_of_its_helix() {
    let surface = JankoSurface::new();
    let key = JankoKey::new(2, 1);
    let place = JankoSurface::helix_place(key);
    let now = surface.point(key, 0.0);
    let later = surface.point(key, 1.0);
    // The same place of its helix at every moment...
    assert_eq!(now.place, place);
    assert_eq!(later.place, place);
    // ...carried across the surface by the turning torus.
    assert!((now.theta - later.theta).abs() > 1e-6 || (now.phi - later.phi).abs() > 1e-6);
    for point in [now, later] {
        assert!((0.0..TAU).contains(&point.theta) && (0.0..TAU).contains(&point.phi));
    }
    // The point returns after the full 720° return: twelve steps.
    let returned = surface.point(key, 12.0);
    assert!((returned.theta - now.theta).abs() < 1e-9 && (returned.phi - now.phi).abs() < 1e-9);
    // The second helix takes the 720° cover's second sheet; the first, the first.
    let bimba = surface.point(JankoKey::new(0, 0), 0.0);
    let pratibimba = surface.point(JankoKey::new(1, 0), 0.0);
    assert_eq!(bimba.sheet, 0);
    assert_eq!(pratibimba.sheet, 1);
    // A semitone (the next row) is the half-turn across the tube at the same ring place.
    assert!((pratibimba.phi - bimba.phi).abs() < 1e-12);
    assert!((pratibimba.theta - bimba.theta - std::f64::consts::PI).abs() < 1e-12);
}
