//! Native control helpers for the retained performance owner. No UI theory,
//! sample clock, graph or device operation is created here.
use crate::m1_engine::{EngineConfig, carrier};
use crate::performance_audio::{KeyTouch, PreparedPerformanceBinding};
use serde::Serialize;
use serde_json::{Value, json};

fn bounded(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() >= 256 || value.chars().any(char::is_control) {
        return Err("bounded native input reference required".into());
    }
    Ok(())
}
/// Resolves each independent touch through the SAME retained K/M1/M2 binding.
/// The serial host allocates member/touch tokens and retains the current lease.
/// Source currentness and scope are checked before this helper is entered.
/// This does not reprepare, advance or reset the resident physical body.
pub fn resolve_performance_touch(
    binding: &PreparedPerformanceBinding,
    touch: KeyTouch,
) -> Result<Value, String> {
    bounded(&touch.touch_ref)?;
    if touch.member == 0 || touch.touch == 0 {
        return Err("nonzero native member/touch required".into());
    }
    binding.validate_native_consumers(binding.native_basis(), binding.physical_body())?;
    let note = binding
        .targets()
        .key_target(touch.key, touch.register, &touch.touch_ref)?;
    if note.hertz < 0.001
        || note.hertz >= f64::from(binding.physical_body().request().sample_rate) * 0.45
    {
        return Err("resolved note outside native admitted audio band".into());
    }
    let config: EngineConfig = serde_json::from_value(binding.native_basis().m1["config"].clone())
        .map_err(|e| e.to_string())?;
    let carrier = carrier(
        config.cycle.parse().map_err(|_| "invalid M1 cycle")?,
        config.tick12,
    )?;
    let d = binding.determination();
    let phase = if d["m1_face"] == json!(1) {
        carrier.opposite_quadrature
    } else {
        carrier.quadrature
    };
    let (n, q) = note
        .exact_ratio
        .map(|ratio| {
            (
                ratio.numerator().to_string(),
                ratio.denominator().to_string(),
            )
        })
        .unwrap_or_else(|| ("0".into(), "0".into()));
    Ok(
        json!({"identity":d["identity"],"source_coordinate":note.source_coordinate.source_ref,
      "source_face":d["m1_face"],"tuning_ref":d["tuning_ref"],"member":touch.member.to_string(),
      "touch":touch.touch.to_string(),"touch_ref":touch.touch_ref,"key":touch.key,"position":touch.key/2,
      "coordinate_face":touch.key%2,"register_octave":touch.register,"pitch_class":note.pitch_class,
      "fundamental_hz":note.fundamental.hertz(),"hertz":note.hertz,"ratio_numerator":n,"ratio_denominator":q,
      "exact_ratio":note.exact_ratio.is_some(),"phase_cos":phase[0],"phase_sin":phase[1]}),
    )
}
#[derive(Debug, Clone, Serialize)]
pub struct NativeKeyboardCell {
    pub row: u8,
    pub column: u8,
    pub register_octave: i8,
    pub key: u8,
    pub pitch_class: u8,
    pub label: String,
    pub hertz: f64,
    pub coordinate: String,
    pub face: u8,
    pub ratio: Option<Value>,
}
/// Mechanical Jankó geometry is six rows, two whole-tone families, three
/// physical copies per sounding note. Every desired chromatic address is
/// looked up in K's native twelve-key targets for the CURRENT basis/lens.
/// Fifths never substitutes a second physical pitch map.
pub fn native_janko_catalog(
    binding: &PreparedPerformanceBinding,
    columns: u8,
    base_register: i8,
    transpose: u8,
) -> Result<Vec<NativeKeyboardCell>, String> {
    if !(6..=32).contains(&columns) || transpose > 11 || !(-16..=15).contains(&base_register) {
        return Err("bounded Janko catalog/register/transpose required".into());
    }
    binding.validate_native_consumers(binding.native_basis(), binding.physical_body())?;
    let mut inverse = [None; 12];
    for key in 0..12u8 {
        let t = binding
            .targets()
            .key_target(key, 0, "native:catalog/lookup")?;
        let slot = usize::from(t.pitch_class);
        if slot >= 12 || inverse[slot].replace(key).is_some() {
            return Err("native K pitch-class substrate is not a bijection".into());
        }
    }
    let mut cells = Vec::with_capacity(usize::from(columns) * 6);
    for row in 0..6u8 {
        for column in 0..columns {
            let semitone = i16::from(transpose) + i16::from(row % 2) + 2 * i16::from(column);
            let pitch = (semitone % 12) as u8;
            let register = i16::from(base_register) + semitone / 12;
            if !(-16..=16).contains(&register) {
                return Err("native keyboard register overflow".into());
            }
            let key = inverse[usize::from(pitch)].ok_or("native key lookup absent")?;
            let t = binding
                .targets()
                .key_target(key, register as i8, "native:catalog/lookup")?;
            if t.hertz < 0.001
                || t.hertz >= f64::from(binding.physical_body().request().sample_rate) * 0.45
            {
                return Err("keyboard outside admitted native audio band".into());
            }
            let ratio=t.exact_ratio.map(|r|json!({"numerator":r.numerator().to_string(),"denominator":r.denominator().to_string()}));
            cells.push(NativeKeyboardCell {
                row,
                column,
                register_octave: register as i8,
                key,
                pitch_class: t.pitch_class,
                label: format!("{} / {}", t.source_coordinate.source_ref, register),
                hertz: t.hertz,
                coordinate: t.source_coordinate.source_ref,
                face: u8::from(t.source_coordinate.face == crate::MFace::Pratibimba),
                ratio,
            });
        }
    }
    Ok(cells)
}
