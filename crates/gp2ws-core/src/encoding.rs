#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Encoding {
    Raw,          // value shown == raw
    Bias(i64),    // shown = raw - bias  (power curve: bias = 0xF282)
    Fixed(u32),   // Q.x fixed point; shown as raw (UI formats the float separately)
    Hex,          // shown as raw, UI renders hex
}

/// raw stored in the EXE -> human value shown in the UI
pub fn decode(raw: i64, enc: Encoding) -> i64 {
    match enc {
        Encoding::Bias(b) => raw - b,
        _ => raw,
    }
}
/// human value -> raw to write back
pub fn encode(value: i64, enc: Encoding) -> i64 {
    match enc {
        Encoding::Bias(b) => value + b,
        _ => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bias_roundtrip_power_curve() {
        let bias = 0xF282;
        // stock raw 0xF29B decodes to 25 with old bias, re-encodes to 0xF29B
        let raw = 0xF29B;
        let shown = decode(raw, Encoding::Bias(bias));
        assert_eq!(shown, 25);
        assert_eq!(encode(shown, Encoding::Bias(bias)), raw);
    }
    #[test]
    fn raw_is_identity() {
        assert_eq!(decode(1313, Encoding::Raw), 1313);
        assert_eq!(encode(1313, Encoding::Raw), 1313);
    }
}
