use crate::exe::ExeImage;

/// (base_file_offset, slot_stride) for tables 1..24. addr = base + slot*stride.
/// All 24 tables are read/written as little-endian `u16`.
pub const MAGIC_LAYOUT: [(usize, usize); 24] = [
    (1280584, 2),
    (1280616, 4),
    (1280680, 4),
    (1280744, 4),
    (1280840, 2),
    (1280872, 2),
    (1280904, 2),
    (1280936, 2),
    (1280976, 4),
    (1280978, 4),
    (1281040, 2),
    (1281072, 2),
    (1281134, 2),
    (1281230, 6),
    (1281232, 6),
    (1281326, 6),
    (1281328, 6),
    (1281422, 2),
    (1281454, 6),
    (1281456, 6),
    (1281458, 6),
    (1281550, 2),
    (1281582, 2),
    (1281614, 4),
];

/// T6 (table index 6, 1-based) is dead data, still written for file fidelity.
pub const MAGIC_DEAD_TABLE: usize = 6;

#[derive(Debug, PartialEq)]
pub enum MagicError {
    /// `.m2d` text did not contain 24 parseable decimal lines.
    BadLineCount(usize),
    /// a line failed to parse as a u16.
    ParseInt { line: usize, value: String },
}

impl std::fmt::Display for MagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicError::BadLineCount(n) => {
                write!(f, "expected 24 values in .m2d, found {n}")
            }
            MagicError::ParseInt { line, value } => {
                write!(f, "line {line}: cannot parse {value:?} as u16")
            }
        }
    }
}

impl std::error::Error for MagicError {}

/// Read all 24 magic-data words for a given slot (0..15).
pub fn read_slot(img: &ExeImage, delta: i64, slot: usize) -> [u16; 24] {
    let mut out = [0u16; 24];
    for (i, slot_out) in out.iter_mut().enumerate() {
        let (base, stride) = MAGIC_LAYOUT[i];
        let off = (base as i64 + delta) as usize + slot * stride;
        *slot_out = img.read(off, 2) as u16;
    }
    out
}

/// Write all 24 magic-data words for a given slot (0..15).
pub fn write_slot(img: &mut ExeImage, delta: i64, slot: usize, vals: &[u16; 24]) {
    for (i, &v) in vals.iter().enumerate() {
        let (base, stride) = MAGIC_LAYOUT[i];
        let off = (base as i64 + delta) as usize + slot * stride;
        img.write(off, 2, v as u64);
    }
}

/// Serialize 24 values to the legacy `.m2d` format: 24 decimal lines in table
/// order 1..24, each terminated by `\r\n` for maximum Windows compatibility.
pub fn to_m2d(vals: &[u16; 24]) -> String {
    let mut s = String::new();
    for v in vals {
        s.push_str(&v.to_string());
        s.push_str("\r\n");
    }
    s
}

/// Parse a `.m2d` text into 24 values. Tolerates both `\n` and `\r\n` line
/// endings and ignores blank trailing lines.
pub fn parse_m2d(text: &str) -> Result<[u16; 24], MagicError> {
    let mut vals: Vec<u16> = Vec::with_capacity(24);
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let v: u16 = line.parse().map_err(|_| MagicError::ParseInt {
            line: idx + 1,
            value: line.to_string(),
        })?;
        vals.push(v);
    }
    if vals.len() != 24 {
        return Err(MagicError::BadLineCount(vals.len()));
    }
    let mut out = [0u16; 24];
    out.copy_from_slice(&vals);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m2d_roundtrip() {
        let v: [u16; 24] = core::array::from_fn(|i| (i as u16) * 100 + 7);
        assert_eq!(parse_m2d(&to_m2d(&v)).unwrap(), v);
    }

    #[test]
    fn to_m2d_uses_crlf_and_24_lines() {
        let v: [u16; 24] = core::array::from_fn(|i| i as u16);
        let s = to_m2d(&v);
        assert_eq!(s.matches("\r\n").count(), 24);
        assert!(s.starts_with("0\r\n1\r\n"));
    }

    #[test]
    fn parse_m2d_tolerates_lf_and_blank_trailing() {
        let mut s = String::new();
        for i in 0..24 {
            s.push_str(&i.to_string());
            s.push('\n');
        }
        s.push_str("\n\n   \n"); // blank trailing lines
        let parsed = parse_m2d(&s).unwrap();
        let expected: [u16; 24] = core::array::from_fn(|i| i as u16);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse_m2d_rejects_wrong_count() {
        assert_eq!(parse_m2d("1\n2\n3\n"), Err(MagicError::BadLineCount(3)));
    }

    #[test]
    fn parse_m2d_rejects_non_integer() {
        let mut s = String::new();
        s.push_str("oops\n");
        for i in 1..24 {
            s.push_str(&i.to_string());
            s.push('\n');
        }
        assert!(matches!(parse_m2d(&s), Err(MagicError::ParseInt { line: 1, .. })));
    }

    #[test]
    fn slot_roundtrip_synthetic() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        let v: [u16; 24] = core::array::from_fn(|i| (i as u16) * 3 + 1);
        write_slot(&mut img, 0, 5, &v);
        assert_eq!(read_slot(&img, 0, 5), v);
    }

    #[test]
    fn slot_roundtrip_with_delta() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        let v: [u16; 24] = core::array::from_fn(|i| (i as u16) * 7 + 2);
        write_slot(&mut img, 0x10, 3, &v);
        assert_eq!(read_slot(&img, 0x10, 3), v);
    }

    #[test]
    fn real_exe_slot0_m2d_roundtrip() {
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

        // Write to a temp .m2d file, re-read, re-parse, assert equality.
        let dir = std::env::temp_dir().join(format!("gp2ws-magic-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("md-slot-1.m2d");
        std::fs::write(&path, to_m2d(&slot0)).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        let parsed = parse_m2d(&text).unwrap();
        assert_eq!(parsed, slot0);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
