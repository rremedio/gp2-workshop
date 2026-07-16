pub mod encoding;
pub mod exe;
pub mod target;
pub mod calibration;
pub mod field;
pub mod physics_fields;
pub mod power_curve;
pub mod tyre_fields;
pub mod magic;
pub mod physics_io;
pub mod display;

use std::path::{Path, PathBuf};

use calibration::Calibration;
use exe::ExeImage;

/// Errors surfaced by the [`Session`] façade.
#[derive(Debug)]
pub enum Error {
    /// Underlying file I/O failed.
    Io(std::io::Error),
    /// The EXE could not be calibrated (unknown/garbage image).
    NotCalibrated,
    /// A verify-after-write read back different bytes than were written.
    VerifyMismatch,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotCalibrated => write!(f, "EXE could not be calibrated"),
            Error::VerifyMismatch => {
                write!(f, "verify-after-write failed: file on disk differs from what was written")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// A loaded GP2.EXE plus its calibration result. This is the small façade the
/// GUI uses so it never touches raw offsets directly.
pub struct Session {
    pub path: PathBuf,
    pub img: ExeImage,
    pub calibration: Calibration,
}

impl Session {
    /// Load the EXE at `path` and calibrate it. Calibration failure is *not* an
    /// error here (the caller may still inspect an uncalibrated image); use
    /// [`Session::delta`] to gate writes.
    pub fn open(path: &Path) -> Result<Session, Error> {
        let img = ExeImage::load(path)?;
        let calibration = calibration::calibrate(&img);
        Ok(Session {
            path: path.to_path_buf(),
            img,
            calibration,
        })
    }

    /// The calibration delta, `Some` iff the EXE calibrated successfully.
    pub fn delta(&self) -> Option<i64> {
        match self.calibration {
            Calibration::Calibrated { delta } => Some(delta),
            Calibration::Failed => None,
        }
    }

    /// Back up the original to `<path>.bak` (only if no backup exists yet),
    /// write the in-memory image to disk, then verify by reloading and
    /// comparing every byte.
    pub fn save_backup_and_write(&mut self) -> Result<(), Error> {
        ExeImage::backup(&self.path)?;
        self.img.save(&self.path)?;
        // Verify-after-write: reload and compare bytes.
        let reloaded = ExeImage::load(&self.path)?;
        if reloaded.bytes != self.img.bytes {
            return Err(Error::VerifyMismatch);
        }
        Ok(())
    }

    // ---- Magic-data helpers -------------------------------------------------

    /// Read all 28 magic-data values for `slot` (0..15). Requires calibration.
    pub fn read_magic_slot(&self, slot: usize) -> Result<[i32; magic::MAGIC_COUNT], Error> {
        let delta = self.delta().ok_or(Error::NotCalibrated)?;
        Ok(magic::read_slot(&self.img, delta, slot))
    }

    /// Write magic-data values for `slot` (0..15) into the in-memory image.
    /// With `legacy` set (data loaded from a 24-line `.m2d`), the pit-record
    /// and new AI fields are skipped — see [`magic::NOT_IN_LEGACY`]. Call
    /// [`Session::save_backup_and_write`] to persist. Requires calibration.
    pub fn write_magic_slot(
        &mut self,
        slot: usize,
        vals: &[i32; magic::MAGIC_COUNT],
        legacy: bool,
    ) -> Result<(), Error> {
        let delta = self.delta().ok_or(Error::NotCalibrated)?;
        magic::write_slot(&mut self.img, delta, slot, vals, legacy);
        Ok(())
    }

    // ---- Physics helpers ----------------------------------------------------

    /// Import the full physics set (fields + power curve) from the EXE.
    /// Requires calibration.
    pub fn import_physics(&self) -> Result<physics_io::PhysicsDoc, Error> {
        let delta = self.delta().ok_or(Error::NotCalibrated)?;
        Ok(physics_io::import_from_exe(&self.img, delta))
    }

    /// Write a physics doc back into the in-memory image, returning any
    /// warnings (unknown/missing keys, bad curve length). Call
    /// [`Session::save_backup_and_write`] to persist. Requires calibration.
    pub fn export_physics(&mut self, doc: &physics_io::PhysicsDoc) -> Result<Vec<String>, Error> {
        let delta = self.delta().ok_or(Error::NotCalibrated)?;
        Ok(physics_io::export_to_exe(&mut self.img, delta, doc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal synthetic image with the calibration anchors in place,
    /// big enough to cover all field offsets, and write it to a unique temp file.
    fn synthetic_exe_file() -> PathBuf {
        let mut img = ExeImage::from_bytes(vec![0u8; 2_000_000]);
        for a in calibration::ANCHORS {
            img.write(a.target.base_offset(), a.width, a.stock);
        }
        let dir = std::env::temp_dir().join(format!(
            "gp2ws-session-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("GP2.EXE");
        img.save(&path).unwrap();
        path
    }

    #[test]
    fn open_calibrates_synthetic() {
        let path = synthetic_exe_file();
        let session = Session::open(&path).unwrap();
        assert_eq!(session.delta(), Some(0));
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn open_uncalibrated_returns_none_delta() {
        // Image with no anchors -> calibration fails, but open() still succeeds.
        let img = ExeImage::from_bytes(vec![0u8; 2_000_000]);
        let dir = std::env::temp_dir().join(format!("gp2ws-session-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("bad.exe");
        img.save(&path).unwrap();

        let session = Session::open(&path).unwrap();
        assert_eq!(session.delta(), None);
        // Calibration-gated helpers must refuse.
        assert!(matches!(session.read_magic_slot(0), Err(Error::NotCalibrated)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_backup_and_verify_roundtrip() {
        let path = synthetic_exe_file();
        let mut session = Session::open(&path).unwrap();

        // Edit a magic slot, persist, reopen, confirm it stuck.
        let vals: [i32; magic::MAGIC_COUNT] = core::array::from_fn(|i| {
            if magic::MAGIC_LAYOUT[i].signed {
                -((i as i32) * 13 + 1)
            } else {
                (i as i32) * 13 + 1
            }
        });
        session.write_magic_slot(2, &vals, false).unwrap();
        session.save_backup_and_write().unwrap();

        // Backup was created.
        assert!(path.with_extension("bak").exists());

        let reopened = Session::open(&path).unwrap();
        assert_eq!(reopened.read_magic_slot(2).unwrap(), vals);

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn physics_helpers_roundtrip_through_session() {
        let path = synthetic_exe_file();
        let mut session = Session::open(&path).unwrap();
        let mut doc = session.import_physics().unwrap();
        // brake_force does not overlap a calibration anchor, so the EXE still
        // calibrates after the edit.
        doc.fields.insert("brake_force".to_string(), 12345);
        let warnings = session.export_physics(&doc).unwrap();
        assert!(warnings.is_empty(), "warnings: {warnings:?}");
        session.save_backup_and_write().unwrap();

        let reopened = Session::open(&path).unwrap();
        let doc2 = reopened.import_physics().unwrap();
        assert_eq!(doc2.fields.get("brake_force"), Some(&12345));

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
}
