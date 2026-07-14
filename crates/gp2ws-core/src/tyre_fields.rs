use crate::encoding::Encoding;
use crate::field::{FieldDesc, SubTab, Tier};
use crate::target::Target;

// NOTE on table stride:
// The plan described t_TyretypeGrip and t_tiretypetab as "4 x u16". Inspecting the
// real EXE shows the four compound entries are actually stored at **stride 4**
// (each meaningful u16 sits in a dword-aligned slot; the high word is zero). Reading
// at stride 2 would interleave zero words. The four entries are therefore at
// IDA base + 0/4/8/12. Width is kept at 2 (the live value never exceeds 16 bits and
// the upper half is unused), addressed at the dword-aligned offsets. Verified against
// the real EXE: grip = 3000/4000/6000/C000, base = 48A0/49A0/4AA0/4BA0.

pub static TYRE_FIELDS: &[FieldDesc] = &[
    // t_TyretypeGrip (wear sensitivity) — IDA 0xD5DDE, 4 entries stride 4.
    FieldDesc {
        id: "tyre_grip_a",
        label: "Tyre Wear Sensitivity A",
        help: "How fast compound A wears out, shown as a multiplier (1.0 = \
               baseline). Higher = wears faster. A is the hardest, most durable \
               compound, so its wear is the lowest of the four. Old editor: \"Wear\" \
               (Tire Types tab). Stock 0.75.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DDE),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x3000,
        range: None,
    },
    FieldDesc {
        id: "tyre_grip_b",
        label: "Tyre Wear Sensitivity B",
        help: "How fast compound B wears out, shown as a multiplier (1.0 = \
               baseline). Higher = wears faster. B sits between hard A and the \
               softer C/D. Old editor: \"Wear\" (Tire Types tab). Stock 1.0.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DDE + 4),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x4000,
        range: None,
    },
    FieldDesc {
        id: "tyre_grip_c",
        label: "Tyre Wear Sensitivity C",
        help: "How fast compound C wears out, shown as a multiplier (1.0 = \
               baseline). Higher = wears faster. C is a soft compound and wears \
               about twice as fast as hard A. Old editor: \"Wear\" (Tire Types tab). \
               Stock 1.5.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DDE + 8),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x6000,
        range: None,
    },
    FieldDesc {
        id: "tyre_grip_d",
        label: "Tyre Wear Sensitivity D",
        help: "How fast compound D wears out, shown as a multiplier (1.0 = \
               baseline). Higher = wears faster. D is the softest, grippiest \
               compound and wears by far the fastest (about 4x hard A). Old editor: \
               \"Wear\" (Tire Types tab). Stock 3.0.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DDE + 12),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0xC000,
        range: None,
    },
    // t_tiretypetab (base grip) — IDA 0xD5DEE, 4 entries stride 4.
    FieldDesc {
        id: "tyre_base_a",
        label: "Tyre Base Grip A",
        help: "The base grip of compound A, shown as a multiplier (1.0 = \
               baseline). Higher = more grip. A is the hardest compound; in stock \
               data the four compounds are only ~4% apart in grip. Old editor: \"Grip\" \
               (Tire Types tab). Stock ~1.135.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DEE),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x48A0,
        range: None,
    },
    FieldDesc {
        id: "tyre_base_b",
        label: "Tyre Base Grip B",
        help: "The base grip of compound B, shown as a multiplier (1.0 = \
               baseline). Higher = more grip. Raise the softer compounds (or \
               lower A) to make compound choice matter more. Old editor: \"Grip\" (Tire \
               Types tab). Stock ~1.150.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DEE + 4),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x49A0,
        range: None,
    },
    FieldDesc {
        id: "tyre_base_c",
        label: "Tyre Base Grip C",
        help: "The base grip of compound C, shown as a multiplier (1.0 = \
               baseline). Higher = more grip. C is a soft compound - more grip \
               than A/B but it wears faster. Old editor: \"Grip\" (Tire Types tab). \
               Stock ~1.166.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DEE + 8),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x4AA0,
        range: None,
    },
    FieldDesc {
        id: "tyre_base_d",
        label: "Tyre Base Grip D",
        help: "The base grip of compound D, shown as a multiplier (1.0 = \
               baseline). Higher = more grip. D is the softest, grippiest \
               compound but wears out the fastest. Old editor: \"Grip\" (Tire Types \
               tab). Stock ~1.182.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DEE + 12),
        width: 2,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x4BA0,
        range: None,
    },
    // worn_floor (dword_D5DFE) — IDA 0xD5DFE, u32.
    FieldDesc {
        id: "tyre_worn_floor",
        label: "Worn Tyre Floor",
        help: "The grip a fully worn-out tyre keeps - grip never drops below this, \
               shown as a multiplier (1.0 = baseline). Higher = worn tyres stay \
               more usable; lower = a heavier penalty for old rubber. In stock \
               data this is below even a fresh hard A. Old editor: \"Min\" (Tire Types \
               tab). Stock ~1.094.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5DFE),
        width: 4,
        signed: false,
        encoding: Encoding::Fixed(14),
        stock: 0x4600,
        range: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exe::ExeImage;

    #[test]
    fn synthetic_roundtrip() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        for (i, f) in TYRE_FIELDS.iter().enumerate() {
            f.write(&mut img, 0, (i as i64 + 1) * 0x100);
        }
        for (i, f) in TYRE_FIELDS.iter().enumerate() {
            assert_eq!(f.read(&img, 0), (i as i64 + 1) * 0x100, "field {}", f.id);
        }
    }

    #[test]
    fn unique_ids() {
        let mut ids: Vec<&str> = TYRE_FIELDS.iter().map(|f| f.id).collect();
        let len = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), len);
    }

    #[test]
    fn stock_values_match_real_exe() {
        let Ok(p) = std::env::var("GP2WS_TEST_EXE") else {
            return;
        };
        let img = ExeImage::load(std::path::Path::new(&p)).unwrap();
        let crate::calibration::Calibration::Calibrated { delta } =
            crate::calibration::calibrate(&img)
        else {
            panic!("not calibrated")
        };
        for f in TYRE_FIELDS {
            // `stock` is already decoded; `read` decodes internally. Compare
            // directly to avoid double-decoding Bias/Fixed fields.
            assert_eq!(f.read(&img, delta), f.stock, "field {} mismatch", f.id);
        }
    }
}
