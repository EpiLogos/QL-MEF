//! M2-5's hours: the planet of the day and the planet of the hour, counted at a place.
//!
//! The day comes from the sky provider (`providers/sky/hours.py`, `ql.sky-day/v1`):
//! the sunrise before an instant, the sunset after it, the next sunrise, and the
//! place's civil weekday of that sunrise. This module adds no astronomy. It reads
//! the order of the planets from the map, `#2-5` `c_2_chaldean_order_verified`
//! (Sun→Venus→Mercury→Moon→Saturn→Jupiter→Mars, each name the `c_1_name` of an
//! M2-5 planet), and counts with it (DLW manual §22.1; #254 D12):
//! - the day's planet rules its first hour, from sunrise;
//! - each hour's planet is the next in the order, so a day of 24 hours moves the
//!   next day's planet three places on (24 mod 7);
//! - twelve hours run from sunrise to sunset and twelve from sunset to the next sunrise.
//!
//! The week is anchored where the map anchors it: the Sun's day is Sunday
//! (`PLANETARY_RESONANCE.timingOptimal` "Sun hour, Sunday…"; the derivation is
//! `docs/kernel-rebuild/m123-scene-map/m2-sky.md` §2, which yields the weekday rulers
//! Sun, Moon, Mars, Mercury, Jupiter, Venus, Saturn). The maqāmāt bound to the hour's
//! planet are the map's tonic and dominant rules (`m2_condition.rs`,
//! `TONIC_/DOMINANT_PLANETARY_RESONANCE`), carried as they stand.
use serde::Serialize;
use serde_json::{Value, json};

use crate::m2_condition::{CorrespondenceRole, correspondence_field};
use crate::m2_sky::{self, SkyNode};

pub const HOURS_CONTRACT: &str = "ql.sky-hours/v1";
pub const DAY_CONTRACT: &str = "ql.sky-day/v1";
/// The node and property the order is read from.
pub const ORDER_NODE: &str = "#2-5";
pub const ORDER_PROPERTY: &str = "c_2_chaldean_order_verified";
/// The Sun's day is the first of the week, Sunday: ISO weekday 7.
pub const SUN_DAY_ISO_WEEKDAY: u8 = 7;
const SUN: &str = "Sun";

/// One planet of the order: the map's name and its coordinate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ruler {
    pub body: String,
    pub coordinate: String,
}

/// One maqām the map binds to a planet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Bound {
    pub maqam_index: u8,
    pub maqam_name: String,
    pub maqam_coordinate: String,
    pub role: CorrespondenceRole,
}

/// The order of the planets as the map states it, each name resolved to its M2-5 planet.
pub fn order() -> Result<Vec<Ruler>, String> {
    let literal = order_literal()?;
    let planets: Vec<&SkyNode> = m2_sky::sky()
        .nodes
        .iter()
        .filter(|n| n.reference.starts_with("#2-5-") && n.just_ratio.is_some())
        .collect();
    let mut rulers = Vec::new();
    for name in literal.split('→').map(str::trim) {
        let found: Vec<&&SkyNode> = planets
            .iter()
            .filter(|n| n.properties["c_1_name"].as_str() == Some(name))
            .collect();
        let [node] = found.as_slice() else {
            return Err(format!(
                "the map's order names {name:?}, which is not exactly one M2-5 planet"
            ));
        };
        rulers.push(Ruler {
            body: name.to_owned(),
            coordinate: node.reference.clone(),
        });
    }
    let mut names: Vec<&str> = rulers.iter().map(|r| r.body.as_str()).collect();
    names.sort_unstable();
    names.dedup();
    if rulers.len() != 7 || names.len() != 7 || !names.contains(&SUN) {
        return Err("the map's order is not seven distinct planets with the Sun among them".into());
    }
    Ok(rulers)
}

fn order_literal() -> Result<&'static str, String> {
    m2_sky::node(ORDER_NODE)
        .and_then(|n| n.properties[ORDER_PROPERTY].as_str())
        .ok_or_else(|| format!("the map's {ORDER_NODE} states no {ORDER_PROPERTY}"))
}

/// The planet whose day an ISO weekday (1 Monday … 7 Sunday) is.
pub fn day_ruler(order: &[Ruler], iso_weekday: u8) -> Result<&Ruler, String> {
    if !(1..=7).contains(&iso_weekday) {
        return Err("a weekday is 1 (Monday) to 7 (Sunday)".into());
    }
    let sun = order
        .iter()
        .position(|r| r.body == SUN)
        .ok_or("the order has no Sun")?;
    let days_after_sunday = usize::from((iso_weekday + 7 - SUN_DAY_ISO_WEEKDAY) % 7);
    Ok(&order[(sun + 3 * days_after_sunday) % order.len()])
}

