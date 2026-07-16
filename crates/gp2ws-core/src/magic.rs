use crate::exe::ExeImage;

/// One per-slot magic-data field: a little-endian 16-bit word at
/// `base + slot*stride` (file offset, before calibration delta).
#[derive(Clone, Copy, Debug)]
pub struct MagicField {
    pub base: usize,
    pub stride: usize,
    /// Signed fields are sign-extended on read and accept negative values
    /// (the pit-record angle biases and the AI floor tuner).
    pub signed: bool,
}

const fn f(base: usize, stride: usize, signed: bool) -> MagicField {
    MagicField {
        base,
        stride,
        signed,
    }
}

/// Number of per-slot values in the v2 model / `.m2d` v2 file.
pub const MAGIC_COUNT: usize = 28;
/// Number of lines in a legacy (community) `.m2d` file.
pub const LEGACY_COUNT: usize = 24;

/// The full per-slot magic block in strict EXE address order (v2 layout).
///
/// Versus the legacy 24-table model: dead T6 is dropped; the four community
/// "T14–17" stride-6 slices (a cross-slot permutation of the real data) are
/// replaced by the six true pit-render record fields (stride 12); and the
/// three tables discovered in the 2026-07 RE are included (indices 4, 13, 14).
pub const MAGIC_LAYOUT: [MagicField; MAGIC_COUNT] = [
    f(1280584, 2, false), //  0: T1  tyre/track grip
    f(1280616, 4, false), //  1: T2  cornering grip (always)
    f(1280680, 4, false), //  2: T3  cornering grip (qual)
    f(1280744, 4, false), //  3: T4  cornering grip (race)
    f(1280808, 2, true),  //  4: NEW AI consistency floor (D58D4, dormant 0)
    f(1280840, 2, false), //  5: T5  AI bravery
    f(1280904, 2, false), //  6: T7  driver pace (qual)
    f(1280936, 2, false), //  7: T8  driver pace (race)
    f(1280976, 4, false), //  8: T9  lap-clock rate (qual)
    f(1280978, 4, false), //  9: T10 lap-clock rate (race)
    f(1281040, 2, false), // 10: T11 difficulty grip (SemiPro)
    f(1281072, 2, false), // 11: T12 difficulty grip (Rookie)
    f(1281134, 2, false), // 12: T13 CC mistake rate
    f(1281166, 4, false), // 13: NEW AI mistake severity min (D5A3A)
    f(1281168, 4, false), // 14: NEW AI mistake severity max (D5A3C)
    f(1281230, 12, true), // 15: pit record: entry angle A
    f(1281232, 12, true), // 16: pit record: entry angle B
    f(1281234, 12, false), // 17: pit record: entry overlap trim (unsigned!)
    f(1281236, 12, true), // 18: pit record: exit angle A
    f(1281238, 12, true), // 19: pit record: exit angle B
    f(1281240, 12, false), // 20: pit record: exit overlap trim (unsigned!)
    f(1281422, 2, false), // 21: T18 pit-approach zone
    f(1281454, 6, false), // 22: T19 pit-in distance
    f(1281456, 6, false), // 23: T20 pit-out distance
    f(1281458, 6, false), // 24: T21 pit-in speed
    f(1281550, 2, false), // 25: T22 fuel burn (human)
    f(1281582, 2, false), // 26: T23 fuel burn (CC)
    f(1281614, 4, false), // 27: T24 reference lap time
];

/// Legacy `.m2d` line (0-based, lines = T1..T24) → v2 layout index.
/// `None` = the line is not usable: T6 is dead data, and the community
/// T14–17 slices are a cross-slot permutation of the pit record — a
/// single-slot file's T14–17 values belong to *other* slots, so loading
/// them per slot would be wrong and writing them corrupts other tracks.
pub const LEGACY_LINE_MAP: [Option<usize>; LEGACY_COUNT] = [
    Some(0),  // T1
    Some(1),  // T2
    Some(2),  // T3
    Some(3),  // T4
    Some(5),  // T5
    None,     // T6 (dead)
    Some(6),  // T7
    Some(7),  // T8
    Some(8),  // T9
    Some(9),  // T10
    Some(10), // T11
    Some(11), // T12
    Some(12), // T13
    None,     // T14 (permuted pit-record view)
    None,     // T15
    None,     // T16
    None,     // T17
    Some(21), // T18
    Some(22), // T19
    Some(23), // T20
    Some(24), // T21
    Some(25), // T22
    Some(26), // T23
    Some(27), // T24
];

/// v2 layout indices that a legacy file does NOT provide (and that a
/// legacy-sourced export must leave untouched in the EXE).
pub const NOT_IN_LEGACY: [usize; 9] = [4, 13, 14, 15, 16, 17, 18, 19, 20];

/// Inclusive value range accepted by field `i`.
pub fn field_range(i: usize) -> (i32, i32) {
    if MAGIC_LAYOUT[i].signed {
        (i16::MIN as i32, i16::MAX as i32)
    } else {
        (0, u16::MAX as i32)
    }
}

