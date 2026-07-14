use crate::encoding::{decode, encode, Encoding};
use crate::exe::ExeImage;

/// 36 u16 entries at this file offset (Direct), stride 2.
pub const POWER_CURVE_BASE: usize = 1282824;
pub const POWER_CURVE_LEN: usize = 36;
/// Old proven bias: decoded value = raw - 0xF282.
pub const POWER_CURVE_BIAS: i64 = 0xF282;

const ENC: Encoding = Encoding::Bias(POWER_CURVE_BIAS);

/// Read the 36-entry power curve, applying the calibration `delta` and old bias.
pub fn read_curve(img: &ExeImage, delta: i64) -> [i64; POWER_CURVE_LEN] {
    let mut out = [0i64; POWER_CURVE_LEN];
    for (i, slot) in out.iter_mut().enumerate() {
        let off = (POWER_CURVE_BASE as i64 + delta) as usize + i * 2;
        let raw = img.read(off, 2) as i64;
        *slot = decode(raw, ENC);
    }
    out
}

/// Write the 36-entry power curve back, re-applying the old bias.
pub fn write_curve(img: &mut ExeImage, delta: i64, vals: &[i64; POWER_CURVE_LEN]) {
    for (i, &v) in vals.iter().enumerate() {
        let off = (POWER_CURVE_BASE as i64 + delta) as usize + i * 2;
        let raw = encode(v, ENC);
        img.write(off, 2, (raw as u64) & 0xFFFF);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_roundtrip() {
        let mut img = ExeImage::from_bytes(vec![0u8; 2_000_000]);
        let mut vals = [0i64; POWER_CURVE_LEN];
        for (i, v) in vals.iter_mut().enumerate() {
            *v = (i as i64) * 7;
        }
        write_curve(&mut img, 0, &vals);
        assert_eq!(read_curve(&img, 0), vals);
        // first written raw word should be bias (value 0)
        assert_eq!(img.read(POWER_CURVE_BASE, 2), POWER_CURVE_BIAS as u64);
        // entry with value 25 -> raw 0xF29B
        let mut v2 = [0i64; POWER_CURVE_LEN];
        v2[2] = 25;
        write_curve(&mut img, 0, &v2);
        assert_eq!(img.read(POWER_CURVE_BASE + 4, 2), 0xF29B);
    }

    #[test]
    fn delta_applied() {
        let mut img = ExeImage::from_bytes(vec![0u8; 2_000_000]);
        let mut vals = [0i64; POWER_CURVE_LEN];
        vals[0] = 100;
        write_curve(&mut img, 0x20, &vals);
        assert_eq!(read_curve(&img, 0x20)[0], 100);
        assert_eq!(img.read(POWER_CURVE_BASE + 0x20, 2), (100 + POWER_CURVE_BIAS) as u64);
    }

    #[test]
    fn real_exe_curve_sanity() {
        let Ok(p) = std::env::var("GP2WS_TEST_EXE") else {
            return;
        };
        let img = ExeImage::load(std::path::Path::new(&p)).unwrap();
        let crate::calibration::Calibration::Calibrated { delta } =
            crate::calibration::calibrate(&img)
        else {
            panic!("not calibrated")
        };
        let curve = read_curve(&img, delta);
        // Old-bias decoded leading entries.
        assert_eq!(&curve[0..4], &[0, 0, 25, 50], "leading entries mismatch: {:?}", &curve[0..8]);
        // Monotonically rising into the mid-range.
        for i in 2..18 {
            assert!(
                curve[i] <= curve[i + 1],
                "curve not monotonic at {}: {} > {}",
                i,
                curve[i],
                curve[i + 1]
            );
        }
        // Prove POWER_CURVE_LEN is the real table length, not over-long:
        // the LAST entry must still be plausible power data (peak falls off
        // to a positive tail), while the word immediately AFTER the table is
        // adjacent data that decodes to an implausible value. This guards
        // against silently writing past the table into neighbouring fields.
        let last = curve[POWER_CURVE_LEN - 1];
        assert!(
            (1..=1000).contains(&last),
            "last curve entry {} is not plausible power data — table length may be wrong",
            last
        );
        let past_end = img.read(
            (POWER_CURVE_BASE as i64 + delta) as usize + POWER_CURVE_LEN * 2,
            2,
        ) as i64
            - POWER_CURVE_BIAS;
        assert!(
            !(0..=1000).contains(&past_end),
            "word past the curve ({}) looks like curve data — table may be longer than {}",
            past_end,
            POWER_CURVE_LEN
        );
    }
}