/// The maqāmāt the map binds to a planet: its tonic and dominant rules, by index then role.
pub fn bound_to(coordinate: &str) -> Vec<Bound> {
    let mut bound: Vec<Bound> = correspondence_field()
        .rules
        .iter()
        .filter(|rule| rule.planet_coordinate == coordinate)
        .map(|rule| Bound {
            maqam_index: rule.maqam_index,
            maqam_name: rule.maqam_name.clone(),
            maqam_coordinate: rule.maqam_coordinate.clone(),
            role: rule.role,
        })
        .collect();
    bound.sort_by_key(|b| (b.maqam_index, b.role == CorrespondenceRole::Dominant));
    bound
}

fn int(day: &Value, key: &str) -> Result<i64, String> {
    day[key]
        .as_i64()
        .ok_or_else(|| format!("the day states no whole {key}"))
}

/// The hours of one `ql.sky-day/v1`: the day's planet, which hour the instant falls in, its planet and the
/// maqāmāt the map binds to it. The day is carried verbatim; a day out of order is refused in words.
pub fn hours(day: &Value) -> Result<Value, String> {
    if day["schema"] != DAY_CONTRACT {
        return Err(format!("the hours are counted from a {DAY_CONTRACT}"));
    }
    let at = int(day, "epoch_unix_ms")?;
    let rise = int(day, "sunrise_unix_ms")?;
    let set = int(day, "sunset_unix_ms")?;
    let next = int(day, "next_sunrise_unix_ms")?;
    if !(rise <= at && at < next && rise < set && set < next) {
        return Err(
            "the day's sunrise, sunset and next sunrise do not stand round the instant".into(),
        );
    }
    let weekday = day["sunrise_iso_weekday"]
        .as_u64()
        .and_then(|w| u8::try_from(w).ok())
        .ok_or("the day states no weekday of its sunrise")?;
    let scope = day["scope"].as_str().ok_or("the day states no scope")?;
    if scope != "world-declared" && scope != "observer-private" {
        return Err(
            "a day is counted at the world's declared place or at a private observer's".into(),
        );
    }
    let order = order()?;
    let ruler = day_ruler(&order, weekday)?.clone();
    let (half, start, end, before) = if at < set {
        ("day", rise, set, 0)
    } else {
        ("night", set, next, 12)
    };
    let span = i128::from(end - start);
    let index = (i128::from(at - start) * 12 / span) as i64;
    let starts = start + (i128::from(index) * span / 12) as i64;
    let ends = start + (i128::from(index + 1) * span / 12) as i64;
    let first = order
        .iter()
        .position(|r| *r == ruler)
        .ok_or("the day's planet is not in the order")?;
    let hour_ruler = order[(first + (before + index) as usize) % order.len()].clone();
    let bound = bound_to(&hour_ruler.coordinate);
    Ok(json!({
        "schema": HOURS_CONTRACT,
        "day": day,
        "private": scope == "observer-private",
        "day_ruler": ruler,
        "hour": {
            "of": index + 1,
            "half": half,
            "starts_unix_ms": starts,
            "ends_unix_ms": ends,
            "ruler": hour_ruler,
            "bound": bound,
        },
        "order": {
            "rulers": order,
            "literal": order_literal()?,
            "source": {"coordinate": ORDER_NODE, "property": ORDER_PROPERTY,
                "registry_revision": m2_sky::sky().registry_revision},
            "sun_day_iso_weekday": SUN_DAY_ISO_WEEKDAY,
            "week_anchor": "PLANETARY_RESONANCE timingOptimal: the Sun's hour and day is Sunday",
        },
        "bound_source": {"schema": crate::m2_condition::CORRESPONDENCE_CONTRACT,
            "registry_revision": correspondence_field().registry_revision,
            "rules": "TONIC_PLANETARY_RESONANCE and DOMINANT_PLANETARY_RESONANCE whose planet is the hour's"},
        "standing": "the day's times are calculated, not observed; the planets are counted by the map's order",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_order_is_the_maps_and_names_its_planets() {
        let order = order().unwrap();
        let names: Vec<&str> = order.iter().map(|r| r.body.as_str()).collect();
        assert_eq!(
            names,
            [
                "Sun", "Venus", "Mercury", "Moon", "Saturn", "Jupiter", "Mars"
            ]
        );
        assert_eq!(order[0].coordinate, "#2-5-0/1");
        assert_eq!(order[3].coordinate, "#2-5-4");
    }

    #[test]
    fn the_week_runs_sun_moon_mars_mercury_jupiter_venus_saturn() {
        let order = order().unwrap();
        let week: Vec<&str> = [7, 1, 2, 3, 4, 5, 6]
            .into_iter()
            .map(|day| day_ruler(&order, day).unwrap().body.as_str())
            .collect();
        assert_eq!(
            week,
            [
                "Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"
            ]
        );
        assert!(day_ruler(&order, 0).is_err());
        assert!(day_ruler(&order, 8).is_err());
    }
}
