use crate::exe::ExeImage;
use crate::target::Target;

pub struct Anchor {
    pub target: Target,
    pub width: u8,
    pub stock: u64,
}

pub const ANCHORS: &[Anchor] = &[
    Anchor { target: Target::Data(0xD53DC), width: 4, stock: 0x40000 }, // off_D53DC
    Anchor { target: Target::Data(0xD5D96), width: 2, stock: 0x0B00 },  // word_D5D96
    Anchor { target: Target::Data(0xD5DEE), width: 2, stock: 0x48A0 },  // t_tiretypetab[A]
];

/// How far (in bytes) to scan on either side of each anchor's base offset.
const SCAN_WINDOW: i64 = 0x400;

#[derive(Debug, PartialEq)]
pub enum Calibration {
    Calibrated { delta: i64 },
    Failed,
}

pub fn calibrate(img: &ExeImage) -> Calibration {
    // try delta = 0 first
    if ANCHORS
        .iter()
        .all(|a| img.read(a.target.base_offset(), a.width) == a.stock)
    {
        return Calibration::Calibrated { delta: 0 };
    }

    // window-scan fallback: search ±SCAN_WINDOW around the first anchor for its stock
    // value, derive a candidate delta, then require every anchor to agree at that delta.
    let first = &ANCHORS[0];
    let base = first.target.base_offset() as i64;
    for d in -SCAN_WINDOW..=SCAN_WINDOW {
        let off = base + d;
        if !anchor_matches(img, first, off) {
            continue;
        }
        if ANCHORS
            .iter()
            .all(|a| anchor_matches(img, a, a.target.base_offset() as i64 + d))
        {
            return Calibration::Calibrated { delta: d };
        }
    }

    Calibration::Failed
}

/// Reads `anchor.width` bytes at `off` (if in bounds) and compares to the stock value.
fn anchor_matches(img: &ExeImage, anchor: &Anchor, off: i64) -> bool {
    if off < 0 {
        return false;
    }
    let off = off as usize;
    if off + anchor.width as usize > img.bytes.len() {
        return false;
    }
    img.read(off, anchor.width) == anchor.stock
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exe::ExeImage;
    fn synthetic() -> ExeImage {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        for a in ANCHORS {
            img.write(a.target.base_offset(), a.width, a.stock);
        }
        img
    }
    #[test]
    fn calibrates_standard_exe() {
        assert_eq!(calibrate(&synthetic()), Calibration::Calibrated { delta: 0 });
    }
    #[test]
    fn fails_on_garbage() {
        let img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        assert_eq!(calibrate(&img), Calibration::Failed);
    }
    #[test]
    fn calibrates_shifted_exe() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        for a in ANCHORS {
            img.write(a.target.base_offset() + 0x10, a.width, a.stock);
        }
        assert_eq!(calibrate(&img), Calibration::Calibrated { delta: 0x10 });
    }
}
