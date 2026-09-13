//! Real provider values must not be silently rounded at the JSON boundary.
//! The standard binary64 parser is the independent conversion oracle here;
//! this is not independent verification of astronomical precision.
#[test]
fn provider_binary64_survives_json_exactly() {
    let longitudes = [
        "123.0353949808801",
        "41.428503200895314",
        "103.52857683989212",
        "107.14662921027215",
        "261.36811978432104",
        "203.76933960039318",
        "324.21448462390146",
        "134.78731943824954",
        "33.04086613043529",
        "53.50895941589172",
        "0.0000000000000001",
        "359.99999999999994",
        "5e-324",
        "2.2250738585072014e-308",
        "1.7976931348623157e308",
    ];
    for text in longitudes {
        let expected: f64 = text.parse().unwrap();
        let parsed: f64 = serde_json::from_str(text).unwrap();
        assert_eq!(parsed.to_bits(), expected.to_bits(), "{text}");
        let encoded = serde_json::to_string(&parsed).unwrap();
        let restored: f64 = serde_json::from_str(&encoded).unwrap();
        assert_eq!(parsed.to_bits(), restored.to_bits(), "{encoded}");
    }
    // Deterministic finite values across the complete binary64 exponent range.
    let mut bits = 0x3ff0_0000_0000_0001_u64;
    for _ in 0..10_000 {
        bits = bits.wrapping_mul(6364136223846793005).wrapping_add(1);
        let value = f64::from_bits(bits);
        if value.is_finite() {
            let text = serde_json::to_string(&value).unwrap();
            let restored: f64 = serde_json::from_str(&text).unwrap();
            assert_eq!(value.to_bits(), restored.to_bits());
        }
    }
}
