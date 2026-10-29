//! Binding's engine: the string the chosen matrix ties between the two codons
//! of a hexagram's generation cell.
//!
//! DLW manual §22.3 "The Binding skin": Binding draws the charge-bearing units
//! of M3 as colour charges bound by strings. It runs its own physics over the
//! kernel's symbols and claims no measured constant; it takes from the kernel
//! only the codons, their charges, their pairing and their counts, and every
//! motion has a kernel event behind it.
//!
//! The physics is LightBox's QCD engine (EpiLogos/LightBox
//! `src/qcd/physics.ts`), carried here as it stands where it applies:
//! - the coupling runs at its scale by the one-loop law
//!   α_s(Q) = 12π / ((33 − 2·n_f)·ln(Q²/Λ²)), Λ = 210 MeV and three flavours,
//!   held between 0.1 and 1.5 as LightBox holds it
//!   (`calculateRunningAlphaS`);
//! - the energy is the Cornell potential V(r) = −(4/3)·α_s·ħc/r + σr
//!   (`calculateCornellPotentialMeV`), ħc = 0.19732698 GeV·fm.
//!
//! LightBox takes the string's length from a canvas: the distance between two
//! quarks a hand has placed. Binding has no hand and no canvas. Its stretch is
//! the charge between the string's two ends, the positive and the negative
//! codon, in the register the reading asks for, and each unit of that charge
//! holds the string at LightBox's meson rest length, 0.8 fm (the radius at
//! which its Cornell force is balanced, `calculateCornellForceMagnitude`). The
//! two codons of a cell differ only in their middle site (a·c·d against a·b·d),
//! so the charge between them is the coin value of c against b. A non-dual cell
//! (b = c) is one codon: no charge lies between its ends, no string is drawn,
//! and it has no energy. It is the singlet handed to the torus.
//!
//! The event behind a motion is the last applied M3 receipt (`ql.m3-receipt/v1`):
//! a form written by select-form, spanda-advance or cast-creases is an
//! inscription; change-line a line change; set-matrix or apply-matrix a matrix
//! change. Every other operation leaves the cell where it was and names no
//! motion.
//!
//! A read: it moves no M3 state and calls no service.
use ql_core::*;
use serde::Deserialize;
use serde_json::{Value, json};

pub const BINDING_CONTRACT: &str = "ql.m3-binding/v1";
/// ħc in GeV·fm (LightBox `HBAR_C_MEV_FM` / 1000).
pub const HBAR_C_GEV_FM: f64 = 0.197_326_98;
/// Λ_QCD in GeV (LightBox `DEFAULT_LAMBDA_QCD_MEV`).
pub const LAMBDA_GEV: f64 = 0.210;
/// The active flavours of the one-loop law (LightBox's default).
pub const FLAVOURS: u8 = 3;
/// The meson's rest length in fm: the string held by one unit of charge
/// (LightBox `calculateCornellForceMagnitude` equilibrium radius).
pub const REST_LENGTH_FM: f64 = 0.8;
/// The coupling's floor and ceiling (LightBox's infrared saturation).
pub const ALPHA_FLOOR: f64 = 0.1;
pub const ALPHA_CEILING: f64 = 1.5;
/// The manual's ranges (§22.3 Parameters): Tension 0.1 to 2 GeV/fm, Coupling
/// 0.3 to 10 GeV.
pub const TENSION_RANGE: (f64, f64) = (0.1, 2.0);
pub const SCALE_RANGE: (f64, f64) = (0.3, 10.0);

const STANDING: &str = "Binding's own physics over the kernel's symbols (manual §22.3): LightBox's one-loop coupling (Λ 210 MeV, three flavours, held between 0.1 and 1.5) and Cornell energy; the stretch is the charge between the cell's two codons in the asked register, one unit to LightBox's meson rest length of 0.8 fm. No measured constant is claimed. The cell is ql_core::generate's; the event is the last applied ql.m3-receipt/v1.";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    hexagram: u8,
    matrix: String,
    register: String,
    tension_gev_per_fm: f64,
    coupling_scale_gev: f64,
    #[serde(default)]
    receipt: Option<Value>,
}

const fn family_name(family: MatrixFamily) -> &'static str {
    match family {
        MatrixFamily::Complementary => "complementary",
        MatrixFamily::MovingResting => "moving-resting",
        MatrixFamily::SameQuality => "same-quality",
    }
}

fn matrix_word(name: &str) -> Option<&'static str> {
    match name {
        "complementary" => Some("Complementary"),
        "moving-resting" => Some("Moving and resting"),
        "same-quality" => Some("Same quality"),
        _ => None,
    }
}

fn family_of(name: &str) -> Result<MatrixFamily, String> {
    MatrixFamily::ALL
        .iter()
        .copied()
        .find(|family| family_name(*family) == name)
        .ok_or_else(|| {
            format!("`{name}`: the matrix is complementary, moving-resting or same-quality")
        })
}

/// The charge a register reads from a codon's four.
fn charge(codon: Codon64, register: &str) -> Result<i32, String> {
    let q = codon.four_charge();
    Ok(i32::from(match register {
        "pp" => q.pp,
        "mm" => q.mm,
        "mp" => q.mp,
        "pm" => q.pm,
        other => return Err(format!("`{other}`: the register is pp, mm, mp or pm")),
    }))
}

