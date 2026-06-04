use crate::exe::ExeImage;
use crate::physics_fields::PHYSICS_FIELDS;
use crate::power_curve::{read_curve, write_curve, POWER_CURVE_LEN};
use crate::tyre_fields::TYRE_FIELDS;
use std::collections::BTreeMap;

pub const PHYSICS_FORMAT: &str = "gp2ws-physics";
pub const PHYSICS_VERSION: u32 = 1;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub struct Meta {
    pub format: String,
    pub version: u32,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub struct PhysicsDoc {
    pub meta: Meta,
    pub fields: BTreeMap<String, i64>,
    pub power_curve: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub enum PhysicsError {
    /// TOML deserialization failed.
    Toml(String),
    /// `meta.format` was not the expected `gp2ws-physics`.
    BadFormat(String),
}

impl std::fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhysicsError::Toml(e) => write!(f, "TOML parse error: {e}"),
            PhysicsError::BadFormat(g) => {
                write!(f, "unexpected physics format {g:?}, expected {PHYSICS_FORMAT:?}")
            }
        }
    }
}

impl std::error::Error for PhysicsError {}

/// All known field ids (physics + tyre), in registry order.
fn all_fields() -> impl Iterator<Item = &'static crate::field::FieldDesc> {
    PHYSICS_FIELDS.iter().chain(TYRE_FIELDS.iter())
}

pub fn to_toml(doc: &PhysicsDoc) -> String {
    toml::to_string_pretty(doc).expect("PhysicsDoc serializes")
}

pub fn from_toml(s: &str) -> Result<PhysicsDoc, PhysicsError> {
    let doc: PhysicsDoc = toml::from_str(s).map_err(|e| PhysicsError::Toml(e.to_string()))?;
    if doc.meta.format != PHYSICS_FORMAT {
        return Err(PhysicsError::BadFormat(doc.meta.format));
    }
    Ok(doc)
}

/// Read every physics + tyre field plus the power curve into a `PhysicsDoc`.
pub fn import_from_exe(img: &ExeImage, delta: i64) -> PhysicsDoc {
    let mut fields = BTreeMap::new();
    for f in all_fields() {
        fields.insert(f.id.to_string(), f.read(img, delta));
    }
    let power_curve = read_curve(img, delta).to_vec();
    PhysicsDoc {
        meta: Meta {
            format: PHYSICS_FORMAT.to_string(),
            version: PHYSICS_VERSION,
        },
        fields,
        power_curve,
    }
}

/// Write a `PhysicsDoc` back to the EXE. Keys not present in the registry, and
/// registry fields absent from the doc, are reported as warnings; everything
/// else is written. A power curve of the wrong length is left untouched (with a
/// warning); a curve of the correct length is written.
pub fn export_to_exe(img: &mut ExeImage, delta: i64, doc: &PhysicsDoc) -> Vec<String> {
    let mut warnings = Vec::new();

    // Write known fields present in the doc.
    for f in all_fields() {
        match doc.fields.get(f.id) {
            Some(&v) => f.write(img, delta, v),
            None => warnings.push(format!("missing field: {}", f.id)),
        }
    }

    // Report doc keys that are not known fields.
    for key in doc.fields.keys() {
        if !all_fields().any(|f| f.id == key) {
            warnings.push(format!("unknown field: {key}"));
        }
    }

    // Power curve.
    if doc.power_curve.len() == POWER_CURVE_LEN {
        let mut arr = [0i64; POWER_CURVE_LEN];
        arr.copy_from_slice(&doc.power_curve);
        write_curve(img, delta, &arr);
    } else {
        warnings.push(format!(
            "power_curve has {} entries, expected {} (left untouched)",
            doc.power_curve.len(),
            POWER_CURVE_LEN
        ));
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_doc() -> PhysicsDoc {
        let mut fields = BTreeMap::new();
        fields.insert("tow_strength".to_string(), 262144);
        fields.insert("brake_force".to_string(), 0x160000);
        PhysicsDoc {
            meta: Meta {
                format: PHYSICS_FORMAT.to_string(),
                version: PHYSICS_VERSION,
            },
            fields,
            power_curve: (0..POWER_CURVE_LEN as i64).collect(),
        }
    }

    #[test]
    fn toml_roundtrip() {
        let doc = sample_doc();
        assert_eq!(from_toml(&to_toml(&doc)).unwrap(), doc);
    }

    #[test]
    fn rejects_wrong_format() {
        // Specifically a BadFormat (not a parse error) when shape is otherwise valid.
        // `power_curve` is a top-level array, so it precedes the [meta]/[fields] tables.
        let bad = from_toml("power_curve = []\n[meta]\nformat='x'\nversion=1\n[fields]\n");
        match bad {
            Err(PhysicsError::BadFormat(g)) => assert_eq!(g, "x"),
            other => panic!("expected BadFormat, got {other:?}"),
        }
    }

    #[test]
    fn import_export_identity() {
        // Synthetic image: populate every field + curve with known values.
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        let delta = 0;
        for (i, f) in all_fields().enumerate() {
            // Use values within each field's width; stay small and positive.
            f.write(&mut img, delta, (i as i64 % 100) + 1);
        }
        let doc1 = import_from_exe(&img, delta);

        // Export into a fresh image, re-import, must be identical.
        let mut img2 = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        let warnings = export_to_exe(&mut img2, delta, &doc1);
        assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
        let doc2 = import_from_exe(&img2, delta);
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn export_reports_unknown_and_missing() {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        let mut fields = BTreeMap::new();
        fields.insert("tow_strength".to_string(), 5i64);
        fields.insert("not_a_real_field".to_string(), 9i64);
        let doc = PhysicsDoc {
            meta: Meta {
                format: PHYSICS_FORMAT.to_string(),
                version: PHYSICS_VERSION,
            },
            fields,
            power_curve: vec![1, 2, 3], // wrong length
        };
        let warnings = export_to_exe(&mut img, 0, &doc);
        assert!(warnings.iter().any(|w| w.contains("unknown field: not_a_real_field")));
        assert!(warnings.iter().any(|w| w.contains("missing field: brake_force")));
        assert!(warnings.iter().any(|w| w.contains("power_curve has 3 entries")));
        // The one known present field was still written.
        let known = PHYSICS_FIELDS.iter().find(|f| f.id == "tow_strength").unwrap();
        assert_eq!(known.read(&img, 0), 5);
    }

    #[test]
    fn real_exe_import_export_roundtrip() {
        let Ok(p) = std::env::var("GP2WS_TEST_EXE") else {
            return;
        };
        let img = ExeImage::load(std::path::Path::new(&p)).unwrap();
        let crate::calibration::Calibration::Calibrated { delta } =
            crate::calibration::calibrate(&img)
        else {
            panic!("not calibrated")
        };
        let doc = import_from_exe(&img, delta);
        // Round-trip through TOML.
        let reparsed = from_toml(&to_toml(&doc)).unwrap();
        assert_eq!(reparsed, doc);
        // Export into a clone and confirm bytes for known fields match.
        let mut clone = ExeImage::from_bytes(img.bytes.clone());
        let warnings = export_to_exe(&mut clone, delta, &doc);
        assert!(warnings.is_empty(), "warnings: {warnings:?}");
        assert_eq!(import_from_exe(&clone, delta), doc);
    }
}
