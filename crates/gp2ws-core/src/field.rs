use crate::encoding::Encoding;
use crate::target::Target;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Section {
    MagicData,
    Physics,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubTab {
    Engine,
    PowerCurve,
    Aero,
    Brakes,
    MassGrip,
    Tyres,
    Slipstream,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tier {
    Basic,
    Advanced,
}

#[derive(Clone, Debug)]
pub struct FieldDesc {
    pub id: &'static str,
    pub label: &'static str,
    pub help: &'static str,
    pub subtab: SubTab,
    pub tier: Tier,
    pub target: Target,
    pub width: u8,
    pub signed: bool,
    pub encoding: Encoding,
    pub stock: i64,
    pub range: Option<(i64, i64)>,
}

impl FieldDesc {
    pub fn read(&self, img: &crate::exe::ExeImage, delta: i64) -> i64 {
        let off = (self.target.base_offset() as i64 + delta) as usize;
        let raw = img.read(off, self.width) as i64;
        let raw = if self.signed {
            sign_extend(raw, self.width)
        } else {
            raw
        };
        crate::encoding::decode(raw, self.encoding)
    }
    pub fn write(&self, img: &mut crate::exe::ExeImage, delta: i64, value: i64) {
        let off = (self.target.base_offset() as i64 + delta) as usize;
        let raw = crate::encoding::encode(value, self.encoding);
        img.write(off, self.width, mask(raw, self.width));
    }
}

fn sign_extend(v: i64, width: u8) -> i64 {
    let bits = width as u32 * 8;
    let shift = 64 - bits;
    (v << shift) >> shift
}
fn mask(v: i64, width: u8) -> u64 {
    (v as u64) & ((1u128 << (width * 8)) - 1) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exe::ExeImage;

    #[test]
    fn raw_unsigned_roundtrip() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1024]);
        let f = FieldDesc {
            id: "test",
            label: "Test",
            help: "",
            subtab: SubTab::Engine,
            tier: Tier::Basic,
            target: Target::Direct(100),
            width: 4,
            signed: false,
            encoding: Encoding::Raw,
            stock: 1313,
            range: None,
        };
        f.write(&mut img, 0, 262144);
        assert_eq!(f.read(&img, 0), 262144);
    }

    #[test]
    fn signed_q14_roundtrip() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1024]);
        let f = FieldDesc {
            id: "gravity",
            label: "Gravity",
            help: "",
            subtab: SubTab::MassGrip,
            tier: Tier::Advanced,
            target: Target::Direct(200),
            width: 4,
            signed: true,
            encoding: Encoding::Raw,
            stock: -0x80000,
            range: None,
        };
        f.write(&mut img, 0, -0x80000);
        assert_eq!(f.read(&img, 0), -0x80000);
    }

    #[test]
    fn bias_curve_roundtrip() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1024]);
        let f = FieldDesc {
            id: "curve",
            label: "Curve",
            help: "",
            subtab: SubTab::PowerCurve,
            tier: Tier::Basic,
            target: Target::Direct(300),
            width: 2,
            signed: false,
            encoding: Encoding::Bias(0xF282),
            stock: 25,
            range: None,
        };
        f.write(&mut img, 0, 25);
        // raw written should be 0xF29B
        assert_eq!(img.read(300, 2), 0xF29B);
        assert_eq!(f.read(&img, 0), 25);
    }

    #[test]
    fn delta_offsets_applied() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1024]);
        let f = FieldDesc {
            id: "d",
            label: "D",
            help: "",
            subtab: SubTab::Engine,
            tier: Tier::Basic,
            target: Target::Direct(100),
            width: 2,
            signed: false,
            encoding: Encoding::Raw,
            stock: 0,
            range: None,
        };
        f.write(&mut img, 0x10, 0xABCD);
        assert_eq!(img.read(100 + 0x10, 2), 0xABCD);
        assert_eq!(f.read(&img, 0x10), 0xABCD);
    }
}