/// A parsed `.m2d` slot. `legacy` is true when it came from a 24-line file:
/// the `NOT_IN_LEGACY` entries of `vals` are zero-filled placeholders and
/// must be skipped when writing to the EXE.
#[derive(Debug, PartialEq)]
pub struct MagicSlot {
    pub vals: [i32; MAGIC_COUNT],
    pub legacy: bool,
}

#[derive(Debug, PartialEq)]
pub enum MagicError {
    /// text did not contain 24 (legacy) or 28 (v2) parseable lines.
    BadLineCount(usize),
    /// a line failed to parse or was out of range for its field.
    ParseInt { line: usize, value: String },
}

impl std::fmt::Display for MagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicError::BadLineCount(n) => {
                write!(f, "expected 24 (legacy) or 28 (v2) values in .m2d, found {n}")
            }
            MagicError::ParseInt { line, value } => {
                write!(f, "line {line}: {value:?} is not a valid value")
            }
        }
    }
}

impl std::error::Error for MagicError {}

/// Read all 28 magic-data values for a given slot (0..15).
pub fn read_slot(img: &ExeImage, delta: i64, slot: usize) -> [i32; MAGIC_COUNT] {
    let mut out = [0i32; MAGIC_COUNT];
    for (i, slot_out) in out.iter_mut().enumerate() {
        let fld = MAGIC_LAYOUT[i];
        let off = (fld.base as i64 + delta) as usize + slot * fld.stride;
        let raw = img.read(off, 2) as u16;
        *slot_out = if fld.signed {
            raw as i16 as i32
        } else {
            raw as i32
        };
    }
    out
}

/// Write magic-data values for a given slot (0..15). When `legacy` is true
/// only the fields a legacy file provides are written — the pit record and
/// the new AI fields (`NOT_IN_LEGACY`) are left untouched in the EXE.
pub fn write_slot(
    img: &mut ExeImage,
    delta: i64,
    slot: usize,
    vals: &[i32; MAGIC_COUNT],
    legacy: bool,
) {
    for (i, &v) in vals.iter().enumerate() {
        if legacy && NOT_IN_LEGACY.contains(&i) {
            continue;
        }
        let fld = MAGIC_LAYOUT[i];
        let off = (fld.base as i64 + delta) as usize + slot * fld.stride;
        img.write(off, 2, (v as i16 as u16) as u64);
    }
}

/// Serialize to the v2 `.m2d` format: 28 decimal lines (signed fields as
/// signed decimals) in EXE address order, `\r\n` terminated. Files are never
/// saved in the legacy format.
pub fn to_m2d(vals: &[i32; MAGIC_COUNT]) -> String {
    let mut s = String::new();
    for v in vals {
        s.push_str(&v.to_string());
        s.push_str("\r\n");
    }
    s
}

