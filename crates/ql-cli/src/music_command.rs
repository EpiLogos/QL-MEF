//! `ql music janko`: the Jankó surface projected from the accepted musical
//! object — a controller read, not a second music authority.

use crate::CliError;
use ql_mef::m2::{catalogue, maqam_pitches};
use ql_mef::{
    ALL_PITCH_CLASSES, JankoKey, JankoSurface, MUSICAL_DERIVATION_SOURCE_BLOB,
    MUSICAL_DERIVATION_SOURCE_PATH, MUSICAL_DERIVATION_VENDOR_COMMIT, MUSICAL_HARMONIC_VERSION,
    ModeKind, MusicalBasis, mode_tonic_landscape,
};

fn usage() -> String {
    "usage: ql music janko window [--columns N] [--first-column C] [--lens 0..=11] [--json]\n\
     usage: ql music janko key --row 0..=5 --column C [--lens 0..=11] [--json]\n\
     usage: ql music janko touch-points --pitch 0..=11 [--json]\n\
     usage: ql music modes [--json]"
        .into()
}

fn flag(args: &[String], name: &str) -> Option<Result<String, CliError>> {
    args.iter()
        .position(|a| a == name)
        .map(|i| match args.get(i + 1) {
            Some(value) => Ok(value.clone()),
            None => Err(CliError(format!("{name} needs a value"))),
        })
}

fn parsed<T: std::str::FromStr>(
    args: &[String],
    name: &str,
    range: &str,
) -> Result<Option<T>, CliError> {
    match flag(args, name) {
        Some(value) => {
            let value = value?;
            value
                .parse::<T>()
                .map(Some)
                .map_err(|_| CliError(format!("{name} must be {range}")))
        }
        None => Ok(None),
    }
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    if args.first().map(String::as_str) == Some("modes") {
        return modes(&args[1..]);
    }
    // args[0] is the `janko` group; the mode is its first argument.
    if args.first().map(String::as_str) != Some("janko") {
        return Err(CliError(usage()));
    }
    let rest = &args[1..];
    let json = rest.iter().any(|a| a == "--json");
    let lens: Option<u8> = parsed(rest, "--lens", "0..=11")?;
    let lens = lens
        .map(|index| {
            ql_mef::LensId::ALL
                .get(usize::from(index))
                .copied()
                .ok_or_else(|| CliError("--lens must be 0..=11".into()))
        })
        .transpose()?;
    let surface = match lens {
        Some(lens) => JankoSurface::anchored(lens),
        None => JankoSurface::new(),
    };
    let value = match rest.first().map(String::as_str).unwrap_or("window") {
        "key" => {
            let rest = &rest[1..];
            let row: u8 = parsed(rest, "--row", "0..=5")?.ok_or_else(|| CliError(usage()))?;
            let column: u16 =
                parsed(rest, "--column", "a column number")?.ok_or_else(|| CliError(usage()))?;
            if row >= ql_mef::ROWS {
                return Err(CliError("--row must be 0..=5".into()));
            }
            let projection = surface.project(JankoKey::new(row, column));
            let mut disclosure = projection.disclosure();
            disclosure["fifths_overlay"] = {
                let overlay = surface.fifths_overlay(projection.key);
                serde_json::json!({
                    "position": overlay.position.value(),
                    "face": overlay.face.kernel_code(),
                })
            };
            disclosure
        }
        "touch-points" => {
            let rest = &rest[1..];
            let pitch: u8 = parsed(rest, "--pitch", "0..=11")?.ok_or_else(|| CliError(usage()))?;
            if pitch > 11 {
                return Err(CliError("--pitch must be 0..=11".into()));
            }
            let keys = surface
                .touch_points(pitch)
                .ok_or_else(|| CliError("pitch outside the accepted substrate".into()))?;
            serde_json::json!({
                "sounding_pitch_class": pitch,
                "touch_points": keys.map(|key| surface.project(key).disclosure()),
            })
        }
        "window" | "surface" | "janko" => {
            let columns: u16 =
                parsed(rest, "--columns", "1..=24")?.unwrap_or(ql_mef::COLUMN_PERIOD);
            if columns == 0 || columns > 24 {
                return Err(CliError("--columns must be 1..=24".into()));
            }
            let first_column: u16 = parsed(rest, "--first-column", "a column number")?.unwrap_or(0);
            let rows: Vec<serde_json::Value> = (0..ql_mef::ROWS)
                .map(|row| {
                    serde_json::json!({
                        "row": row,
                        "whole_tone_row_family": row % 2,
                        "keys": (first_column..first_column.saturating_add(columns))
                            .map(|column| surface.project(JankoKey::new(row, column)).disclosure())
                            .collect::<Vec<_>>(),
                    })
                })
                .collect();
            serde_json::json!({
                "schema": "ql.janko-surface/v1",
                "rows": ql_mef::ROWS,
                "column_period": ql_mef::COLUMN_PERIOD,
                "touch_points": ql_mef::TOUCH_POINTS,
                "substrate_pitch_classes": ALL_PITCH_CLASSES,
                "lens": lens.map(|lens| lens.index()),
                "surface": rows,
            })
        }
        _ => return Err(CliError(usage())),
    };
    if json {
        serde_json::to_string(&value).map_err(|e| CliError(e.to_string()))
    } else {
        serde_json::to_string_pretty(&value).map_err(|e| CliError(e.to_string()))
    }
}

