//! `ql music janko`: the Jankó surface projected from the accepted musical
//! object — a controller read, not a second music authority.

use crate::CliError;
use ql_mef::{ALL_PITCH_CLASSES, JankoKey, JankoSurface};

fn usage() -> String {
    "usage: ql music janko window [--columns N] [--first-column C] [--lens 0..=11] [--json]\n\
     usage: ql music janko key --row 0..=5 --column C [--lens 0..=11] [--json]\n\
     usage: ql music janko touch-points --pitch 0..=11 [--json]"
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
