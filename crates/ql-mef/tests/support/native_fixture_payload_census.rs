// Control-only evidence helper called immediately after the genuine
// producer::packet(), before the unchanged bounded carrier assertion.
// It measures actual serde bytes, not estimates or substituted native output.
fn report_original_native_fixture_sizes(packet: &serde_json::Value) {
    fn visit(path: &str, value: &serde_json::Value, depth: usize) {
        let bytes = serde_json::to_vec(value)
            .expect("actual native fixture serialization")
            .len();
        if depth <= 2 {
            eprintln!("actual_native_fixture_size path={path} bytes={bytes}");
        }
        if depth < 2 {
            if let Some(object) = value.as_object() {
                for (key, child) in object {
                    visit(&format!("{path}/{key}"), child, depth + 1);
                }
            }
        }
    }
    visit("", packet, 0);
}