/// `ql music modes`: the kernel's mode catalogue as one versioned reading.
/// Every value is read from the kernel; nothing here re-derives a mode.
fn modes(args: &[String]) -> Result<String, CliError> {
    if args.iter().any(|a| a != "--json") {
        return Err(CliError(usage()));
    }
    let json = args.iter().any(|a| a == "--json");
    let value = modes_reading()?;
    if json {
        serde_json::to_string(&value).map_err(|e| CliError(e.to_string()))
    } else {
        serde_json::to_string_pretty(&value).map_err(|e| CliError(e.to_string()))
    }
}

fn basis_name(basis: MusicalBasis) -> &'static str {
    match basis {
        MusicalBasis::Chromatic => "chromatic",
        MusicalBasis::Fifths => "fifths",
    }
}

fn mode_name(mode: ModeKind) -> &'static str {
    match mode {
        ModeKind::Ionian => "ionian",
        ModeKind::Dorian => "dorian",
        ModeKind::Phrygian => "phrygian",
        ModeKind::Lydian => "lydian",
        ModeKind::Mixolydian => "mixolydian",
        ModeKind::Aeolian => "aeolian",
        ModeKind::Locrian => "locrian",
    }
}

fn modes_reading() -> Result<serde_json::Value, CliError> {
    let kernel = catalogue();
    let musical_derivation = serde_json::json!({
        "path": MUSICAL_DERIVATION_SOURCE_PATH,
        "blob": MUSICAL_DERIVATION_SOURCE_BLOB,
        "vendor_commit": MUSICAL_DERIVATION_VENDOR_COMMIT,
        "version": MUSICAL_HARMONIC_VERSION,
    });

    let bases: Vec<serde_json::Value> = MusicalBasis::ALL
        .into_iter()
        .map(|basis| {
            let ratio = basis.generator_ratio();
            serde_json::json!({
                "basis": basis_name(basis),
                "generator_ratio": {
                    "num": ratio.numerator(),
                    "den": ratio.denominator(),
                },
                "generator_semitones": basis.generator_semitones(),
                "conjugate_axis_semitones": basis.conjugate_axis_semitones(),
            })
        })
        .collect();

    // Every lens/CF instance the kernel serves, for every basis it defines.
    let mut lens_cf: Vec<(String, serde_json::Value)> = Vec::new();
    for basis in MusicalBasis::ALL {
        for instance in mode_tonic_landscape(basis) {
            let basis_label = basis_name(basis);
            let lens = instance.lens.code();
            let mode = mode_name(instance.mode);
            let key = format!("{basis_label}/{lens}/{mode}");
            let pitches = instance.pitches;
            // Exact semitone steps between successive degrees, from the served pitches.
            let steps: [u8; 7] =
                std::array::from_fn(|i| (pitches[(i + 1) % 7] + 12 - pitches[i]) % 12);
            lens_cf.push((
                key.clone(),
                serde_json::json!({
                    "kind": "lens-cf",
                    "key": key,
                    "label": format!("{mode} on {lens}, {basis_label} basis"),
                    "lens": lens,
                    "anchor_basis": basis_label,
                    "mode": mode,
                    "context_frame": instance.context_frame.code(),
                    "scale_beneath_tonic": instance.scale_beneath_tonic,
                    "tonic": instance.tonic,
                    "pitches": pitches,
                    "steps_semitones": steps,
                    "tuning": null,
                    "provenance": {
                        "owner": "ql-mef",
                        "symbol": "mode_tonic_landscape",
                        "operator": "mode_tonic_instance",
                        "revision": MUSICAL_HARMONIC_VERSION,
                        "source": musical_derivation.clone(),
                    },
                }),
            ));
        }
    }
    lens_cf.sort_by(|a, b| a.0.cmp(&b.0));
    let lens_cf: Vec<serde_json::Value> = lens_cf.into_iter().map(|(_, value)| value).collect();

    // Source scales: exactly the maqam rows that maqam_pitches serves.
    let maqam = kernel.table("maqam").map_err(CliError)?;
    let ratios = kernel.table("ratio").map_err(CliError)?;
    let sources: Vec<serde_json::Value> = kernel
        .sources()
        .iter()
        .map(|source| serde_json::json!({"path": source.path, "sha256": source.sha256}))
        .collect();
    let mut source_scales: Vec<serde_json::Value> = Vec::new();
    for mode in 0..72_u8 {
        maqam_pitches(mode, 1.0).map_err(CliError)?;
        let row = maqam.row(usize::from(mode)).map_err(CliError)?;
        if row.len() != 11 {
            return Err(CliError(format!(
                "maqam row {mode} has an unexpected width"
            )));
        }
        let family = row[0];
        let mode_in_family = row[1];
        let steps = row[2..9].to_vec();
        let mut from_root = [0_u64; 8];
        for degree in 0..7 {
            from_root[degree + 1] = from_root[degree] + row[2 + degree];
        }
        let family_index = usize::try_from(family)
            .map_err(|_| CliError(format!("maqam family {family} is out of range")))?;
        let ratio = ratios.row(family_index).map_err(CliError)?;
        if ratio.len() != 2 {
            return Err(CliError("M2 ratio row has an unexpected width".into()));
        }
        let family_ratio = if ratio[0] == 0 && ratio[1] == 0 {
            serde_json::Value::Null
        } else {
            serde_json::json!({"num": ratio[0], "den": ratio[1]})
        };
        let key = format!("maqam/{mode:02}");
        source_scales.push(serde_json::json!({
            "kind": "source",
            "key": key,
            "label": format!("Maqam {mode} (family {family}, mode {mode_in_family})"),
            "family": "maqam",
            "maqam_family": family,
            "mode_in_family": mode_in_family,
            "pitches": null,
            "tuning": null,
            "tuning_unit": "quarter-tone",
            "quarter_tone_steps": steps,
            "quarter_tones_from_root": from_root,
            "cents_from_root": from_root.map(|quarter_tones| quarter_tones * 50),
            "family_ratio": family_ratio,
            "planet_ruler": row[9],
            "provenance": {
                "owner": "ql-mef",
                "symbol": "maqam_pitches",
                "table": "maqam",
                "source_symbol": maqam.source_symbol(),
                "ratio_table": "ratio",
                "ratio_symbol": ratios.source_symbol(),
                "revision": kernel.registry_revision(),
                "registry_revision": kernel.registry_revision(),
                "sources": sources.clone(),
            },
        }));
    }

    // Named absences: never invented, each with the kernel fact that places it.
    let sources_absent = serde_json::json!([
        {
            "scale": "shem",
            "status": "absent",
            "register": "Register72::Shem",
            "table": "shem",
            "reason": "Register72::Shem is a register of 72 names on an 8 by 9 choir grid. The retained Shem table holds choir, position, element, decan, planet and meaning, and no pitch set or interval pattern. Nothing is served for it.",
        },
        {
            "scale": "ficino-planetary",
            "status": "needs owner authoring",
            "table": null,
            "reason": "QL-MEF holds no authored planetary scale with intervals. The corpus has prose only: Ficino's Dorian and Lydian associations in docs/kernel-rebuild/EPI-WORLD-COORDINATE-COVERAGE-2026-09-30.json and fixtures/kernel/bimba-content-v1.json. The planet table holds one frequency per planet, not a scale.",
        },
    ]);

    Ok(serde_json::json!({
        "schema": "ql.music-modes/v1",
        "bases": bases,
        "lens_cf": lens_cf,
        "source_scales": source_scales,
        "sources_absent": sources_absent,
        "provenance": {
            "owner": "ql-mef",
            "musical_derivation": musical_derivation,
            "m2_registry_revision": kernel.registry_revision(),
        },
    }))
}
