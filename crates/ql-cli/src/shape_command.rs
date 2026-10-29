//! Read-only disclosure of the core's exact shape-address presentation (`ql shape presentation`), and the shape binding of a
//! constellation a caller drafts (`ql shape bind <request.json|->`, `ql.shape-bind/v1`; O-I #742): the canonical shape its members
//! disclose, the `ql-mef/shape-binding/v1` view a Wiki keeps for it and the identity of each same-position generated relation, so the
//! Wiki owner can fold the whole. Bind reads no Wiki, keeps no store, creates no member and asserts no relation.
use crate::CliError;
use ql_core::{
    PresentationAddress, QL_GEOMETRY_SHAPE_VERSION, QlCoordinate, SHAPE_PRESENTATION_VERSION,
    shape_presentation,
};
use ql_wiki::{ShapeBindRequest, bind_shape};
use serde_json::{Value, json};
use std::io::Read;

const USAGE: &str = "usage: ql shape presentation <exact-shape-ref> [--json] | ql shape bind <request.json|-> [--json]";
const MAX_BIND_INPUT: u64 = 1024 * 1024;

/// The bind request at `path` (`-` for stdin), bounded to 1 MiB.
fn bind_input(path: &str) -> Result<String, CliError> {
    let reader: Box<dyn Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(|e| CliError(e.to_string()))?)
    };
    let mut input = String::new();
    reader
        .take(MAX_BIND_INPUT + 1)
        .read_to_string(&mut input)
        .map_err(|e| CliError(e.to_string()))?;
    if input.len() as u64 > MAX_BIND_INPUT {
        return Err(CliError("shape bind request exceeds 1 MiB".into()));
    }
    Ok(input)
}

fn bind(path: &str, json_output: bool) -> Result<String, CliError> {
    let request: ShapeBindRequest = serde_json::from_str(&bind_input(path)?)
        .map_err(|e| CliError(format!("invalid shape bind request: {e}")))?;
    let reading = bind_shape(&request).map_err(|e| CliError(e.to_string()))?;
    if json_output {
        serde_json::to_string_pretty(&reading).map_err(CliError::from)
    } else {
        Ok(format!(
            "{} · {} round {}\n{} generation site(s); no members or relations created.",
            reading.shape_ref,
            reading.grain,
            reading.binding.whole_ref,
            reading.generation_sites.len()
        ))
    }
}

fn coordinate(value: QlCoordinate) -> Value {
    json!({"position":value.position.value(),"face":value.face.as_str()})
}

pub fn command(args: &[String], json_output: bool) -> Result<String, CliError> {
    let [operation, reference] = args else {
        return Err(CliError(USAGE.into()));
    };
    if operation == "bind" {
        return bind(reference, json_output);
    }
    if operation != "presentation" {
        return Err(CliError(USAGE.into()));
    }
    let reading = shape_presentation(reference).ok_or_else(|| {
        CliError(
            "No numerical presentation is supplied for this exact native shape reference".into(),
        )
    })?;
    let sites: Vec<Value> = reading.sites.iter().map(|site| {
        let address = match site.address {
            PresentationAddress::Position(position) => json!({"kind":"position","coordinate":coordinate(position)}),
            PresentationAddress::Cell(cell) => json!({"kind":"cell","row":coordinate(cell.row),"column":coordinate(cell.column)}),
        };
        json!({"address":address,"xyz":site.xyz})
    }).collect();
    let view = json!({
        "schema":SHAPE_PRESENTATION_VERSION,"owner":"ql-core",
        "shape_ref":reading.shape.shape_ref(),"geometry_version":QL_GEOMETRY_SHAPE_VERSION,
        "embedding":reading.embedding,"standing":"numerical presentation convention; not a semantic metric",
        "row_axis":reading.row_axis.into_iter().map(coordinate).collect::<Vec<_>>(),
        "column_axis":reading.column_axis.into_iter().map(coordinate).collect::<Vec<_>>(),
        "sites":sites,
        "constraints":{"frame":"normalized-local","partial_occupancy":true,
            "unique_addresses":true,"fixed_local_slots":true,"whole_transform":"presentation-only",
            "creates_members":false,"asserts_relations":false},
        "source_refs":["docs/geometry/GEOMETRY-CONSTELLATION-WAYFINDER.md",
            "docs/geometry/CANONICAL-CONSTELLATION-RESOLUTION.md","crates/ql-core/src/shape.rs"]
    });
    if json_output {
        serde_json::to_string_pretty(&view).map_err(CliError::from)
    } else {
        Ok(format!(
            "{}\n{} · {} addressed sites\nNumerical presentation only; no members or relations created.",
            reference,
            reading.embedding,
            sites.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ql_core::{ConstellationGrain, QlShape, SixBySixField};
    #[test]
    fn real_cli_dispatch_discloses_core_addresses_and_separate_display_standing() {
        for (reference, count) in [
            (
                QlShape::Constellation(ConstellationGrain::SixFold).shape_ref(),
                6,
            ),
            (SixBySixField::canonical().shape_ref().into(), 36),
        ] {
            let output = crate::execute_cli(&[
                "shape".into(),
                "presentation".into(),
                reference.clone(),
                "--json".into(),
            ])
            .unwrap();
            let reading: Value = serde_json::from_str(&output).unwrap();
            assert_eq!(reading["shape_ref"], reference);
            assert_eq!(reading["schema"], SHAPE_PRESENTATION_VERSION);
            assert_eq!(reading["sites"].as_array().unwrap().len(), count);
            assert_eq!(reading["constraints"]["asserts_relations"], false);
            assert_eq!(reading["constraints"]["creates_members"], false);
            if count == 36 {
                assert_eq!(reading["sites"][0]["address"]["row"]["face"], "direct");
                assert_eq!(
                    reading["sites"][0]["address"]["column"]["face"],
                    "conjugate"
                );
            }
        }
        assert!(
            crate::execute_cli(&["shape".into(), "presentation".into(), "sixfold".into()]).is_err()
        );
        assert!(
            crate::execute_cli(&[
                "shape".into(),
                "presentation".into(),
                QlShape::FourByFourByFour.shape_ref()
            ])
            .is_err()
        );
    }
}