/// The running coupling at `scale_gev` (LightBox `calculateRunningAlphaS`).
pub fn running_coupling(scale_gev: f64) -> f64 {
    let beta0 = 33.0 - 2.0 * f64::from(FLAVOURS);
    let ratio = (scale_gev.max(0.010) / LAMBDA_GEV).max(1.05);
    let raw = 12.0 * std::f64::consts::PI / (beta0 * 2.0 * ratio.ln());
    raw.clamp(ALPHA_FLOOR, ALPHA_CEILING)
}

/// The Cornell energy of a string of length `r_fm` (LightBox
/// `calculateCornellPotentialMeV`, in GeV; its floor of 0.02 fm kept).
pub fn cornell_energy(r_fm: f64, tension_gev_per_fm: f64, alpha_s: f64) -> f64 {
    let r = r_fm.max(0.02);
    -(4.0 / 3.0) * alpha_s * HBAR_C_GEV_FM / r + tension_gev_per_fm * r
}

fn within(value: f64, (low, high): (f64, f64), what: &str, unit: &str) -> Result<(), String> {
    if value.is_finite() && (low..=high).contains(&value) {
        Ok(())
    } else {
        Err(format!("{what} is {low} to {high} {unit}, got {value}"))
    }
}

fn codon_json(codon: Codon64, register: &str) -> Result<Value, String> {
    let q = codon.four_charge();
    Ok(
        json!({"sequence":codon.to_string(),"address":codon.address(),
        "coin_values":codon.site_values().map(|v| v.value()),
        "charges_pp_mm_mp_pm":[q.pp,q.mm,q.mp,q.pm],"charge":charge(codon, register)?}),
    )
}

/// The kernel event an applied receipt names, or `None` when its operations
/// move no cell.
pub fn binding_event(receipt: &Value) -> Option<Value> {
    if receipt["schema"] != "ql.m3-receipt/v1" || receipt["status"] != "applied" {
        return None;
    }
    let after = &receipt["after"];
    let before = &receipt["before"];
    let hexagram = |state: &Value| state["generation"]["hexagram_id"].as_u64();
    let operations = receipt["command"]["operations"].as_array()?;
    let moving = operations.iter().rev().find(|op| {
        matches!(
            op["operation"].as_str(),
            Some(
                "select-form"
                    | "spanda-advance"
                    | "cast-creases"
                    | "change-line"
                    | "set-matrix"
                    | "apply-matrix"
            )
        )
    })?;
    let (kind, what) = match moving["operation"].as_str()? {
        "change-line" => (
            "line",
            format!(
                "Line {} of hexagram {} changed",
                moving["line"].as_u64()?,
                hexagram(before)?
            ),
        ),
        "set-matrix" | "apply-matrix" => {
            let from = matrix_word(before["law"]["matrix"]["family"].as_str()?)?;
            let to = matrix_word(after["law"]["matrix"]["family"].as_str()?)?;
            ("matrix", format!("{from} to {}", to.to_lowercase()))
        }
        _ => (
            "inscription",
            format!("Hexagram {} inscribed", hexagram(after)?),
        ),
    };
    Some(
        json!({"kind":kind,"what":what,"operation":moving["operation"],
        "generation":receipt["after_generation"]}),
    )
}

/// Binding's reading of a request (`ql.m3-binding/v1`).
pub fn binding_reading(request: &Value) -> Result<Value, String> {
    let request: Request = serde_json::from_value(request.clone())
        .map_err(|error| format!("Binding request: {error}"))?;
    if request.hexagram >= 64 {
        return Err(format!("hexagram {} is outside 0..64", request.hexagram));
    }
    within(
        request.tension_gev_per_fm,
        TENSION_RANGE,
        "Tension",
        "GeV/fm",
    )?;
    within(request.coupling_scale_gev, SCALE_RANGE, "Coupling", "GeV")?;
    let family = family_of(&request.matrix)?;
    let register = request.register.as_str();
    let cell = generate(family, request.hexagram);
    let plus = charge(cell.positive, register)?;
    let minus = charge(cell.negative, register)?;
    let between = (plus - minus).unsigned_abs();
    let stretch = REST_LENGTH_FM * f64::from(between);
    let alpha = running_coupling(request.coupling_scale_gev);
    let energy = (between > 0).then(|| cornell_energy(stretch, request.tension_gev_per_fm, alpha));
    let event = request.receipt.as_ref().and_then(binding_event);
    Ok(
        json!({"schema":BINDING_CONTRACT,"hexagram_id":request.hexagram,
        "matrix":{"family":family_name(family)},"register":register,
        "cell":{"lower":format!("{}{}",cell.lower_pair.first(),cell.lower_pair.second()),
            "upper":format!("{}{}",cell.upper_pair.first(),cell.upper_pair.second()),
            "positive":codon_json(cell.positive, register)?,"negative":codon_json(cell.negative, register)?,
            "non_dual":cell.is_non_dual()},
        "coupling":{"scale_gev":request.coupling_scale_gev,"alpha_s":alpha,"lambda_gev":LAMBDA_GEV,
            "flavours":FLAVOURS,"law":"one-loop"},
        "string":{"tension_gev_per_fm":request.tension_gev_per_fm,"charge_between":between,
            "rest_length_fm":REST_LENGTH_FM,"stretch_fm":stretch,"energy_gev":energy},
        "event":event,"standing":STANDING}),
    )
}