/// Parse a `.m2d` text. 28 lines = v2 (EXE address order); 24 lines = legacy
/// (community table order T1..T24 — dead T6 and the permuted T14–17 lines
/// are discarded, see `LEGACY_LINE_MAP`). Tolerates `\n`/`\r\n` and blank
/// trailing lines.
pub fn parse_m2d(text: &str) -> Result<MagicSlot, MagicError> {
    let mut lines: Vec<(usize, &str)> = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if !line.is_empty() {
            lines.push((idx + 1, line));
        }
    }

    let parse = |lineno: usize, s: &str, min: i32, max: i32| -> Result<i32, MagicError> {
        match s.parse::<i32>() {
            Ok(v) if v >= min && v <= max => Ok(v),
            _ => Err(MagicError::ParseInt {
                line: lineno,
                value: s.to_string(),
            }),
        }
    };

    match lines.len() {
        MAGIC_COUNT => {
            let mut vals = [0i32; MAGIC_COUNT];
            for (i, &(lineno, s)) in lines.iter().enumerate() {
                let (min, max) = field_range(i);
                vals[i] = parse(lineno, s, min, max)?;
            }
            Ok(MagicSlot {
                vals,
                legacy: false,
            })
        }
        LEGACY_COUNT => {
            let mut vals = [0i32; MAGIC_COUNT];
            for (li, &(lineno, s)) in lines.iter().enumerate() {
                // legacy files are plain u16 lines
                let v = parse(lineno, s, 0, u16::MAX as i32)?;
                if let Some(idx) = LEGACY_LINE_MAP[li] {
                    // signed target fields keep their bit pattern
                    vals[idx] = if MAGIC_LAYOUT[idx].signed {
                        v as u16 as i16 as i32
                    } else {
                        v
                    };
                }
            }
            Ok(MagicSlot { vals, legacy: true })
        }
        n => Err(MagicError::BadLineCount(n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_is_contiguous_and_ordered() {
        // strictly increasing bases, and the pit record spans exactly the
        // 192 bytes between T13's end-adjacent pair and T18.
        for w in MAGIC_LAYOUT.windows(2) {
            assert!(w[0].base < w[1].base);
        }
        assert_eq!(MAGIC_LAYOUT[15].base, 1281230); // record base (old T14)
        assert_eq!(MAGIC_LAYOUT[15].base + 16 * 12, MAGIC_LAYOUT[21].base); // → T18
        assert_eq!(MAGIC_LAYOUT[3].base + 16 * 4, MAGIC_LAYOUT[4].base); // T4 → D58D4
        assert_eq!(MAGIC_LAYOUT[12].base + 16 * 2, MAGIC_LAYOUT[13].base); // T13 → D5A3A
    }

    #[test]
    fn m2d_v2_roundtrip_with_negatives() {
        let mut v = [0i32; MAGIC_COUNT];
        for (i, val) in v.iter_mut().enumerate() {
            *val = if MAGIC_LAYOUT[i].signed {
                -(i as i32) * 100 - 7
            } else {
                (i as i32) * 100 + 7
            };
        }
        let parsed = parse_m2d(&to_m2d(&v)).unwrap();
        assert!(!parsed.legacy);
        assert_eq!(parsed.vals, v);
    }

    #[test]
    fn to_m2d_uses_crlf_and_28_lines() {
        let v = [0i32; MAGIC_COUNT];
        let s = to_m2d(&v);
        assert_eq!(s.matches("\r\n").count(), MAGIC_COUNT);
    }

    #[test]
    fn parse_legacy_maps_lines_and_flags() {
        // legacy line i (0-based) carries value 1000+i
        let mut s = String::new();
        for i in 0..LEGACY_COUNT {
            s.push_str(&(1000 + i).to_string());
            s.push('\n');
        }
        let parsed = parse_m2d(&s).unwrap();
        assert!(parsed.legacy);
        // T1 (line 1) → idx 0; T5 (line 5) → idx 5; T7 (line 7) → idx 6;
        // T18 (line 18) → idx 21; T24 (line 24) → idx 27.
        assert_eq!(parsed.vals[0], 1000);
        assert_eq!(parsed.vals[5], 1004);
        assert_eq!(parsed.vals[6], 1006);
        assert_eq!(parsed.vals[21], 1017);
        assert_eq!(parsed.vals[27], 1023);
        // fields legacy files don't carry stay zero
        for &i in &NOT_IN_LEGACY {
            assert_eq!(parsed.vals[i], 0, "idx {i}");
        }
    }

    #[test]
    fn parse_rejects_wrong_count_and_range() {
        assert_eq!(parse_m2d("1\n2\n3\n"), Err(MagicError::BadLineCount(3)));
        // unsigned field (line 1 = T1) rejects negatives in v2
        let mut v = [0i32; MAGIC_COUNT];
        v[0] = 1;
        let mut s = to_m2d(&v);
        s = s.replacen("1\r\n", "-1\r\n", 1);
        assert!(matches!(
            parse_m2d(&s),
            Err(MagicError::ParseInt { line: 1, .. })
        ));
    }

    #[test]
    fn slot_roundtrip_synthetic_full_and_legacy() {
        let mut img = ExeImage::from_bytes(vec![0u8; 2_000_000]);
        let mut v = [0i32; MAGIC_COUNT];
        for (i, val) in v.iter_mut().enumerate() {
            *val = if MAGIC_LAYOUT[i].signed {
                -((i as i32) * 3 + 1)
            } else {
                (i as i32) * 3 + 1
            };
        }
        write_slot(&mut img, 0, 5, &v, false);
        assert_eq!(read_slot(&img, 0, 5), v);

        // legacy write must not touch the pit record / new AI fields
        let mut v2 = v;
        for (i, val) in v2.iter_mut().enumerate() {
            *val = if MAGIC_LAYOUT[i].signed { -999 } else { 999 };
        }
        write_slot(&mut img, 0, 5, &v2, true);
        let after = read_slot(&img, 0, 5);
        for i in 0..MAGIC_COUNT {
            if NOT_IN_LEGACY.contains(&i) {
                assert_eq!(after[i], v[i], "idx {i} must be untouched");
            } else {
                assert_eq!(after[i], v2[i], "idx {i} must be written");
            }
        }
    }

    #[test]
    fn real_exe_slot0_values_and_roundtrip() {
        let Ok(p) = std::env::var("GP2WS_TEST_EXE") else {
            return;
        };
        let img = ExeImage::load(std::path::Path::new(&p)).unwrap();
        let crate::calibration::Calibration::Calibrated { delta } =
            crate::calibration::calibrate(&img)
        else {
            panic!("not calibrated")
        };
        let slot0 = read_slot(&img, delta, 0);
        // Brazil's pit record per docs/MAGIC-T14-17-PIT-RENDER.md §2
        assert_eq!(&slot0[15..21], &[1792, -512, 4, -2176, 2048, 4]);
        // the three new tables: dormant floor 0, mistake window 512/2048
        assert_eq!(slot0[4], 0);
        assert_eq!(slot0[13], 512);
        assert_eq!(slot0[14], 2048);

        let parsed = parse_m2d(&to_m2d(&slot0)).unwrap();
        assert!(!parsed.legacy);
        assert_eq!(parsed.vals, slot0);
    }
}
