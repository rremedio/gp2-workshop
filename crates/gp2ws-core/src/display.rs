//! Pure display/parse helpers shared by the GUI. Kept here (not in the GUI
//! crate) so they can be unit-tested without a windowing system.

use crate::encoding::Encoding;

/// Format a decoded field value for a tooltip / secondary display, based on its
/// [`Encoding`]:
/// - `Fixed(q)` → the float interpretation `value / 2^q`, plus the hex of the value.
/// - `Hex`      → `0x...` hex form.
/// - everything else → plain decimal.
pub fn format_hint(value: i64, enc: Encoding) -> String {
    match enc {
        Encoding::Fixed(q) => {
            let scale = (1u64 << q) as f64;
            let f = value as f64 / scale;
            format!("{f} (= {} / 2^{q}); raw 0x{:X}", value, value)
        }
        Encoding::Hex => format!("0x{:X}", value),
        _ => value.to_string(),
    }
}

/// Format the float value of a `Fixed(q)` encoding, or `None` for other
/// encodings (which have no float interpretation).
pub fn fixed_float(value: i64, enc: Encoding) -> Option<f64> {
    match enc {
        Encoding::Fixed(q) => Some(value as f64 / (1u64 << q) as f64),
        _ => None,
    }
}

/// How the editable text box for a field should be seeded from a decoded value.
/// `Hex` fields edit in hex (`0x...`); everything else edits in decimal.
pub fn value_to_edit_string(value: i64, enc: Encoding) -> String {
    match enc {
        Encoding::Hex => format!("0x{:X}", value),
        _ => value.to_string(),
    }
}

/// Parse a user-entered string back into a decoded value, honoring `Hex`
/// fields (which accept `0x` / bare-hex). Decimal (incl. negative) otherwise.
/// Returns `None` if the text is not a valid integer in the expected base.
pub fn parse_edit_string(text: &str, enc: Encoding) -> Option<i64> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    match enc {
        Encoding::Hex => {
            let hex = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
            i64::from_str_radix(hex, 16).ok()
        }
        _ => {
            // Allow an explicit 0x for convenience even on non-hex fields.
            if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                i64::from_str_radix(hex, 16).ok()
            } else {
                t.parse::<i64>().ok()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_float_q14() {
        // 0x4000 in Q.14 == 1.0
        assert_eq!(fixed_float(0x4000, Encoding::Fixed(14)), Some(1.0));
        assert_eq!(fixed_float(0x2000, Encoding::Fixed(14)), Some(0.5));
        assert_eq!(fixed_float(5, Encoding::Raw), None);
    }

    #[test]
    fn hex_edit_roundtrip() {
        let s = value_to_edit_string(0x2C00, Encoding::Hex);
        assert_eq!(s, "0x2C00");
        assert_eq!(parse_edit_string(&s, Encoding::Hex), Some(0x2C00));
    }

    #[test]
    fn decimal_edit_roundtrip() {
        let s = value_to_edit_string(1313, Encoding::Raw);
        assert_eq!(s, "1313");
        assert_eq!(parse_edit_string(&s, Encoding::Raw), Some(1313));
        assert_eq!(parse_edit_string("-524288", Encoding::Raw), Some(-524288));
    }

    #[test]
    fn parse_rejects_garbage() {
        assert_eq!(parse_edit_string("", Encoding::Raw), None);
        assert_eq!(parse_edit_string("xyz", Encoding::Raw), None);
        assert_eq!(parse_edit_string("zz", Encoding::Hex), None);
    }

    #[test]
    fn hex_field_accepts_bare_and_prefixed() {
        assert_eq!(parse_edit_string("FC00", Encoding::Hex), Some(0xFC00));
        assert_eq!(parse_edit_string("0xfc00", Encoding::Hex), Some(0xFC00));
    }

    #[test]
    fn format_hint_variants() {
        assert_eq!(format_hint(1313, Encoding::Raw), "1313");
        assert_eq!(format_hint(0x2C00, Encoding::Hex), "0x2C00");
        let h = format_hint(0x4000, Encoding::Fixed(14));
        assert!(h.starts_with("1 ") || h.starts_with("1.0") || h.starts_with("1 ("));
        assert!(h.contains("2^14"));
    }
}
