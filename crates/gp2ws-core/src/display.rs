//! Pure display/parse helpers shared by the GUI. Kept here (not in the GUI
//! crate) so they can be unit-tested without a windowing system.

use crate::encoding::Encoding;

/// The raw value masked to `width` bytes — i.e. the bit pattern actually stored
/// in the EXE. For a signed field, `read` sign-extends the stored word into a
/// negative `i64`; masking recovers the original word so hex displays as e.g.
/// `0xA000` rather than the 64-bit `0xFFFFFFFFFFFFA000`.
fn masked(value: i64, width: u8) -> u64 {
    let bits = width as u32 * 8;
    if bits >= 64 {
        value as u64
    } else {
        (value as u64) & (((1u128 << bits) - 1) as u64)
    }
}

/// Format a decoded field value for a tooltip / secondary display, based on its
/// [`Encoding`]:
/// - `Fixed(q)` → the float interpretation `value / 2^q`, plus the hex of the value.
/// - `Hex`      → `0x...` hex form (width-masked, so signed words read cleanly).
/// - everything else → plain decimal.
pub fn format_hint(value: i64, enc: Encoding, width: u8) -> String {
    match enc {
        Encoding::Fixed(q) => {
            let scale = (1u64 << q) as f64;
            let f = value as f64 / scale;
            format!("{f} (= {} / 2^{q}); raw 0x{:X}", value, masked(value, width))
        }
        Encoding::Hex => format!("0x{:X}", masked(value, width)),
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
/// `Hex` fields edit in hex (`0x...`, width-masked so a signed word like
/// `0xA000` shows as `0xA000`, not the sign-extended 64-bit form); everything
/// else edits in decimal (signed values show with a leading `-`).
pub fn value_to_edit_string(value: i64, enc: Encoding, width: u8) -> String {
    match enc {
        Encoding::Hex => format!("0x{:X}", masked(value, width)),
        _ => value.to_string(),
    }
}

/// Sign-extend the low `width` bytes of `raw` into an `i64` (no-op when the
/// field is unsigned or the top bit is clear).
fn sign_extend(raw: u64, width: u8) -> i64 {
    let bits = width as u32 * 8;
    if bits >= 64 {
        return raw as i64;
    }
    let shift = 64 - bits;
    ((raw << shift) as i64) >> shift
}

/// Parse a user-entered string back into a decoded value.
/// - `Hex` fields accept `0x`/bare hex and are interpreted as the raw word of
///   the field's `width`, then sign-extended when `signed` (so `0xA000` in a
///   signed 16-bit field becomes `-24576`, round-tripping with the display).
/// - other fields parse decimal (incl. negative), or an explicit `0x` hex.
/// Returns `None` if the text is not a valid integer in the expected base.
pub fn parse_edit_string(text: &str, enc: Encoding, width: u8, signed: bool) -> Option<i64> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    match enc {
        Encoding::Hex => {
            let hex = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
            let raw = u64::from_str_radix(hex, 16).ok()?;
            let raw = masked(raw as i64, width);
            Some(if signed { sign_extend(raw, width) } else { raw as i64 })
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
        let s = value_to_edit_string(0x2C00, Encoding::Hex, 2);
        assert_eq!(s, "0x2C00");
        assert_eq!(parse_edit_string(&s, Encoding::Hex, 2, false), Some(0x2C00));
    }

    #[test]
    fn signed_hex_word_roundtrip() {
        // ai_follow_base: stored word 0xA000 in a signed 16-bit field == -24576.
        // It must display as 0xA000 (not 0xFFFFFFFFFFFFA000) and parse back to -24576.
        let s = value_to_edit_string(-24576, Encoding::Hex, 2);
        assert_eq!(s, "0xA000");
        assert_eq!(parse_edit_string(&s, Encoding::Hex, 2, true), Some(-24576));
        // Tooltip hint shows the masked word too.
        assert_eq!(format_hint(-24576, Encoding::Hex, 2), "0xA000");
    }

    #[test]
    fn decimal_edit_roundtrip() {
        let s = value_to_edit_string(1313, Encoding::Raw, 4);
        assert_eq!(s, "1313");
        assert_eq!(parse_edit_string(&s, Encoding::Raw, 4, false), Some(1313));
        assert_eq!(parse_edit_string("-524288", Encoding::Raw, 4, true), Some(-524288));
    }

    #[test]
    fn parse_rejects_garbage() {
        assert_eq!(parse_edit_string("", Encoding::Raw, 4, false), None);
        assert_eq!(parse_edit_string("xyz", Encoding::Raw, 4, false), None);
        assert_eq!(parse_edit_string("zz", Encoding::Hex, 2, false), None);
    }

    #[test]
    fn hex_field_accepts_bare_and_prefixed() {
        assert_eq!(parse_edit_string("FC00", Encoding::Hex, 4, false), Some(0xFC00));
        assert_eq!(parse_edit_string("0xfc00", Encoding::Hex, 4, false), Some(0xFC00));
    }

    #[test]
    fn format_hint_variants() {
        assert_eq!(format_hint(1313, Encoding::Raw, 4), "1313");
        assert_eq!(format_hint(0x2C00, Encoding::Hex, 2), "0x2C00");
        let h = format_hint(0x4000, Encoding::Fixed(14), 4);
        assert!(h.starts_with("1 ") || h.starts_with("1.0") || h.starts_with("1 ("));
        assert!(h.contains("2^14"));
    }
}
