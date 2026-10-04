//! Actual N/A/P producer packet for the existing native component floor.
mod producer {
    include!("../tests/support/physical_force_routes_fixture.rs");
}
fn main() -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(&producer::packet()?).map_err(|e| e.to_string())?
    );
    Ok(())
}
